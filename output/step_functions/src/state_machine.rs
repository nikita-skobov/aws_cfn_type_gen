

/// Provisions a state machine. A state machine consists of a collection of states that can     do work (Task states), determine to which states to transition next       (Choice states), stop an execution with an error (Fail     states), and so on. State machines are specified using a JSON-based, structured     language.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStateMachine {


    /// 
    /// The Amazon States Language definition of the state machine. The state machine definition must be in JSON or YAML, and the format of the object must     match the format of your AWS Step Functions template file. See Amazon States Language.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<serde_json::Value>,


    /// 
    /// Defines what execution history events are logged and where they are logged.
    /// 
    /// NoteBy default, the level is set to OFF. For more information       see Log Levels in the AWS Step Functions User Guide.
    /// 
    /// Required: No
    ///
    /// Type: LoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,


    /// 
    /// A map (string to string) that specifies the mappings for placeholder variables in the     state machine definition. This enables the customer to inject values obtained at runtime,     for example from intrinsic functions, in the state machine definition. Variables can be     template parameter names, resource logical IDs, resource attributes, or a variable in a     key-value map.
    /// 
    /// Required: No
    ///
    /// Type: Map of Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionSubstitutions")]
    pub definition_substitutions: Option<std::collections::HashMap<String, serde_json::Value>>,


    /// 
    /// The name of the S3 bucket where the state machine definition is stored. The state machine definition must be a JSON or YAML file.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionS3Location")]
    pub definition_s3_location: Option<S3Location>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to use for this state machine.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Determines whether a STANDARD or EXPRESS state machine is     created. The default is STANDARD. You cannot update the type of a     state machine once it has been created. For more information on STANDARD and       EXPRESS workflows, see Standard Versus Express        Workflows in the AWS Step Functions Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StateMachineType")]
    pub state_machine_type: Option<String>,


    /// 
    /// The Amazon States Language definition of the state machine. The state machine definition must be in JSON. See Amazon States Language.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefinitionString")]
    pub definition_string: Option<String>,


    /// 
    /// The name of the state machine.
    /// 
    /// A name must not contain:
    /// 
    /// white space            brackets < > { } [ ]                  wildcard characters ? *                  special characters " # % \ ^ | ~ ` $ & , ; : /                  control characters (U+0000-001F, U+007F-009F)
    /// 
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this       resource. You can perform updates that require no or some interruption. If you must       replace the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StateMachineName")]
    pub state_machine_name: Option<String>,


    /// 
    /// Selects whether or not the state machine's AWS X-Ray tracing is enabled.
    /// 
    /// Required: No
    ///
    /// Type: TracingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TracingConfiguration")]
    pub tracing_configuration: Option<TracingConfiguration>,


    /// 
    /// The list of tags to add to a resource.
    /// 
    /// Tags may only contain Unicode letters, digits, white space, or these symbols: _ . : / = + - @.
    /// 
    /// Required: No
    ///
    /// Type: List of TagsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,

}

impl cfn_resources::CfnResource for CfnStateMachine {
    fn type_string() -> &'static str {
        "AWS::StepFunctions::StateMachine"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Defines the S3 bucket location where a state machine definition is stored. The state machine definition must be a JSON or YAML file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// For versioning-enabled buckets, a specific version of the state machine     definition.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The name of the state machine definition file (Amazon S3 object name).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The name of the S3 bucket where the state machine definition JSON or YAML file is stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

}


/// Selects whether or not the state machine's AWS X-Ray tracing is enabled. To configure     your state machine to send trace data to X-Ray, set Enabled to       true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TracingConfiguration {


    /// 
    /// When set to true, X-Ray tracing is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


/// Defines a destination for LoggingConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogDestination {


    /// 
    /// An object describing a CloudWatch log group. For more information, see AWS::Logs::LogGroup in the AWS CloudFormation User Guide.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLogsLogGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroup")]
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,

}


/// The TagsEntry property specifies tags to identify a     state machine.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagsEntry {


    /// 
    /// The value for a key-value pair in a tag entry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key for a key-value pair in a tag entry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}


/// Defines what execution history events are logged and where they are logged.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfiguration {


    /// 
    /// Defines which category of execution history events are logged.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Level")]
    pub level: Option<String>,


    /// 
    /// Determines whether execution data is included in your log. When set to false,    data is excluded.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeExecutionData")]
    pub include_execution_data: Option<bool>,


    /// 
    /// An array of objects that describes where your execution history events will be logged.    Limited to size 1. Required, if your log level is not set to OFF.
    /// 
    /// Required: No
    ///
    /// Type: List of LogDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<LogDestination>>,

}


/// Defines a CloudWatch log group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLogsLogGroup {


    /// 
    /// The ARN of the the CloudWatch log group to which you want your logs emitted to. The ARN    must end with :*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: Option<String>,

}
