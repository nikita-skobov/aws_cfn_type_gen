
pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// No documentation provided by AWS
    #[serde(rename = "PluginsS3ObjectVersion")]
    pub plugins_s3_object_version: Option<S3ObjectVersion>,
    /// Logging configuration for the environment.
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// Scheduler compute units.
    #[serde(rename = "Schedulers")]
    pub schedulers: Option<Schedulers>,
    /// No documentation provided by AWS
    #[serde(rename = "PluginsS3Path")]
    pub plugins_s3_path: Option<RelativePath>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceBucketArn")]
    pub source_bucket_arn: Option<S3BucketArn>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: EnvironmentName,
    /// Choice for mode of webserver access including over public internet or via private VPC endpoint.
    #[serde(rename = "WebserverAccessMode")]
    pub webserver_access_mode: Option<WebserverAccessMode>,
    /// Version of airflow to deploy to the environment.
    #[serde(rename = "AirflowVersion")]
    pub airflow_version: Option<AirflowVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "StartupScriptS3Path")]
    pub startup_script_s3_path: Option<RelativePath>,
    /// IAM role to be used by tasks.
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<ExecutionRoleArn>,
    /// Minimum worker compute units.
    #[serde(rename = "MinWorkers")]
    pub min_workers: Option<MinWorkers>,
    /// Configures the network resources of the environment.
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// A map of tags for the environment.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "RequirementsS3Path")]
    pub requirements_s3_path: Option<RelativePath>,
    /// Templated configuration for airflow processes and backing infrastructure.
    #[serde(rename = "EnvironmentClass")]
    pub environment_class: Option<EnvironmentClass>,
    /// No documentation provided by AWS
    #[serde(rename = "StartupScriptS3ObjectVersion")]
    pub startup_script_s3_object_version: Option<S3ObjectVersion>,
    /// Key/value pairs representing Airflow configuration variables.
    /// Keys are prefixed by their section:
    /// 
    /// [core]
    /// dags_folder={AIRFLOW_HOME}/dags
    /// 
    /// Would be represented as
    /// 
    /// "core.dags_folder": "{AIRFLOW_HOME}/dags"
    #[serde(rename = "AirflowConfigurationOptions")]
    pub airflow_configuration_options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DagS3Path")]
    pub dag_s3_path: Option<RelativePath>,
    /// Start time for the weekly maintenance window.
    #[serde(rename = "WeeklyMaintenanceWindowStart")]
    pub weekly_maintenance_window_start: Option<WeeklyMaintenanceWindowStart>,
    /// The identifier of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use for MWAA data encryption.
    /// 
    /// You can specify the CMK using any of the following:
    /// 
    /// Key ID. For example, key/1234abcd-12ab-34cd-56ef-1234567890ab.
    /// 
    /// Key alias. For example, alias/ExampleAlias.
    /// 
    /// Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/abcd1234-a123-456a-a12b-a123b4cd56ef.
    /// 
    /// Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.
    /// 
    /// AWS authenticates the CMK asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<KmsKey>,
    /// Maximum worker compute units.
    #[serde(rename = "MaxWorkers")]
    pub max_workers: Option<MaxWorkers>,
    /// No documentation provided by AWS
    #[serde(rename = "RequirementsS3ObjectVersion")]
    pub requirements_s3_object_version: Option<S3ObjectVersion>,

}

pub type UpdateStatus = String;pub type KmsKey = String;
#[derive(serde::Serialize, Default)]
pub struct LastUpdate {
    #[serde(rename = "Error")]
    pub error: Option<UpdateError>,
    #[serde(rename = "Status")]
    pub status: Option<UpdateStatus>,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<UpdateCreatedAt>,

}
pub type AirflowArn = String;pub type EnvironmentName = String;pub type AirflowVersion = String;pub type EnvironmentClass = String;pub type MinWorkers = usize;pub type EnvironmentArn = String;pub type CreatedAt = String;pub type SecurityGroupId = String;pub type ConfigValue = String;pub type Schedulers = usize;
#[derive(serde::Serialize, Default)]
pub struct LoggingConfigurationInput {
    #[serde(rename = "DagProcessingLogs")]
    pub dag_processing_logs: Option<ModuleLoggingConfigurationInput>,
    #[serde(rename = "WorkerLogs")]
    pub worker_logs: Option<ModuleLoggingConfigurationInput>,
    #[serde(rename = "SchedulerLogs")]
    pub scheduler_logs: Option<ModuleLoggingConfigurationInput>,
    #[serde(rename = "WebserverLogs")]
    pub webserver_logs: Option<ModuleLoggingConfigurationInput>,
    #[serde(rename = "TaskLogs")]
    pub task_logs: Option<ModuleLoggingConfigurationInput>,

}

#[derive(serde::Serialize, Default)]
pub struct ModuleLoggingConfigurationInput {
    #[serde(rename = "Enabled")]
    pub enabled: Option<LoggingEnabled>,
    #[serde(rename = "LogLevel")]
    pub log_level: Option<LoggingLevel>,

}
pub type ErrorCode = String;pub type WebserverAccessMode = String;pub type S3BucketArn = String;pub type ConfigKey = String;pub type S3ObjectVersion = String;pub type LoggingLevel = String;pub type ErrorMessage = String;
#[derive(serde::Serialize, Default)]
pub struct UpdateError {
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,
    #[serde(rename = "ErrorMessage")]
    pub error_message: Option<ErrorMessage>,

}

#[derive(serde::Serialize, Default)]
pub struct ModuleLoggingConfiguration {
    #[serde(rename = "CloudWatchLogGroupArn")]
    pub cloud_watch_log_group_arn: Option<CloudWatchLogGroupArn>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<LoggingEnabled>,
    #[serde(rename = "LogLevel")]
    pub log_level: Option<LoggingLevel>,

}
pub type WeeklyMaintenanceWindowStart = String;pub type LoggingEnabled = bool;
#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<SubnetId>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<SecurityGroupId>>,

}
pub type WebserverUrl = String;pub type SubnetId = String;pub type MaxWorkers = usize;pub type UpdateCreatedAt = String;pub type ServiceRoleArn = String;pub type ExecutionRoleArn = String;pub type CloudWatchLogGroupArn = String;pub type RelativePath = String;
#[derive(serde::Serialize, Default)]
pub struct LoggingConfiguration {
    #[serde(rename = "DagProcessingLogs")]
    pub dag_processing_logs: Option<ModuleLoggingConfiguration>,
    #[serde(rename = "WebserverLogs")]
    pub webserver_logs: Option<ModuleLoggingConfiguration>,
    #[serde(rename = "SchedulerLogs")]
    pub scheduler_logs: Option<ModuleLoggingConfiguration>,
    #[serde(rename = "WorkerLogs")]
    pub worker_logs: Option<ModuleLoggingConfiguration>,
    #[serde(rename = "TaskLogs")]
    pub task_logs: Option<ModuleLoggingConfiguration>,

}
pub type EnvironmentStatus = String;

}
