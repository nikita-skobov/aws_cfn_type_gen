

/// Describes the association between a device and a link.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLinkAssociation {


    /// 
    /// The device ID for the link association.
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
    #[serde(rename = "LinkId")]
    pub link_id: String,

}



impl cfn_resources::CfnResource for CfnLinkAssociation {
    fn type_string() -> &'static str {
        "AWS::NetworkManager::LinkAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

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

        
        let the_val = &self.link_id;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'link_id'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.link_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'link_id'. {} is less than 0", the_val.len()));
        }

        
        Ok(())
    }
}