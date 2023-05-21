

/// Specifies a static route for a transit gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayRoute {


    /// 
    /// The ID of the transit gateway route table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: String,


    /// 
    /// The ID of the attachment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: Option<String>,


    /// 
    /// The CIDR block used for destination matches.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: Option<String>,


    /// 
    /// Indicates whether to drop traffic that matches this route.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Blackhole")]
    pub blackhole: Option<bool>,

}

impl cfn_resources::CfnResource for CfnTransitGatewayRoute {
    fn type_string() -> &'static str {
        "AWS::EC2::TransitGatewayRoute"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
