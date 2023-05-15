
pub mod cfn_firewall_domain_list {

#[derive(serde::Serialize, Default)]
pub struct CfnFirewallDomainList {
    /// An inline list of domains to use for this domain list.
    #[serde(rename = "Domains")]
    pub domains: Option<Domains>,
    /// Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// S3 URL to import domains from.
    #[serde(rename = "DomainFileUrl")]
    pub domain_file_url: Option<String>,

}

pub type Domains = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_firewall_rule_group {

#[derive(serde::Serialize, Default)]
pub struct CfnFirewallRuleGroup {
    /// FirewallRuleGroupName
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// FirewallRules
    #[serde(rename = "FirewallRules")]
    pub firewall_rules: Option<Vec<FirewallRule>>,
    /// Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct FirewallRule {
    #[serde(rename = "BlockOverrideDnsType")]
    pub block_override_dns_type: Option<String>,
    #[serde(rename = "BlockResponse")]
    pub block_response: Option<String>,
    #[serde(rename = "FirewallDomainListId")]
    pub firewall_domain_list_id: String,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "BlockOverrideTtl")]
    pub block_override_ttl: Option<usize>,
    #[serde(rename = "Priority")]
    pub priority: usize,
    #[serde(rename = "BlockOverrideDomain")]
    pub block_override_domain: Option<String>,

}


}

pub mod cfn_firewall_rule_group_association {

#[derive(serde::Serialize, Default)]
pub struct CfnFirewallRuleGroupAssociation {
    /// Priority
    #[serde(rename = "Priority")]
    pub priority: usize,
    /// MutationProtectionStatus
    #[serde(rename = "MutationProtection")]
    pub mutation_protection: Option<String>,
    /// VpcId
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// FirewallRuleGroupId
    #[serde(rename = "FirewallRuleGroupId")]
    pub firewall_rule_group_id: String,
    /// Tags
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_resolver_config {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverConfig {
    /// Represents the desired status of AutodefinedReverse. The only supported value on creation is DISABLE. Deletion of this resource will return AutodefinedReverse to its default value (ENABLED).
    #[serde(rename = "AutodefinedReverseFlag")]
    pub autodefined_reverse_flag: String,
    /// ResourceId
    #[serde(rename = "ResourceId")]
    pub resource_id: String,

}



}

pub mod cfn_resolver_dnssecconfig {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverDNSSECConfig {
    /// ResourceId
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

}



}

pub mod cfn_resolver_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverEndpoint {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of IpAddressRequest
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<IpAddressRequest>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredInstanceType")]
    pub preferred_instance_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResolverEndpointType")]
    pub resolver_endpoint_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Direction")]
    pub direction: String,
    /// No documentation provided by AWS
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct IpAddressRequest {
    #[serde(rename = "Ipv6")]
    pub ipv6: Option<String>,
    #[serde(rename = "Ip")]
    pub ip: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_resolver_query_logging_config {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverQueryLoggingConfig {
    /// ResolverQueryLogConfigName
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// destination arn
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}



}

pub mod cfn_resolver_query_logging_config_association {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverQueryLoggingConfigAssociation {
    /// ResourceId
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    /// ResolverQueryLogConfigId
    #[serde(rename = "ResolverQueryLogConfigId")]
    pub resolver_query_log_config_id: Option<String>,

}



}

pub mod cfn_resolver_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverRule {
    /// When you want to forward DNS queries for specified domain name to resolvers on your network, specify FORWARD. When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for a subdomain of that domain, specify SYSTEM.
    #[serde(rename = "RuleType")]
    pub rule_type: String,
    /// DNS queries for this domain name are forwarded to the IP addresses that are specified in TargetIps
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// An array that contains the IP addresses and ports that an outbound endpoint forwards DNS queries to. Typically, these are the IP addresses of DNS resolvers on your network. Specify IPv4 addresses. IPv6 is not supported.
    #[serde(rename = "TargetIps")]
    pub target_ips: Option<Vec<TargetAddress>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name for the Resolver rule
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The ID of the endpoint that the rule is associated with.
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct TargetAddress {
    #[serde(rename = "Ip")]
    pub ip: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "Ipv6")]
    pub ipv6: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_resolver_rule_association {

#[derive(serde::Serialize, Default)]
pub struct CfnResolverRuleAssociation {
    /// The ID of the Resolver rule that you associated with the VPC that is specified by VPCId.
    #[serde(rename = "ResolverRuleId")]
    pub resolver_rule_id: String,
    /// The name of an association between a Resolver rule and a VPC.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The ID of the VPC that you associated the Resolver rule with.
    #[serde(rename = "VPCId")]
    pub vpcid: String,

}



}
