/// The AWS::RDS::DBCluster resource creates an Amazon Aurora DB cluster or Multi-AZ DB cluster.
///
/// For more information about creating an Aurora DB cluster, see         Creating an Amazon Aurora DB cluster in the Amazon Aurora User Guide.
///
/// For more information about creating a Multi-AZ DB cluster, see       Creating a Multi-AZ DB cluster       in the Amazon RDS User Guide.
///
/// Updating DB clusters
///
/// When properties labeled "Update requires:               Replacement" are updated, AWS CloudFormation first creates a replacement DB       cluster, then changes references from other dependent resources to point to the       replacement DB cluster, and finally deletes the old DB cluster.
///
/// Currently, when you are updating the stack for an Aurora Serverless DB cluster, you can't include changes to       any other properties when you specify one of the following properties: PreferredBackupWindow,       PreferredMaintenanceWindow, and Port. This limitation doesn't apply to provisioned       DB clusters.
///
/// For more information about updating other properties of this resource, see         ModifyDBCluster. For more information about updating stacks, see         AWS         CloudFormation Stacks Updates.
///
/// Deleting DB clusters
///
/// The default DeletionPolicy for AWS::RDS::DBCluster resources       is Snapshot. For more information about how AWS CloudFormation deletes       resources, see         DeletionPolicy Attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBCluster {
    ///
    /// The amount of storage in gibibytes (GiB) to allocate to each DB instance in the Multi-AZ DB cluster.
    ///
    /// This setting is required to create a Multi-AZ DB cluster.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocatedStorage")]
    pub allocated_storage: Option<i64>,

    ///
    /// Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster.      IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other Amazon Web Services      on your behalf.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: List of DBClusterRole
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatedRoles")]
    pub associated_roles: Option<Vec<DBClusterRole>>,

    ///
    /// A value that indicates whether minor engine upgrades are applied automatically to the DB cluster during the maintenance window.       By default, minor engine upgrades are applied automatically.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,

    ///
    /// A list of Availability Zones (AZs) where instances in the DB cluster can be created. For information on       AWS Regions and Availability Zones, see       Choosing the Regions and         Availability Zones in the Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// The target backtrack window, in seconds. To disable backtracking, set this value to       0.
    ///
    /// NoteCurrently, Backtrack is only supported for Aurora MySQL DB clusters.
    ///
    /// Default: 0
    ///
    /// Constraints:
    ///
    /// If specified, this value must be set to a number from 0 to 259,200 (72 hours).
    ///
    /// Valid for: Aurora MySQL DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BacktrackWindow")]
    pub backtrack_window: Option<i64>,

    ///
    /// The number of days for which automated backups are retained.
    ///
    /// Default: 1
    ///
    /// Constraints:
    ///
    /// Must be a value from 1 to 35
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<i64>,

    ///
    /// A value that indicates whether to copy all tags from the DB cluster to snapshots of the DB cluster.       The default is not to copy them.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,

    ///
    /// The DB cluster identifier. This parameter is stored as a lowercase string.
    ///
    /// Constraints:
    ///
    /// Must contain from 1 to 63 letters, numbers, or hyphens.               First character must be a letter.               Can't end with a hyphen or contain two consecutive hyphens.
    ///
    /// Example: my-cluster1
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,

    ///
    /// The compute and memory capacity of each DB instance in the Multi-AZ DB cluster, for example db.m6gd.xlarge.       Not all DB instance classes are available in all AWS Regions, or for all database engines.
    ///
    /// For the full list of DB instance classes and availability for your engine, see DB instance class in the Amazon RDS User Guide.
    ///
    /// This setting is required to create a Multi-AZ DB cluster.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBClusterInstanceClass")]
    pub dbcluster_instance_class: Option<String>,

    ///
    /// The name of the DB cluster parameter group to associate with this DB cluster.
    ///
    /// ImportantIf you apply a parameter group to an existing DB cluster, then its DB instances         might need to reboot. This can result in an outage while the DB instances are         rebooting.If you apply a change to parameter group associated with a stopped DB cluster,         then the update stack waits until the DB cluster is started.
    ///
    /// To list all of the available DB cluster parameter group names, use the following       command:
    ///
    /// aws rds describe-db-cluster-parameter-groups --query         "DBClusterParameterGroups[].DBClusterParameterGroupName" --output text
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,

    ///
    /// The name of the DB parameter group to apply to all instances of the DB cluster.
    ///
    /// NoteWhen you apply a parameter group using the DBInstanceParameterGroupName parameter, the DB      cluster isn't rebooted automatically. Also, parameter changes are applied immediately rather than        during the next maintenance window.
    ///
    /// Default: The existing name setting
    ///
    /// Constraints:
    ///
    /// The DB parameter group must be in the same DB parameter group family as this DB cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceParameterGroupName")]
    pub dbinstance_parameter_group_name: Option<String>,

    ///
    /// A DB subnet group that you want to associate with this DB cluster.
    ///
    /// If you are restoring a DB cluster to a point in time with RestoreType set to copy-on-write, and don't       specify a DB subnet group name, then the DB cluster is restored with a default DB subnet group.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,

    ///
    /// Reserved for future use.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSystemId")]
    pub dbsystem_id: Option<String>,

    ///
    /// The name of your database. If you don't provide a name, then Amazon RDS won't create a       database in this DB cluster. For naming constraints, see Naming         Constraints in the Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,

    ///
    /// A value that indicates whether the DB cluster has deletion protection enabled.       The database can't be deleted when deletion protection is enabled. By default,       deletion protection is disabled.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,

    ///
    /// Indicates the directory ID of the Active Directory to create the DB cluster.
    ///
    /// For Amazon Aurora DB clusters, Amazon RDS can use Kerberos authentication to authenticate users that connect to the DB cluster.
    ///
    /// For more information, see Kerberos authentication       in the Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

    ///
    /// Specifies the name of the IAM role to use when making API calls to the Directory Service.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainIAMRoleName")]
    pub domain_iamrole_name: Option<String>,

    ///
    /// The list of log types that need to be enabled for exporting to CloudWatch Logs. The values       in the list depend on the DB engine being used. For more information, see                Publishing Database Logs to Amazon CloudWatch Logs in the Amazon Aurora User Guide.
    ///
    /// Aurora MySQL
    ///
    /// Valid values: audit, error, general, slowquery
    ///
    /// Aurora PostgreSQL
    ///
    /// Valid values: postgresql
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,

    ///
    /// A value that indicates whether to enable the HTTP endpoint for an Aurora Serverless DB cluster. By default, the HTTP endpoint       is disabled.
    ///
    /// When enabled, the HTTP endpoint provides a connectionless web service API for running       SQL queries on the Aurora Serverless DB cluster. You can also query your database       from inside the RDS console with the query editor.
    ///
    /// For more information, see Using the Data API for Aurora Serverless in the       Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableHttpEndpoint")]
    pub enable_http_endpoint: Option<bool>,

    ///
    /// A value that indicates whether to enable mapping of AWS Identity and Access       Management (IAM) accounts to database accounts. By default, mapping is disabled.
    ///
    /// For more information, see                IAM Database Authentication in the Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    pub enable_iamdatabase_authentication: Option<bool>,

    ///
    /// The name of the database engine to be used for this DB cluster.
    ///
    /// Valid Values:
    ///
    /// aurora (for MySQL 5.6-compatible Aurora)               aurora-mysql (for MySQL 5.7-compatible and MySQL 8.0-compatible Aurora)               aurora-postgresql               mysql               postgres
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Engine")]
    pub engine: Option<String>,

    ///
    /// The DB engine mode of the DB cluster, either provisioned, serverless,       parallelquery, global, or multimaster.
    ///
    /// The serverless engine mode only applies for Aurora Serverless v1 DB clusters.
    ///
    /// The parallelquery engine mode isn't required for Aurora MySQL version 1.23 and higher 1.x versions,       and version 2.09 and higher 2.x versions.
    ///
    /// The global engine mode isn't required for Aurora MySQL version 1.22 and higher 1.x versions,       and global engine mode isn't required for any 2.x versions.
    ///
    /// The multimaster engine mode only applies for DB clusters created with Aurora MySQL version 5.6.10a.
    ///
    /// For Aurora PostgreSQL, the global engine mode isn't required, and both the parallelquery       and the multimaster engine modes currently aren't supported.
    ///
    /// Limitations and requirements apply to some DB engine modes. For more information, see the       following sections in the Amazon Aurora User Guide:
    ///
    /// Limitations of Aurora Serverless                                               Limitations of Parallel Query                                               Limitations of Aurora Global Databases                                               Limitations of Multi-Master Clusters
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineMode")]
    pub engine_mode: Option<String>,

    ///
    /// The version number of the database engine to use.
    ///
    /// To list all of the available engine versions for aurora (for MySQL       5.6-compatible Aurora), use the following command:
    ///
    /// aws rds describe-db-engine-versions --engine aurora --query         "DBEngineVersions[].EngineVersion"
    ///
    /// To list all of the available engine versions for aurora-mysql (for MySQL       5.7-compatible Aurora), use the following command:
    ///
    /// aws rds describe-db-engine-versions --engine aurora-mysql --query         "DBEngineVersions[].EngineVersion"
    ///
    /// To list all of the available engine versions for aurora-postgresql, use       the following command:
    ///
    /// aws rds describe-db-engine-versions --engine aurora-postgresql --query         "DBEngineVersions[].EngineVersion"
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,

    ///
    /// If you are configuring an Aurora global database cluster and want your Aurora DB cluster to be a secondary member in the global database       cluster, specify the global cluster ID of the global database cluster. To define the primary database cluster of the global cluster,       use the AWS::RDS::GlobalCluster       resource.
    ///
    /// If you aren't configuring a global database cluster, don't specify this property.
    ///
    /// NoteTo remove the DB cluster from a global database cluster, specify an empty value for the GlobalClusterIdentifier property.
    ///
    /// For information about Aurora global databases, see       Working with Amazon Aurora Global Databases in the Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "GlobalClusterIdentifier")]
    pub global_cluster_identifier: Option<String>,

    ///
    /// The amount of Provisioned IOPS (input/output operations per second) to be initially allocated       for each DB instance in the Multi-AZ DB cluster.
    ///
    /// For information about valid IOPS values, see Provisioned IOPS storage in the Amazon RDS         User Guide.
    ///
    /// This setting is required to create a Multi-AZ DB cluster.
    ///
    /// Constraints: Must be a multiple between .5 and 50 of the storage amount for the DB cluster.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) of the AWS KMS key that is       used to encrypt the database instances in the DB cluster, such as         arn:aws:kms:us-east-1:012345678910:key/abcd1234-a123-456a-a12b-a123b4cd56ef.       If you enable the StorageEncrypted property but don't specify this       property, the default KMS key is used. If you specify this property, you must set the         StorageEncrypted property to true.
    ///
    /// If you specify the SnapshotIdentifier property, the StorageEncrypted property       value is inherited from the snapshot, and if the DB cluster is encrypted, the specified KmsKeyId       property is used.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// A value that indicates whether to manage the master user password with AWS Secrets Manager.
    ///
    /// For more information, see Password management with AWS Secrets Manager       in the Amazon RDS User Guide and Password management with AWS Secrets Manager       in the Amazon Aurora User Guide.
    ///
    /// Constraints:
    ///
    /// Can't manage the master user password with AWS Secrets Manager if MasterUserPassword           is specified.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManageMasterUserPassword")]
    pub manage_master_user_password: Option<bool>,

    ///
    /// The master password for the DB instance.
    ///
    /// NoteIf you specify the SourceDBClusterIdentifier, SnapshotIdentifier, or GlobalClusterIdentifier         property, don't specify this property. The value is inherited from the source DB cluster, the snapshot, or the primary DB         cluster for the global database cluster, respectively.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,

    ///
    /// Contains the secret managed by RDS in AWS Secrets Manager for the master user password.
    ///
    /// For more information, see Password management with AWS Secrets Manager       in the Amazon RDS User Guide and Password management with AWS Secrets Manager       in the Amazon Aurora User Guide.
    ///
    /// Required: No
    ///
    /// Type: MasterUserSecret
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserSecret")]
    pub master_user_secret: Option<MasterUserSecret>,

    ///
    /// The name of the master user for the DB cluster.
    ///
    /// NoteIf you specify the SourceDBClusterIdentifier, SnapshotIdentifier, or GlobalClusterIdentifier         property, don't specify this property. The value is inherited from the source DB cluster, the snapshot, or the primary DB         cluster for the global database cluster, respectively.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "MasterUsername")]
    pub master_username: Option<String>,

    ///
    /// The interval, in seconds, between points when Enhanced Monitoring metrics are collected for the DB cluster. To turn off       collecting Enhanced Monitoring metrics, specify 0. The default is 0.
    ///
    /// If MonitoringRoleArn is specified, also set MonitoringInterval       to a value other than 0.
    ///
    /// Valid Values: 0, 1, 5, 10, 15, 30, 60
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringInterval")]
    pub monitoring_interval: Option<i64>,

    ///
    /// The Amazon Resource Name (ARN) for the IAM role that permits RDS to send Enhanced Monitoring metrics to Amazon CloudWatch Logs.       An example is arn:aws:iam:123456789012:role/emaccess. For information on creating a monitoring role,       see Setting         up and enabling Enhanced Monitoring in the Amazon RDS User Guide.
    ///
    /// If MonitoringInterval is set to a value other than 0, supply a MonitoringRoleArn value.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringRoleArn")]
    pub monitoring_role_arn: Option<String>,

    ///
    /// The network type of the DB cluster.
    ///
    /// Valid values:
    ///
    /// IPV4                                 DUAL
    ///
    /// The network type is determined by the DBSubnetGroup specified for the DB cluster.       A DBSubnetGroup can support only the IPv4 protocol or the IPv4 and IPv6       protocols (DUAL).
    ///
    /// For more information, see       Working with a DB instance in a VPC in the       Amazon Aurora User Guide.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,

    ///
    /// A value that indicates whether to turn on Performance Insights for the DB cluster.
    ///
    /// For more information, see       Using Amazon Performance Insights in the Amazon RDS User Guide.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerformanceInsightsEnabled")]
    pub performance_insights_enabled: Option<bool>,

    ///
    /// The AWS KMS key identifier for encryption of Performance Insights data.
    ///
    /// The AWS KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
    ///
    /// If you don't specify a value for PerformanceInsightsKMSKeyId, then Amazon RDS       uses your default KMS key. There is a default KMS key for your AWS account.       Your AWS account has a different default KMS key for each AWS Region.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerformanceInsightsKmsKeyId")]
    pub performance_insights_kms_key_id: Option<String>,

    ///
    /// The number of days to retain Performance Insights data. The default is 7 days. The following values are valid:
    ///
    /// 7                        month * 31, where month is a number of months from 1-23               731
    ///
    /// For example, the following values are valid:
    ///
    /// 93 (3 months * 31)               341 (11 months * 31)               589 (19 months * 31)               731
    ///
    /// If you specify a retention period such as 94, which isn't a valid value, RDS issues an error.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    pub performance_insights_retention_period: Option<i64>,

    ///
    /// The port number on which the DB instances in the DB cluster accept connections.
    ///
    /// Default:
    ///
    /// When EngineMode is provisioned, 3306           (for both Aurora MySQL and Aurora PostgreSQL)               When EngineMode is serverless:                                                    3306 when Engine is aurora or                 aurora-mysql                       5432 when Engine is                 aurora-postgresql
    ///
    /// ImportantThe No interruption on update behavior only applies to DB clusters. If you are updating a         DB instance, see Port         for the AWS::RDS::DBInstance resource.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,

    ///
    /// The daily time range during which automated backups are created. For more information, see       Backup Window in the Amazon Aurora User Guide.
    ///
    /// Constraints:
    ///
    /// Must be in the format hh24:mi-hh24:mi.               Must be in Universal Coordinated Time (UTC).               Must not conflict with the preferred maintenance window.               Must be at least 30 minutes.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,

    ///
    /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// The default is a 30-minute window selected at random from an       8-hour block of time for each AWS Region, occurring on a random day of the       week. To see the time blocks available, see                Adjusting the Preferred DB Cluster Maintenance Window in the Amazon Aurora User Guide.
    ///
    /// Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.
    ///
    /// Constraints: Minimum 30-minute window.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,

    ///
    /// A value that indicates whether the DB cluster is publicly accessible.
    ///
    /// When the DB cluster is publicly accessible, its Domain Name System (DNS) endpoint       resolves to the private IP address from within the DB cluster's virtual private cloud       (VPC). It resolves to the public IP address from outside of the DB cluster's VPC. Access       to the DB cluster is ultimately controlled by the security group it uses. That public       access isn't permitted if the security group assigned to the DB cluster doesn't permit       it.
    ///
    /// When the DB cluster isn't publicly accessible, it is an internal DB cluster with a DNS name that resolves to a private IP address.
    ///
    /// Default: The default behavior varies depending on whether DBSubnetGroupName is specified.
    ///
    /// If DBSubnetGroupName isn't specified, and PubliclyAccessible isn't specified, the following applies:
    ///
    /// If the default VPC in the target Region doesn’t have an internet gateway attached to it, the DB cluster is private.               If the default VPC in the target Region has an internet gateway attached to it, the DB cluster is public.
    ///
    /// If DBSubnetGroupName is specified, and PubliclyAccessible isn't specified, the following applies:
    ///
    /// If the subnets are part of a VPC that doesn’t have an internet gateway attached to it, the DB cluster is private.               If the subnets are part of a VPC that has an internet gateway attached to it, the DB cluster is public.
    ///
    /// Valid for: Multi-AZ DB clusters only
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) of the source DB instance or DB cluster if this DB       cluster is created as a read replica.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationSourceIdentifier")]
    pub replication_source_identifier: Option<String>,

    ///
    /// The date and time to restore the DB cluster to.
    ///
    /// Valid Values: Value must be a time in Universal Coordinated Time (UTC) format
    ///
    /// Constraints:
    ///
    /// Must be before the latest restorable time for the DB instance               Must be specified if UseLatestRestorableTime parameter isn't provided               Can't be specified if the UseLatestRestorableTime parameter is enabled               Can't be specified if the RestoreType parameter is copy-on-write
    ///
    /// Example: 2015-03-07T23:45:00Z
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestoreToTime")]
    pub restore_to_time: Option<DBClusterRestoreToTimeEnum>,

    ///
    /// The type of restore to be performed. You can specify one of the following values:
    ///
    /// full-copy - The new DB cluster is restored as a full copy of the         source DB cluster.                        copy-on-write - The new DB cluster is restored as a clone of the         source DB cluster.
    ///
    /// Constraints: You can't specify copy-on-write if the engine version of the source DB cluster is earlier than 1.11.
    ///
    /// If you don't specify a RestoreType value, then the new DB cluster is       restored as a full copy of the source DB cluster.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestoreType")]
    pub restore_type: Option<String>,

    ///
    /// The ScalingConfiguration property type specifies the scaling       configuration of an Aurora Serverless DB cluster.
    ///
    /// This property is only supported for Aurora Serverless v1. For Aurora Serverless v2, use ServerlessV2ScalingConfiguration property.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: ScalingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingConfiguration")]
    pub scaling_configuration: Option<ScalingConfiguration>,

    ///
    /// The ServerlessV2ScalingConfiguration property type specifies the scaling configuration of an Aurora Serverless V2 DB cluster.
    ///
    /// This property is only supported for Aurora Serverless v2. For Aurora Serverless v1, use ScalingConfiguration property.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: ServerlessV2ScalingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,

    ///
    /// The identifier for the DB snapshot or DB cluster snapshot to restore       from.
    ///
    /// You can use either the name or the Amazon Resource Name (ARN) to specify a DB       cluster snapshot. However, you can use only the ARN to specify a DB snapshot.
    ///
    /// After you restore a DB cluster with a SnapshotIdentifier property, you       must specify the same SnapshotIdentifier property for any future updates to       the DB cluster. When you specify this property for an update, the DB cluster is not       restored from the snapshot again, and the data in the database is not changed. However,       if you don't specify the SnapshotIdentifier property, an empty DB cluster       is created, and the original DB cluster is deleted. If you specify a property that is       different from the previous snapshot restore property, a new DB cluster is restored from       the specified SnapshotIdentifier property, and the original DB cluster is       deleted.
    ///
    /// If you specify the SnapshotIdentifier property to restore a DB cluster (as opposed to specifying it for DB cluster updates),       then don't specify the following properties:
    ///
    /// GlobalClusterIdentifier               MasterUsername               MasterUserPassword               ReplicationSourceIdentifier               RestoreType               SourceDBClusterIdentifier               SourceRegion               StorageEncrypted (for an encrypted snapshot)               UseLatestRestorableTime
    ///
    /// Constraints:
    ///
    /// Must match the identifier of an existing Snapshot.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,

    ///
    /// When restoring a DB cluster to a point in time, the identifier of the source DB cluster from which to restore.
    ///
    /// Constraints:
    ///
    /// Must match the identifier of an existing DBCluster.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,

    ///
    /// The AWS Region which contains the source DB cluster when replicating a DB cluster. For       example, us-east-1.
    ///
    /// Valid for: Aurora DB clusters only
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceRegion")]
    pub source_region: Option<String>,

    ///
    /// Indicates whether the DB cluster is encrypted.
    ///
    /// If you specify the KmsKeyId property, then you must enable encryption.
    ///
    /// If you specify the SourceDBClusterIdentifier property, don't specify this property. The       value is inherited from the source DB cluster, and if the DB cluster is encrypted, the specified       KmsKeyId property is used.
    ///
    /// If you specify the SnapshotIdentifier and the specified snapshot is encrypted,       don't specify this property. The value is inherited from the snapshot, and the specified KmsKeyId       property is used.
    ///
    /// If you specify the SnapshotIdentifier and the specified snapshot isn't encrypted, you can use this property       to specify that the restored DB cluster is encrypted. Specify the KmsKeyId property for the KMS key       to use for encryption. If you don't want the restored DB cluster to be encrypted, then don't set this property       or set it to false.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,

    ///
    /// Specifies the storage type to be associated with the DB cluster.
    ///
    /// This setting is required to create a Multi-AZ DB cluster.
    ///
    /// When specified for a Multi-AZ DB cluster, a value for the Iops parameter is required.
    ///
    /// Valid values: aurora, aurora-iopt1 (Aurora DB clusters); io1 (Multi-AZ DB clusters)
    ///
    /// Default: aurora (Aurora DB clusters); io1 (Multi-AZ DB clusters)
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,

    ///
    /// An optional array of key-value pairs to apply to this DB cluster.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// A value that indicates whether to restore the DB cluster to the latest restorable       backup time. By default, the DB cluster is not restored to the latest restorable backup       time.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,

    ///
    /// A list of EC2 VPC security groups to associate with this DB cluster.
    ///
    /// If you plan to update the resource, don't specify VPC security groups in a shared VPC.
    ///
    /// Valid for: Aurora DB clusters and Multi-AZ DB clusters
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DBClusterRestoreToTimeEnum {
    /// Value must be a time in Universal Coordinated Time (UTC) format
    #[serde(rename = "Value must be a time in Universal Coordinated Time (UTC) format")]
    Valuemustbeatimeinuniversalcoordinatedtimeutcformat,
}

impl Default for DBClusterRestoreToTimeEnum {
    fn default() -> Self {
        DBClusterRestoreToTimeEnum::Valuemustbeatimeinuniversalcoordinatedtimeutcformat
    }
}

impl cfn_resources::CfnResource for CfnDBCluster {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::DBCluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.master_user_secret
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.scaling_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.serverless_v2_scaling_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an AWS Identity and Access Management (IAM) role that is associated with a DB cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DBClusterRole {
    ///
    /// The name of the feature associated with the AWS Identity and Access Management (IAM)       role. IAM roles that are associated with a DB cluster grant permission for the DB       cluster to access other AWS services on your behalf. For the list of supported feature       names, see the SupportedFeatureNames description in DBEngineVersion       in the Amazon RDS API Reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureName")]
    pub feature_name: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that is associated with the DB cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

impl cfn_resources::CfnResource for DBClusterRole {
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

/// Specifies the connection endpoint for the primary instance of the DB cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Endpoint {
    ///
    /// Specifies the connection endpoint for the primary instance of the DB cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,

    ///
    /// Specifies the port that the database engine is listening on.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<String>,
}

impl cfn_resources::CfnResource for Endpoint {
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

/// The MasterUserSecret return value specifies the secret managed by RDS in AWS Secrets Manager for the master user password.
///
/// For more information, see Password management with AWS Secrets Manager       in the Amazon RDS User Guide and Password management with AWS Secrets Manager       in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MasterUserSecret {
    ///
    /// The AWS KMS key identifier that is used to encrypt the secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
}

impl cfn_resources::CfnResource for MasterUserSecret {
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

/// The ReadEndpoint return value specifies the reader endpoint for the DB cluster.
///
/// The reader endpoint for a DB cluster load-balances connections across the Aurora Replicas that are available in a DB cluster.          As clients request new connections to the reader endpoint, Aurora distributes the connection requests among the Aurora Replicas          in the DB cluster. This functionality can help balance your read workload across multiple Aurora Replicas in your DB cluster.
///
/// If a failover occurs, and the Aurora Replica that you are connected to is promoted to be the primary instance, your connection          is dropped. To continue sending your read workload to other Aurora Replicas in the cluster, you can then reconnect to the          reader endpoint.
///
/// For more information about Aurora endpoints, see Amazon Aurora connection management          in the Amazon Aurora User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReadEndpoint {
    ///
    /// The host address of the reader endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,
}

impl cfn_resources::CfnResource for ReadEndpoint {
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

/// The ScalingConfiguration property type specifies the scaling       configuration of an Aurora Serverless DB cluster.
///
/// For more information, see Using Amazon Aurora         Serverless in the Amazon Aurora User Guide.
///
/// This property is only supported for Aurora Serverless v1. For Aurora Serverless v2, use ServerlessV2ScalingConfiguration property.
///
/// Valid for: Aurora DB clusters only
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingConfiguration {
    ///
    /// A value that indicates whether to allow or disallow automatic pause for an Aurora DB cluster in serverless DB engine mode.       A DB cluster can be paused only when it's idle (it has no connections).
    ///
    /// NoteIf a DB cluster is paused for more than seven days, the DB cluster might be backed up with a snapshot.         In this case, the DB cluster is restored when there is a request to connect to it.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoPause")]
    pub auto_pause: Option<bool>,

    ///
    /// The maximum capacity for an Aurora DB cluster in serverless DB engine mode.
    ///
    /// For Aurora MySQL, valid capacity values are 1, 2, 4, 8, 16, 32, 64, 128, and 256.
    ///
    /// For Aurora PostgreSQL, valid capacity values are 2, 4, 8, 16, 32, 64, 192, and 384.
    ///
    /// The maximum capacity must be greater than or equal to the minimum capacity.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<i64>,

    ///
    /// The minimum capacity for an Aurora DB cluster in serverless DB engine mode.
    ///
    /// For Aurora MySQL, valid capacity values are 1, 2, 4, 8, 16, 32, 64, 128, and 256.
    ///
    /// For Aurora PostgreSQL, valid capacity values are 2, 4, 8, 16, 32, 64, 192, and 384.
    ///
    /// The minimum capacity must be less than or equal to the maximum capacity.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: Option<i64>,

    ///
    /// The amount of time, in seconds, that Aurora Serverless v1 tries to find a scaling point       to perform seamless scaling before enforcing the timeout action. The default is 300.
    ///
    /// Specify a value between 60 and 600 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondsBeforeTimeout")]
    pub seconds_before_timeout: Option<i64>,

    ///
    /// The time, in seconds, before an Aurora DB cluster in serverless mode is paused.
    ///
    /// Specify a value between 300 and 86,400 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondsUntilAutoPause")]
    pub seconds_until_auto_pause: Option<i64>,

    ///
    /// The action to take when the timeout is reached, either ForceApplyCapacityChange or RollbackCapacityChange.
    ///
    /// ForceApplyCapacityChange sets the capacity to the specified value as soon as possible.
    ///
    /// RollbackCapacityChange, the default, ignores the capacity change if a scaling point isn't found in the timeout period.
    ///
    /// ImportantIf you specify ForceApplyCapacityChange, connections that         prevent Aurora Serverless v1 from finding a scaling point might be dropped.
    ///
    /// For more information, see           Autoscaling for Aurora Serverless v1 in the Amazon Aurora User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutAction")]
    pub timeout_action: Option<String>,
}

impl cfn_resources::CfnResource for ScalingConfiguration {
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

/// The ServerlessV2ScalingConfiguration property type specifies the scaling       configuration of an Aurora Serverless V2 DB cluster.
///
/// For more information, see Using Amazon Aurora Serverless v2 in the       Amazon Aurora User Guide.
///
/// If you have an Aurora cluster, you must set the ScalingConfigurationInfo attribute before you add a DB instance that uses the        db.serverless DB instance class. For more information, see        Clusters that use Aurora Serverless v2 must have a capacity range specified in the        Amazon Aurora User Guide.
///
/// This property is only supported for Aurora Serverless v2. For Aurora Serverless v1, use ScalingConfiguration property.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerlessV2ScalingConfiguration {
    ///
    /// The maximum number of Aurora capacity units (ACUs) for a DB instance in an Aurora Serverless v2 cluster.       You can specify ACU values in half-step increments, such as 40, 40.5, 41, and so on. The largest value       that you can use is 128.
    ///
    /// The maximum capacity must be higher than 0.5 ACUs. For more information, see        Choosing the maximum Aurora Serverless v2 capacity setting for a cluster in the        Amazon Aurora User Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,

    ///
    /// The minimum number of Aurora capacity units (ACUs) for a DB instance in an Aurora Serverless v2 cluster.       You can specify ACU values in half-step increments, such as 8, 8.5, 9, and so on. The smallest value       that you can use is 0.5.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: Option<f64>,
}

impl cfn_resources::CfnResource for ServerlessV2ScalingConfiguration {
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

impl cfn_resources::CfnResource for Tag {
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
