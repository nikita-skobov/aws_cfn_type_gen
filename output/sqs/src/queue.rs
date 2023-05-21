

/// The AWS::SQS::Queue resource creates an Amazon SQS standard or FIFO    queue.
///
/// Keep the following caveats in mind:
///
/// For more information about creating FIFO (first-in-first-out) queues, see Creating an Amazon SQS queue (AWS CloudFormation) in the Amazon SQS Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnQueue {


    /// 
    /// Enables server-side queue encryption using SQS owned encryption keys. Only one    server-side encryption option is supported per queue (for example, SSE-KMS or SSE-SQS).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqsManagedSseEnabled")]
    pub sqs_managed_sse_enabled: Option<bool>,


    /// 
    /// The tags that you attach to this queue. For more information, see Resource     tag in the AWS CloudFormation User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// For first-in-first-out (FIFO) queues, specifies whether to enable content-based    deduplication. During the deduplication interval, Amazon SQS treats messages that are    sent with identical content as duplicates and delivers only one copy of the message. For more    information, see the ContentBasedDeduplication attribute for the     CreateQueue    action in the Amazon SQS API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentBasedDeduplication")]
    pub content_based_deduplication: Option<bool>,


    /// 
    /// The limit of how many bytes that a message can contain before Amazon SQS rejects    it. You can specify an integer value from 1,024 bytes (1 KiB) to     262,144 bytes (256 KiB). The default value is 262,144 (256    KiB).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumMessageSize")]
    pub maximum_message_size: Option<i64>,


    /// 
    /// The time in seconds for which the delivery of all messages in the queue is delayed. You    can specify an integer value of 0 to 900 (15 minutes). The default    value is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: Option<i64>,


    /// 
    /// The number of seconds that Amazon SQS retains a message. You can specify an    integer value from 60 seconds (1 minute) to 1,209,600 seconds (14    days). The default value is 345,600 seconds (4 days).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageRetentionPeriod")]
    pub message_retention_period: Option<i64>,


    /// 
    /// The string that includes the parameters for the dead-letter queue functionality of the    source queue as a JSON object. The parameters are as follows:
    /// 
    /// deadLetterTargetArn: The Amazon Resource Name (ARN) of the dead-letter      queue to which Amazon SQS moves messages after the value of       maxReceiveCount is exceeded.        maxReceiveCount: The number of times a message is delivered to the source      queue before being moved to the dead-letter queue. When the ReceiveCount for      a message exceeds the maxReceiveCount for a queue, Amazon SQS moves      the message to the dead-letter-queue.
    /// 
    /// NoteThe dead-letter queue of a FIFO queue must also be a FIFO queue. Similarly, the     dead-letter queue of a standard queue must also be a standard queue.
    /// 
    /// JSON
    /// 
    /// { "deadLetterTargetArn" : String, "maxReceiveCount" :      Integer }
    /// 
    /// YAML
    /// 
    /// deadLetterTargetArn : String
    /// 
    /// maxReceiveCount : Integer
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedrivePolicy")]
    pub redrive_policy: Option<serde_json::Value>,


    /// 
    /// If set to true, creates a FIFO queue. If you don't specify this property, Amazon SQS creates a standard queue. For more information, see FIFO queues in    the Amazon SQS Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "FifoQueue")]
    pub fifo_queue: Option<bool>,


    /// 
    /// The ID of an AWS Key Management Service (KMS) for Amazon SQS, or a    custom KMS. To use the AWS managed KMS for Amazon SQS, specify a    (default) alias ARN, alias name (e.g. alias/aws/sqs), key ARN, or key ID. For    more information, see the following:
    /// 
    /// Encryption at rest in the Amazon SQS Developer       Guide                  CreateQueue      in the Amazon SQS API Reference                  Request Parameters in the AWS Key Management Service       API Reference             The Key Management Service (KMS) section of the AWS Key Management Service Best Practices whitepaper
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsMasterKeyId")]
    pub kms_master_key_id: Option<String>,


    /// 
    /// Specifies the duration, in seconds, that the ReceiveMessage action call waits until a    message is in the queue in order to include it in the response, rather than returning an empty    response if a message isn't yet available. You can specify an integer from 1 to 20. Short    polling is used as the default or when you specify 0 for this property. For more information,    see Consuming messages using long polling in the Amazon SQS Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReceiveMessageWaitTimeSeconds")]
    pub receive_message_wait_time_seconds: Option<i64>,


    /// 
    /// The string that includes the parameters for the permissions for the dead-letter queue    redrive permission and which source queues can specify dead-letter queues as a JSON object.    The parameters are as follows:
    /// 
    /// redrivePermission: The permission type that defines which source queues      can specify the current queue as the dead-letter queue. Valid values are:                                  allowAll: (Default) Any source queues in this AWS        account in the same Region can specify this queue as the dead-letter queue.            denyAll: No source queues can specify this queue as the dead-letter        queue.            byQueue: Only queues specified by the sourceQueueArns        parameter can specify this queue as the dead-letter queue.              sourceQueueArns: The Amazon Resource Names (ARN)s of the source queues      that can specify this queue as the dead-letter queue and redrive messages. You can specify      this parameter only when the redrivePermission parameter is set to       byQueue. You can specify up to 10 source queue ARNs. To allow more than 10      source queues to specify dead-letter queues, set the redrivePermission      parameter to allowAll.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedriveAllowPolicy")]
    pub redrive_allow_policy: Option<serde_json::Value>,


    /// 
    /// A name for the queue. To create a FIFO queue, the name of your FIFO queue must end with    the .fifo suffix. For more information, see FIFO queues in    the Amazon SQS Developer Guide.
    /// 
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses    that ID for the queue name. For more information, see Name type in the      AWS CloudFormation User Guide.
    /// 
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this     resource. You can perform updates that require no or some interruption. If you must replace     the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueueName")]
    pub queue_name: Option<String>,


    /// 
    /// For high throughput for FIFO queues, specifies whether the FIFO queue throughput quota    applies to the entire queue or per message group. Valid values are perQueue and     perMessageGroupId.
    /// 
    /// To enable high throughput for a FIFO queue, set this attribute to     perMessageGroupId    and set the DeduplicationScope attribute to     messageGroup. If you set these attributes to anything other than these values,    normal throughput is in effect and deduplication occurs as specified. For more information,    see High     throughput for FIFO queues and Quotas related to     messages in the Amazon SQS Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FifoThroughputLimit")]
    pub fifo_throughput_limit: Option<String>,


    /// 
    /// The length of time during which a message will be unavailable after a message is delivered    from the queue. This blocks other components from receiving the same message and gives the    initial component time to process and delete the message from the queue.
    /// 
    /// Values must be from 0 to 43,200 seconds (12 hours). If you don't specify a value, AWS CloudFormation uses the default value of 30 seconds.
    /// 
    /// For more information about Amazon SQS queue visibility timeouts, see Visibility     timeout in the Amazon SQS Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibilityTimeout")]
    pub visibility_timeout: Option<i64>,


    /// 
    /// The length of time in seconds for which Amazon SQS can reuse a data key to encrypt    or decrypt messages before calling AWS KMS again. The value must be an integer    between 60 (1 minute) and 86,400 (24 hours). The default is 300 (5 minutes).
    /// 
    /// NoteA shorter time period provides better security, but results in more calls to AWS KMS, which might incur charges after Free Tier. For more information, see Encryption at rest in the Amazon SQS Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    pub kms_data_key_reuse_period_seconds: Option<i64>,


    /// 
    /// For high throughput for FIFO queues, specifies whether message deduplication occurs at the    message group or queue level. Valid values are messageGroup and     queue.
    /// 
    /// To enable high throughput for a FIFO queue, set this attribute to     messageGroup    and set the FifoThroughputLimit attribute to     perMessageGroupId. If you set these attributes to anything other than these    values, normal throughput is in effect and deduplication occurs as specified. For more    information, see High     throughput for FIFO queues and Quotas related to     messages in the Amazon SQS Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeduplicationScope")]
    pub deduplication_scope: Option<String>,

}

impl cfn_resources::CfnResource for CfnQueue {
    fn type_string() -> &'static str {
        "AWS::SQS::Queue"
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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
