/// The AWS::MSK::BatchScramSecret resource Property description not available. for MSK.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnBatchScramSecret {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretArnList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn_list: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CfnBatchScramSecret {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::BatchScramSecret"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
