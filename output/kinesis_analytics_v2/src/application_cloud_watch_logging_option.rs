

/// Adds an Amazon CloudWatch log stream to monitor application configuration errors.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationCloudWatchLoggingOption {


    /// 
    /// The name of the application.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: String,


    /// 
    /// Provides a description of Amazon CloudWatch logging options, including the log stream    Amazon Resource Name (ARN).
    /// 
    /// Required: Yes
    ///
    /// Type: CloudWatchLoggingOption
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOption")]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,

}



impl cfn_resources::CfnResource for CfnApplicationCloudWatchLoggingOption {
    fn type_string() -> &'static str {
        "AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Provides a description of Amazon CloudWatch logging options, including the log stream    Amazon Resource Name (ARN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLoggingOption {


    /// 
    /// The ARN of the CloudWatch log to receive application messages.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogStreamARN")]
    pub log_stream_arn: String,

}


