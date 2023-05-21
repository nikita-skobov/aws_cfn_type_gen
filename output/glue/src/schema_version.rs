

/// The AWS::Glue::SchemaVersion is an AWS Glue resource type that manages schema versions of schemas in the AWS Glue Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSchemaVersion {


    /// 
    /// The schema that includes the schema version.
    /// 
    /// Required: Yes
    ///
    /// Type: Schema
    ///
    /// Update requires: Replacement
    #[serde(rename = "Schema")]
    pub schema: Schema,


    /// 
    /// The schema definition for the schema version.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaDefinition")]
    pub schema_definition: String,

}



impl cfn_resources::CfnResource for CfnSchemaVersion {
    fn type_string() -> &'static str {
        "AWS::Glue::SchemaVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A wrapper structure to contain schema identity fields. Either SchemaArn, or SchemaName and RegistryName has to be provided.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schema {


    /// 
    /// The name of the registry where the schema is stored. Either SchemaArn, or SchemaName and RegistryName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the schema. Either SchemaArn, or SchemaName and RegistryName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,


    /// 
    /// The name of the schema. Either SchemaArn, or SchemaName and RegistryName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,

}


