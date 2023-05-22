/// Enables a virtual private gateway (VGW) to propagate routes to the specified route table     of a VPC.
///
/// If you reference a VPN gateway that is in the same template as your VPN gateway route     propagation, you must explicitly declare a dependency on the VPN gateway attachment. The       AWS::EC2::VPNGatewayRoutePropagation resource cannot use the VPN gateway     until it has successfully attached to the VPC. Add a DependsOn       Attribute in the AWS::EC2::VPNGatewayRoutePropagation resource to     explicitly declare a dependency on the VPN gateway attachment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPNGatewayRoutePropagation {
    ///
    /// The ID of the route table. The routing table must be associated with the same VPC that       the virtual private gateway is attached to.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Vec<String>,

    ///
    /// The ID of the virtual private gateway that is attached to a VPC. The virtual private       gateway must be attached to the same VPC that the routing tables are associated with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnVPNGatewayRoutePropagationid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPNGatewayRoutePropagationid;
impl CfnVPNGatewayRoutePropagationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnVPNGatewayRoutePropagation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPNGatewayRoutePropagation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
