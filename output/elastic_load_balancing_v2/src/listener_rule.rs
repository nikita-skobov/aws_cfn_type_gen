/// Specifies a listener rule. The listener must be associated with an Application Load     Balancer. Each rule consists of a priority, one or more actions, and one or more     conditions.
///
/// For more information, see Quotas for your Application Load Balancers in the      User Guide for Application Load Balancers.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnListenerRule {
    ///
    /// The actions.
    ///
    /// The rule must include exactly one of the following types of actions:       forward, fixed-response, or redirect, and it must     be the last action to be performed. If the rule is for an HTTPS listener, it can also     optionally include an authentication action.
    ///
    /// Required: Yes
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,

    ///
    /// The conditions.
    ///
    /// The rule can optionally include up to one of each of the following conditions:       http-request-method, host-header, path-pattern,     and source-ip. A rule can also optionally include one or more of each of the     following conditions: http-header and query-string.
    ///
    /// Required: Yes
    ///
    /// Type: List of RuleCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Conditions")]
    pub conditions: Vec<RuleCondition>,

    ///
    /// The Amazon Resource Name (ARN) of the listener.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListenerArn")]
    pub listener_arn: cfn_resources::StrVal,

    ///
    /// The rule priority. A listener can't have multiple rules with the same priority.
    ///
    /// If you try to reorder rules by updating their priorities, do not specify a new priority     if an existing rule already uses this priority, as this can cause an error. If you need to     reuse a priority with a different rule, you must remove it as a priority first, and then     specify it in a subsequent update.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,

    #[serde(skip_serializing)]
    pub att_rule_arn: CfnListenerRulerulearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnListenerRulerulearn;
impl CfnListenerRulerulearn {
    pub fn att_name(&self) -> &'static str {
        r#"RuleArn"#
    }
}

impl cfn_resources::CfnResource for CfnListenerRule {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticLoadBalancingV2::ListenerRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.priority;

        if *the_val > 50000 as _ {
            return Err(format!(
                "Max validation failed on field 'priority'. {} is greater than 50000",
                the_val
            ));
        }

        let the_val = &self.priority;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'priority'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Specifies an action for a listener rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Action {
    ///
    /// [HTTPS listeners] Information for using Amazon Cognito to authenticate users. Specify only    when Type is authenticate-cognito.
    ///
    /// Required: No
    ///
    /// Type: AuthenticateCognitoConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticateCognitoConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authenticate_cognito_config: Option<AuthenticateCognitoConfig>,

    ///
    /// [HTTPS listeners] Information about an identity provider that is compliant with OpenID    Connect (OIDC). Specify only when Type is authenticate-oidc.
    ///
    /// Required: No
    ///
    /// Type: AuthenticateOidcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticateOidcConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authenticate_oidc_config: Option<AuthenticateOidcConfig>,

    ///
    /// [Application Load Balancer] Information for creating an action that returns a custom HTTP    response. Specify only when Type is fixed-response.
    ///
    /// Required: No
    ///
    /// Type: FixedResponseConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedResponseConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub fixed_response_config: Option<FixedResponseConfig>,

    ///
    /// Information for creating an action that distributes requests among one or more target    groups. For Network Load Balancers, you can specify a single target group. Specify only when     Type is forward. If you specify both ForwardConfig    and TargetGroupArn, you can specify only one target group using     ForwardConfig and it must be the same target group specified in     TargetGroupArn.
    ///
    /// Required: No
    ///
    /// Type: ForwardConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub forward_config: Option<ForwardConfig>,

    ///
    /// The order for the action. This value is required for rules with multiple actions. The    action with the lowest value for order is performed first.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub order: Option<i64>,

    ///
    /// [Application Load Balancer] Information for creating a redirect action. Specify only when     Type is redirect.
    ///
    /// Required: No
    ///
    /// Type: RedirectConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub redirect_config: Option<RedirectConfig>,

    ///
    /// The Amazon Resource Name (ARN) of the target group. Specify only when Type is     forward and you want to route to a single target group. To route to one or more    target groups, use ForwardConfig instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_group_arn: Option<cfn_resources::StrVal>,

    ///
    /// The type of action.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: authenticate-cognito | authenticate-oidc | fixed-response | forward | redirect
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ActionTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ActionTypeEnum {
    /// authenticate-cognito
    #[serde(rename = "authenticate-cognito")]
    Authenticatecognito,

    /// authenticate-oidc
    #[serde(rename = "authenticate-oidc")]
    Authenticateoidc,

    /// fixed-response
    #[serde(rename = "fixed-response")]
    Fixedresponse,

    /// forward
    #[serde(rename = "forward")]
    Forward,

    /// redirect
    #[serde(rename = "redirect")]
    Redirect,
}

impl Default for ActionTypeEnum {
    fn default() -> Self {
        ActionTypeEnum::Authenticatecognito
    }
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authenticate_cognito_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.authenticate_oidc_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.fixed_response_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.forward_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.order {
            if *the_val > 50000 as _ {
                return Err(format!(
                    "Max validation failed on field 'order'. {} is greater than 50000",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.order {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'order'. {} is less than 1",
                    the_val
                ));
            }
        }

        self.redirect_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies information required when integrating with Amazon Cognito to authenticate     users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuthenticateCognitoConfig {
    ///
    /// The query parameters (up to 10) to include in the redirect request to the authorization    endpoint.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationRequestExtraParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,

    ///
    /// The behavior if the user is not authenticated. The following are possible values:
    ///
    /// deny - Return an HTTP 401 Unauthorized error.               allow - Allow the request to be forwarded to the target.               authenticate - Redirect the request to the IdP authorization endpoint. This is      the default value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: allow | authenticate | deny
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnUnauthenticatedRequest")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_unauthenticated_request: Option<AuthenticateCognitoConfigOnUnauthenticatedRequestEnum>,

    ///
    /// The set of user claims to be requested from the IdP. The default is    openid.
    ///
    /// To verify which scope values your IdP supports and how to separate multiple values, see    the documentation for your IdP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub scope: Option<cfn_resources::StrVal>,

    ///
    /// The name of the cookie used to maintain session information. The default is    AWSELBAuthSessionCookie.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionCookieName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub session_cookie_name: Option<cfn_resources::StrVal>,

    ///
    /// The maximum duration of the authentication session, in seconds. The default is 604800    seconds (7 days).
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
    /// The Amazon Resource Name (ARN) of the Amazon Cognito user pool.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolArn")]
    pub user_pool_arn: cfn_resources::StrVal,

    ///
    /// The ID of the Amazon Cognito user pool client.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolClientId")]
    pub user_pool_client_id: cfn_resources::StrVal,

    ///
    /// The domain prefix or fully-qualified domain name of the Amazon Cognito user pool.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolDomain")]
    pub user_pool_domain: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AuthenticateCognitoConfigOnUnauthenticatedRequestEnum {
    /// allow
    #[serde(rename = "allow")]
    Allow,

    /// authenticate
    #[serde(rename = "authenticate")]
    Authenticate,

    /// deny
    #[serde(rename = "deny")]
    Deny,
}

impl Default for AuthenticateCognitoConfigOnUnauthenticatedRequestEnum {
    fn default() -> Self {
        AuthenticateCognitoConfigOnUnauthenticatedRequestEnum::Allow
    }
}

impl cfn_resources::CfnResource for AuthenticateCognitoConfig {
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

/// Specifies information required using an identity provide (IdP) that is compliant with     OpenID Connect (OIDC) to authenticate users.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuthenticateOidcConfig {
    ///
    /// The query parameters (up to 10) to include in the redirect request to the authorization    endpoint.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationRequestExtraParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,

    ///
    /// The authorization endpoint of the IdP. This must be a full URL, including the HTTPS    protocol, the domain, and the path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: cfn_resources::StrVal,

    ///
    /// The OAuth 2.0 client identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: cfn_resources::StrVal,

    ///
    /// The OAuth 2.0 client secret. This parameter is required if you are creating a rule. If you    are modifying a rule, you can omit this parameter if you set     UseExistingClientSecret to true.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub client_secret: Option<cfn_resources::StrVal>,

    ///
    /// The OIDC issuer identifier of the IdP. This must be a full URL, including the HTTPS    protocol, the domain, and the path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    pub issuer: cfn_resources::StrVal,

    ///
    /// The behavior if the user is not authenticated. The following are possible values:
    ///
    /// deny - Return an HTTP 401 Unauthorized error.               allow - Allow the request to be forwarded to the target.               authenticate - Redirect the request to the IdP authorization endpoint. This is      the default value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: allow | authenticate | deny
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnUnauthenticatedRequest")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_unauthenticated_request: Option<AuthenticateOidcConfigOnUnauthenticatedRequestEnum>,

    ///
    /// The set of user claims to be requested from the IdP. The default is    openid.
    ///
    /// To verify which scope values your IdP supports and how to separate multiple values, see    the documentation for your IdP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub scope: Option<cfn_resources::StrVal>,

    ///
    /// The name of the cookie used to maintain session information. The default is    AWSELBAuthSessionCookie.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionCookieName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub session_cookie_name: Option<cfn_resources::StrVal>,

    ///
    /// The maximum duration of the authentication session, in seconds. The default is 604800    seconds (7 days).
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
    /// The token endpoint of the IdP. This must be a full URL, including the HTTPS protocol, the    domain, and the path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: cfn_resources::StrVal,

    ///
    /// Indicates whether to use the existing client secret when modifying a rule. If you are    creating a rule, you can omit this parameter or set it to false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseExistingClientSecret")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub use_existing_client_secret: Option<bool>,

    ///
    /// The user info endpoint of the IdP. This must be a full URL, including the HTTPS protocol,    the domain, and the path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AuthenticateOidcConfigOnUnauthenticatedRequestEnum {
    /// allow
    #[serde(rename = "allow")]
    Allow,

    /// authenticate
    #[serde(rename = "authenticate")]
    Authenticate,

    /// deny
    #[serde(rename = "deny")]
    Deny,
}

impl Default for AuthenticateOidcConfigOnUnauthenticatedRequestEnum {
    fn default() -> Self {
        AuthenticateOidcConfigOnUnauthenticatedRequestEnum::Allow
    }
}

impl cfn_resources::CfnResource for AuthenticateOidcConfig {
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

/// Specifies information required when returning a custom HTTP response.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FixedResponseConfig {
    ///
    /// The content type.
    ///
    /// Valid Values: text/plain | text/css | text/html | application/javascript |    application/json
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub content_type: Option<FixedResponseConfigContentTypeEnum>,

    ///
    /// The message.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageBody")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub message_body: Option<cfn_resources::StrVal>,

    ///
    /// The HTTP response code (2XX, 4XX, or 5XX).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^(2|4|5)\d\d$
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FixedResponseConfigContentTypeEnum {
    /// text/plain
    #[serde(rename = "text/plain")]
    Textplain,

    /// text/css
    #[serde(rename = "text/css")]
    Textcss,

    /// text/html
    #[serde(rename = "text/html")]
    Texthtml,

    /// application/javascript
    #[serde(rename = "application/javascript")]
    Applicationjavascript,

    /// application/json
    #[serde(rename = "application/json")]
    Applicationjson,
}

impl Default for FixedResponseConfigContentTypeEnum {
    fn default() -> Self {
        FixedResponseConfigContentTypeEnum::Textplain
    }
}

impl cfn_resources::CfnResource for FixedResponseConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.message_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'message_body'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.message_body {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'message_body'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Information for creating an action that distributes requests among one or more target    groups. For Network Load Balancers, you can specify a single target group. Specify only when     Type is forward. If you specify both ForwardConfig    and TargetGroupArn, you can specify only one target group using     ForwardConfig and it must be the same target group specified in     TargetGroupArn.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ForwardConfig {
    ///
    /// Information about the target group stickiness for a rule.
    ///
    /// Required: No
    ///
    /// Type: TargetGroupStickinessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupStickinessConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_group_stickiness_config: Option<TargetGroupStickinessConfig>,

    ///
    /// Information about how traffic will be distributed between multiple target groups in a    forward rule.
    ///
    /// Required: No
    ///
    /// Type: List of TargetGroupTuple
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_groups: Option<Vec<TargetGroupTuple>>,
}

impl cfn_resources::CfnResource for ForwardConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.target_group_stickiness_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about a host header condition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HostHeaderConfig {
    ///
    /// The host names. The maximum size of each name is 128 characters. The comparison is    case insensitive. The following wildcard characters are supported: * (matches 0 or more    characters) and ? (matches exactly 1 character).
    ///
    /// If you specify multiple strings, the condition is satisfied if one of the strings matches    the host name.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for HostHeaderConfig {
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

/// Information about an HTTP header condition.
///
/// There is a set of standard HTTP header fields. You can also define custom HTTP header    fields.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpHeaderConfig {
    ///
    /// The name of the HTTP header field. The maximum size is 40 characters. The header name is     case insensitive. The allowed characters are specified by RFC 7230. Wildcards are not     supported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpHeaderName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub http_header_name: Option<cfn_resources::StrVal>,

    ///
    /// The strings to compare against the value of the HTTP header. The maximum size of    each string is 128 characters. The comparison strings are case insensitive. The following    wildcard characters are supported: * (matches 0 or more characters) and ? (matches exactly 1    character).
    ///
    /// If the same header appears multiple times in the request, we search them in order until a    match is found.
    ///
    /// If you specify multiple strings, the condition is satisfied if one of the strings matches    the value of the HTTP header. To require that all of the strings are a match, create one    condition per string.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for HttpHeaderConfig {
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

/// Information about an HTTP method condition.
///
/// HTTP defines a set of request methods, also referred to as HTTP verbs. For more    information, see the HTTP Method     Registry. You can also define custom HTTP methods.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HttpRequestMethodConfig {
    ///
    /// The name of the request method. The maximum size is 40 characters. The allowed characters    are A-Z, hyphen (-), and underscore (_). The comparison is case sensitive. Wildcards are not    supported; therefore, the method name must be an exact match.
    ///
    /// If you specify multiple strings, the condition is satisfied if one of the strings matches    the HTTP request method. We recommend that you route GET and HEAD requests in the same way,    because the response to a HEAD request may be cached.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for HttpRequestMethodConfig {
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

/// Information about a path pattern condition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PathPatternConfig {
    ///
    /// The path patterns to compare against the request URL. The maximum size of each     string is 128 characters. The comparison is case sensitive. The following wildcard     characters are supported: * (matches 0 or more characters) and ? (matches exactly 1     character).
    ///
    /// If you specify multiple strings, the condition is satisfied if one of them matches the     request URL. The path pattern is compared only to the path of the URL, not to its query     string.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for PathPatternConfig {
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

/// Information about a query string condition.
///
/// The query string component of a URI starts after the first '?' character and is terminated    by either a '#' character or the end of the URI. A typical query string contains key/value    pairs separated by '&' characters. The allowed characters are specified by RFC 3986. Any    character can be percentage encoded.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QueryStringConfig {
    ///
    /// The key/value pairs or values to find in the query string. The maximum size of    each string is 128 characters. The comparison is case insensitive. The following wildcard    characters are supported: * (matches 0 or more characters) and ? (matches exactly 1    character). To search for a literal '*' or '?' character in a query string, you must escape    these characters in Values using a '\' character.
    ///
    /// If you specify multiple key/value pairs or values, the condition is satisfied if one of    them is found in the query string.
    ///
    /// Required: No
    ///
    /// Type: List of QueryStringKeyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<QueryStringKeyValue>>,
}

impl cfn_resources::CfnResource for QueryStringConfig {
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

/// Information about a key/value pair.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct QueryStringKeyValue {
    ///
    /// The key. You can omit the key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for QueryStringKeyValue {
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

/// Information about a redirect action.
///
/// A URI consists of the following components: protocol://hostname:port/path?query. You must    modify at least one of the following components to avoid a redirect loop: protocol, hostname,    port, or path. Any components that you do not modify retain their original values.
///
/// You can reuse URI components using the following reserved keywords:
///
/// For example, you can change the path to "/new/#{path}", the hostname to "example.#{host}",    or the query to "#{query}&value=xyz".
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RedirectConfig {
    ///
    /// The hostname. This component is not percent-encoded. The hostname can contain    #{host}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub host: Option<cfn_resources::StrVal>,

    ///
    /// The absolute path, starting with the leading "/". This component is not percent-encoded.    The path can contain #{host}, #{path}, and #{port}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// The port. You can specify a value from 1 to 65535 or #{port}.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub port: Option<cfn_resources::StrVal>,

    ///
    /// The protocol. You can specify HTTP, HTTPS, or #{protocol}. You can redirect HTTP to HTTP,    HTTP to HTTPS, and HTTPS to HTTPS. You cannot redirect HTTPS to HTTP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^(HTTPS?|#\{protocol\})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub protocol: Option<cfn_resources::StrVal>,

    ///
    /// The query parameters, URL-encoded when necessary, but not percent-encoded. Do not include    the leading "?", as it is automatically added. You can specify any of the reserved    keywords.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Query")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub query: Option<cfn_resources::StrVal>,

    ///
    /// The HTTP redirect code. The redirect is either permanent (HTTP 301) or temporary (HTTP    302).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP_301 | HTTP_302
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: RedirectConfigStatusCodeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RedirectConfigStatusCodeEnum {
    /// HTTP_301
    #[serde(rename = "HTTP_301")]
    Http301,

    /// HTTP_302
    #[serde(rename = "HTTP_302")]
    Http302,
}

impl Default for RedirectConfigStatusCodeEnum {
    fn default() -> Self {
        RedirectConfigStatusCodeEnum::Http301
    }
}

impl cfn_resources::CfnResource for RedirectConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.host {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'host'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.host {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'host'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.query {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'query'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.query {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'query'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies a condition for a listener rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RuleCondition {
    ///
    /// The field in the HTTP request. The following are the possible values:
    ///
    /// http-header                                http-request-method                                host-header                                path-pattern                                query-string                                source-ip
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// Information for a host header condition. Specify only when Field is     host-header.
    ///
    /// Required: No
    ///
    /// Type: HostHeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostHeaderConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub host_header_config: Option<HostHeaderConfig>,

    ///
    /// Information for an HTTP header condition. Specify only when Field is     http-header.
    ///
    /// Required: Conditional
    ///
    /// Type: HttpHeaderConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpHeaderConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub http_header_config: Option<HttpHeaderConfig>,

    ///
    /// Information for an HTTP method condition. Specify only when Field is     http-request-method.
    ///
    /// Required: Conditional
    ///
    /// Type: HttpRequestMethodConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRequestMethodConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub http_request_method_config: Option<HttpRequestMethodConfig>,

    ///
    /// Information for a path pattern condition. Specify only when Field is     path-pattern.
    ///
    /// Required: No
    ///
    /// Type: PathPatternConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathPatternConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path_pattern_config: Option<PathPatternConfig>,

    ///
    /// Information for a query string condition. Specify only when Field is     query-string.
    ///
    /// Required: Conditional
    ///
    /// Type: QueryStringConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub query_string_config: Option<QueryStringConfig>,

    ///
    /// Information for a source IP condition. Specify only when Field is     source-ip.
    ///
    /// Required: Conditional
    ///
    /// Type: SourceIpConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIpConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub source_ip_config: Option<SourceIpConfig>,

    ///
    /// The condition value. Specify only when Field is host-header or       path-pattern. Alternatively, to specify multiple host names or multiple     path patterns, use HostHeaderConfig or PathPatternConfig.
    ///
    /// If Field is host-header and you're not using       HostHeaderConfig, you can specify a single host name (for example,     my.example.com). A host name is case insensitive, can be up to 128 characters in length,     and can contain any of the following characters.
    ///
    /// A-Z, a-z, 0-9            - .            * (matches 0 or more characters)            ? (matches exactly 1 character)
    ///
    /// If Field is path-pattern and you're not using       PathPatternConfig, you can specify a single path pattern (for example,     /img/*). A path pattern is case-sensitive, can be up to 128 characters in length, and can     contain any of the following characters.
    ///
    /// A-Z, a-z, 0-9            _ - . $ / ~ " ' @ : +            & (using &amp;)            * (matches 0 or more characters)            ? (matches exactly 1 character)
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for RuleCondition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.field {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'field'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        self.host_header_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.http_header_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.http_request_method_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.path_pattern_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.query_string_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.source_ip_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about a source IP condition.
///
/// You can use this condition to route based on the IP address of the source that connects to    the load balancer. If a client is behind a proxy, this is the IP address of the proxy not the    IP address of the client.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SourceIpConfig {
    ///
    /// The source IP addresses, in CIDR format. You can use both IPv4 and IPv6     addresses. Wildcards are not supported.
    ///
    /// If you specify multiple addresses, the condition is satisfied if the source IP address     of the request matches one of the CIDR blocks. This condition is not satisfied by the     addresses in the X-Forwarded-For header.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for SourceIpConfig {
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

/// Information about the target group stickiness for a rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TargetGroupStickinessConfig {
    ///
    /// The time period, in seconds, during which requests from a client should be routed to the    same target group. The range is 1-604800 seconds (7 days).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub duration_seconds: Option<i64>,

    ///
    /// Indicates whether target group stickiness is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for TargetGroupStickinessConfig {
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

/// Information about how traffic will be distributed between multiple target groups in a    forward rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TargetGroupTuple {
    ///
    /// The Amazon Resource Name (ARN) of the target group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_group_arn: Option<cfn_resources::StrVal>,

    ///
    /// The weight. The range is 0 to 999.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub weight: Option<i64>,
}

impl cfn_resources::CfnResource for TargetGroupTuple {
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
