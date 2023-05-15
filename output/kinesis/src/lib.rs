
pub mod cfn_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnStream {
    /// The mode in which the stream is running.
    #[serde(rename = "StreamModeDetails")]
    pub stream_mode_details: Option<StreamModeDetails>,
    /// When specified, enables or updates server-side encryption using an AWS KMS key for a specified stream.
    #[serde(rename = "StreamEncryption")]
    pub stream_encryption: Option<StreamEncryption>,
    /// The number of hours for the data records that are stored in shards to remain accessible.
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: Option<usize>,
    /// An arbitrary set of tags (key–value pairs) to associate with the Kinesis stream.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The number of shards that the stream uses. Required when StreamMode = PROVISIONED is passed.
    #[serde(rename = "ShardCount")]
    pub shard_count: Option<usize>,
    /// The name of the Kinesis stream.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct StreamEncryption {
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
    #[serde(rename = "KeyId")]
    pub key_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct StreamModeDetails {
    #[serde(rename = "StreamMode")]
    pub stream_mode: String,

}


}

pub mod cfn_stream_consumer {

#[derive(serde::Serialize, Default)]
pub struct CfnStreamConsumer {
    /// No documentation provided by AWS
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConsumerName")]
    pub consumer_name: String,

}



}
