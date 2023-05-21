/// The AWS::Cognito::UserPoolClient resource specifies an Amazon Cognito user    pool client.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolClient {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_oauth_flows_user_pool_client: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_oauth_scopes: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_session_validity: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_urls: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_redirect_uri: Option<cfn_resources::StrVal>,

    ///
    /// Activates the propagation of additional user context data. For more information about       propagation of user context data, see Adding advanced security to a user pool. If you don’t include this       parameter, you can't send device fingerprint information, including source IP address,       to Amazon Cognito advanced security. You can only activate         EnablePropagateAdditionalUserContextData in an app client that has a       client secret.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePropagateAdditionalUserContextData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_propagate_additional_user_context_data: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_token_revocation: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,

    ///
    /// Boolean to specify whether you want to generate a secret for the user pool client       being created.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "GenerateSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_secret: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_validity: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_urls: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_user_existence_errors: Option<UserPoolClientPreventUserExistenceErrorsEnum>,

    ///
    /// The read attributes.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<i64>,

    ///
    /// A list of provider names for the identity providers (IdPs) that are supported on this       client. The following are supported: COGNITO, Facebook,         Google, SignInWithApple, and LoginWithAmazon.       You can also specify the names that you configured for the SAML and OIDC IdPs in your       user pool, for example MySAMLIdP or MyOIDCIdP.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_identity_providers: Option<Vec<String>>,

    ///
    /// The units in which the validity times are represented. The default unit for       RefreshToken is days, and default for ID and access tokens are hours.
    ///
    /// Required: No
    ///
    /// Type: TokenValidityUnits
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenValidityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_validity_units: Option<TokenValidityUnits>,

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
    pub user_pool_id: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum UserPoolClientPreventUserExistenceErrorsEnum {
    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

    /// LEGACY
    #[serde(rename = "LEGACY")]
    Legacy,
}

impl Default for UserPoolClientPreventUserExistenceErrorsEnum {
    fn default() -> Self {
        UserPoolClientPreventUserExistenceErrorsEnum::Enabled
    }
}

impl cfn_resources::CfnResource for CfnUserPoolClient {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolClient"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.allowed_oauth_flows {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'allowed_oauth_flows'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.allowed_oauth_scopes {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'allowed_oauth_scopes'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        self.analytics_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.auth_session_validity {
            if *the_val > 15 as _ {
                return Err(format!(
                    "Max validation failed on field 'auth_session_validity'. {} is greater than 15",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.auth_session_validity {
            if *the_val < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'auth_session_validity'. {} is less than 3",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.callback_urls {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'callback_urls'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.client_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'client_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.client_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'client_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.default_redirect_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'default_redirect_uri'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.default_redirect_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'default_redirect_uri'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.logout_urls {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'logout_urls'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        self.token_validity_units
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 55 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_pool_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<cfn_resources::StrVal>,

    ///
    /// The external ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// If UserDataShared is true, Amazon Cognito includes user data in the       events that it publishes to Amazon Pinpoint analytics.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserDataShared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data_shared: Option<bool>,
}

impl cfn_resources::CfnResource for AnalyticsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The time units you use when you set the duration of ID, access, and refresh tokens.       The default unit for RefreshToken is days, and the default for ID and access tokens is       hours.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TokenValidityUnits {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<TokenValidityUnitsAccessTokenEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<TokenValidityUnitsIdTokenEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<TokenValidityUnitsRefreshTokenEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TokenValidityUnitsAccessTokenEnum {
    /// days
    #[serde(rename = "days")]
    Days,

    /// hours
    #[serde(rename = "hours")]
    Hours,

    /// minutes
    #[serde(rename = "minutes")]
    Minutes,

    /// seconds
    #[serde(rename = "seconds")]
    Seconds,
}

impl Default for TokenValidityUnitsAccessTokenEnum {
    fn default() -> Self {
        TokenValidityUnitsAccessTokenEnum::Days
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TokenValidityUnitsIdTokenEnum {
    /// days
    #[serde(rename = "days")]
    Days,

    /// hours
    #[serde(rename = "hours")]
    Hours,

    /// minutes
    #[serde(rename = "minutes")]
    Minutes,

    /// seconds
    #[serde(rename = "seconds")]
    Seconds,
}

impl Default for TokenValidityUnitsIdTokenEnum {
    fn default() -> Self {
        TokenValidityUnitsIdTokenEnum::Days
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TokenValidityUnitsRefreshTokenEnum {
    /// days
    #[serde(rename = "days")]
    Days,

    /// hours
    #[serde(rename = "hours")]
    Hours,

    /// minutes
    #[serde(rename = "minutes")]
    Minutes,

    /// seconds
    #[serde(rename = "seconds")]
    Seconds,
}

impl Default for TokenValidityUnitsRefreshTokenEnum {
    fn default() -> Self {
        TokenValidityUnitsRefreshTokenEnum::Days
    }
}

impl cfn_resources::CfnResource for TokenValidityUnits {
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
