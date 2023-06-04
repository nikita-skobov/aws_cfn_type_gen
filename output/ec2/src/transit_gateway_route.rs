/// Specifies a static route for a transit gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayRoute {
    ///
    /// Indicates whether to drop traffic that matches this route.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Blackhole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackhole: Option<bool>,

    ///
    /// The CIDR block used for destination matches.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the attachment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayAttachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_attachment_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the transit gateway route table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnTransitGatewayRoute {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayRoute"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
