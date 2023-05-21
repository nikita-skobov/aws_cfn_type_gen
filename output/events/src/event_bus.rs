

/// Creates a new event bus within your account. This can be a custom event bus which you can    use to receive events from your custom applications and services, or it can be a partner event    bus which can be matched to a partner event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventBus {


    /// 
    /// If you are creating a partner event bus, this specifies the partner event source that the    new event bus will be matched with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: aws\.partner(/[\.\-_A-Za-z0-9]+){2,}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventSourceName")]
    pub event_source_name: Option<String>,


    /// 
    /// The name of the new event bus.
    /// 
    /// Custom event bus names can't contain the / character, but you can use the / character in partner event bus names. In addition, for partner event buses, the name must exactly match the name of the partner event    source that this event bus is matched to.
    /// 
    /// You can't use the name default for a custom event bus, as this name is already used for your account's    default event bus.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [/\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Tags to associate with the event bus.
    /// 
    /// Required: No
    ///
    /// Type: List of TagEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagEntry>>,

}



impl cfn_resources::CfnResource for CfnEventBus {
    fn type_string() -> &'static str {
        "AWS::Events::EventBus"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.event_source_name {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'event_source_name'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.event_source_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'event_source_name'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.name;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A key-value pair associated with an AWS resource. In EventBridge, rules and event buses    support tagging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagEntry {


    /// 
    /// A string you can use to assign a value. The combination of tag keys and values can help    you organize and categorize your resources.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the specified tag key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for TagEntry {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.key;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'key'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'key'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.value;

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'value'. {} is greater than 256", the_val.len()));
        }

        
        let the_val = &self.value;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'value'. {} is less than 0", the_val.len()));
        }

        
        Ok(())
    }
}