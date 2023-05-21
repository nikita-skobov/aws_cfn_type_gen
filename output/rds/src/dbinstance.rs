

/// The AWS::RDS::DBInstance resource creates an Amazon DB instance. The new DB instance can be an RDS DB instance, or it can be a DB instance       in an Aurora DB cluster.
///
/// For more information about creating an RDS DB instance, see Creating an Amazon       RDS DB instance in the Amazon RDS User Guide.
///
/// For more information about creating a DB instance in an Aurora DB cluster, see       Creating an Amazon Aurora DB cluster in the Amazon Aurora User Guide.
///
/// If you import an existing DB instance, and the template configuration doesn't match the actual configuration of the DB instance,       AWS CloudFormation applies the changes in the template during the import operation.
///
/// Updating DB instances
///
/// When properties labeled "Update requires:       Replacement"       are updated, AWS CloudFormation first creates a replacement DB       instance, then changes references from other dependent resources to point to the       replacement DB instance, and finally deletes the old DB instance.
///
/// For more information about updating other properties of this resource, see         ModifyDBInstance. For more information about updating stacks,       see AWS         CloudFormation Stacks Updates.
///
/// Deleting DB instances
///
/// For DB instances that are part of an Aurora DB cluster, you can set a deletion policy       for your DB instance to control how AWS CloudFormation handles the DB instance when the       stack is deleted. For Amazon RDS DB instances, you can choose to         retain the DB instance, to delete the DB       instance, or to create a snapshot of the DB instance. The default       AWS CloudFormation behavior depends on the DBClusterIdentifier       property:
///
/// For more information, see DeletionPolicy Attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBInstance {


    /// 
    /// The amount of storage in gibibytes (GiB) to be initially allocated for the database       instance.
    /// 
    /// NoteIf any value is set in the Iops parameter,           AllocatedStorage must be at least 100 GiB, which corresponds to the         minimum Iops value of 1,000. If you increase the Iops value (in 1,000         IOPS increments), then you must also increase the AllocatedStorage         value (in 100-GiB increments).
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Aurora cluster volumes automatically grow as the amount of data in your       database increases, though you are only charged for the space that you use in an Aurora cluster volume.
    /// 
    /// MySQL
    /// 
    /// Constraints to the amount of storage for each storage type are the following:
    /// 
    /// General Purpose (SSD) storage (gp2): Must be an integer from 20 to 65536.Provisioned IOPS storage (io1): Must be an integer from 100 to 65536.Magnetic storage (standard): Must be an integer from 5 to 3072.
    /// 
    /// MariaDB
    /// 
    /// Constraints to the amount of storage for each storage type are the following:
    /// 
    /// General Purpose (SSD) storage (gp2): Must be an integer from 20 to 65536.Provisioned IOPS storage (io1): Must be an integer from 100 to 65536.Magnetic storage (standard): Must be an integer from 5 to 3072.
    /// 
    /// PostgreSQL
    /// 
    /// Constraints to the amount of storage for each storage type are the following:
    /// 
    /// General Purpose (SSD) storage (gp2): Must be an integer from 20 to 65536.Provisioned IOPS storage (io1): Must be an integer from 100 to 65536.Magnetic storage (standard): Must be an integer from 5 to 3072.
    /// 
    /// Oracle
    /// 
    /// Constraints to the amount of storage for each storage type are the following:
    /// 
    /// General Purpose (SSD) storage (gp2): Must be an integer from 20 to 65536.Provisioned IOPS storage (io1): Must be an integer from 100 to 65536.Magnetic storage (standard): Must be an integer from 10 to 3072.
    /// 
    /// SQL Server
    /// 
    /// Constraints to the amount of storage for each storage type are the following:
    /// 
    /// General Purpose (SSD) storage (gp2):                                       Enterprise and Standard editions: Must be an integer from 20 to 16384.Web and Express editions: Must be an integer from 20 to 16384.             Provisioned IOPS storage (io1):                                       Enterprise and Standard editions: Must be an integer from 20 to 16384.Web and Express editions: Must be an integer from 20 to 16384.             Magnetic storage (standard):                                       Enterprise and Standard editions: Must be an integer from 20 to 1024.Web and Express editions: Must be an integer from 20 to 1024.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocatedStorage")]
    pub allocated_storage: Option<String>,


    /// 
    /// A value that indicates whether major version upgrades are allowed. Changing this       parameter doesn't result in an outage and the change is asynchronously applied as soon       as possible.
    /// 
    /// Constraints: Major version upgrades must be allowed when specifying a value for the         EngineVersion parameter that is a different major version than the DB       instance's current version.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: Option<bool>,


    /// 
    /// The AWS Identity and Access Management (IAM) roles associated with the DB instance.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The associated roles are managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of DBInstanceRole
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatedRoles")]
    pub associated_roles: Option<Vec<DBInstanceRole>>,


    /// 
    /// A value that indicates whether minor engine upgrades are applied automatically to the       DB instance during the maintenance window. By default, minor engine upgrades are applied       automatically.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,


    /// 
    /// The Availability Zone (AZ) where the database will be created. For information on     AWS Regions and Availability Zones, see     Regions     and Availability Zones.
    /// 
    /// Amazon Aurora
    /// 
    /// Each Aurora DB cluster hosts copies of its storage in three separate Availability Zones. Specify one of these       Availability Zones. Aurora automatically chooses an appropriate Availability Zone if you don't specify one.
    /// 
    /// Default: A random, system-chosen Availability Zone in the endpoint's AWS Region.
    /// 
    /// Example: us-east-1d
    /// 
    /// Constraint: The AvailabilityZone parameter can't be specified if the DB instance is a Multi-AZ deployment.       The specified Availability Zone must be in the same AWS Region as the current endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The number of days for which automated backups are retained. Setting this parameter to       a positive number enables backups. Setting this parameter to 0 disables automated       backups.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The retention period for automated backups is managed by the DB       cluster.
    /// 
    /// Default: 1
    /// 
    /// Constraints:
    /// 
    /// Must be a value from 0 to 35               Can't be set to 0 if the DB instance is a source to read replicas
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<i64>,


    /// 
    /// The identifier of the CA certificate for this DB instance.
    /// 
    /// NoteSpecifying or updating this property triggers a reboot.
    /// 
    /// For more information about CA certificate identifiers for RDS DB engines, see         Rotating Your SSL/TLS Certificate in the Amazon RDS User Guide.
    /// 
    /// For more information about CA certificate identifiers for Aurora DB engines, see                 Rotating Your SSL/TLS Certificate in the Amazon Aurora User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CACertificateIdentifier")]
    pub cacertificate_identifier: Option<String>,


    /// 
    /// The details of the DB instance's server certificate.
    /// 
    /// Required: No
    ///
    /// Type: CertificateDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateDetails")]
    pub certificate_details: Option<CertificateDetails>,


    /// 
    /// A value that indicates whether the DB instance is restarted when you rotate your       SSL/TLS certificate.
    /// 
    /// By default, the DB instance is restarted when you rotate your SSL/TLS certificate. The certificate       is not updated until the DB instance is restarted.
    /// 
    /// ImportantSet this parameter only if you are not using SSL/TLS to connect to the DB instance.
    /// 
    /// If you are using SSL/TLS to connect to the DB instance, follow the appropriate instructions for your       DB engine to rotate your SSL/TLS certificate:
    /// 
    /// For more information about rotating your SSL/TLS certificate for RDS DB engines, see                        Rotating Your SSL/TLS Certificate. in the Amazon RDS User Guide.                       For more information about rotating your SSL/TLS certificate for Aurora DB engines, see                        Rotating Your SSL/TLS Certificate in the Amazon Aurora User Guide.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateRotationRestart")]
    pub certificate_rotation_restart: Option<bool>,


    /// 
    /// For supported engines, indicates that the DB instance should be associated with the       specified character set.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The character set is managed by the DB cluster. For more information,       see AWS::RDS::DBCluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CharacterSetName")]
    pub character_set_name: Option<String>,


    /// 
    /// A value that indicates whether to copy tags from the DB instance to snapshots of the DB instance. By default, tags are not copied.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Copying tags to snapshots is managed by the DB cluster. Setting this      value for an Aurora DB instance has no effect on the DB cluster setting.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,


    /// 
    /// The instance profile associated with the underlying Amazon EC2 instance of an       RDS Custom DB instance. The instance profile must meet the following requirements:
    /// 
    /// The profile must exist in your account.               The profile must have an IAM role that Amazon EC2 has permissions to assume.               The instance profile name and the associated IAM role name must start with the prefix AWSRDSCustom.
    /// 
    /// For the list of permissions required for the IAM role, see                Configure IAM and your VPC in the Amazon RDS User Guide.
    /// 
    /// This setting is required for RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomIAMInstanceProfile")]
    pub custom_iaminstance_profile: Option<String>,


    /// 
    /// The identifier of the DB cluster that the instance will belong to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,


    /// 
    /// The identifier for the RDS for MySQL Multi-AZ DB cluster snapshot to restore from.
    /// 
    /// For more information on Multi-AZ DB clusters, see Multi-AZ DB         cluster deployments in the Amazon RDS User       Guide.
    /// 
    /// Constraints:
    /// 
    /// Must match the identifier of an existing Multi-AZ DB cluster snapshot.               Can't be specified when DBSnapshotIdentifier is specified.               Must be specified when DBSnapshotIdentifier isn't specified.               If you are restoring from a shared manual Multi-AZ DB cluster snapshot, the DBClusterSnapshotIdentifier           must be the ARN of the shared snapshot.               Can't be the identifier of an Aurora DB cluster snapshot.               Can't be the identifier of an RDS for PostgreSQL Multi-AZ DB cluster snapshot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    pub dbcluster_snapshot_identifier: Option<String>,


    /// 
    /// The compute and memory capacity of the DB instance, for example,         db.m4.large. Not all DB instance classes are available in all AWS       Regions, or for all database engines.
    /// 
    /// For the full list of DB instance classes, and availability for your engine, see       DB Instance         Class in the Amazon RDS User Guide. For more information about DB instance class         pricing and AWS Region support for DB instance classes, see         Amazon RDS Pricing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: Option<String>,


    /// 
    /// A name for the DB instance. If you specify a name, AWS CloudFormation converts it to       lowercase. If you don't specify a name, AWS CloudFormation generates a unique physical       ID and uses that ID for the DB instance. For more information, see Name       Type.
    /// 
    /// For information about constraints that apply to DB instance identifiers, see         Naming constraints in           Amazon RDS in the Amazon RDS User Guide.
    /// 
    /// ImportantIf you specify a name, you can't perform updates that require replacement of this         resource. You can perform updates that require no or some interruption. If you must         replace the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: Option<String>,


    /// 
    /// The meaning of this parameter differs according to the database engine you use.
    /// 
    /// ImportantIf you specify the         DBSnapshotIdentifier property, this property only applies to RDS for Oracle.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The database name is managed by the DB cluster.
    /// 
    /// MySQL
    /// 
    /// The name of the database to create when the DB instance is created. If this parameter       is not specified, no database is created in the DB instance.
    /// 
    /// Constraints:
    /// 
    /// Must contain 1 to 64 letters or numbers.               Can't be a word reserved by the specified database engine
    /// 
    /// MariaDB
    /// 
    /// The name of the database to create when the DB instance is created. If this parameter       is not specified, no database is created in the DB instance.
    /// 
    /// Constraints:
    /// 
    /// Must contain 1 to 64 letters or numbers.               Can't be a word reserved by the specified database engine
    /// 
    /// PostgreSQL
    /// 
    /// The name of the database to create when the DB instance is created. If this parameter       is not specified, the default postgres database is created in the DB instance.
    /// 
    /// Constraints:
    /// 
    /// Must begin with a letter. Subsequent characters can be           letters, underscores, or digits (0-9).               Must contain 1 to 63 characters.               Can't be a word reserved by the specified database engine
    /// 
    /// Oracle
    /// 
    /// The Oracle System ID (SID) of the created DB instance. If you specify         null, the default value ORCL is used. You can't specify       the string NULL, or any other reserved word, for DBName.
    /// 
    /// Default: ORCL
    /// 
    /// Constraints:
    /// 
    /// Can't be longer than 8 characters
    /// 
    /// SQL Server
    /// 
    /// Not applicable. Must be null.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBName")]
    pub dbname: Option<String>,


    /// 
    /// The name of an existing DB parameter group or a reference to an AWS::RDS::DBParameterGroup resource created in the template.
    /// 
    /// To list all of the available DB parameter group names, use the following       command:
    /// 
    /// aws rds describe-db-parameter-groups --query         "DBParameterGroups[].DBParameterGroupName" --output text
    /// 
    /// NoteIf any of the data members of the referenced parameter group are changed during an         update, the DB instance might need to be restarted, which causes some interruption.         If the parameter group contains static parameters, whether they were changed or not,         an update triggers a reboot.
    /// 
    /// If you don't specify a value for DBParameterGroupName property,      the default DB parameter group for the specified engine and engine version is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DBParameterGroupName")]
    pub dbparameter_group_name: Option<String>,


    /// 
    /// A list of the DB security groups to assign to the DB instance. The list can include       both the name of existing DB security groups or references to AWS::RDS::DBSecurityGroup       resources created in the template.
    /// 
    /// If you set DBSecurityGroups, you must not set VPCSecurityGroups, and vice versa.       Also, note that the DBSecurityGroups property exists only for backwards compatibility with older       regions and is no longer recommended for providing security information to an RDS DB       instance. Instead, use VPCSecurityGroups.
    /// 
    /// ImportantIf you specify this property, AWS CloudFormation sends only the following         properties (if specified) to Amazon RDS during create operations:                                                                                                                                                                                                     AllocatedStorage                   AutoMinorVersionUpgrade                   AvailabilityZone                   BackupRetentionPeriod                   CharacterSetName                   DBInstanceClass                   DBName                   DBParameterGroupName                   DBSecurityGroups                   DBSubnetGroupName                   Engine                   EngineVersion                   Iops                   LicenseModel                   MasterUsername                   MasterUserPassword                   MultiAZ                   OptionGroupName                   PreferredBackupWindow                   PreferredMaintenanceWindow         All other properties are ignored. Specify a virtual private cloud (VPC) security         group if you want to submit other properties, such as StorageType,           StorageEncrypted, or KmsKeyId. If you're already using         the DBSecurityGroups property, you can't use these other properties by         updating your DB instance to use a VPC security group. You must recreate the DB         instance.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBSecurityGroups")]
    pub dbsecurity_groups: Option<Vec<String>>,


    /// 
    /// The name or Amazon Resource Name (ARN) of the DB snapshot that's used to restore the       DB instance. If you're restoring from a shared manual DB snapshot, you must specify the       ARN of the snapshot.
    /// 
    /// By specifying this property, you can create a DB instance from the specified DB       snapshot. If the DBSnapshotIdentifier property is an empty string or the         AWS::RDS::DBInstance declaration has no         DBSnapshotIdentifier property, AWS CloudFormation creates a new       database. If the property contains a value (other than an empty string), AWS       CloudFormation creates a database from the specified snapshot. If a snapshot with the       specified name doesn't exist, AWS CloudFormation can't create the database and it rolls       back the stack.
    /// 
    /// Some DB instance properties aren't valid when you restore from a snapshot, such as the         MasterUsername and MasterUserPassword properties. For       information about the properties that you can specify, see the         RestoreDBInstanceFromDBSnapshot action in the Amazon RDS API         Reference.
    /// 
    /// After you restore a DB instance with a DBSnapshotIdentifier property, you       must specify the same DBSnapshotIdentifier property for any future updates       to the DB instance. When you specify this property for an update, the DB instance is not       restored from the DB snapshot again, and the data in the database is not changed.       However, if you don't specify the DBSnapshotIdentifier property, an empty       DB instance is created, and the original DB instance is deleted. If you specify a       property that is different from the previous snapshot restore property, a new DB       instance is restored from the specified DBSnapshotIdentifier property, and       the original DB instance is deleted.
    /// 
    /// If you specify the DBSnapshotIdentifier property to restore a DB instance (as opposed to specifying it for DB instance updates),       then don't specify the following properties:
    /// 
    /// CharacterSetName               DBClusterIdentifier               DBName               DeleteAutomatedBackups               EnablePerformanceInsights               KmsKeyId               MasterUsername               MasterUserPassword               PerformanceInsightsKMSKeyId               PerformanceInsightsRetentionPeriod               PromotionTier               SourceDBInstanceIdentifier               SourceRegion               StorageEncrypted (for an encrypted snapshot)               Timezone
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Snapshot restore is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DBSnapshotIdentifier")]
    pub dbsnapshot_identifier: Option<String>,


    /// 
    /// A DB subnet group to associate with the DB instance. If you update this value, the new       subnet group must be a subnet group in a new VPC.
    /// 
    /// If there's no DB subnet group, then the DB instance isn't a VPC DB instance.
    /// 
    /// For more information about using Amazon RDS in a VPC, see Using Amazon RDS with Amazon Virtual       Private Cloud (VPC) in the Amazon RDS User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The DB subnet group is managed by the DB cluster. If specified, the setting must match the DB cluster setting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,


    /// 
    /// A value that indicates whether to remove automated backups immediately after the DB       instance is deleted. This parameter isn't case-sensitive. The default is to remove       automated backups immediately after the DB instance is deleted.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered.       Manual DB cluster snapshots of the DB cluster are not deleted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteAutomatedBackups")]
    pub delete_automated_backups: Option<bool>,


    /// 
    /// A value that indicates whether the DB instance has deletion protection enabled.       The database can't be deleted when deletion protection is enabled. By default,       deletion protection is disabled. For more information, see                Deleting a DB Instance.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. You can enable or disable deletion protection for the DB cluster.       For more information, see CreateDBCluster. DB instances in a DB       cluster can be deleted even when deletion protection is enabled for the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,


    /// 
    /// The Active Directory directory ID to create the DB instance in. Currently, only Microsoft SQL Server, Oracle, and PostgreSQL DB instances can be created in an Active Directory Domain.
    /// 
    /// For more information, see       Kerberos Authentication in the Amazon RDS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// 
    /// Specify the name of the IAM role to be used when making API calls to the Directory Service.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The domain is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainIAMRoleName")]
    pub domain_iamrole_name: Option<String>,


    /// 
    /// The list of log types that need to be enabled for exporting to CloudWatch Logs. The values       in the list depend on the DB engine being used. For more information, see       Publishing Database Logs to Amazon CloudWatch Logs in the Amazon Relational Database           Service User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. CloudWatch Logs exports are managed by the DB cluster.
    /// 
    /// MariaDB
    /// 
    /// Valid values: audit, error, general, slowquery
    /// 
    /// Microsoft SQL Server
    /// 
    /// Valid values: agent, error
    ///
    /// MySQL
    /// 
    /// Valid values: audit, error, general, slowquery
    /// 
    /// Oracle
    /// 
    /// Valid values: alert, audit, listener, trace
    /// 
    /// PostgreSQL
    /// 
    /// Valid values: postgresql, upgrade
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,


    /// 
    /// A value that indicates whether to enable mapping of AWS Identity and Access       Management (IAM) accounts to database accounts. By default, mapping is disabled.
    /// 
    /// This property is supported for RDS for MariaDB, RDS for MySQL, and RDS for PostgreSQL.        For more information, see       IAM Database Authentication for MariaDB, MySQL, and PostgreSQL in the Amazon RDS User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Mapping AWS IAM accounts to database accounts is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    pub enable_iamdatabase_authentication: Option<bool>,


    /// 
    /// A value that indicates whether to enable Performance Insights for the DB instance. For more information, see       Using Amazon Performance Insights in the Amazon RDS User Guide.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePerformanceInsights")]
    pub enable_performance_insights: Option<bool>,


    /// 
    /// Specifies the connection endpoint.
    /// 
    /// NoteThe endpoint might not be shown for instances whose status is creating.
    /// 
    /// Required: No
    ///
    /// Type: Endpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,


    /// 
    /// The name of the database engine that you want to use for this DB instance.
    /// 
    /// NoteWhen you are creating a DB instance, the Engine property is required.
    /// 
    /// Valid Values:
    /// 
    /// aurora (for MySQL 5.6-compatible Aurora)               aurora-mysql (for MySQL 5.7-compatible and MySQL 8.0-compatible Aurora)               aurora-postgresql                        custom-oracle-ee               custom-oracle-ee-cdb               custom-sqlserver-ee              custom-sqlserver-se                custom-sqlserver-web               mariadb               mysql               oracle-ee               oracle-ee-cdb               oracle-se2               oracle-se2-cdb               postgres               sqlserver-ee               sqlserver-se               sqlserver-ex               sqlserver-web
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Engine")]
    pub engine: Option<String>,


    /// 
    /// The version number of the database engine to use.
    /// 
    /// For a list of valid engine versions, use the DescribeDBEngineVersions action.
    /// 
    /// The following are the database engines and links to information about the major and minor versions that are available with       Amazon RDS. Not every database engine is available for every AWS Region.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The version number of the database engine to be used by the DB       instance is managed by the DB cluster.
    /// 
    /// MariaDB
    /// 
    /// See MariaDB on Amazon RDS Versions in the       Amazon RDS User Guide.
    /// 
    /// Microsoft SQL Server
    /// 
    /// See Microsoft SQL Server Versions on Amazon RDS in the       Amazon RDS User Guide.
    /// 
    /// MySQL
    /// 
    /// See MySQL on Amazon RDS Versions in the       Amazon RDS User Guide.
    /// 
    /// Oracle
    /// 
    /// See Oracle Database Engine Release Notes in the       Amazon RDS User Guide.
    /// 
    /// PostgreSQL
    /// 
    /// See Supported PostgreSQL Database Versions in the       Amazon RDS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,


    /// 
    /// The number of I/O operations per second (IOPS) that the database provisions. The value       must be equal to or greater than 1000.
    /// 
    /// If you specify this property, you must follow the range of allowed ratios of your       requested IOPS rate to the amount of storage that you allocate (IOPS to allocated       storage). For example, you can provision an Oracle database instance with 1000 IOPS and       200 GiB of storage (a ratio of 5:1), or specify 2000 IOPS with 200 GiB of storage (a ratio       of 10:1). For more information, see Amazon RDS         Provisioned IOPS Storage to Improve Performance in the Amazon RDS         User Guide.
    /// 
    /// NoteIf you specify io1 for the StorageType property, then         you must also specify the Iops property.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// The ARN of the AWS KMS key that's used to encrypt       the DB instance, such as         arn:aws:kms:us-east-1:012345678910:key/abcd1234-a123-456a-a12b-a123b4cd56ef.       If you enable the StorageEncrypted property but don't specify this property, AWS       CloudFormation uses the default KMS key. If you specify this property, you must set       the StorageEncrypted property to true.
    /// 
    /// If you specify the SourceDBInstanceIdentifier property, the value is       inherited from the source DB instance if the read replica is created in the same       region.
    /// 
    /// If you create an encrypted read replica in a different AWS Region, then you must       specify a KMS key for the destination AWS Region. KMS encryption keys are specific to       the region that they're created in, and you can't use encryption keys from one region in       another region.
    /// 
    /// If you specify the SnapshotIdentifier property, the StorageEncrypted property       value is inherited from the snapshot, and if the DB instance is encrypted, the specified       KmsKeyId property is used.
    /// 
    /// If you specify DBSecurityGroups, AWS CloudFormation ignores this       property. To specify both a security group and this property, you must use a VPC       security group. For more information about Amazon RDS and VPC, see Using Amazon RDS with Amazon VPC       in the Amazon RDS User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The KMS key identifier is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// License model information for this DB instance.
    /// 
    /// Valid values:
    /// 
    /// Aurora MySQL - general-public-license               Aurora PostgreSQL - postgresql-license               MariaDB - general-public-license               Microsoft SQL Server - license-included               MySQL - general-public-license               Oracle - bring-your-own-license or license-included               PostgreSQL - postgresql-license
    /// 
    /// NoteIf you've specified DBSecurityGroups and then you update the license         model, AWS CloudFormation replaces the underlying DB instance. This will incur some         interruptions to database availability.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseModel")]
    pub license_model: Option<String>,


    /// 
    /// A value that indicates whether to manage the master user password with AWS Secrets Manager.
    /// 
    /// For more information, see Password management with AWS Secrets Manager       in the Amazon RDS User Guide.
    /// 
    /// Constraints:
    /// 
    /// Can't manage the master user password with AWS Secrets Manager if MasterUserPassword           is specified.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManageMasterUserPassword")]
    pub manage_master_user_password: Option<bool>,


    /// 
    /// The password for the master user. The password can include any printable ASCII       character except "/", """, or "@".
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The password for the master user is managed by the DB cluster.
    /// 
    /// MariaDB
    /// 
    /// Constraints: Must contain from 8 to 41 characters.
    /// 
    /// Microsoft SQL Server
    /// 
    /// Constraints: Must contain from 8 to 128 characters.
    /// 
    /// MySQL
    /// 
    /// Constraints: Must contain from 8 to 41 characters.
    /// 
    /// Oracle
    /// 
    /// Constraints: Must contain from 8 to 30 characters.
    /// 
    /// PostgreSQL
    /// 
    /// Constraints: Must contain from 8 to 128 characters.
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
    /// For more information, see Password management with AWS Secrets Manager       in the Amazon RDS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: MasterUserSecret
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserSecret")]
    pub master_user_secret: Option<MasterUserSecret>,


    /// 
    /// The master user name for the DB instance.
    /// 
    /// NoteIf you specify the SourceDBInstanceIdentifier or           DBSnapshotIdentifier property, don't specify this property. The         value is inherited from the source DB instance or snapshot.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The name for the master user is managed by the DB cluster.
    /// 
    /// MariaDB
    /// 
    /// Constraints:
    /// 
    /// Required for MariaDB.               Must be 1 to 16 letters or numbers.               Can't be a reserved word for the chosen database engine.
    /// 
    /// Microsoft SQL Server
    /// 
    /// Constraints:
    /// 
    /// Required for SQL Server.               Must be 1 to 128 letters or numbers.               The first character must be a letter.               Can't be a reserved word for the chosen database engine.
    /// 
    /// MySQL
    /// 
    /// Constraints:
    /// 
    /// Required for MySQL.               Must be 1 to 16 letters or numbers.               First character must be a letter.               Can't be a reserved word for the chosen database engine.
    /// 
    /// Oracle
    /// 
    /// Constraints:
    /// 
    /// Required for Oracle.               Must be 1 to 30 letters or numbers.               First character must be a letter.               Can't be a reserved word for the chosen database engine.
    /// 
    /// PostgreSQL
    /// 
    /// Constraints:
    /// 
    /// Required for PostgreSQL.               Must be 1 to 63 letters or numbers.               First character must be a letter.               Can't be a reserved word for the chosen database engine.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterUsername")]
    pub master_username: Option<String>,


    /// 
    /// The upper limit in gibibytes (GiB) to which Amazon RDS can automatically scale the storage of the DB instance.
    /// 
    /// For more information about this setting, including limitations that apply to it, see                Managing capacity automatically with Amazon RDS storage autoscaling       in the Amazon RDS User Guide.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Storage is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAllocatedStorage")]
    pub max_allocated_storage: Option<i64>,


    /// 
    /// The interval, in seconds, between points when Enhanced Monitoring metrics are collected for      the DB instance. To disable collection of Enhanced Monitoring metrics, specify 0. The default is 0.
    /// 
    /// If MonitoringRoleArn is specified, then you must set MonitoringInterval    to a value other than 0.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Valid Values: 0, 1, 5, 10, 15, 30, 60
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringInterval")]
    pub monitoring_interval: Option<i64>,


    /// 
    /// The ARN for the IAM role that permits RDS to send enhanced monitoring metrics to Amazon CloudWatch Logs. For      example, arn:aws:iam:123456789012:role/emaccess. For information on creating a monitoring role,    see Setting Up and Enabling Enhanced Monitoring      in the Amazon RDS User Guide.
    /// 
    /// If MonitoringInterval is set to a value other than 0, then you must supply a MonitoringRoleArn value.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringRoleArn")]
    pub monitoring_role_arn: Option<String>,


    /// 
    /// Specifies whether the database instance is a Multi-AZ DB instance deployment.       You can't set the AvailabilityZone parameter if the MultiAZ       parameter is set to true.
    /// 
    /// For more information, see       Multi-AZ deployments for high availability in the Amazon RDS User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Amazon Aurora storage is replicated across all of the       Availability Zones and doesn't require the MultiAZ option to be set.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "MultiAZ")]
    pub multi_az: Option<bool>,


    /// 
    /// The name of the NCHAR character set for the Oracle DB instance.
    /// 
    /// This parameter doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NcharCharacterSetName")]
    pub nchar_character_set_name: Option<String>,


    /// 
    /// The network type of the DB instance.
    /// 
    /// Valid values:
    /// 
    /// IPV4                                 DUAL
    /// 
    /// The network type is determined by the DBSubnetGroup specified for the DB instance.       A DBSubnetGroup can support only the IPv4 protocol or the IPv4 and IPv6       protocols (DUAL).
    /// 
    /// For more information, see       Working with a DB instance in a VPC in the       Amazon RDS User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,


    /// 
    /// Indicates that the DB instance should be associated with the specified option       group.
    /// 
    /// Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be       removed from an option group. Also, that option group can't be removed from a DB       instance once it is associated with a DB instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionGroupName")]
    pub option_group_name: Option<String>,


    /// 
    /// The AWS KMS key identifier for encryption of Performance Insights data.
    /// 
    /// The KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
    /// 
    /// If you do not specify a value for PerformanceInsightsKMSKeyId, then Amazon RDS       uses your default KMS key. There is a default KMS key for your AWS account.       Your AWS account has a different default KMS key for each AWS Region.
    /// 
    /// For information about enabling Performance Insights, see       EnablePerformanceInsights.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    pub performance_insights_kmskey_id: Option<String>,


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
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    pub performance_insights_retention_period: Option<i64>,


    /// 
    /// The port number on which the database accepts connections.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The port number is managed by the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    pub port: Option<String>,


    /// 
    /// The daily time range during which automated backups are created if automated backups are enabled,       using the BackupRetentionPeriod parameter. For more information, see         Backup Window in the Amazon RDS User Guide.
    /// 
    /// Constraints:
    /// 
    /// Must be in the format hh24:mi-hh24:mi.       Must be in Universal Coordinated Time (UTC).       Must not conflict with the preferred maintenance window.       Must be at least 30 minutes.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The daily time range for creating automated backups is managed by       the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,


    /// 
    /// The weekly time range during which system maintenance can occur, in Universal       Coordinated Time (UTC).
    /// 
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    /// 
    /// The default is a 30-minute window selected at random from an 8-hour block of time for       each AWS Region, occurring on a random day of the week. To see the time blocks       available, see Adjusting the Preferred DB Instance Maintenance Window in the         Amazon RDS User Guide.
    /// 
    /// NoteThis property applies when AWS CloudFormation initially creates the DB instance.         If you use AWS CloudFormation to update the DB instance, those updates are applied         immediately.
    /// 
    /// Constraints: Minimum 30-minute window.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,


    /// 
    /// The number of CPU cores and the number of threads per core for the DB instance class of the DB instance.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable.
    /// 
    /// Required: No
    ///
    /// Type: List of ProcessorFeature
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Option<Vec<ProcessorFeature>>,


    /// 
    /// A value that specifies the order in which an Aurora Replica is promoted to the primary instance      after a failure of the existing primary instance. For more information,    see      Fault Tolerance for an Aurora DB Cluster in the Amazon Aurora User Guide.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Default: 1
    /// 
    /// Valid Values: 0 - 15
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PromotionTier")]
    pub promotion_tier: Option<i64>,


    /// 
    /// Indicates whether the DB instance is an internet-facing instance. If you specify true, AWS CloudFormation creates an instance with a publicly resolvable DNS name,       which resolves to a public IP address. If you specify false, AWS CloudFormation creates an internal instance with a DNS name that resolves to a private IP address.
    /// 
    /// The default behavior value depends on your VPC setup and the database subnet group. For more information, see the PubliclyAccessible parameter in       the CreateDBInstance       in the Amazon RDS API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,


    /// 
    /// The open mode of an Oracle read replica.       For more information, see Working with Oracle Read Replicas for Amazon RDS       in the Amazon RDS User Guide.
    /// 
    /// This setting is only supported in RDS for Oracle.
    /// 
    /// Default: open-read-only
    /// 
    /// Valid Values: open-read-only or mounted
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicaMode")]
    pub replica_mode: Option<DBInstanceReplicaModeEnum>,


    /// 
    /// The date and time to restore from.
    /// 
    /// Valid Values: Value must be a time in Universal Coordinated Time (UTC) format
    /// 
    /// Constraints:
    /// 
    /// Must be before the latest restorable time for the DB instance               Can't be specified if the UseLatestRestorableTime parameter is enabled
    /// 
    /// Example: 2009-09-07T23:45:00Z
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "RestoreTime")]
    pub restore_time: Option<DBInstanceRestoreTimeEnum>,


    /// 
    /// The identifier of the Multi-AZ DB cluster that will act as the source for the read       replica. Each DB cluster can have up to 15 read replicas.
    /// 
    /// Constraints:
    /// 
    /// Must be the identifier of an existing Multi-AZ DB cluster.               Can't be specified if the SourceDBInstanceIdentifier parameter is           also specified.               The specified DB cluster must have automatic backups enabled, that is, its           backup retention period must be greater than 0.               The source DB cluster must be in the same AWS Region as the read replica.           Cross-Region replication isn't supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the replicated automated backups from which to restore, for example,       arn:aws:rds:useast-1:123456789012:auto-backup:ab-L2IJCEXJP7XQ7HOJ4SIEXAMPLE.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SourceDBInstanceAutomatedBackupsArn")]
    pub source_dbinstance_automated_backups_arn: Option<String>,


    /// 
    /// If you want to create a read replica DB instance, specify the ID of the source DB       instance. Each DB instance can have a limited number of read replicas. For more       information, see Working with Read         Replicas in the Amazon RDS User Guide.
    /// 
    /// For information about constraints that apply to DB instance identifiers, see         Naming constraints in         Amazon RDS in the Amazon RDS User Guide.
    /// 
    /// The SourceDBInstanceIdentifier property determines whether a DB instance       is a read replica. If you remove the SourceDBInstanceIdentifier property       from your template and then update your stack, AWS CloudFormation       promotes the Read Replica to a standalone DB instance.
    /// 
    /// Important                                                                                  If you specify a source DB instance that uses VPC security groups, we             recommend that you specify the VPCSecurityGroups property. If             you don't specify the property, the read replica inherits the value of the               VPCSecurityGroups property from the source DB when you             create the replica. However, if you update the stack, AWS CloudFormation             reverts the replica's VPCSecurityGroups property to the default             value because it's not defined in the stack's template. This change might             cause unexpected issues.                   Read replicas don't support deletion policies. AWS CloudFormation ignores             any deletion policy that's associated with a read replica.                   If you specify SourceDBInstanceIdentifier, don't specify the               DBSnapshotIdentifier property. You can't create a read             replica from a snapshot.                   Don't set the BackupRetentionPeriod, DBName,               MasterUsername, MasterUserPassword, and               PreferredBackupWindow properties. The database attributes             are inherited from the source DB instance, and backups are disabled for read             replicas.                   If the source DB instance is in a different region than the read replica,             specify the source region in SourceRegion, and specify an ARN             for a valid DB instance in SourceDBInstanceIdentifier. For more             information, see               Constructing a Amazon RDS Amazon Resource Name (ARN) in the               Amazon RDS User Guide.                   For DB instances in Amazon Aurora clusters, don't specify this property.             Amazon RDS automatically assigns writer and reader DB instances.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SourceDBInstanceIdentifier")]
    pub source_dbinstance_identifier: Option<String>,


    /// 
    /// The resource ID of the source DB instance from which to restore.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SourceDbiResourceId")]
    pub source_dbi_resource_id: Option<String>,


    /// 
    /// The ID of the region that contains the source DB instance for the read replica.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceRegion")]
    pub source_region: Option<String>,


    /// 
    /// A value that indicates whether the DB instance is encrypted. By default, it isn't encrypted.
    /// 
    /// If you specify the KmsKeyId property, then you must enable encryption.
    /// 
    /// If you specify the SourceDBInstanceIdentifier property, don't specify this property. The       value is inherited from the source DB instance, and if the DB instance is encrypted, the specified       KmsKeyId property is used.
    /// 
    /// If you specify the SnapshotIdentifier and the specified snapshot is encrypted,       don't specify this property. The value is inherited from the snapshot, and the specified KmsKeyId       property is used.
    /// 
    /// If you specify the SnapshotIdentifier and the specified snapshot isn't encrypted, you can use this property       to specify that the restored DB instance is encrypted. Specify the KmsKeyId property for the KMS key       to use for encryption. If you don't want the restored DB instance to be encrypted, then don't set this property       or set it to false.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The encryption for DB instances is managed by      the DB cluster.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,


    /// 
    /// Specifies the storage throughput value for the DB instance. This setting applies only to the gp3 storage type.
    /// 
    /// This setting doesn't apply to RDS Custom or Amazon Aurora.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageThroughput")]
    pub storage_throughput: Option<i64>,


    /// 
    /// Specifies the storage type to be associated with the DB instance.
    /// 
    /// Valid values: gp2 | gp3 | io1 | standard
    /// 
    /// The standard value is also known as magnetic.
    /// 
    /// If you specify io1 or gp3, you must also include a value for the         Iops parameter.
    /// 
    /// Default: io1 if the Iops parameter is specified, otherwise         gp2
    /// 
    /// For more information, see Amazon RDS DB Instance         Storage in the Amazon RDS User Guide.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. Aurora data is stored in the cluster volume, which is a single, virtual volume that uses solid state drives (SSDs).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,


    /// 
    /// An optional array of key-value pairs to apply to this DB instance.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The time zone of the DB instance.       The time zone parameter is currently supported only by       Microsoft SQL Server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,


    /// 
    /// A value that indicates whether the DB instance class of the DB instance uses its default       processor features.
    /// 
    /// This setting doesn't apply to RDS Custom.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseDefaultProcessorFeatures")]
    pub use_default_processor_features: Option<bool>,


    /// 
    /// A value that indicates whether the DB instance is restored from the latest backup time. By default, the DB instance      isn't restored from the latest backup time.
    /// 
    /// Constraints: Can't be specified if the RestoreTime parameter is provided.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,


    /// 
    /// A list of the VPC security group IDs to assign to the DB instance. The list can       include both the physical IDs of existing VPC security groups and references to AWS::EC2::SecurityGroup resources created in the template.
    /// 
    /// If you plan to update the resource, don't specify VPC security groups in a shared VPC.
    /// 
    /// If you set VPCSecurityGroups, you must not set DBSecurityGroups, and vice versa.
    /// 
    /// ImportantYou can migrate a DB instance in your stack from an RDS DB security group to a VPC         security group, but keep the following in mind:                                            You can't revert to using an RDS security group after you establish a VPC             security group membership.                   When you migrate your DB instance to VPC security groups, if your stack             update rolls back because the DB instance update fails or because an update             fails in another AWS CloudFormation resource, the rollback fails because it             can't revert to an RDS security group.                   To use the properties that are available when you use a VPC security             group, you must recreate the DB instance. If you don't, AWS CloudFormation             submits only the property values that are listed in the DBSecurityGroups property.
    /// 
    /// To avoid this situation, migrate your DB instance to using VPC security groups only       when that is the only change in your stack template.
    /// 
    /// Amazon Aurora
    /// 
    /// Not applicable. The associated list of EC2 VPC security groups is managed by       the DB cluster. If specified, the setting must match the DB cluster setting.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VPCSecurityGroups")]
    pub vpcsecurity_groups: Option<Vec<String>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DBInstanceReplicaModeEnum {

    /// open-read-only or mounted
    #[serde(rename = "open-read-only or mounted")]
    Openreadonlyormounted,

}

impl Default for DBInstanceReplicaModeEnum {
    fn default() -> Self {
        DBInstanceReplicaModeEnum::Openreadonlyormounted
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DBInstanceRestoreTimeEnum {

    /// Value must be a time in Universal Coordinated Time (UTC) format
    #[serde(rename = "Value must be a time in Universal Coordinated Time (UTC) format")]
    Valuemustbeatimeinuniversalcoordinatedtimeutcformat,

}

impl Default for DBInstanceRestoreTimeEnum {
    fn default() -> Self {
        DBInstanceRestoreTimeEnum::Valuemustbeatimeinuniversalcoordinatedtimeutcformat
    }
}


impl cfn_resources::CfnResource for CfnDBInstance {
    fn type_string() -> &'static str {
        "AWS::RDS::DBInstance"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.certificate_details.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.endpoint.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.master_user_secret.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Returns the details of the DB instance’s server certificate.
///
/// For more information, see Using SSL/TLS to encrypt a connection to a DB       instance in the Amazon RDS User Guide and              Using SSL/TLS to encrypt a connection to a DB cluster in the Amazon Aurora       User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CertificateDetails {


    /// 
    /// The CA identifier of the CA certificate used for the DB instance's server certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CAIdentifier")]
    pub caidentifier: Option<String>,


    /// 
    /// The expiration date of the DB instance’s server certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidTill")]
    pub valid_till: Option<String>,

}



impl cfn_resources::CfnResource for CertificateDetails {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Describes an AWS Identity and Access Management (IAM) role that is associated with a DB instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DBInstanceRole {


    /// 
    /// The name of the feature associated with the AWS Identity and Access Management (IAM)       role. IAM roles that are associated with a DB instance grant permission for the DB       instance to access other AWS services on your behalf. For the list of supported feature       names, see the SupportedFeatureNames description in DBEngineVersion       in the Amazon RDS API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureName")]
    pub feature_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that is associated with the DB       instance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



impl cfn_resources::CfnResource for DBInstanceRole {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// This data type represents the information you need to connect to an Amazon RDS DB instance.    This data type is used as a response element in the following actions:
///
/// For the data structure that represents Amazon Aurora DB cluster endpoints,     see DBClusterEndpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Endpoint {


    /// 
    /// Specifies the DNS address of the DB instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,


    /// 
    /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,


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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ProcessorFeature property type specifies the processor features of a       DB instance class status.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProcessorFeature {


    /// 
    /// The name of the processor feature. Valid names are coreCount and threadsPerCore.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value of a processor feature name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}



impl cfn_resources::CfnResource for ProcessorFeature {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}