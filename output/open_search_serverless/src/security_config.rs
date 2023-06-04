/// Specifies a security configuration for OpenSearch Serverless. For more information, see       SAML         authentication for Amazon OpenSearch Serverless.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSecurityConfig {
    ///
    /// The description of the security configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the security configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// SAML options for the security configuration in the form of a key-value map.
    ///
    /// Required: No
    ///
    /// Type: SamlConfigOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamlOptions")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub saml_options: Option<SamlConfigOptions>,

    ///
    /// The type of security configuration. Currently the only option is saml.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cfn_type: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_id: CfnSecurityConfigid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSecurityConfigid;
impl CfnSecurityConfigid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnSecurityConfig {
    fn type_string(&self) -> &'static str {
        "AWS::OpenSearchServerless::SecurityConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.saml_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes SAML options for an OpenSearch Serverless security configuration in the form of a key-value       map.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SamlConfigOptions {
    ///
    /// The group attribute for this SAML integration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupAttribute")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub group_attribute: Option<cfn_resources::StrVal>,

    ///
    /// The XML IdP metadata file generated from your identity provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: cfn_resources::StrVal,

    ///
    /// The session timeout, in minutes. Default is 60 minutes (12 hours).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeout")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub session_timeout: Option<i64>,

    ///
    /// A user attribute for this SAML integration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAttribute")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_attribute: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SamlConfigOptions {
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
