/// The AWS::Neptune::DBCluster resource creates an Amazon Neptune DB cluster.     Neptune is a fully managed graph database.
///
/// If no DeletionPolicy is set for AWS::Neptune::DBCluster     resources, the default deletion behavior is that the entire volume will be deleted without a snapshot.     To retain a backup of the volume, the DeletionPolicy should be set to Snapshot.     For more information about how AWS CloudFormation deletes resources,     see DeletionPolicy Attribute.
///
/// You can use AWS::Neptune::DBCluster.DeletionProtection to help guard against     unintended deletion of your DB cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDBCluster {
    ///
    /// Provides a list of the Amazon Identity and Access Management (IAM) roles that are associated    with the DB cluster. IAM roles that are associated with a DB cluster grant permission for the    DB cluster to access other Amazon services on your behalf.
    ///
    /// Required: No
    ///
    /// Type: List of DBClusterRole
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociatedRoles")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub associated_roles: Option<Vec<DBClusterRole>>,

    ///
    /// Provides the list of EC2 Availability Zones that instances in the DB cluster can be    created in.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// Specifies the number of days for which automatic DB snapshots are retained.
    ///
    /// An update may require some interruption. See ModifyDBInstance in the Amazon Neptune User Guide for more information.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub backup_retention_period: Option<i64>,

    ///
    /// If set to true, tags are copied to any snapshot of    the DB cluster that is created.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub copy_tags_to_snapshot: Option<bool>,

    ///
    /// Contains a user-supplied DB cluster identifier. This identifier is the unique key that    identifies a DB cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Provides the name of the DB cluster parameter group.
    ///
    /// An update may require some interruption. See ModifyDBInstance      in the Amazon Neptune User Guide for more information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dbcluster_parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the DB parameter group to apply to all instances of the DB cluster.      Used only in case of a major engine version upgrade request
    ///
    /// Note that when you apply a parameter group using DBInstanceParameterGroupName,      parameter changes are applied immediately, not during the next maintenance window.
    ///
    /// Constraints                  The DB parameter group must be in the same DB parameter group family       as the target DB cluster version.The DBInstanceParameterGroupName parameter is only       valid for major engine version upgrades.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBInstanceParameterGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dbinstance_parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// Specifies information on the subnet group associated with the DB cluster, including the    name, description, and subnets in the subnet group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dbsubnet_group_name: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether or not the DB cluster has deletion protection         enabled. The database can't be deleted when deletion protection is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub deletion_protection: Option<bool>,

    ///
    /// Specifies a list of log types that are enabled for export to CloudWatch Logs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,

    ///
    /// Indicates the database engine version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// True if mapping of Amazon Identity and Access Management (IAM) accounts to database accounts    is enabled, and otherwise false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamAuthEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub iam_auth_enabled: Option<bool>,

    ///
    /// If StorageEncrypted is true, the Amazon KMS key identifier for the    encrypted DB cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the daily time range during which automated backups are created if automated    backups are enabled, as determined by the BackupRetentionPeriod.
    ///
    /// An update may require some interruption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub preferred_backup_window: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the weekly time range during which system maintenance can occur, in Universal    Coordinated Time (UTC).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub preferred_maintenance_window: Option<cfn_resources::StrVal>,

    ///
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    ///
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB    snapshot with a default configuration and default security group.
    ///
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB    cluster restore point with the same configuration as the original source DB cluster, except    that the new DB cluster is created with the default security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestoreToTime")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub restore_to_time: Option<cfn_resources::StrVal>,

    ///
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    ///
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB    snapshot with a default configuration and default security group.
    ///
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB    cluster restore point with the same configuration as the original source DB cluster, except    that the new DB cluster is created with the default security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub restore_type: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ServerlessScalingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerlessScalingConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub serverless_scaling_configuration: Option<ServerlessScalingConfiguration>,

    ///
    /// Specifies the identifier for a DB cluster snapshot. Must match the identifier    of an existing snapshot.
    ///
    /// After you restore a DB cluster using a SnapshotIdentifier,    you must specify the same SnapshotIdentifier for any future    updates to the DB cluster. When you specify this property for an update, the DB    cluster is not restored from the snapshot again, and the data in the database is not    changed.
    ///
    /// However, if you don't specify the SnapshotIdentifier, an empty    DB cluster is created, and the original DB cluster is deleted. If you specify a    property that is different from the previous snapshot restore property, the DB    cluster is restored from the snapshot specified by the SnapshotIdentifier,    and the original DB cluster is deleted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshot_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    ///
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB    snapshot with a default configuration and default security group.
    ///
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB    cluster restore point with the same configuration as the original source DB cluster, except    that the new DB cluster is created with the default security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub source_dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether the DB cluster is encrypted.
    ///
    /// If you specify the DBClusterIdentifier,           DBSnapshotIdentifier, or SourceDBInstanceIdentifier          property, don't specify this property. The value is inherited from the cluster,          snapshot, or source DB instance. If you specify the KmsKeyId property, you          must enable encryption.
    ///
    /// If you specify the KmsKeyId, you must enable encryption by setting         StorageEncrypted to true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub storage_encrypted: Option<bool>,

    ///
    /// The tags assigned to this cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Creates a new DB cluster from a DB snapshot or DB cluster snapshot.
    ///
    /// If a DB snapshot is specified, the target DB cluster is created from the source DB    snapshot with a default configuration and default security group.
    ///
    /// If a DB cluster snapshot is specified, the target DB cluster is created from the source DB    cluster restore point with the same configuration as the original source DB cluster, except    that the new DB cluster is created with the default security group.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub use_latest_restorable_time: Option<bool>,

    ///
    /// Provides a list of VPC security groups that the DB cluster belongs to.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpc_security_group_ids: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_cluster_resource_id: CfnDBClusterclusterresourceid,

    #[serde(skip_serializing)]
    pub att_endpoint: CfnDBClusterendpoint,

    #[serde(skip_serializing)]
    pub att_port: CfnDBClusterport,

    #[serde(skip_serializing)]
    pub att_read_endpoint: CfnDBClusterreadendpoint,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDBClusterclusterresourceid;
impl CfnDBClusterclusterresourceid {
    pub fn att_name(&self) -> &'static str {
        r#"ClusterResourceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDBClusterendpoint;
impl CfnDBClusterendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"Endpoint"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDBClusterport;
impl CfnDBClusterport {
    pub fn att_name(&self) -> &'static str {
        r#"Port"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDBClusterreadendpoint;
impl CfnDBClusterreadendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"ReadEndpoint"#
    }
}

impl cfn_resources::CfnResource for CfnDBCluster {
    fn type_string(&self) -> &'static str {
        "AWS::Neptune::DBCluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.serverless_scaling_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an Amazon Identity and Access Management (IAM) role that is associated with a DB    cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DBClusterRole {
    ///
    /// The name of the feature associated with the Amazon Identity and Access Management (IAM) role.    For the list of supported feature names, see DescribeDBEngineVersions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FeatureName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub feature_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that is associated with the DB    cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DBClusterRole {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The ServerlessScalingConfiguration property type specifies Property description not available. for an AWS::Neptune::DBCluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerlessScalingConfiguration {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: f64,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: f64,
}

impl cfn_resources::CfnResource for ServerlessScalingConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
