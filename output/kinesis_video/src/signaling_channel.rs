

/// Specifies a signaling channel.
///
/// CreateSignalingChannel is an asynchronous operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSignalingChannel {


    /// 
    /// The period of time a signaling channel retains undelivered messages before they are       discarded.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 5
    ///
    /// Maximum: 120
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageTtlSeconds")]
    pub message_ttl_seconds: Option<i64>,


    /// 
    /// A name for the signaling channel that you are creating. It must be unique for each AWS account and AWS Region.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


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
    /// A type of the signaling channel that you are creating. Currently,         SINGLE_MASTER is the only supported channel type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FULL_MESH | SINGLE_MASTER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<SignalingChannelTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SignalingChannelTypeEnum {

    /// FULL_MESH
    #[serde(rename = "FULL_MESH")]
    Fullmesh,

    /// SINGLE_MASTER
    #[serde(rename = "SINGLE_MASTER")]
    Singlemaster,

}

impl Default for SignalingChannelTypeEnum {
    fn default() -> Self {
        SignalingChannelTypeEnum::Fullmesh
    }
}


impl cfn_resources::CfnResource for CfnSignalingChannel {
    fn type_string() -> &'static str {
        "AWS::KinesisVideo::SignalingChannel"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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


