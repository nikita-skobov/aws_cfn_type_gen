

/// The AWS::SSMIncidents::ReplicationSet resource specifies a set of Regions       that Incident Manager data is replicated to and the KMS key used to encrypt       the data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnReplicationSet {


    /// 
    /// Determines if the replication set deletion protection is enabled or not. If deletion       protection is enabled, you can't delete the last Region in the replication set.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtected")]
    pub deletion_protected: Option<bool>,


    /// 
    /// Specifies the Regions of the replication set.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ReplicationRegion
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regions")]
    pub regions: Vec<ReplicationRegion>,


    /// 
    /// A list of tags to add to the replication set.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnReplicationSet {
    fn type_string(&self) -> &'static str {
        "AWS::SSMIncidents::ReplicationSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The RegionConfiguration property specifies the Region and KMS key to add       to the replication set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RegionConfiguration {


    /// 
    /// The KMS key ID to use to encrypt your replication set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SseKmsKeyId")]
    pub sse_kms_key_id: String,

}



impl cfn_resources::CfnResource for RegionConfiguration {
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

/// The ReplicationRegion property type specifies the Region and KMS key to       add to the replication set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationRegion {


    /// 
    /// Specifies the Region configuration.
    /// 
    /// Required: No
    ///
    /// Type: RegionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionConfiguration")]
    pub region_configuration: Option<RegionConfiguration>,


    /// 
    /// Specifies the region name to add to the replication set.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionName")]
    pub region_name: Option<String>,

}



impl cfn_resources::CfnResource for ReplicationRegion {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.region_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

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