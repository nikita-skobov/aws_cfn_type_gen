

/// The AWS::SageMaker::Device resource is an Amazon SageMaker resource type       that allows you to register your Devices against an existing SageMaker Edge Manager       DeviceFleet. Each device must be listed individually in the CFN specification.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDevice {


    /// 
    /// Edge device you want to create.
    /// 
    /// Required: No
    ///
    /// Type: Device
    ///
    /// Update requires: No interruption
    #[serde(rename = "Device")]
    pub device: Option<Box<Device>>,


    /// 
    /// The name of the fleet the device belongs to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceFleetName")]
    pub device_fleet_name: String,


    /// 
    /// An array of key-value pairs that contain metadata to help you categorize and organize       your devices. Each tag consists of a key and a value, both of which you define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnDevice {
    fn type_string() -> &'static str {
        "AWS::SageMaker::Device"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.device.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.device_fleet_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'device_fleet_name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.device_fleet_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'device_fleet_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Information of a particular device.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Device {


    /// 
    /// Description of the device.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40
    ///
    /// Pattern: [\S\s]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the device.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    pub device_name: String,


    /// 
    /// AWS Internet of Things (IoT) object name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9:_-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotThingName")]
    pub iot_thing_name: Option<String>,

}



impl cfn_resources::CfnResource for Device {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 40 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 40", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.device_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'device_name'. {} is greater than 63", the_val.len()));
        }

        
        let the_val = &self.device_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'device_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.iot_thing_name {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'iot_thing_name'. {} is greater than 128", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}