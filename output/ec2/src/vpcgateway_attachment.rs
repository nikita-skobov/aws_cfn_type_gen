/// Attaches an internet gateway, or a virtual private gateway to a VPC, enabling     connectivity between the internet and the VPC.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVPCGatewayAttachment {
    ///
    /// The ID of the internet gateway.
    ///
    /// You must specify either InternetGatewayId or VpnGatewayId, but     not both.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternetGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_gateway_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the VPC.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,

    ///
    /// The ID of the virtual private gateway.
    ///
    /// You must specify either InternetGatewayId or VpnGatewayId, but     not both.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_gateway_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnVPCGatewayAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPCGatewayAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
