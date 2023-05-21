/// The     AWS::Greengrass::LoggerDefinitionVersion resource represents a logger definition version for AWS IoT Greengrass.     A logger definition version contains a list of loggers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoggerDefinitionVersion {
    ///
    /// The ID of the logger definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: cfn_resources::StrVal,

    ///
    /// The loggers in this version.
    ///
    /// Required: Yes
    ///
    /// Type: List of Logger
    ///
    /// Update requires: Replacement
    #[serde(rename = "Loggers")]
    pub loggers: Vec<Logger>,
}

impl cfn_resources::CfnResource for CfnLoggerDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::LoggerDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A logger represents logging settings for the AWS IoT Greengrass group, which can be stored in CloudWatch and the local file system of your core device.   All log entries include a timestamp, log level, and information about the event. For more information, see Monitoring with AWS IoT Greengrass Logs in the    AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Loggers 		 property of the AWS::Greengrass::LoggerDefinitionVersion resource contains a      list of Logger property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Logger {
    ///
    /// The source of the log event. Valid values are GreengrassSystem or Lambda. 				 When GreengrassSystem is used, events from Greengrass system components are logged. 				 When Lambda is used, events from user-defined Lambda functions are logged.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Component")]
    pub component: cfn_resources::StrVal,

    ///
    /// A descriptive or arbitrary ID for the logger. This value must be unique within       the logger definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The log-level threshold. Log events below this threshold are filtered out and aren't stored. 				 Valid values are DEBUG, INFO (recommended), WARN, 				 ERROR, or FATAL.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Level")]
    pub level: cfn_resources::StrVal,

    ///
    /// The amount of file space (in KB) to use when writing logs to the local file system. 				 This property does not apply for CloudWatch Logs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<i64>,

    ///
    /// The storage mechanism for log events. Valid values are FileSystem or AWSCloudWatch. 				 When AWSCloudWatch is used, log events are sent to CloudWatch Logs. 				 When FileSystem is used, log events are stored on the local file system.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Logger {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
