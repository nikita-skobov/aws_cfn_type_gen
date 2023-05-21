

/// The AWS::AppSync::ApiKey resource creates a unique key that you can distribute to clients who     are executing GraphQL operations with AWS AppSync that require an API key.
#[derive(Default, serde::Serialize)]
pub struct CfnApiKey {


    /// 
    /// The API key ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyId")]
    pub api_key_id: Option<String>,


    /// 
    /// Unique description of your API key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The time after which the API key expires. The date is represented as seconds since the     epoch, rounded down to the nearest hour.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expires")]
    pub expires: Option<f64>,


    /// 
    /// Unique AWS AppSync GraphQL API ID for this API key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,

}