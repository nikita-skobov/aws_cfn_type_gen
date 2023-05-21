

/// The AWS::Glue::SchemaVersion is an AWS Glue resource type that manages schema versions of schemas in the AWS Glue Schema Registry.
#[derive(Default, serde::Serialize)]
pub struct CfnSchemaVersion {


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

}


/// A wrapper structure to contain schema identity fields. Either SchemaArn, or SchemaName and RegistryName has to be provided.
#[derive(Default, serde::Serialize)]
pub struct Schema {


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

}
