

/// The AWS::Glue::SchemaVersionMetadata is an AWS Glue resource type that defines the metadata key-value pairs for a schema version in AWS Glue Schema Registry.
#[derive(Default, serde::Serialize)]
pub struct CfnSchemaVersionMetadata {


    /// 
    /// A metadata key's corresponding value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The version number of the schema.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: String,


    /// 
    /// A metadata key in a key-value pair for metadata.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: String,

}
