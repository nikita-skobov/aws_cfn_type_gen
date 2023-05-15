
pub mod cfn_dbcluster {

#[derive(serde::Serialize, Default)]
pub struct CfnDBCluster {
    /// The name of the DB parameter group to apply to all instances of the DB cluster. Used only in case of a major EngineVersion upgrade request.
    #[serde(rename = "DBInstanceParameterGroupName")]
    pub dbinstance_parameter_group_name: Option<String>,
    /// Indicates whether the DB cluster is encrypted.
    /// 
    /// If you specify the `DBClusterIdentifier`, `DBSnapshotIdentifier`, or `SourceDBInstanceIdentifier` property, don't specify this property. The value is inherited from the cluster, snapshot, or source DB instance. If you specify the KmsKeyId property, you must enable encryption.
    /// 
    /// If you specify the KmsKeyId, you must enable encryption by setting StorageEncrypted to true.
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// The tags assigned to this cluster.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Provides a list of VPC security groups that the DB cluster belongs to.
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    /// 
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.
    /// 
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,
    /// Specifies the daily time range during which automated backups are created if automated backups are enabled, as determined by the BackupRetentionPeriod.
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// If `StorageEncrypted` is true, the Amazon KMS key identifier for the encrypted DB cluster.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// The DB cluster identifier. Contains a user-supplied DB cluster identifier. This identifier is the unique key that identifies a DB cluster stored as a lowercase string.
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,
    /// Specifies the weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// Indicates the database engine version.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// Provides the name of the DB cluster parameter group.
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,
    /// Specifies a list of log types that are enabled for export to CloudWatch Logs.
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// True if mapping of Amazon Identity and Access Management (IAM) accounts to database accounts is enabled, and otherwise false.
    #[serde(rename = "IamAuthEnabled")]
    pub iam_auth_enabled: Option<bool>,
    /// A value that indicates whether to copy all tags from the DB cluster to snapshots of the DB cluster. The default behaviour is not to copy them.
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// Specifies information on the subnet group associated with the DB cluster, including the name, description, and subnets in the subnet group.
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// Specifies the number of days for which automatic DB snapshots are retained.
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<usize>,
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    /// 
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.
    /// 
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.
    #[serde(rename = "RestoreToTime")]
    pub restore_to_time: Option<String>,
    /// Provides the list of EC2 Availability Zones that instances in the DB cluster can be created in.
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    /// 
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.
    /// 
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.
    #[serde(rename = "RestoreType")]
    pub restore_type: Option<String>,
    /// Indicates whether or not the DB cluster has deletion protection enabled. The database can't be deleted when deletion protection is enabled.
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    /// 
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB snapshot with a default configuration and default security group.
    /// 
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB cluster restore point with the same configuration as the original source DB cluster, except that the new DB cluster is created with the default security group.
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,
    /// Contains the scaling configuration used by the Neptune Serverless Instances within this DB cluster.
    #[serde(rename = "ServerlessScalingConfiguration")]
    pub serverless_scaling_configuration: Option<ServerlessScalingConfiguration>,
    /// Specifies the identifier for a DB cluster snapshot. Must match the identifier of an existing snapshot.
    /// 
    /// After you restore a DB cluster using a SnapshotIdentifier, you must specify the same SnapshotIdentifier for any future updates to the DB cluster. When you specify this property for an update, the DB cluster is not restored from the snapshot again, and the data in the database is not changed.
    /// 
    /// However, if you don't specify the SnapshotIdentifier, an empty DB cluster is created, and the original DB cluster is deleted. If you specify a property that is different from the previous snapshot restore property, the DB cluster is restored from the snapshot specified by the SnapshotIdentifier, and the original DB cluster is deleted.
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,
    /// Provides a list of the AWS Identity and Access Management (IAM) roles that are associated with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the DB cluster to access other AWS services on your behalf.
    #[serde(rename = "AssociatedRoles")]
    pub associated_roles: Option<Vec<DBClusterRole>>,

}


#[derive(serde::Serialize, Default)]
pub struct DBClusterRole {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "FeatureName")]
    pub feature_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServerlessScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: f64,
    #[serde(rename = "MinCapacity")]
    pub min_capacity: f64,

}


}

pub mod cfn_dbcluster_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBClusterParameterGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,
    /// No documentation provided by AWS
    #[serde(rename = "Family")]
    pub family: String,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: (),
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

pub mod cfn_dbinstance {

#[derive(serde::Serialize, Default)]
pub struct CfnDBInstance {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: String,
    /// No documentation provided by AWS
    #[serde(rename = "AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSnapshotIdentifier")]
    pub dbsnapshot_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBParameterGroupName")]
    pub dbparameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_dbparameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBParameterGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: (),
    /// No documentation provided by AWS
    #[serde(rename = "Family")]
    pub family: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_dbsubnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBSubnetGroup {
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupDescription")]
    pub dbsubnet_group_description: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
