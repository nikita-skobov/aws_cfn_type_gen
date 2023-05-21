

/// The     AWS::Greengrass::LoggerDefinition resource represents a logger definition for AWS IoT Greengrass.   Logger definitions are used to organize your logger definition versions.
///
/// Logger definitions can reference multiple logger definition versions. All logger definition versions      must be associated with a logger definition. Each logger definition version can contain one or more loggers.
#[derive(Default, serde::Serialize)]
pub struct CfnLoggerDefinition {


    /// 
    /// The name of the logger definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The logger definition version to include when the logger definition is created.          A logger definition version contains a list of          logger property types.
    /// 
    /// NoteTo associate a logger definition version after the logger definition is created, 				   create an AWS::Greengrass::LoggerDefinitionVersion 				   resource and specify the ID of this logger definition.
    /// 
    /// Required: No
    ///
    /// Type: LoggerDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<LoggerDefinitionVersion>,


    /// 
    /// Application-specific metadata to attach to the logger definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    /// 
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    /// 
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,

}


/// A logger definition version contains a list of loggers.
///
/// In an AWS CloudFormation template, LoggerDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::LoggerDefinition resource.
#[derive(Default, serde::Serialize)]
pub struct LoggerDefinitionVersion {


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


/// A logger represents logging settings for the AWS IoT Greengrass group, which can be stored in CloudWatch and the local file system of your core device. All log   entries include a timestamp, log level, and information about the event. For more information, see Monitoring with AWS IoT Greengrass Logs in the    AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Loggers 		 property of the LoggerDefinitionVersion property type contains a list       of Logger property types.
#[derive(Default, serde::Serialize)]
pub struct Logger {


    /// 
    /// The amount of file space (in KB) to use when writing logs to the local file system. 				 This property does not apply for CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Space")]
    pub space: Option<i64>,


    /// 
    /// The log-level threshold. Log events below this threshold are filtered out and aren't stored. 				 Valid values are DEBUG, INFO (recommended), WARN, 				 ERROR, or FATAL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Level")]
    pub level: String,


    /// 
    /// A descriptive or arbitrary ID for the logger. This value must be unique within       the logger definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The storage mechanism for log events. Valid values are FileSystem or AWSCloudWatch. 				 When AWSCloudWatch is used, log events are sent to CloudWatch Logs. 				 When FileSystem is used, log events are stored on the local file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The source of the log event. Valid values are GreengrassSystem or Lambda. 				 When GreengrassSystem is used, events from Greengrass system components are logged. 				 When Lambda is used, events from user-defined Lambda functions are logged.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Component")]
    pub component: String,

}