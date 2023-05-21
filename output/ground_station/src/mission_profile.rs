

/// Mission profiles specify parameters and provide references to config objects to define how Ground Station lists and executes contacts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMissionProfile {


    /// 
    /// Amount of time in seconds after a contact ends that you’d like to receive a CloudWatch Event indicating the pass has finished.       For more information on CloudWatch Events, see the What Is CloudWatch Events?
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactPostPassDurationSeconds")]
    pub contact_post_pass_duration_seconds: Option<i64>,


    /// 
    /// Amount of time in seconds prior to contact start that you'd like to receive a CloudWatch Event indicating an upcoming pass.       For more information on CloudWatch Events, see the What Is CloudWatch Events?
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactPrePassDurationSeconds")]
    pub contact_pre_pass_duration_seconds: Option<i64>,


    /// 
    /// A list containing lists of config ARNs. Each list of config ARNs is an edge, with a "from" config and a "to" config.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DataflowEdge
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataflowEdges")]
    pub dataflow_edges: Vec<DataflowEdge>,


    /// 
    /// Minimum length of a contact in seconds that Ground Station will return when listing contacts.       Ground Station will not return contacts shorter than this duration.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumViableContactDurationSeconds")]
    pub minimum_viable_contact_duration_seconds: i64,


    /// 
    /// The name of the mission profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: StreamsKmsKey
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamsKmsKey")]
    pub streams_kms_key: Option<StreamsKmsKey>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamsKmsRole")]
    pub streams_kms_role: Option<String>,


    /// 
    /// Tags assigned to the mission profile.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ARN of a tracking config objects that defines how to track the satellite through the sky during a contact.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackingConfigArn")]
    pub tracking_config_arn: String,

}



impl cfn_resources::CfnResource for CfnMissionProfile {
    fn type_string(&self) -> &'static str {
        "AWS::GroundStation::MissionProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.streams_kms_key.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A dataflow edge defines from where and to where data will flow during a contact.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataflowEdge {


    /// 
    /// The ARN of the destination for this dataflow edge.       For example, specify the ARN of a dataflow endpoint config for a downlink edge or an antenna uplink config for an uplink edge.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<String>,


    /// 
    /// The ARN of the source for this dataflow edge.       For example, specify the ARN of an antenna downlink config for a downlink edge or a dataflow endpoint config for an uplink edge.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Option<String>,

}



impl cfn_resources::CfnResource for DataflowEdge {
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

/// The StreamsKmsKey property type specifies Property description not available. for an AWS::GroundStation::MissionProfile.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StreamsKmsKey {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsAliasArn")]
    pub kms_alias_arn: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,

}



impl cfn_resources::CfnResource for StreamsKmsKey {
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