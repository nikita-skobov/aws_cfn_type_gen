use std::{collections::HashMap, io::Read};

use downloader::{Zipped, get_cfn_resource_provider_schema};

use serde::{Deserialize};

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
    pub ty: String,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
    pub ty: PropertyType,
    #[serde(rename = "enum")]
    pub enm: PropertyType,
    #[serde(rename = "const")]
    pub cnst: String,

    // i dont care:
    pub items: serde_json::Value,
    #[serde(rename = "$ref")]
    pub r: String,
    #[serde(rename = "$comment")]
    pub comment: String,
    pub pattern_properties: serde_json::Value,
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
        sort_by_service(name, schema, &mut map);
    });
    map
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_files_deser_correctly() {
        let zip_archive = get_cfn_resource_provider_schema("us-east-2", true).expect("Failed to");
        let mut map = HashMap::new();
        iterate_all_zip_files(zip_archive, &mut |name, data| {
            let schema = get_schema(name, &data);
            sort_by_service(name, schema, &mut map);
        });
        println!("{:#?}", map.keys());
    }
}
