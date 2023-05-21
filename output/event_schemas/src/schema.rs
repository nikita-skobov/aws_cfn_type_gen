

/// Use the AWS::EventSchemas::Schema resource to specify an event       schema.
#[derive(Default, serde::Serialize)]
pub struct CfnSchema {


    /// 
    /// Tags associated with the schema.
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
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryName")]
    pub registry_name: String,


    /// 
    /// The name of the schema.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,


    /// 
    /// The type of schema.
    /// 
    /// Valid types include OpenApi3 and JSONSchemaDraft4.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The source of the schema definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: String,


    /// 
    /// A description of the schema.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// Tags to associate with the schema.
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
