use std::collections::HashMap;

use parser::{CfnResourceProviderSchema, Property};

pub const INDENT: &str = "    ";

pub fn emit() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = format!("{base_dir}/../output");
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        panic!("Failed to create output dir {out_dir}\n{:?}", e);
    }

    let schemas = parser::get_all_provider_schemas("us-east-2", true);
    for (service_name, types) in schemas {
        // println!("{service_name}");
        emit_service(&out_dir, service_name, types);
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

pub fn emit_service(out_dir: &str, service_name: String, types: Vec<CfnResourceProviderSchema>) {
    let service_name_safe = convert_snake_case(&service_name);
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
    emit_lib_file(&src_dir, &service_name_safe, types);
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
{extra_deps}
"#);

    let out = format!("{base_path}/Cargo.toml");
    if let Err(e) = std::fs::write(&out, data) {
        panic!("Failed to write {out}\n{:?}", e);
    }
}

pub fn emit_lib_file(base_path: &str, service_name: &String, types: Vec<CfnResourceProviderSchema>) {
    let contents = emit_lib_file_contents(service_name, types);
    let out = format!("{base_path}/lib.rs");
    if let Err(e) = std::fs::write(&out, contents) {
        panic!("Failed to write {out}\n{:?}", e);
    }
}

pub fn emit_lib_file_contents(service_name: &String, types: Vec<CfnResourceProviderSchema>) -> String {
    let mut out = "".to_string();
    for t in types {
        emit_type_definition(&mut out, service_name, t);
    }
    out
}

pub fn emit_type_definition(out: &mut String, _service_name: &String, schema: CfnResourceProviderSchema) {
    let item_name = &schema.item_name;
    let mod_name = convert_snake_case(&item_name);
    let item_defs = emit_property_defs(&item_name, &schema);
    let helper_items = emit_helper_defs(&schema);
    let item = format!(r#"
pub mod cfn_{mod_name} {{

#[derive(serde::Serialize, Default)]
pub struct Cfn{item_name} {{
{item_defs}
}}

{helper_items}

}}
"#);

    out.push_str(&item);
}


pub fn emit_helper_defs(schema: &CfnResourceProviderSchema) -> String {
    let mut out = "".to_string();
    for (helper_name, helper) in schema.definitions.iter() {
        out.push_str(&emit_helper_def(schema, helper_name, helper));
    }
    out
}

pub fn emit_property_helper_defs(schema: &CfnResourceProviderSchema, struct_item_name: &String, helper: &Property) -> String {
    let mut out = "".to_string();
    for (prop_name, prop) in helper.properties.iter() {
        out.push_str(&emit_property_helper_def(&schema.definitions, struct_item_name, helper, prop_name, prop));
    }
    out
}

pub fn emit_property_helper_def(all_deps: &HashMap<String, Property>, struct_item_name: &String, schema: &Property, helper_name: &String, helper: &Property) -> String {
    let snake_case_name = sanitize_snake_case_name(convert_snake_case(&helper_name));
    // let doc_str = emit_property_doc(schema, prop_name, prop);
    let mut typ = get_type_name(&struct_item_name, &helper, all_deps);
    if !is_required(helper_name, &schema.required) {
        typ = format!("Option<{typ}>")
    }
    // let mut out = format!("{doc_str}\n{INDENT}pub {snake_case_name}: {typ},\n");
    format!("{INDENT}#[serde(rename = \"{helper_name}\")]\n{INDENT}pub {snake_case_name}: {typ},\n")
}

pub fn emit_helper_def(schema: &CfnResourceProviderSchema, helper_name: &String, helper: &Property) -> String {
    // if it's a simple type, then output it as a type alias
    if let Some(simp) = helper.get_simple_type() {
        return format!("pub type {} = {};", helper_name, simp);
    }
    let item_name = helper_name;
    let item_defs = emit_property_helper_defs(schema, item_name, helper);
    // otherwise, it's a struct
    format!(r#"
#[derive(serde::Serialize, Default)]
pub struct {item_name} {{
{item_defs}
}}
"#)
}

pub fn emit_property_defs(this_item_name: &str, schema: &CfnResourceProviderSchema) -> String {
    let mut out = "".to_string();
    for (prop_name, prop_schema) in schema.properties.iter() {
        out.push_str(&emit_property_def(schema, this_item_name, prop_name, prop_schema));
    }
    out
}

pub fn get_property_description(schema: &CfnResourceProviderSchema, prop_name: &String, prop: &Property) -> String {
    // first check if its inlined already in the property:
    if !prop.description.is_empty() {
        return prop.description.to_string();
    }
    // check if its an array of items:
    // if so, let the description just be made up
    if let Some(child_name) = prop.get_item_name() {
        return format!("List of {child_name}")
    }
    // if we simply point to another item, then try to fetch the description of that:
    if let Some(ref_name) = &prop.reference {
        if let Some(def) = schema.definitions.get(ref_name) {
            return get_property_description(schema, ref_name, def);
        }
    }

    let plan_b = format!("No documentation provided by AWS");
    // uhhh... try to find it in the definitions map?
    if let Some(def) = schema.definitions.get(prop_name) {
        if def.description.is_empty() {
            return plan_b;
        } else {
            return def.description.clone();
        }
    }
    // otherwise give up passive aggressively
    return plan_b;    
}

pub fn emit_property_doc(schema: &CfnResourceProviderSchema, prop_name: &String, prop: &Property) -> String {
    let mut out = format!("");
    let description = get_property_description(schema, prop_name, prop);
    for line in description.lines() {
        let line = line.trim();
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&format!("{INDENT}/// {line}"));
    }
    out
}

pub fn is_required(name: &str, requireds: &Vec<String>) -> bool {
    for r in requireds {
        if name == r { return true }
    }
    false
}

fn sanitize_snake_case_name(snake_case_name: String) -> String {
    if snake_case_name == "type" {
        return "ty".to_string();
    }
    if snake_case_name == "match" {
        return "mtch".to_string();
    }
    if snake_case_name == "override" {
        return "overide".to_string();
    }
    if snake_case_name == "else" {
        return "els".to_string();
    }
    snake_case_name
}

pub fn get_type_name(this_item_name: &str, prop: &Property, all_deps: &HashMap<String, Property>) -> String {
    if let Some(simp) = prop.get_simple_type() {
        simp
    } else {
        // first, check if its a simple reference to some other struct.
        if let Some(other_obj_name) = prop.get_reference_name() {
            // now, need to lookup if that object has this property as a dependency.
            // if so: then its cyclical, and we must add indirection via Box.
            let mut is_cyclical = false;
            if let Some(dep) = all_deps.get(&other_obj_name) {
                for (_, prop_def) in dep.properties.iter() {
                    if let Some(ref_name) = prop_def.get_reference_name() {
                        if ref_name == this_item_name {
                            is_cyclical = true;
                            break;
                        }
                    }
                }
            }
            if is_cyclical {
                format!("Box<{other_obj_name}>")
            } else {
                other_obj_name
            }
        } else {
            // now check if maybe its a List of a complex type:
            if let Some(child_name) = prop.get_item_name() {
                format!("Vec<{child_name}>")
            } else {
                // TODO:...
                "()".to_string()
            }
        }
    }
}

pub fn emit_property_def(schema: &CfnResourceProviderSchema, this_item_name: &str, prop_name: &String, prop: &Property) -> String {
    // if this field is an output then we dont want it to be serializable as an input...
    if !schema.field_is_input(&prop_name) {
        return "".to_string();
    }
    let snake_case_name = sanitize_snake_case_name(convert_snake_case(&prop_name));

    // TODO: add serde renames to each item
    let doc_str = emit_property_doc(schema, prop_name, prop);
    let mut typ = get_type_name(this_item_name, &prop, &schema.definitions);
    if !is_required(&prop_name, &schema.required) {
        typ = format!("Option<{typ}>")
    }
    format!("{doc_str}\n{INDENT}#[serde(rename = \"{prop_name}\")]\n{INDENT}pub {snake_case_name}: {typ},\n")
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn it_works() {
    //     emit();
    // }
}
