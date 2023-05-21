use std::{io::{Read, Write}, collections::HashMap, process::Stdio};

use downloader::{get_cfn_resource_provider_schema, get_cfn_resource_spec, Zipped};
use html_extractor::extract_documentation;
use parser::{get_schema, get_service_name, get_spec_schema, CfnResourceProviderSchema, CfnResourceSpecSchema, PropertyLike};

use rayon::prelude::*;

/// as we parse, we create a service crate for each
/// resourec type. eg a resource type 'AWS::Cloudfront::Distribution'
/// would have service crate name 'Cloudfront'.
/// this object's job is to create the actual crate for this service.
/// since there are many resource types per service, internally we append to an
/// existing crate if we previously created it. for that reason there is only one
/// module per service at a time, even though at the end there will be multiple.
pub struct ServiceCrate {
    pub name: String,
    pub module: ModuleDef,
}

pub struct ModuleDef {
    /// the name of the module => name of resource type.
    /// eg in 'AWS::Cloudfront::Distribution', this name would be 'Distribution'
    pub name: String,
    /// doc comment to put at the top of the module file.
    pub doc_comment: String,
    /// the main struct for this module. eg in case of cloudfront distribution this would be
    /// the Distribution struct, which would have 2 fields: DistributionConfig, and Tags
    pub entrypoint_struct: CfnStruct,
    /// helper structs that are referenced by the entrypoint struct.
    /// in case of a cloudfront distribution, this would be the DistributionConfig, plus
    /// all of the complex types under the DistributionConfig
    pub auxiliary_structs: Vec<CfnStruct>,
    /// the raw name like 'AWS::Cloudfront::Distribution'
    pub resource_name: String,
}

impl ModuleDef {
    pub fn safe_name(&self) -> String {
        let name = convert_snake_case(&self.name);
        safe_ident(&name)
    }
}

fn safe_ident(name: &str) -> String {
    if name == "macro" || name == "type" || name == "override" || name == "match" || name == "else" {
        format!("cfn_{}", name)
    } else {
        name.to_string()
    }
}

pub struct CfnStruct {
    pub name: String,
    /// the doc comment that goes above the struct's name
    pub documentation: String,
    pub fields: Vec<CfnField>,
}

pub struct CfnField {
    pub name: String,
    pub required: bool,
    /// the type can be a primitive (like string, int, etc.)
    /// list of primitives, complex (like a custom struct)
    /// or list of complex. it is the job of the caller to properly create the type.
    /// for example if the caller has a list of complex struct, it should set
    /// type to "Vec<ComplexType>". Note in case of optional fields,
    /// the caller does not need to add "Option<...>". Instead we handle
    /// this internally via checking the required field.
    pub ty: String,
    /// it is the caller's job to properly format this
    pub documentation: String,

    pub valid_values: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
}

impl CfnField {
    pub fn get_type(
        &self,
        field_enums: &mut HashMap<String, Vec<String>>,
        owning_struct_name: &str,
        use_map_tracker: &mut HashMap<String, Vec<String>>
    ) -> String {
        let mut should_box = false;
        // check for cyclical data structures:
        if let Some(fields_of_ty) = use_map_tracker.get(&self.ty) {
            if fields_of_ty.iter().any(|x| x == owning_struct_name) {
                // the type of this field is some other struct that referenced our
                // owning struct. therefore we'd have a cyclical data structure.
                // so we fix that by Boxing it
                should_box = true;
            }
        }
        let use_type = if let Some(valid_values) = &self.valid_values {
            if self.ty == "String" {
                let enum_name = format!("{}{}Enum", owning_struct_name, self.name);
                field_enums.insert(enum_name.clone(), valid_values.clone());
                enum_name
            } else {
                self.ty.clone()
            }
        } else {
            // there are no valid_values, so check if this is a string,
            // if so, use StrVal instead of String:
            if self.ty == "String" {
                "cfn_resources::StrVal".to_string()
            } else {
                self.ty.clone()
            }
        };
        let use_type = if should_box {
            format!("Box<{}>", use_type)
        } else {
            use_type
        };
        if !self.required {
            format!("Option<{}>", use_type)
        } else {
            use_type
        }
    }
    pub fn get_field_name(&self) -> String {
        safe_ident(&convert_snake_case(&self.name))
    }
}

impl ServiceCrate {
    pub fn output(self) {
        let base_dir = env!("CARGO_MANIFEST_DIR");
        let out_dir = format!("{base_dir}/../output");
        if let Err(e) = std::fs::create_dir_all(&out_dir) {
            panic!("Failed to create output dir {out_dir}\n{:?}", e);
        }

        let service_name_safe = convert_snake_case(&self.name);
        // println!("{service_name} -> {service_name_safe}");
        let crate_dir = format!("{out_dir}/{service_name_safe}");
        let src_dir = format!("{}/src", crate_dir);
        if let Err(e) = std::fs::create_dir_all(&crate_dir) {
            panic!("Failed to create crate dir {crate_dir}\n{:?}", e);
        }
        if let Err(e) = std::fs::create_dir_all(&src_dir) {
            panic!("Failed to create src dir {src_dir}\n{:?}", e);
        }
        emit_cargo_toml(&crate_dir, &service_name_safe, "");
        let lib_file_path = format!("{src_dir}/lib.rs");
        let mut lib_file = match std::fs::File::options().append(true).create(true).open(&lib_file_path) {
            Ok(f) => f,
            Err(e) => {
                panic!("Failed to open {lib_file_path}\n{:?}", e);
            }
        };

        // append the specific module we will be making
        let append_line = format!("pub mod {};\n", self.module.safe_name());
        if let Err(e) = lib_file.write_all(append_line.as_bytes()) {
            panic!("Failed to write module line to {lib_file_path}\n{:?}", e);
        }
        let out_file_path = format!("{src_dir}/{}.rs", self.module.safe_name());
        emit_module_file(&out_file_path, &self.module);
    }
}

pub fn emit_cfn_resource_impl(
    validations: String,
    use_name: &str,
    resource_type_str: &str
) -> String {
    format!("
impl cfn_resources::CfnResource for {} {{
    fn type_string(&self) -> &'static str {{
        \"{}\"
    }}

    fn properties(&self) -> serde_json::Value {{
        serde_json::to_value(self).expect(\"Failed to serialize to value\")
    }}

    fn validate(&self) -> Result<(), String> {{
{validations}
        Ok(())
    }}
}}", use_name, resource_type_str)
}

pub fn format_file(s: String) -> Result<String, String> {
    let mut cmd = std::process::Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn().map_err(|e| format!("Failed to spawn rustfmt\n{:?}", e))?;
    if let Some(handle) = &mut cmd.stdin.take() {
        if let Err(e) = handle.write_all(s.as_bytes()) {
            return Err(format!("Failed to write to rustfmt stdin\n{:?}", e));
        }
    } else {
        return Err(format!("Failed to aquire stdin handle"));
    };
    let out = cmd.wait_with_output().map_err(|e| format!("Failed to wait for rustfmt\n{:?}", e))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        return Err(err);
    }
    let out_string = String::from_utf8_lossy(&out.stdout).to_string();
    Ok(out_string)
}

pub fn emit_module_file(path: &str, module: &ModuleDef) {
    let mut use_map_tracker = HashMap::new();
    let use_name = format!("Cfn{}", module.name);
    let mut field_validations = "".to_string();
    let mut out_str = emit_struct(&mut field_validations, &mut use_map_tracker, &use_name, &module.entrypoint_struct);
    out_str.push_str(&emit_cfn_resource_impl(field_validations, &use_name, &module.resource_name));

    for aux in module.auxiliary_structs.iter() {
        let mut aux_field_validations = "".to_string();
        out_str.push_str(&emit_struct(&mut aux_field_validations, &mut use_map_tracker, &aux.name, aux));
        out_str.push_str(&emit_cfn_resource_impl(aux_field_validations, &aux.name, "NOT_A_VALID_CFN_RESOURCE"));
    }
    let out_str = match format_file(out_str) {
        Ok(o) => o,
        Err(e) => {
            panic!("Failed to format {path}\n{e}");
        }
    };
    if let Err(e) = std::fs::write(path, out_str) {
        panic!("Failed while writing out to {path}\nFor module {}\n{:?}", module.name, e);
    }
}

pub fn add_or_insert_to_use_map(use_map_tracker: &mut HashMap<String, Vec<String>>, key: String, val: String) {
    match use_map_tracker.get_mut(key.as_str()) {
        Some(existing) => {
            existing.push(val);
        }
        None => {
            use_map_tracker.insert(key, vec![val]);
        }
    }
}

pub fn emit_field_enum_variant(val: &String) -> (String, String) {
    let mut val_renamed = "".to_string();
    for (i, c) in val.chars().enumerate() {
        if i == 0 {
            if c.is_numeric() {
                val_renamed.push('E');
            }
            val_renamed.push(c.to_ascii_uppercase());
        } else {
            if c == '-' || c == '_' || c == '.' || c == ' ' || c == '/' || c == '\\' || c == ')' || c == '(' || c == '[' || c == ']' || c == '{' || c == '}' || c == '@' || c == '$' || c == '!' || c == ':' || c == ';' || c == '=' || c == '%' || c == '"' || c == '\'' || c == '#' || c == '&' {
                continue;
            }
            val_renamed.push(c.to_ascii_lowercase());
        }
    }
    if val_renamed == "Self" {
        val_renamed.insert(0, 'S');
    }
    let out = format!("
    /// {val}
    #[serde(rename = \"{val}\")]
    {val_renamed},
");
    (val_renamed, out)
}

pub fn emit_field_enum(name: String, values: Vec<String>) -> String {
    let mut first = "".to_string();
    let mut variants = "".to_string();
    for (i, v) in values.iter().enumerate() {
        let (variant_name, code) = emit_field_enum_variant(v);
        if i == 0 {
            first = variant_name;
        }
        variants.push_str(&code);
    }
    format!("
#[derive(Clone, Debug, serde::Serialize)]
pub enum {name} {{
{variants}
}}

impl Default for {name} {{
    fn default() -> Self {{
        {name}::{first}
    }}
}}
")
}

pub fn emit_field_enums(field_enums: HashMap<String, Vec<String>>) -> String {
    let mut out = "".to_string();
    let mut list = vec![];
    for (name, values) in field_enums {
        list.push((name, values));
    }
    // make enum outputs deterministic
    list.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, values) in list {
        out.push_str(&emit_field_enum(name, values));
    }
    out
}

pub fn add_check_for_max_or_min(out: &mut String, f: &CfnField, limit: f64, direction: &str) {
    let (validation_type, word) = if direction == ">" {
        ("Max", "greater")
    } else {
        ("Min", "less")
    };

    let inner = if f.ty == "String" {
        // if its a string, we actually represent it as StrVal, so we need to only apply
        // this check if its a String underneath:
        format!("
        if let cfn_resources::StrVal::String(s) = &the_val {{
            if s.len() {} {} as _ {{
                return Err(format!(\"{} validation failed on field '{}'. {{}} is {} than {}\", s.len()));
            }}
        }}
", direction, limit, validation_type, f.get_field_name(), word, limit)
    } else if f.ty.starts_with("Vec<") {
        format!("
        if the_val.len() {} {} as _ {{
            return Err(format!(\"{} validation failed on field '{}'. {{}} is {} than {}\", the_val.len()));
        }}
", direction, limit, validation_type, f.get_field_name(), word, limit)
    } else if f.ty == "i64" || f.ty == "f64" {
        format!("
        if *the_val {} {} as _ {{
            return Err(format!(\"{} validation failed on field '{}'. {{}} is {} than {}\", the_val));
        }}
", direction, limit, validation_type, f.get_field_name(), word, limit)
    } else {
        return;
    };

    let push_s = if !f.required {
        format!("
        if let Some(the_val) = &self.{} {{
{}
        }}
        ", f.get_field_name(), inner)
    } else {
        format!("
        let the_val = &self.{};
{}
        ", f.get_field_name(), inner)
    };

    out.push_str(&push_s);
}

pub fn emit_field_validation(is_custom_type: bool, f: &CfnField) -> String {
    if f.valid_values.is_some() {
        return "".to_string();
    }

    if is_custom_type {
        let validation_call = if f.required {
            "validate()?;"
        } else {
            "as_ref().map_or(Ok(()), |val| val.validate())?;"
        };
        return format!("
        self.{}.{}
", f.get_field_name(), validation_call);
    }

    let mut out_str = "".to_string();
    // check if there's min or max:
    match (&f.maximum, &f.minimum) {
        (None, None) => {},
        (None, Some(min)) => {
            add_check_for_max_or_min(&mut out_str, f, *min, "<");
        }
        (Some(max), None) => {
            add_check_for_max_or_min(&mut out_str, f, *max, ">");
        }
        (Some(max), Some(min)) => {
            add_check_for_max_or_min(&mut out_str, f, *max, ">");
            add_check_for_max_or_min(&mut out_str, f, *min, "<");
        }
    }

    out_str
}

pub fn emit_struct(validations: &mut String, use_map_tracker: &mut HashMap<String, Vec<String>>, use_name: &str, s: &CfnStruct) -> String {
    let mut fields = "".to_string();
    let mut field_enums = HashMap::new();
    for f in s.fields.iter() {
        // add to the use tracker to say that
        // this current struct uses that other custom type
        if is_custom_type(&f.ty) {
            validations.push_str(&emit_field_validation(true, f));
            add_or_insert_to_use_map(use_map_tracker, s.name.clone(), f.ty.clone());
        } else {
            validations.push_str(&emit_field_validation(false, f));
        }
        fields.push_str(&emit_field(&mut field_enums, use_map_tracker, &s.name, f));
    }
    let doc_comment = emit_doc_comments("", &s.documentation);
    let field_enums = emit_field_enums(field_enums);

format!("
{doc_comment}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct {} {{
{fields}
}}

{field_enums}
", use_name)
}

pub fn emit_doc_comments(indent: &str, f: &String) -> String {
    let mut out = "".to_string();
    let mut last_was_empty = true;
    for line in f.lines() {
        out.push('\n');
        if !last_was_empty && !line.is_empty() {
            // add a newline
            out.push_str(&format!("{indent}///\n"));
        }
        out.push_str(&format!("{indent}/// {line}"));
        last_was_empty = line.is_empty();
    }
    out
}

pub fn emit_field(
    field_enums: &mut HashMap<String, Vec<String>>,
    use_map_tracker: &mut HashMap<String, Vec<String>>,
    owning_struct_name: &str,
    f: &CfnField
) -> String {
    let docs = emit_doc_comments("    ", &f.documentation);
    let mut skip_if_none = "";
    // if its not required that means we wrap it with an Option, and in
    // that case we dont want to serialize if its none
    if !f.required {
        skip_if_none = "    #[serde(skip_serializing_if = \"Option::is_none\")]";
    }
format!("
{docs}
    #[serde(rename = \"{}\")]
{skip_if_none}
    pub {}: {},
", f.name, f.get_field_name(), f.get_type(field_enums, owning_struct_name, use_map_tracker))
}

pub fn emit_cargo_toml(base_path: &str, service_name: &String, extra_deps: &str) {
    let data = format!(r#"
[package]
name = "{service_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
cfn_resources = {{ path = "../cfn_resources" }}
{extra_deps}
"#);

    let out = format!("{base_path}/Cargo.toml");
    if let Err(e) = std::fs::write(&out, data) {
        panic!("Failed to write {out}\n{:?}", e);
    }
}

pub fn load_service() -> (Zipped, Zipped) {
    let resource_provider_data = get_cfn_resource_provider_schema("us-east-2", true).expect("Failed to get cfn resource provider schema");
    let resource_spec_data = get_cfn_resource_spec("us-east-2", true).expect("Failed to get cfn resource spec");
    (resource_spec_data, resource_provider_data)
}

pub fn download_and_iterate(cb: &mut impl FnMut(String, CfnResourceProviderSchema, CfnResourceSpecSchema)) {
    let (mut resource_spec, mut resource_provider) = load_service();
    let provider_file_names: Vec<String> = resource_provider.file_names().map(|x| x.to_string()).collect();
    for serv in provider_file_names {
        let mut zipfile = resource_provider.by_name(&serv).expect(&format!("Failed to extract {serv}"));
        let mut s = String::new();
        if let Err(e) = zipfile.read_to_string(&mut s) {
            panic!("Failed to read zip file {}\n{:?}", serv, e);
        }
        let mut provider_schema = get_schema(&serv, &s);
        let serv_name = get_service_name(&serv, &mut provider_schema);
        let spec_file_name = format!("{}{}Specification.json", serv_name, provider_schema.item_name);
        let mut zipfile2 = resource_spec.by_name(&spec_file_name).expect(&format!("Failed to extract assumed spec file {spec_file_name} while reading {serv}"));
        let mut s = String::new();
        if let Err(e) = zipfile2.read_to_string(&mut s) {
            panic!("Failed to read zip file {}\n{:?}", spec_file_name, e);
        }
        let spec_schema = get_spec_schema(&spec_file_name, &s);
        cb(serv_name, provider_schema, spec_schema)
    }
}


pub fn convert_snake_case(name: &str) -> String {
    let mut out = "".to_string();
    let mut last_was_cap = false;
    for (i, c) in name.chars().enumerate() {
        if i == 0 {
            out.push(c.to_ascii_lowercase());
            if c.is_ascii_uppercase() {
                last_was_cap = true;
            }
            continue;
        }
        if c.is_ascii_uppercase() && !last_was_cap {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
        last_was_cap = c.is_ascii_uppercase();
    }
    out
}

pub fn map_primitive_type(s: &str) -> String {
    match s {
        "String" => s.to_string(),
        "Long" => "i64".to_string(),
        "Integer" => "i64".to_string(),
        "Double" => "f64".to_string(),
        "Boolean" => "bool".to_string(),
        "Timestamp" => "String".to_string(),
        // this is Json
        _ => "serde_json::Value".to_string(),
    }
}

/// returns true if the type is not a wrapper type like Vec or HashMap,
/// and also not a primitive type
pub fn is_custom_type(name: &str) -> bool {
    match name {
        "i64" | "String" | "f64" | "bool" | "serde_json::Value" => false,
        x => {
            if x.starts_with("Vec<") || x.starts_with("std::") {
                return false;
            }
            true
        }
    }
}

pub fn get_type_from_prop<T: PropertyLike>(prop: &T) -> String {
    // String, Long, Integer, Double, Boolean, Timestamp or Json
    let prim = prop.primitive_type();
    match prim {
        "String" | "Long" | "Integer" | "Double" | "Boolean" | "Timestamp" | "Json" => {
            return map_primitive_type(prim);
        }
        // not primitive, so move on.
        _ => {}
    }
    let typ = prop.typ();
    let prim_item_type = prop.primitive_item_type();
    let item_type = prop.item_type();

    let resolve_inner_item_type = || -> String {
        if !item_type.is_empty() {
            return item_type.to_string();
        }
        // otherwise we assume that primitive item type is set,
        // so we return the mapping:
        map_primitive_type(prim_item_type)
    };

    match typ {
        "List" => format!("Vec<{}>", resolve_inner_item_type()),
        "Map" => format!("std::collections::HashMap<String, {}>", resolve_inner_item_type()),
        // if its not list or map, use it as is:
        x => x.to_string(),
    }
}

pub fn parse_valid_values(doc: &str) -> Option<Vec<String>> {
    // Valid Values: | Allowed values: | Allowed Values: | 
    // ^ if found empty list, must ignore, and change type back to string (this means newline)
    if let Some((_, right)) = doc.split_once("Valid Values:") {
        return parse_valid_values_line(right);
    }
    // try Allowed values:
    if let Some((_, right)) = doc.split_once("Allowed values:") {
        return parse_valid_values_line(right);
    }

    // try Allowed Values:
    if let Some((_, right)) = doc.split_once("Allowed Values:") {
        return parse_valid_values_line(right);
    }

    None
}

pub fn trim_surrounding_quotes(s: &mut String) {
    while s.starts_with('"') && s.ends_with('"') {
        s.remove(0);
        s.pop();
    }
}

pub fn parse_valid_values_line(line: &str) -> Option<Vec<String>> {
    let mut out = vec![];
    if let Some((left, _)) = line.split_once("\n") {
        let left = left.trim();
        if left.is_empty() { return None; }

        let pipe_split = left.split("|");
        for word in pipe_split {
            let mut valid_value = word.trim().to_string();
            trim_surrounding_quotes(&mut valid_value);
            out.push(valid_value);
        }
    }
    if !out.is_empty() {
        Some(out)
    } else {
        None
    }
}

pub fn parse_min_max_values(doc: &str) -> (Option<f64>, Option<f64>) {
    let mut max_value = None;
    let mut min_value = None;
    if let Some((_, right)) = doc.split_once("Maximum:") {
        if let Some((left, _)) = right.split_once("\n") {
            let left = left.trim();
            if let Ok(num) = left.parse::<f64>() {
                max_value = Some(num);
            }
        }
    }
    if let Some((_, right)) = doc.split_once("Minimum:") {
        if let Some((left, _)) = right.split_once("\n") {
            let left = left.trim();
            if let Ok(num) = left.parse::<f64>() {
                min_value = Some(num);
            }
        }
    }
    (min_value, max_value)
}

pub fn prop_to_cfn_field<T: PropertyLike>(name: &str, prop: &T) -> CfnField {
    let doc = prop.doc().to_string();
    let (minimum, maximum) = parse_min_max_values(&doc);
    CfnField {
        name: name.to_string(),
        required: prop.required(),
        ty: get_type_from_prop(prop),
        valid_values: parse_valid_values(&doc),
        documentation: doc,
        maximum,
        minimum,
        pattern: None,
    }
    // Maximum:
    // Minimum:
    // Pattern:
}

pub fn resource_type_to_cfn_struct<T: PropertyLike>(name: &str, doc: &str, props: &HashMap<String, T>) -> CfnStruct {
    let name = get_last_delimiter_part(name);
    let mut fields = vec![];
    for (prop_name, prop) in props.iter() {
        fields.push(prop_to_cfn_field(prop_name, prop));
    }
    // make fields deterministic
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    CfnStruct {
        name,
        documentation: doc.to_string(),
        fields
    }
}


pub fn get_last_delimiter_part(s: &str) -> String {
    let mut out = "".to_string();
    for c in s.chars().rev() {
        if c == '.' || c == ':' {
            break;
        }
        out.insert(0, c);
    }
    out
}

pub fn spec_to_service(service_name: String, spec: CfnResourceSpecSchema) -> ServiceCrate {
    let mut opt = None;
    for (resource_name, resource_type) in spec.resource_type.iter() {
        opt = Some((resource_name, resource_type));
    }
    let (resource_name, resource_type) = match opt {
        Some(x) => x,
        None => panic!("Failed to extract resource type for {service_name}"),
    };
    let mut part_split = resource_name.rsplit(":");
    let last_part = if let Some(a) = part_split.next() {
        a
    } else {
        panic!("Failed to extract resource name for {service_name} - {resource_name}");
    };
    let module_name = last_part;
    let entrypoint_struct = resource_type_to_cfn_struct(last_part, &resource_type.documentation, &resource_type.properties);
    let mut auxiliary_structs = vec![];
    for (name, prop) in spec.property_types.iter() {
        auxiliary_structs.push(resource_type_to_cfn_struct(&name, &prop.documentation, &prop.properties));
    }
    // make structs deterministic
    auxiliary_structs.sort_by(|a, b| a.name.cmp(&b.name));
    ServiceCrate {
        name: convert_snake_case(&service_name),
        module: ModuleDef {
            name: module_name.to_string(),
            doc_comment: resource_type.documentation.to_string(),
            entrypoint_struct,
            auxiliary_structs,
            resource_name: resource_name.to_string(),
        }
    }
}

fn main() {
    if let Ok(data) = std::fs::read_to_string("temp.json") {
        let data: Vec<(String, CfnResourceSpecSchema)> = serde_json::from_str(&data).expect("Failed to deserialize");
        for (name, spec) in data {
            let service = spec_to_service(name, spec);
            service.output();
        }
        std::process::exit(0);
    }
    println!("Dont have temp.json, going to fetch it");
    let mut all_specs = vec![];
    download_and_iterate(&mut |serv_name, _provider, spec| {
        all_specs.push((serv_name, spec));
    });
    all_specs.par_iter_mut()
        .for_each(|(_serv_name, x)| {
            let mut cache = html_extractor::HtmlCache::default();
            x.resolve_all_doc_links(&mut |resource_name, doc_link| {
                if !doc_link.starts_with("http://") {
                    // assume we already handled this one, so skip it.
                    return;
                }
                // skip resources that dont have cloudformation anymore
                if resource_name.contains("::Shield") {
                    return;
                }
                // custom fix: the URL in their generated file is wrong.
                if doc_link == "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html#cfn-ec2-securitygroup-ingress-sourceprefixlistid" {
                    *doc_link = "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule-1.html#cfn-ec2-securitygroup-ingress-sourceprefixlistid".to_string();
                }
                if doc_link.starts_with("http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-group-mixedinstancespolicy.html") {
                    *doc_link = doc_link.replace("http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-group-mixedinstancespolicy.html", "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-mixedinstancespolicy.html");
                }
                if doc_link.starts_with("http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy") {
                    *doc_link = doc_link.replace("http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-as-mixedinstancespolicy", "http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup");
                }
                *doc_link = doc_link.replace("http://", "https://");
                let doc_comment = extract_documentation(&mut cache, &doc_link);
                *doc_link = doc_comment;
            });
        });
    let serialized = serde_json::to_string(&all_specs).expect("Failed to serialize!!!!!");
    let _ = std::fs::write("temp.json", serialized);
    println!("Num specs {}", all_specs.len());
}
