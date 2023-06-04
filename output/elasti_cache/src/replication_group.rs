/// The AWS::ElastiCache::ReplicationGroup    resource creates an Amazon ElastiCache Redis replication group. A Redis (cluster mode disabled) replication group is a collection of cache clusters, where one of the clusters is a primary read-write cluster and the others are read-only replicas.
///
/// A Redis (cluster mode enabled) cluster is comprised of from 1 to 90 shards (API/CLI: node groups).    Each shard has a primary node and up to 5 read-only replica nodes. The configuration can range from 90 shards and 0 replicas to 15 shards and 5 replicas, which is the maximum number or replicas allowed.
///
/// The node or shard limit can be increased to a maximum of 500 per cluster if the Redis engine version is 5.0.6 or higher. For example, you can choose to configure a 500 node cluster that ranges between    83 shards (one primary and 5 replicas per shard) and 500 shards (single primary and no replicas). Make sure there are enough available IP addresses to accommodate the increase.    Common pitfalls include the subnets in the subnet group have too small a CIDR range or the subnets are shared and heavily used by other clusters. For more information, see    Creating a Subnet Group. For versions below 5.0.6,      the limit is 250 per cluster.
///
/// To request a limit increase, see    Amazon Service Limits    and choose the limit type Nodes per cluster per instance type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnReplicationGroup {
    ///
    /// A flag that enables encryption at rest when set to true.
    ///
    /// You cannot modify the value of AtRestEncryptionEnabled after the replication       group is created.       To enable encryption at rest on a replication group you must set AtRestEncryptionEnabled to       true when you create the replication group.
    ///
    /// Required:       Only available when creating a replication group in an Amazon VPC using redis version 3.2.6 or 4.x onward.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub at_rest_encryption_enabled: Option<bool>,

    ///
    /// Reserved parameter.        The password used to access a password protected server.
    ///
    /// AuthToken can be specified only on replication groups where TransitEncryptionEnabled is       true. For more information, see Authenticating Users with the Redis AUTH Command.
    ///
    /// ImportantFor HIPAA compliance, you must specify TransitEncryptionEnabled as true,       an AuthToken, and a CacheSubnetGroup.
    ///
    /// Password constraints:
    ///
    /// Must be only printable ASCII characters.               Must be at least 16 characters and no more than 128         characters in length.               Nonalphanumeric characters are restricted to (!, &, #, $, ^, <, >, -, ).
    ///
    /// For more information, see AUTH password at http://redis.io/commands/AUTH.
    ///
    /// NoteIf ADDING the AuthToken, update requires Replacement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AuthToken")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub auth_token: Option<cfn_resources::StrVal>,

    ///
    /// If you are running Redis engine version 6.0 or later, set this parameter to yes if you want to opt-in to the next minor version upgrade campaign. This parameter is disabled for previous versions.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub auto_minor_version_upgrade: Option<bool>,

    ///
    /// Specifies whether a read-only replica is automatically promoted to read/write primary if the existing primary fails.
    ///
    /// AutomaticFailoverEnabled must be enabled for Redis (cluster mode enabled) replication groups.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticFailoverEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub automatic_failover_enabled: Option<bool>,

    ///
    /// The compute and memory capacity of the nodes in the node group (shard).
    ///
    /// The following node types are supported by ElastiCache. 			Generally speaking, the current generation types provide more memory and computational power 			at lower cost when compared to their equivalent previous generation counterparts.
    ///
    /// General purpose:                                              Current generation:                       M6g node types:             cache.m6g.large,             cache.m6g.xlarge,             cache.m6g.2xlarge,             cache.m6g.4xlarge,             cache.m6g.12xlarge,             cache.m6g.24xlarge                                    	                                                       M5 node types:             cache.m5.large,             cache.m5.xlarge,             cache.m5.2xlarge,             cache.m5.4xlarge,             cache.m5.12xlarge,             cache.m5.24xlarge                                    	                                 M4 node types:             cache.m4.large,             cache.m4.xlarge,             cache.m4.2xlarge,             cache.m4.4xlarge,             cache.m4.10xlarge                       T4g node types:             cache.t4g.micro,             cache.t4g.small,             cache.t4g.medium                       T3 node types:             cache.t3.micro,             cache.t3.small,             cache.t3.medium                      T2 node types:             cache.t2.micro,             cache.t2.small,             cache.t2.medium                                                      Previous generation: (not recommended)           T1 node types:             cache.t1.micro                      M1 node types:             cache.m1.small,             cache.m1.medium,             cache.m1.large,             cache.m1.xlarge                      M3 node types:             cache.m3.medium,             cache.m3.large,             cache.m3.xlarge,             cache.m3.2xlarge                     Compute optimized:                                  Previous generation: (not recommended)           C1 node types:             cache.c1.xlargeMemory optimized:                                                                 Current generation:            R6gd node types:             cache.r6gd.xlarge,             cache.r6gd.2xlarge,             cache.r6gd.4xlarge,             cache.r6gd.8xlarge,             cache.r6gd.12xlarge,             cache.r6gd.16xlarge           NoteThe r6gd family is available in the following regions: us-east-2, us-east-1, us-west-2, us-west-1, eu-west-1, eu-central-1, ap-northeast-1, ap-southeast-1, ap-southeast-2.             R6g node types:             cache.r6g.large,             cache.r6g.xlarge,             cache.r6g.2xlarge,             cache.r6g.4xlarge,             cache.r6g.12xlarge,             cache.r6g.24xlarge                      R5 node types:             cache.r5.large,             cache.r5.xlarge,             cache.r5.2xlarge,             cache.r5.4xlarge,             cache.r5.12xlarge,             cache.r5.24xlarge                      R4 node types:             cache.r4.large,             cache.r4.xlarge,             cache.r4.2xlarge,             cache.r4.4xlarge,             cache.r4.8xlarge,             cache.r4.16xlarge                                                                                                                                   Previous generation: (not recommended)           M2 node types:						             cache.m2.xlarge,             cache.m2.2xlarge,             cache.m2.4xlarge                      R3 node types:             cache.r3.large,             cache.r3.xlarge,             cache.r3.2xlarge,              cache.r3.4xlarge,             cache.r3.8xlarge
    ///
    /// For region availability, see Supported Node Types by Amazon Region
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheNodeType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cache_node_type: Option<cfn_resources::StrVal>,

    ///
    /// The name of the parameter group to associate with this replication group.       If this argument is omitted, the default cache parameter group for the specified engine is       used.
    ///
    /// If you are running Redis version 3.2.4 or later, only one node group (shard), and want to use a default parameter group,       we recommend that you specify the parameter group by name.
    ///
    /// To create a Redis (cluster mode disabled) replication group, use CacheParameterGroupName=default.redis3.2.               To create a Redis (cluster mode enabled) replication group, use CacheParameterGroupName=default.redis3.2.cluster.on.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cache_parameter_group_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of cache security group names to associate with this replication group.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cache_security_group_names: Option<Vec<String>>,

    ///
    /// The name of the cache subnet group to be used for the replication group.
    ///
    /// ImportantIf you're going to launch your cluster in an Amazon VPC,         you need to create a subnet group before you start creating a cluster.         For more information, see AWS::ElastiCache::SubnetGroup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cache_subnet_group_name: Option<cfn_resources::StrVal>,

    /// Enabled or Disabled. To modify cluster mode from Disabled to Enabled,    you must first set the cluster mode to Compatible. Compatible mode allows your Redis clients to connect using both cluster mode enabled and cluster mode disabled.    After you migrate all Redis clients to use cluster mode enabled, you can then complete cluster mode configuration and set the cluster mode to Enabled.    For more information, see Modify cluster mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cluster_mode: Option<cfn_resources::StrVal>,

    ///
    /// Enables data tiering. Data tiering is only supported for replication groups using the r6gd node type. This parameter must be set to true when using r6gd nodes.    For more information, see Data tiering.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataTieringEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub data_tiering_enabled: Option<bool>,

    ///
    /// The name of the cache engine to be used for the clusters in this replication group. The value must be set to Redis.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine: Option<cfn_resources::StrVal>,

    ///
    /// The version number of the cache engine to be used for the clusters in this replication group.       To view the supported cache engine versions, use the DescribeCacheEngineVersions operation.
    ///
    /// Important: You can upgrade to a newer engine version (see Selecting a Cache Engine and Version) in the ElastiCache User Guide,       but you cannot downgrade to an earlier engine version.       If you want to use an earlier engine version,       you must delete the existing cluster or replication group and       create it anew with the earlier engine version.
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
    /// The name of the Global datastore
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub global_replication_group_id: Option<cfn_resources::StrVal>,

    ///
    /// The network type you choose when creating a replication group, either ipv4 | ipv6. IPv6 is supported for workloads using Redis engine version 6.2 onward or Memcached engine version 1.6.6 on all instances built on the       Nitro system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ipv4 | ipv6
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpDiscovery")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ip_discovery: Option<ReplicationGroupIpDiscoveryEnum>,

    ///
    /// The ID of the KMS key used to encrypt the disk on the cluster.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    /// Specifies the destination, format and type of the logs.
    ///
    /// Required: No
    ///
    /// Type: List of LogDeliveryConfigurationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub log_delivery_configurations: Option<Vec<LogDeliveryConfigurationRequest>>,

    ///
    /// A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more information, see Minimizing Downtime: Multi-AZ.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiAZEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub multi_azenabled: Option<bool>,

    ///
    /// Must be either ipv4 | ipv6 | dual_stack. IPv6 is supported for workloads using Redis engine version 6.2 onward or Memcached engine version 1.6.6 on all instances built on the       Nitro system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dual_stack | ipv4 | ipv6
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub network_type: Option<ReplicationGroupNetworkTypeEnum>,

    ///
    /// NodeGroupConfiguration is a property of the AWS::ElastiCache::ReplicationGroup resource that configures an Amazon ElastiCache (ElastiCache) Redis cluster node group.
    ///
    /// If you set UseOnlineResharding to true, you can update NodeGroupConfiguration without interruption. When UseOnlineResharding is set to false, or is not specified, updating NodeGroupConfiguration results in replacement.
    ///
    /// Required: No
    ///
    /// Type: List of NodeGroupConfiguration
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NodeGroupConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub node_group_configuration: Option<Vec<NodeGroupConfiguration>>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS)       topic to which notifications are sent.
    ///
    /// NoteThe Amazon SNS topic owner must be the same as the cluster owner.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTopicArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notification_topic_arn: Option<cfn_resources::StrVal>,

    ///
    /// The number of clusters this replication group initially has.
    ///
    /// This parameter is not used if there is more than one node group (shard).        You should use ReplicasPerNodeGroup instead.
    ///
    /// If AutomaticFailoverEnabled is true, the value of this parameter must be at least 2.       If AutomaticFailoverEnabled is false you can omit this parameter (it will default to 1), or you       can explicitly set it to a value between 2 and 6.
    ///
    /// The maximum permitted value for NumCacheClusters is 6 (1 primary plus 5 replicas).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumCacheClusters")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub num_cache_clusters: Option<i64>,

    ///
    /// An optional parameter that specifies the number of node groups (shards) for this Redis (cluster mode enabled) replication group.       For Redis (cluster mode disabled) either omit this parameter or set it to 1.
    ///
    /// If you set UseOnlineResharding to true, you can update NumNodeGroups without interruption. When UseOnlineResharding is set to false, or is not specified, updating NumNodeGroups results in replacement.
    ///
    /// Default: 1
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NumNodeGroups")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub num_node_groups: Option<i64>,

    ///
    /// The port number on which each member of the replication group accepts connections.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub port: Option<i64>,

    ///
    /// A list of EC2 Availability Zones in which the replication group's clusters are created.       The order of the Availability Zones in the list is the order in which clusters are allocated.       The primary cluster is created in the first AZ in the list.
    ///
    /// This parameter is not used if there is more than one node group (shard).        You should use NodeGroupConfiguration instead.
    ///
    /// NoteIf you are creating your replication group in an Amazon VPC (recommended),       you can only locate clusters in Availability Zones associated with the subnets in the selected subnet group.The number of Availability Zones listed must equal the value of NumCacheClusters.
    ///
    /// Default: system chosen Availability Zones.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreferredCacheClusterAZs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub preferred_cache_cluster_azs: Option<Vec<String>>,

    ///
    /// Specifies the weekly time range during which maintenance  on the cluster is performed. It is specified as a range in  the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum  maintenance window is a 60 minute period.
    ///
    /// Valid values for ddd are:
    ///
    /// sun                                mon                                tue                                wed                                thu                                fri                                sat
    ///
    /// Example: sun:23:00-mon:01:30
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
    /// The identifier of the cluster that serves as the primary for this replication       group. This cluster must already exist and have a status of available.
    ///
    /// This parameter is not required if NumCacheClusters,       NumNodeGroups, or       ReplicasPerNodeGroup is specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrimaryClusterId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub primary_cluster_id: Option<cfn_resources::StrVal>,

    ///
    /// An optional parameter that specifies the number of replica nodes in each node group (shard).       Valid values are 0 to 5.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicasPerNodeGroup")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replicas_per_node_group: Option<i64>,

    ///
    /// A user-created description for the replication group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationGroupDescription")]
    pub replication_group_description: cfn_resources::StrVal,

    ///
    /// The replication group identifier. This parameter is stored as a lowercase string.
    ///
    /// Constraints:
    ///
    /// A name must contain from 1 to 40 alphanumeric characters or hyphens.               The first character must be a letter.               A name cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicationGroupId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replication_group_id: Option<cfn_resources::StrVal>,

    ///
    /// One or more Amazon VPC security groups associated with this replication group.
    ///
    /// Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud (Amazon VPC).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// A list of Amazon Resource Names (ARN) that uniquely identify      the Redis RDB snapshot files stored in Amazon S3.      The snapshot files are used to populate the new replication group.      The Amazon S3 object name in the ARN cannot contain any commas.      The new replication group will have the number of node groups (console: shards)      specified by the parameter NumNodeGroups or the number of      node groups configured by NodeGroupConfiguration regardless      of the number of ARNs specified here.
    ///
    /// Example of an Amazon S3 ARN: arn:aws:s3:::my_bucket/snapshot1.rdb
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotArns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshot_arns: Option<Vec<String>>,

    ///
    /// The name of a snapshot from which to restore data into the new replication group.       The snapshot status changes to restoring while the new replication group is being created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshot_name: Option<cfn_resources::StrVal>,

    ///
    /// The number of days for which ElastiCache retains automatic snapshots before deleting them.       For example, if you set SnapshotRetentionLimit to 5,       a snapshot that was taken today is retained for 5 days before being deleted.
    ///
    /// Default: 0 (i.e., automatic backups are disabled for this cluster).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshot_retention_limit: Option<i64>,

    ///
    /// The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).
    ///
    /// Example: 05:00-09:00
    ///
    /// If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotWindow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshot_window: Option<cfn_resources::StrVal>,

    ///
    /// The cluster ID that is used as the daily snapshot source for the replication group.       This parameter cannot be set for Redis (cluster mode enabled) replication groups.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshottingClusterId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub snapshotting_cluster_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of tags to be added to this resource.      Tags are comma-separated key,value pairs (e.g. Key=myKey, Value=myKeyValue. You can include multiple tags as shown following:      Key=myKey, Value=myKeyValue Key=mySecondKey, Value=mySecondKeyValue. Tags on replication groups will be replicated to all nodes.
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
    /// A flag that enables in-transit encryption when set to true.
    ///
    /// You cannot modify the value of TransitEncryptionEnabled       after the cluster is created. To enable in-transit encryption on a cluster       you must set TransitEncryptionEnabled to true       when you create a cluster.
    ///
    /// This parameter is valid only if the Engine parameter is redis,       the EngineVersion parameter is 3.2.6 or 4.x onward,       and the cluster is being created in an Amazon VPC.
    ///
    /// If you enable in-transit encryption, you must also specify a value for       CacheSubnetGroup.
    ///
    /// Required:       Only available when creating a replication group in an Amazon VPC using redis version 3.2.6 or 4.x onward.
    ///
    /// Default: false
    ///
    /// ImportantFor HIPAA compliance, you must specify TransitEncryptionEnabled as true,       an AuthToken, and a CacheSubnetGroup.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub transit_encryption_enabled: Option<bool>,

    ///
    /// A setting that allows you to migrate your clients to use in-transit encryption, with no downtime.
    ///
    /// When setting TransitEncryptionEnabled to true, you can set your    TransitEncryptionMode to preferred in the same request, to allow    both encrypted and unencrypted connections at the same time. Once you migrate all your Redis    clients to use encrypted connections you can modify the value to required to    allow encrypted connections only.
    ///
    /// Setting TransitEncryptionMode to required is a two-step    process that requires you to first set the TransitEncryptionMode to preferred,    after that you can set TransitEncryptionMode to required.
    ///
    /// This process will not trigger the replacement of the replication group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: preferred | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub transit_encryption_mode: Option<ReplicationGroupTransitEncryptionModeEnum>,

    ///
    /// The ID of user group to associate with the replication group.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_group_ids: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_configuration_end_point_address: CfnReplicationGroupconfigurationendpointaddress,

    #[serde(skip_serializing)]
    pub att_configuration_end_point_port: CfnReplicationGroupconfigurationendpointport,

    #[serde(skip_serializing)]
    pub att_primary_end_point_address: CfnReplicationGroupprimaryendpointaddress,

    #[serde(skip_serializing)]
    pub att_primary_end_point_port: CfnReplicationGroupprimaryendpointport,

    #[serde(skip_serializing)]
    pub att_read_end_point_addresses: CfnReplicationGroupreadendpointaddresses,

    #[serde(skip_serializing)]
    pub att_read_end_point_ports: CfnReplicationGroupreadendpointports,

    #[serde(skip_serializing)]
    pub att_reader_end_point_address: CfnReplicationGroupreaderendpointaddress,

    #[serde(skip_serializing)]
    pub att_reader_end_point_port: CfnReplicationGroupreaderendpointport,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReplicationGroupIpDiscoveryEnum {
    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

    /// ipv6
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl Default for ReplicationGroupIpDiscoveryEnum {
    fn default() -> Self {
        ReplicationGroupIpDiscoveryEnum::Ipv4
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReplicationGroupNetworkTypeEnum {
    /// dual_stack
    #[serde(rename = "dual_stack")]
    Dualstack,

    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

    /// ipv6
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl Default for ReplicationGroupNetworkTypeEnum {
    fn default() -> Self {
        ReplicationGroupNetworkTypeEnum::Dualstack
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ReplicationGroupTransitEncryptionModeEnum {
    /// preferred
    #[serde(rename = "preferred")]
    Preferred,

    /// required
    #[serde(rename = "required")]
    Required,
}

impl Default for ReplicationGroupTransitEncryptionModeEnum {
    fn default() -> Self {
        ReplicationGroupTransitEncryptionModeEnum::Preferred
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupconfigurationendpointaddress;
impl CfnReplicationGroupconfigurationendpointaddress {
    pub fn att_name(&self) -> &'static str {
        r#"ConfigurationEndPoint.Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupconfigurationendpointport;
impl CfnReplicationGroupconfigurationendpointport {
    pub fn att_name(&self) -> &'static str {
        r#"ConfigurationEndPoint.Port"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupprimaryendpointaddress;
impl CfnReplicationGroupprimaryendpointaddress {
    pub fn att_name(&self) -> &'static str {
        r#"PrimaryEndPoint.Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupprimaryendpointport;
impl CfnReplicationGroupprimaryendpointport {
    pub fn att_name(&self) -> &'static str {
        r#"PrimaryEndPoint.Port"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupreadendpointaddresses;
impl CfnReplicationGroupreadendpointaddresses {
    pub fn att_name(&self) -> &'static str {
        r#"ReadEndPoint.Addresses"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupreadendpointports;
impl CfnReplicationGroupreadendpointports {
    pub fn att_name(&self) -> &'static str {
        r#"ReadEndPoint.Ports"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupreaderendpointaddress;
impl CfnReplicationGroupreaderendpointaddress {
    pub fn att_name(&self) -> &'static str {
        r#"ReaderEndPoint.Address"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnReplicationGroupreaderendpointport;
impl CfnReplicationGroupreaderendpointport {
    pub fn att_name(&self) -> &'static str {
        r#"ReaderEndPoint.Port"#
    }
}

impl cfn_resources::CfnResource for CfnReplicationGroup {
    fn type_string(&self) -> &'static str {
        "AWS::ElastiCache::ReplicationGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The configuration details of the CloudWatch Logs destination. Note that this field is marked    as required but only if CloudWatch Logs was chosen as the destination.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CloudWatchLogsDestinationDetails {
    /// The name of the CloudWatch Logs log group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudWatchLogsDestinationDetails {
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

/// Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DestinationDetails {
    /// The configuration details of the CloudWatch Logs destination. Note that this field is marked      as required but only if CloudWatch Logs was chosen as the destination.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchLogsDestinationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsDetails")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cloud_watch_logs_details: Option<CloudWatchLogsDestinationDetails>,

    /// The configuration details of the Kinesis Data Firehose destination. Note that this field is marked    as required but only if Kinesis Data Firehose was chosen as the destination.
    ///
    /// Required: No
    ///
    /// Type: KinesisFirehoseDestinationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseDetails")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kinesis_firehose_details: Option<KinesisFirehoseDestinationDetails>,
}

impl cfn_resources::CfnResource for DestinationDetails {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloud_watch_logs_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_firehose_details
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration details of the Kinesis Data Firehose destination. Note that this field is marked    as required but only if Kinesis Data Firehose was chosen as the destination.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct KinesisFirehoseDestinationDetails {
    /// The name of the Kinesis Data Firehose delivery stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KinesisFirehoseDestinationDetails {
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

/// Specifies the destination, format and type of the logs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LogDeliveryConfigurationRequest {
    /// Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.
    ///
    /// Required: Yes
    ///
    /// Type: DestinationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationDetails")]
    pub destination_details: DestinationDetails,

    /// Specify either CloudWatch Logs or Kinesis Data Firehose as the destination type. Valid values are either cloudwatch-logs or kinesis-firehose.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationType")]
    pub destination_type: cfn_resources::StrVal,

    /// Valid values are either json or text.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogFormat")]
    pub log_format: cfn_resources::StrVal,

    /// Valid value is either slow-log, which refers to slow-log or engine-log.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogType")]
    pub log_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LogDeliveryConfigurationRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_details.validate()?;

        Ok(())
    }
}

/// NodeGroupConfiguration is a property of the AWS::ElastiCache::ReplicationGroup resource that configures an Amazon ElastiCache (ElastiCache) Redis cluster node group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NodeGroupConfiguration {
    ///
    /// Either the ElastiCache for Redis supplied 4-digit id or a user supplied id for the node group these       configuration values apply to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4
    ///
    /// Pattern: \d+
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NodeGroupId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub node_group_id: Option<cfn_resources::StrVal>,

    ///
    /// The Availability Zone where the primary node of this node group (shard) is launched.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrimaryAvailabilityZone")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub primary_availability_zone: Option<cfn_resources::StrVal>,

    ///
    /// A list of Availability Zones to be used for the read replicas.       The number of Availability Zones in this list must match the value of ReplicaCount       or ReplicasPerNodeGroup if not specified.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicaAvailabilityZones")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replica_availability_zones: Option<Vec<String>>,

    ///
    /// The number of read replica nodes in this node group (shard).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub replica_count: Option<i64>,

    ///
    /// A string of comma-separated values where the first set of values are the slot numbers (zero based), and the second set of values     are the keyspaces for each slot. The following example specifies three slots (numbered 0, 1, and 2): 0,1,2,0-4999,5000-9999,10000-16,383.
    ///
    /// If you don't specify a value, ElastiCache allocates keys equally among each slot.
    ///
    /// When you use an UseOnlineResharding update policy to update the number of node groups without interruption, ElastiCache evenly distributes the keyspaces     between the specified number of slots. This cannot be updated later. Therefore, after updating the number of node groups in this way, you should     remove the value specified for the Slots property of each NodeGroupConfiguration from the stack template, as it no longer reflects     the actual values in each node group. For more information, see UseOnlineResharding Policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Slots")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub slots: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for NodeGroupConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.node_group_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 4 as _ {
                    return Err(format!(
                        "Max validation failed on field 'node_group_id'. {} is greater than 4",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.node_group_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'node_group_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

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
