
pub mod cfn_cache_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCacheCluster {
    /// No documentation provided by AWS
    #[serde(rename = "CacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AZMode")]
    pub azmode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,
    /// List of LogDeliveryConfigurationRequest
    #[serde(rename = "LogDeliveryConfigurations")]
    pub log_delivery_configurations: Option<Vec<LogDeliveryConfigurationRequest>>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationEndpointPort")]
    pub configuration_endpoint_port: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotRetentionLimit")]
    pub snapshot_retention_limit: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheSecurityGroupNames")]
    pub cache_security_group_names: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheSubnetGroupName")]
    pub cache_subnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Engine")]
    pub engine: String,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotWindow")]
    pub snapshot_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredAvailabilityZones")]
    pub preferred_availability_zones: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "NumCacheNodes")]
    pub num_cache_nodes: usize,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredAvailabilityZone")]
    pub preferred_availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationEndpointAddress")]
    pub configuration_endpoint_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RedisEndpointAddress")]
    pub redis_endpoint_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotArns")]
    pub snapshot_arns: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "IpDiscovery")]
    pub ip_discovery: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitEncryptionEnabled")]
    pub transit_encryption_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationTopicArn")]
    pub notification_topic_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RedisEndpointPort")]
    pub redis_endpoint_port: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DestinationDetails {
    #[serde(rename = "KinesisFirehoseDetails")]
    pub kinesis_firehose_details: Option<KinesisFirehoseDestinationDetails>,
    #[serde(rename = "CloudWatchLogsDetails")]
    pub cloud_watch_logs_details: Option<CloudWatchLogsDestinationDetails>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogsDestinationDetails {
    #[serde(rename = "LogGroup")]
    pub log_group: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogDeliveryConfigurationRequest {
    #[serde(rename = "LogType")]
    pub log_type: String,
    #[serde(rename = "LogFormat")]
    pub log_format: String,
    #[serde(rename = "DestinationType")]
    pub destination_type: String,
    #[serde(rename = "DestinationDetails")]
    pub destination_details: DestinationDetails,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseDestinationDetails {
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: String,

}


}

pub mod cfn_global_replication_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGlobalReplicationGroup {
    /// The engine version of the Global Datastore.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// Cache parameter group name to use for the new engine version. This parameter cannot be modified independently.
    #[serde(rename = "CacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,
    /// The cache node type of the Global Datastore
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: Option<String>,
    /// Indicates the number of node groups in the Global Datastore.
    #[serde(rename = "GlobalNodeGroupCount")]
    pub global_node_group_count: Option<usize>,
    /// The optional description of the Global Datastore
    #[serde(rename = "GlobalReplicationGroupDescription")]
    pub global_replication_group_description: Option<String>,
    /// The replication groups that comprise the Global Datastore.
    #[serde(rename = "Members")]
    pub members: Vec<GlobalReplicationGroupMember>,
    /// Describes the replication group IDs, the AWS regions where they are stored and the shard configuration for each that comprise the Global Datastore
    #[serde(rename = "RegionalConfigurations")]
    pub regional_configurations: Option<Vec<RegionalConfiguration>>,
    /// The suffix name of a Global Datastore. Amazon ElastiCache automatically applies a prefix to the Global Datastore ID when it is created. Each AWS Region has its own prefix.
    #[serde(rename = "GlobalReplicationGroupIdSuffix")]
    pub global_replication_group_id_suffix: Option<String>,
    /// AutomaticFailoverEnabled
    #[serde(rename = "AutomaticFailoverEnabled")]
    pub automatic_failover_enabled: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct RegionalConfiguration {
    #[serde(rename = "ReshardingConfigurations")]
    pub resharding_configurations: Option<Vec<ReshardingConfiguration>>,
    #[serde(rename = "ReplicationGroupRegion")]
    pub replication_group_region: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    pub replication_group_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ReshardingConfiguration {
    #[serde(rename = "PreferredAvailabilityZones")]
    pub preferred_availability_zones: Option<Vec<String>>,
    #[serde(rename = "NodeGroupId")]
    pub node_group_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalReplicationGroupMember {
    #[serde(rename = "ReplicationGroupId")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ReplicationGroupRegion")]
    pub replication_group_region: Option<String>,
    #[serde(rename = "Role")]
    pub role: Option<String>,

}


}

pub mod cfn_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnParameterGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Properties")]
    pub properties: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheParameterGroupFamily")]
    pub cache_parameter_group_family: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_replication_group {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationGroup {
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotArns")]
    pub snapshot_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationTopicArn")]
    pub notification_topic_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PrimaryEndPointPort")]
    pub primary_end_point_port: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationEndPointAddress")]
    pub configuration_end_point_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshottingClusterId")]
    pub snapshotting_cluster_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheSecurityGroupNames")]
    pub cache_security_group_names: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ReaderEndPointAddress")]
    pub reader_end_point_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DataTieringEnabled")]
    pub data_tiering_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotRetentionLimit")]
    pub snapshot_retention_limit: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Engine")]
    pub engine: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GlobalReplicationGroupId")]
    pub global_replication_group_id: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterMode")]
    pub cluster_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredCacheClusterAZs")]
    pub preferred_cache_cluster_azs: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "MultiAZEnabled")]
    pub multi_azenabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadEndPointPortsList")]
    pub read_end_point_ports_list: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PrimaryClusterId")]
    pub primary_cluster_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NumCacheClusters")]
    pub num_cache_clusters: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitEncryptionMode")]
    pub transit_encryption_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationEndPointPort")]
    pub configuration_end_point_port: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadEndPointAddresses")]
    pub read_end_point_addresses: Option<String>,
    /// List of LogDeliveryConfigurationRequest
    #[serde(rename = "LogDeliveryConfigurations")]
    pub log_delivery_configurations: Option<Vec<LogDeliveryConfigurationRequest>>,
    /// No documentation provided by AWS
    #[serde(rename = "NumNodeGroups")]
    pub num_node_groups: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AutomaticFailoverEnabled")]
    pub automatic_failover_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ReaderEndPointPort")]
    pub reader_end_point_port: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotWindow")]
    pub snapshot_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IpDiscovery")]
    pub ip_discovery: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PrimaryEndPointAddress")]
    pub primary_end_point_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationGroupDescription")]
    pub replication_group_description: String,
    /// No documentation provided by AWS
    #[serde(rename = "AtRestEncryptionEnabled")]
    pub at_rest_encryption_enabled: Option<bool>,
    /// List of NodeGroupConfiguration
    #[serde(rename = "NodeGroupConfiguration")]
    pub node_group_configuration: Option<Vec<NodeGroupConfiguration>>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitEncryptionEnabled")]
    pub transit_encryption_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkType")]
    pub network_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadEndPointPorts")]
    pub read_end_point_ports: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicasPerNodeGroup")]
    pub replicas_per_node_group: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "UserGroupIds")]
    pub user_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ReadEndPointAddressesList")]
    pub read_end_point_addresses_list: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheSubnetGroupName")]
    pub cache_subnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthToken")]
    pub auth_token: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DestinationDetails {
    #[serde(rename = "CloudWatchLogsDetails")]
    pub cloud_watch_logs_details: Option<CloudWatchLogsDestinationDetails>,
    #[serde(rename = "KinesisFirehoseDetails")]
    pub kinesis_firehose_details: Option<KinesisFirehoseDestinationDetails>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogsDestinationDetails {
    #[serde(rename = "LogGroup")]
    pub log_group: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseDestinationDetails {
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogDeliveryConfigurationRequest {
    #[serde(rename = "DestinationDetails")]
    pub destination_details: DestinationDetails,
    #[serde(rename = "LogFormat")]
    pub log_format: String,
    #[serde(rename = "LogType")]
    pub log_type: String,
    #[serde(rename = "DestinationType")]
    pub destination_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct NodeGroupConfiguration {
    #[serde(rename = "NodeGroupId")]
    pub node_group_id: Option<String>,
    #[serde(rename = "ReplicaCount")]
    pub replica_count: Option<usize>,
    #[serde(rename = "PrimaryAvailabilityZone")]
    pub primary_availability_zone: Option<String>,
    #[serde(rename = "ReplicaAvailabilityZones")]
    pub replica_availability_zones: Option<Vec<String>>,
    #[serde(rename = "Slots")]
    pub slots: Option<String>,

}


}

pub mod cfn_security_group {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityGroup {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_security_group_ingress {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityGroupIngress {
    /// No documentation provided by AWS
    #[serde(rename = "CacheSecurityGroupName")]
    pub cache_security_group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: String,

}



}

pub mod cfn_subnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetGroup {
    /// The name for the cache subnet group. This value is stored as a lowercase string.
    #[serde(rename = "CacheSubnetGroupName")]
    pub cache_subnet_group_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The description for the cache subnet group.
    #[serde(rename = "Description")]
    pub description: String,
    /// The EC2 subnet IDs for the cache subnet group.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUser {
    /// The ID of the user.
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// Passwords used for this user account. You can create up to two passwords for each user.
    #[serde(rename = "Passwords")]
    pub passwords: Option<Vec<String>>,
    /// An array of key-value pairs to apply to this user.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The username of the user.
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// Must be redis.
    #[serde(rename = "Engine")]
    pub engine: String,
    /// Access permissions string used for this user account.
    #[serde(rename = "AccessString")]
    pub access_string: Option<String>,
    /// Indicates a password is not required for this user account.
    #[serde(rename = "NoPasswordRequired")]
    pub no_password_required: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationMode")]
    pub authentication_mode: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_user_group {

#[derive(serde::Serialize, Default)]
pub struct CfnUserGroup {
    /// List of users associated to this user group.
    #[serde(rename = "UserIds")]
    pub user_ids: Vec<String>,
    /// An array of key-value pairs to apply to this user.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Must be redis.
    #[serde(rename = "Engine")]
    pub engine: String,
    /// The ID of the user group.
    #[serde(rename = "UserGroupId")]
    pub user_group_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}


}
