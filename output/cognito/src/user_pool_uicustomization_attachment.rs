

/// The AWS::Cognito::UserPoolUICustomizationAttachment resource sets the UI    customization information for a user pool's built-in app UI.
///
/// You can specify app UI customization settings for a single client (with a specific     clientId) or for all clients (by setting the clientId to     ALL). If you specify ALL, the default configuration is used for    every client that has had no UI customization set previously. If you specify UI customization    settings for a particular client, it no longer falls back to the ALL    configuration.
///
/// Setting a logo image isn't supported from AWS CloudFormation. Use the Amazon Cognito     SetUICustomization API operation to set the image.
#[derive(Default, serde::Serialize)]
pub struct CfnUserPoolUICustomizationAttachment {


    /// 
    /// The client ID for the client app. You can specify the UI customization settings for a    single client (with a specific clientId) or for all clients (by setting the clientId to     ALL).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// The CSS values in the UI customization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CSS")]
    pub css: Option<String>,


    /// 
    /// The user pool ID for the user pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,

}