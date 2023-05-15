
pub mod cfn_listener {

#[derive(serde::Serialize, Default)]
pub struct CfnListener {
    /// No documentation provided by AWS
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SslPolicy")]
    pub ssl_policy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AlpnPolicy")]
    pub alpn_policy: Option<Vec<String>>,
    /// List of Certificate
    #[serde(rename = "Certificates")]
    pub certificates: Option<Vec<Certificate>>,
    /// List of Action
    #[serde(rename = "DefaultActions")]
    pub default_actions: Vec<Action>,

}


#[derive(serde::Serialize, Default)]
pub struct ForwardConfig {
    #[serde(rename = "TargetGroupStickinessConfig")]
    pub target_group_stickiness_config: Option<TargetGroupStickinessConfig>,
    #[serde(rename = "TargetGroups")]
    pub target_groups: Option<Vec<TargetGroupTuple>>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthenticateOidcConfig {
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<String>,
    #[serde(rename = "OnUnauthenticatedRequest")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "UseExistingClientSecret")]
    pub use_existing_client_secret: Option<bool>,
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<()>,
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,
    #[serde(rename = "Issuer")]
    pub issuer: String,
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: String,
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: String,

}

#[derive(serde::Serialize, Default)]
pub struct FixedResponseConfig {
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "MessageBody")]
    pub message_body: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Certificate {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupTuple {
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RedirectConfig {
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "Host")]
    pub host: Option<String>,
    #[serde(rename = "Query")]
    pub query: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupStickinessConfig {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "AuthenticateOidcConfig")]
    pub authenticate_oidc_config: Option<AuthenticateOidcConfig>,
    #[serde(rename = "Order")]
    pub order: Option<usize>,
    #[serde(rename = "FixedResponseConfig")]
    pub fixed_response_config: Option<FixedResponseConfig>,
    #[serde(rename = "ForwardConfig")]
    pub forward_config: Option<ForwardConfig>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "AuthenticateCognitoConfig")]
    pub authenticate_cognito_config: Option<AuthenticateCognitoConfig>,
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "RedirectConfig")]
    pub redirect_config: Option<RedirectConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthenticateCognitoConfig {
    #[serde(rename = "OnUnauthenticatedRequest")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<()>,
    #[serde(rename = "UserPoolDomain")]
    pub user_pool_domain: String,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "UserPoolClientId")]
    pub user_pool_client_id: String,
    #[serde(rename = "UserPoolArn")]
    pub user_pool_arn: String,
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<String>,

}


}

pub mod cfn_listener_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnListenerCertificate {
    /// List of Certificate
    #[serde(rename = "Certificates")]
    pub certificates: Vec<Certificate>,
    /// No documentation provided by AWS
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct Certificate {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}


}

pub mod cfn_listener_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnListenerRule {
    /// No documentation provided by AWS
    #[serde(rename = "Priority")]
    pub priority: usize,
    /// List of RuleCondition
    #[serde(rename = "Conditions")]
    pub conditions: Vec<RuleCondition>,
    /// No documentation provided by AWS
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,
    /// List of Action
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,

}


#[derive(serde::Serialize, Default)]
pub struct PathPatternConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRequestMethodConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupStickinessConfig {
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<usize>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupTuple {
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct FixedResponseConfig {
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "MessageBody")]
    pub message_body: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RedirectConfig {
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "Query")]
    pub query: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "Host")]
    pub host: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct QueryStringKeyValue {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ForwardConfig {
    #[serde(rename = "TargetGroupStickinessConfig")]
    pub target_group_stickiness_config: Option<TargetGroupStickinessConfig>,
    #[serde(rename = "TargetGroups")]
    pub target_groups: Option<Vec<TargetGroupTuple>>,

}

#[derive(serde::Serialize, Default)]
pub struct HostHeaderConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleCondition {
    #[serde(rename = "PathPatternConfig")]
    pub path_pattern_config: Option<PathPatternConfig>,
    #[serde(rename = "SourceIpConfig")]
    pub source_ip_config: Option<SourceIpConfig>,
    #[serde(rename = "HttpHeaderConfig")]
    pub http_header_config: Option<HttpHeaderConfig>,
    #[serde(rename = "QueryStringConfig")]
    pub query_string_config: Option<QueryStringConfig>,
    #[serde(rename = "HostHeaderConfig")]
    pub host_header_config: Option<HostHeaderConfig>,
    #[serde(rename = "Field")]
    pub field: Option<String>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "HttpRequestMethodConfig")]
    pub http_request_method_config: Option<HttpRequestMethodConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct QueryStringConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<QueryStringKeyValue>>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthenticateOidcConfig {
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<usize>,
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "UseExistingClientSecret")]
    pub use_existing_client_secret: Option<bool>,
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,
    #[serde(rename = "OnUnauthenticatedRequest")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "Issuer")]
    pub issuer: String,
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: String,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: String,
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceIpConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "AuthenticateCognitoConfig")]
    pub authenticate_cognito_config: Option<AuthenticateCognitoConfig>,
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "FixedResponseConfig")]
    pub fixed_response_config: Option<FixedResponseConfig>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "RedirectConfig")]
    pub redirect_config: Option<RedirectConfig>,
    #[serde(rename = "Order")]
    pub order: Option<usize>,
    #[serde(rename = "AuthenticateOidcConfig")]
    pub authenticate_oidc_config: Option<AuthenticateOidcConfig>,
    #[serde(rename = "ForwardConfig")]
    pub forward_config: Option<ForwardConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpHeaderConfig {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "HttpHeaderName")]
    pub http_header_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthenticateCognitoConfig {
    #[serde(rename = "UserPoolClientId")]
    pub user_pool_client_id: String,
    #[serde(rename = "OnUnauthenticatedRequest")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "UserPoolArn")]
    pub user_pool_arn: String,
    #[serde(rename = "SessionCookieName")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "UserPoolDomain")]
    pub user_pool_domain: String,
    #[serde(rename = "AuthenticationRequestExtraParams")]
    pub authentication_request_extra_params: Option<()>,
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<usize>,

}


}

pub mod cfn_load_balancer {

#[derive(serde::Serialize, Default)]
pub struct CfnLoadBalancer {
    /// No documentation provided by AWS
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,
    /// List of SubnetMapping
    #[serde(rename = "SubnetMappings")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,
    /// No documentation provided by AWS
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of LoadBalancerAttribute
    #[serde(rename = "LoadBalancerAttributes")]
    pub load_balancer_attributes: Option<Vec<LoadBalancerAttribute>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct LoadBalancerAttribute {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SubnetMapping {
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    #[serde(rename = "PrivateIPv4Address")]
    pub private_ipv4_address: Option<String>,
    #[serde(rename = "AllocationId")]
    pub allocation_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_target_group {

#[derive(serde::Serialize, Default)]
pub struct CfnTargetGroup {
    /// [HTTP/HTTPS health checks] The HTTP or gRPC codes to use when checking for a successful response from a target.
    #[serde(rename = "Matcher")]
    pub matcher: Option<Matcher>,
    /// Indicates whether health checks are enabled. If the target type is lambda, health checks are disabled by default but can be enabled. If the target type is instance, ip, or alb, health checks are always enabled and cannot be disabled.
    #[serde(rename = "HealthCheckEnabled")]
    pub health_check_enabled: Option<bool>,
    /// The attributes.
    #[serde(rename = "TargetGroupAttributes")]
    pub target_group_attributes: Option<Vec<TargetGroupAttribute>>,
    /// The type of IP address used for this target group. The possible values are ipv4 and ipv6.
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,
    /// The number of consecutive health check failures required before considering a target unhealthy.
    #[serde(rename = "UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: Option<usize>,
    /// The tags.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The approximate amount of time, in seconds, between health checks of an individual target.
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<usize>,
    /// The port on which the targets receive traffic. This port is used unless you specify a port override when registering the target. If the target is a Lambda function, this parameter does not apply. If the protocol is GENEVE, the supported port is 6081.
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// The protocol the load balancer uses when performing health checks on targets.
    #[serde(rename = "HealthCheckProtocol")]
    pub health_check_protocol: Option<String>,
    /// [HTTP/HTTPS protocol] The protocol version. The possible values are GRPC, HTTP1, and HTTP2.
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,
    /// The protocol to use for routing traffic to the targets.
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    /// The port the load balancer uses when performing health checks on targets.
    #[serde(rename = "HealthCheckPort")]
    pub health_check_port: Option<String>,
    /// The identifier of the virtual private cloud (VPC). If the target is a Lambda function, this parameter does not apply.
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    /// The type of target that you must specify when registering targets with this target group. You can't specify targets for a target group using more than one target type.
    #[serde(rename = "TargetType")]
    pub target_type: Option<String>,
    /// The targets.
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<TargetDescription>>,
    /// The amount of time, in seconds, during which no response from a target means a failed health check.
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: Option<usize>,
    /// [HTTP/HTTPS health checks] The destination for health checks on the targets. [HTTP1 or HTTP2 protocol version] The ping path. The default is /. [GRPC protocol version] The path of a custom health check method with the format /package.service/method. The default is /AWS.ALB/healthcheck.
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,
    /// The number of consecutive health checks successes required before considering an unhealthy target healthy.
    #[serde(rename = "HealthyThresholdCount")]
    pub healthy_threshold_count: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupAttribute {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Matcher {
    #[serde(rename = "GrpcCode")]
    pub grpc_code: Option<String>,
    #[serde(rename = "HttpCode")]
    pub http_code: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetDescription {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}


}
