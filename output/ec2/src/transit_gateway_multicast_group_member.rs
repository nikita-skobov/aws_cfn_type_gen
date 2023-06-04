/// Registers members (network interfaces) with the transit gateway multicast group. A     member is a network interface associated with a supported EC2 instance that receives     multicast traffic. For information about supported instances, see Multicast Consideration in Amazon VPC Transit       Gateways.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayMulticastGroupMember {
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
    /// The group members' network interface IDs to register with the transit gateway multicast group.
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
    pub att_member_type: CfnTransitGatewayMulticastGroupMembermembertype,

    #[serde(skip_serializing)]
    pub att_resource_id: CfnTransitGatewayMulticastGroupMemberresourceid,

    #[serde(skip_serializing)]
    pub att_resource_type: CfnTransitGatewayMulticastGroupMemberresourcetype,

    #[serde(skip_serializing)]
    pub att_source_type: CfnTransitGatewayMulticastGroupMembersourcetype,

    #[serde(skip_serializing)]
    pub att_subnet_id: CfnTransitGatewayMulticastGroupMembersubnetid,

    #[serde(skip_serializing)]
    pub att_transit_gateway_attachment_id:
        CfnTransitGatewayMulticastGroupMembertransitgatewayattachmentid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMembermembertype;
impl CfnTransitGatewayMulticastGroupMembermembertype {
    pub fn att_name(&self) -> &'static str {
        r#"MemberType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMemberresourceid;
impl CfnTransitGatewayMulticastGroupMemberresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMemberresourcetype;
impl CfnTransitGatewayMulticastGroupMemberresourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMembersourcetype;
impl CfnTransitGatewayMulticastGroupMembersourcetype {
    pub fn att_name(&self) -> &'static str {
        r#"SourceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMembersubnetid;
impl CfnTransitGatewayMulticastGroupMembersubnetid {
    pub fn att_name(&self) -> &'static str {
        r#"SubnetId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayMulticastGroupMembertransitgatewayattachmentid;
impl CfnTransitGatewayMulticastGroupMembertransitgatewayattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayAttachmentId"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayMulticastGroupMember {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayMulticastGroupMember"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
