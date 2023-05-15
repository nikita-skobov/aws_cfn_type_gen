
pub mod cfn_signaling_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnSignalingChannel {
    /// The type of the Kinesis Video Signaling Channel to create. Currently, SINGLE_MASTER is the only supported channel type.
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// The name of the Kinesis Video Signaling Channel.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The period of time a signaling channel retains undelivered messages before they are discarded.
    #[serde(rename = "MessageTtlSeconds")]
    pub message_ttl_seconds: Option<usize>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnStream {
    /// The media type of the stream. Consumers of the stream can use this information when processing the stream.
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,
    /// AWS KMS key ID that Kinesis Video Streams uses to encrypt stream data.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// The number of hours till which Kinesis Video will retain the data in the stream
    #[serde(rename = "DataRetentionInHours")]
    pub data_retention_in_hours: Option<usize>,
    /// The name of the Kinesis Video stream.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// An array of key-value pairs associated with the Kinesis Video Stream.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the device that is writing to the stream.
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
