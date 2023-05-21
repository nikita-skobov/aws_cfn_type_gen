

/// For DNS queries that originate in your VPCs, specifies which Resolver endpoint the queries pass through, 			one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResolverRule {


    /// 
    /// DNS queries for this domain name are forwarded to the IP addresses that are specified in TargetIps. If a query matches 			multiple Resolver rules (example.com and www.example.com), the query is routed using the Resolver rule that contains the most specific domain name 			(www.example.com).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// The name for the Resolver rule, which you specified when you created the Resolver rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Tags help organize and categorize your Resolver rules. Each tag consists of a key and an optional value, both of which you define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ID of the endpoint that the rule is associated with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResolverEndpointId")]
    pub resolver_endpoint_id: Option<String>,


    /// 
    /// When you want to forward DNS queries for specified domain name to resolvers on your network, specify FORWARD.
    /// 
    /// When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for 			a subdomain of that domain, specify SYSTEM.
    /// 
    /// For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify FORWARD 			for RuleType. To then have Resolver process queries for apex.example.com, you create a rule and specify 			SYSTEM for RuleType.
    /// 
    /// Currently, only Resolver can create rules that have a value of RECURSIVE for RuleType.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FORWARD | RECURSIVE | SYSTEM
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleType")]
    pub rule_type: String,


    /// 
    /// An array that contains the IP addresses and ports that an outbound endpoint forwards DNS queries to. Typically, 			these are the IP addresses of DNS resolvers on your network.
    /// 
    /// Required: No
    ///
    /// Type: List of TargetAddress
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIps")]
    pub target_ips: Option<Vec<TargetAddress>>,

}

impl cfn_resources::CfnResource for CfnResolverRule {
    fn type_string() -> &'static str {
        "AWS::Route53Resolver::ResolverRule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// In a 			CreateResolverRule 			request, an array of the IPs that you want to forward DNS queries to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetAddress {


    /// 
    /// The port at Ip that you want to forward DNS queries to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,


    /// 
    /// One IPv6 address that you want to forward DNS queries to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 7
    ///
    /// Maximum: 39
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6")]
    pub ipv6: Option<String>,


    /// 
    /// One IPv4 address that you want to forward DNS queries to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 7
    ///
    /// Maximum: 36
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ip")]
    pub ip: Option<String>,

}
