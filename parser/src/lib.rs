use std::{collections::HashMap, io::Read};

use downloader::{Zipped, get_cfn_resource_provider_schema};

use serde::{Deserialize, Serialize};

/// the schema of the data that cloudformation provides from these files:
/// https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-resource-specification-format.html
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug)]
pub struct CfnResourceSpecSchema {
    pub property_types: HashMap<String, Properties>,
    pub resource_specification_version: String,
    // pub resource_types: Option<HashMap<String, ResourceType>>,
    pub resource_type: HashMap<String, ResourceType>,
}

impl CfnResourceSpecSchema {
    /// callback takes: name of resource, doc link
    /// callback should resolve, and modify in place each doc link string
    /// to be the actual documentation rather than a link
    pub fn resolve_all_doc_links(&mut self, cb: &mut impl FnMut(&String, &mut String)) {
        for (name, resource_type) in self.resource_type.iter_mut() {
            cb(name, &mut resource_type.documentation);
            for (_prop_name, prop) in resource_type.properties.iter_mut() {
                cb(name, &mut prop.documentation);
                for (_prop_name, prop) in prop.properties.iter_mut() {
                    cb(name, &mut prop.documentation);
                }
            }

            // we know/assume that resource type only has 1 item.
            // so what we do next is we also iterate over the properties, but we do it from within this loop
            // because we still have access to the resource name, which we pass to the callback.
            for (_prop_name, prop) in self.property_types.iter_mut() {
                cb(name, &mut prop.documentation);
                for (_prop_name, prop) in prop.properties.iter_mut() {
                    cb(name, &mut prop.documentation);
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug)]
pub struct ResourceType {
    pub attributes: HashMap<String, Attribute>,
    pub documentation: String,
    pub properties: HashMap<String, Properties>,
    pub additional_properties: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug)]
pub struct Attribute {
    pub item_type: String,
    #[serde(rename = "Type")]
    pub ty: String,
    pub primitive_item_type: String,
    pub primitive_type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug, Clone)]
pub struct PropertyTypeSpec {
    pub documentation: String,
    pub duplicates_allowed: bool,
    pub item_type: String,
    pub required: bool,
    #[serde(rename = "Type")]
    pub ty: String,
    pub update_type: String,
    pub primitive_item_type: String,
    pub primitive_type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug)]
pub struct Properties {
    pub documentation: String,
    pub required: bool,
    pub duplicates_allowed: bool,
    #[serde(rename = "Type")]
    pub ty: String,
    pub properties: HashMap<String, PropertyTypeSpec>,
    pub update_type: String,
    pub primitive_item_type: String,
    pub primitive_type: String,
    pub item_type: String,
}

pub trait PropertyLike {
    fn doc(&self) -> &str;
    fn required(&self) -> bool;
    fn duplicates_allowed(&self) -> bool;
    fn typ(&self) -> &str;
    fn primitive_type(&self) -> &str;
    fn primitive_item_type(&self) -> &str;
    fn item_type(&self) -> &str;
    fn update_type(&self) -> &str;
    fn nested_properties(&self) -> HashMap<String, PropertyTypeSpec>;
}

impl PropertyLike for PropertyTypeSpec {
    fn doc(&self) -> &str {
        &self.documentation
    }

    fn required(&self) -> bool {
        self.required
    }

    fn duplicates_allowed(&self) -> bool {
        self.duplicates_allowed
    }

    fn typ(&self) -> &str {
        &self.ty
    }

    fn primitive_type(&self) -> &str {
        &self.primitive_type
    }

    fn primitive_item_type(&self) -> &str {
        &self.primitive_item_type
    }

    fn item_type(&self) -> &str {
        &self.item_type
    }

    fn update_type(&self) -> &str {
        &self.update_type
    }

    fn nested_properties(&self) -> HashMap<String, PropertyTypeSpec> {
        HashMap::new()
    }
}

impl PropertyLike for Properties {
    fn doc(&self) -> &str {
        &self.documentation
    }

    fn required(&self) -> bool {
        self.required
    }

    fn duplicates_allowed(&self) -> bool {
        self.duplicates_allowed
    }

    fn typ(&self) -> &str {
        &self.ty
    }

    fn primitive_type(&self) -> &str {
        &self.primitive_type
    }

    fn primitive_item_type(&self) -> &str {
        &self.primitive_item_type
    }

    fn item_type(&self) -> &str {
        &self.item_type
    }

    fn update_type(&self) -> &str {
        &self.update_type
    }

    fn nested_properties(&self) -> HashMap<String, PropertyTypeSpec> {
        self.properties.clone()
    }
}

pub fn get_spec_schema(name: &str, data: &String) -> CfnResourceSpecSchema {
    match serde_json::from_str::<CfnResourceSpecSchema>(&data) {
        Ok(o) => o,
        Err(e) => {
            panic!("Failed to deserialize {name}\n{:?}", e);
        }
    }
}

/// the schema of the data that cloudformation provides from these files:
/// https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/resource-type-schemas.html
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default, Debug)]
pub struct CfnResourceProviderSchema {
    // this isnt in the schema. we form this after parsing
    pub item_name: String,

    // simple metadata stuff:
    pub type_name: String,
    pub description: String,
    pub additional_properties: bool,
    pub taggable: bool,
    #[serde(rename = "$comment")]
    pub comment: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "type")]
    pub ty: TypeWrapper,
    pub title: String,

    // meat and potatoes:
    pub definitions: HashMap<String, Property>,
    pub properties: HashMap<String, Property>,

    // extra complex metadata. defines which fields are
    // inputs/outputs, permissions, etc.
    pub required: Vec<String>,
    pub all_of: Vec<Requirement>,
    pub any_of: Vec<Requirement>,
    pub one_of: Vec<Requirement>,
    pub handlers: Handlers,
    pub create_only_properties: Vec<String>,
    pub primary_identifier: Vec<String>,
    pub additional_identifiers: Vec<Vec<String>>,
    pub read_only_properties: Vec<String>,
    pub write_only_properties: Vec<String>,
    pub deprecated_properties: Vec<String>,
    pub conditional_create_only_properties: Vec<String>,
    pub non_public_properties: Vec<String>,
    pub non_public_definitions: Vec<String>,
    pub tagging: Tagging,
    pub replacement_strategy: String,

    // stuff i dont care about:
    pub source_url: String,
    pub resource_link: serde_json::Value,
    pub documentation_url: String,
    pub remote: serde_json::Value,
    pub property_transform: serde_json::Value,
    pub type_configuration: serde_json::Value,
}

impl CfnResourceProviderSchema {
    /// checks the list of "read_only_properties".
    /// if this field name is in the read only list,
    /// that means this field is not an input, but rather
    /// an output that can be later retrieved via GetAtt
    pub fn field_is_input(&self, field_name: &str) -> bool {
        for name in self.read_only_properties.iter() {
            if name.ends_with(field_name) {
                return false;
            }
        }
        true
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[derive(Default, Debug)]
pub struct Property {
    pub insertion_order: bool,
    pub array_type: ArrayType,
    pub title: String,
    pub description: String,
    pub examples: serde_json::Value,
    pub default: DefaultType,
    pub multiple_of: f64,
    pub maximum: f64,
    pub exclusive_maximum: f64,
    pub minimum: f64,
    pub exclusive_minimum: f64,
    pub max_length: usize,
    pub min_length: usize,
    pub pattern: String,
    pub max_items: usize,
    pub min_items: usize,
    pub unique_items: bool,
    pub required: Vec<String>,
    pub all_of: Vec<Property>,
    pub any_of: Vec<Property>,
    pub one_of: Vec<Property>,
    pub format: String,
    pub contains: Vec<String>,
    pub max_properties: usize,
    pub min_properties: usize,
    pub properties: HashMap<String, Property>,
    pub additional_properties: bool,
    pub dependencies: HashMap<String, serde_json::Value>,
    #[serde(rename = "type")]
    pub ty: Option<TypeWrapper>,
    #[serde(rename = "enum")]
    pub enm: PropertyType,
    #[serde(rename = "const")]
    pub cnst: String,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,

    /// this is a hack. the items schema is the same as the properties.
    /// but we cant just do items: Property
    /// because thats a cyclical data structure and rust complains.
    /// instead the hack becomes:
    /// capture all extra fields from this struct and put them into a hashmap.
    /// we expect to only find 1 field here which should be "items".
    /// at parse time we perform a runtime check to ensure the only extra field here is "items"
    #[serde(flatten)]
    pub items: HashMap<String, Property>,

    #[serde(rename = "$comment")]
    pub comment: String,
    pub pattern_properties: serde_json::Value,
}

impl Property {
    pub fn get_reference_name(&self) -> Option<String> {
        if let Some(r) = &self.reference {
            let mut split = r.rsplit("/");
            if let Some(f) = split.next() {
                return Some(f.to_string());
            }
        }
        None
    }

    /// if this is an array (type: Array)
    /// return the name of the item that the array consists of
    pub fn get_item_name(&self) -> Option<String> {
        if let Some(ty) = &self.ty {
            match ty {
                TypeWrapper::Single(Type::Array) => {
                    if let Some(item) = self.items.get("items") {
                        return item.get_reference_name();
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_item(&self) -> Option<&Property> {
        self.items.get("items")
    }

    /// if this type can be represented by some type in rust's standard library,
    /// we use it. for example, String, f64, Vec<String>, etc.
    pub fn get_simple_type(&self) -> Option<String> {
        match &self.ty {
            Some(TypeWrapper::Single(x)) => {
                match x {
                    Type::Array => {
                        if let Some(item) = self.get_item() {
                            if let Some(simple) = item.get_simple_type() {
                                return Some(format!("Vec<{}>", simple));
                            }
                        }
                    }
                    Type::String => return Some("String".to_string()),
                    Type::Boolean => return Some("bool".to_string()),
                    Type::Integer => return Some("usize".to_string()),
                    Type::Number => return Some("f64".to_string()),
                    Type::Null => panic!("{:#?}\n\nI have a single simple Null type??", self),
                    Type::Object => {},
                }
            }
            _ => {}
        }
        None
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Type {
    String,
    Object,
    Array,
    Boolean,
    Integer,
    Number,
    Null,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum TypeWrapper {
    Single(Type),
    Multiple(Vec<Type>),
}
impl Default for TypeWrapper {
    fn default() -> Self {
        Self::Single(Type::String)
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum ArrayType {
    Standard,
    AttributeList,
}

impl Default for ArrayType {
    fn default() -> Self {
        ArrayType::Standard
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Requirement {
    Required(Vec<String>),
    AllOf(Vec<Requirement>),
    OneOf(Vec<Requirement>),
    AnyOf(Vec<Requirement>),
}

impl Default for Requirement {
    fn default() -> Self {
        Self::Required(vec![])
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default)]
pub struct Handlers {
    pub create: HandlerDef,
    pub read: HandlerDef,
    pub update: HandlerDef,
    pub delete: HandlerDef,
    pub list: HandlerDef,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default)]
pub struct HandlerDef {
    pub permissions: Vec<String>,
    pub timeout_in_minutes: u32,
    // i dont care:
    pub handler_schema: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum PropertyType {
    String(String),
    VecStrings(Vec<String>),
    VecNums(Vec<f64>),
}
impl Default for PropertyType {
    fn default() -> Self {
        Self::String("".to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum DefaultType {
    Bool(bool),
    String(String),
    Num(f64),
    Map(HashMap<String, DefaultType>),
    Null(Option<()>),
    List(Vec<DefaultType>),
}
impl Default for DefaultType {
    fn default() -> Self {
        Self::Null(None)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
#[derive(Default)]
pub struct Tagging {
    pub taggable: bool,
    pub tag_on_create: bool,
    pub tag_updatable: bool,
    pub cloud_formation_system_tags: bool,
    pub tag_property: String,
}

pub fn iterate_all_zip_files(mut contents: Zipped, cb: &mut impl FnMut(&str, String)) {
    let mut filenames = contents.file_names().map(|x| x.to_string()).collect::<Vec<String>>();
    filenames.sort();
    for filename in filenames.iter() {
        match contents.by_name(&filename) {
            Ok(mut file) => {
                let mut s = String::new();
                if let Ok(_) = file.read_to_string(&mut s) {
                    cb(filename, s);
                }
            }
            Err(e) => {
                panic!("REEEE\n{:?}", e);
            }
        }
        // TODO: what if we fail?
    }
}

pub fn get_schema(name: &str, data: &String) -> CfnResourceProviderSchema {
    match serde_json::from_str::<CfnResourceProviderSchema>(data) {
        Err(e) => {
            panic!("Failed to deserialize {name}\n{:?}", e);
        }
        Ok(o) => o,
    }
}

pub fn get_service_name(name: &str, schema: &mut CfnResourceProviderSchema) -> String {
    let mut segments = schema.type_name.split("::");
    let _ = segments.next()
        .unwrap_or_else(|| {
            panic!("Failed to get first type name segment of {} From {}", schema.type_name, name);
        });
    // assume first == "AWS"
    let second = segments.next()
        .unwrap_or_else(|| {
            panic!("Failed to get second type name segment of {} From {}", schema.type_name, name);
        });
    let third = segments.next()
        .unwrap_or_else(|| {
            panic!("Failed to get item name of {} From {}", schema.type_name, name);
        });
    schema.item_name = third.to_string();
    second.to_string()
}

pub fn sort_by_service(name: &str, mut schema: CfnResourceProviderSchema, map: &mut HashMap<String, Vec<CfnResourceProviderSchema>>) {
    let name = get_service_name(name, &mut schema);
    if let Some(existing) = map.get_mut(&name) {
        existing.push(schema);
    } else {
        map.insert(name.clone(), vec![schema]);
    }
}

pub fn get_all_provider_schemas(
    region: &str,
    cache: bool
) -> HashMap<String, Vec<CfnResourceProviderSchema>> {
    let zip_archive = get_cfn_resource_provider_schema(region, cache).expect("Failed to get schema file");
    let mut map = HashMap::new();
    iterate_all_zip_files(zip_archive, &mut |name, data| {
        let schema = get_schema(name, &data);
        for (pro_name, prop) in schema.properties.iter() {
            if prop.items.len() > 1 {
                panic!("Unexpected extra key in the items map for {} : {}\n{:?}", name, pro_name, prop.items);
            }
            if prop.reference.is_some() {
                if let Some(typ) = &prop.ty {
                    match typ {
                        // this is the only case where
                        // having both $ref and type is allowed.
                        // because $ref implies type is object
                        TypeWrapper::Single(Type::Object) => {}
                        _ => {
                            panic!("{} at {} has both $ref and type.\n{:#?}", name, pro_name, prop);
                        }
                    }
                }
            }
        }
        sort_by_service(name, schema, &mut map);
    });
    map
}


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn all_files_deser_correctly() {
//         get_all_provider_schemas("us-east-2", true);
//     }
// }
