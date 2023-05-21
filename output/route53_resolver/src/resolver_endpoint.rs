

/// Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResolverEndpoint {


    /// 
    /// The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward 			DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC.
    /// 
    /// Required: Yes
    ///
    /// Type: List of IpAddressRequest
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<IpAddressRequest>,


    /// 
    /// The ID of one or more security groups that control access to this VPC. The security group must include one or more inbound rules 			(for inbound endpoints) or outbound rules (for outbound endpoints). Inbound and outbound rules must allow TCP and UDP access. 			For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,


    /// 
    /// The Resolver endpoint IP address type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DUALSTACK | IPV4 | IPV6
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResolverEndpointType")]
    pub resolver_endpoint_type: Option<ResolverEndpointResolverEndpointTypeEnum>,


    /// 
    /// A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.
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
    /// Route 53 Resolver doesn't support updating tags through CloudFormation.
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


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreferredInstanceType")]
    pub preferred_instance_type: Option<String>,


    /// 
    /// Indicates whether the Resolver endpoint allows inbound or outbound DNS queries:
    /// 
    /// INBOUND: allows DNS queries to your VPC from your network                        OUTBOUND: allows DNS queries from your VPC to your network
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: INBOUND | OUTBOUND
    ///
    /// Update requires: Replacement
    #[serde(rename = "Direction")]
    pub direction: ResolverEndpointDirectionEnum,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ResolverEndpointDirectionEnum {

    /// INBOUND
    #[serde(rename = "INBOUND")]
    Inbound,

    /// OUTBOUND
    #[serde(rename = "OUTBOUND")]
    Outbound,

}

impl Default for ResolverEndpointDirectionEnum {
    fn default() -> Self {
        ResolverEndpointDirectionEnum::Inbound
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ResolverEndpointResolverEndpointTypeEnum {

    /// DUALSTACK
    #[serde(rename = "DUALSTACK")]
    Dualstack,

    /// IPV4
    #[serde(rename = "IPV4")]
    Ipv4,

    /// IPV6
    #[serde(rename = "IPV6")]
    Ipv6,

}

impl Default for ResolverEndpointResolverEndpointTypeEnum {
    fn default() -> Self {
        ResolverEndpointResolverEndpointTypeEnum::Dualstack
    }
}


impl cfn_resources::CfnResource for CfnResolverEndpoint {
    fn type_string() -> &'static str {
        "AWS::Route53Resolver::ResolverEndpoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// In a 			CreateResolverEndpoint 			request, the IP address that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). 			IpAddressRequest also includes the ID of the subnet that contains the IP address.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IpAddressRequest {


    /// 
    /// The ID of the subnet that contains the IP address.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The IPv6 address that you want to use for DNS queries.
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
    /// The IPv4 address that you want to use for DNS queries.
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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


