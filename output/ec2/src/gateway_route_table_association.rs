

/// Associates a virtual private gateway or internet gateway with a route table. The gateway     and route table must be in the same VPC. This association causes the incoming traffic to     the gateway to be routed according to the routes in the route table.
#[derive(Default, serde::Serialize)]
pub struct CfnGatewayRouteTableAssociation {


    /// 
    /// The ID of the gateway.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GatewayId")]
    pub gateway_id: String,


    /// 
    /// The ID of the route table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,

}
