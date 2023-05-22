/// Associates a subnet with a route table. The subnet and route table must be in the same     VPC. This association causes traffic originating from the subnet to be routed according to     the routes in the route table. A route table can be associated with multiple subnets. To     create a route table, see AWS::EC2::RouteTable.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubnetRouteTableAssociation {
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
    pub route_table_id: cfn_resources::StrVal,

    ///
    /// The ID of the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnSubnetRouteTableAssociationid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubnetRouteTableAssociationid;
impl CfnSubnetRouteTableAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnSubnetRouteTableAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::SubnetRouteTableAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
