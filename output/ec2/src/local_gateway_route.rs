

/// Creates a static route for the specified local gateway route table. You must specify one of the      following targets:
#[derive(Default, serde::Serialize)]
pub struct CfnLocalGatewayRoute {


    /// 
    /// The ID of the virtual interface group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalGatewayVirtualInterfaceGroupId")]
    pub local_gateway_virtual_interface_group_id: Option<String>,


    /// 
    /// The ID of the network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// The CIDR block used for destination matches.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,


    /// 
    /// The ID of the local gateway route table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalGatewayRouteTableId")]
    pub local_gateway_route_table_id: String,

}
