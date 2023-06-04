/// Use the AWS::EventSchemas::Schema resource to specify an event       schema.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSchema {
    ///
    /// The source of the schema definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: cfn_resources::StrVal,

    ///
    /// A description of the schema.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the schema registry.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryName")]
    pub registry_name: cfn_resources::StrVal,

    ///
    /// The name of the schema.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<cfn_resources::StrVal>,

    ///
    /// Tags associated with the schema.
    ///
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagsEntry>>,

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
    pub cfn_type: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_schema_arn: CfnSchemaschemaarn,

    #[serde(skip_serializing)]
    pub att_schema_name: CfnSchemaschemaname,

    #[serde(skip_serializing)]
    pub att_schema_version: CfnSchemaschemaversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSchemaschemaarn;
impl CfnSchemaschemaarn {
    pub fn att_name(&self) -> &'static str {
        r#"SchemaArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSchemaschemaname;
impl CfnSchemaschemaname {
    pub fn att_name(&self) -> &'static str {
        r#"SchemaName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSchemaschemaversion;
impl CfnSchemaschemaversion {
    pub fn att_name(&self) -> &'static str {
        r#"SchemaVersion"#
    }
}

impl cfn_resources::CfnResource for CfnSchema {
    fn type_string(&self) -> &'static str {
        "AWS::EventSchemas::Schema"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Tags to associate with the schema.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagsEntry {
    ///
    /// They key of a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// They value of a key-value pair.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TagsEntry {
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
