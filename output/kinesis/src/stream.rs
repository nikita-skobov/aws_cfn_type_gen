

/// Creates a Kinesis stream that captures and transports data records that are emitted       from data sources. For information about creating streams, see CreateStream in the Amazon Kinesis API Reference.
#[derive(Default, serde::Serialize)]
pub struct CfnStream {


    /// 
    /// The number of hours for the data records that are stored in shards to remain       accessible. The default value is 24. For more information about the stream retention       period, see Changing the Data Retention         Period in the Amazon Kinesis Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: Option<i64>,


    /// 
    /// Specifies the capacity mode to which you want to set your data stream. Currently, in       Kinesis Data Streams, you can choose between an on-demand capacity mode and a provisioned capacity mode for your data streams.
    /// 
    /// Required: No
    ///
    /// Type: StreamModeDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamModeDetails")]
    pub stream_mode_details: Option<StreamModeDetails>,


    /// 
    /// An arbitrary set of tags (key–value pairs) to associate with the Kinesis stream.       For information about constraints for this property, see Tag Restrictions       in the Amazon Kinesis Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// When specified, enables or updates server-side encryption using an AWS KMS key for a specified stream. Removing this property from your stack       template and updating your stack disables encryption.
    /// 
    /// Required: No
    ///
    /// Type: StreamEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamEncryption")]
    pub stream_encryption: Option<StreamEncryption>,


    /// 
    /// The name of the Kinesis stream. If you don't specify a name, AWS       CloudFormation generates a unique physical ID and uses that ID for the stream name. For       more information, see Name Type.
    /// 
    /// If you specify a name, you cannot perform updates that require replacement of this       resource. You can perform updates that require no or some interruption. If you must       replace the resource, specify a new name.
    /// 
    /// Required: No
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The number of shards that the stream uses. For greater provisioned throughput,       increase the number of shards.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShardCount")]
    pub shard_count: Option<i64>,

}


/// Specifies the capacity mode to which you want to set your data stream. Currently, in       Kinesis Data Streams, you can choose between an on-demand capacity mode and a provisioned capacity mode for your data streams.
#[derive(Default, serde::Serialize)]
pub struct StreamModeDetails {


    /// 
    /// Specifies the capacity mode to which you want to set your data stream. Currently, in       Kinesis Data Streams, you can choose between an on-demand capacity mode and a provisioned capacity mode for your data streams.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | PROVISIONED
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamMode")]
    pub stream_mode: String,

}


/// Enables or updates server-side encryption using an AWS KMS key for a       specified stream.
///
/// Starting encryption is an asynchronous operation. Upon receiving the request, Kinesis       Data Streams returns immediately and sets the status of the stream to         UPDATING. After the update is complete, Kinesis Data Streams sets the       status of the stream back to ACTIVE. Updating or applying encryption       normally takes a few seconds to complete, but it can take minutes. You can continue to       read and write data to your stream while its status is UPDATING. Once the       status of the stream is ACTIVE, encryption begins for records written to       the stream.
///
/// API Limits: You can successfully apply a new AWS KMS key for       server-side encryption 25 times in a rolling 24-hour period.
///
/// Note: It can take up to 5 seconds after the stream is in an ACTIVE status       before all records written to the stream are encrypted. After you enable encryption, you       can verify that encryption is applied by inspecting the API response from         PutRecord or PutRecords.
#[derive(Default, serde::Serialize)]
pub struct StreamEncryption {


    /// 
    /// The encryption type to use. The only valid value is KMS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,


    /// 
    /// The GUID for the customer-managed AWS KMS key to use for encryption.       This value can be a globally unique identifier, a fully specified Amazon Resource Name       (ARN) to either an alias or a key, or an alias name prefixed by "alias/".You can also       use a master key owned by Kinesis Data Streams by specifying the alias         aws/kinesis.
    /// 
    /// Key ARN example:             arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012                       Alias ARN example:             arn:aws:kms:us-east-1:123456789012:alias/MyAliasName                       Globally unique key ID example:             12345678-1234-1234-1234-123456789012                       Alias name example: alias/MyAliasName                       Master key owned by Kinesis Data Streams:           alias/aws/kinesis
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyId")]
    pub key_id: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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
