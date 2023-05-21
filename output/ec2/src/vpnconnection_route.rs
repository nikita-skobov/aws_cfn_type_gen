

/// Specifies a static route for a VPN connection between an existing virtual private     gateway and a VPN customer gateway. The static route allows traffic to be routed from the     virtual private gateway to the VPN customer gateway.
///
/// For more information, see AWS Site-to-Site VPN in the        AWS Site-to-Site VPN User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnVPNConnectionRoute {


    /// 
    /// The ID of the VPN connection.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: String,


    /// 
    /// The CIDR block associated with the local subnet of the customer network.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,

}
