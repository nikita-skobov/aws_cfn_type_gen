

/// Specifies an Amazon Elastic Block Store (Amazon EBS) volume. You can attach the volume     to an instance in the same Availability Zone using AWS::EC2::VolumeAttachment.
///
/// When you use AWS CloudFormation to update an Amazon EBS volume that modifies       Iops, Size, or VolumeType, there is a cooldown     period before another operation can occur. This can cause your stack to report being in       UPDATE_IN_PROGRESS or UPDATE_ROLLBACK_IN_PROGRESS for long     periods of time.
///
/// Amazon EBS does not support sizing down an Amazon EBS volume. AWS CloudFormation     does not attempt to modify an Amazon EBS volume to a smaller size on rollback.
///
/// Some common scenarios when you might encounter a cooldown period for Amazon EBS     include:
///
/// For more information on the cooldown period, see Requirements when       modifying volumes.
///
/// DeletionPolicy attribute
///
/// To control how AWS CloudFormation handles the volume when the stack is deleted,     set a deletion policy for your volume. You can choose to retain the volume, to delete the     volume, or to create a snapshot of the volume. For more information, see DeletionPolicy attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVolume {


    /// 
    /// The tags to apply to the volume during creation.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The throughput to provision for a volume, with a maximum of 1,000 MiB/s.
    /// 
    /// This parameter is valid only for gp3 volumes. The default value is 125.
    /// 
    /// Valid Range: Minimum value of 125. Maximum value of 1000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Throughput")]
    pub throughput: Option<i64>,


    /// 
    /// The ID of the Availability Zone in which to create the volume. For example, us-east-1a.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,


    /// 
    /// The volume type. This parameter can be one of the following values:
    /// 
    /// General Purpose SSD: gp2 | gp3                       Provisioned IOPS SSD: io1 | io2                       Throughput Optimized HDD: st1                       Cold HDD: sc1                       Magnetic: standard
    /// 
    /// For more information, see Amazon EBS volume types in the    Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Default: gp2
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | io2 | sc1 | st1 | standard
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the Outpost.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,


    /// 
    /// Indicates whether the volume should be encrypted.    The effect of setting the encryption state to true depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled.    For more information, see Encryption by default    in the Amazon Elastic Compute Cloud User Guide.
    /// 
    /// Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption.    For more information, see Supported     instance types.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// Indicates whether the volume is auto-enabled for I/O operations. By default, Amazon EBS     disables I/O to the volume from attached EC2 instances when it determines that a volume's     data is potentially inconsistent. If the consistency of the volume is not a concern, and     you prefer that the volume be made available immediately if it's impaired, you can     configure the volume to automatically enable I/O.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoEnableIO")]
    pub auto_enable_io: Option<bool>,


    /// 
    /// The identifier of the AWS KMS key to use for Amazon EBS encryption. If       KmsKeyId is specified, the encrypted state must be     true.
    /// 
    /// If you omit this property and your account is enabled for encryption by default, or       Encrypted is set to true, then the volume     is encrypted using the default key specified for your account. If your account does not     have a default key, then the volume is encrypted using the AWS managed key.
    /// 
    /// Alternatively, if you want to specify a different key, you can specify one of the     following:
    /// 
    /// Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.            Key alias. Specify the alias for the key, prefixed with alias/. For        example, for a key with the alias my_cmk, use alias/my_cmk.        Or to specify the AWS managed key, use        alias/aws/ebs.            Key ARN. For example,        arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.            Alias ARN. For example,        arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// 
    /// Indicates whether Amazon EBS Multi-Attach is enabled.
    /// 
    /// AWS CloudFormation does not currently support updating a single-attach volume to     be multi-attach enabled, updating a multi-attach enabled volume to be single-attach, or     updating the size or number of I/O operations per second (IOPS) of a multi-attach enabled     volume.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiAttachEnabled")]
    pub multi_attach_enabled: Option<bool>,


    /// 
    /// The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size.    If you specify a snapshot, the default is the snapshot size. You can specify a volume    size that is equal to or larger than the snapshot size.
    /// 
    /// The following are the supported volumes sizes for each volume type:
    /// 
    /// gp2 and gp3: 1-16,384                        io1 and io2: 4-16,384                        st1 and sc1: 125-16,384                        standard: 1-1,024
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<i64>,


    /// 
    /// The number of I/O operations per second (IOPS). For gp3, io1, and io2 volumes, this represents    the number of IOPS that are provisioned for the volume. For gp2 volumes, this represents the baseline    performance of the volume and the rate at which the volume accumulates I/O credits for bursting.
    /// 
    /// The following are the supported values for each volume type:
    /// 
    /// gp3: 3,000-16,000 IOPS                        io1: 100-64,000 IOPS                        io2: 100-64,000 IOPS
    /// 
    /// io1 and io2 volumes support up to 64,000 IOPS only on     Instances built on the Nitro System. Other instance families support performance    up to 32,000 IOPS.
    /// 
    /// This parameter is required for io1 and io2 volumes.    The default for gp3 volumes is 3,000 IOPS.    This parameter is not supported for gp2, st1, sc1, or standard volumes.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,

}

impl cfn_resources::CfnResource for CfnVolume {
    fn type_string() -> &'static str {
        "AWS::EC2::Volume"
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
