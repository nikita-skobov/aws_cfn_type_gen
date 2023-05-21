

/// Creates an FSx for ONTAP or Amazon FSx for OpenZFS storage volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVolume {


    /// 
    /// Specifies the ID of the volume backup to use to create a new volume.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupId")]
    pub backup_id: Option<String>,


    /// 
    /// The name of the volume.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 203
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,203}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The configuration of an Amazon FSx for NetApp ONTAP volume.
    /// 
    /// Required: No
    ///
    /// Type: OntapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OntapConfiguration")]
    pub ontap_configuration: Option<OntapConfiguration>,


    /// 
    /// The configuration of an Amazon FSx for OpenZFS volume.
    /// 
    /// Required: No
    ///
    /// Type: OpenZFSConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenZFSConfiguration")]
    pub open_zfsconfiguration: Option<OpenZFSConfiguration>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The type of the volume.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ONTAP | OPENZFS
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<VolumeVolumeTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum VolumeVolumeTypeEnum {

    /// ONTAP
    #[serde(rename = "ONTAP")]
    Ontap,

    /// OPENZFS
    #[serde(rename = "OPENZFS")]
    Openzfs,

}

impl Default for VolumeVolumeTypeEnum {
    fn default() -> Self {
        VolumeVolumeTypeEnum::Ontap
    }
}


impl cfn_resources::CfnResource for CfnVolume {
    fn type_string() -> &'static str {
        "AWS::FSx::Volume"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies who can mount an OpenZFS file system and the options available while       mounting the file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientConfigurations {


    /// 
    /// A value that specifies who can mount the file system. You can provide a wildcard       character (*), an IP address (0.0.0.0), or a CIDR address         (192.0.2.0/24). By default, Amazon FSx uses the wildcard       character when specifying the client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[ -~]{1,128}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Clients")]
    pub clients: String,


    /// 
    /// The options to use when mounting the file system. For a list of options that you can       use with Network File System (NFS), see the exports(5) - Linux man page. When       choosing your options, consider the following:
    /// 
    /// crossmnt is used by default. If you don't specify           crossmnt when changing the client configuration, you won't be           able to see or access snapshots in your file system's snapshot directory.                        sync is used by default. If you instead specify             async, the system acknowledges writes before writing to disk.           If the system crashes before the writes are finished, you lose the unwritten           data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    pub options: Vec<String>,

}




/// The configuration object for mounting a Network File System (NFS) file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NfsExports {


    /// 
    /// A list of configuration objects that contain the client and options for mounting the       OpenZFS file system.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ClientConfigurations
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientConfigurations")]
    pub client_configurations: Vec<ClientConfigurations>,

}




/// Specifies the configuration of the ONTAP volume that you are creating.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OntapConfiguration {


    /// 
    /// A boolean flag indicating whether tags for the volume should be copied to backups. This value defaults to       false. If it's set to true, all tags for the volume are copied to all automatic and user-initiated backups       where the user doesn't specify tags. If this value is true, and you specify one or more tags, only the       specified tags are copied to backups. If you specify one or more tags when creating a user-initiated       backup, no tags are copied from the volume, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<String>,


    /// 
    /// Specifies the location in the SVM's namespace where the volume is mounted.       This parameter is required. The JunctionPath must have a leading       forward slash, such as /vol3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,255}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "JunctionPath")]
    pub junction_path: Option<String>,


    /// 
    /// Specifies the type of volume you are creating. Valid values are the following:
    /// 
    /// RW specifies a read/write volume. RW is the default.                        DP specifies a data-protection volume. A DP volume         is read-only and can be used as the destination of a NetApp SnapMirror relationship.
    /// 
    /// For more information, see Volume types       in the Amazon FSx for NetApp ONTAP User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DP | RW
    ///
    /// Update requires: Replacement
    #[serde(rename = "OntapVolumeType")]
    pub ontap_volume_type: Option<OntapConfigurationOntapVolumeTypeEnum>,


    /// 
    /// Specifies the security style for the volume. If a volume's security style is not specified,       it is automatically set to the root volume's security style. The security style determines the type of permissions       that FSx for ONTAP uses to control data access. For more information, see       Volume security style       in the Amazon FSx for NetApp ONTAP User Guide.       Specify one of the following values:
    /// 
    /// UNIX if the file system is managed by a UNIX         administrator, the majority of users are NFS clients, and an application         accessing the data uses a UNIX user as the service account.                                NTFS if the file system is managed by a Windows         administrator, the majority of users are SMB clients, and an application         accessing the data uses a Windows user as the service account.                        MIXED if the file system is managed by both UNIX         and Windows administrators and users consist of both NFS and SMB clients.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MIXED | NTFS | UNIX
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityStyle")]
    pub security_style: Option<OntapConfigurationSecurityStyleEnum>,


    /// 
    /// Specifies the size of the volume, in megabytes (MB), that you are creating.       Provide any whole number in the range of 20–104857600 to specify the size of       the volume.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInMegabytes")]
    pub size_in_megabytes: String,


    /// 
    /// Specifies the snapshot policy for the volume. There are three built-in snapshot policies:
    /// 
    /// default: This is the default policy. A maximum of six hourly snapshots taken five minutes past       the hour. A maximum of two daily snapshots taken Monday through Saturday at 10 minutes after       midnight. A maximum of two weekly snapshots taken every Sunday at 15 minutes after midnight.                        default-1weekly: This policy is the same as the default policy except       that it only retains one snapshot from the weekly schedule.                        none: This policy does not take any snapshots. This policy can be assigned to volumes to       prevent automatic snapshots from being taken.
    /// 
    /// You can also provide the name of a custom policy that you created with the ONTAP CLI or REST API.
    /// 
    /// For more information, see Snapshot policies       in the Amazon FSx for NetApp ONTAP User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotPolicy")]
    pub snapshot_policy: Option<String>,


    /// 
    /// Set to true to enable deduplication, compression, and compaction storage       efficiency features on the volume, or set to false to disable them.       This parameter is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageEfficiencyEnabled")]
    pub storage_efficiency_enabled: Option<String>,


    /// 
    /// Specifies the ONTAP SVM in which to create the volume.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 21
    ///
    /// Maximum: 21
    ///
    /// Pattern: ^(svm-[0-9a-f]{17,})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageVirtualMachineId")]
    pub storage_virtual_machine_id: String,


    /// 
    /// Describes the data tiering policy for an ONTAP volume. When enabled, Amazon FSx for ONTAP's intelligent       tiering automatically transitions a volume's data between the file system's primary storage and capacity       pool storage based on your access patterns.
    /// 
    /// Valid tiering policies are the following:
    /// 
    /// SNAPSHOT_ONLY - (Default value) moves cold snapshots to the capacity pool storage tier.
    /// 
    /// AUTO - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.
    /// 
    /// ALL - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.
    /// 
    /// NONE - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.
    /// 
    /// Required: No
    ///
    /// Type: TieringPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "TieringPolicy")]
    pub tiering_policy: Option<TieringPolicy>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OntapConfigurationSecurityStyleEnum {

    /// MIXED
    #[serde(rename = "MIXED")]
    Mixed,

    /// NTFS
    #[serde(rename = "NTFS")]
    Ntfs,

    /// UNIX
    #[serde(rename = "UNIX")]
    Unix,

}

impl Default for OntapConfigurationSecurityStyleEnum {
    fn default() -> Self {
        OntapConfigurationSecurityStyleEnum::Mixed
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OntapConfigurationOntapVolumeTypeEnum {

    /// DP
    #[serde(rename = "DP")]
    Dp,

    /// RW
    #[serde(rename = "RW")]
    Rw,

}

impl Default for OntapConfigurationOntapVolumeTypeEnum {
    fn default() -> Self {
        OntapConfigurationOntapVolumeTypeEnum::Dp
    }
}



/// Specifies the configuration of the Amazon FSx for OpenZFS volume that you are creating.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenZFSConfiguration {


    /// 
    /// A Boolean value indicating whether tags for the volume should be copied to snapshots.       This value defaults to false. If it's set to true, all tags       for the volume are copied to snapshots where the user doesn't specify tags. If this       value is true, and you specify one or more tags, only the specified tags       are copied to snapshots. If you specify one or more tags when creating the snapshot, no       tags are copied from the volume, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToSnapshots")]
    pub copy_tags_to_snapshots: Option<bool>,


    /// 
    /// Specifies the method used to compress the data on the volume. The compression       type is NONE by default.
    /// 
    /// NONE - Doesn't compress the data on the volume.           NONE is the default.                        ZSTD - Compresses the data in the volume using the Zstandard           (ZSTD) compression algorithm. Compared to LZ4, Z-Standard provides a better           compression ratio to minimize on-disk storage utilization.                        LZ4 - Compresses the data in the volume using the LZ4           compression algorithm. Compared to Z-Standard, LZ4 is less compute-intensive           and delivers higher write throughput speeds.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LZ4 | NONE | ZSTD
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<OpenZFSConfigurationDataCompressionTypeEnum>,


    /// 
    /// The configuration object for mounting a Network File System (NFS) file system.
    /// 
    /// Required: No
    ///
    /// Type: List of NfsExports
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "NfsExports")]
    pub nfs_exports: Option<Vec<NfsExports>>,


    /// 
    /// To delete the volume's child volumes, snapshots, and clones, use the string        DELETE_CHILD_VOLUMES_AND_SNAPSHOTS.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,


    /// 
    /// The configuration object that specifies the snapshot to use as the origin of the data       for the volume.
    /// 
    /// Required: No
    ///
    /// Type: OriginSnapshot
    ///
    /// Update requires: Replacement
    #[serde(rename = "OriginSnapshot")]
    pub origin_snapshot: Option<OriginSnapshot>,


    /// 
    /// The ID of the volume to use as the parent volume of the volume that you are creating.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 23
    ///
    /// Maximum: 23
    ///
    /// Pattern: ^(fsvol-[0-9a-f]{17,})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentVolumeId")]
    pub parent_volume_id: String,


    /// 
    /// A Boolean value indicating whether the volume is read-only.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,


    /// 
    /// Specifies the suggested block size for a volume in a ZFS dataset, in kibibytes (KiB). Valid values are 4, 8,       16, 32, 64, 128, 256, 512, or 1024 KiB. The default is 128 KiB.       We recommend using the default setting for the majority of use cases.       Generally, workloads that write in fixed small or large record sizes       may benefit from setting a custom record size, like database workloads       (small record size) or media streaming workloads (large record size).       For additional guidance on when       to set a custom record size, see              ZFS Record size in the Amazon FSx for OpenZFS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 4
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordSizeKiB")]
    pub record_size_ki_b: Option<i64>,


    /// 
    /// Sets the maximum storage size in gibibytes (GiB) for the volume. You can specify       a quota that is larger than the storage on the parent volume. A volume quota limits       the amount of storage that the volume can consume to the configured amount, but does not       guarantee the space will be available on the parent volume. To guarantee quota space, you must also set       StorageCapacityReservationGiB. To not specify a storage capacity quota, set this to -1.
    /// 
    /// For more information, see       Volume properties       in the Amazon FSx for OpenZFS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: -1
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: Option<i64>,


    /// 
    /// Specifies the amount of storage in gibibytes (GiB) to reserve from the parent volume. Setting       StorageCapacityReservationGiB guarantees that the specified amount of storage space       on the parent volume will always be available for the volume.       You can't reserve more storage than the parent volume has. To not specify a storage capacity       reservation, set this to 0 or -1. For more information, see       Volume properties       in the Amazon FSx for OpenZFS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: -1
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageCapacityReservationGiB")]
    pub storage_capacity_reservation_gi_b: Option<i64>,


    /// 
    /// An object specifying how much storage users or groups can use on the volume.
    /// 
    /// Required: No
    ///
    /// Type: List of UserAndGroupQuotas
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserAndGroupQuotas")]
    pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OpenZFSConfigurationDataCompressionTypeEnum {

    /// LZ4
    #[serde(rename = "LZ4")]
    Lz4,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// ZSTD
    #[serde(rename = "ZSTD")]
    Zstd,

}

impl Default for OpenZFSConfigurationDataCompressionTypeEnum {
    fn default() -> Self {
        OpenZFSConfigurationDataCompressionTypeEnum::Lz4
    }
}



/// The configuration object that specifies the snapshot to use as the origin of the data       for the volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OriginSnapshot {


    /// 
    /// The strategy used when copying data from the snapshot to the new volume.
    /// 
    /// CLONE - The new volume references the data in the origin           snapshot. Cloning a snapshot is faster than copying data from the snapshot to a           new volume and doesn't consume disk throughput. However, the origin snapshot           can't be deleted if there is a volume using its copied data.                         FULL_COPY - Copies all data from the snapshot to the new volume.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CLONE | FULL_COPY
    ///
    /// Update requires: Replacement
    #[serde(rename = "CopyStrategy")]
    pub copy_strategy: OriginSnapshotCopyStrategyEnum,


    /// 
    /// Specifies the snapshot to use when creating an OpenZFS volume from a snapshot.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotARN")]
    pub snapshot_arn: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OriginSnapshotCopyStrategyEnum {

    /// CLONE
    #[serde(rename = "CLONE")]
    Clone,

    /// FULL_COPY
    #[serde(rename = "FULL_COPY")]
    Fullcopy,

}

impl Default for OriginSnapshotCopyStrategyEnum {
    fn default() -> Self {
        OriginSnapshotCopyStrategyEnum::Clone
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




/// Describes the data tiering policy for an ONTAP volume. When enabled, Amazon FSx for ONTAP's intelligent       tiering automatically transitions a volume's data between the file system's primary storage and capacity       pool storage based on your access patterns.
///
/// Valid tiering policies are the following:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TieringPolicy {


    /// 
    /// Specifies the number of days that user data in a volume must remain inactive before it is considered "cold"       and moved to the capacity pool. Used with the AUTO and SNAPSHOT_ONLY tiering policies.       Enter a whole number between 2 and 183. Default values are 31 days for AUTO and 2 days for       SNAPSHOT_ONLY.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 183
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoolingPeriod")]
    pub cooling_period: Option<i64>,


    /// 
    /// Specifies the tiering policy used to transition data. Default value is SNAPSHOT_ONLY.
    /// 
    /// SNAPSHOT_ONLY - moves cold snapshots to the capacity pool storage tier.                        AUTO - moves cold user data and snapshots to the capacity pool storage tier         based on your access patterns.                        ALL - moves all user data blocks in both the active file system and Snapshot copies to the         storage pool tier.                        NONE - keeps a volume's data in the primary storage tier, preventing it from being moved to         the capacity pool tier.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | AUTO | NONE | SNAPSHOT_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<TieringPolicyNameEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TieringPolicyNameEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// AUTO
    #[serde(rename = "AUTO")]
    Auto,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SNAPSHOT_ONLY
    #[serde(rename = "SNAPSHOT_ONLY")]
    Snapshotonly,

}

impl Default for TieringPolicyNameEnum {
    fn default() -> Self {
        TieringPolicyNameEnum::All
    }
}



/// An object specifying how much storage users or groups can use on the volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserAndGroupQuotas {


    /// 
    /// The ID of the user or group.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: i64,


    /// 
    /// The amount of storage that the user or group can use in gibibytes (GiB).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: i64,


    /// 
    /// A value that specifies whether the quota applies to a user or group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GROUP | USER
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: UserAndGroupQuotasTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum UserAndGroupQuotasTypeEnum {

    /// GROUP
    #[serde(rename = "GROUP")]
    Group,

    /// USER
    #[serde(rename = "USER")]
    User,

}

impl Default for UserAndGroupQuotasTypeEnum {
    fn default() -> Self {
        UserAndGroupQuotasTypeEnum::Group
    }
}

