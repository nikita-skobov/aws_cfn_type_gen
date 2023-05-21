

/// Creates a DAX cluster. All nodes in the cluster run the same DAX caching software.
#[derive(Default, serde::Serialize)]
pub struct CfnCluster {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic to which       notifications will be sent.
    /// 
    /// NoteThe Amazon SNS topic owner must be same as the DAX         cluster owner.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTopicARN")]
    pub notification_topic_arn: Option<String>,


    /// 
    /// A list of security group IDs to be assigned to each node in the DAX       cluster. (Each of the security group ID is system-generated.)
    /// 
    /// If this parameter is not specified, DAX assigns the default VPC       security group to each node.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// A range of time when maintenance of DAX cluster software will be performed. For       example: sun:01:00-sun:09:00. Cluster maintenance normally takes less than       30 minutes, and is performed automatically within the maintenance window.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,


    /// 
    /// The node type for the nodes in the cluster. (All nodes in a DAX cluster are of       the same type.)
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NodeType")]
    pub node_type: String,


    /// 
    /// Represents the settings used to enable server-side encryption on the       cluster.
    /// 
    /// Required: No
    ///
    /// Type: SSESpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<SSESpecification>,


    /// 
    /// The encryption type of the cluster's endpoint. Available values are:
    /// 
    /// NONE - The cluster's endpoint will be unencrypted.               TLS - The cluster's endpoint will be encrypted with Transport           Layer Security, and will provide an x509 certificate for           authentication.
    /// 
    /// The default value is NONE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | TLS
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterEndpointEncryptionType")]
    pub cluster_endpoint_encryption_type: Option<String>,


    /// 
    /// The parameter group to be associated with the DAX cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,


    /// 
    /// The description of the cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A valid Amazon Resource Name (ARN) that identifies an IAM role. At runtime, DAX       will assume this role and use the role's permissions to access DynamoDB on your       behalf.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IAMRoleARN")]
    pub iamrole_arn: String,


    /// 
    /// The number of nodes in the DAX cluster. A replication factor of 1       will create a single-node cluster, without any read replicas. For additional fault       tolerance, you can create a multiple node cluster with one or more read replicas. To do       this, set ReplicationFactor to a number between 3 (one primary and two read       replicas) and 10 (one primary and nine read replicas). If the         AvailabilityZones parameter is provided, its length must equal the         ReplicationFactor.
    /// 
    /// Note        AWS recommends that you have at least two read replicas per         cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationFactor")]
    pub replication_factor: i64,


    /// 
    /// The Availability Zones (AZs) in which the cluster nodes will reside after the       cluster has been created or updated. If provided, the length of this list must equal the         ReplicationFactor parameter. If you omit this parameter, DAX will spread the nodes across Availability Zones for the highest       availability.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,


    /// 
    /// A set of tags to associate with the DAX cluster.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// The name of the subnet group to be used for the replication group.
    /// 
    /// ImportantDAX clusters can only run in an Amazon VPC environment. All of the subnets         that you specify in a subnet group must exist in the same VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: Option<String>,


    /// 
    /// The name of the DAX cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,

}


/// Represents the settings used to enable server-side encryption.
#[derive(Default, serde::Serialize)]
pub struct SSESpecification {


    /// 
    /// Indicates whether server-side encryption is enabled (true) or disabled (false) on       the cluster.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SSEEnabled")]
    pub sseenabled: Option<bool>,

}
