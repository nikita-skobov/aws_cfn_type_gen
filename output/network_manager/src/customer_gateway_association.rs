

/// Specifies an association between a customer gateway, a device, and optionally, a link.       If you specify a link, it must be associated with the specified device. The customer       gateway must be connected to a VPN attachment on a transit gateway that's registered in       your global network.
///
/// You cannot associate a customer gateway with more than one device and link.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCustomerGatewayAssociation {


    /// 
    /// The Amazon Resource Name (ARN) of the customer gateway.
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
    #[serde(rename = "CustomerGatewayArn")]
    pub customer_gateway_arn: String,


    /// 
    /// The ID of the device.
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
    #[serde(rename = "DeviceId")]
    pub device_id: String,


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
    /// The ID of the link.
    /// 
    /// Required: No
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
    #[serde(rename = "LinkId")]
    pub link_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnCustomerGatewayAssociation {
    fn type_string() -> &'static str {
        "AWS::NetworkManager::CustomerGatewayAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.customer_gateway_arn;

        if the_val.len() > 500 as _ {
            return Err(format!("Max validation failed on field 'customer_gateway_arn'. {} is greater than 500", the_val.len()));
        }

        
        let the_val = &self.customer_gateway_arn;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'customer_gateway_arn'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.device_id;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'device_id'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.device_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'device_id'. {} is less than 0", the_val.len()));
        }

        
        let the_val = &self.global_network_id;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'global_network_id'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.global_network_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'global_network_id'. {} is less than 0", the_val.len()));
        }

        
        if let Some(the_val) = &self.link_id {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'link_id'. {} is greater than 50", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.link_id {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'link_id'. {} is less than 0", the_val.len()));
        }

        }
        
        Ok(())
    }
}