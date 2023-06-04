/// Associates a virtual private gateway or internet gateway with a route table. The gateway     and route table must be in the same VPC. This association causes the incoming traffic to     the gateway to be routed according to the routes in the route table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub gateway_id: cfn_resources::StrVal,

    ///
    /// The ID of the route table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableId")]
    pub route_table_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_association_id: CfnGatewayRouteTableAssociationassociationid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGatewayRouteTableAssociationassociationid;
impl CfnGatewayRouteTableAssociationassociationid {
    pub fn att_name(&self) -> &'static str {
        r#"AssociationId"#
    }
}

impl cfn_resources::CfnResource for CfnGatewayRouteTableAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::GatewayRouteTableAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
