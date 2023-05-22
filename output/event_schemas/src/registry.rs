/// Use the AWS::EventSchemas::Registry to specify a schema registry. Schema       registries are containers for Schemas. Registries collect and organize schemas so that       your schemas are in logical groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the schema registry.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<cfn_resources::StrVal>,

    ///
    /// Tags to associate with the registry.
    ///
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagsEntry>>,

    #[serde(skip_serializing)]
    pub att_registry_arn: CfnRegistryregistryarn,

    #[serde(skip_serializing)]
    pub att_registry_name: CfnRegistryregistryname,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRegistryregistryarn;
impl CfnRegistryregistryarn {
    pub fn att_name(&self) -> &'static str {
        r#"RegistryArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRegistryregistryname;
impl CfnRegistryregistryname {
    pub fn att_name(&self) -> &'static str {
        r#"RegistryName"#
    }
}

impl cfn_resources::CfnResource for CfnRegistry {
    fn type_string(&self) -> &'static str {
        "AWS::EventSchemas::Registry"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Tags to associate with the schema registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
