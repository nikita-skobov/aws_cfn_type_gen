

/// Specifies a subnet for the specified VPC.
///
/// For an IPv4 only subnet, specify an IPv4 CIDR block. If the VPC has an IPv6 CIDR block,      you can create an IPv6 only subnet or a dual stack subnet instead. For an IPv6 only subnet,      specify an IPv6 CIDR block. For a dual stack subnet, specify both an IPv4 CIDR block and      an IPv6 CIDR block.
///
/// For more information, see Subnets for your VPC in the Amazon VPC User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubnet {


    /// 
    /// Indicates whether a network interface created in this subnet receives an IPv6 address.     The default value is false.
    /// 
    /// If you specify AssignIpv6AddressOnCreation, you must also specify     Ipv6CidrBlock.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssignIpv6AddressOnCreation")]
    pub assign_ipv6_address_on_creation: Option<bool>,


    /// 
    /// The Availability Zone of the subnet.
    /// 
    /// If you update this property, you must also update the CidrBlock     property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The AZ ID of the subnet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZoneId")]
    pub availability_zone_id: Option<String>,


    /// 
    /// The IPv4 CIDR block assigned to the subnet.
    /// 
    /// If you update this property, we create a new subnet, and then delete the existing     one.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,


    /// 
    /// Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet      should return synthetic IPv6 addresses for IPv4-only destinations. For more information, see DNS64 and NAT64 in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDns64")]
    pub enable_dns64: Option<bool>,


    /// 
    /// The IPv6 CIDR block.
    /// 
    /// If you specify AssignIpv6AddressOnCreation, you must also specify     Ipv6CidrBlock.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: Option<String>,


    /// 
    /// Indicates whether this is an IPv6 only subnet. For more information, see Subnet basics in the Amazon Virtual Private Cloud User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6Native")]
    pub ipv6_native: Option<bool>,


    /// 
    /// Indicates whether instances launched in this subnet receive a public IPv4 address. The     default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapPublicIpOnLaunch")]
    pub map_public_ip_on_launch: Option<bool>,


    /// 
    /// The Amazon Resource Name (ARN) of the Outpost.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,


    /// 
    /// The hostname type for EC2 instances launched into this subnet and how DNS A and AAAA record queries to the instances should be handled. For more information, see Amazon EC2 instance hostname types in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Available options:
    /// 
    /// EnableResourceNameDnsAAAARecord (true | false)EnableResourceNameDnsARecord (true | false)HostnameType (ip-name | resource-name)
    /// 
    /// Required: No
    ///
    /// Type: PrivateDnsNameOptionsOnLaunch
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateDnsNameOptionsOnLaunch")]
    pub private_dns_name_options_on_launch: Option<PrivateDnsNameOptionsOnLaunch>,


    /// 
    /// Any tags assigned to the subnet.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ID of the VPC the subnet is in.
    /// 
    /// If you update this property, you must also update the CidrBlock     property.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}



impl cfn_resources::CfnResource for CfnSubnet {
    fn type_string() -> &'static str {
        "AWS::EC2::Subnet"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.private_dns_name_options_on_launch.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the options for instance hostnames.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrivateDnsNameOptionsOnLaunch {


    /// 
    /// Indicates whether to respond to DNS queries for instance hostname with DNS AAAA       records.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,


    /// 
    /// Indicates whether to respond to DNS queries for instance hostnames with DNS A       records.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableResourceNameDnsARecord")]
    pub enable_resource_name_dns_arecord: Option<bool>,


    /// 
    /// The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name       must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name       must be based on the instance ID. For dual-stack subnets, you can specify whether DNS       names use the instance IPv4 address or the instance ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ip-name | resource-name
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostnameType")]
    pub hostname_type: Option<PrivateDnsNameOptionsOnLaunchHostnameTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PrivateDnsNameOptionsOnLaunchHostnameTypeEnum {

    /// ip-name
    #[serde(rename = "ip-name")]
    Ipname,

    /// resource-name
    #[serde(rename = "resource-name")]
    Resourcename,

}

impl Default for PrivateDnsNameOptionsOnLaunchHostnameTypeEnum {
    fn default() -> Self {
        PrivateDnsNameOptionsOnLaunchHostnameTypeEnum::Ipname
    }
}


impl cfn_resources::CfnResource for PrivateDnsNameOptionsOnLaunch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}