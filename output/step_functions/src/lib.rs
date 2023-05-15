
pub mod cfn_activity {

#[derive(serde::Serialize, Default)]
pub struct CfnActivity {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,

}


#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_state_machine {

#[derive(serde::Serialize, Default)]
pub struct CfnStateMachine {
    /// No documentation provided by AWS
    #[serde(rename = "DefinitionString")]
    pub definition_string: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefinitionS3Location")]
    pub definition_s3_location: Option<S3Location>,
    /// No documentation provided by AWS
    #[serde(rename = "DefinitionSubstitutions")]
    pub definition_substitutions: Option<DefinitionSubstitutions>,
    /// No documentation provided by AWS
    #[serde(rename = "TracingConfiguration")]
    pub tracing_configuration: Option<TracingConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "StateMachineType")]
    pub state_machine_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Option<Definition>,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "StateMachineName")]
    pub state_machine_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct Definition {

}

#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LogDestination {
    #[serde(rename = "CloudWatchLogsLogGroup")]
    pub cloud_watch_logs_log_group: Option<CloudWatchLogsLogGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingConfiguration {
    #[serde(rename = "IncludeExecutionData")]
    pub include_execution_data: Option<bool>,
    #[serde(rename = "Level")]
    pub level: Option<String>,
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<LogDestination>>,

}

#[derive(serde::Serialize, Default)]
pub struct TracingConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct DefinitionSubstitutions {

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogsLogGroup {
    #[serde(rename = "LogGroupArn")]
    pub log_group_arn: Option<String>,

}


}
