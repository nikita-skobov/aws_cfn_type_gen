

/// Information about the integration of DevOps Guru with CloudWatch log groups for log anomaly detection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLogAnomalyDetectionIntegration {

}

impl cfn_resources::CfnResource for CfnLogAnomalyDetectionIntegration {
    fn type_string() -> &'static str {
        "AWS::DevOpsGuru::LogAnomalyDetectionIntegration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
