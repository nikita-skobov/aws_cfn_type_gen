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
    pub application_name: cfn_resources::StrVal,

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
    fn type_string(&self) -> &'static str {
        "AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'application_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.application_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'application_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.cloud_watch_logging_option.validate()?;

        Ok(())
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
    pub log_stream_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudWatchLoggingOption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.log_stream_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'log_stream_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.log_stream_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'log_stream_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
