

/// Create or update cluster policy.
#[derive(Default, serde::Serialize)]
pub struct CfnClusterPolicy {


    /// 
    /// Resource policy for the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,


    /// 
    /// The Amazon Resource Name (ARN) that uniquely identifies the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,

}
