/// Creates a multicast domain using the specified transit gateway.
///
/// The transit gateway must be in the available state before you create a domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayMulticastDomain {
    ///
    /// The options for the transit gateway multicast domain.
    ///
    /// AutoAcceptSharedAssociations (enable | disable)            Igmpv2Support (enable | disable)            StaticSourcesSupport (enable | disable)
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
    /// The tags for the transit gateway multicast domain.
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

    #[serde(skip_serializing)]
    pub att_creation_time: CfnTransitGatewayMulticastDomaincreationtime,

    #[serde(skip_serializing)]
    pub att_state: CfnTransitGatewayMulticastDomainstate,

    #[serde(skip_serializing)]
    pub att_transit_gateway_multicast_domain_arn:
        CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainarn,

    #[serde(skip_serializing)]
    pub att_transit_gateway_multicast_domain_id:
        CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastDomaincreationtime;
impl CfnTransitGatewayMulticastDomaincreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastDomainstate;
impl CfnTransitGatewayMulticastDomainstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainarn;
impl CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainarn {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayMulticastDomainArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainid;
impl CfnTransitGatewayMulticastDomaintransitgatewaymulticastdomainid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayMulticastDomainId"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayMulticastDomain {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayMulticastDomain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.options.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The options for the transit gateway multicast domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Options {
    ///
    /// Indicates whether to automatically accept cross-account subnet associations that are associated with the transit gateway multicast domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAcceptSharedAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_shared_associations: Option<OptionsAutoAcceptSharedAssociationsEnum>,

    ///
    /// Specify whether to enable Internet Group Management Protocol (IGMP) version 2 for the transit gateway multicast domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Igmpv2Support")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igmpv2_support: Option<OptionsIgmpv2SupportEnum>,

    ///
    /// Specify whether to enable support for statically configuring multicast group sources for a domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticSourcesSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_sources_support: Option<OptionsStaticSourcesSupportEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsAutoAcceptSharedAssociationsEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsAutoAcceptSharedAssociationsEnum {
    fn default() -> Self {
        OptionsAutoAcceptSharedAssociationsEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsIgmpv2SupportEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsIgmpv2SupportEnum {
    fn default() -> Self {
        OptionsIgmpv2SupportEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum OptionsStaticSourcesSupportEnum {
    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,
}

impl Default for OptionsStaticSourcesSupportEnum {
    fn default() -> Self {
        OptionsStaticSourcesSupportEnum::Disable
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
