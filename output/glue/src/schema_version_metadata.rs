

/// The AWS::Glue::SchemaVersionMetadata is an AWS Glue resource type that defines the metadata key-value pairs for a schema version in AWS Glue Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSchemaVersionMetadata {


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
    /// A metadata key's corresponding value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for CfnSchemaVersionMetadata {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::SchemaVersionMetadata"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}