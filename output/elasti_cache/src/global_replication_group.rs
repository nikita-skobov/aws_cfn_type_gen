

/// Consists of a primary cluster that accepts writes and an associated secondary cluster that resides in a different Amazon region. The secondary cluster accepts only reads. The primary     cluster automatically replicates updates to the secondary cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGlobalReplicationGroup {


    /// Specifies whether a read-only replica is automatically promoted to read/write primary if the existing primary fails.
    /// 
    /// AutomaticFailoverEnabled must be enabled for Redis (cluster mode enabled) replication groups.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticFailoverEnabled")]
    pub automatic_failover_enabled: Option<bool>,


    /// 
    /// The cache node type of the Global datastore
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: Option<String>,


    /// The name of the cache parameter group to use with the Global datastore. It must be compatible with the major engine version used by the Global datastore.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,


    /// 
    /// The Elasticache Redis engine version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,


    /// The number of node groups that comprise the Global Datastore.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalNodeGroupCount")]
    pub global_node_group_count: Option<i64>,


    /// 
    /// The optional description of the Global datastore
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalReplicationGroupDescription")]
    pub global_replication_group_description: Option<String>,


    /// The suffix name of a Global Datastore. The suffix guarantees uniqueness of the Global Datastore name across multiple regions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlobalReplicationGroupIdSuffix")]
    pub global_replication_group_id_suffix: Option<String>,


    /// 
    /// The replication groups that comprise the Global datastore.
    /// 
    /// Required: Yes
    ///
    /// Type: List of GlobalReplicationGroupMember
    ///
    /// Update requires: No interruption
    #[serde(rename = "Members")]
    pub members: Vec<GlobalReplicationGroupMember>,


    /// The Regions that comprise the Global Datastore.
    ///
    /// Required: No
    ///
    /// Type: List of RegionalConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionalConfigurations")]
    pub regional_configurations: Option<Vec<RegionalConfiguration>>,

}



impl cfn_resources::CfnResource for CfnGlobalReplicationGroup {
    fn type_string() -> &'static str {
        "AWS::ElastiCache::GlobalReplicationGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A member of a Global datastore. It contains the Replication Group Id, the Amazon region and the role of the replication group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlobalReplicationGroupMember {


    /// 
    /// The replication group id of the Global datastore member.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationGroupId")]
    pub replication_group_id: Option<String>,


    /// 
    /// The Amazon region of the Global datastore member.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationGroupRegion")]
    pub replication_group_region: Option<String>,


    /// 
    /// Indicates the role of the replication group, PRIMARY or SECONDARY.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Role")]
    pub role: Option<String>,

}




/// A list of the replication groups
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RegionalConfiguration {


    /// 
    /// The name of the secondary cluster
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationGroupId")]
    pub replication_group_id: Option<String>,


    /// 
    /// The Amazon region where the cluster is stored
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationGroupRegion")]
    pub replication_group_region: Option<String>,


    /// A list of PreferredAvailabilityZones objects that specifies the configuration of a node group in the resharded cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of ReshardingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReshardingConfigurations")]
    pub resharding_configurations: Option<Vec<ReshardingConfiguration>>,

}




/// A list of PreferredAvailabilityZones objects that specifies       the configuration of a node group in the resharded cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReshardingConfiguration {


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
    /// Update requires: No interruption
    #[serde(rename = "NodeGroupId")]
    pub node_group_id: Option<String>,


    /// 
    /// A list of preferred availability zones for the nodes in this cluster.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredAvailabilityZones")]
    pub preferred_availability_zones: Option<Vec<String>>,

}


