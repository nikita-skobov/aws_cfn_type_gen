/// Information about the integration of DevOps Guru with CloudWatch log groups for log anomaly detection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLogAnomalyDetectionIntegration {
    #[serde(skip_serializing)]
    pub att_account_id: CfnLogAnomalyDetectionIntegrationaccountid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLogAnomalyDetectionIntegrationaccountid;
impl CfnLogAnomalyDetectionIntegrationaccountid {
    pub fn att_name(&self) -> &'static str {
        r#"AccountId"#
    }
}

impl cfn_resources::CfnResource for CfnLogAnomalyDetectionIntegration {
    fn type_string(&self) -> &'static str {
        "AWS::DevOpsGuru::LogAnomalyDetectionIntegration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
