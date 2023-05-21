

/// Specifies a route in a route table.
///
/// You must specify either DestinationCidrBlock or     DestinationIpv6CidrBlock, plus the ID of one of the target     resources.
///
/// If you create a route that references a transit gateway in the same template where you     create the transit gateway, you must declare a dependency on the transit gateway     attachment. The route table cannot use the transit gateway until it has successfully     attached to the VPC. Add a DependsOn       Attribute in the AWS::EC2::Route resource to explicitly declare a     dependency on the AWS::EC2::TransitGatewayAttachment resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRoute {


    /// 
    /// The ID of the carrier gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CarrierGatewayId")]
    pub carrier_gateway_id: Option<String>,


    /// 
    /// The IPv6 CIDR block used for the destination match.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationIpv6CidrBlock")]
    pub destination_ipv6_cidr_block: Option<String>,


    /// 
    /// The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,


    /// 
    /// The ID of a VPC peering connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: Option<String>,


    /// 
    /// The ID of the local gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalGatewayId")]
    pub local_gateway_id: Option<String>,


    /// 
    /// The IPv4 CIDR block used for the destination match.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: Option<String>,


    /// 
    /// The ID of an internet gateway or virtual private gateway attached to your VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayId")]
    pub gateway_id: Option<String>,


    /// 
    /// The ID of a NAT gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: Option<String>,


    /// 
    /// The ID of a NAT instance in your VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,


    /// 
    /// The ID of the egress-only internet gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressOnlyInternetGatewayId")]
    pub egress_only_internet_gateway_id: Option<String>,


    /// 
    /// The ID of a transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: Option<String>,


    /// 
    /// The ID of the route table. The routing table must be associated with the same VPC that       the virtual private gateway is attached to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,


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

}

impl cfn_resources::CfnResource for CfnRoute {
    fn type_string() -> &'static str {
        "AWS::EC2::Route"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
