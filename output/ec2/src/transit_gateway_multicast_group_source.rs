

/// Registers sources (network interfaces) with the specified transit gateway multicast     domain.
///
/// A multicast source is a network interface attached to a supported instance that sends     multicast traffic. For information about supported instances, see Multicast Considerations in Amazon VPC Transit       Gateways.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayMulticastGroupSource {


    /// 
    /// The group sources' network interface IDs to register with the transit gateway multicast group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,


    /// 
    /// The IP address assigned to the transit gateway multicast group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupIpAddress")]
    pub group_ip_address: String,


    /// 
    /// The ID of the transit gateway multicast domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,

}
