

/// The AWS::ApiGateway::Account resource specifies the IAM role that Amazon API Gateway uses to write API logs to Amazon CloudWatch Logs. To avoid overwriting other roles, you should only have one AWS::ApiGateway::Account resource per region per account.
#[derive(Default, serde::Serialize)]
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
    pub cloud_watch_role_arn: Option<String>,

}
