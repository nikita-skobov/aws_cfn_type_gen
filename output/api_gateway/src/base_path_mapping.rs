/// The AWS::ApiGateway::BasePathMapping resource creates a base path that clients who call your API must use in the invocation URL.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnBasePathMapping {
    ///
    /// The base path name that callers of the API must provide as part of the URL after the domain name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BasePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<cfn_resources::StrVal>,

    ///
    /// The domain name of the BasePathMapping resource to be described.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<cfn_resources::StrVal>,

    ///
    /// The string identifier of the associated RestApi.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the associated stage.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnBasePathMapping {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::BasePathMapping"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
