/// The AWS::GlobalAccelerator::Accelerator resource is a Global Accelerator resource type that contains information about 	   		how you create an accelerator. An accelerator includes one or more listeners that process inbound connections and direct traffic 	   		to one or more endpoint groups, each of which includes endpoints, such as Application Load Balancers, Network Load Balancers, 	   		and Amazon EC2 instances.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAccelerator {
    ///
    /// Indicates whether the accelerator is enabled. The value is true or false. The default value is true.
    ///
    /// If the value is set to true, the accelerator cannot be deleted. If set to false, accelerator can be deleted.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    ///
    /// The IP address type that an accelerator supports. For a standard accelerator, the value can be IPV4 or DUAL_STACK.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DUAL_STACK | IPV4
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<AcceleratorIpAddressTypeEnum>,

    ///
    /// Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose IP addresses 			from your own pool to use for the accelerator's static IP addresses when you create an accelerator. You can 			specify one or two addresses, separated by a comma. Do not include the /32 suffix.
    ///
    /// Only one IP address from each of your IP address ranges can be used for each accelerator. If you specify only 			one IP address from your IP address range, Global Accelerator assigns a second static IP address for the 			accelerator from the AWS IP address pool.
    ///
    /// Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new 			accelerator with the new addresses.
    ///
    /// For more information, see Bring Your Own 			IP Addresses (BYOIP) in the AWS Global Accelerator Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,

    ///
    /// The name of the accelerator. The name must contain only alphanumeric characters or 			hyphens (-), and must not begin or end with a hyphen.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Create tags for an accelerator.
    ///
    /// For more information, see Tagging 				 in the AWS Global Accelerator Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_accelerator_arn: CfnAcceleratoracceleratorarn,

    #[serde(skip_serializing)]
    pub att_dns_name: CfnAcceleratordnsname,

    #[serde(skip_serializing)]
    pub att_dual_stack_dns_name: CfnAcceleratordualstackdnsname,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AcceleratorIpAddressTypeEnum {
    /// DUAL_STACK
    #[serde(rename = "DUAL_STACK")]
    Dualstack,

    /// IPV4
    #[serde(rename = "IPV4")]
    Ipv4,
}

impl Default for AcceleratorIpAddressTypeEnum {
    fn default() -> Self {
        AcceleratorIpAddressTypeEnum::Dualstack
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAcceleratoracceleratorarn;
impl CfnAcceleratoracceleratorarn {
    pub fn att_name(&self) -> &'static str {
        r#"AcceleratorArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAcceleratordnsname;
impl CfnAcceleratordnsname {
    pub fn att_name(&self) -> &'static str {
        r#"DnsName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAcceleratordualstackdnsname;
impl CfnAcceleratordualstackdnsname {
    pub fn att_name(&self) -> &'static str {
        r#"DualStackDnsName"#
    }
}

impl cfn_resources::CfnResource for CfnAccelerator {
    fn type_string(&self) -> &'static str {
        "AWS::GlobalAccelerator::Accelerator"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    s.len()
                ));
            }
        }

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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
