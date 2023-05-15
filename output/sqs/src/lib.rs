
pub mod cfn_queue {

#[derive(serde::Serialize, Default)]
pub struct CfnQueue {
    /// The number of seconds that Amazon SQS retains a message. You can specify an integer value from 60 seconds (1 minute) to 1,209,600 seconds (14 days). The default value is 345,600 seconds (4 days).
    #[serde(rename = "MessageRetentionPeriod")]
    pub message_retention_period: Option<usize>,
    /// A name for the queue. To create a FIFO queue, the name of your FIFO queue must end with the .fifo suffix.
    #[serde(rename = "QueueName")]
    pub queue_name: Option<String>,
    /// The ID of an AWS managed customer master key (CMK) for Amazon SQS or a custom CMK. To use the AWS managed CMK for Amazon SQS, specify the (default) alias alias/aws/sqs.
    #[serde(rename = "KmsMasterKeyId")]
    pub kms_master_key_id: Option<String>,
    /// Specifies whether message deduplication occurs at the message group or queue level. Valid values are messageGroup and queue.
    #[serde(rename = "DeduplicationScope")]
    pub deduplication_scope: Option<String>,
    /// If set to true, creates a FIFO queue. If you don't specify this property, Amazon SQS creates a standard queue.
    #[serde(rename = "FifoQueue")]
    pub fifo_queue: Option<bool>,
    /// The limit of how many bytes that a message can contain before Amazon SQS rejects it. You can specify an integer value from 1,024 bytes (1 KiB) to 262,144 bytes (256 KiB). The default value is 262,144 (256 KiB).
    #[serde(rename = "MaximumMessageSize")]
    pub maximum_message_size: Option<usize>,
    /// Specifies whether the FIFO queue throughput quota applies to the entire queue or per message group. Valid values are perQueue and perMessageGroupId. The perMessageGroupId value is allowed only when the value for DeduplicationScope is messageGroup.
    #[serde(rename = "FifoThroughputLimit")]
    pub fifo_throughput_limit: Option<String>,
    /// The time in seconds for which the delivery of all messages in the queue is delayed. You can specify an integer value of 0 to 900 (15 minutes). The default value is 0.
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: Option<usize>,
    /// A string that includes the parameters for the dead-letter queue functionality (redrive policy) of the source queue.
    #[serde(rename = "RedrivePolicy")]
    pub redrive_policy: Option<()>,
    /// The string that includes the parameters for the permissions for the dead-letter queue redrive permission and which source queues can specify dead-letter queues as a JSON object.
    #[serde(rename = "RedriveAllowPolicy")]
    pub redrive_allow_policy: Option<()>,
    /// The length of time during which a message will be unavailable after a message is delivered from the queue. This blocks other components from receiving the same message and gives the initial component time to process and delete the message from the queue. Values must be from 0 to 43,200 seconds (12 hours). If you don't specify a value, AWS CloudFormation uses the default value of 30 seconds.
    #[serde(rename = "VisibilityTimeout")]
    pub visibility_timeout: Option<usize>,
    /// The tags that you attach to this queue.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// For first-in-first-out (FIFO) queues, specifies whether to enable content-based deduplication. During the deduplication interval, Amazon SQS treats messages that are sent with identical content as duplicates and delivers only one copy of the message.
    #[serde(rename = "ContentBasedDeduplication")]
    pub content_based_deduplication: Option<bool>,
    /// Enables server-side queue encryption using SQS owned encryption keys. Only one server-side encryption option is supported per queue (e.g. SSE-KMS or SSE-SQS ).
    #[serde(rename = "SqsManagedSseEnabled")]
    pub sqs_managed_sse_enabled: Option<bool>,
    /// Specifies the duration, in seconds, that the ReceiveMessage action call waits until a message is in the queue in order to include it in the response, rather than returning an empty response if a message isn't yet available. You can specify an integer from 1 to 20. Short polling is used as the default or when you specify 0 for this property.
    #[serde(rename = "ReceiveMessageWaitTimeSeconds")]
    pub receive_message_wait_time_seconds: Option<usize>,
    /// The length of time in seconds for which Amazon SQS can reuse a data key to encrypt or decrypt messages before calling AWS KMS again. The value must be an integer between 60 (1 minute) and 86,400 (24 hours). The default is 300 (5 minutes).
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    pub kms_data_key_reuse_period_seconds: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_queue_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnQueuePolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Queues")]
    pub queues: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),

}



}
