

/// Specifies an Elastic IP (EIP) address and can, optionally, associate it with an Amazon       EC2 instance.
///
/// You can allocate an Elastic IP address from an address pool owned by AWS or from an address pool created from a public IPv4 address range that you have brought       to AWS for use with your AWS resources using bring your       own IP addresses (BYOIP). For more information, see Bring Your Own IP Addresses (BYOIP)       in the Amazon EC2 User Guide.
///
/// For more information, see Elastic IP Addresses       in the Amazon EC2 User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnEIP {


    /// 
    /// The ID of the instance.
    /// 
    /// ImportantUpdates to the InstanceId property may require some         interruptions. Updates on an EIP reassociates the address on its         associated resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,


    /// 
    /// The Elastic IP address you are accepting for transfer. You can only accept one transferred address. For more information on Elastic IP address transfers, see Transfer Elastic IP addresses in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransferAddress")]
    pub transfer_address: Option<String>,


    /// 
    /// A unique set of Availability Zones, Local Zones, or Wavelength Zones from which AWS    advertises IP addresses. Use this parameter to limit the IP address to this location. IP    addresses cannot move between network border groups.
    /// 
    /// Use DescribeAvailabilityZones to view the network border groups.
    /// 
    /// You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 Classic,    you receive an InvalidParameterCombination error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkBorderGroup")]
    pub network_border_group: Option<String>,


    /// 
    /// Any tags assigned to the Elastic IP address.
    /// 
    /// ImportantUpdates to the Tags property may require some         interruptions. Updates on an EIP reassociates the address on its         associated resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ID of an address pool that you own. Use this parameter to let Amazon EC2 select an       address from the address pool.
    /// 
    /// ImportantUpdates to the PublicIpv4Pool property may require some         interruptions. Updates on an EIP reassociates the address on its         associated resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PublicIpv4Pool")]
    pub public_ipv4_pool: Option<String>,


    /// 
    /// The network (vpc).
    /// 
    /// If you define an Elastic IP address and associate it with a VPC that is defined in the       same template, you must declare a dependency on the VPC-gateway attachment by using the       DependsOn         Attribute on this resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: standard | vpc
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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
