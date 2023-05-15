
pub mod cfn_data_repository_association {

#[derive(serde::Serialize, Default)]
pub struct CfnDataRepositoryAssociation {
    /// The globally unique ID of the file system, assigned by Amazon FSx.
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// A boolean flag indicating whether an import data repository task to import metadata should run after the data repository association is created. The task runs if this flag is set to true.
    #[serde(rename = "BatchImportMetaDataOnCreate")]
    pub batch_import_meta_data_on_create: Option<bool>,
    /// This path specifies where in your file system files will be exported from or imported to. This file system directory can be linked to only one Amazon S3 bucket, and no other S3 bucket can be linked to the directory.
    #[serde(rename = "FileSystemPath")]
    pub file_system_path: String,
    /// The path to the Amazon S3 data repository that will be linked to the file system. The path can be an S3 bucket or prefix in the format s3://myBucket/myPrefix/ . This path specifies where in the S3 data repository files will be imported from or exported to.
    #[serde(rename = "DataRepositoryPath")]
    pub data_repository_path: String,
    /// For files imported from a data repository, this value determines the stripe count and maximum amount of data per file (in MiB) stored on a single physical disk. The maximum number of disks that a single file can be striped across is limited by the total number of disks that make up the file system.
    #[serde(rename = "ImportedFileChunkSize")]
    pub imported_file_chunk_size: Option<usize>,
    /// The configuration for an Amazon S3 data repository linked to an Amazon FSx Lustre file system with a data repository association. The configuration defines which file events (new, changed, or deleted files or directories) are automatically imported from the linked data repository to the file system or automatically exported from the file system to the data repository.
    #[serde(rename = "S3")]
    pub s3: Option<S3>,
    /// A list of Tag values, with a maximum of 50 elements.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct AutoImportPolicy {
    #[serde(rename = "Events")]
    pub events: EventTypes,

}
pub type EventType = String;
#[derive(serde::Serialize, Default)]
pub struct EventTypes {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct AutoExportPolicy {
    #[serde(rename = "Events")]
    pub events: EventTypes,

}

#[derive(serde::Serialize, Default)]
pub struct S3 {
    #[serde(rename = "AutoImportPolicy")]
    pub auto_import_policy: Option<AutoImportPolicy>,
    #[serde(rename = "AutoExportPolicy")]
    pub auto_export_policy: Option<AutoExportPolicy>,

}


}

pub mod cfn_file_system {

#[derive(serde::Serialize, Default)]
pub struct CfnFileSystem {
    /// No documentation provided by AWS
    #[serde(rename = "FileSystemTypeVersion")]
    pub file_system_type_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WindowsConfiguration")]
    pub windows_configuration: Option<WindowsConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OntapConfiguration")]
    pub ontap_configuration: Option<OntapConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "LustreConfiguration")]
    pub lustre_configuration: Option<LustreConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "StorageCapacity")]
    pub storage_capacity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "OpenZFSConfiguration")]
    pub open_zfsconfiguration: Option<OpenZFSConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupId")]
    pub backup_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct OpenZFSConfiguration {
    #[serde(rename = "CopyTagsToVolumes")]
    pub copy_tags_to_volumes: Option<bool>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    pub deployment_type: String,
    #[serde(rename = "RootVolumeConfiguration")]
    pub root_volume_configuration: Option<RootVolumeConfiguration>,
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<usize>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: Option<usize>,
    #[serde(rename = "DiskIopsConfiguration")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AuditLogConfiguration {
    #[serde(rename = "FileShareAccessAuditLogLevel")]
    pub file_share_access_audit_log_level: String,
    #[serde(rename = "AuditLogDestination")]
    pub audit_log_destination: Option<String>,
    #[serde(rename = "FileAccessAuditLogLevel")]
    pub file_access_audit_log_level: String,

}

#[derive(serde::Serialize, Default)]
pub struct WindowsConfiguration {
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<String>,
    #[serde(rename = "ActiveDirectoryId")]
    pub active_directory_id: Option<String>,
    #[serde(rename = "PreferredSubnetId")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "AuditLogConfiguration")]
    pub audit_log_configuration: Option<AuditLogConfiguration>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryConfiguration>,
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: usize,
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct UserAndGroupQuotas {
    #[serde(rename = "Id")]
    pub id: Option<usize>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct NfsExports {
    #[serde(rename = "ClientConfigurations")]
    pub client_configurations: Option<Vec<ClientConfigurations>>,

}

#[derive(serde::Serialize, Default)]
pub struct OntapConfiguration {
    #[serde(rename = "ThroughputCapacity")]
    pub throughput_capacity: Option<usize>,
    #[serde(rename = "EndpointIpAddressRange")]
    pub endpoint_ip_address_range: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "DeploymentType")]
    pub deployment_type: String,
    #[serde(rename = "PreferredSubnetId")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Option<Vec<String>>,
    #[serde(rename = "FsxAdminPassword")]
    pub fsx_admin_password: Option<String>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SelfManagedActiveDirectoryConfiguration {
    #[serde(rename = "FileSystemAdministratorsGroup")]
    pub file_system_administrators_group: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
    #[serde(rename = "DnsIps")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LustreConfiguration {
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    pub weekly_maintenance_start_time: Option<String>,
    #[serde(rename = "ImportPath")]
    pub import_path: Option<String>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "ExportPath")]
    pub export_path: Option<String>,
    #[serde(rename = "AutoImportPolicy")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "DriveCacheType")]
    pub drive_cache_type: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    pub imported_file_chunk_size: Option<usize>,
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<String>,
    #[serde(rename = "PerUnitStorageThroughput")]
    pub per_unit_storage_throughput: Option<usize>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    pub automatic_backup_retention_days: Option<usize>,
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct RootVolumeConfiguration {
    #[serde(rename = "UserAndGroupQuotas")]
    pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas>>,
    #[serde(rename = "NfsExports")]
    pub nfs_exports: Option<Vec<NfsExports>>,
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "CopyTagsToSnapshots")]
    pub copy_tags_to_snapshots: Option<bool>,
    #[serde(rename = "RecordSizeKiB")]
    pub record_size_ki_b: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DiskIopsConfiguration {
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientConfigurations {
    #[serde(rename = "Clients")]
    pub clients: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,

}


}

pub mod cfn_snapshot {

#[derive(serde::Serialize, Default)]
pub struct CfnSnapshot {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
    /// List of Tag
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

pub mod cfn_storage_virtual_machine {

#[derive(serde::Serialize, Default)]
pub struct CfnStorageVirtualMachine {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ActiveDirectoryConfiguration")]
    pub active_directory_configuration: Option<ActiveDirectoryConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "RootVolumeSecurityStyle")]
    pub root_volume_security_style: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SvmAdminPassword")]
    pub svm_admin_password: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct SelfManagedActiveDirectoryConfiguration {
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "DnsIps")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "FileSystemAdministratorsGroup")]
    pub file_system_administrators_group: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ActiveDirectoryConfiguration {
    #[serde(rename = "NetBiosName")]
    pub net_bios_name: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryConfiguration>,

}


}

pub mod cfn_volume {

#[derive(serde::Serialize, Default)]
pub struct CfnVolume {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "OpenZFSConfiguration")]
    pub open_zfsconfiguration: Option<OpenZFSConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "OntapConfiguration")]
    pub ontap_configuration: Option<OntapConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupId")]
    pub backup_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NfsExports {
    #[serde(rename = "ClientConfigurations")]
    pub client_configurations: Vec<ClientConfigurations>,

}

#[derive(serde::Serialize, Default)]
pub struct OntapConfiguration {
    #[serde(rename = "SecurityStyle")]
    pub security_style: Option<String>,
    #[serde(rename = "StorageEfficiencyEnabled")]
    pub storage_efficiency_enabled: Option<String>,
    #[serde(rename = "SizeInMegabytes")]
    pub size_in_megabytes: String,
    #[serde(rename = "OntapVolumeType")]
    pub ontap_volume_type: Option<String>,
    #[serde(rename = "TieringPolicy")]
    pub tiering_policy: Option<TieringPolicy>,
    #[serde(rename = "SnapshotPolicy")]
    pub snapshot_policy: Option<String>,
    #[serde(rename = "StorageVirtualMachineId")]
    pub storage_virtual_machine_id: String,
    #[serde(rename = "CopyTagsToBackups")]
    pub copy_tags_to_backups: Option<String>,
    #[serde(rename = "JunctionPath")]
    pub junction_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct OriginSnapshot {
    #[serde(rename = "CopyStrategy")]
    pub copy_strategy: String,
    #[serde(rename = "SnapshotARN")]
    pub snapshot_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct TieringPolicy {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CoolingPeriod")]
    pub cooling_period: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct UserAndGroupQuotas {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ClientConfigurations {
    #[serde(rename = "Options")]
    pub options: Vec<String>,
    #[serde(rename = "Clients")]
    pub clients: String,

}

#[derive(serde::Serialize, Default)]
pub struct OpenZFSConfiguration {
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "StorageCapacityReservationGiB")]
    pub storage_capacity_reservation_gi_b: Option<usize>,
    #[serde(rename = "DataCompressionType")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "OriginSnapshot")]
    pub origin_snapshot: Option<OriginSnapshot>,
    #[serde(rename = "CopyTagsToSnapshots")]
    pub copy_tags_to_snapshots: Option<bool>,
    #[serde(rename = "NfsExports")]
    pub nfs_exports: Option<Vec<NfsExports>>,
    #[serde(rename = "RecordSizeKiB")]
    pub record_size_ki_b: Option<usize>,
    #[serde(rename = "ParentVolumeId")]
    pub parent_volume_id: String,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    pub storage_capacity_quota_gi_b: Option<usize>,
    #[serde(rename = "UserAndGroupQuotas")]
    pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas>>,

}


}
