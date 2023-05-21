

/// Specifies a listener for an Application Load Balancer, Network Load Balancer, or     Gateway Load Balancer.
#[derive(Default, serde::Serialize)]
pub struct CfnListener {


    /// 
    /// The actions for the default rule. You cannot define a condition for a default     rule.
    /// 
    /// To create additional rules for an Application Load Balancer, use AWS::ElasticLoadBalancingV2::ListenerRule.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultActions")]
    pub default_actions: Vec<Action>,


    /// 
    /// The port on which the load balancer is listening. You cannot specify a port for a Gateway    Load Balancer.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The default SSL server certificate for a secure listener. You must provide exactly one     certificate if the listener protocol is HTTPS or TLS.
    /// 
    /// To create a certificate list for a secure listener, use AWS::ElasticLoadBalancingV2::ListenerCertificate.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of Certificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificates")]
    pub certificates: Option<Vec<Certificate>>,


    /// 
    /// [TLS listener] The name of the Application-Layer Protocol Negotiation (ALPN)    policy.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlpnPolicy")]
    pub alpn_policy: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Name (ARN) of the load balancer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: String,


    /// 
    /// [HTTPS and TLS listeners] The security policy that defines which protocols and ciphers are    supported.
    /// 
    /// For more information, see Security policies in the Application Load Balancers Guide and     Security policies in the Network Load Balancers Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SslPolicy")]
    pub ssl_policy: Option<String>,


    /// 
    /// The protocol for connections from clients to the load balancer. For Application Load    Balancers, the supported protocols are HTTP and HTTPS. For Network Load Balancers, the    supported protocols are TCP, TLS, UDP, and TCP_UDP. You can’t specify the UDP or TCP_UDP    protocol if dual-stack mode is enabled. You cannot specify a protocol for a Gateway Load    Balancer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GENEVE | HTTP | HTTPS | TCP | TCP_UDP | TLS | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}


/// Specifies information required when integrating with Amazon Cognito to authenticate     users.
#[derive(Default, serde::Serialize)]
pub struct AuthenticateCognitoConfig {


    /// 
    /// The maximum duration of the authentication session, in seconds. The default is 604800    seconds (7 days).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<String>,


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
    pub scope: Option<String>,


    /// 
    /// The name of the cookie used to maintain session information. The default is    AWSELBAuthSessionCookie.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,


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
    pub on_unauthenticated_request: Option<String>,


    /// 
    /// The query parameters (up to 10) to include in the redirect request to the authorization    endpoint.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The domain prefix or fully-qualified domain name of the Amazon Cognito user pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolDomain")]
    pub user_pool_domain: String,


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon Cognito user pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolArn")]
    pub user_pool_arn: String,


    /// 
    /// The ID of the Amazon Cognito user pool client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserPoolClientId")]
    pub user_pool_client_id: String,

}


/// Information for creating an action that distributes requests among one or more target    groups. For Network Load Balancers, you can specify a single target group. Specify only when     Type is forward. If you specify both ForwardConfig    and TargetGroupArn, you can specify only one target group using     ForwardConfig and it must be the same target group specified in     TargetGroupArn.
#[derive(Default, serde::Serialize)]
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
    pub target_groups: Option<Vec<TargetGroupTuple>>,

}


/// Specifies an SSL server certificate to use as the default certificate for a secure     listener.
#[derive(Default, serde::Serialize)]
pub struct Certificate {


    /// 
    /// The Amazon Resource Name (ARN) of the certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}


/// Information about the target group stickiness for a rule.
#[derive(Default, serde::Serialize)]
pub struct TargetGroupStickinessConfig {


    /// 
    /// Indicates whether target group stickiness is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The time period, in seconds, during which requests from a client should be routed to the    same target group. The range is 1-604800 seconds (7 days).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<i64>,

}


/// Information about how traffic will be distributed between multiple target groups in a    forward rule.
#[derive(Default, serde::Serialize)]
pub struct TargetGroupTuple {


    /// 
    /// The weight. The range is 0 to 999.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<i64>,


    /// 
    /// The Amazon Resource Name (ARN) of the target group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,

}


/// Information about a redirect action.
///
/// A URI consists of the following components: protocol://hostname:port/path?query. You must    modify at least one of the following components to avoid a redirect loop: protocol, hostname,    port, or path. Any components that you do not modify retain their original values.
///
/// You can reuse URI components using the following reserved keywords:
///
/// For example, you can change the path to "/new/#{path}", the hostname to "example.#{host}",    or the query to "#{query}&value=xyz".
#[derive(Default, serde::Serialize)]
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
    pub host: Option<String>,


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
    pub path: Option<String>,


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
    pub query: Option<String>,


    /// 
    /// The port. You can specify a value from 1 to 65535 or #{port}.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,


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
    pub status_code: String,


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
    pub protocol: Option<String>,

}


/// Specifies an action for a listener rule.
#[derive(Default, serde::Serialize)]
pub struct Action {


    /// 
    /// [Application Load Balancer] Information for creating an action that returns a custom HTTP    response. Specify only when Type is fixed-response.
    /// 
    /// Required: No
    ///
    /// Type: FixedResponseConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedResponseConfig")]
    pub fixed_response_config: Option<FixedResponseConfig>,


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
    pub order: Option<i64>,


    /// 
    /// The Amazon Resource Name (ARN) of the target group. Specify only when Type is     forward and you want to route to a single target group. To route to one or more    target groups, use ForwardConfig instead.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,


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
    pub cfn_type: String,


    /// 
    /// Information for creating an action that distributes requests among one or more target    groups. For Network Load Balancers, you can specify a single target group. Specify only when     Type is forward. If you specify both ForwardConfig    and TargetGroupArn, you can specify only one target group using     ForwardConfig and it must be the same target group specified in     TargetGroupArn.
    /// 
    /// Required: No
    ///
    /// Type: ForwardConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardConfig")]
    pub forward_config: Option<ForwardConfig>,


    /// 
    /// [Application Load Balancer] Information for creating a redirect action. Specify only when     Type is redirect.
    /// 
    /// Required: No
    ///
    /// Type: RedirectConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedirectConfig")]
    pub redirect_config: Option<RedirectConfig>,


    /// 
    /// [HTTPS listeners] Information for using Amazon Cognito to authenticate users. Specify only    when Type is authenticate-cognito.
    /// 
    /// Required: No
    ///
    /// Type: AuthenticateCognitoConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticateCognitoConfig")]
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
    pub authenticate_oidc_config: Option<AuthenticateOidcConfig>,

}


/// Specifies information required using an identity provide (IdP) that is compliant with     OpenID Connect (OIDC) to authenticate users.
#[derive(Default, serde::Serialize)]
pub struct AuthenticateOidcConfig {


    /// 
    /// The maximum duration of the authentication session, in seconds. The default is 604800    seconds (7 days).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<String>,


    /// 
    /// The OAuth 2.0 client secret. This parameter is required if you are creating a rule. If you    are modifying a rule, you can omit this parameter if you set     UseExistingClientSecret to true.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,


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
    pub scope: Option<String>,


    /// 
    /// The OAuth 2.0 client identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientId")]
    pub client_id: String,


    /// 
    /// The authorization endpoint of the IdP. This must be a full URL, including the HTTPS    protocol, the domain, and the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: String,


    /// 
    /// The query parameters (up to 10) to include in the redirect request to the authorization    endpoint.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The token endpoint of the IdP. This must be a full URL, including the HTTPS protocol, the    domain, and the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: String,


    /// 
    /// The user info endpoint of the IdP. This must be a full URL, including the HTTPS protocol,    the domain, and the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: String,


    /// 
    /// The OIDC issuer identifier of the IdP. This must be a full URL, including the HTTPS    protocol, the domain, and the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Issuer")]
    pub issuer: String,


    /// 
    /// The name of the cookie used to maintain session information. The default is    AWSELBAuthSessionCookie.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,


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
    pub on_unauthenticated_request: Option<String>,


    /// 
    /// Indicates whether to use the existing client secret when modifying a rule. If you are    creating a rule, you can omit this parameter or set it to false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseExistingClientSecret")]
    pub use_existing_client_secret: Option<bool>,

}


/// Specifies information required when returning a custom HTTP response.
#[derive(Default, serde::Serialize)]
pub struct FixedResponseConfig {


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
    pub message_body: Option<String>,


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
    pub status_code: String,


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
    pub content_type: Option<String>,

}
