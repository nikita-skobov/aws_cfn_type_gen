

/// Configure logging.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLogging {


    /// 
    /// The default log level. Valid Values: DEBUG | INFO | ERROR | WARN | DISABLED
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultLogLevel")]
    pub default_log_level: LoggingDefaultLogLevelEnum,


    /// 
    /// The role ARN used for the log.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The account ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccountId")]
    pub account_id: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LoggingDefaultLogLevelEnum {

    /// DEBUG
    #[serde(rename = "DEBUG")]
    Debug,

    /// INFO
    #[serde(rename = "INFO")]
    Info,

    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

    /// WARN
    #[serde(rename = "WARN")]
    Warn,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

}

impl Default for LoggingDefaultLogLevelEnum {
    fn default() -> Self {
        LoggingDefaultLogLevelEnum::Debug
    }
}


impl cfn_resources::CfnResource for CfnLogging {
    fn type_string() -> &'static str {
        "AWS::IoT::Logging"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
