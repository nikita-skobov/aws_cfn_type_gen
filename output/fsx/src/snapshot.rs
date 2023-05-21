

/// A snapshot of an Amazon FSx for OpenZFS volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSnapshot {


    /// 
    /// The name of the snapshot.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 203
    ///
    /// Pattern: ^[a-zA-Z0-9_:.-]{1,203}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ID of the volume that the snapshot is of.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 23
    ///
    /// Maximum: 23
    ///
    /// Pattern: ^(fsvol-[0-9a-f]{17,})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

}



impl cfn_resources::CfnResource for CfnSnapshot {
    fn type_string(&self) -> &'static str {
        "AWS::FSx::Snapshot"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 203 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 203", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.volume_id;

        if the_val.len() > 23 as _ {
            return Err(format!("Max validation failed on field 'volume_id'. {} is greater than 23", the_val.len()));
        }

        
        let the_val = &self.volume_id;

        if the_val.len() < 23 as _ {
            return Err(format!("Min validation failed on field 'volume_id'. {} is less than 23", the_val.len()));
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}