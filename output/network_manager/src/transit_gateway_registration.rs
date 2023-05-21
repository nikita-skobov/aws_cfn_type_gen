

/// Registers a transit gateway in your global network. Not all Regions support transit       gateways for global networks. For a list of the supported Regions, see Region Availability in the         AWS Transit Gateways for Global         Networks User Guide. The transit gateway can be in any of the supported       AWS Regions, but it must be owned by the same AWS account that owns the global       network. You cannot register a transit gateway in more than one global network.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayRegistration {


    /// 
    /// The Amazon Resource Name (ARN) of the transit gateway.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: String,


    /// 
    /// The ID of the global network.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,

}
