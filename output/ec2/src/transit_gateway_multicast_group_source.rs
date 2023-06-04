/// Registers sources (network interfaces) with the specified transit gateway multicast     domain.
///
/// A multicast source is a network interface attached to a supported instance that sends     multicast traffic. For information about supported instances, see Multicast Considerations in Amazon VPC Transit       Gateways.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayMulticastGroupSource {
    ///
    /// The IP address assigned to the transit gateway multicast group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupIpAddress")]
    pub group_ip_address: cfn_resources::StrVal,

    ///
    /// The group sources' network interface IDs to register with the transit gateway multicast group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: cfn_resources::StrVal,

    ///
    /// The ID of the transit gateway multicast domain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_member_type: CfnTransitGatewayMulticastGroupSourcemembertype,

    #[serde(skip_serializing)]
    pub att_resource_id: CfnTransitGatewayMulticastGroupSourceresourceid,

    #[serde(skip_serializing)]
    pub att_resource_type: CfnTransitGatewayMulticastGroupSourceresourcetype,

    #[serde(skip_serializing)]
    pub att_source_type: CfnTransitGatewayMulticastGroupSourcesourcetype,

    #[serde(skip_serializing)]
    pub att_subnet_id: CfnTransitGatewayMulticastGroupSourcesubnetid,

    #[serde(skip_serializing)]
    pub att_transit_gateway_attachment_id:
        CfnTransitGatewayMulticastGroupSourcetransitgatewayattachmentid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourcemembertype;
impl CfnTransitGatewayMulticastGroupSourcemembertype {
    pub fn att_name(&self) -> &'static str {
        r#"MemberType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourceresourceid;
impl CfnTransitGatewayMulticastGroupSourceresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourceresourcetype;
impl CfnTransitGatewayMulticastGroupSourceresourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourcesourcetype;
impl CfnTransitGatewayMulticastGroupSourcesourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"SourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourcesubnetid;
impl CfnTransitGatewayMulticastGroupSourcesubnetid {
    pub fn att_name(&self) -> &'static str {
        r#"SubnetId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupSourcetransitgatewayattachmentid;
impl CfnTransitGatewayMulticastGroupSourcetransitgatewayattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayAttachmentId"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayMulticastGroupSource {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayMulticastGroupSource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
