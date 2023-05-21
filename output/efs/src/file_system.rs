

/// The AWS::EFS::FileSystem resource creates a new, empty file system in     Amazon Elastic File System (Amazon EFS). You must create a mount target      (AWS::EFS::MountTarget) to mount your EFS file system on an      Amazon EC2 or other AWS cloud compute resource.
#[derive(Default, serde::Serialize)]
pub struct CfnFileSystem {


    /// 
    /// A Boolean value that, if true, creates an encrypted file system. When creating an    encrypted file system, you have the option of specifying a KmsKeyId for an existing AWS KMS key. If you don't specify a KMS key, then the default KMS key for    Amazon EFS, /aws/elasticfilesystem, is used to protect the encrypted file system.
    /// 
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// Use the BackupPolicy to turn automatic backups on or off for the file system.
    /// 
    /// Required: No
    ///
    /// Type: BackupPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPolicy")]
    pub backup_policy: Option<BackupPolicy>,


    /// 
    /// Use to create one or more tags associated with the file system. Each     tag is a user-defined key-value pair. Name your file system on creation by including a     "Key":"Name","Value":"{value}" key-value pair. Each key must be unique. For more     information, see Tagging AWS resources     in the         AWS General Reference Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ElasticFileSystemTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemTags")]
    pub file_system_tags: Option<Vec<ElasticFileSystemTag>>,


    /// 
    /// (Optional) A boolean that specifies whether or not to bypass the FileSystemPolicy lockout safety check. The lockout safety check    determines whether the policy in the request will lock out, or prevent, the IAM principal that is making the request from making future PutFileSystemPolicy requests on this file system.    Set BypassPolicyLockoutSafetyCheck to True only when you intend to prevent    the IAM principal that is making the request from making subsequent PutFileSystemPolicy requests on this file system.    The default value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    pub bypass_policy_lockout_safety_check: Option<bool>,


    /// 
    /// The throughput, measured in    MiB/s,    that you want to provision for a file system that you're creating. Valid values are    1-1024. Required if ThroughputMode is set to provisioned. The upper    limit for throughput is 1024 MiB/s. To increase this limit, contact AWS Support. For    more information, see Amazon EFS quotas that you can increase in the Amazon EFS User Guide.
    /// 
    /// Required: Conditional
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProvisionedThroughputInMibps")]
    pub provisioned_throughput_in_mibps: Option<f64>,


    /// 
    /// An array of LifecyclePolicy objects that define the file system's     LifecycleConfiguration object. A LifecycleConfiguration object    informs EFS lifecycle management and intelligent tiering of the following:
    /// 
    /// When to move files in the file system from primary storage to the IA storage class.               When to move files that are in IA storage to primary storage.
    /// 
    /// NoteAmazon EFS requires that each LifecyclePolicy object have only a single transition.         This means that in a request body, LifecyclePolicies needs to be structured as an array of         LifecyclePolicy objects, one object for each transition, TransitionToIA,         TransitionToPrimaryStorageClass.    See the example requests in the following section for more information.
    /// 
    /// Required: No
    ///
    /// Type: List of LifecyclePolicy
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecyclePolicies")]
    pub lifecycle_policies: Option<Vec<LifecyclePolicy>>,


    /// 
    /// The performance mode of the file system. We recommend generalPurpose    performance mode for most file systems. File systems using the maxIO performance    mode can scale to higher levels of aggregate throughput and operations per second with a    tradeoff of slightly higher latencies for most file operations. The performance mode    can't be changed after the file system has been created.
    /// 
    /// NoteThe maxIO mode is not supported on file systems using One Zone storage classes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: generalPurpose | maxIO
    ///
    /// Update requires: Replacement
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: Option<String>,


    /// 
    /// Specifies the throughput mode for the file system. The mode can be bursting,     provisioned, or elastic. If you set ThroughputMode to     provisioned, you must also set a value for     ProvisionedThroughputInMibps. After you create the file system, you can    decrease your file system's throughput in Provisioned Throughput mode or change between    the throughput modes, with certain time restrictions. For more information, see Specifying     throughput with provisioned mode in the Amazon EFS User     Guide.
    /// 
    /// Default is bursting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: bursting | elastic | provisioned
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThroughputMode")]
    pub throughput_mode: Option<String>,


    /// 
    /// The ID of the AWS KMS key to be used to protect the encrypted file system. This    parameter is only required if you want to use a nondefault KMS key. If this parameter is not    specified, the default KMS key for Amazon EFS is used. This ID can be in one of the following    formats:
    /// 
    /// Key ID - A unique identifier of the key, for example       1234abcd-12ab-34cd-56ef-1234567890ab.               ARN - An Amazon Resource Name (ARN) for the key, for example       arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab.               Key alias - A previously created display name for a key, for example       alias/projectKey1.               Key alias ARN - An ARN for a key alias, for example       arn:aws:kms:us-west-2:444455556666:alias/projectKey1.
    /// 
    /// If KmsKeyId is specified, the Encrypted parameter must be set to true.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}|mrk-[0-9a-f]{32}|alias/[a-zA-Z0-9/_-]+|(arn:aws[-a-z]*:kms:[a-z0-9-]+:\d{12}:((key/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})|(key/mrk-[0-9a-f]{32})|(alias/[a-zA-Z0-9/_-]+))))$
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// Used to create a file system that uses One Zone storage classes. It specifies the AWS    Availability Zone in which to create the file system. Use the format us-east-1a    to specify the Availability Zone. For    more information about One Zone storage classes, see Using EFS storage classes in the Amazon EFS User Guide.
    /// 
    /// NoteOne Zone storage classes are not available in all Availability Zones in AWS Regions where     Amazon EFS is available.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .+
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZoneName")]
    pub availability_zone_name: Option<String>,


    /// 
    /// The FileSystemPolicy for the EFS file system. A file system policy is an IAM resource policy used to control NFS access to an EFS file system.        For more information, see Using IAM to control NFS access to Amazon EFS      in the Amazon EFS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20000
    ///
    /// Pattern: [\s\S]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemPolicy")]
    pub file_system_policy: Option<serde_json::Value>,

}


/// A tag is a key-value pair attached to a file system. Allowed characters in the Key and Value properties        are letters, white space, and numbers that    can be represented in UTF-8, and the following characters: + - = . _ : /
#[derive(Default, serde::Serialize)]
pub struct ElasticFileSystemTag {


    /// 
    /// The tag key (String). The key can't start with aws:.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^(?![aA]{1}[wW]{1}[sS]{1}:)([\p{L}\p{Z}\p{N}_.:/=+\-@]+)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value of the tag key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// Describes a policy used by EFS lifecycle management and EFS Intelligent-Tiering that    specifies when to transition files into and out of the file system's Infrequent Access (IA)    storage class. For more information, see EFS Intelligent‐Tiering and EFS Lifecycle     Management.
#[derive(Default, serde::Serialize)]
pub struct LifecyclePolicy {


    /// 
    /// Describes the period of time that a file is not accessed, after which it transitions to IA storage. Metadata    operations such as listing the contents of a directory don't count as file access    events.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AFTER_14_DAYS | AFTER_1_DAY | AFTER_30_DAYS | AFTER_60_DAYS | AFTER_7_DAYS | AFTER_90_DAYS
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionToIA")]
    pub transition_to_ia: Option<String>,


    /// 
    /// Describes when to transition a file from IA storage to primary storage. Metadata    operations such as listing the contents of a directory don't count as file access    events.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AFTER_1_ACCESS
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionToPrimaryStorageClass")]
    pub transition_to_primary_storage_class: Option<String>,

}


/// The backup policy turns automatic backups for the file system on or off.
#[derive(Default, serde::Serialize)]
pub struct BackupPolicy {


    /// 
    /// Set the backup policy status for the file system.
    /// 
    /// ENABLED - Turns automatic backups on for the file system.                                            DISABLED - Turns automatic backups off for the file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | DISABLING | ENABLED | ENABLING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,

}
