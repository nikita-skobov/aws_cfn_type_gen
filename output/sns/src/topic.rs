/// The AWS::SNS::Topic resource creates a topic to which notifications can be     published.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTopic {
    ///
    /// Enables content-based deduplication for FIFO topics.
    ///
    /// By default, ContentBasedDeduplication is set to false.        If you create a FIFO topic and this attribute is false, you must specify        a value for the MessageDeduplicationId parameter for the Publish action.                  When you set ContentBasedDeduplication to true, Amazon SNS uses a SHA-256 hash to generate the MessageDeduplicationId        using the body of the message (but not the attributes of the message).       (Optional) To override the generated value, you can specify a value for the the          MessageDeduplicationId parameter for the Publish        action.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentBasedDeduplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_based_deduplication: Option<bool>,

    ///
    /// The body of the policy document you want to use for this topic.
    ///
    /// You can only add one policy per topic.
    ///
    /// The policy must be in JSON string format.
    ///
    /// Length Constraints: Maximum length of 30,720.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protection_policy: Option<serde_json::Value>,

    ///
    /// The display name to use for an Amazon SNS topic with SMS subscriptions. The     display name must be maximum 100 characters long, including hyphens (-), underscores (_),     spaces, and tabs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<cfn_resources::StrVal>,

    ///
    /// Set to true to create a FIFO topic.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "FifoTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fifo_topic: Option<bool>,

    ///
    /// The ID of an AWS managed customer master key (CMK) for Amazon SNS     or a custom CMK. For more information, see Key terms. For     more examples, see       KeyId      in the AWS Key Management Service API Reference.
    ///
    /// This property applies only to server-side-encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The signature version corresponds to the hashing algorithm used while creating the     signature of the notifications, subscription confirmations, or unsubscribe confirmation     messages sent by Amazon SNS. By default, SignatureVersion is set to       1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SignatureVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_version: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon SNS subscriptions (endpoints) for this topic.
    ///
    /// ImportantIf you specify the Subscription property in the        AWS::SNS::Topic resource and it creates an associated subscription       resource, the associated subscription is not deleted when the        AWS::SNS::Topic resource is deleted.
    ///
    /// Required: No
    ///
    /// Type: List of Subscription
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<Subscription>>,

    ///
    /// The list of tags to add to a new topic.
    ///
    /// NoteTo be able to tag a topic on creation, you must have the           sns:CreateTopic and sns:TagResource         permissions.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The name of the topic you want to create. Topic names must include only uppercase and     lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256     characters long. FIFO topic names must end with .fifo.
    ///
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses     that ID for the topic name. For more information, see Name     type.
    ///
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this       resource. You can perform updates that require no or some interruption. If you must       replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<cfn_resources::StrVal>,

    ///
    /// Tracing mode of an Amazon SNS topic. By default TracingConfig is set to       PassThrough, and the topic passes through the tracing header it receives     from an SNS publisher to its subscriptions. If set to Active, SNS will vend     X-Ray segment data to topic owner account if the sampled flag in the tracing header is     true. Only supported on standard topics.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_topic_arn: CfnTopictopicarn,

    #[serde(skip_serializing)]
    pub att_topic_name: CfnTopictopicname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopictopicarn;
impl CfnTopictopicarn {
    pub fn att_name(&self) -> &'static str {
        r#"TopicArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopictopicname;
impl CfnTopictopicname {
    pub fn att_name(&self) -> &'static str {
        r#"TopicName"#
    }
}

impl cfn_resources::CfnResource for CfnTopic {
    fn type_string(&self) -> &'static str {
        "AWS::SNS::Topic"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Subscription is an embedded property that describes the subscription endpoints     of an Amazon SNS topic.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Subscription {
    ///
    /// The endpoint that receives notifications from the Amazon SNS topic. The endpoint     value depends on the protocol that you specify. For more information, see the       Endpoint parameter of the       Subscribe      action in the Amazon SNS API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: cfn_resources::StrVal,

    ///
    /// The subscription's protocol. For more information, see the Protocol     parameter of the       Subscribe      action in the Amazon SNS API Reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Subscription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
