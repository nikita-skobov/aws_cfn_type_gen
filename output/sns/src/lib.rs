
pub mod cfn_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnSubscription {
    /// No documentation provided by AWS
    #[serde(rename = "RawMessageDelivery")]
    pub raw_message_delivery: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "FilterPolicyScope")]
    pub filter_policy_scope: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DeliveryPolicy")]
    pub delivery_policy: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Region")]
    pub region: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// No documentation provided by AWS
    #[serde(rename = "RedrivePolicy")]
    pub redrive_policy: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "SubscriptionRoleArn")]
    pub subscription_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FilterPolicy")]
    pub filter_policy: Option<()>,

}



}

pub mod cfn_topic {

#[derive(serde::Serialize, Default)]
pub struct CfnTopic {
    /// The name of the topic you want to create. Topic names must include only uppercase and lowercase ASCII letters, numbers, underscores, and hyphens, and must be between 1 and 256 characters long. FIFO topic names must end with .fifo.
    /// 
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the topic name. For more information, see Name Type.
    #[serde(rename = "TopicName")]
    pub topic_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Tracing mode of an Amazon SNS topic. By default TracingConfig is set to PassThrough, and the topic passes through the tracing header it receives from an SNS publisher to its subscriptions. If set to Active, SNS will vend X-Ray segment data to topic owner account if the sampled flag in the tracing header is true. Only supported on standard topics.
    #[serde(rename = "TracingConfig")]
    pub tracing_config: Option<String>,
    /// Enables content-based deduplication for FIFO topics. By default, ContentBasedDeduplication is set to false. If you create a FIFO topic and this attribute is false, you must specify a value for the MessageDeduplicationId parameter for the Publish action.
    /// 
    /// When you set ContentBasedDeduplication to true, Amazon SNS uses a SHA-256 hash to generate the MessageDeduplicationId using the body of the message (but not the attributes of the message).
    /// 
    /// (Optional) To override the generated value, you can specify a value for the the MessageDeduplicationId parameter for the Publish action.
    /// 
    #[serde(rename = "ContentBasedDeduplication")]
    pub content_based_deduplication: Option<bool>,
    /// The body of the policy document you want to use for this topic.
    /// 
    /// You can only add one policy per topic.
    /// 
    /// The policy must be in JSON string format.
    /// 
    /// Length Constraints: Maximum length of 30720
    #[serde(rename = "DataProtectionPolicy")]
    pub data_protection_policy: Option<()>,
    /// Set to true to create a FIFO topic.
    #[serde(rename = "FifoTopic")]
    pub fifo_topic: Option<bool>,
    /// Version of the Amazon SNS signature used. If the SignatureVersion is 1, Signature is a Base64-encoded SHA1withRSA signature of the Message, MessageId, Type, Timestamp, and TopicArn values. If the SignatureVersion is 2, Signature is a Base64-encoded SHA256withRSA signature of the Message, MessageId, Type, Timestamp, and TopicArn values.
    #[serde(rename = "SignatureVersion")]
    pub signature_version: Option<String>,
    /// The display name to use for an Amazon SNS topic with SMS subscriptions.
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// The SNS subscriptions (endpoints) for this topic.
    #[serde(rename = "Subscription")]
    pub subscription: Option<Vec<Subscription>>,
    /// The ID of an AWS-managed customer master key (CMK) for Amazon SNS or a custom CMK. For more information, see Key Terms. For more examples, see KeyId in the AWS Key Management Service API Reference.
    /// 
    /// This property applies only to [server-side-encryption](https://docs.aws.amazon.com/sns/latest/dg/sns-server-side-encryption.html).
    #[serde(rename = "KmsMasterKeyId")]
    pub kms_master_key_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Subscription {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_topic_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnTopicPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Topics")]
    pub topics: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),

}



}
