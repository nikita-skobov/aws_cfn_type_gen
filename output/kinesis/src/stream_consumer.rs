

/// Use the AWS CloudFormation         AWS::Kinesis::StreamConsumer resource to register a consumer with a       Kinesis data stream. The consumer you register can then call SubscribeToShard       to receive data from the stream using enhanced fan-out, at a rate of up to 2 MiB per       second for every shard you subscribe to. This rate is unaffected by the total number of       consumers that read from the same stream.
///
/// You can register up to five consumers per stream. However, you can request a limit       increase using the Kinesis Data Streams limits         form. A given consumer can only be registered with one stream at a time.
///
/// For more information, see Using Consumers         with Enhanced Fan-Out.
#[derive(Default, serde::Serialize)]
pub struct CfnStreamConsumer {


    /// 
    /// The ARN of the stream with which you registered the consumer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws.*:kinesis:.*:\d{12}:stream/\S+
    ///
    /// Update requires: Replacement
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,


    /// 
    /// The name of the consumer is something you choose when you register the       consumer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConsumerName")]
    pub consumer_name: String,

}
