
pub mod cfn_acl {

#[derive(serde::Serialize, Default)]
pub struct CfnACL {
    /// List of users associated to this acl.
    #[serde(rename = "UserNames")]
    pub user_names: Option<Vec<String>>,
    /// The name of the acl.
    #[serde(rename = "ACLName")]
    pub aclname: String,
    /// An array of key-value pairs to apply to this cluster.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// The name of the cluster. This value must be unique as it also serves as the cluster identifier.
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<String>,
    /// Enables data tiering. Data tiering is only supported for clusters using the r6gd node type. This parameter must be set when using r6gd nodes.
    #[serde(rename = "DataTiering")]
    pub data_tiering: Option<DataTieringStatus>,
    /// The number of replicas to apply to each shard. The limit is 5.
    #[serde(rename = "NumReplicasPerShard")]
    pub num_replicas_per_shard: Option<usize>,
    /// The daily time range (in UTC) during which MemoryDB begins taking a daily snapshot of your cluster.
    #[serde(rename = "SnapshotWindow")]
    pub snapshot_window: Option<String>,
    /// The ID of the KMS key used to encrypt the cluster.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// The compute and memory capacity of the nodes in the cluster.
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// The status of the Amazon SNS notification topic. Notifications are sent only if the status is enabled.
    #[serde(rename = "SnsTopicStatus")]
    pub sns_topic_status: Option<String>,
    /// An optional description of the cluster.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The number of days for which MemoryDB retains automatic snapshots before deleting them. For example, if you set SnapshotRetentionLimit to 5, a snapshot that was taken today is retained for 5 days before being deleted.
    #[serde(rename = "SnapshotRetentionLimit")]
    pub snapshot_retention_limit: Option<usize>,
    /// The name of a snapshot from which to restore data into the new cluster. The snapshot status changes to restoring while the new cluster is being created.
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,
    /// The Redis engine version used by the cluster.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// A list of Amazon Resource Names (ARN) that uniquely identify the RDB snapshot files stored in Amazon S3. The snapshot files are used to populate the new cluster. The Amazon S3 object name in the ARN cannot contain any commas.
    #[serde(rename = "SnapshotArns")]
    pub snapshot_arns: Option<Vec<String>>,
    /// The name of the parameter group associated with the cluster.
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,
    /// The cluster endpoint.
    #[serde(rename = "ClusterEndpoint")]
    pub cluster_endpoint: Option<Endpoint>,
    /// A flag that enables automatic minor version upgrade when set to true.
    /// 
    /// You cannot modify the value of AutoMinorVersionUpgrade after the cluster is created. To enable AutoMinorVersionUpgrade on a cluster you must set AutoMinorVersionUpgrade to true when you create a cluster.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// An array of key-value pairs to apply to this cluster.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the subnet group to be used for the cluster.
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: Option<String>,
    /// One or more Amazon VPC security groups associated with this cluster.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// The number of shards the cluster will contain.
    #[serde(rename = "NumShards")]
    pub num_shards: Option<usize>,
    /// A flag that enables in-transit encryption when set to true.
    /// 
    /// You cannot modify the value of TransitEncryptionEnabled after the cluster is created. To enable in-transit encryption on a cluster you must set TransitEncryptionEnabled to true when you create a cluster.
    #[serde(rename = "TLSEnabled")]
    pub tlsenabled: Option<bool>,
    /// The user-supplied name of a final cluster snapshot. This is the unique name that identifies the snapshot. MemoryDB creates the snapshot, and then deletes the cluster immediately afterward.
    #[serde(rename = "FinalSnapshotName")]
    pub final_snapshot_name: Option<String>,
    /// Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.
    #[serde(rename = "MaintenanceWindow")]
    pub maintenance_window: Option<String>,
    /// The name of the Access Control List to associate with the cluster.
    #[serde(rename = "ACLName")]
    pub aclname: String,

}

pub type DataTieringStatus = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Endpoint {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "Address")]
    pub address: Option<String>,

}


}

pub mod cfn_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnParameterGroup {
    /// An array of key-value pairs to apply to this parameter group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the parameter group family that this parameter group is compatible with.
    #[serde(rename = "Family")]
    pub family: String,
    /// A description of the parameter group.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the parameter group.
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// An map of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional.
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_subnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetGroup {
    /// An array of key-value pairs to apply to this subnet group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A list of VPC subnet IDs for the subnet group.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// The name of the subnet group. This value must be unique as it also serves as the subnet group identifier.
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: String,
    /// An optional description of the subnet group.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUser {
    /// The name of the user.
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationMode")]
    pub authentication_mode: Option<()>,
    /// Access permissions string used for this user account.
    #[serde(rename = "AccessString")]
    pub access_string: Option<String>,
    /// An array of key-value pairs to apply to this user.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}


}
