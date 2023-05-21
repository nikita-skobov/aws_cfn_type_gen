

/// Specifies a network address translation (NAT) gateway in the specified subnet. You can     create either a public NAT gateway or a private NAT gateway. The default is a public NAT     gateway. If you create a public NAT gateway, you must specify an elastic IP address.
///
/// With a NAT gateway, instances in a private subnet can connect to the internet, other       AWS services, or an on-premises network using the IP address of the NAT     gateway.
///
/// If you add a default route (AWS::EC2::Route resource) that points to a NAT     gateway, specify the NAT gateway ID for the route's NatGatewayId     property.
///
/// For more information, see NAT Gateways in the       Amazon VPC User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnNatGateway {


    /// 
    /// The tags for the NAT gateway.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// [Public NAT gateway only] The allocation ID of the Elastic IP address that's associated with the NAT gateway.      This property is required for a public NAT gateway and cannot be specified with a private NAT gateway.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AllocationId")]
    pub allocation_id: Option<String>,


    /// 
    /// The ID of the subnet in which the NAT gateway is located.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxDrainDurationSeconds")]
    pub max_drain_duration_seconds: Option<i64>,


    /// 
    /// Secondary private IPv4 addresses. For more information about secondary addresses, see Create a NAT gateway in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// NoteSecondaryPrivateIpAddressCount and SecondaryPrivateIpAddresses cannot be set at the same time.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryPrivateIpAddresses")]
    pub secondary_private_ip_addresses: Option<Vec<String>>,


    /// 
    /// Indicates whether the NAT gateway supports public or private connectivity.      The default is public connectivity.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: private | public
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectivityType")]
    pub connectivity_type: Option<String>,


    /// 
    /// Secondary EIP allocation IDs. For more information, see Create a NAT gateway       in the Amazon VPC User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryAllocationIds")]
    pub secondary_allocation_ids: Option<Vec<String>>,


    /// 
    /// [Private NAT gateway only] The number of secondary private IPv4 addresses you want to assign to the NAT gateway. For more information about secondary addresses, see Create a NAT gateway in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// NoteSecondaryPrivateIpAddressCount and SecondaryPrivateIpAddresses cannot be set at the same time.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 31
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<i64>,


    /// 
    /// The private IPv4 address to assign to the NAT gateway. If you don't provide an address, a private IPv4 address will be automatically assigned.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,

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