

/// Associates the specified subnets and transit gateway attachments with the specified     transit gateway multicast domain.
///
/// The transit gateway attachment must be in the available state before you can add a     resource.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayMulticastDomainAssociation {


    /// 
    /// The ID of the transit gateway attachment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,


    /// 
    /// The IDs of the subnets to associate with the transit gateway multicast domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The ID of the transit gateway multicast domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,

}