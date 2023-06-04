/// The AWS::ApiGateway::Account resource specifies the IAM role that Amazon API Gateway uses to write API logs to Amazon CloudWatch Logs. To avoid overwriting other roles, you should only have one AWS::ApiGateway::Account resource per region per account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAccount {
    ///
    /// The ARN of an Amazon CloudWatch role for the current Account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchRoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cloud_watch_role_arn: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_id: CfnAccountid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccountid;
impl CfnAccountid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnAccount {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Account"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
