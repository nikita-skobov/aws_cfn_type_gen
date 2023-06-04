/// The AWS::Glue::SchemaVersion is an AWS Glue resource type that manages schema versions of schemas in the AWS Glue Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub schema_definition: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_version_id: CfnSchemaVersionversionid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSchemaVersionversionid;
impl CfnSchemaVersionversionid {
    pub fn att_name(&self) -> &'static str {
        r#"VersionId"#
    }
}

impl cfn_resources::CfnResource for CfnSchemaVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::SchemaVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.schema.validate()?;

        Ok(())
    }
}

/// A wrapper structure to contain schema identity fields. Either SchemaArn, or SchemaName and RegistryName has to be provided.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub registry_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the schema. Either SchemaArn, or SchemaName and RegistryName has to be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub schema_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the schema. Either SchemaArn, or SchemaName and RegistryName has to be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub schema_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Schema {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
