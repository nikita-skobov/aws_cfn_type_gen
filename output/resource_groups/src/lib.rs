
pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceQuery")]
    pub resource_query: Option<ResourceQuery>,
    /// The name of the resource group
    #[serde(rename = "Name")]
    pub name: String,
    /// The description of the resource group
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<Configuration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct TagFilter {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Query {
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "StackIdentifier")]
    pub stack_identifier: Option<String>,
    #[serde(rename = "ResourceTypeFilters")]
    pub resource_type_filters: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationParameter {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceQuery {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Query")]
    pub query: Option<Query>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationItem {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ConfigurationParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct Configuration {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}
