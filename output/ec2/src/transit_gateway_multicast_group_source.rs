/// Registers sources (network interfaces) with the specified transit gateway multicast     domain.
///
/// A multicast source is a network interface attached to a supported instance that sends     multicast traffic. For information about supported instances, see Multicast Considerations in Amazon VPC Transit       Gateways.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
