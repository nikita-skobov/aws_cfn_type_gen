

/// The AWS::Cognito::IdentityPool resource creates an Amazon Cognito identity    pool.
///
/// To avoid deleting the resource accidentally from AWS CloudFormation, use DeletionPolicy     Attribute and the UpdateReplacePolicy Attribute to retain the resource on deletion or    replacement.
#[derive(Default, serde::Serialize)]
pub struct CfnIdentityPool {


    /// 
    /// The configuration options to be applied to the identity pool.
    /// 
    /// Required: No
    ///
    /// Type: PushSync
    ///
    /// Update requires: No interruption
    #[serde(rename = "PushSync")]
    pub push_sync: Option<PushSync>,


    /// 
    /// The name of your Amazon Cognito identity pool.
    /// 
    /// Minimum length: 1
    /// 
    /// Maximum length: 128
    /// 
    /// Pattern: [\w\s+=,.@-]+
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdentityPoolName")]
    pub identity_pool_name: Option<String>,


    /// 
    /// The Amazon Cognito user pools and their client IDs.
    /// 
    /// Required: No
    ///
    /// Type: List of CognitoIdentityProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoIdentityProviders")]
    pub cognito_identity_providers: Option<Vec<CognitoIdentityProvider>>,


    /// 
    /// The events to configure.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoEvents")]
    pub cognito_events: Option<serde_json::Value>,


    /// 
    /// Specifies whether the identity pool supports unauthenticated logins.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,


    /// 
    /// The Amazon Resource Names (ARNs) of the OpenID connect providers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_arns: Option<Vec<String>>,


    /// 
    /// Enables the Basic (Classic) authentication flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowClassicFlow")]
    pub allow_classic_flow: Option<bool>,


    /// 
    /// Configuration options for configuring Amazon Cognito streams.
    /// 
    /// Required: No
    ///
    /// Type: CognitoStreams
    ///
    /// Update requires: No interruption
    #[serde(rename = "CognitoStreams")]
    pub cognito_streams: Option<CognitoStreams>,


    /// 
    /// The Amazon Resource Names (ARNs) of the Security Assertion Markup Language (SAML)    providers.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamlProviderARNs")]
    pub saml_provider_arns: Option<Vec<String>>,


    /// 
    /// Key-value pairs that map provider names to provider app IDs.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedLoginProviders")]
    pub supported_login_providers: Option<serde_json::Value>,


    /// 
    /// The "domain" Amazon Cognito uses when referencing your users. This name acts as a    placeholder that allows your backend and the Amazon Cognito service to communicate about the    developer provider. For the DeveloperProviderName, you can use letters and    periods (.), underscores (_), and dashes (-).
    /// 
    /// Minimum length: 1
    /// 
    /// Maximum length: 100
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeveloperProviderName")]
    pub developer_provider_name: Option<String>,

}


/// CognitoStreams is a property of the AWS::Cognito::IdentityPool resource that defines configuration options for Amazon    Cognito streams.
#[derive(Default, serde::Serialize)]
pub struct CognitoStreams {


    /// 
    /// The name of the Amazon Cognito stream to receive updates. This stream must be in the    developer's account and in the same Region as the identity pool.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the role Amazon Cognito can assume to publish to the    stream. This role must grant access to Amazon Cognito (cognito-sync) to invoke     PutRecord on your Amazon Cognito stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// Status of the Amazon Cognito streams. Valid values are: ENABLED or     DISABLED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamingStatus")]
    pub streaming_status: Option<String>,

}


/// PushSync is a property of the AWS::Cognito::IdentityPool resource that defines the configuration options to be    applied to an Amazon Cognito identity pool.
#[derive(Default, serde::Serialize)]
pub struct PushSync {


    /// 
    /// An IAM role configured to allow Amazon Cognito to call Amazon SNS on behalf of the    developer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The ARNs of the Amazon SNS platform applications that could be used by clients.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationArns")]
    pub application_arns: Option<Vec<String>>,

}


/// CognitoIdentityProvider is a property of the AWS::Cognito::IdentityPool resource that represents an Amazon Cognito user pool and    its client ID.
#[derive(Default, serde::Serialize)]
pub struct CognitoIdentityProvider {


    /// 
    /// The client ID for the Amazon Cognito user pool.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,


    /// 
    /// TRUE if server-side token validation is enabled for the identity provider’s token.
    /// 
    /// After you set the ServerSideTokenCheck to TRUE for an identity pool, that    identity pool checks with the integrated user pools to make sure the user has not been    globally signed out or deleted before the identity pool provides an OIDC token or AWS credentials for the user.
    /// 
    /// If the user is signed out or deleted, the identity pool returns a 400 Not Authorized    error.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerSideTokenCheck")]
    pub server_side_token_check: Option<bool>,


    /// 
    /// The provider name for an Amazon Cognito user pool. For example:     cognito-idp.us-east-2.amazonaws.com/us-east-2_123456789.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

}