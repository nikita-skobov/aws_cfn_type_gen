
pub mod cfn_cache_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnCachePolicy {
    /// No documentation provided by AWS
    #[serde(rename = "CachePolicyConfig")]
    pub cache_policy_config: CachePolicyConfig,

}


#[derive(serde::Serialize, Default)]
pub struct HeadersConfig {
    #[serde(rename = "HeaderBehavior")]
    pub header_behavior: String,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct CookiesConfig {
    #[serde(rename = "Cookies")]
    pub cookies: Option<Vec<String>>,
    #[serde(rename = "CookieBehavior")]
    pub cookie_behavior: String,

}

#[derive(serde::Serialize, Default)]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(rename = "CookiesConfig")]
    pub cookies_config: CookiesConfig,
    #[serde(rename = "QueryStringsConfig")]
    pub query_strings_config: QueryStringsConfig,
    #[serde(rename = "HeadersConfig")]
    pub headers_config: HeadersConfig,
    #[serde(rename = "EnableAcceptEncodingGzip")]
    pub enable_accept_encoding_gzip: bool,
    #[serde(rename = "EnableAcceptEncodingBrotli")]
    pub enable_accept_encoding_brotli: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CachePolicyConfig {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
    pub parameters_in_cache_key_and_forwarded_to_origin: ParametersInCacheKeyAndForwardedToOrigin,
    #[serde(rename = "DefaultTTL")]
    pub default_ttl: f64,
    #[serde(rename = "MaxTTL")]
    pub max_ttl: f64,
    #[serde(rename = "MinTTL")]
    pub min_ttl: f64,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryStringsConfig {
    #[serde(rename = "QueryStrings")]
    pub query_strings: Option<Vec<String>>,
    #[serde(rename = "QueryStringBehavior")]
    pub query_string_behavior: String,

}


}

pub mod cfn_cloud_front_origin_access_identity {

#[derive(serde::Serialize, Default)]
pub struct CfnCloudFrontOriginAccessIdentity {
    /// No documentation provided by AWS
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,

}


#[derive(serde::Serialize, Default)]
pub struct CloudFrontOriginAccessIdentityConfig {
    #[serde(rename = "Comment")]
    pub comment: String,

}


}

pub mod cfn_continuous_deployment_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnContinuousDeploymentPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    pub continuous_deployment_policy_config: ContinuousDeploymentPolicyConfig,

}


#[derive(serde::Serialize, Default)]
pub struct SingleWeightConfig {
    #[serde(rename = "Weight")]
    pub weight: f64,
    #[serde(rename = "SessionStickinessConfig")]
    pub session_stickiness_config: Option<SessionStickinessConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct SessionStickinessConfig {
    #[serde(rename = "IdleTTL")]
    pub idle_ttl: usize,
    #[serde(rename = "MaximumTTL")]
    pub maximum_ttl: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TrafficConfig {
    #[serde(rename = "SingleWeightConfig")]
    pub single_weight_config: Option<SingleWeightConfig>,
    #[serde(rename = "SingleHeaderConfig")]
    pub single_header_config: Option<SingleHeaderConfig>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct SingleHeaderConfig {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Header")]
    pub header: String,

}

#[derive(serde::Serialize, Default)]
pub struct ContinuousDeploymentPolicyConfig {
    #[serde(rename = "StagingDistributionDnsNames")]
    pub staging_distribution_dns_names: Vec<String>,
    #[serde(rename = "TrafficConfig")]
    pub traffic_config: Option<TrafficConfig>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}


}

pub mod cfn_distribution {

#[derive(serde::Serialize, Default)]
pub struct CfnDistribution {
    /// No documentation provided by AWS
    #[serde(rename = "DistributionConfig")]
    pub distribution_config: DistributionConfig,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct LambdaFunctionAssociation {
    #[serde(rename = "EventType")]
    pub event_type: Option<String>,
    #[serde(rename = "IncludeBody")]
    pub include_body: Option<bool>,
    #[serde(rename = "LambdaFunctionARN")]
    pub lambda_function_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LegacyCustomOrigin {
    #[serde(rename = "HTTPSPort")]
    pub httpsport: Option<usize>,
    #[serde(rename = "OriginProtocolPolicy")]
    pub origin_protocol_policy: String,
    #[serde(rename = "OriginSSLProtocols")]
    pub origin_sslprotocols: Vec<String>,
    #[serde(rename = "DNSName")]
    pub dnsname: String,
    #[serde(rename = "HTTPPort")]
    pub httpport: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginShield {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "OriginShieldRegion")]
    pub origin_shield_region: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultCacheBehavior {
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: Option<Vec<String>>,
    #[serde(rename = "LambdaFunctionAssociations")]
    pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,
    #[serde(rename = "TrustedKeyGroups")]
    pub trusted_key_groups: Option<Vec<String>>,
    #[serde(rename = "TrustedSigners")]
    pub trusted_signers: Option<Vec<String>>,
    #[serde(rename = "ForwardedValues")]
    pub forwarded_values: Option<ForwardedValues>,
    #[serde(rename = "Compress")]
    pub compress: Option<bool>,
    #[serde(rename = "ResponseHeadersPolicyId")]
    pub response_headers_policy_id: Option<String>,
    #[serde(rename = "DefaultTTL")]
    pub default_ttl: Option<f64>,
    #[serde(rename = "MinTTL")]
    pub min_ttl: Option<f64>,
    #[serde(rename = "CachePolicyId")]
    pub cache_policy_id: Option<String>,
    #[serde(rename = "TargetOriginId")]
    pub target_origin_id: String,
    #[serde(rename = "FieldLevelEncryptionId")]
    pub field_level_encryption_id: Option<String>,
    #[serde(rename = "MaxTTL")]
    pub max_ttl: Option<f64>,
    #[serde(rename = "OriginRequestPolicyId")]
    pub origin_request_policy_id: Option<String>,
    #[serde(rename = "RealtimeLogConfigArn")]
    pub realtime_log_config_arn: Option<String>,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub viewer_protocol_policy: String,
    #[serde(rename = "CachedMethods")]
    pub cached_methods: Option<Vec<String>>,
    #[serde(rename = "FunctionAssociations")]
    pub function_associations: Option<Vec<FunctionAssociation>>,
    #[serde(rename = "SmoothStreaming")]
    pub smooth_streaming: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct GeoRestriction {
    #[serde(rename = "RestrictionType")]
    pub restriction_type: String,
    #[serde(rename = "Locations")]
    pub locations: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Origin {
    #[serde(rename = "OriginPath")]
    pub origin_path: Option<String>,
    #[serde(rename = "CustomOriginConfig")]
    pub custom_origin_config: Option<CustomOriginConfig>,
    #[serde(rename = "ConnectionAttempts")]
    pub connection_attempts: Option<usize>,
    #[serde(rename = "OriginCustomHeaders")]
    pub origin_custom_headers: Option<Vec<OriginCustomHeader>>,
    #[serde(rename = "OriginShield")]
    pub origin_shield: Option<OriginShield>,
    #[serde(rename = "S3OriginConfig")]
    pub s3_origin_config: Option<S3OriginConfig>,
    #[serde(rename = "ConnectionTimeout")]
    pub connection_timeout: Option<usize>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "OriginAccessControlId")]
    pub origin_access_control_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginGroupMembers {
    #[serde(rename = "Items")]
    pub items: Vec<OriginGroupMember>,
    #[serde(rename = "Quantity")]
    pub quantity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct CustomOriginConfig {
    #[serde(rename = "OriginProtocolPolicy")]
    pub origin_protocol_policy: String,
    #[serde(rename = "HTTPPort")]
    pub httpport: Option<usize>,
    #[serde(rename = "OriginReadTimeout")]
    pub origin_read_timeout: Option<usize>,
    #[serde(rename = "OriginSSLProtocols")]
    pub origin_sslprotocols: Option<Vec<String>>,
    #[serde(rename = "HTTPSPort")]
    pub httpsport: Option<usize>,
    #[serde(rename = "OriginKeepaliveTimeout")]
    pub origin_keepalive_timeout: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginGroups {
    #[serde(rename = "Quantity")]
    pub quantity: usize,
    #[serde(rename = "Items")]
    pub items: Option<Vec<OriginGroup>>,

}

#[derive(serde::Serialize, Default)]
pub struct StatusCodes {
    #[serde(rename = "Items")]
    pub items: Vec<usize>,
    #[serde(rename = "Quantity")]
    pub quantity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct S3OriginConfig {
    #[serde(rename = "OriginAccessIdentity")]
    pub origin_access_identity: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginGroupMember {
    #[serde(rename = "OriginId")]
    pub origin_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Logging {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "IncludeCookies")]
    pub include_cookies: Option<bool>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Restrictions {
    #[serde(rename = "GeoRestriction")]
    pub geo_restriction: GeoRestriction,

}

#[derive(serde::Serialize, Default)]
pub struct OriginGroup {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "FailoverCriteria")]
    pub failover_criteria: OriginGroupFailoverCriteria,
    #[serde(rename = "Members")]
    pub members: OriginGroupMembers,

}

#[derive(serde::Serialize, Default)]
pub struct LegacyS3Origin {
    #[serde(rename = "DNSName")]
    pub dnsname: String,
    #[serde(rename = "OriginAccessIdentity")]
    pub origin_access_identity: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Cookies {
    #[serde(rename = "WhitelistedNames")]
    pub whitelisted_names: Option<Vec<String>>,
    #[serde(rename = "Forward")]
    pub forward: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct CacheBehavior {
    #[serde(rename = "CachedMethods")]
    pub cached_methods: Option<Vec<String>>,
    #[serde(rename = "MinTTL")]
    pub min_ttl: Option<f64>,
    #[serde(rename = "CachePolicyId")]
    pub cache_policy_id: Option<String>,
    #[serde(rename = "FieldLevelEncryptionId")]
    pub field_level_encryption_id: Option<String>,
    #[serde(rename = "LambdaFunctionAssociations")]
    pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,
    #[serde(rename = "ResponseHeadersPolicyId")]
    pub response_headers_policy_id: Option<String>,
    #[serde(rename = "TrustedKeyGroups")]
    pub trusted_key_groups: Option<Vec<String>>,
    #[serde(rename = "DefaultTTL")]
    pub default_ttl: Option<f64>,
    #[serde(rename = "RealtimeLogConfigArn")]
    pub realtime_log_config_arn: Option<String>,
    #[serde(rename = "ViewerProtocolPolicy")]
    pub viewer_protocol_policy: String,
    #[serde(rename = "FunctionAssociations")]
    pub function_associations: Option<Vec<FunctionAssociation>>,
    #[serde(rename = "OriginRequestPolicyId")]
    pub origin_request_policy_id: Option<String>,
    #[serde(rename = "MaxTTL")]
    pub max_ttl: Option<f64>,
    #[serde(rename = "TrustedSigners")]
    pub trusted_signers: Option<Vec<String>>,
    #[serde(rename = "Compress")]
    pub compress: Option<bool>,
    #[serde(rename = "SmoothStreaming")]
    pub smooth_streaming: Option<bool>,
    #[serde(rename = "PathPattern")]
    pub path_pattern: String,
    #[serde(rename = "TargetOriginId")]
    pub target_origin_id: String,
    #[serde(rename = "ForwardedValues")]
    pub forwarded_values: Option<ForwardedValues>,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct FunctionAssociation {
    #[serde(rename = "EventType")]
    pub event_type: Option<String>,
    #[serde(rename = "FunctionARN")]
    pub function_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomErrorResponse {
    #[serde(rename = "ResponsePagePath")]
    pub response_page_path: Option<String>,
    #[serde(rename = "ErrorCode")]
    pub error_code: usize,
    #[serde(rename = "ErrorCachingMinTTL")]
    pub error_caching_min_ttl: Option<f64>,
    #[serde(rename = "ResponseCode")]
    pub response_code: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginCustomHeader {
    #[serde(rename = "HeaderValue")]
    pub header_value: String,
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct OriginGroupFailoverCriteria {
    #[serde(rename = "StatusCodes")]
    pub status_codes: StatusCodes,

}

#[derive(serde::Serialize, Default)]
pub struct DistributionConfig {
    #[serde(rename = "CNAMEs")]
    pub cnames: Option<Vec<String>>,
    #[serde(rename = "CustomErrorResponses")]
    pub custom_error_responses: Option<Vec<CustomErrorResponse>>,
    #[serde(rename = "OriginGroups")]
    pub origin_groups: Option<OriginGroups>,
    #[serde(rename = "Origins")]
    pub origins: Option<Vec<Origin>>,
    #[serde(rename = "CustomOrigin")]
    pub custom_origin: Option<LegacyCustomOrigin>,
    #[serde(rename = "DefaultRootObject")]
    pub default_root_object: Option<String>,
    #[serde(rename = "HttpVersion")]
    pub http_version: Option<String>,
    #[serde(rename = "S3Origin")]
    pub s3_origin: Option<LegacyS3Origin>,
    #[serde(rename = "CacheBehaviors")]
    pub cache_behaviors: Option<Vec<CacheBehavior>>,
    #[serde(rename = "DefaultCacheBehavior")]
    pub default_cache_behavior: DefaultCacheBehavior,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Restrictions")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "Staging")]
    pub staging: Option<bool>,
    #[serde(rename = "ViewerCertificate")]
    pub viewer_certificate: Option<ViewerCertificate>,
    #[serde(rename = "ContinuousDeploymentPolicyId")]
    pub continuous_deployment_policy_id: Option<String>,
    #[serde(rename = "PriceClass")]
    pub price_class: Option<String>,
    #[serde(rename = "WebACLId")]
    pub web_aclid: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "IPV6Enabled")]
    pub ipv6_enabled: Option<bool>,
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,

}

#[derive(serde::Serialize, Default)]
pub struct ViewerCertificate {
    #[serde(rename = "AcmCertificateArn")]
    pub acm_certificate_arn: Option<String>,
    #[serde(rename = "IamCertificateId")]
    pub iam_certificate_id: Option<String>,
    #[serde(rename = "MinimumProtocolVersion")]
    pub minimum_protocol_version: Option<String>,
    #[serde(rename = "CloudFrontDefaultCertificate")]
    pub cloud_front_default_certificate: Option<bool>,
    #[serde(rename = "SslSupportMethod")]
    pub ssl_support_method: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ForwardedValues {
    #[serde(rename = "QueryString")]
    pub query_string: bool,
    #[serde(rename = "Cookies")]
    pub cookies: Option<Cookies>,
    #[serde(rename = "QueryStringCacheKeys")]
    pub query_string_cache_keys: Option<Vec<String>>,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<String>>,

}


}

pub mod cfn_function {

#[derive(serde::Serialize, Default)]
pub struct CfnFunction {
    /// No documentation provided by AWS
    #[serde(rename = "FunctionConfig")]
    pub function_config: FunctionConfig,
    /// No documentation provided by AWS
    #[serde(rename = "AutoPublish")]
    pub auto_publish: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionCode")]
    pub function_code: String,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionMetadata")]
    pub function_metadata: Option<FunctionMetadata>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct FunctionMetadata {
    #[serde(rename = "FunctionARN")]
    pub function_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FunctionConfig {
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Runtime")]
    pub runtime: String,

}


}

pub mod cfn_key_group {

#[derive(serde::Serialize, Default)]
pub struct CfnKeyGroup {
    /// No documentation provided by AWS
    #[serde(rename = "KeyGroupConfig")]
    pub key_group_config: KeyGroupConfig,

}


#[derive(serde::Serialize, Default)]
pub struct KeyGroupConfig {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Items")]
    pub items: Vec<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}


}

pub mod cfn_monitoring_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnMonitoringSubscription {
    /// No documentation provided by AWS
    #[serde(rename = "DistributionId")]
    pub distribution_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "MonitoringSubscription")]
    pub monitoring_subscription: MonitoringSubscription,

}


#[derive(serde::Serialize, Default)]
pub struct RealtimeMetricsSubscriptionConfig {
    #[serde(rename = "RealtimeMetricsSubscriptionStatus")]
    pub realtime_metrics_subscription_status: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringSubscription {
    #[serde(rename = "RealtimeMetricsSubscriptionConfig")]
    pub realtime_metrics_subscription_config: Option<RealtimeMetricsSubscriptionConfig>,

}


}

pub mod cfn_origin_access_control {

#[derive(serde::Serialize, Default)]
pub struct CfnOriginAccessControl {
    /// No documentation provided by AWS
    #[serde(rename = "OriginAccessControlConfig")]
    pub origin_access_control_config: OriginAccessControlConfig,

}


#[derive(serde::Serialize, Default)]
pub struct OriginAccessControlConfig {
    #[serde(rename = "SigningProtocol")]
    pub signing_protocol: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "SigningBehavior")]
    pub signing_behavior: String,
    #[serde(rename = "OriginAccessControlOriginType")]
    pub origin_access_control_origin_type: String,

}


}

pub mod cfn_origin_request_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnOriginRequestPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "OriginRequestPolicyConfig")]
    pub origin_request_policy_config: OriginRequestPolicyConfig,

}


#[derive(serde::Serialize, Default)]
pub struct QueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    pub query_string_behavior: String,
    #[serde(rename = "QueryStrings")]
    pub query_strings: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct CookiesConfig {
    #[serde(rename = "CookieBehavior")]
    pub cookie_behavior: String,
    #[serde(rename = "Cookies")]
    pub cookies: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct OriginRequestPolicyConfig {
    #[serde(rename = "CookiesConfig")]
    pub cookies_config: CookiesConfig,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "HeadersConfig")]
    pub headers_config: HeadersConfig,
    #[serde(rename = "QueryStringsConfig")]
    pub query_strings_config: QueryStringsConfig,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HeadersConfig {
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<String>>,
    #[serde(rename = "HeaderBehavior")]
    pub header_behavior: String,

}


}

pub mod cfn_public_key {

#[derive(serde::Serialize, Default)]
pub struct CfnPublicKey {
    /// No documentation provided by AWS
    #[serde(rename = "PublicKeyConfig")]
    pub public_key_config: PublicKeyConfig,

}


#[derive(serde::Serialize, Default)]
pub struct PublicKeyConfig {
    #[serde(rename = "EncodedKey")]
    pub encoded_key: String,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CallerReference")]
    pub caller_reference: String,

}


}

pub mod cfn_realtime_log_config {

#[derive(serde::Serialize, Default)]
pub struct CfnRealtimeLogConfig {
    /// No documentation provided by AWS
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: f64,
    /// No documentation provided by AWS
    #[serde(rename = "Fields")]
    pub fields: Vec<String>,
    /// List of EndPoint
    #[serde(rename = "EndPoints")]
    pub end_points: Vec<EndPoint>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct KinesisStreamConfig {
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct EndPoint {
    #[serde(rename = "StreamType")]
    pub stream_type: String,
    #[serde(rename = "KinesisStreamConfig")]
    pub kinesis_stream_config: KinesisStreamConfig,

}


}

pub mod cfn_response_headers_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResponseHeadersPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    pub response_headers_policy_config: ResponseHeadersPolicyConfig,

}


#[derive(serde::Serialize, Default)]
pub struct AccessControlAllowHeaders {
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StrictTransportSecurity {
    #[serde(rename = "Override")]
    pub overide: bool,
    #[serde(rename = "Preload")]
    pub preload: Option<bool>,
    #[serde(rename = "AccessControlMaxAgeSec")]
    pub access_control_max_age_sec: usize,
    #[serde(rename = "IncludeSubdomains")]
    pub include_subdomains: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlAllowMethods {
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlExposeHeaders {
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomHeader {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Override")]
    pub overide: bool,
    #[serde(rename = "Header")]
    pub header: String,

}

#[derive(serde::Serialize, Default)]
pub struct RemoveHeadersConfig {
    #[serde(rename = "Items")]
    pub items: Vec<RemoveHeader>,

}

#[derive(serde::Serialize, Default)]
pub struct ContentSecurityPolicy {
    #[serde(rename = "ContentSecurityPolicy")]
    pub content_security_policy: String,
    #[serde(rename = "Override")]
    pub overide: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ResponseHeadersPolicyConfig {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "SecurityHeadersConfig")]
    pub security_headers_config: Option<SecurityHeadersConfig>,
    #[serde(rename = "CustomHeadersConfig")]
    pub custom_headers_config: Option<CustomHeadersConfig>,
    #[serde(rename = "CorsConfig")]
    pub cors_config: Option<CorsConfig>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ServerTimingHeadersConfig")]
    pub server_timing_headers_config: Option<ServerTimingHeadersConfig>,
    #[serde(rename = "RemoveHeadersConfig")]
    pub remove_headers_config: Option<RemoveHeadersConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ServerTimingHeadersConfig {
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: Option<f64>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct RemoveHeader {
    #[serde(rename = "Header")]
    pub header: String,

}

#[derive(serde::Serialize, Default)]
pub struct XSSProtection {
    #[serde(rename = "ModeBlock")]
    pub mode_block: Option<bool>,
    #[serde(rename = "Override")]
    pub overide: bool,
    #[serde(rename = "Protection")]
    pub protection: bool,
    #[serde(rename = "ReportUri")]
    pub report_uri: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomHeadersConfig {
    #[serde(rename = "Items")]
    pub items: Vec<CustomHeader>,

}

#[derive(serde::Serialize, Default)]
pub struct ReferrerPolicy {
    #[serde(rename = "Override")]
    pub overide: bool,
    #[serde(rename = "ReferrerPolicy")]
    pub referrer_policy: String,

}

#[derive(serde::Serialize, Default)]
pub struct SecurityHeadersConfig {
    #[serde(rename = "ContentSecurityPolicy")]
    pub content_security_policy: Option<ContentSecurityPolicy>,
    #[serde(rename = "ReferrerPolicy")]
    pub referrer_policy: Option<ReferrerPolicy>,
    #[serde(rename = "FrameOptions")]
    pub frame_options: Option<FrameOptions>,
    #[serde(rename = "ContentTypeOptions")]
    pub content_type_options: Option<ContentTypeOptions>,
    #[serde(rename = "StrictTransportSecurity")]
    pub strict_transport_security: Option<StrictTransportSecurity>,
    #[serde(rename = "XSSProtection")]
    pub xssprotection: Option<XSSProtection>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlAllowOrigins {
    #[serde(rename = "Items")]
    pub items: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CorsConfig {
    #[serde(rename = "OriginOverride")]
    pub origin_override: bool,
    #[serde(rename = "AccessControlAllowMethods")]
    pub access_control_allow_methods: AccessControlAllowMethods,
    #[serde(rename = "AccessControlAllowOrigins")]
    pub access_control_allow_origins: AccessControlAllowOrigins,
    #[serde(rename = "AccessControlExposeHeaders")]
    pub access_control_expose_headers: Option<AccessControlExposeHeaders>,
    #[serde(rename = "AccessControlMaxAgeSec")]
    pub access_control_max_age_sec: Option<usize>,
    #[serde(rename = "AccessControlAllowHeaders")]
    pub access_control_allow_headers: AccessControlAllowHeaders,
    #[serde(rename = "AccessControlAllowCredentials")]
    pub access_control_allow_credentials: bool,

}

#[derive(serde::Serialize, Default)]
pub struct FrameOptions {
    #[serde(rename = "FrameOption")]
    pub frame_option: String,
    #[serde(rename = "Override")]
    pub overide: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ContentTypeOptions {
    #[serde(rename = "Override")]
    pub overide: bool,

}


}

pub mod cfn_streaming_distribution {

#[derive(serde::Serialize, Default)]
pub struct CfnStreamingDistribution {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    /// No documentation provided by AWS
    #[serde(rename = "StreamingDistributionConfig")]
    pub streaming_distribution_config: StreamingDistributionConfig,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Logging {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct S3Origin {
    #[serde(rename = "OriginAccessIdentity")]
    pub origin_access_identity: String,
    #[serde(rename = "DomainName")]
    pub domain_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct StreamingDistributionConfig {
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,
    #[serde(rename = "PriceClass")]
    pub price_class: Option<String>,
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "S3Origin")]
    pub s3_origin: S3Origin,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "TrustedSigners")]
    pub trusted_signers: TrustedSigners,

}

#[derive(serde::Serialize, Default)]
pub struct TrustedSigners {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "AwsAccountNumbers")]
    pub aws_account_numbers: Option<Vec<String>>,

}


}
