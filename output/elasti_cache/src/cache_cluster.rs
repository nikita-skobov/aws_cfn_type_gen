

/// The AWS::ElastiCache::CacheCluster type creates an Amazon ElastiCache cache cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCacheCluster {


    /// 
    /// Specifies whether the nodes in this Memcached cluster are created in a single Availability Zone or       created across multiple Availability Zones in the cluster's region.
    /// 
    /// This parameter is only supported for Memcached clusters.
    /// 
    /// If the AZMode and PreferredAvailabilityZones are not specified,       ElastiCache assumes single-az mode.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: cross-az | single-az
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AZMode")]
    pub azmode: Option<CacheClusterAZModeEnum>,


    /// 
    /// If you are running Redis engine version 6.0 or later, set this parameter to yes if you want to opt-in to the next minor version upgrade campaign. This parameter is disabled for previous versions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,


    /// 
    /// The compute and memory capacity of the nodes in the node group (shard).
    /// 
    /// The following node types are supported by ElastiCache. 			Generally speaking, the current generation types provide more memory and computational power 			at lower cost when compared to their equivalent previous generation counterparts.     Changing the CacheNodeType of a Memcached instance is currently not supported. If you need to scale using Memcached, we recommend forcing a replacement update by changing the LogicalResourceId of the resource.
    /// 
    /// General purpose:                                              Current generation:                                  M6g node types:             cache.m6g.large,             cache.m6g.xlarge,             cache.m6g.2xlarge,             cache.m6g.4xlarge,             cache.m6g.8xlarge,             cache.m6g.12xlarge,             cache.m6g.16xlarge,             cache.m6g.24xlarge                                    	                      M5 node types:             cache.m5.large,             cache.m5.xlarge,             cache.m5.2xlarge,             cache.m5.4xlarge,             cache.m5.12xlarge,             cache.m5.24xlarge                                    	                                 M4 node types:             cache.m4.large,             cache.m4.xlarge,             cache.m4.2xlarge,             cache.m4.4xlarge,             cache.m4.10xlarge                      T4g node types:             cache.t4g.micro,             cache.t4g.small,             cache.t4g.medium                                  T3 node types:             cache.t3.micro,             cache.t3.small,             cache.t3.medium                                 T2 node types:             cache.t2.micro,             cache.t2.small,             cache.t2.medium                                                      Previous generation: (not recommended)           T1 node types:             cache.t1.micro                      M1 node types:             cache.m1.small,             cache.m1.medium,             cache.m1.large,             cache.m1.xlarge                      M3 node types:             cache.m3.medium,             cache.m3.large,             cache.m3.xlarge,             cache.m3.2xlarge                     Compute optimized:                                  Previous generation: (not recommended)           C1 node types:             cache.c1.xlargeMemory optimized:                                                                 Current generation:                       R6gd node types:             cache.r6gd.xlarge,             cache.r6gd.2xlarge,             cache.r6gd.4xlarge,             cache.r6gd.8xlarge,             cache.r6gd.12xlarge,             cache.r6gd.16xlarge           NoteThe r6gd family is available in the following regions: us-east-2, us-east-1, us-west-2, us-west-1, eu-west-1, eu-central-1, ap-northeast-1, ap-southeast-1, ap-southeast-2.                   R6g node types:             cache.r6g.large,             cache.r6g.xlarge,             cache.r6g.2xlarge,             cache.r6g.4xlarge,             cache.r6g.8xlarge,             cache.r6g.12xlarge,             cache.r6g.16xlarge,             cache.r6g.24xlarge                      R5 node types:             cache.r5.large,             cache.r5.xlarge,             cache.r5.2xlarge,             cache.r5.4xlarge,             cache.r5.12xlarge,             cache.r5.24xlarge                      R4 node types:             cache.r4.large,             cache.r4.xlarge,             cache.r4.2xlarge,             cache.r4.4xlarge,             cache.r4.8xlarge,             cache.r4.16xlarge                                                                                                                                   Previous generation: (not recommended)           M2 node types:						             cache.m2.xlarge,             cache.m2.2xlarge,             cache.m2.4xlarge                      R3 node types:             cache.r3.large,             cache.r3.xlarge,             cache.r3.2xlarge,              cache.r3.4xlarge,             cache.r3.8xlarge
    /// 
    /// For region availability, see Supported Node Types by Region
    /// 
    /// Additional node type info
    /// 
    /// All current generation instance types are created in Amazon VPC by default.Redis append-only files (AOF) are not supported for T1 or T2 instances.Redis Multi-AZ with automatic failover is not supported on T1 instances.Redis configuration variables appendonly and         appendfsync are not supported on Redis version 2.8.22 and later.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: String,


    /// 
    /// The name of the parameter group to associate with this cluster.       If this argument is omitted, the default parameter group for the specified engine is used.       You cannot use any parameter group which has cluster-enabled='yes' when creating a cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,


    /// 
    /// A list of security group names to associate with this cluster.
    /// 
    /// Use this parameter only when you are creating a cluster outside of an Amazon Virtual Private Cloud (Amazon VPC).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheSecurityGroupNames")]
    pub cache_security_group_names: Option<Vec<String>>,


    /// 
    /// The name of the subnet group to be used for the cluster.
    /// 
    /// Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).
    /// 
    /// ImportantIf you're going to launch your cluster in an Amazon VPC,         you need to create a subnet group before you start creating a cluster.         For more information, see AWS::ElastiCache::SubnetGroup.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CacheSubnetGroupName")]
    pub cache_subnet_group_name: Option<String>,


    /// 
    /// A name for the cache cluster. If you don't specify a name, AWSCloudFormation generates a     unique physical ID and uses that ID for the cache cluster. For more information,     see Name Type.
    /// 
    /// The name must contain 1 to 50 alphanumeric characters or hyphens. The name must     start with a letter and cannot end with a hyphen or contain two consecutive     hyphens.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,


    /// 
    /// The name of the cache engine to be used for this cluster.
    /// 
    /// Valid values for this parameter are: memcached | redis
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Engine")]
    pub engine: String,


    /// 
    /// The version number of the cache engine to be used for this cluster.       To view the supported cache engine versions, use the DescribeCacheEngineVersions operation.
    /// 
    /// Important: You can upgrade to a newer engine version (see Selecting a Cache Engine and Version), but you cannot downgrade to an earlier engine version.       If you want to use an earlier engine version,       you must delete the existing cluster or replication group and create it anew with the earlier engine version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,


    /// 
    /// The network type you choose when modifying a cluster, either ipv4 | ipv6. IPv6 is supported for workloads using Redis engine version 6.2 onward or Memcached engine version 1.6.6 on all instances built on the       Nitro system.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ipv4 | ipv6
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpDiscovery")]
    pub ip_discovery: Option<CacheClusterIpDiscoveryEnum>,


    /// Specifies the destination, format and type of the logs.
    ///
    /// Required: No
    ///
    /// Type: List of LogDeliveryConfigurationRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDeliveryConfigurations")]
    pub log_delivery_configurations: Option<Vec<LogDeliveryConfigurationRequest>>,


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
    pub network_type: Option<CacheClusterNetworkTypeEnum>,


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic      to which notifications are sent.
    /// 
    /// NoteThe Amazon SNS topic owner must be the same as the cluster owner.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTopicArn")]
    pub notification_topic_arn: Option<String>,


    /// 
    /// The number of cache nodes that the cache cluster should have.
    /// 
    /// NoteHowever, if the PreferredAvailabilityZone and PreferredAvailabilityZones properties were not previously specified and you don't specify any new values,     an update requires replacement.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NumCacheNodes")]
    pub num_cache_nodes: i64,


    /// 
    /// The port number on which each of the cache nodes accepts connections.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The EC2 Availability Zone in which the cluster is created.
    /// 
    /// All nodes belonging to this cluster are placed in the preferred Availability Zone.       If you want to create your nodes across multiple Availability Zones, use PreferredAvailabilityZones.
    /// 
    /// Default: System chosen Availability Zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PreferredAvailabilityZone")]
    pub preferred_availability_zone: Option<String>,


    /// 
    /// A list of the Availability Zones in which cache nodes are created. The order of the zones in the list is not important.
    /// 
    /// This option is only supported on Memcached.
    /// 
    /// NoteIf you are creating your cluster in an Amazon VPC (recommended) you can only locate nodes in Availability Zones that are associated with the subnets in the selected subnet group.The number of Availability Zones listed must equal the value of NumCacheNodes.
    /// 
    /// If you want all the nodes in the same Availability Zone, use PreferredAvailabilityZone instead, or       repeat the Availability Zone multiple times in the list.
    /// 
    /// Default: System chosen Availability Zones.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PreferredAvailabilityZones")]
    pub preferred_availability_zones: Option<Vec<String>>,


    /// 
    /// Specifies the weekly time range during which maintenance       on the cluster is performed. It is specified as a range in       the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum       maintenance window is a 60 minute period.       Valid values for ddd are:
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
    pub preferred_maintenance_window: Option<String>,


    /// 
    /// A single-element string list containing an Amazon Resource Name (ARN) that uniquely identifies       a Redis RDB snapshot file stored in Amazon S3.       The snapshot file is used to populate the node group (shard).       The Amazon S3 object name in the ARN cannot contain any commas.
    /// 
    /// NoteThis parameter is only valid if the Engine parameter is redis.
    /// 
    /// Example of an Amazon S3 ARN: arn:aws:s3:::my_bucket/snapshot1.rdb
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotArns")]
    pub snapshot_arns: Option<Vec<String>>,


    /// 
    /// The name of a Redis snapshot from which to restore data into the new node group (shard).       The snapshot status changes to restoring while the new node group (shard) is being created.
    /// 
    /// NoteThis parameter is only valid if the Engine parameter is redis.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,


    /// 
    /// The number of days for which ElastiCache retains automatic snapshots before deleting them.      For example, if you set SnapshotRetentionLimit to 5,      a snapshot taken today is retained for 5 days before being deleted.
    /// 
    /// NoteThis parameter is only valid if the Engine parameter is redis.
    /// 
    /// Default: 0 (i.e., automatic backups are disabled for this cache cluster).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotRetentionLimit")]
    pub snapshot_retention_limit: Option<i64>,


    /// 
    /// The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).
    /// 
    /// Example: 05:00-09:00
    /// 
    /// If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.
    /// 
    /// NoteThis parameter is only valid if the Engine parameter is redis.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotWindow")]
    pub snapshot_window: Option<String>,


    /// 
    /// A list of tags to be added to this resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A flag that enables in-transit encryption when set to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryptionEnabled")]
    pub transit_encryption_enabled: Option<bool>,


    /// 
    /// One or more VPC security groups associated with the cluster.
    /// 
    /// Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).
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
pub enum CacheClusterNetworkTypeEnum {

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

impl Default for CacheClusterNetworkTypeEnum {
    fn default() -> Self {
        CacheClusterNetworkTypeEnum::Dualstack
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CacheClusterIpDiscoveryEnum {

    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

    /// ipv6
    #[serde(rename = "ipv6")]
    Ipv6,

}

impl Default for CacheClusterIpDiscoveryEnum {
    fn default() -> Self {
        CacheClusterIpDiscoveryEnum::Ipv4
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CacheClusterAZModeEnum {

    /// cross-az
    #[serde(rename = "cross-az")]
    Crossaz,

    /// single-az
    #[serde(rename = "single-az")]
    Singleaz,

}

impl Default for CacheClusterAZModeEnum {
    fn default() -> Self {
        CacheClusterAZModeEnum::Crossaz
    }
}


impl cfn_resources::CfnResource for CfnCacheCluster {
    fn type_string() -> &'static str {
        "AWS::ElastiCache::CacheCluster"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Configuration details of a CloudWatch Logs destination. Note that this field is marked    as required but only if CloudWatch Logs was chosen as the destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogsDestinationDetails {


    /// The name of the CloudWatch Logs log group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: String,

}




/// Configuration details of either a CloudWatch Logs destination or Kinesis Data Firehose destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationDetails {


    /// The configuration details of the CloudWatch Logs destination. Note that this field is marked    as required but only if CloudWatch Logs was chosen as the destination.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchLogsDestinationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsDetails")]
    pub cloud_watch_logs_details: Option<CloudWatchLogsDestinationDetails>,


    /// The configuration details of the Kinesis Data Firehose destination. Note that this field is marked    as required but only if Kinesis Data Firehose was chosen as the destination.
    ///
    /// Required: No
    ///
    /// Type: KinesisFirehoseDestinationDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseDetails")]
    pub kinesis_firehose_details: Option<KinesisFirehoseDestinationDetails>,

}




/// The configuration details of the Kinesis Data Firehose destination. Note that this field is marked   as required but only if Kinesis Data Firehose was chosen as the destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseDestinationDetails {


    /// The name of the Kinesis Data Firehose delivery stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStream")]
    pub delivery_stream: String,

}




/// Specifies the destination, format and type of the logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub destination_type: String,


    /// Valid values are either json or text.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogFormat")]
    pub log_format: String,


    /// Valid value is either slow-log, which refers to slow-log or engine-log.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogType")]
    pub log_type: String,

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


