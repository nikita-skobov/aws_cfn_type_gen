

/// Enables the specified attachment to propagate routes to the specified propagation route     table.
///
/// For more information about enabling transit gateway route propagation, see EnableTransitGatewayRouteTablePropagation in the Amazon EC2 API     Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayRouteTablePropagation {


    /// 
    /// The ID of the attachment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,


    /// 
    /// The ID of the propagation route table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: String,

}