/// Specifies a static route for a VPN connection between an existing virtual private     gateway and a VPN customer gateway. The static route allows traffic to be routed from the     virtual private gateway to the VPN customer gateway.
///
/// For more information, see AWS Site-to-Site VPN in the        AWS Site-to-Site VPN User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVPNConnectionRoute {
    ///
    /// The CIDR block associated with the local subnet of the customer network.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: cfn_resources::StrVal,

    ///
    /// The ID of the VPN connection.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnVPNConnectionRoute {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPNConnectionRoute"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
