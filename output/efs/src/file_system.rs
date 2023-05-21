/// The AWS::EFS::FileSystem resource creates a new, empty file system in     Amazon Elastic File System (Amazon EFS). You must create a mount target      (AWS::EFS::MountTarget) to mount your EFS file system on an      Amazon EC2 or other AWS cloud compute resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFileSystem {
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
    pub performance_mode: Option<FileSystemPerformanceModeEnum>,

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
    pub throughput_mode: Option<FileSystemThroughputModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FileSystemPerformanceModeEnum {
    /// generalPurpose
    #[serde(rename = "generalPurpose")]
    Generalpurpose,

    /// maxIO
    #[serde(rename = "maxIO")]
    Maxio,
}

impl Default for FileSystemPerformanceModeEnum {
    fn default() -> Self {
        FileSystemPerformanceModeEnum::Generalpurpose
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FileSystemThroughputModeEnum {
    /// bursting
    #[serde(rename = "bursting")]
    Bursting,

    /// elastic
    #[serde(rename = "elastic")]
    Elastic,

    /// provisioned
    #[serde(rename = "provisioned")]
    Provisioned,
}

impl Default for FileSystemThroughputModeEnum {
    fn default() -> Self {
        FileSystemThroughputModeEnum::Bursting
    }
}

impl cfn_resources::CfnResource for CfnFileSystem {
    fn type_string(&self) -> &'static str {
        "AWS::EFS::FileSystem"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.availability_zone_name {
            if the_val.len() > 64 as _ {
                return Err(format!("Max validation failed on field 'availability_zone_name'. {} is greater than 64", the_val.len()));
            }
        }

        if let Some(the_val) = &self.availability_zone_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'availability_zone_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.backup_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.kms_key_id {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.lifecycle_policies {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'lifecycle_policies'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The backup policy turns automatic backups for the file system on or off.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub status: BackupPolicyStatusEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum BackupPolicyStatusEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// DISABLING
    #[serde(rename = "DISABLING")]
    Disabling,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

    /// ENABLING
    #[serde(rename = "ENABLING")]
    Enabling,
}

impl Default for BackupPolicyStatusEnum {
    fn default() -> Self {
        BackupPolicyStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for BackupPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A tag is a key-value pair attached to a file system. Allowed characters in the Key and Value properties        are letters, white space, and numbers that    can be represented in UTF-8, and the following characters: + - = . _ : /
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for ElasticFileSystemTag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'key'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'key'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Describes a policy used by EFS lifecycle management and EFS Intelligent-Tiering that    specifies when to transition files into and out of the file system's Infrequent Access (IA)    storage class. For more information, see EFS Intelligent‐Tiering and EFS Lifecycle     Management.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub transition_to_ia: Option<LifecyclePolicyTransitionToIAEnum>,

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
    pub transition_to_primary_storage_class:
        Option<LifecyclePolicyTransitionToPrimaryStorageClassEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LifecyclePolicyTransitionToIAEnum {
    /// AFTER_14_DAYS
    #[serde(rename = "AFTER_14_DAYS")]
    After14days,

    /// AFTER_1_DAY
    #[serde(rename = "AFTER_1_DAY")]
    After1day,

    /// AFTER_30_DAYS
    #[serde(rename = "AFTER_30_DAYS")]
    After30days,

    /// AFTER_60_DAYS
    #[serde(rename = "AFTER_60_DAYS")]
    After60days,

    /// AFTER_7_DAYS
    #[serde(rename = "AFTER_7_DAYS")]
    After7days,

    /// AFTER_90_DAYS
    #[serde(rename = "AFTER_90_DAYS")]
    After90days,
}

impl Default for LifecyclePolicyTransitionToIAEnum {
    fn default() -> Self {
        LifecyclePolicyTransitionToIAEnum::After14days
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LifecyclePolicyTransitionToPrimaryStorageClassEnum {
    /// AFTER_1_ACCESS
    #[serde(rename = "AFTER_1_ACCESS")]
    After1access,
}

impl Default for LifecyclePolicyTransitionToPrimaryStorageClassEnum {
    fn default() -> Self {
        LifecyclePolicyTransitionToPrimaryStorageClassEnum::After1access
    }
}

impl cfn_resources::CfnResource for LifecyclePolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
