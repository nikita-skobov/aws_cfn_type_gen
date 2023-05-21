/// Create remote VPC connection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVpcConnection {
    ///
    /// The type of private link authentication.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Authentication")]
    pub authentication: cfn_resources::StrVal,

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
    /// Create tags when creating the VPC connection.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The Amazon Resource Name (ARN) of the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetClusterArn")]
    pub target_cluster_arn: cfn_resources::StrVal,

    ///
    /// The VPC id of the remote client.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnVpcConnection {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::VpcConnection"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
