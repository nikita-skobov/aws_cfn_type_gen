
pub mod cfn_dbcluster {

#[derive(serde::Serialize, Default)]
pub struct CfnDBCluster {
    /// No documentation provided by AWS
    #[serde(rename = "UseLatestRestorableTime")]
    pub use_latest_restorable_time: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DeletionProtection")]
    pub deletion_protection: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterParameterGroupName")]
    pub dbcluster_parameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RestoreToTime")]
    pub restore_to_time: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceDBClusterIdentifier")]
    pub source_dbcluster_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MasterUsername")]
    pub master_username: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotIdentifier")]
    pub snapshot_identifier: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredBackupWindow")]
    pub preferred_backup_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "RestoreType")]
    pub restore_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StorageEncrypted")]
    pub storage_encrypted: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupRetentionPeriod")]
    pub backup_retention_period: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableCloudwatchLogsExports")]
    pub enable_cloudwatch_logs_exports: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_dbcluster_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBClusterParameterGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: (),
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,
    /// No documentation provided by AWS
    #[serde(rename = "Family")]
    pub family: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
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

pub mod cfn_dbinstance {

#[derive(serde::Serialize, Default)]
pub struct CfnDBInstance {
    /// No documentation provided by AWS
    #[serde(rename = "DBClusterIdentifier")]
    pub dbcluster_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DBInstanceClass")]
    pub dbinstance_class: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnablePerformanceInsights")]
    pub enable_performance_insights: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DBInstanceIdentifier")]
    pub dbinstance_identifier: Option<String>,
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

pub mod cfn_dbsubnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnDBSubnetGroup {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupDescription")]
    pub dbsubnet_group_description: String,
    /// No documentation provided by AWS
    #[serde(rename = "DBSubnetGroupName")]
    pub dbsubnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
