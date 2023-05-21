

/// Associates the specified attachment with the specified transit gateway route table. You     can associate one route table with an attachment.
///
/// Before you can update the route table associated with an attachment, you must     disassociate the transit gateway route table that is currently associated with the     attachment. First update the stack to remove the associated transit gateway route table,     and then update the stack with the ID of the new transit gateway route table to     associate.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayRouteTableAssociation {


    /// 
    /// The ID of the route table for the transit gateway.
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
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,

}
