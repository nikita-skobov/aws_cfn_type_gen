

/// Create remote VPC connection.
#[derive(Default, serde::Serialize)]
pub struct CfnVpcConnection {


    /// 
    /// Create tags when creating the VPC connection.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The security groups to attach to the ENIs for the broker nodes.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,


    /// 
    /// The type of private link authentication.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Authentication")]
    pub authentication: String,


    /// 
    /// The Amazon Resource Name (ARN) of the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetClusterArn")]
    pub target_cluster_arn: String,


    /// 
    /// The VPC id of the remote client.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


    /// 
    /// The list of subnets in the client VPC to connect to.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientSubnets")]
    pub client_subnets: Vec<String>,

}
