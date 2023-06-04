/// Specifies a route in a route table.
///
/// You must specify either DestinationCidrBlock or     DestinationIpv6CidrBlock, plus the ID of one of the target     resources.
///
/// If you create a route that references a transit gateway in the same template where you     create the transit gateway, you must declare a dependency on the transit gateway     attachment. The route table cannot use the transit gateway until it has successfully     attached to the VPC. Add a DependsOn       Attribute in the AWS::EC2::Route resource to explicitly declare a     dependency on the AWS::EC2::TransitGatewayAttachment resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub carrier_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The IPv4 CIDR block used for the destination match.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub destination_cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// The IPv6 CIDR block used for the destination match.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationIpv6CidrBlock")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub destination_ipv6_cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the egress-only internet gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressOnlyInternetGatewayId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub egress_only_internet_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of an internet gateway or virtual private gateway attached to your VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GatewayId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a NAT instance in your VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the local gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalGatewayId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub local_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a NAT gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGatewayId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub nat_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub network_interface_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the route table. The routing table must be associated with the same VPC that       the virtual private gateway is attached to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RouteTableId")]
    pub route_table_id: cfn_resources::StrVal,

    ///
    /// The ID of a transit gateway.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub transit_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcEndpointId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_endpoint_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of a VPC peering connection.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_peering_connection_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnRoute {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::Route"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
