
pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationFactor")]
    pub replication_factor: usize,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "IAMRoleARN")]
    pub iamrole_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationTopicARN")]
    pub notification_topic_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterEndpointEncryptionType")]
    pub cluster_endpoint_encryption_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<SSESpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct SSESpecification {
    #[serde(rename = "SSEEnabled")]
    pub sseenabled: Option<bool>,

}


}

pub mod cfn_parameter_group {

#[derive(serde::Serialize, Default)]
pub struct CfnParameterGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ParameterNameValues")]
    pub parameter_name_values: Option<()>,

}



}

pub mod cfn_subnet_group {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetGroup {
    /// No documentation provided by AWS
    #[serde(rename = "SubnetGroupName")]
    pub subnet_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}



}
