

/// Associates a subnet with a route table. The subnet and route table must be in the same     VPC. This association causes traffic originating from the subnet to be routed according to     the routes in the route table. A route table can be associated with multiple subnets. To     create a route table, see AWS::EC2::RouteTable.
#[derive(Default, serde::Serialize)]
pub struct CfnSubnetRouteTableAssociation {


    /// 
    /// The ID of the subnet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The ID of the route table.
    /// 
    /// The physical ID changes when the route table ID is changed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,

}
