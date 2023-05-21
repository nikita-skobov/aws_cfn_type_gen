

/// The AWS::Cognito::UserPoolClient resource specifies an Amazon Cognito user    pool client.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolClient {


    /// 
    /// Amazon Cognito creates a session token for each API request in an authentication flow. AuthSessionValidity is the duration, in minutes, of that session token. Your user pool native user must respond to each authentication challenge before the session expires.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 3
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthSessionValidity")]
    pub auth_session_validity: Option<i64>,


    /// 
    /// The default redirect URI. Must be in the CallbackURLs list.
    /// 
    /// A redirect URI must:
    /// 
    /// Be an absolute URI.               Be registered with the authorization server.               Not include a fragment component.
    /// 
    /// See OAuth 2.0 -         Redirection Endpoint.
    /// 
    /// Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes       only.
    /// 
    /// App callback URLs such as myapp://example are also supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRedirectURI")]
    pub default_redirect_uri: Option<String>,


    /// 
    /// The user pool attributes that the app client can write to.
    /// 
    /// If your app client allows users to sign in through an IdP, this array must include all       attributes that you have mapped to IdP attributes. Amazon Cognito updates mapped attributes when       users sign in to your application through an IdP. If your app client does not have write       access to a mapped attribute, Amazon Cognito throws an error when it tries to update the       attribute. For more information, see Specifying IdP Attribute Mappings for Your user       pool.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteAttributes")]
    pub write_attributes: Option<Vec<String>>,


    /// 
    /// The user pool analytics configuration for collecting metrics and sending them to your       Amazon Pinpoint campaign.
    /// 
    /// NoteIn AWS Regions where Amazon Pinpoint isn't available, user pools only support sending         events to Amazon Pinpoint projects in AWS Region us-east-1. In Regions where Amazon Pinpoint is         available, user pools support sending events to Amazon Pinpoint projects within that same         Region.
    /// 
    /// Required: No
    ///
    /// Type: AnalyticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnalyticsConfiguration")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,


    /// 
    /// Activates or deactivates token revocation. For more information about revoking tokens,       see RevokeToken.
    /// 
    /// If you don't include this parameter, token revocation is automatically activated for       the new user pool client.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableTokenRevocation")]
    pub enable_token_revocation: Option<bool>,


    /// 
    /// Activates the propagation of additional user context data. For more information about       propagation of user context data, see Adding advanced security to a user pool. If you don’t include this       parameter, you can't send device fingerprint information, including source IP address,       to Amazon Cognito advanced security. You can only activate         EnablePropagateAdditionalUserContextData in an app client that has a       client secret.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    pub enable_propagate_additional_user_context_data: Option<bool>,


    /// 
    /// The allowed OAuth scopes. Possible values provided by OAuth are phone,         email, openid, and profile. Possible values       provided by AWS are aws.cognito.signin.user.admin. Custom       scopes created in Resource Servers are also supported.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOAuthScopes")]
    pub allowed_oauth_scopes: Option<Vec<String>>,


    /// 
    /// The refresh token time limit. After this limit expires, your user can't use their refresh    token. To specify the time unit for RefreshTokenValidity as seconds,     minutes, hours, or days, set a     TokenValidityUnits value in your API request.
    /// 
    /// For example, when you set RefreshTokenValidity as 10 and     TokenValidityUnits as days, your user can refresh their session    and retrieve new access and ID tokens for 10 days.
    /// 
    /// The default time unit for RefreshTokenValidity in an API request is days. You    can't set RefreshTokenValidity to 0. If you do, Amazon Cognito overrides the    value with the default value of 30 days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshTokenValidity")]
    pub refresh_token_validity: Option<i64>,


    /// 
    /// Use this setting to choose which errors and responses are returned by Cognito APIs during    authentication, account confirmation, and password recovery when the user does not exist in    the user pool. When set to ENABLED and the user does not exist, authentication    returns an error indicating either the username or password was incorrect, and account    confirmation and password recovery return a response indicating a code was sent to a simulated    destination. When set to LEGACY, those APIs will return a     UserNotFoundException exception if the user does not exist in the user pool.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ENABLED | LEGACY
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreventUserExistenceErrors")]
    pub prevent_user_existence_errors: Option<String>,


    /// 
    /// A list of allowed logout URLs for the IdPs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogoutURLs")]
    pub logout_urls: Option<Vec<String>>,


    /// 
    /// The client name for the user pool client you would like to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w\s+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientName")]
    pub client_name: Option<String>,


    /// 
    /// The access token time limit. After this limit expires, your user can't use their access    token. To specify the time unit for AccessTokenValidity as seconds,     minutes, hours, or days, set a     TokenValidityUnits value in your API request.
    /// 
    /// For example, when you set AccessTokenValidity to 10 and     TokenValidityUnits to hours, your user can authorize access with    their access token for 10 hours.
    /// 
    /// The default time unit for AccessTokenValidity in an API request is hours.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessTokenValidity")]
    pub access_token_validity: Option<i64>,


    /// 
    /// The allowed OAuth flows.
    /// 
    /// code                  Use a code grant flow, which provides an authorization code as the             response. This code can be exchanged for access tokens with the               /oauth2/token endpoint.                       implicit                  Issue the access token (and, optionally, ID token, based on scopes)             directly to your user.                       client_credentials                  Issue the access token from the /oauth2/token endpoint             directly to a non-person user using a combination of the client ID and             client secret.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOAuthFlows")]
    pub allowed_oauth_flows: Option<Vec<String>>,


    /// 
    /// Set to true if the client is allowed to follow the OAuth protocol when interacting       with Amazon Cognito user pools.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    pub allowed_oauth_flows_user_pool_client: Option<bool>,


    /// 
    /// The authentication flows that you want your user pool client to support. For each app client in your user pool, you can sign in your users with any combination of one or more flows, including with a user name and Secure Remote Password (SRP), a user name and password, or a custom authentication process that you define with Lambda functions.
    /// 
    /// NoteIf you don't specify a value for ExplicitAuthFlows, your user client supports ALLOW_REFRESH_TOKEN_AUTH, ALLOW_USER_SRP_AUTH, and ALLOW_CUSTOM_AUTH.
    /// 
    /// Valid values include:
    /// 
    /// ALLOW_ADMIN_USER_PASSWORD_AUTH: Enable admin based user password       authentication flow ADMIN_USER_PASSWORD_AUTH. This setting replaces       the ADMIN_NO_SRP_AUTH setting. With this authentication flow, your app       passes a user name and password to Amazon Cognito in the request, instead of using the Secure       Remote Password (SRP) protocol to securely transmit the password.                        ALLOW_CUSTOM_AUTH: Enable Lambda trigger based       authentication.                        ALLOW_USER_PASSWORD_AUTH: Enable user password-based       authentication. In this flow, Amazon Cognito receives the password in the request instead       of using the SRP protocol to verify passwords.                        ALLOW_USER_SRP_AUTH: Enable SRP-based authentication.                        ALLOW_REFRESH_TOKEN_AUTH: Enable authflow to refresh       tokens.
    /// 
    /// In some environments, you will see the values ADMIN_NO_SRP_AUTH, CUSTOM_AUTH_FLOW_ONLY, or USER_PASSWORD_AUTH. You can't assign these legacy ExplicitAuthFlows values to user pool clients at the same time as values that begin with ALLOW_, like ALLOW_USER_SRP_AUTH.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplicitAuthFlows")]
    pub explicit_auth_flows: Option<Vec<String>>,


    /// 
    /// A list of provider names for the identity providers (IdPs) that are supported on this       client. The following are supported: COGNITO, Facebook,         Google, SignInWithApple, and LoginWithAmazon.       You can also specify the names that you configured for the SAML and OIDC IdPs in your       user pool, for example MySAMLIdP or MyOIDCIdP.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedIdentityProviders")]
    pub supported_identity_providers: Option<Vec<String>>,


    /// 
    /// Boolean to specify whether you want to generate a secret for the user pool client       being created.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "GenerateSecret")]
    pub generate_secret: Option<bool>,


    /// 
    /// The units in which the validity times are represented. The default unit for       RefreshToken is days, and default for ID and access tokens are hours.
    /// 
    /// Required: No
    ///
    /// Type: TokenValidityUnits
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenValidityUnits")]
    pub token_validity_units: Option<TokenValidityUnits>,


    /// 
    /// The read attributes.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadAttributes")]
    pub read_attributes: Option<Vec<String>>,


    /// 
    /// The user pool ID for the user pool where you want to create a user pool client.
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


    /// 
    /// The ID token time limit. After this limit expires, your user can't use their ID token. To    specify the time unit for IdTokenValidity as seconds,     minutes, hours, or days, set a     TokenValidityUnits value in your API request.
    /// 
    /// For example, when you set IdTokenValidity as 10 and     TokenValidityUnits as hours, your user can authenticate their    session with their ID token for 10 hours.
    /// 
    /// The default time unit for IdTokenValidity in an API request is hours.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdTokenValidity")]
    pub id_token_validity: Option<i64>,


    /// 
    /// A list of allowed redirect (callback) URLs for the IdPs.
    /// 
    /// A redirect URI must:
    /// 
    /// Be an absolute URI.               Be registered with the authorization server.               Not include a fragment component.
    /// 
    /// See OAuth 2.0 -         Redirection Endpoint.
    /// 
    /// Amazon Cognito requires HTTPS over HTTP except for http://localhost for testing purposes       only.
    /// 
    /// App callback URLs such as myapp://example are also supported.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "CallbackURLs")]
    pub callback_urls: Option<Vec<String>>,

}

impl cfn_resources::CfnResource for CfnUserPoolClient {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolClient"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The Amazon Pinpoint analytics configuration necessary to collect metrics for a user       pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalyticsConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of an Amazon Pinpoint project. You can use the     Amazon Pinpoint project for integration with the chosen user pool client. Amazon Cognito publishes events to the Amazon Pinpoint project that the app ARN    declares.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationArn")]
    pub application_arn: Option<String>,


    /// 
    /// The external ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,


    /// 
    /// If UserDataShared is true, Amazon Cognito includes user data in the       events that it publishes to Amazon Pinpoint analytics.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserDataShared")]
    pub user_data_shared: Option<bool>,


    /// 
    /// The ARN of an AWS Identity and Access Management role that authorizes Amazon Cognito to publish events to Amazon Pinpoint       analytics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:[\w+=/,.@-]+:[\w+=/,.@-]+:([\w+=/,.@-]*)?:[0-9]+:[\w+=/,.@-]+(:[\w+=/,.@-]+)?(:[\w+=/,.@-]+)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The application ID for an Amazon Pinpoint application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[0-9a-fA-F]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationId")]
    pub application_id: Option<String>,

}


/// The time units you use when you set the duration of ID, access, and refresh tokens.       The default unit for RefreshToken is days, and the default for ID and access tokens is       hours.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TokenValidityUnits {


    /// 
    /// A time unit of seconds, minutes, hours, or         days for the value that you set in the         RefreshTokenValidity parameter. The default         RefreshTokenValidity time unit is days.         RefreshTokenValidity duration can range from 60 minutes to 10       years.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: days | hours | minutes | seconds
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshToken")]
    pub refresh_token: Option<String>,


    /// 
    /// A time unit of seconds, minutes, hours, or         days for the value that you set in the IdTokenValidity       parameter. The default IdTokenValidity time unit is hours.         IdTokenValidity duration can range from five minutes to one day.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: days | hours | minutes | seconds
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdToken")]
    pub id_token: Option<String>,


    /// 
    /// A time unit of seconds, minutes, hours, or         days for the value that you set in the AccessTokenValidity       parameter. The default AccessTokenValidity time unit is hours.         AccessTokenValidity duration can range from five minutes to one       day.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: days | hours | minutes | seconds
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessToken")]
    pub access_token: Option<String>,

}
