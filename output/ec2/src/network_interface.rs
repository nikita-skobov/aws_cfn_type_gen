/// Describes a network interface in an Amazon EC2 instance for AWS CloudFormation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNetworkInterface {
    ///
    /// A description for the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The security group IDs associated with this network interface.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_set: Option<Vec<String>>,

    ///
    /// The type of network interface. The default is interface. The supported values       are efa and trunk.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: branch | efa | trunk
    ///
    /// Update requires: Replacement
    #[serde(rename = "InterfaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<NetworkInterfaceInterfaceTypeEnum>,

    ///
    /// The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically       selects the IPv6 addresses from the subnet range. To specify specific IPv6 addresses, use       the Ipv6Addresses property and don't specify this property.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6AddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<i64>,

    ///
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet to       associate with the network interface. If you're specifying a number of IPv6 addresses, use       the Ipv6AddressCount property and don't specify this property.
    ///
    /// Required: No
    ///
    /// Type: List of InstanceIpv6Address
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,

    ///
    /// Assigns a single private IP address to the network interface, which is used as the       primary private IP address. If you want to specify multiple private IP address, use the       PrivateIpAddresses property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<cfn_resources::StrVal>,

    ///
    /// Assigns private IP addresses to the network interface. You can specify a primary private       IP address by setting the value of the Primary property to true       in the PrivateIpAddressSpecification property. If you want EC2 to       automatically assign private IP addresses, use the       SecondaryPrivateIpAddressCount property and do not specify this       property.
    ///
    /// Required: No
    ///
    /// Type: List of PrivateIpAddressSpecification
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification>>,

    ///
    /// The number of secondary private IPv4 addresses to assign to a network interface. When       you specify a number of secondary IPv4 addresses, Amazon EC2 selects these IP addresses       within the subnet's IPv4 CIDR range. You can't specify this option and specify more than       one private IP address using privateIpAddresses.
    ///
    /// You can't specify a count of private IPv4 addresses if you've specified one of the following:       specific private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4 prefixes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_private_ip_address_count: Option<i64>,

    ///
    /// Enable or disable source/destination checks, which ensure that the instance       is either the source or the destination of any traffic that it receives.       If the value is true, source/destination checks are enabled;       otherwise, they are disabled. The default value is true.       You must disable source/destination checks if the instance runs services       such as network address translation, routing, or firewalls.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceDestCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<bool>,

    ///
    /// The ID of the subnet to associate with the network interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,

    ///
    /// An arbitrary set of tags (key-value pairs) for this network interface.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkInterfaceInterfaceTypeEnum {
    /// branch
    #[serde(rename = "branch")]
    Branch,

    /// efa
    #[serde(rename = "efa")]
    Efa,

    /// trunk
    #[serde(rename = "trunk")]
    Trunk,
}

impl Default for NetworkInterfaceInterfaceTypeEnum {
    fn default() -> Self {
        NetworkInterfaceInterfaceTypeEnum::Branch
    }
}

impl cfn_resources::CfnResource for CfnNetworkInterface {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkInterface"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes the IPv6 addresses to associate with the network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceIpv6Address {
    ///
    /// An IPv6 address to associate with the network interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for InstanceIpv6Address {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes a secondary private IPv4 address for a network interface.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateIpAddressSpecification {
    ///
    /// Sets the private IP address as the primary private address. You can set only one primary       private IP address. If you don't specify a primary private IP address, Amazon EC2       automatically assigns a primary private IP address.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Primary")]
    pub primary: bool,

    ///
    /// The private IP address of the network interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PrivateIpAddressSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
