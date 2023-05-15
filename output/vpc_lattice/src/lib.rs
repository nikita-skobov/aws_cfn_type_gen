
pub mod cfn_access_log_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessLogSubscription {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_auth_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnAuthPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: (),

}



}

pub mod cfn_listener {

#[derive(serde::Serialize, Default)]
pub struct CfnListener {
    /// No documentation provided by AWS
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,
    /// No documentation provided by AWS
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Forward {
    #[serde(rename = "TargetGroups")]
    pub target_groups: Vec<WeightedTargetGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct WeightedTargetGroup {
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "TargetGroupIdentifier")]
    pub target_group_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct FixedResponse {
    #[serde(rename = "StatusCode")]
    pub status_code: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultAction {
    #[serde(rename = "Forward")]
    pub forward: Option<Forward>,
    #[serde(rename = "FixedResponse")]
    pub fixed_response: Option<FixedResponse>,

}


}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Policy")]
    pub policy: (),

}



}

pub mod cfn_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnRule {
    /// No documentation provided by AWS
    #[serde(rename = "Match")]
    pub mtch: Match,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: Action,
    /// No documentation provided by AWS
    #[serde(rename = "Priority")]
    pub priority: usize,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ListenerIdentifier")]
    pub listener_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct HeaderMatchType {
    #[serde(rename = "Contains")]
    pub contains: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FixedResponse {
    #[serde(rename = "StatusCode")]
    pub status_code: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "Forward")]
    pub forward: Option<Forward>,
    #[serde(rename = "FixedResponse")]
    pub fixed_response: Option<FixedResponse>,

}

#[derive(serde::Serialize, Default)]
pub struct Match {
    #[serde(rename = "HttpMatch")]
    pub http_match: HttpMatch,

}

#[derive(serde::Serialize, Default)]
pub struct HttpMatch {
    #[serde(rename = "PathMatch")]
    pub path_match: Option<PathMatch>,
    #[serde(rename = "Method")]
    pub method: Option<String>,
    #[serde(rename = "HeaderMatches")]
    pub header_matches: Option<Vec<HeaderMatch>>,

}

#[derive(serde::Serialize, Default)]
pub struct PathMatchType {
    #[serde(rename = "Exact")]
    pub exact: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderMatch {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Match")]
    pub mtch: HeaderMatchType,
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PathMatch {
    #[serde(rename = "Match")]
    pub mtch: PathMatchType,
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct WeightedTargetGroup {
    #[serde(rename = "TargetGroupIdentifier")]
    pub target_group_identifier: String,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Forward {
    #[serde(rename = "TargetGroups")]
    pub target_groups: Vec<WeightedTargetGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service {

#[derive(serde::Serialize, Default)]
pub struct CfnService {
    /// No documentation provided by AWS
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DnsEntry")]
    pub dns_entry: Option<DnsEntry>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomDomainName")]
    pub custom_domain_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DnsEntry {
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service_network {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceNetwork {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service_network_service_association {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceNetworkServiceAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceNetworkIdentifier")]
    pub service_network_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DnsEntry")]
    pub dns_entry: Option<DnsEntry>,

}


#[derive(serde::Serialize, Default)]
pub struct DnsEntry {
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_service_network_vpc_association {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceNetworkVpcAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "VpcIdentifier")]
    pub vpc_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceNetworkIdentifier")]
    pub service_network_identifier: Option<String>,

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
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// List of Target
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,
    /// No documentation provided by AWS
    #[serde(rename = "Config")]
    pub config: Option<TargetGroupConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupConfig {
    #[serde(rename = "Port")]
    pub port: usize,
    #[serde(rename = "VpcIdentifier")]
    pub vpc_identifier: String,
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheckConfig>,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Matcher {
    #[serde(rename = "HttpCode")]
    pub http_code: String,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheckConfig {
    #[serde(rename = "Matcher")]
    pub matcher: Option<Matcher>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<usize>,
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: Option<usize>,
    #[serde(rename = "UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: Option<usize>,
    #[serde(rename = "HealthyThresholdCount")]
    pub healthy_threshold_count: Option<usize>,

}


}
