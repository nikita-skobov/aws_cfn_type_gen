

/// The AWS::FSx::FileSystem resource is an Amazon FSx resource type       that specifies an Amazon FSx file system. You can create any of the following       supported file system types:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFileSystem {


    /// 
    /// Sets the storage type for the file system that you're creating. Valid values are         SSD and HDD.
    /// 
    /// Set to SSD to use solid state drive storage. SSD is supported on all Windows,           Lustre, ONTAP, and OpenZFS deployment types.               Set to HDD to use hard disk drive storage.         HDD is supported on SINGLE_AZ_2 and MULTI_AZ_1 Windows file system deployment types,         and on PERSISTENT_1 Lustre file system deployment types.
    /// 
    /// Default value is SSD. For more information, see Storage         type options in the FSx for Windows File Server User         Guide and Multiple storage         options in the FSx for Lustre User       Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HDD | SSD
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageType")]
    pub storage_type: Option<FileSystemStorageTypeEnum>,


    /// 
    /// A list of IDs specifying the security groups to apply to all network interfaces       created for file system access. This list isn't returned in later requests to       describe the file system.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The Amazon FSx for OpenZFS configuration properties for the file system that you are creating.
    /// 
    /// Required: No
    ///
    /// Type: OpenZFSConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenZFSConfiguration")]
    pub open_zfsconfiguration: Option<OpenZFSConfiguration>,


    /// 
    /// The ID of the AWS Key Management Service (AWS KMS) key used to encrypt Amazon FSx file       system data. Used as follows with Amazon FSx file system types:
    /// 
    /// Amazon FSx for Lustre PERSISTENT_1        and PERSISTENT_2 deployment types only.                  SCRATCH_1 and SCRATCH_2 types are encrypted using           the Amazon FSx service AWS KMS key for your account.               Amazon FSx for NetApp ONTAP               Amazon FSx for OpenZFS               Amazon FSx for Windows File Server
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The ID of the file system backup that you are using to create a file system. For more information,       see CreateFileSystemFromBackup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupId")]
    pub backup_id: Option<String>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
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
    /// The Lustre configuration for the file system being created.
    /// 
    /// NoteThe following parameters are not supported for file systems with the Lustre Persistent_2       deployment type.                                                               AutoImportPolicy                                      ExportPath                                      ImportedChunkSize                                      ImportPath
    /// 
    /// Required: Conditional
    ///
    /// Type: LustreConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LustreConfiguration")]
    pub lustre_configuration: Option<LustreConfiguration>,


    /// 
    /// The ONTAP configuration properties of the FSx for ONTAP file system that you       are creating.
    /// 
    /// Required: No
    ///
    /// Type: OntapConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OntapConfiguration")]
    pub ontap_configuration: Option<OntapConfiguration>,


    /// 
    /// (Optional) For FSx for Lustre file systems, sets the Lustre version for the       file system that you're creating. Valid values are 2.10 and         2.12:
    /// 
    /// 2.10 is supported by the Scratch and Persistent_1 Lustre deployment types.               2.12 is supported by all Lustre deployment types. 2.12 is         required when setting FSx for Lustre DeploymentType to         PERSISTENT_2.
    /// 
    /// Default value = 2.10, except when DeploymentType is set to       PERSISTENT_2, then the default is 2.12.
    /// 
    /// NoteIf you set FileSystemTypeVersion to 2.10 for a        PERSISTENT_2 Lustre deployment type, the CreateFileSystem       operation fails.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^[0-9](.[0-9]*)*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemTypeVersion")]
    pub file_system_type_version: Option<String>,


    /// 
    /// The type of Amazon FSx file system, which can be LUSTRE,         WINDOWS, ONTAP, or OPENZFS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,


    /// 
    /// The configuration object for the Microsoft Windows file system you are creating. This       value is required if FileSystemType is set to WINDOWS.
    /// 
    /// Required: Conditional
    ///
    /// Type: WindowsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "WindowsConfiguration")]
    pub windows_configuration: Option<WindowsConfiguration>,


    /// 
    /// Sets the storage capacity of the file system that you're creating.         StorageCapacity is required if you are creating a new file system.
    /// 
    /// FSx for Lustre file systems - The amount of       storage capacity that you can configure depends on the value that you set for         StorageType and the Lustre DeploymentType, as       follows:
    /// 
    /// For SCRATCH_2, PERSISTENT_2 and             PERSISTENT_1 deployment types using SSD storage type, the valid           values are 1200 GiB, 2400 GiB, and increments of 2400 GiB.               For PERSISTENT_1 HDD file systems, valid values are increments of           6000 GiB for 12 MB/s/TiB file systems and increments of 1800 GiB for 40 MB/s/TiB           file systems.               For SCRATCH_1 deployment type, valid values are 1200 GiB, 2400           GiB, and increments of 3600 GiB.
    /// 
    /// FSx for ONTAP file systems - The amount of       storage capacity that you can configure is from 1024 GiB up to 196,608 GiB (192       TiB).
    /// 
    /// FSx for OpenZFS file systems - The amount of storage       capacity that you can configure is from 64 GiB up to 524,288 GiB (512 TiB). If you are creating a     file system from a backup, you can specify a storage capacity equal to or greater than the original     file system's storage capacity.
    /// 
    /// FSx for Windows File Server file systems - The amount       of storage capacity that you can configure depends on the value that you set for         StorageType as follows:
    /// 
    /// For SSD storage, valid values are 32 GiB-65,536 GiB (64 TiB).               For HDD storage, valid values are 2000 GiB-65,536 GiB (64 TiB).
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageCapacity")]
    pub storage_capacity: Option<i64>,


    /// 
    /// Specifies the IDs of the subnets that the file system will be accessible from. For       Windows and ONTAP MULTI_AZ_1 deployment types,provide exactly two subnet       IDs, one for the preferred file server and one for the standby file server. You specify       one of these subnets as the preferred subnet using the WindowsConfiguration >         PreferredSubnetID or OntapConfiguration > PreferredSubnetID       properties. For more information about Multi-AZ file system configuration, see         Availability and durability: Single-AZ and Multi-AZ file systems in the         Amazon FSx for Windows User Guide and         Availability and durability in the Amazon FSx for ONTAP User         Guide.
    /// 
    /// For Windows SINGLE_AZ_1 and SINGLE_AZ_2 and all Lustre       deployment types, provide exactly one subnet ID.      The file server is launched in that subnet's Availability Zone.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FileSystemStorageTypeEnum {

    /// HDD
    #[serde(rename = "HDD")]
    Hdd,

    /// SSD
    #[serde(rename = "SSD")]
    Ssd,

}

impl Default for FileSystemStorageTypeEnum {
    fn default() -> Self {
        FileSystemStorageTypeEnum::Hdd
    }
}


impl cfn_resources::CfnResource for CfnFileSystem {
    fn type_string() -> &'static str {
        "AWS::FSx::FileSystem"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The SSD IOPS (input/output operations per second) configuration for an Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS file system. The       default is 3 IOPS per GB of storage capacity, but you can provision additional IOPS per       GB of storage. The configuration consists of the total number of provisioned SSD IOPS       and how the amount was provisioned (by the customer or by the system).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DiskIopsConfiguration {


    /// 
    /// Specifies whether the number of IOPS for the file system is       using the system default (AUTOMATIC) or was       provisioned by the customer (USER_PROVISIONED).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | USER_PROVISIONED
    ///
    /// Update requires: Replacement
    #[serde(rename = "Mode")]
    pub mode: Option<DiskIopsConfigurationModeEnum>,


    /// 
    /// The total number of SSD IOPS provisioned for the file system.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DiskIopsConfigurationModeEnum {

    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// USER_PROVISIONED
    #[serde(rename = "USER_PROVISIONED")]
    Userprovisioned,

}

impl Default for DiskIopsConfigurationModeEnum {
    fn default() -> Self {
        DiskIopsConfigurationModeEnum::Automatic
    }
}



/// The Microsoft Windows configuration for the file system that's being created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WindowsConfiguration {


    /// 
    /// A boolean flag indicating whether tags for the file system should be copied to       backups. This value defaults to false. If it's set to true, all tags for the file       system are copied to all automatic and user-initiated backups where the user       doesn't specify tags. If this value is true, and you specify one or more tags, only       the specified tags are copied to backups. If you specify one or more tags when creating a       user-initiated backup, no tags are copied from the file system, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,


    /// 
    /// An array of one or more DNS alias names that you want to associate with the Amazon FSx       file system. Aliases allow you to use existing DNS names to access the data in your       Amazon FSx file system. You can associate up to 50 aliases with a file system at any       time.
    /// 
    /// For more information, see Working with DNS         Aliases and Walkthrough 5: Using DNS aliases to access your file system, including       additional steps you must take to be able to access your file system using a DNS       alias.
    /// 
    /// An alias name has to meet the following requirements:
    /// 
    /// Formatted as a fully-qualified domain name (FQDN),             hostname.domain, for example,             accounting.example.com.               Can contain alphanumeric characters, the underscore (_), and the hyphen           (-).               Cannot start or end with a hyphen.               Can start with a numeric.
    /// 
    /// For DNS alias names, Amazon FSx stores alphabetical characters as lowercase letters       (a-z), regardless of how you specify them: as uppercase letters, lowercase letters, or       the corresponding letters in escape codes.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,


    /// 
    /// Sets the throughput capacity of an Amazon FSx file system, measured in megabytes per       second (MB/s), in 2 to the nth increments, between 2^3 (8) and 2^11       (2048).
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 8
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: i64,


    /// 
    /// The configuration that Amazon FSx for Windows File Server uses to audit and log       user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server       file system.
    /// 
    /// Required: No
    ///
    /// Type: AuditLogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditLogConfiguration")]
    pub audit_log_configuration: Option<AuditLogConfiguration>,


    /// 
    /// The ID for an existing AWS Managed Microsoft Active Directory (AD)       instance that the file system should join when it's created. Required if you are joining the file system to an existing     AWS Managed Microsoft AD.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^d-[0-9a-f]{10}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ActiveDirectoryId")]
    pub active_directory_id: Option<String>,


    /// 
    /// A recurring daily time, in the format HH:MM. HH is the       zero-padded hour of the day (0-23), and MM is the zero-padded minute of the       hour. For example, 05:00 specifies 5 AM daily.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,


    /// 
    /// The number of days to retain automatic backups. Setting this property to         0 disables automatic backups. You can retain automatic backups for a       maximum of 90 days. The default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<i64>,


    /// 
    /// The configuration that Amazon FSx uses to join a FSx for Windows File Server file system or an ONTAP storage virtual machine (SVM) to       a self-managed (including on-premises) Microsoft Active Directory (AD)       directory. For more information, see                Using Amazon FSx with your self-managed Microsoft Active Directory or       Managing SVMs.
    /// 
    /// Required: No
    ///
    /// Type: SelfManagedActiveDirectoryConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryConfiguration>,


    /// 
    /// A recurring weekly time, in the format D:HH:MM.
    /// 
    /// D is the day of the week, for which 1 represents Monday and 7       represents Sunday. For further details, see the ISO-8601 spec as described on Wikipedia.
    /// 
    /// HH is the zero-padded hour of the day (0-23), and MM is       the zero-padded minute of the hour.
    /// 
    /// For example, 1:05:00 specifies maintenance at 5 AM Monday.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,


    /// 
    /// Specifies the file system deployment type, valid values are the following:
    /// 
    /// MULTI_AZ_1 - Deploys a high availability file system that is configured           for Multi-AZ redundancy to tolerate temporary Availability Zone (AZ) unavailability. You           can only deploy a Multi-AZ file system in AWS Regions that have a minimum of three Availability Zones. Also         supports HDD storage type                        SINGLE_AZ_1 - (Default) Choose to deploy a file system that is configured for single AZ redundancy.                        SINGLE_AZ_2 - The latest generation Single AZ file system.           Specifies a file system that is configured for single AZ redundancy and supports HDD storage type.
    /// 
    /// For more information, see                Availability and Durability: Single-AZ and Multi-AZ File Systems.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_AZ_1 | SINGLE_AZ_1 | SINGLE_AZ_2
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<WindowsConfigurationDeploymentTypeEnum>,


    /// 
    /// Required when DeploymentType is set to MULTI_AZ_1. This       specifies the subnet in which you want the preferred file server to be located. For         in-AWS applications, we recommend that you launch your clients in the       same availability zone as your preferred file server to reduce cross-availability zone data       transfer costs and minimize latency.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreferredSubnetId")]
    pub preferred_subnet_id: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum WindowsConfigurationDeploymentTypeEnum {

    /// MULTI_AZ_1
    #[serde(rename = "MULTI_AZ_1")]
    Multiaz1,

    /// SINGLE_AZ_1
    #[serde(rename = "SINGLE_AZ_1")]
    Singleaz1,

    /// SINGLE_AZ_2
    #[serde(rename = "SINGLE_AZ_2")]
    Singleaz2,

}

impl Default for WindowsConfigurationDeploymentTypeEnum {
    fn default() -> Self {
        WindowsConfigurationDeploymentTypeEnum::Multiaz1
    }
}



/// The configuration for the Amazon FSx for Lustre file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LustreConfiguration {


    /// 
    /// A recurring weekly time, in the format D:HH:MM.
    /// 
    /// D is the day of the week, for which 1 represents Monday and 7       represents Sunday. For further details, see the ISO-8601 spec as described on Wikipedia.
    /// 
    /// HH is the zero-padded hour of the day (0-23), and MM is       the zero-padded minute of the hour.
    /// 
    /// For example, 1:05:00 specifies maintenance at 5 AM Monday.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,


    /// 
    /// (Optional) Available with Scratch and Persistent_1 deployment types. When you       create your file system, your existing S3 objects appear as file and directory listings.       Use this property to choose how Amazon FSx keeps your file and directory listings up to date       as you add or modify objects in your linked S3 bucket. AutoImportPolicy can       have the following values:
    /// 
    /// NONE - (Default) AutoImport is off. Amazon FSx only updates         file and directory listings from the linked S3 bucket         when the file system is created. FSx does not update file and directory         listings for any new or changed objects after choosing this option.                        NEW - AutoImport is on. Amazon FSx automatically imports         directory listings of any new objects added to the linked S3 bucket that         do not currently exist in the FSx file system.                         NEW_CHANGED - AutoImport is on. Amazon FSx automatically imports         file and directory listings of any new objects added to the S3 bucket and any         existing objects that are changed in the S3 bucket after you choose this option.                        NEW_CHANGED_DELETED - AutoImport is on. Amazon FSx automatically         imports file and directory listings of any new objects added to the S3 bucket, any         existing objects that are changed in the S3 bucket, and any objects that were deleted         in the S3 bucket.
    /// 
    /// For more information, see       Automatically import updates from your S3 bucket.
    /// 
    /// ImportantThis parameter is not supported for Lustre file systems using the Persistent_2 deployment type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NEW | NEW_CHANGED | NEW_CHANGED_DELETED | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoImportPolicy")]
    pub auto_import_policy: Option<LustreConfigurationAutoImportPolicyEnum>,


    /// 
    /// (Optional) Choose SCRATCH_1 and SCRATCH_2 deployment       types when you need temporary storage and shorter-term processing of data.       The SCRATCH_2 deployment type provides in-transit encryption of data and higher burst       throughput capacity than SCRATCH_1.
    /// 
    /// Choose PERSISTENT_1 for longer-term storage and for throughput-focused       workloads that aren’t latency-sensitive.       PERSISTENT_1 supports encryption of data in transit, and is available in all       AWS Regions in which FSx for Lustre is available.
    /// 
    /// Choose PERSISTENT_2 for longer-term storage and for latency-sensitive workloads       that require the highest levels of IOPS/throughput. PERSISTENT_2 supports       SSD storage, and offers higher PerUnitStorageThroughput (up to 1000 MB/s/TiB). PERSISTENT_2       is available in a limited number of AWS Regions.       For more information, and an up-to-date list of AWS Regions in which       PERSISTENT_2 is available, see       File         system deployment options for FSx for Lustre in the Amazon FSx for Lustre User Guide.
    /// 
    /// NoteIf you choose PERSISTENT_2, and you set FileSystemTypeVersion to           2.10, the CreateFileSystem operation fails.
    /// 
    /// Encryption of data in transit is automatically turned on when you access         SCRATCH_2, PERSISTENT_1 and PERSISTENT_2 file         systems from Amazon EC2 instances that support automatic encryption in         the AWS Regions where they are available. For more information about         encryption in transit for FSx for Lustre file systems, see Encrypting data in         transit in the Amazon FSx for Lustre User Guide.
    /// 
    /// (Default = SCRATCH_1)
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: PERSISTENT_1 | PERSISTENT_2 | SCRATCH_1 | SCRATCH_2
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<LustreConfigurationDeploymentTypeEnum>,


    /// 
    /// (Optional) Available with Scratch and Persistent_1       deployment types. Specifies the path in the Amazon S3 bucket where the root of your       Amazon FSx file system is exported. The path must use the same Amazon S3 bucket as       specified in ImportPath. You can provide an optional prefix to which new and changed       data is to be exported from your Amazon FSx for Lustre file system. If an         ExportPath value is not provided, Amazon FSx sets a default export       path, s3://import-bucket/FSxLustre[creation-timestamp]. The timestamp is in       UTC format, for example       s3://import-bucket/FSxLustre20181105T222312Z.
    /// 
    /// The Amazon S3 export bucket must be the same as the import bucket specified by         ImportPath. If you specify only a bucket name, such as         s3://import-bucket, you get a 1:1 mapping of file system objects to S3       bucket objects. This mapping means that the input data in S3 is overwritten on export.       If you provide a custom prefix in the export path, such as         s3://import-bucket/[custom-optional-prefix], Amazon FSx exports the       contents of your file system to that export prefix in the Amazon S3 bucket.
    /// 
    /// ImportantThis parameter is not supported for file systems using the           Persistent_2 deployment type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 4357
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{3,4357}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExportPath")]
    pub export_path: Option<String>,


    /// 
    /// A Boolean flag indicating whether tags for the file system should be copied to       backups. This value defaults to false. If it's set to true, all tags for the file system       are copied to all automatic and user-initiated backups where the user doesn't specify       tags. If this value is true, and you specify one or more tags, only the specified tags       are copied to backups. If you specify one or more tags when creating a user-initiated       backup, no tags are copied from the file system, regardless of this value. Only valid       for use with PERSISTENT_1 deployment types.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,


    /// 
    /// (Optional) For files imported from a data repository, this value determines the stripe       count and maximum amount of data per file (in MiB) stored on a single physical disk. The       maximum number of disks that a single file can be striped across is limited by the total       number of disks that make up the file system.
    /// 
    /// The default chunk size is 1,024 MiB (1 GiB) and can go as high as 512,000 MiB (500       GiB). Amazon S3 objects have a maximum size of 5 TB.
    /// 
    /// ImportantThis parameter is not supported for Lustre file systems using the Persistent_2       deployment type.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512000
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImportedFileChunkSize")]
    pub imported_file_chunk_size: Option<i64>,


    /// 
    /// A recurring daily time, in the format HH:MM. HH is the       zero-padded hour of the day (0-23), and MM is the zero-padded minute of the       hour. For example, 05:00 specifies 5 AM daily.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,


    /// 
    /// Sets the data compression configuration for the file system. DataCompressionType       can have the following values:
    /// 
    /// NONE - (Default) Data compression is turned off when         the file system is created.                        LZ4 - Data compression is turned on with the LZ4         algorithm.
    /// 
    /// For more information, see Lustre data compression       in the Amazon FSx for Lustre User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LZ4 | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<LustreConfigurationDataCompressionTypeEnum>,


    /// 
    /// (Optional) The path to the Amazon S3 bucket (including the optional prefix) that       you're using as the data repository for your Amazon FSx for Lustre file system. The root       of your FSx for Lustre file system will be mapped to the root of the Amazon S3 bucket       you select. An example is s3://import-bucket/optional-prefix. If you       specify a prefix after the Amazon S3 bucket name, only object keys with that prefix are       loaded into the file system.
    /// 
    /// ImportantThis parameter is not supported for Lustre file systems using the Persistent_2       deployment type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 4357
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{3,4357}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImportPath")]
    pub import_path: Option<String>,


    /// 
    /// Required with PERSISTENT_1 and PERSISTENT_2 deployment       types, provisions the amount of read and write throughput for each 1 tebibyte (TiB) of       file system storage capacity, in MB/s/TiB. File system throughput capacity is calculated       by multiplying ﬁle system storage capacity (TiB) by the         PerUnitStorageThroughput (MB/s/TiB). For a 2.4-TiB ﬁle system,       provisioning 50 MB/s/TiB of PerUnitStorageThroughput yields 120 MB/s of ﬁle       system throughput. You pay for the amount of throughput that you provision.
    /// 
    /// Valid values:
    /// 
    /// For PERSISTENT_1 SSD storage: 50, 100, 200 MB/s/TiB.               For PERSISTENT_1 HDD storage: 12, 40 MB/s/TiB.               For PERSISTENT_2 SSD storage: 125, 250, 500, 1000           MB/s/TiB.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 12
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "PerUnitStorageThroughput")]
    pub per_unit_storage_throughput: Option<i64>,


    /// 
    /// The type of drive cache used by PERSISTENT_1 file systems that are provisioned with       HDD storage devices. This parameter is required when storage type is HDD. Set this property to       READ to improve the performance for frequently accessed files by caching up to 20%       of the total storage capacity of the file system.
    /// 
    /// This parameter is required when StorageType is set to HDD and DeploymentType is       PERSISTENT_1.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | READ
    ///
    /// Update requires: Replacement
    #[serde(rename = "DriveCacheType")]
    pub drive_cache_type: Option<LustreConfigurationDriveCacheTypeEnum>,


    /// 
    /// The number of days to retain automatic backups. Setting this property to         0 disables automatic backups. You can retain automatic backups for a       maximum of 90 days. The default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LustreConfigurationDeploymentTypeEnum {

    /// PERSISTENT_1
    #[serde(rename = "PERSISTENT_1")]
    Persistent1,

    /// PERSISTENT_2
    #[serde(rename = "PERSISTENT_2")]
    Persistent2,

    /// SCRATCH_1
    #[serde(rename = "SCRATCH_1")]
    Scratch1,

    /// SCRATCH_2
    #[serde(rename = "SCRATCH_2")]
    Scratch2,

}

impl Default for LustreConfigurationDeploymentTypeEnum {
    fn default() -> Self {
        LustreConfigurationDeploymentTypeEnum::Persistent1
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LustreConfigurationAutoImportPolicyEnum {

    /// NEW
    #[serde(rename = "NEW")]
    New,

    /// NEW_CHANGED
    #[serde(rename = "NEW_CHANGED")]
    Newchanged,

    /// NEW_CHANGED_DELETED
    #[serde(rename = "NEW_CHANGED_DELETED")]
    Newchangeddeleted,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for LustreConfigurationAutoImportPolicyEnum {
    fn default() -> Self {
        LustreConfigurationAutoImportPolicyEnum::New
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LustreConfigurationDriveCacheTypeEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// READ
    #[serde(rename = "READ")]
    Read,

}

impl Default for LustreConfigurationDriveCacheTypeEnum {
    fn default() -> Self {
        LustreConfigurationDriveCacheTypeEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LustreConfigurationDataCompressionTypeEnum {

    /// LZ4
    #[serde(rename = "LZ4")]
    Lz4,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for LustreConfigurationDataCompressionTypeEnum {
    fn default() -> Self {
        LustreConfigurationDataCompressionTypeEnum::Lz4
    }
}



/// The configuration for this Amazon FSx for NetApp ONTAP file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OntapConfiguration {


    /// 
    /// Sets the throughput capacity for the file system that you're creating. Valid values       are 128, 256, 512, 1024, 2048, and 4096 MBps.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 8
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: Option<i64>,


    /// 
    /// The number of days to retain automatic backups. Setting this property to         0 disables automatic backups. You can retain automatic backups for a       maximum of 90 days. The default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<i64>,


    /// 
    /// The SSD IOPS configuration for the FSx for ONTAP file system.
    /// 
    /// Required: No
    ///
    /// Type: DiskIopsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DiskIopsConfiguration")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,


    /// 
    /// A recurring daily time, in the format HH:MM. HH is the       zero-padded hour of the day (0-23), and MM is the zero-padded minute of the       hour. For example, 05:00 specifies 5 AM daily.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,


    /// 
    /// The ONTAP administrative password for the fsxadmin user with which you       administer your file system using the NetApp ONTAP CLI and REST API.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{8,50}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "FsxAdminPassword")]
    pub fsx_admin_password: Option<String>,


    /// 
    /// A recurring weekly time, in the format D:HH:MM.
    /// 
    /// D is the day of the week, for which 1 represents Monday and 7       represents Sunday. For further details, see the ISO-8601 spec as described on Wikipedia.
    /// 
    /// HH is the zero-padded hour of the day (0-23), and MM is       the zero-padded minute of the hour.
    /// 
    /// For example, 1:05:00 specifies maintenance at 5 AM Monday.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,


    /// 
    /// (Multi-AZ only) Specifies the IP address range in which the endpoints to access your       file system will be created. By default in the Amazon FSx API, Amazon FSx       selects an unused IP address range for you from the 198.19.* range. By default in the       Amazon FSx console, Amazon FSx chooses the last 64 IP addresses from       the VPC’s primary CIDR range to use as the endpoint IP address range for the file system.       You can have overlapping endpoint IP addresses for file systems deployed in the       same VPC/route tables, as long as they don't overlap with any subnet.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 9
    ///
    /// Maximum: 17
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{9,17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointIpAddressRange")]
    pub endpoint_ip_address_range: Option<String>,


    /// 
    /// (Multi-AZ only) Specifies the virtual private cloud (VPC) route tables in which your       file system's endpoints will be created. You should specify all VPC route tables       associated with the subnets in which your clients are located. By default, Amazon FSx       selects your VPC's default route table.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Option<Vec<String>>,


    /// 
    /// Required when DeploymentType is set to MULTI_AZ_1. This       specifies the subnet in which you want the preferred file server to be located.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreferredSubnetId")]
    pub preferred_subnet_id: Option<String>,


    /// 
    /// Specifies the FSx for ONTAP file system deployment type to use in creating       the file system.
    /// 
    /// MULTI_AZ_1 - (Default) A high availability file system configured           for Multi-AZ redundancy to tolerate temporary Availability Zone (AZ)           unavailability.                         SINGLE_AZ_1 - A file system configured for Single-AZ           redundancy.
    /// 
    /// For information about the use cases for Multi-AZ and Single-AZ deployments, refer to         Choosing a file system deployment type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_AZ_1 | SINGLE_AZ_1
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentType")]
    pub deployment_type: OntapConfigurationDeploymentTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OntapConfigurationDeploymentTypeEnum {

    /// MULTI_AZ_1
    #[serde(rename = "MULTI_AZ_1")]
    Multiaz1,

    /// SINGLE_AZ_1
    #[serde(rename = "SINGLE_AZ_1")]
    Singleaz1,

}

impl Default for OntapConfigurationDeploymentTypeEnum {
    fn default() -> Self {
        OntapConfigurationDeploymentTypeEnum::Multiaz1
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




/// The configuration of an Amazon FSx for OpenZFS root volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RootVolumeConfiguration {


    /// 
    /// A Boolean value indicating whether tags for the volume should be copied to snapshots       of the volume. This value defaults to false. If it's set to true,       all tags for the volume are copied to snapshots where the user doesn't specify tags. If this       value is true and you specify one or more tags, only the specified tags are       copied to snapshots. If you specify one or more tags when creating the snapshot, no tags       are copied from the volume, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CopyTagsToSnapshots")]
    pub copy_tags_to_snapshots: Option<bool>,


    /// 
    /// Specifies the record size of an OpenZFS root volume, in kibibytes (KiB). Valid values are 4, 8,     16, 32, 64, 128, 256, 512, or 1024 KiB. The default is 128 KiB. Most workloads should use the     default record size. Database workflows can benefit from a smaller record size, while streaming     workflows can benefit from a larger record size. For additional guidance on setting a custom record     size, see     Tips for maximizing performance in the     Amazon FSx for OpenZFS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 4
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "RecordSizeKiB")]
    pub record_size_ki_b: Option<i64>,


    /// 
    /// An object specifying how much storage users or groups can use on the volume.
    /// 
    /// Required: No
    ///
    /// Type: List of UserAndGroupQuotas
    ///
    /// Maximum: 500
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserAndGroupQuotas")]
    pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas>>,


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
    /// Update requires: Replacement
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<RootVolumeConfigurationDataCompressionTypeEnum>,


    /// 
    /// A Boolean value indicating whether the volume is read-only. Setting this value to         true can be useful after you have completed changes to a volume and no       longer want changes to occur.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,


    /// 
    /// The configuration object for mounting a file system.
    /// 
    /// Required: No
    ///
    /// Type: List of NfsExports
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "NfsExports")]
    pub nfs_exports: Option<Vec<NfsExports>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RootVolumeConfigurationDataCompressionTypeEnum {

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

impl Default for RootVolumeConfigurationDataCompressionTypeEnum {
    fn default() -> Self {
        RootVolumeConfigurationDataCompressionTypeEnum::Lz4
    }
}



/// The configuration object for mounting a file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NfsExports {


    /// 
    /// A list of configuration objects that contain the client and options for mounting the       OpenZFS file system.
    /// 
    /// Required: No
    ///
    /// Type: List of ClientConfigurations
    ///
    /// Maximum: 25
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientConfigurations")]
    pub client_configurations: Option<Vec<ClientConfigurations>>,

}




/// The OpenZFS configuration for the file system that's being created.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenZFSConfiguration {


    /// 
    /// A Boolean value indicating whether tags for the file system should be copied to volumes.       This value defaults to false. If it's set to true, all tags       for the file system are copied to volumes where the user doesn't specify tags. If this       value is true, and you specify one or more tags, only the specified tags       are copied to volumes. If you specify one or more tags when creating the volume, no       tags are copied from the file system, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToVolumes")]
    pub copy_tags_to_volumes: Option<bool>,


    /// 
    /// A Boolean value indicating whether tags for the file system should be copied to       backups. This value defaults to false. If it's set to true,       all tags for the file system are copied to all automatic and user-initiated backups       where the user doesn't specify tags. If this value is true, and you specify       one or more tags, only the specified tags are copied to backups. If you specify one or       more tags when creating a user-initiated backup, no tags are copied from the file       system, regardless of this value.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,


    /// 
    /// To delete a file system if there are child volumes present below the root volume,       use the string DELETE_CHILD_VOLUMES_AND_SNAPSHOTS. If your file system       has child volumes and you don't use this option, the delete request will fail.
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
    /// The configuration Amazon FSx uses when creating the root value of the Amazon FSx for OpenZFS       file system. All volumes are children of the root volume.
    /// 
    /// Required: No
    ///
    /// Type: RootVolumeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RootVolumeConfiguration")]
    pub root_volume_configuration: Option<RootVolumeConfiguration>,


    /// 
    /// A recurring weekly time, in the format D:HH:MM.
    /// 
    /// D is the day of the week, for which 1 represents Monday and 7       represents Sunday. For further details, see the ISO-8601 spec as described on Wikipedia.
    /// 
    /// HH is the zero-padded hour of the day (0-23), and MM is       the zero-padded minute of the hour.
    /// 
    /// For example, 1:05:00 specifies maintenance at 5 AM Monday.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,


    /// 
    /// A recurring daily time, in the format HH:MM. HH is the       zero-padded hour of the day (0-23), and MM is the zero-padded minute of the       hour. For example, 05:00 specifies 5 AM daily.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,


    /// 
    /// Specifies the throughput of an Amazon FSx for OpenZFS file system, measured in megabytes per second (MB/s). Valid values depend on the DeploymentType you choose, as follows:
    /// 
    /// For SINGLE_AZ_1, valid values are 64, 128, 256, 512, 1024, 2048, 3072, or 4096 MB/s.               For SINGLE_AZ_2, valid values are 160, 320, 640, 1280, 2560, 3840, 5120, 7680, or 10240 MB/s.
    /// 
    /// You pay for additional throughput capacity that you provision.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 8
    ///
    /// Maximum: 100000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: Option<i64>,


    /// 
    /// Specifies the file system deployment type. Single AZ deployment types are configured       for redundancy within a single Availability Zone in an AWS Region .       Valid values are the following:
    /// 
    /// SINGLE_AZ_1- (Default) Creates file systems with throughput capacities of 64 - 4,096 MB/s.         Single_AZ_1 is available in all AWS Regions where Amazon FSx         for OpenZFS is available, except US West (Oregon).                        SINGLE_AZ_2- Creates file systems with throughput capacities of 160 - 10,240 MB/s         using an NVMe L2ARC cache. Single_AZ_2 is available only in the US East (N. Virginia), US East (Ohio),         US West (Oregon), and Europe (Ireland) AWS Regions.
    /// 
    /// For more information, see: Deployment type availability       and File system performance       in the Amazon FSx for OpenZFS User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SINGLE_AZ_1 | SINGLE_AZ_2
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentType")]
    pub deployment_type: OpenZFSConfigurationDeploymentTypeEnum,


    /// 
    /// The SSD IOPS (input/output operations per second) configuration for an Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS file system. The       default is 3 IOPS per GB of storage capacity, but you can provision additional IOPS per       GB of storage. The configuration consists of the total number of provisioned SSD IOPS       and how the amount was provisioned (by the customer or by the system).
    /// 
    /// Required: No
    ///
    /// Type: DiskIopsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "DiskIopsConfiguration")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,


    /// 
    /// The number of days to retain automatic backups. Setting this property to         0 disables automatic backups. You can retain automatic backups for a       maximum of 90 days. The default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OpenZFSConfigurationDeploymentTypeEnum {

    /// SINGLE_AZ_1
    #[serde(rename = "SINGLE_AZ_1")]
    Singleaz1,

    /// SINGLE_AZ_2
    #[serde(rename = "SINGLE_AZ_2")]
    Singleaz2,

}

impl Default for OpenZFSConfigurationDeploymentTypeEnum {
    fn default() -> Self {
        OpenZFSConfigurationDeploymentTypeEnum::Singleaz1
    }
}



/// The configuration that Amazon FSx for Windows File Server uses to audit and log       user accesses of files, folders, and file shares on the Amazon FSx for Windows File Server       file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuditLogConfiguration {


    /// 
    /// Sets which attempt type is logged by Amazon FSx for file share accesses.
    /// 
    /// SUCCESS_ONLY - only successful attempts to access file           shares are logged.                        FAILURE_ONLY - only failed attempts to access file           shares are logged.                        SUCCESS_AND_FAILURE - both successful attempts and           failed attempts to access file shares are logged.                        DISABLED - access auditing of file shares is turned off.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | FAILURE_ONLY | SUCCESS_AND_FAILURE | SUCCESS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileShareAccessAuditLogLevel")]
    pub file_share_access_audit_log_level: AuditLogConfigurationFileShareAccessAuditLogLevelEnum,


    /// 
    /// Sets which attempt type is logged by Amazon FSx for file and folder accesses.
    /// 
    /// SUCCESS_ONLY - only successful attempts to access files           or folders are logged.                        FAILURE_ONLY - only failed attempts to access files           or folders are logged.                        SUCCESS_AND_FAILURE - both successful attempts and           failed attempts to access files or folders are logged.                        DISABLED - access auditing of files and folders is turned off.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | FAILURE_ONLY | SUCCESS_AND_FAILURE | SUCCESS_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileAccessAuditLogLevel")]
    pub file_access_audit_log_level: AuditLogConfigurationFileAccessAuditLogLevelEnum,


    /// 
    /// The Amazon Resource Name (ARN) for the destination of the audit logs.       The destination can be any Amazon CloudWatch Logs log group ARN or       Amazon Kinesis Data Firehose delivery stream ARN.
    /// 
    /// The name of the Amazon CloudWatch Logs log group must begin with       the /aws/fsx prefix. The name of the Amazon Kinesis Data       Firehouse delivery stream must begin with the aws-fsx prefix.
    /// 
    /// The destination ARN (either CloudWatch Logs log group or Kinesis       Data Firehose delivery stream) must be in the same AWS partition,       AWS Region, and AWS account as your Amazon FSx file system.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 8
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^arn:[^:]{1,63}:[^:]{0,63}:[^:]{0,63}:(?:|\d{12}):[^/].{0,1023}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuditLogDestination")]
    pub audit_log_destination: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AuditLogConfigurationFileShareAccessAuditLogLevelEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// FAILURE_ONLY
    #[serde(rename = "FAILURE_ONLY")]
    Failureonly,

    /// SUCCESS_AND_FAILURE
    #[serde(rename = "SUCCESS_AND_FAILURE")]
    Successandfailure,

    /// SUCCESS_ONLY
    #[serde(rename = "SUCCESS_ONLY")]
    Successonly,

}

impl Default for AuditLogConfigurationFileShareAccessAuditLogLevelEnum {
    fn default() -> Self {
        AuditLogConfigurationFileShareAccessAuditLogLevelEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AuditLogConfigurationFileAccessAuditLogLevelEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// FAILURE_ONLY
    #[serde(rename = "FAILURE_ONLY")]
    Failureonly,

    /// SUCCESS_AND_FAILURE
    #[serde(rename = "SUCCESS_AND_FAILURE")]
    Successandfailure,

    /// SUCCESS_ONLY
    #[serde(rename = "SUCCESS_ONLY")]
    Successonly,

}

impl Default for AuditLogConfigurationFileAccessAuditLogLevelEnum {
    fn default() -> Self {
        AuditLogConfigurationFileAccessAuditLogLevelEnum::Disabled
    }
}



/// The configuration that Amazon FSx uses to join a FSx for Windows File Server file system or an ONTAP storage virtual machine (SVM) to       a self-managed (including on-premises) Microsoft Active Directory (AD)       directory. For more information, see                Using Amazon FSx with your self-managed Microsoft Active Directory or       Managing SVMs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SelfManagedActiveDirectoryConfiguration {


    /// 
    /// A list of up to three IP addresses of DNS servers or domain controllers in the       self-managed AD directory.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsIps")]
    pub dns_ips: Option<Vec<String>>,


    /// 
    /// The user name for the service account on your self-managed AD domain that Amazon FSx       will use to join to your AD domain. This account must have the permission to join       computers to the domain in the organizational unit provided in         OrganizationalUnitDistinguishedName, or in the default location of your       AD domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,256}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,


    /// 
    /// (Optional) The name of the domain group whose members are granted administrative       privileges for the file system. Administrative privileges include taking ownership of       files and folders, setting audit controls (audit ACLs) on files and folders, and               administering the file system remotely by using the FSx Remote PowerShell.       The group that you specify must already exist in your domain. If you don't provide one,       your AD domain's Domain Admins group is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,256}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemAdministratorsGroup")]
    pub file_system_administrators_group: Option<String>,


    /// 
    /// (Optional) The fully qualified distinguished name of the organizational unit within       your self-managed AD directory. Amazon       FSx only accepts OU as the direct parent of the file system. An example is         OU=FSx,DC=yourdomain,DC=corp,DC=com. To learn more, see RFC 2253. If none is provided, the       FSx file system is created in the default location of your self-managed AD directory.
    /// 
    /// ImportantOnly Organizational Unit (OU) objects can be the direct parent of the file system         that you're creating.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2000
    ///
    /// Pattern: ^[^\u0000\u0085\u2028\u2029\r\n]{1,2000}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,


    /// 
    /// The password for the service account on your self-managed AD domain that Amazon FSx       will use to join to your AD domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^.{1,256}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: Option<String>,


    /// 
    /// The fully qualified domain name of the self-managed AD directory, such as         corp.example.com.
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
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

}




/// Specifies who can mount an OpenZFS file system and the options available while       mounting the file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientConfigurations {


    /// 
    /// A value that specifies who can mount the file system. You can provide a wildcard       character (*), an IP address (0.0.0.0), or a CIDR address         (192.0.2.0/24). By default, Amazon FSx uses the wildcard       character when specifying the client.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[ -~]{1,128}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Clients")]
    pub clients: Option<String>,


    /// 
    /// The options to use when mounting the file system. For a list of options that you can       use with Network File System (NFS), see the exports(5) - Linux man page. When       choosing your options, consider the following:
    /// 
    /// crossmnt is used by default. If you don't specify           crossmnt when changing the client configuration, you won't be           able to see or access snapshots in your file system's snapshot directory.                        sync is used by default. If you instead specify             async, the system acknowledges writes before writing to disk.           If the system crashes before the writes are finished, you lose the unwritten           data.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,

}




/// The configuration for how much storage a user or group can use on the volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserAndGroupQuotas {


    /// 
    /// The ID of the user or group.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: Option<i64>,


    /// 
    /// A value that specifies whether the quota applies to a user or group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GROUP | USER
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<UserAndGroupQuotasTypeEnum>,


    /// 
    /// The amount of storage that the user or group can use in gibibytes (GiB).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: Option<i64>,

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

