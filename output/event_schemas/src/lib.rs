
pub mod cfn_discoverer {

#[derive(serde::Serialize, Default)]
pub struct CfnDiscoverer {
    /// No documentation provided by AWS
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "CrossAccount")]
    pub cross_account: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_registry {

#[derive(serde::Serialize, Default)]
pub struct CfnRegistry {
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_registry_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnRegistryPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: (),
    /// No documentation provided by AWS
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RevisionId")]
    pub revision_id: Option<String>,

}



}

pub mod cfn_schema {

#[derive(serde::Serialize, Default)]
pub struct CfnSchema {
    /// No documentation provided by AWS
    #[serde(rename = "RegistryName")]
    pub registry_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Content")]
    pub content: String,

}


#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
