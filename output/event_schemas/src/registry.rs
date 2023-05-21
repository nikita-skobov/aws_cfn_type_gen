

/// Use the AWS::EventSchemas::Registry to specify a schema registry. Schema       registries are containers for Schemas. Registries collect and organize schemas so that       your schemas are in logical groups.
#[derive(Default, serde::Serialize)]
pub struct CfnRegistry {


    /// 
    /// A description of the registry to be created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Tags to associate with the registry.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,


    /// 
    /// The name of the schema registry.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,

}


/// Tags to associate with the schema registry.
#[derive(Default, serde::Serialize)]
pub struct TagsEntry {


    /// 
    /// They value of a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// They key of a key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}
