/// Create or update cluster policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClusterPolicy {
    ///
    /// The Amazon Resource Name (ARN) that uniquely identifies the cluster.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_current_version: CfnClusterPolicycurrentversion,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClusterPolicycurrentversion;
impl CfnClusterPolicycurrentversion {
    pub fn att_name(&self) -> &'static str {
        r#"CurrentVersion"#
    }
}

impl cfn_resources::CfnResource for CfnClusterPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::ClusterPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
