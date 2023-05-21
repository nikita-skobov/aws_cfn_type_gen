/// The AWS::AppSync::ApiKey resource creates a unique key that you can distribute to clients who     are executing GraphQL operations with AWS AppSync that require an API key.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApiKey {
    ///
    /// Unique AWS AppSync GraphQL API ID for this API key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// The API key ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Unique description of your API key.
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
    /// The time after which the API key expires. The date is represented as seconds since the     epoch, rounded down to the nearest hour.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
}

impl cfn_resources::CfnResource for CfnApiKey {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::ApiKey"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
