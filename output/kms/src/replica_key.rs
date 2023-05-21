

/// The AWS::KMS::ReplicaKey resource specifies a multi-Region replica key that    is based on a multi-Region primary key.
///
/// Multi-Region keys are an AWS KMS feature that lets you    create multiple interoperable KMS keys in different AWS Regions. Because these    KMS keys have the same key ID, key material, and other metadata, you can use them to encrypt data    in one AWS Region and decrypt it in a different AWS Region    without making a cross-Region call or exposing the plaintext data. For more information, see     Multi-Region keys in the AWS Key Management Service Developer     Guide.
///
/// A multi-Region primary key is a fully functional symmetric encryption KMS key, HMAC KMS key, or    asymmetric KMS key that is also the model for replica keys in other AWS Regions.    To create a multi-Region primary key, add an AWS::KMS::Key    resource to your CloudFormation stack. Set its MultiRegion property to    true.
///
/// A multi-Region replica key is a fully functional KMS key that has the    same key ID and key material as a multi-Region primary key, but is located in a different     AWS Region of the same AWS partition. There can be    multiple replicas of a primary key, but each must be in a different AWS Region.
///
/// When you create a replica key in AWS CloudFormation, the replica key is created in the AWS Region    represented by the endpoint you use for the request. If you try to replicate a multi-Region key into a Region in which the key type   is not supported, the request will fail.
///
/// A primary key and its replicas have the same key ID and key material. They also have the    same key spec, key usage, key material origin, and automatic key rotation status. These    properties are known as shared properties. If they change, AWS KMS synchronizes the change to all related multi-Region keys. All other properties    of a replica key can differ, including its key policy, tags, aliases, and key state. AWS KMS does not synchronize these properties.
///
/// Regions
///
/// AWS KMS CloudFormation resources are available in all AWS Regions in which AWS KMS and    AWS CloudFormation are supported. You can use the AWS::KMS::ReplicaKey    resource to create replica keys in all Regions that support multi-Region KMS keys. For    details, see Multi-Region keys in AWS KMS in the AWS Key Management Service Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnReplicaKey {


    /// 
    /// Specifies whether the replica key is enabled. Disabled KMS keys cannot be used in    cryptographic operations.
    /// 
    /// When Enabled is true, the key state of the    KMS key is Enabled. When Enabled is false, the key state of    the KMS key is Disabled. The default value is true.
    /// 
    /// The actual key state of the replica might be affected by actions taken outside of    CloudFormation, such as running the EnableKey, DisableKey,    or ScheduleKeyDeletion operations. Also, while the replica key is being created, its    key state is Creating. When the process is complete, the key state of the replica    key changes to Enabled.
    /// 
    /// For information about the key states of a KMS key, see Key state: Effect on your KMS key in the      AWS Key Management Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Specifies the multi-Region primary key to replicate. The primary key must be in a    different AWS Region of the same AWS partition. You can    create only one replica of a given primary key in each AWS Region .
    /// 
    /// ImportantIf you change the PrimaryKeyArn value of a replica key, the existing     replica key is scheduled for deletion and a new replica key is created based on the     specified primary key. While it is scheduled for deletion, the existing replica key becomes     unusable. You can cancel the scheduled deletion of the key outside of CloudFormation.However, if you inadvertently delete a replica key, you can decrypt ciphertext encrypted     by that replica key by using any related multi-Region key. If necessary, you can recreate     the replica in the same Region after the previous one is completely deleted. For details,     see Deleting multi-Region      keys in the AWS Key Management Service Developer Guide
    /// 
    /// Specify the key ARN of an existing multi-Region primary key. For example,     arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrimaryKeyArn")]
    pub primary_key_arn: String,


    /// 
    /// Assigns one or more tags to the replica key.
    /// 
    /// NoteTagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see      ABAC for       AWS KMS in the AWS Key Management Service Developer      Guide.
    /// 
    /// Tags are not a shared property of multi-Region keys. You can specify the same tags or    different tags for each key in a set of related multi-Region keys. AWS KMS does    not synchronize this property.
    /// 
    /// Each tag consists of a tag key and a tag value. Both the tag key and the tag value are    required, but the tag value can be an empty (null) string. You cannot have more than one tag    on a KMS key with the same tag key. If you specify an existing tag key with a different tag value,     AWS KMS replaces the current tag value with the specified one.
    /// 
    /// When you assign tags to an AWSresource, AWSgenerates a    cost allocation report with usage and costs aggregated by tags. Tags can also be used to    control access to a KMS key. For details, see Tagging keys.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A description of the KMS key.
    /// 
    /// The default value is an empty string (no description).
    /// 
    /// The description is not a shared property of multi-Region keys. You can specify the same    description or a different description for each key in a set of related multi-Region keys. AWS Key Management Service does not synchronize this property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 8192
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The key policy that authorizes use of the replica key.
    /// 
    /// The key policy is not a shared property of multi-Region keys. You can specify the same key    policy or a different key policy for each key in a set of related multi-Region keys. AWS KMS does not synchronize this property.
    /// 
    /// The key policy must conform to the following rules.
    /// 
    /// The key policy must give the caller PutKeyPolicy permission on      the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information,      refer to the scenario in the Default key policy section of the       AWS Key Management Service Developer Guide      .        Each statement in the key policy must contain one or more principals. The principals      in the key policy must exist and be visible to AWS KMS. When you create a new       AWSprincipal (for example, an IAM user or role), you might need to      enforce a delay before including the new principal in a key policy because the new      principal might not be immediately visible to AWS KMS. For more information,      see Changes that I make are not always immediately visible in the AWS Identity and Access Management User Guide.
    /// 
    /// A key policy document can include only the following characters:
    /// 
    /// Printable ASCII characters from the space character (\u0020) through the end of the ASCII character range.        Printable characters in the Basic Latin and Latin-1 Supplement character set (through \u00FF).        The tab (\u0009), line feed (\u000A), and carriage return (\u000D) special characters
    /// 
    /// Minimum: 1
    /// 
    /// Maximum: 32768
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyPolicy")]
    pub key_policy: serde_json::Value,


    /// 
    /// Specifies the number of days in the waiting period before AWS KMS deletes a    replica key that has been removed from a CloudFormation stack. Enter a value between 7 and 30    days. The default value is 30 days.
    /// 
    /// When you remove a replica key from a CloudFormation stack, AWS KMS schedules    the replica key for deletion and starts the mandatory waiting period. The     PendingWindowInDays property determines the length of waiting period. During    the waiting period, the key state of replica key is Pending Deletion, which    prevents it from being used in cryptographic operations. When the waiting period expires,     AWS KMS permanently deletes the replica key.
    /// 
    /// If the KMS key is a multi-Region primary key with replica keys, the waiting period begins when    the last of its replica keys is deleted. Otherwise, the waiting period begins    immediately.
    /// 
    /// You cannot use a CloudFormation template to cancel deletion of the replica after you    remove it from the stack, regardless of the waiting period. However, if you specify a replica    key in your template that is based on the same primary key as the original replica key,    CloudFormation creates a new replica key with the same key ID, key material, and other shared    properties of the original replica key. This new replica key can decrypt ciphertext that was    encrypted under the original replica key, or any related multi-Region key.
    /// 
    /// For detailed information about deleting multi-Region keys, see Deleting multi-Region     keys in the AWS Key Management Service Developer Guide.
    /// 
    /// For information about the PendingDeletion key state, see Key state: Effect on     your KMS key in the AWS Key Management Service Developer Guide. For    more information about deleting KMS keys, see the ScheduleKeyDeletion    operation in the AWS Key Management Service API Reference and Deleting KMS     keys in the AWS Key Management Service Developer Guide.
    /// 
    /// Minimum: 7
    /// 
    /// Maximum: 30
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: Option<i64>,

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
