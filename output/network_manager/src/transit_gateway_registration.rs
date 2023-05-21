

/// Registers a transit gateway in your global network. Not all Regions support transit       gateways for global networks. For a list of the supported Regions, see Region Availability in the         AWS Transit Gateways for Global         Networks User Guide. The transit gateway can be in any of the supported       AWS Regions, but it must be owned by the same AWS account that owns the global       network. You cannot register a transit gateway in more than one global network.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayRegistration {


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

}



impl cfn_resources::CfnResource for CfnTransitGatewayRegistration {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::TransitGatewayRegistration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.global_network_id;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'global_network_id'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.global_network_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'global_network_id'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.transit_gateway_arn;

        if the_val.len() > 500 as _ {
            return Err(format!("Max validation failed on field 'transit_gateway_arn'. {} is greater than 500", the_val.len()));
        }

        
        let the_val = &self.transit_gateway_arn;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'transit_gateway_arn'. {} is less than 0", the_val.len()));
        }

        
        Ok(())
    }
}