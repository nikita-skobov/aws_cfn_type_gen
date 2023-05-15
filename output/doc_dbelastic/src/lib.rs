
pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// No documentation provided by AWS
    #[serde(rename = "AuthType")]
    pub auth_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ShardCount")]
    pub shard_count: usize,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AdminUserPassword")]
    pub admin_user_password: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AdminUserName")]
    pub admin_user_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ShardCapacity")]
    pub shard_capacity: usize,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
