/// Use the AWS::NetworkFirewall::Firewall to provide stateful, managed, network firewall and intrusion detection and prevention filtering for your VPCs in Amazon VPC.
///
/// The firewall defines the configuration settings for an AWS Network Firewall firewall. The settings include the firewall policy, the subnets in your VPC to use for the firewall endpoints, and any tags that are attached to the firewall AWS resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFirewall {
    ///
    /// A flag indicating whether it is possible to delete the firewall. A setting of TRUE indicates     that the firewall is protected against deletion. Use this setting to protect against     accidentally deleting a firewall that is in use. When you create a firewall, the operation initializes this flag to TRUE.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteProtection")]
    pub delete_protection: Option<bool>,

    ///
    /// A description of the firewall.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^.*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The descriptive name of the firewall. You can't change the name of a firewall after you create it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirewallName")]
    pub firewall_name: String,

    ///
    /// The Amazon Resource Name (ARN) of the firewall policy.
    ///
    /// The relationship of firewall to firewall policy is many to one. Each firewall requires     one firewall policy association, and you can use the same firewall policy for multiple     firewalls.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallPolicyArn")]
    pub firewall_policy_arn: String,

    ///
    /// A setting indicating whether the firewall is protected against a change to the firewall policy association.     Use this setting to protect against     accidentally modifying the firewall policy for a firewall that is in use. When you create a firewall, the operation initializes this setting to TRUE.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallPolicyChangeProtection")]
    pub firewall_policy_change_protection: Option<bool>,

    ///
    /// A setting indicating whether the firewall is protected against changes to the subnet associations.     Use this setting to protect against     accidentally modifying the subnet associations for a firewall that is in use. When you create a firewall, the operation initializes this setting to TRUE.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetChangeProtection")]
    pub subnet_change_protection: Option<bool>,

    ///
    /// The public subnets that Network Firewall is using for the firewall. Each subnet must belong     to a different Availability Zone.
    ///
    /// Required: Yes
    ///
    /// Type: List of SubnetMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetMappings")]
    pub subnet_mappings: Vec<SubnetMapping>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The unique identifier of the VPC where the firewall is in use. You can't change the VPC of a firewall after you create the firewall.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^vpc-[0-9a-f]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

impl cfn_resources::CfnResource for CfnFirewall {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkFirewall::Firewall"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.firewall_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'firewall_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.firewall_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'firewall_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.firewall_policy_arn;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'firewall_policy_arn'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.firewall_policy_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'firewall_policy_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.vpc_id;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'vpc_id'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.vpc_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'vpc_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The ID for a subnet that you want to associate with the firewall. AWS Network Firewall     creates an instance of the associated firewall in each subnet that you specify, to filter     traffic in the subnet's Availability Zone.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SubnetMapping {
    ///
    /// The subnet's IP address type. You can't change the IP address type after you create the subnet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DUALSTACK | IPV4 | IPV6
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPAddressType")]
    pub ipaddress_type: Option<SubnetMappingIPAddressTypeEnum>,

    ///
    /// The unique identifier for the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SubnetMappingIPAddressTypeEnum {
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

impl Default for SubnetMappingIPAddressTypeEnum {
    fn default() -> Self {
        SubnetMappingIPAddressTypeEnum::Dualstack
    }
}

impl cfn_resources::CfnResource for SubnetMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
