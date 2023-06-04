/// The AWS::DocDB::DBCluster Amazon DocumentDB (with MongoDB compatibility) resource describes a DBCluster.      Amazon DocumentDB is a fully managed, MongoDB-compatible document database engine. For more information, see      DBCluster in the Amazon DocumentDB Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDBCluster {
    ///
    /// A list of Amazon EC2 Availability Zones that instances in the       cluster can be created in.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// The number of days for which automated backups are retained. You       must specify a minimum value of 1.
    ///
    /// Default: 1
    ///
    /// Constraints:
    ///
    /// Must be a value from 1 to 35.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,

    ///
    /// The cluster identifier. This parameter is stored as a lowercase       string.
    ///
    /// Constraints:
    ///
    /// Must contain from 1 to 63 letters, numbers, or hyphens.                         The first character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Example: my-cluster
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The name of the cluster parameter group to associate with this       cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbcluster_parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// A subnet group to associate with this cluster.
    ///
    /// Constraints: Must match the name of an existing       DBSubnetGroup. Must not be default.
    ///
    /// Example: mySubnetgroup
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbsubnet_group_name: Option<cfn_resources::StrVal>,

    ///
    /// Protects clusters from being accidentally deleted. If enabled, the     cluster cannot be deleted unless it is modified and      DeletionProtection is disabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,

    ///
    /// The list of log types that need to be enabled for exporting to Amazon CloudWatch       Logs. You can enable audit logs or profiler logs. For more information, see       Auditing Amazon DocumentDB Events       and Profiling Amazon DocumentDB Operations.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,

    ///
    /// The version number of the database engine to use. The --engine-version will default to the latest major engine version. For production workloads, we recommend explicitly declaring this parameter with the intended major engine version.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// The AWS KMS key identifier for an encrypted cluster.
    ///
    /// The AWS KMS key identifier is the Amazon Resource Name (ARN) for the AWS KMS encryption key. If you are creating a cluster using the same AWS account that owns the AWS KMS encryption key that is used to encrypt the new cluster, you can use the AWS KMS key alias instead of the ARN for the AWS KMS encryption key.
    ///
    /// If an encryption key is not specified in KmsKeyId:
    ///
    /// If the StorageEncrypted parameter is           true, Amazon DocumentDB uses your default encryption key.
    ///
    /// AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Regions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The password for the master database user. This password can       contain any printable ASCII character except forward slash (/),       double quote ("), or the "at" symbol (@).
    ///
    /// Constraints: Must contain from 8 to 100 characters.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<cfn_resources::StrVal>,

    ///
    /// The name of the master user for the cluster.
    ///
    /// Constraints:
    ///
    /// Must be from 1 to 63 letters or numbers.               The first character must be a letter.               Cannot be a reserved word for the chosen database engine.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the port that the database engine is listening on.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The daily time range during which automated backups are created if       automated backups are enabled using the BackupRetentionPeriod parameter.
    ///
    /// The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region.
    ///
    /// Constraints:
    ///
    /// Must be in the format hh24:mi-hh24:mi.               Must be in Universal Coordinated Time (UTC).               Must not conflict with the preferred maintenance window.                         Must be at least 30 minutes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<cfn_resources::StrVal>,

    ///
    /// The weekly time range during which system maintenance can occur,       in Universal Coordinated Time (UTC).
    ///
    /// Format: ddd:hh24:mi-ddd:hh24:mi
    ///
    /// The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.
    ///
    /// Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun
    ///
    /// Constraints: Minimum 30-minute window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestoreToTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_to_time: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<cfn_resources::StrVal>,

    ///
    /// The identifier for the snapshot or cluster snapshot to restore from.
    ///
    /// You can use either the name or the Amazon Resource Name (ARN) to specify a cluster       snapshot. However, you can use only the ARN to specify a snapshot.
    ///
    /// Constraints:
    ///
    /// Must match the identifier of an existing snapshot.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dbcluster_identifier: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the cluster is encrypted.
    ///
    /// Required: Conditional
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,

    ///
    /// The tags to be assigned to the cluster.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,

    ///
    /// A list of EC2 VPC security groups to associate with this cluster.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
        "AWS::DocDB::DBCluster"
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
