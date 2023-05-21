

/// Specifies a security configuration for OpenSearch Serverless. For more information, see       SAML         authentication for Amazon OpenSearch Serverless.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecurityConfig {


    /// 
    /// The type of security configuration. Currently the only option is saml.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The name of the security configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The description of the security configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// SAML options for the security configuration in the form of a key-value map.
    /// 
    /// Required: No
    ///
    /// Type: SamlConfigOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamlOptions")]
    pub saml_options: Option<SamlConfigOptions>,

}



impl cfn_resources::CfnResource for CfnSecurityConfig {
    fn type_string() -> &'static str {
        "AWS::OpenSearchServerless::SecurityConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes SAML options for an OpenSearch Serverless security configuration in the form of a key-value       map.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SamlConfigOptions {


    /// 
    /// The XML IdP metadata file generated from your identity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: String,


    /// 
    /// The group attribute for this SAML integration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupAttribute")]
    pub group_attribute: Option<String>,


    /// 
    /// The session timeout, in minutes. Default is 60 minutes (12 hours).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeout")]
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
    pub user_attribute: Option<String>,

}


