
pub mod cfn_component_version {

#[derive(serde::Serialize, Default)]
pub struct CfnComponentVersion {
    /// No documentation provided by AWS
    #[serde(rename = "InlineRecipe")]
    pub inline_recipe: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "LambdaFunction")]
    pub lambda_function: Option<LambdaFunctionRecipeSource>,

}


#[derive(serde::Serialize, Default)]
pub struct LambdaLinuxProcessParams {
    #[serde(rename = "ContainerParams")]
    pub container_params: Option<LambdaContainerParams>,
    #[serde(rename = "IsolationMode")]
    pub isolation_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaEventSource {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Topic")]
    pub topic: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentDependencyRequirement {
    #[serde(rename = "DependencyType")]
    pub dependency_type: Option<String>,
    #[serde(rename = "VersionRequirement")]
    pub version_requirement: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentPlatform {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaFunctionRecipeSource {
    #[serde(rename = "ComponentPlatforms")]
    pub component_platforms: Option<Vec<ComponentPlatform>>,
    #[serde(rename = "ComponentDependencies")]
    pub component_dependencies: Option<()>,
    #[serde(rename = "ComponentLambdaParameters")]
    pub component_lambda_parameters: Option<LambdaExecutionParameters>,
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: Option<String>,
    #[serde(rename = "ComponentName")]
    pub component_name: Option<String>,
    #[serde(rename = "ComponentVersion")]
    pub component_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaDeviceMount {
    #[serde(rename = "Path")]
    pub path: Option<FilesystemPath>,
    #[serde(rename = "Permission")]
    pub permission: Option<LambdaFilesystemPermission>,
    #[serde(rename = "AddGroupOwner")]
    pub add_group_owner: Option<LambdaAddGroupOwnerBoolean>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaVolumeMount {
    #[serde(rename = "DestinationPath")]
    pub destination_path: Option<FilesystemPath>,
    #[serde(rename = "Permission")]
    pub permission: Option<LambdaFilesystemPermission>,
    #[serde(rename = "AddGroupOwner")]
    pub add_group_owner: Option<LambdaAddGroupOwnerBoolean>,
    #[serde(rename = "SourcePath")]
    pub source_path: Option<FilesystemPath>,

}
pub type FilesystemPath = String;pub type LambdaFilesystemPermission = String;pub type LambdaAddGroupOwnerBoolean = bool;
#[derive(serde::Serialize, Default)]
pub struct LambdaExecutionParameters {
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<usize>,
    #[serde(rename = "EventSources")]
    pub event_sources: Option<Vec<LambdaEventSource>>,
    #[serde(rename = "StatusTimeoutInSeconds")]
    pub status_timeout_in_seconds: Option<usize>,
    #[serde(rename = "LinuxProcessParams")]
    pub linux_process_params: Option<LambdaLinuxProcessParams>,
    #[serde(rename = "MaxIdleTimeInSeconds")]
    pub max_idle_time_in_seconds: Option<usize>,
    #[serde(rename = "MaxInstancesCount")]
    pub max_instances_count: Option<usize>,
    #[serde(rename = "InputPayloadEncodingType")]
    pub input_payload_encoding_type: Option<String>,
    #[serde(rename = "ExecArgs")]
    pub exec_args: Option<Vec<String>>,
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<()>,
    #[serde(rename = "Pinned")]
    pub pinned: Option<bool>,
    #[serde(rename = "MaxQueueSize")]
    pub max_queue_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaContainerParams {
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<LambdaVolumeMount>>,
    #[serde(rename = "MemorySizeInKB")]
    pub memory_size_in_kb: Option<usize>,
    #[serde(rename = "MountROSysfs")]
    pub mount_rosysfs: Option<bool>,
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<LambdaDeviceMount>>,

}


}

pub mod cfn_deployment {

#[derive(serde::Serialize, Default)]
pub struct CfnDeployment {
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentName")]
    pub deployment_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "IotJobConfiguration")]
    pub iot_job_configuration: Option<DeploymentIoTJobConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentPolicies")]
    pub deployment_policies: Option<DeploymentPolicies>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Components")]
    pub components: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ParentTargetArn")]
    pub parent_target_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct IoTJobRateIncreaseCriteria {

}

#[derive(serde::Serialize, Default)]
pub struct IoTJobExecutionsRolloutConfig {
    #[serde(rename = "MaximumPerMinute")]
    pub maximum_per_minute: Option<usize>,
    #[serde(rename = "ExponentialRate")]
    pub exponential_rate: Option<IoTJobExponentialRolloutRate>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentPolicies {
    #[serde(rename = "FailureHandlingPolicy")]
    pub failure_handling_policy: Option<String>,
    #[serde(rename = "ComponentUpdatePolicy")]
    pub component_update_policy: Option<DeploymentComponentUpdatePolicy>,
    #[serde(rename = "ConfigurationValidationPolicy")]
    pub configuration_validation_policy: Option<DeploymentConfigurationValidationPolicy>,

}

#[derive(serde::Serialize, Default)]
pub struct IoTJobAbortCriteria {
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: f64,
    #[serde(rename = "MinNumberOfExecutedThings")]
    pub min_number_of_executed_things: usize,
    #[serde(rename = "FailureType")]
    pub failure_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentComponentUpdatePolicy {
    #[serde(rename = "Action")]
    pub action: Option<String>,
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct IoTJobExponentialRolloutRate {
    #[serde(rename = "IncrementFactor")]
    pub increment_factor: f64,
    #[serde(rename = "RateIncreaseCriteria")]
    pub rate_increase_criteria: IoTJobRateIncreaseCriteria,
    #[serde(rename = "BaseRatePerMinute")]
    pub base_rate_per_minute: usize,

}

#[derive(serde::Serialize, Default)]
pub struct IoTJobAbortConfig {
    #[serde(rename = "CriteriaList")]
    pub criteria_list: Vec<IoTJobAbortCriteria>,

}

#[derive(serde::Serialize, Default)]
pub struct SystemResourceLimits {
    #[serde(rename = "Cpus")]
    pub cpus: Option<f64>,
    #[serde(rename = "Memory")]
    pub memory: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentIoTJobConfiguration {
    #[serde(rename = "JobExecutionsRolloutConfig")]
    pub job_executions_rollout_config: Option<IoTJobExecutionsRolloutConfig>,
    #[serde(rename = "TimeoutConfig")]
    pub timeout_config: Option<IoTJobTimeoutConfig>,
    #[serde(rename = "AbortConfig")]
    pub abort_config: Option<IoTJobAbortConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentRunWith {
    #[serde(rename = "WindowsUser")]
    pub windows_user: Option<String>,
    #[serde(rename = "SystemResourceLimits")]
    pub system_resource_limits: Option<SystemResourceLimits>,
    #[serde(rename = "PosixUser")]
    pub posix_user: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct IoTJobTimeoutConfig {
    #[serde(rename = "InProgressTimeoutInMinutes")]
    pub in_progress_timeout_in_minutes: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentDeploymentSpecification {
    #[serde(rename = "ConfigurationUpdate")]
    pub configuration_update: Option<ComponentConfigurationUpdate>,
    #[serde(rename = "RunWith")]
    pub run_with: Option<ComponentRunWith>,
    #[serde(rename = "ComponentVersion")]
    pub component_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentConfigurationUpdate {
    #[serde(rename = "Reset")]
    pub reset: Option<Vec<String>>,
    #[serde(rename = "Merge")]
    pub merge: Option<String>,

}
pub type NumberOfThings = usize;
#[derive(serde::Serialize, Default)]
pub struct DeploymentConfigurationValidationPolicy {
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<usize>,

}


}
