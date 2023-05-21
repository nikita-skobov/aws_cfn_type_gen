

/// Specifies a target network to associate with a Client VPN endpoint. A target network is     a subnet in a VPC. You can associate multiple subnets from the same VPC with a Client VPN     endpoint. You can associate only one subnet in each Availability Zone. We recommend that     you associate at least two subnets to provide Availability Zone redundancy.
#[derive(Default, serde::Serialize)]
pub struct CfnClientVpnTargetNetworkAssociation {


    /// 
    /// The ID of the Client VPN endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,


    /// 
    /// The ID of the subnet to associate with the Client VPN endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,

}