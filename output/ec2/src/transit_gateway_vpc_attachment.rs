/// Specifies a VPC attachment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayVpcAttachment {
    ///
    /// The IDs of one or more subnets to add. You can specify at most one subnet per Availability Zone.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddSubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_subnet_ids: Option<Vec<String>>,

    ///
    /// The VPC attachment options, in JSON or YAML.
    ///
    /// ApplianceModeSupport - Set to enable or disable. The default is disable.            DnsSupport - Set to enable or disable. The default is enable.            Ipv6Support - Set to enable or disable. The default is disable.
    ///
    /// Required: No
    ///
    /// Type: Options
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,

    ///
    /// The IDs of one or more subnets to remove.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_subnet_ids: Option<Vec<String>>,

    ///
    /// The IDs of the subnets.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

    ///
    /// The tags for the VPC attachment.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the transit gateway.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: cfn_resources::StrVal,

    ///
    /// The ID of the VPC.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnTransitGatewayVpcAttachmentid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayVpcAttachmentid;
impl CfnTransitGatewayVpcAttachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayVpcAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayVpcAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.options.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the VPC attachment options.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Options {
    ///
    /// Indicates whether appliance mode support is enabled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplianceModeSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appliance_mode_support: Option<OptionsApplianceModeSupportEnum>,

    ///
    /// Indicates whether DNS support is enabled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_support: Option<OptionsDnsSupportEnum>,

    ///
    /// Indicates whether IPv6 support is disabled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Support")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_support: Option<OptionsIpv6SupportEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsApplianceModeSupportEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsApplianceModeSupportEnum {
    fn default() -> Self {
        OptionsApplianceModeSupportEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsDnsSupportEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsDnsSupportEnum {
    fn default() -> Self {
        OptionsDnsSupportEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsIpv6SupportEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsIpv6SupportEnum {
    fn default() -> Self {
        OptionsIpv6SupportEnum::Disable
    }
}

impl cfn_resources::CfnResource for Options {
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
