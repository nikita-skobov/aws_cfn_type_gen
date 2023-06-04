/// Creates a static route for the specified local gateway route table. You must specify one of the      following targets:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRoute {
    ///
    /// The CIDR block used for destination matches.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: cfn_resources::StrVal,

    ///
    /// The ID of the local gateway route table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalGatewayRouteTableId")]
    pub local_gateway_route_table_id: cfn_resources::StrVal,

    ///
    /// The ID of the virtual interface group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalGatewayVirtualInterfaceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_gateway_virtual_interface_group_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the network interface.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_state: CfnLocalGatewayRoutestate,

    #[serde(skip_serializing)]
    pub att_cfn_type: CfnLocalGatewayRoutecfntype,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRoutestate;
impl CfnLocalGatewayRoutestate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRoutecfntype;
impl CfnLocalGatewayRoutecfntype {
    pub fn att_name(&self) -> &'static str {
        r#"Type"#
    }
}

impl cfn_resources::CfnResource for CfnLocalGatewayRoute {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::LocalGatewayRoute"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
