
pub mod cfn_gateway_route {

#[derive(serde::Serialize, Default)]
pub struct CfnGatewayRoute {
    /// No documentation provided by AWS
    #[serde(rename = "VirtualGatewayName")]
    pub virtual_gateway_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: GatewayRouteSpec,
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "GatewayRouteName")]
    pub gateway_route_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRouteHeader {
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,
    #[serde(rename = "Match")]
    pub mtch: Option<HttpGatewayRouteHeaderMatch>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryParameter {
    #[serde(rename = "Match")]
    pub mtch: Option<HttpQueryParameterMatch>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRoutePathRewrite {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcGatewayRouteMetadata {
    #[serde(rename = "Match")]
    pub mtch: Option<GatewayRouteMetadataMatch>,
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteHostnameMatch {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcGatewayRouteMatch {
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameMatch>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Vec<GrpcGatewayRouteMetadata>>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRouteAction {
    #[serde(rename = "Target")]
    pub target: GatewayRouteTarget,
    #[serde(rename = "Rewrite")]
    pub rewrite: Option<HttpGatewayRouteRewrite>,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteRangeMatch {
    #[serde(rename = "End")]
    pub end: usize,
    #[serde(rename = "Start")]
    pub start: usize,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRoute {
    #[serde(rename = "Action")]
    pub action: HttpGatewayRouteAction,
    #[serde(rename = "Match")]
    pub mtch: HttpGatewayRouteMatch,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRoutePrefixRewrite {
    #[serde(rename = "DefaultPrefix")]
    pub default_prefix: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcGatewayRouteAction {
    #[serde(rename = "Target")]
    pub target: GatewayRouteTarget,
    #[serde(rename = "Rewrite")]
    pub rewrite: Option<GrpcGatewayRouteRewrite>,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteMetadataMatch {
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<GatewayRouteRangeMatch>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpPathMatch {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Regex")]
    pub regex: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteHostnameRewrite {
    #[serde(rename = "DefaultTargetHostname")]
    pub default_target_hostname: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRouteHeaderMatch {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<GatewayRouteRangeMatch>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "Regex")]
    pub regex: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteTarget {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "VirtualService")]
    pub virtual_service: GatewayRouteVirtualService,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRouteMatch {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpGatewayRouteHeader>>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Method")]
    pub method: Option<String>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameMatch>,
    #[serde(rename = "Path")]
    pub path: Option<HttpPathMatch>,
    #[serde(rename = "QueryParameters")]
    pub query_parameters: Option<Vec<QueryParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpQueryParameterMatch {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcGatewayRouteRewrite {
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcGatewayRoute {
    #[serde(rename = "Action")]
    pub action: GrpcGatewayRouteAction,
    #[serde(rename = "Match")]
    pub mtch: GrpcGatewayRouteMatch,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteSpec {
    #[serde(rename = "HttpRoute")]
    pub http_route: Option<HttpGatewayRoute>,
    #[serde(rename = "GrpcRoute")]
    pub grpc_route: Option<GrpcGatewayRoute>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "Http2Route")]
    pub http2_route: Option<HttpGatewayRoute>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpGatewayRouteRewrite {
    #[serde(rename = "Prefix")]
    pub prefix: Option<HttpGatewayRoutePrefixRewrite>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,
    #[serde(rename = "Path")]
    pub path: Option<HttpGatewayRoutePathRewrite>,

}

#[derive(serde::Serialize, Default)]
pub struct GatewayRouteVirtualService {
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,

}


}

pub mod cfn_mesh {

#[derive(serde::Serialize, Default)]
pub struct CfnMesh {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: Option<MeshSpec>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MeshServiceDiscovery {
    #[serde(rename = "IpPreference")]
    pub ip_preference: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EgressFilter {
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct MeshSpec {
    #[serde(rename = "EgressFilter")]
    pub egress_filter: Option<EgressFilter>,
    #[serde(rename = "ServiceDiscovery")]
    pub service_discovery: Option<MeshServiceDiscovery>,

}


}

pub mod cfn_route {

#[derive(serde::Serialize, Default)]
pub struct CfnRoute {
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: RouteSpec,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualRouterName")]
    pub virtual_router_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteName")]
    pub route_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct GrpcTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,
    #[serde(rename = "PerRequest")]
    pub per_request: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRouteHeader {
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Match")]
    pub mtch: Option<HeaderMatchMethod>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRetryPolicy {
    #[serde(rename = "TcpRetryEvents")]
    pub tcp_retry_events: Option<Vec<String>>,
    #[serde(rename = "PerRetryTimeout")]
    pub per_retry_timeout: Duration,
    #[serde(rename = "MaxRetries")]
    pub max_retries: usize,
    #[serde(rename = "HttpRetryEvents")]
    pub http_retry_events: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct TcpRouteMatch {
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpQueryParameterMatch {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRoute {
    #[serde(rename = "Action")]
    pub action: GrpcRouteAction,
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<GrpcRetryPolicy>,
    #[serde(rename = "Match")]
    pub mtch: GrpcRouteMatch,
    #[serde(rename = "Timeout")]
    pub timeout: Option<GrpcTimeout>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderMatchMethod {
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<MatchRange>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TcpTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRouteMetadata {
    #[serde(rename = "Match")]
    pub mtch: Option<GrpcRouteMetadataMatchMethod>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRouteMatch {
    #[serde(rename = "Metadata")]
    pub metadata: Option<Vec<GrpcRouteMetadata>>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "MethodName")]
    pub method_name: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,
    #[serde(rename = "PerRequest")]
    pub per_request: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRoute {
    #[serde(rename = "Timeout")]
    pub timeout: Option<HttpTimeout>,
    #[serde(rename = "Match")]
    pub mtch: HttpRouteMatch,
    #[serde(rename = "Action")]
    pub action: HttpRouteAction,
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<HttpRetryPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct WeightedTarget {
    #[serde(rename = "Weight")]
    pub weight: usize,
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "VirtualNode")]
    pub virtual_node: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryParameter {
    #[serde(rename = "Match")]
    pub mtch: Option<HttpQueryParameterMatch>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Duration {
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Value")]
    pub value: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRouteAction {
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRouteMetadataMatchMethod {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<MatchRange>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TcpRoute {
    #[serde(rename = "Match")]
    pub mtch: Option<TcpRouteMatch>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<TcpTimeout>,
    #[serde(rename = "Action")]
    pub action: TcpRouteAction,

}

#[derive(serde::Serialize, Default)]
pub struct HttpRouteMatch {
    #[serde(rename = "Path")]
    pub path: Option<HttpPathMatch>,
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Method")]
    pub method: Option<String>,
    #[serde(rename = "QueryParameters")]
    pub query_parameters: Option<Vec<QueryParameter>>,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpRouteHeader>>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRouteAction {
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct RouteSpec {
    #[serde(rename = "TcpRoute")]
    pub tcp_route: Option<TcpRoute>,
    #[serde(rename = "Http2Route")]
    pub http2_route: Option<HttpRoute>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "HttpRoute")]
    pub http_route: Option<HttpRoute>,
    #[serde(rename = "GrpcRoute")]
    pub grpc_route: Option<GrpcRoute>,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcRetryPolicy {
    #[serde(rename = "MaxRetries")]
    pub max_retries: usize,
    #[serde(rename = "HttpRetryEvents")]
    pub http_retry_events: Option<Vec<String>>,
    #[serde(rename = "PerRetryTimeout")]
    pub per_retry_timeout: Duration,
    #[serde(rename = "GrpcRetryEvents")]
    pub grpc_retry_events: Option<Vec<String>>,
    #[serde(rename = "TcpRetryEvents")]
    pub tcp_retry_events: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct TcpRouteAction {
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct MatchRange {
    #[serde(rename = "Start")]
    pub start: usize,
    #[serde(rename = "End")]
    pub end: usize,

}

#[derive(serde::Serialize, Default)]
pub struct HttpPathMatch {
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


}

pub mod cfn_virtual_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualGateway {
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: VirtualGatewaySpec,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualGatewayName")]
    pub virtual_gateway_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayTlsValidationContext {
    #[serde(rename = "Trust")]
    pub trust: VirtualGatewayTlsValidationContextTrust,
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewaySpec {
    #[serde(rename = "Listeners")]
    pub listeners: Vec<VirtualGatewayListener>,
    #[serde(rename = "BackendDefaults")]
    pub backend_defaults: Option<VirtualGatewayBackendDefaults>,
    #[serde(rename = "Logging")]
    pub logging: Option<VirtualGatewayLogging>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListener {
    #[serde(rename = "ConnectionPool")]
    pub connection_pool: Option<VirtualGatewayConnectionPool>,
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<VirtualGatewayHealthCheckPolicy>,
    #[serde(rename = "PortMapping")]
    pub port_mapping: VirtualGatewayPortMapping,
    #[serde(rename = "TLS")]
    pub tls: Option<VirtualGatewayListenerTls>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTls {
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Certificate")]
    pub certificate: VirtualGatewayListenerTlsCertificate,
    #[serde(rename = "Validation")]
    pub validation: Option<VirtualGatewayListenerTlsValidationContext>,

}

#[derive(serde::Serialize, Default)]
pub struct SubjectAlternativeNames {
    #[serde(rename = "Match")]
    pub mtch: SubjectAlternativeNameMatchers,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayHealthCheckPolicy {
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: usize,
    #[serde(rename = "IntervalMillis")]
    pub interval_millis: usize,
    #[serde(rename = "TimeoutMillis")]
    pub timeout_millis: usize,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: usize,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SubjectAlternativeNameMatchers {
    #[serde(rename = "Exact")]
    pub exact: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayClientTlsCertificate {
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayFileAccessLog {
    #[serde(rename = "Format")]
    pub format: Option<LoggingFormat>,
    #[serde(rename = "Path")]
    pub path: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayHttpConnectionPool {
    #[serde(rename = "MaxConnections")]
    pub max_connections: usize,
    #[serde(rename = "MaxPendingRequests")]
    pub max_pending_requests: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsValidationContext {
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(rename = "Trust")]
    pub trust: VirtualGatewayListenerTlsValidationContextTrust,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsValidationContextTrust {
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayClientPolicy {
    #[serde(rename = "TLS")]
    pub tls: Option<VirtualGatewayClientPolicyTls>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsCertificate {
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,
    #[serde(rename = "ACM")]
    pub acm: Option<VirtualGatewayListenerTlsAcmCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayBackendDefaults {
    #[serde(rename = "ClientPolicy")]
    pub client_policy: Option<VirtualGatewayClientPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingFormat {
    #[serde(rename = "Json")]
    pub json: Option<Vec<JsonFormatRef>>,
    #[serde(rename = "Text")]
    pub text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayTlsValidationContextFileTrust {
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayClientPolicyTls {
    #[serde(rename = "Enforce")]
    pub enforce: Option<bool>,
    #[serde(rename = "Validation")]
    pub validation: VirtualGatewayTlsValidationContext,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<usize>>,
    #[serde(rename = "Certificate")]
    pub certificate: Option<VirtualGatewayClientTlsCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayPortMapping {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Port")]
    pub port: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayTlsValidationContextAcmTrust {
    #[serde(rename = "CertificateAuthorityArns")]
    pub certificate_authority_arns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayTlsValidationContextSdsTrust {
    #[serde(rename = "SecretName")]
    pub secret_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsFileCertificate {
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayConnectionPool {
    #[serde(rename = "HTTP2")]
    pub http2: Option<VirtualGatewayHttp2ConnectionPool>,
    #[serde(rename = "HTTP")]
    pub http: Option<VirtualGatewayHttpConnectionPool>,
    #[serde(rename = "GRPC")]
    pub grpc: Option<VirtualGatewayGrpcConnectionPool>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayAccessLog {
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayFileAccessLog>,

}

#[derive(serde::Serialize, Default)]
pub struct JsonFormatRef {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayHttp2ConnectionPool {
    #[serde(rename = "MaxRequests")]
    pub max_requests: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayGrpcConnectionPool {
    #[serde(rename = "MaxRequests")]
    pub max_requests: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsSdsCertificate {
    #[serde(rename = "SecretName")]
    pub secret_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayTlsValidationContextTrust {
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
    #[serde(rename = "ACM")]
    pub acm: Option<VirtualGatewayTlsValidationContextAcmTrust>,
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayLogging {
    #[serde(rename = "AccessLog")]
    pub access_log: Option<VirtualGatewayAccessLog>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualGatewayListenerTlsAcmCertificate {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}


}

pub mod cfn_virtual_node {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualNode {
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: VirtualNodeSpec,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualNodeName")]
    pub virtual_node_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct VirtualNodeTcpConnectionPool {
    #[serde(rename = "MaxConnections")]
    pub max_connections: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeConnectionPool {
    #[serde(rename = "TCP")]
    pub tcp: Option<VirtualNodeTcpConnectionPool>,
    #[serde(rename = "HTTP")]
    pub http: Option<VirtualNodeHttpConnectionPool>,
    #[serde(rename = "HTTP2")]
    pub http2: Option<VirtualNodeHttp2ConnectionPool>,
    #[serde(rename = "GRPC")]
    pub grpc: Option<VirtualNodeGrpcConnectionPool>,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTimeout {
    #[serde(rename = "TCP")]
    pub tcp: Option<TcpTimeout>,
    #[serde(rename = "GRPC")]
    pub grpc: Option<GrpcTimeout>,
    #[serde(rename = "HTTP")]
    pub http: Option<HttpTimeout>,
    #[serde(rename = "HTTP2")]
    pub http2: Option<HttpTimeout>,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsValidationContext {
    #[serde(rename = "Trust")]
    pub trust: ListenerTlsValidationContextTrust,
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

}

#[derive(serde::Serialize, Default)]
pub struct DnsServiceDiscovery {
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "ResponseType")]
    pub response_type: Option<String>,
    #[serde(rename = "IpPreference")]
    pub ip_preference: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JsonFormatRef {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ClientPolicy {
    #[serde(rename = "TLS")]
    pub tls: Option<ClientPolicyTls>,

}

#[derive(serde::Serialize, Default)]
pub struct PortMapping {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Port")]
    pub port: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeSpec {
    #[serde(rename = "Backends")]
    pub backends: Option<Vec<Backend>>,
    #[serde(rename = "ServiceDiscovery")]
    pub service_discovery: Option<ServiceDiscovery>,
    #[serde(rename = "Listeners")]
    pub listeners: Option<Vec<Listener>>,
    #[serde(rename = "BackendDefaults")]
    pub backend_defaults: Option<BackendDefaults>,
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,

}

#[derive(serde::Serialize, Default)]
pub struct SubjectAlternativeNameMatchers {
    #[serde(rename = "Exact")]
    pub exact: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientPolicyTls {
    #[serde(rename = "Validation")]
    pub validation: TlsValidationContext,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<usize>>,
    #[serde(rename = "Enforce")]
    pub enforce: Option<bool>,
    #[serde(rename = "Certificate")]
    pub certificate: Option<ClientTlsCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Duration {
    #[serde(rename = "Unit")]
    pub unit: String,
    #[serde(rename = "Value")]
    pub value: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsCertificate {
    #[serde(rename = "SDS")]
    pub sds: Option<ListenerTlsSdsCertificate>,
    #[serde(rename = "ACM")]
    pub acm: Option<ListenerTlsAcmCertificate>,
    #[serde(rename = "File")]
    pub file: Option<ListenerTlsFileCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualServiceBackend {
    #[serde(rename = "ClientPolicy")]
    pub client_policy: Option<ClientPolicy>,
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SubjectAlternativeNames {
    #[serde(rename = "Match")]
    pub mtch: SubjectAlternativeNameMatchers,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeGrpcConnectionPool {
    #[serde(rename = "MaxRequests")]
    pub max_requests: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TcpTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeHttp2ConnectionPool {
    #[serde(rename = "MaxRequests")]
    pub max_requests: usize,

}

#[derive(serde::Serialize, Default)]
pub struct AccessLog {
    #[serde(rename = "File")]
    pub file: Option<FileAccessLog>,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsAcmCertificate {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct GrpcTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,
    #[serde(rename = "PerRequest")]
    pub per_request: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct TlsValidationContextTrust {
    #[serde(rename = "ACM")]
    pub acm: Option<TlsValidationContextAcmTrust>,
    #[serde(rename = "SDS")]
    pub sds: Option<TlsValidationContextSdsTrust>,
    #[serde(rename = "File")]
    pub file: Option<TlsValidationContextFileTrust>,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheck {
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: usize,
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "IntervalMillis")]
    pub interval_millis: usize,
    #[serde(rename = "TimeoutMillis")]
    pub timeout_millis: usize,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TlsValidationContext {
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,
    #[serde(rename = "Trust")]
    pub trust: TlsValidationContextTrust,

}

#[derive(serde::Serialize, Default)]
pub struct BackendDefaults {
    #[serde(rename = "ClientPolicy")]
    pub client_policy: Option<ClientPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsValidationContextTrust {
    #[serde(rename = "File")]
    pub file: Option<TlsValidationContextFileTrust>,
    #[serde(rename = "SDS")]
    pub sds: Option<TlsValidationContextSdsTrust>,

}

#[derive(serde::Serialize, Default)]
pub struct TlsValidationContextSdsTrust {
    #[serde(rename = "SecretName")]
    pub secret_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTls {
    #[serde(rename = "Certificate")]
    pub certificate: ListenerTlsCertificate,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Validation")]
    pub validation: Option<ListenerTlsValidationContext>,

}

#[derive(serde::Serialize, Default)]
pub struct Logging {
    #[serde(rename = "AccessLog")]
    pub access_log: Option<AccessLog>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientTlsCertificate {
    #[serde(rename = "SDS")]
    pub sds: Option<ListenerTlsSdsCertificate>,
    #[serde(rename = "File")]
    pub file: Option<ListenerTlsFileCertificate>,

}

#[derive(serde::Serialize, Default)]
pub struct TlsValidationContextAcmTrust {
    #[serde(rename = "CertificateAuthorityArns")]
    pub certificate_authority_arns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeHttpConnectionPool {
    #[serde(rename = "MaxPendingRequests")]
    pub max_pending_requests: Option<usize>,
    #[serde(rename = "MaxConnections")]
    pub max_connections: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TlsValidationContextFileTrust {
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,

}

#[derive(serde::Serialize, Default)]
pub struct OutlierDetection {
    #[serde(rename = "MaxEjectionPercent")]
    pub max_ejection_percent: usize,
    #[serde(rename = "MaxServerErrors")]
    pub max_server_errors: usize,
    #[serde(rename = "BaseEjectionDuration")]
    pub base_ejection_duration: Duration,
    #[serde(rename = "Interval")]
    pub interval: Duration,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingFormat {
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Json")]
    pub json: Option<Vec<JsonFormatRef>>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpTimeout {
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,
    #[serde(rename = "PerRequest")]
    pub per_request: Option<Duration>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceDiscovery {
    #[serde(rename = "DNS")]
    pub dns: Option<DnsServiceDiscovery>,
    #[serde(rename = "AWSCloudMap")]
    pub awscloud_map: Option<AwsCloudMapServiceDiscovery>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsCloudMapServiceDiscovery {
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    #[serde(rename = "Attributes")]
    pub attributes: Option<Vec<AwsCloudMapInstanceAttribute>>,
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "IpPreference")]
    pub ip_preference: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Listener {
    #[serde(rename = "ConnectionPool")]
    pub connection_pool: Option<VirtualNodeConnectionPool>,
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheck>,
    #[serde(rename = "PortMapping")]
    pub port_mapping: PortMapping,
    #[serde(rename = "OutlierDetection")]
    pub outlier_detection: Option<OutlierDetection>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<ListenerTimeout>,
    #[serde(rename = "TLS")]
    pub tls: Option<ListenerTls>,

}

#[derive(serde::Serialize, Default)]
pub struct Backend {
    #[serde(rename = "VirtualService")]
    pub virtual_service: Option<VirtualServiceBackend>,

}

#[derive(serde::Serialize, Default)]
pub struct FileAccessLog {
    #[serde(rename = "Format")]
    pub format: Option<LoggingFormat>,
    #[serde(rename = "Path")]
    pub path: String,

}

#[derive(serde::Serialize, Default)]
pub struct AwsCloudMapInstanceAttribute {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsFileCertificate {
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,
    #[serde(rename = "PrivateKey")]
    pub private_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ListenerTlsSdsCertificate {
    #[serde(rename = "SecretName")]
    pub secret_name: String,

}


}

pub mod cfn_virtual_router {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualRouter {
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualRouterName")]
    pub virtual_router_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: VirtualRouterSpec,

}


#[derive(serde::Serialize, Default)]
pub struct PortMapping {
    #[serde(rename = "Port")]
    pub port: usize,
    #[serde(rename = "Protocol")]
    pub protocol: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualRouterSpec {
    #[serde(rename = "Listeners")]
    pub listeners: Vec<VirtualRouterListener>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualRouterListener {
    #[serde(rename = "PortMapping")]
    pub port_mapping: PortMapping,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_virtual_service {

#[derive(serde::Serialize, Default)]
pub struct CfnVirtualService {
    /// No documentation provided by AWS
    #[serde(rename = "MeshName")]
    pub mesh_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Spec")]
    pub spec: VirtualServiceSpec,
    /// No documentation provided by AWS
    #[serde(rename = "MeshOwner")]
    pub mesh_owner: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct VirtualServiceSpec {
    #[serde(rename = "Provider")]
    pub provider: Option<VirtualServiceProvider>,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualServiceProvider {
    #[serde(rename = "VirtualRouter")]
    pub virtual_router: Option<VirtualRouterServiceProvider>,
    #[serde(rename = "VirtualNode")]
    pub virtual_node: Option<VirtualNodeServiceProvider>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualRouterServiceProvider {
    #[serde(rename = "VirtualRouterName")]
    pub virtual_router_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct VirtualNodeServiceProvider {
    #[serde(rename = "VirtualNodeName")]
    pub virtual_node_name: String,

}


}
