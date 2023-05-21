

/// Creates the connector, which captures the parameters for an outbound connection for the    AS2 protocol. The connector is required for sending files to an externally hosted AS2 server.    For more details about connectors, see Create AS2 connectors.
#[derive(Default, serde::Serialize)]
pub struct CfnConnector {


    /// 
    /// With AS2, you can send files by calling StartFileTransfer and specifying the    file paths in the request parameter, SendFilePaths. We use the file’s parent    directory (for example, for --send-file-paths /bucket/dir/file.txt, parent    directory is /bucket/dir/) to temporarily store a processed AS2 message file,    store the MDN when we receive them from the partner, and write a final JSON file containing    relevant metadata of the transmission. So, the AccessRole needs to provide read    and write access to the parent directory of the file location used in the     StartFileTransfer request. Additionally, you need to provide read and write    access to the parent directory of the files that you intend to send with     StartFileTransfer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessRole")]
    pub access_role: String,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that allows a connector to turn    on CloudWatch logging for Amazon S3 events. When set, you can view connector    activity in your CloudWatch logs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingRole")]
    pub logging_role: Option<String>,


    /// 
    /// Key-value pairs that can be used to group and search for connectors.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A structure that contains the parameters for a connector object.
    /// 
    /// Required: Yes
    ///
    /// Type: As2Config
    ///
    /// Update requires: No interruption
    #[serde(rename = "As2Config")]
    pub as2_config: As2Config,


    /// 
    /// The URL of the partner's AS2 endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: String,

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


/// A structure that contains the parameters for a connector object.
#[derive(Default, serde::Serialize)]
pub struct As2Config {


    /// 
    /// Used for outbound requests (from an AWS Transfer Family server to a partner AS2 server) to determine whether    the partner response for transfers is synchronous or asynchronous. Specify either of the following values:
    /// 
    /// SYNC: The system expects a synchronous MDN response, confirming that the file was transferred successfully (or not).                        NONE: Specifies that no MDN response is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "MdnResponse")]
    pub mdn_response: Option<String>,


    /// 
    /// A unique identifier for the partner profile for the connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartnerProfileId")]
    pub partner_profile_id: Option<String>,


    /// 
    /// The algorithm that is used to encrypt the file.
    /// 
    /// NoteYou can only specify NONE if the URL for your connector uses HTTPS. This ensures that     no traffic is sent in clear text.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AES128_CBC | AES192_CBC | AES256_CBC | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionAlgorithm")]
    pub encryption_algorithm: Option<String>,


    /// 
    /// Used as the Subject HTTP header attribute in AS2 messages that are being sent with the connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[\p{Print}\p{Blank}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageSubject")]
    pub message_subject: Option<String>,


    /// 
    /// A unique identifier for the AS2 local profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalProfileId")]
    pub local_profile_id: Option<String>,


    /// 
    /// The signing algorithm for the MDN response.
    /// 
    /// NoteIf set to DEFAULT (or not set at all), the value for SigningAlgorithm is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT | NONE | SHA1 | SHA256 | SHA384 | SHA512
    ///
    /// Update requires: No interruption
    #[serde(rename = "MdnSigningAlgorithm")]
    pub mdn_signing_algorithm: Option<String>,


    /// 
    /// The algorithm that is used to sign the AS2 messages sent with the connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SHA1 | SHA256 | SHA384 | SHA512
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: Option<String>,


    /// 
    /// Specifies whether the AS2 file is compressed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ZLIB
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compression")]
    pub compression: Option<String>,

}