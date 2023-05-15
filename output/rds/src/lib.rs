
pub mod cfn_dbcluster {

#[derive(serde::Serialize, Default)]
pub struct CfnDBCluster {
    /// A list of Availability Zones (AZs) where instances in the DB cluster can be created. For information on AWS Regions and Availability Zones, see Choosing the Regions and Availability Zones in the Amazon Aurora User Guide.
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// The name of the DB cluster parameter group to associate with this DB cluster.
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,
    /// If you are configuring an Aurora global database cluster and want your Aurora DB cluster to be a secondary member in the global database cluster, specify the global cluster ID of the global database cluster. To define the primary database cluster of the global cluster, use the AWS::RDS::GlobalCluster resource.
    /// 
    /// If you aren't configuring a global database cluster, don't specify this property.
    #[serde(rename = "GlobalClusterIdentifier")]
    pub global_cluster_identifier: Option<String>,
    /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC). The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week. To see the time blocks available, see Adjusting the Preferred DB Cluster Maintenance Window in the Amazon Aurora User Guide.
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The target backtrack window, in seconds. To disable backtracking, set this value to 0.
    #[serde(rename = "BacktrackWindow")]
    pub backtrack_window: Option<usize>,
    /// A value that indicates whether minor engine upgrades are applied automatically to the DB cluster during the maintenance window. By default, minor engine upgrades are applied automatically.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// The AWS Region which contains the source DB cluster when replicating a DB cluster. For example, us-east-1.
    #[serde(rename = "SourceRegion")]
    pub source_region: Option<String>,
    /// Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other AWS services on your behalf.
    #[serde(rename = "AssociatedRoles")]
    pub associated_roles: Option<Vec<DBClusterRole>>,
    /// The daily time range during which automated backups are created if automated backups are enabled using the BackupRetentionPeriod parameter. The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. To see the time blocks available, see Adjusting the Preferred DB Cluster Maintenance Window in the Amazon Aurora User Guide.
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// The name of the DB parameter group to apply to all instances of the DB cluster.
    #[serde(rename = "DBInstanceParameterGroupName")]
    pub dbinstance_parameter_group_name: Option<String>,
    /// A value that indicates whether to turn on Performance Insights for the DB cluster.
    #[serde(rename = "PerformanceInsightsEnabled")]
    pub performance_insights_enabled: Option<bool>,
    /// Indicates whether the DB instance is encrypted.
    /// If you specify the DBClusterIdentifier, SnapshotIdentifier, or SourceDBInstanceIdentifier property, don't specify this property. The value is inherited from the cluster, snapshot, or source DB instance.
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// The network type of the DB cluster.
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,
    /// A value that indicates whether to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts. By default, mapping is disabled.
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    pub enable_iamdatabase_authentication: Option<bool>,
    /// A value that indicates whether the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled.
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,
    /// Contains the secret managed by RDS in AWS Secrets Manager for the master user password.
    #[serde(rename = "MasterUserSecret")]
    pub master_user_secret: Option<MasterUserSecret>,
    /// The name of your database. If you don't provide a name, then Amazon RDS won't create a database in this DB cluster. For naming constraints, see Naming Constraints in the Amazon RDS User Guide.
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    /// The master password for the DB instance.
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    /// The identifier for the DB snapshot or DB cluster snapshot to restore from.
    /// You can use either the name or the Amazon Resource Name (ARN) to specify a DB cluster snapshot. However, you can use only the ARN to specify a DB snapshot.
    /// After you restore a DB cluster with a SnapshotIdentifier property, you must specify the same SnapshotIdentifier property for any future updates to the DB cluster. When you specify this property for an update, the DB cluster is not restored from the snapshot again, and the data in the database is not changed. However, if you don't specify the SnapshotIdentifier property, an empty DB cluster is created, and the original DB cluster is deleted. If you specify a property that is different from the previous snapshot restore property, the DB cluster is restored from the specified SnapshotIdentifier property, and the original DB cluster is deleted.
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,
    /// The ScalingConfiguration property type specifies the scaling configuration of an Aurora Serverless DB cluster.
    #[serde(rename = "ScalingConfiguration")]
    pub scaling_configuration: Option<ScalingConfiguration>,
    /// The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB cluster is created as a Read Replica.
    #[serde(rename = "ReplicationSourceIdentifier")]
    pub replication_source_identifier: Option<String>,
    /// The compute and memory capacity of each DB instance in the Multi-AZ DB cluster, for example db.m6g.xlarge.
    #[serde(rename = "DBClusterInstanceClass")]
    pub dbcluster_instance_class: Option<String>,
    /// Contains the scaling configuration of an Aurora Serverless v2 DB cluster.
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    /// The DB cluster identifier. This parameter is stored as a lowercase string.
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,
    /// A value that indicates whether to copy all tags from the DB cluster to snapshots of the DB cluster. The default is not to copy them.
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// The Amazon Resource Name (ARN) of the AWS Key Management Service master key that is used to encrypt the database instances in the DB cluster, such as arn:aws:kms:us-east-1:012345678910:key/abcd1234-a123-456a-a12b-a123b4cd56ef. If you enable the StorageEncrypted property but don't specify this property, the default master key is used. If you specify this property, you must set the StorageEncrypted property to true.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// A value that indicates whether the DB cluster is publicly accessible.
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// A DB subnet group that you want to associate with this DB cluster.
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// The type of restore to be performed. You can specify one of the following values:
    /// full-copy - The new DB cluster is restored as a full copy of the source DB cluster.
    /// copy-on-write - The new DB cluster is restored as a clone of the source DB cluster.
    #[serde(rename = "RestoreType")]
    pub restore_type: Option<String>,
    /// The name of the master user for the DB cluster. You must specify MasterUsername, unless you specify SnapshotIdentifier. In that case, don't specify MasterUsername.
    #[serde(rename = "MasterUsername")]
    pub master_username: Option<String>,
    /// The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB cluster. To turn off collecting Enhanced Monitoring metrics, specify 0. The default is 0.
    #[serde(rename = "MonitoringInterval")]
    pub monitoring_interval: Option<usize>,
    /// The identifier of the source DB cluster from which to restore.
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,
    /// A value that indicates whether to restore the DB cluster to the latest restorable backup time. By default, the DB cluster is not restored to the latest restorable backup time.
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,
    /// A value that indicates whether to manage the master user password with AWS Secrets Manager.
    #[serde(rename = "ManageMasterUserPassword")]
    pub manage_master_user_password: Option<bool>,
    /// The Amazon Web Services KMS key identifier for encryption of Performance Insights data.
    #[serde(rename = "PerformanceInsightsKmsKeyId")]
    pub performance_insights_kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadEndpoint")]
    pub read_endpoint: Option<ReadEndpoint>,
    /// Reserved for future use.
    #[serde(rename = "DBSystemId")]
    pub dbsystem_id: Option<String>,
    /// The DB engine mode of the DB cluster, either provisioned, serverless, parallelquery, global, or multimaster.
    #[serde(rename = "EngineMode")]
    pub engine_mode: Option<String>,
    /// The number of days for which automated backups are retained.
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<usize>,
    /// The Amazon Resource Name (ARN) for the IAM role that permits RDS to send Enhanced Monitoring metrics to Amazon CloudWatch Logs.
    #[serde(rename = "MonitoringRoleArn")]
    pub monitoring_role_arn: Option<String>,
    /// The date and time to restore the DB cluster to. Value must be a time in Universal Coordinated Time (UTC) format. An example: 2015-03-07T23:45:00Z
    #[serde(rename = "RestoreToTime")]
    pub restore_to_time: Option<String>,
    /// A list of EC2 VPC security groups to associate with this DB cluster.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// The version number of the database engine to use.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// Specifies the storage type to be associated with the DB cluster.
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,
    /// The name of the database engine to be used for this DB cluster. Valid Values: aurora (for MySQL 5.6-compatible Aurora), aurora-mysql (for MySQL 5.7-compatible Aurora), and aurora-postgresql
    #[serde(rename = "Engine")]
    pub engine: Option<String>,
    /// The amount of storage in gibibytes (GiB) to allocate to each DB instance in the Multi-AZ DB cluster.
    #[serde(rename = "AllocatedStorage")]
    pub allocated_storage: Option<usize>,
    /// A value that indicates whether to enable the HTTP endpoint for an Aurora Serverless DB cluster. By default, the HTTP endpoint is disabled.
    #[serde(rename = "EnableHttpEndpoint")]
    pub enable_http_endpoint: Option<bool>,
    /// The amount of time, in days, to retain Performance Insights data.
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    pub performance_insights_retention_period: Option<usize>,
    /// The Active Directory directory ID to create the DB cluster in.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// The list of log types that need to be enabled for exporting to CloudWatch Logs. The values in the list depend on the DB engine being used. For more information, see Publishing Database Logs to Amazon CloudWatch Logs in the Amazon Aurora User Guide.
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// The amount of Provisioned IOPS (input/output operations per second) to be initially allocated for each DB instance in the Multi-AZ DB cluster.
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    /// Specify the name of the IAM role to be used when making API calls to the Directory Service.
    #[serde(rename = "DomainIAMRoleName")]
    pub domain_iamrole_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Endpoint {
    #[serde(rename = "Port")]
    pub port: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ServerlessV2ScalingConfiguration {
    #[serde(rename = "MinCapacity")]
    pub min_capacity: Option<f64>,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DBClusterRole {
    #[serde(rename = "FeatureName")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReadEndpoint {
    #[serde(rename = "Address")]
    pub address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<usize>,
    #[serde(rename = "AutoPause")]
    pub auto_pause: Option<bool>,
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: Option<String>,
    #[serde(rename = "SecondsUntilAutoPause")]
    pub seconds_until_auto_pause: Option<usize>,
    #[serde(rename = "SecondsBeforeTimeout")]
    pub seconds_before_timeout: Option<usize>,
    #[serde(rename = "MinCapacity")]
    pub min_capacity: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MasterUserSecret {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,

}


}

pub mod cfn_dbcluster_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBClusterParameterGroup {
    /// The DB cluster parameter group family name. A DB cluster parameter group can be associated with one and only one DB cluster parameter group family, and can be applied only to a DB cluster running a DB engine and engine version compatible with that DB cluster parameter group family.
    #[serde(rename = "Family")]
    pub family: String,
    /// A friendly description for this DB cluster parameter group.
    #[serde(rename = "Description")]
    pub description: String,
    /// The list of tags for the cluster parameter group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,
    /// An array of parameters to be modified. A maximum of 20 parameters can be modified in a single request.
    #[serde(rename = "Parameters")]
    pub parameters: (),

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_dbinstance {

#[derive(serde::Serialize, Default)]
pub struct CfnDBInstance {
    /// The compute and memory capacity of the DB instance, for example, db.m4.large. Not all DB instance classes are available in all AWS Regions, or for all database engines.
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: Option<String>,
    /// The amount of storage (in gigabytes) to be initially allocated for the database instance.
    #[serde(rename = "AllocatedStorage")]
    pub allocated_storage: Option<String>,
    /// The daily time range during which automated backups are created if automated backups are enabled, using the BackupRetentionPeriod parameter.
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// License model information for this DB instance.
    #[serde(rename = "LicenseModel")]
    pub license_model: Option<String>,
    /// The number of days for which automated backups are retained. Setting this parameter to a positive number enables backups. Setting this parameter to 0 disables automated backups.
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<usize>,
    /// A value that indicates whether minor engine upgrades are applied automatically to the DB instance during the maintenance window. By default, minor engine upgrades are applied automatically.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// The AWS Identity and Access Management (IAM) roles associated with the DB instance.
    #[serde(rename = "AssociatedRoles")]
    pub associated_roles: Option<Vec<DBInstanceRole>>,
    /// For supported engines, indicates that the DB instance should be associated with the specified character set.
    #[serde(rename = "CharacterSetName")]
    pub character_set_name: Option<String>,
    /// A list of the VPC security group IDs to assign to the DB instance. The list can include both the physical IDs of existing VPC security groups and references to AWS::EC2::SecurityGroup resources created in the template.
    #[serde(rename = "VPCSecurityGroups")]
    pub vpcsecurity_groups: Option<Vec<String>>,
    /// The ARN of the AWS Key Management Service (AWS KMS) master key that's used to encrypt the DB instance.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// The name of the NCHAR character set for the Oracle DB instance. This parameter doesn't apply to RDS Custom.
    #[serde(rename = "NcharCharacterSetName")]
    pub nchar_character_set_name: Option<String>,
    /// Specifies the connection endpoint.
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,
    /// The resource ID of the source DB instance from which to restore.
    #[serde(rename = "SourceDbiResourceId")]
    pub source_dbi_resource_id: Option<String>,
    /// The password for the master user.
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    /// The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB instance. To disable collecting Enhanced Monitoring metrics, specify 0. The default is 0.
    #[serde(rename = "MonitoringInterval")]
    pub monitoring_interval: Option<usize>,
    /// A value that indicates whether to enable Performance Insights for the DB instance.
    #[serde(rename = "EnablePerformanceInsights")]
    pub enable_performance_insights: Option<bool>,
    /// The instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance. The instance profile must meet the following requirements:
    /// * The profile must exist in your account.
    /// * The profile must have an IAM role that Amazon EC2 has permissions to assume.
    /// * The instance profile name and the associated IAM role name must start with the prefix AWSRDSCustom .
    /// For the list of permissions required for the IAM role, see Configure IAM and your VPC in the Amazon RDS User Guide .
    /// 
    /// This setting is required for RDS Custom.
    #[serde(rename = "CustomIAMInstanceProfile")]
    pub custom_iaminstance_profile: Option<String>,
    /// A value that indicates whether major version upgrades are allowed. Changing this parameter doesn't result in an outage and the change is asynchronously applied as soon as possible.
    #[serde(rename = "AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: Option<bool>,
    /// The Amazon Resource Name (ARN) of the replicated automated backups from which to restore.
    #[serde(rename = "SourceDBInstanceAutomatedBackupsArn")]
    pub source_dbinstance_automated_backups_arn: Option<String>,
    /// The master user name for the DB instance.
    #[serde(rename = "MasterUsername")]
    pub master_username: Option<String>,
    /// Specifies the storage throughput for the DB instance.
    #[serde(rename = "StorageThroughput")]
    pub storage_throughput: Option<usize>,
    /// Returns the details of the DB instance's server certificate.
    #[serde(rename = "CertificateDetails")]
    pub certificate_details: Option<CertificateDetails>,
    /// A value that specifies the order in which an Aurora Replica is promoted to the primary instance after a failure of the existing primary instance.
    #[serde(rename = "PromotionTier")]
    pub promotion_tier: Option<usize>,
    /// The network type of the DB cluster.
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,
    /// Indicates whether the DB instance is an internet-facing instance. If you specify true, AWS CloudFormation creates an instance with a publicly resolvable DNS name, which resolves to a public IP address. If you specify false, AWS CloudFormation creates an internal instance with a DNS name that resolves to a private IP address.
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// The date and time to restore from.
    #[serde(rename = "RestoreTime")]
    pub restore_time: Option<String>,
    /// A value that indicates whether the DB instance is restored from the latest backup time. By default, the DB instance isn't restored from the latest backup time.
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,
    /// The identifier of the Multi-AZ DB cluster that will act as the source for the read replica. Each DB cluster can have up to 15 read replicas.
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,
    /// A value that indicates whether the DB instance has deletion protection enabled. The database can't be deleted when deletion protection is enabled. By default, deletion protection is disabled.
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,
    /// The list of log types that need to be enabled for exporting to CloudWatch Logs. The values in the list depend on the DB engine being used.
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// The Availability Zone (AZ) where the database will be created. For information on AWS Regions and Availability Zones.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// Specifies the storage type to be associated with the DB instance.
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,
    /// The identifier of the CA certificate for this DB instance.
    #[serde(rename = "CACertificateIdentifier")]
    pub cacertificate_identifier: Option<String>,
    /// The version number of the database engine to use.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// The ARN from the key store with which to associate the instance for TDE encryption.
    #[serde(rename = "TdeCredentialArn")]
    pub tde_credential_arn: Option<String>,
    /// The name of the database engine that you want to use for this DB instance.
    #[serde(rename = "Engine")]
    pub engine: Option<String>,
    /// A value that indicates whether to manage the master user password with AWS Secrets Manager.
    #[serde(rename = "ManageMasterUserPassword")]
    pub manage_master_user_password: Option<bool>,
    /// The number of I/O operations per second (IOPS) that the database provisions.
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    /// Tags to assign to the DB instance.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The upper limit to which Amazon RDS can automatically scale the storage of the DB instance.
    #[serde(rename = "MaxAllocatedStorage")]
    pub max_allocated_storage: Option<usize>,
    /// A value that indicates whether the DB instance is restarted when you rotate your SSL/TLS certificate.
    /// By default, the DB instance is restarted when you rotate your SSL/TLS certificate. The certificate is not updated until the DB instance is restarted.
    /// If you are using SSL/TLS to connect to the DB instance, follow the appropriate instructions for your DB engine to rotate your SSL/TLS certificate
    /// This setting doesn't apply to RDS Custom.
    #[serde(rename = "CertificateRotationRestart")]
    pub certificate_rotation_restart: Option<bool>,
    /// Specifies whether the database instance is a multiple Availability Zone deployment.
    #[serde(rename = "MultiAZ")]
    pub multi_az: Option<bool>,
    /// The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.
    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Option<Vec<ProcessorFeature>>,
    /// A list of the DB security groups to assign to the DB instance. The list can include both the name of existing DB security groups or references to AWS::RDS::DBSecurityGroup resources created in the template.
    #[serde(rename = "DBSecurityGroups")]
    pub dbsecurity_groups: Option<Vec<String>>,
    /// The open mode of an Oracle read replica. The default is open-read-only.
    #[serde(rename = "ReplicaMode")]
    pub replica_mode: Option<String>,
    /// The identifier for the RDS for MySQL Multi-AZ DB cluster snapshot to restore from. For more information on Multi-AZ DB clusters, see Multi-AZ deployments with two readable standby DB instances in the Amazon RDS User Guide .
    /// 
    /// Constraints:
    /// * Must match the identifier of an existing Multi-AZ DB cluster snapshot.
    /// * Can't be specified when DBSnapshotIdentifier is specified.
    /// * Must be specified when DBSnapshotIdentifier isn't specified.
    /// * If you are restoring from a shared manual Multi-AZ DB cluster snapshot, the DBClusterSnapshotIdentifier must be the ARN of the shared snapshot.
    /// * Can't be the identifier of an Aurora DB cluster snapshot.
    /// * Can't be the identifier of an RDS for PostgreSQL Multi-AZ DB cluster snapshot.
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    pub dbcluster_snapshot_identifier: Option<String>,
    /// A value that indicates whether the DB instance is encrypted. By default, it isn't encrypted.
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// Indicates that the DB instance should be associated with the specified option group.
    #[serde(rename = "OptionGroupName")]
    pub option_group_name: Option<String>,
    /// The password for the given ARN from the key store in order to access the device.
    #[serde(rename = "TdeCredentialPassword")]
    pub tde_credential_password: Option<String>,
    /// The amount of time, in days, to retain Performance Insights data. Valid values are 7 or 731 (2 years).
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    pub performance_insights_retention_period: Option<usize>,
    /// The time zone of the DB instance. The time zone parameter is currently supported only by Microsoft SQL Server.
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    /// The name or Amazon Resource Name (ARN) of the DB snapshot that's used to restore the DB instance. If you're restoring from a shared manual DB snapshot, you must specify the ARN of the snapshot.
    #[serde(rename = "DBSnapshotIdentifier")]
    pub dbsnapshot_identifier: Option<String>,
    /// The ARN for the IAM role that permits RDS to send enhanced monitoring metrics to Amazon CloudWatch Logs.
    #[serde(rename = "MonitoringRoleArn")]
    pub monitoring_role_arn: Option<String>,
    /// he weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The Active Directory directory ID to create the DB instance in. Currently, only MySQL, Microsoft SQL Server, Oracle, and PostgreSQL DB instances can be created in an Active Directory Domain.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// The identifier of the DB cluster that the instance will belong to.
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,
    /// If you want to create a Read Replica DB instance, specify the ID of the source DB instance. Each DB instance can have a limited number of Read Replicas.
    #[serde(rename = "SourceDBInstanceIdentifier")]
    pub source_dbinstance_identifier: Option<String>,
    /// The meaning of this parameter differs according to the database engine you use.
    #[serde(rename = "DBName")]
    pub dbname: Option<String>,
    /// Specify the name of the IAM role to be used when making API calls to the Directory Service.
    #[serde(rename = "DomainIAMRoleName")]
    pub domain_iamrole_name: Option<String>,
    /// A value that indicates whether the DB instance class of the DB instance uses its default processor features.
    #[serde(rename = "UseDefaultProcessorFeatures")]
    pub use_default_processor_features: Option<bool>,
    /// A value that indicates whether to enable mapping of AWS Identity and Access Management (IAM) accounts to database accounts. By default, mapping is disabled.
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    pub enable_iamdatabase_authentication: Option<bool>,
    /// A value that indicates whether to copy tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// The ID of the region that contains the source DB instance for the Read Replica.
    #[serde(rename = "SourceRegion")]
    pub source_region: Option<String>,
    /// A name for the DB instance. If you specify a name, AWS CloudFormation converts it to lowercase. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the DB instance.
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: Option<String>,
    /// A DB subnet group to associate with the DB instance. If you update this value, the new subnet group must be a subnet group in a new VPC.
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// Contains the secret managed by RDS in AWS Secrets Manager for the master user password.
    #[serde(rename = "MasterUserSecret")]
    pub master_user_secret: Option<MasterUserSecret>,
    /// The name of an existing DB parameter group or a reference to an AWS::RDS::DBParameterGroup resource created in the template.
    #[serde(rename = "DBParameterGroupName")]
    pub dbparameter_group_name: Option<String>,
    /// The AWS KMS key identifier for encryption of Performance Insights data. The KMS key ID is the Amazon Resource Name (ARN), KMS key identifier, or the KMS key alias for the KMS encryption key.
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    pub performance_insights_kmskey_id: Option<String>,
    /// A value that indicates whether to remove automated backups immediately after the DB instance is deleted. This parameter isn't case-sensitive. The default is to remove automated backups immediately after the DB instance is deleted.
    #[serde(rename = "DeleteAutomatedBackups")]
    pub delete_automated_backups: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct DBInstanceRole {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "FeatureName")]
    pub feature_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MasterUserSecret {
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CertificateDetails {
    #[serde(rename = "ValidTill")]
    pub valid_till: Option<String>,
    #[serde(rename = "CAIdentifier")]
    pub caidentifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Endpoint {
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProcessorFeature {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}

pub mod cfn_dbparameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBParameterGroup {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Provides the customer-specified description for this DB parameter group.
    #[serde(rename = "Description")]
    pub description: String,
    /// The DB parameter group family name.
    #[serde(rename = "Family")]
    pub family: String,
    /// Specifies the name of the DB parameter group
    #[serde(rename = "DBParameterGroupName")]
    pub dbparameter_group_name: Option<String>,
    /// An array of parameter names and values for the parameter update.
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_dbproxy {

#[derive(serde::Serialize, Default)]
pub struct CfnDBProxy {
    /// The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access secrets in AWS Secrets Manager.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The identifier for the proxy. This name must be unique for all proxies owned by your AWS account in the specified AWS Region.
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,
    /// The authorization mechanism that the proxy uses.
    #[serde(rename = "Auth")]
    pub auth: Vec<AuthFormat>,
    /// The number of seconds that a connection to the proxy can be inactive before the proxy disconnects it.
    #[serde(rename = "IdleClientTimeout")]
    pub idle_client_timeout: Option<usize>,
    /// An optional set of key-value pairs to associate arbitrary data of your choosing with the proxy.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagFormat>>,
    /// Whether the proxy includes detailed information about SQL statements in its logs.
    #[serde(rename = "DebugLogging")]
    pub debug_logging: Option<bool>,
    /// The kinds of databases that the proxy can connect to.
    #[serde(rename = "EngineFamily")]
    pub engine_family: String,
    /// VPC security group IDs to associate with the new proxy.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// VPC subnet IDs to associate with the new proxy.
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Vec<String>,
    /// A Boolean parameter that specifies whether Transport Layer Security (TLS) encryption is required for connections to the proxy.
    #[serde(rename = "RequireTLS")]
    pub require_tls: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct TagFormat {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthFormat {
    #[serde(rename = "ClientPasswordAuthType")]
    pub client_password_auth_type: Option<String>,
    #[serde(rename = "AuthScheme")]
    pub auth_scheme: Option<String>,
    #[serde(rename = "IAMAuth")]
    pub iamauth: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,

}


}

pub mod cfn_dbproxy_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnDBProxyEndpoint {
    /// VPC security group IDs to associate with the new DB proxy endpoint.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// The identifier for the DB proxy endpoint. This name must be unique for all DB proxy endpoints owned by your AWS account in the specified AWS Region.
    #[serde(rename = "DBProxyEndpointName")]
    pub dbproxy_endpoint_name: String,
    /// A value that indicates whether the DB proxy endpoint can be used for read/write or read-only operations.
    #[serde(rename = "TargetRole")]
    pub target_role: Option<String>,
    /// The identifier for the proxy. This name must be unique for all proxies owned by your AWS account in the specified AWS Region.
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,
    /// VPC subnet IDs to associate with the new DB proxy endpoint.
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Vec<String>,
    /// An optional set of key-value pairs to associate arbitrary data of your choosing with the DB proxy endpoint.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagFormat>>,

}


#[derive(serde::Serialize, Default)]
pub struct TagFormat {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_dbproxy_target_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBProxyTargetGroup {
    /// The identifier for the DBProxyTargetGroup
    #[serde(rename = "TargetGroupName")]
    pub target_group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DBInstanceIdentifiers")]
    pub dbinstance_identifiers: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionPoolConfigurationInfo")]
    pub connection_pool_configuration_info: Option<ConnectionPoolConfigurationInfoFormat>,
    /// The identifier for the proxy.
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterIdentifiers")]
    pub dbcluster_identifiers: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectionPoolConfigurationInfoFormat {
    #[serde(rename = "SessionPinningFilters")]
    pub session_pinning_filters: Option<Vec<String>>,
    #[serde(rename = "InitQuery")]
    pub init_query: Option<String>,
    #[serde(rename = "ConnectionBorrowTimeout")]
    pub connection_borrow_timeout: Option<usize>,
    #[serde(rename = "MaxIdleConnectionsPercent")]
    pub max_idle_connections_percent: Option<usize>,
    #[serde(rename = "MaxConnectionsPercent")]
    pub max_connections_percent: Option<usize>,

}


}

pub mod cfn_dbsecurity_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBSecurityGroup {
    /// List of Ingress
    #[serde(rename = "DBSecurityGroupIngress")]
    pub dbsecurity_group_ingress: Vec<Ingress>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "EC2VpcId")]
    pub ec2_vpc_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupDescription")]
    pub group_description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Ingress {
    #[serde(rename = "EC2SecurityGroupId")]
    pub ec2_security_group_id: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,
    #[serde(rename = "CIDRIP")]
    pub cidrip: Option<String>,
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: Option<String>,

}


}

pub mod cfn_dbsecurity_group_ingress {

#[derive(serde::Serialize, Default)]
pub struct CfnDBSecurityGroupIngress {
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupId")]
    pub ec2_security_group_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CIDRIP")]
    pub cidrip: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSecurityGroupName")]
    pub dbsecurity_group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,

}



}

pub mod cfn_dbsubnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBSubnetGroup {
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupDescription")]
    pub dbsubnet_group_description: String,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_event_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnEventSubscription {
    /// The list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it cannot end with a hyphen or contain two consecutive hyphens.
    #[serde(rename = "SourceIds")]
    pub source_ids: Option<Vec<String>>,
    /// A Boolean value; set to true to activate the subscription, set to false to create the subscription but not active it.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// A list of event categories for a SourceType that you want to subscribe to. You can see a list of the categories for a given SourceType in the Events topic in the Amazon RDS User Guide or by using the DescribeEventCategories action.
    #[serde(rename = "EventCategories")]
    pub event_categories: Option<Vec<String>>,
    /// The name of the subscription.
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
    /// The type of source that will be generating the events. For example, if you want to be notified of events generated by a DB instance, you would set this parameter to db-instance. if this value is not specified, all events are returned.
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_global_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnGlobalCluster {
    /// The name of the database engine to be used for this DB cluster. Valid Values: aurora (for MySQL 5.6-compatible Aurora), aurora-mysql (for MySQL 5.7-compatible Aurora).
    /// If you specify the SourceDBClusterIdentifier property, don't specify this property. The value is inherited from the cluster.
    #[serde(rename = "Engine")]
    pub engine: Option<String>,
    /// The cluster identifier of the new global database cluster. This parameter is stored as a lowercase string.
    #[serde(rename = "GlobalClusterIdentifier")]
    pub global_cluster_identifier: Option<String>,
    /// The deletion protection setting for the new global database. The global database can't be deleted when deletion protection is enabled.
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,
    /// The version number of the database engine to use. If you specify the SourceDBClusterIdentifier property, don't specify this property. The value is inherited from the cluster.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// The storage encryption setting for the new global database cluster.
    /// If you specify the SourceDBClusterIdentifier property, don't specify this property. The value is inherited from the cluster.
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// The Amazon Resource Name (ARN) to use as the primary cluster of the global database. This parameter is optional. This parameter is stored as a lowercase string.
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,

}



}

pub mod cfn_option_group {

#[derive(serde::Serialize, Default)]
pub struct CfnOptionGroup {
    /// Indicates the major engine version associated with this option group.
    #[serde(rename = "MajorEngineVersion")]
    pub major_engine_version: String,
    /// Specifies the name of the option group.
    #[serde(rename = "OptionGroupName")]
    pub option_group_name: Option<String>,
    /// Indicates what options are available in the option group.
    #[serde(rename = "OptionConfigurations")]
    pub option_configurations: Option<Vec<OptionConfiguration>>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Indicates the name of the engine that this option group can be applied to.
    #[serde(rename = "EngineName")]
    pub engine_name: String,
    /// Provides a description of the option group.
    #[serde(rename = "OptionGroupDescription")]
    pub option_group_description: String,

}


#[derive(serde::Serialize, Default)]
pub struct OptionConfiguration {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "OptionSettings")]
    pub option_settings: Option<Vec<OptionSetting>>,
    #[serde(rename = "VpcSecurityGroupMemberships")]
    pub vpc_security_group_memberships: Option<Vec<String>>,
    #[serde(rename = "OptionVersion")]
    pub option_version: Option<String>,
    #[serde(rename = "OptionName")]
    pub option_name: String,
    #[serde(rename = "DBSecurityGroupMemberships")]
    pub dbsecurity_group_memberships: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OptionSetting {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}
