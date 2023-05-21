/// Creates a continuous deployment for a target, which is a AWS IoT Greengrass core device    or group of core devices. When you add a new core device to a group of core devices that has a    deployment, AWS IoT Greengrass deploys that group's deployment to the new device.
///
/// You can define one deployment for each target. When you create a new deployment for a    target that has an existing deployment, you replace the previous deployment. AWS IoT Greengrass applies the new deployment to the target devices.
///
/// You can only add, update, or delete up to 10 deployments at a time to a single    target.
///
/// Every deployment has a revision number that indicates how many deployment revisions you    define for a target. Use this operation to create a new revision of an existing deployment.    This operation returns the revision number of the new deployment when you create it.
///
/// For more information, see the Create deployments    in the AWS IoT Greengrass V2 Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeployment {
    ///
    /// The components to deploy. This is a dictionary, where each key is the name of a component,    and each key's value is the version and configuration to deploy for that component.
    ///
    /// Required: No
    ///
    /// Type: Map of ComponentDeploymentSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "Components")]
    pub components: Option<std::collections::HashMap<String, ComponentDeploymentSpecification>>,

    ///
    /// The name of the deployment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentName")]
    pub deployment_name: Option<String>,

    ///
    /// The deployment policies for the deployment. These policies define how the deployment    updates components and handles failure.
    ///
    /// Required: No
    ///
    /// Type: DeploymentPolicies
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentPolicies")]
    pub deployment_policies: Option<DeploymentPolicies>,

    ///
    /// The job configuration for the deployment configuration. The job configuration specifies    the rollout, timeout, and stop configurations for the deployment configuration.
    ///
    /// Required: No
    ///
    /// Type: DeploymentIoTJobConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "IotJobConfiguration")]
    pub iot_job_configuration: Option<DeploymentIoTJobConfiguration>,

    ///
    /// The parent deployment's ARN for a subdeployment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentTargetArn")]
    pub parent_target_arn: Option<String>,

    ///
    /// Application-specific metadata to attach to the deployment. You can use tags in IAM policies to control access to AWS IoT Greengrass resources. You can also use    tags to categorize your resources. For more information, see Tag your AWS IoT Greengrass Version 2     resources in the AWS IoT Greengrass V2 Developer Guide.
    ///
    /// This Json property type is processed as a map of key-value pairs. It uses the    following format, which is different from most Tags implementations in AWS CloudFormation templates.
    ///
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// The ARN of the target AWS IoT thing or thing group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
}

impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string(&self) -> &'static str {
        "AWS::GreengrassV2::Deployment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.deployment_policies
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_job_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about a deployment's update to a component's configuration on AWS IoT Greengrass core devices. For more information, see Update component     configurations in the AWS IoT Greengrass V2 Developer    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentConfigurationUpdate {
    ///
    /// A serialized JSON string that contains the configuration object to merge to target    devices. The core device merges this configuration with the component's existing    configuration. If this is the first time a component deploys on a device, the core device    merges this configuration with the component's default configuration. This means that the core    device keeps it's existing configuration for keys and values that you don't specify in this    object. For more information, see Merge configuration updates in the AWS IoT Greengrass V2 Developer     Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Merge")]
    pub merge: Option<String>,

    ///
    /// The list of configuration nodes to reset to default values on target devices. Use JSON    pointers to specify each node to reset. JSON pointers start with a forward slash     (/) and use forward slashes to separate the key for each level in the object.    For more information, see the JSON pointer     specification and Reset configuration updates in the AWS IoT Greengrass V2 Developer     Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Reset")]
    pub reset: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ComponentConfigurationUpdate {
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

/// Contains information about a component to deploy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentDeploymentSpecification {
    ///
    /// The version of the component.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentVersion")]
    pub component_version: Option<String>,

    ///
    /// The configuration updates to deploy for the component. You can define reset updates and    merge updates. A reset updates the keys that you specify to the default configuration for the    component. A merge updates the core device's component configuration with the keys and values    that you specify. The AWS IoT Greengrass Core software applies reset updates before it    applies merge updates. For more information, see Update component     configuration.
    ///
    /// Required: No
    ///
    /// Type: ComponentConfigurationUpdate
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationUpdate")]
    pub configuration_update: Option<ComponentConfigurationUpdate>,

    ///
    /// The system user and group that the software uses to run component    processes on the core device. If you omit this parameter, the software    uses the system user and group that you configure for the core device. For more information,    see Configure the user and group that run components in the AWS IoT Greengrass V2 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ComponentRunWith
    ///
    /// Update requires: Replacement
    #[serde(rename = "RunWith")]
    pub run_with: Option<ComponentRunWith>,
}

impl cfn_resources::CfnResource for ComponentDeploymentSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.configuration_update
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.run_with
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information system user and group that the AWS IoT Greengrass Core software uses    to run component processes on the core device. For more information, see Configure the user and group that run components in the AWS IoT Greengrass V2 Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentRunWith {
    ///
    /// The POSIX system user and (optional) group to use to run this component. Specify the user    and group separated by a colon (:) in the following format:     user:group. The group is optional. If you don't specify a group, the AWS IoT Greengrass Core software uses the primary user for the group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PosixUser")]
    pub posix_user: Option<String>,

    ///
    /// The system resource limits to apply to this component's process on the core device.     AWS IoT Greengrass supports this feature only on Linux core devices.
    ///
    /// If you omit this parameter, the AWS IoT Greengrass Core software uses the default system    resource limits that you configure on the AWS IoT Greengrass nucleus component. For more    information, see Configure system resource limits for components .
    ///
    /// Required: No
    ///
    /// Type: SystemResourceLimits
    ///
    /// Update requires: Replacement
    #[serde(rename = "SystemResourceLimits")]
    pub system_resource_limits: Option<SystemResourceLimits>,

    ///
    /// The Windows user to use to run this component on Windows core devices. The user must exist    on each Windows core device, and its name and password must be in the LocalSystem account's    Credentials Manager instance.
    ///
    /// If you omit this parameter, the AWS IoT Greengrass Core software uses the default Windows    user that you configure on the AWS IoT Greengrass nucleus component. For more information,    see Configure the user and group that run components.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WindowsUser")]
    pub windows_user: Option<String>,
}

impl cfn_resources::CfnResource for ComponentRunWith {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.system_resource_limits
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about a deployment's policy that defines when components are safe to    update.
///
/// Each component on a device can report whether or not it's ready to update. After a    component and its dependencies are ready, they can apply the update in the deployment. You can    configure whether or not the deployment notifies components of an update and waits for a    response. You specify the amount of time each component has to respond to the update    notification.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentComponentUpdatePolicy {
    ///
    /// Whether or not to notify components and wait for components to become safe to update.    Choose from the following options:
    ///
    /// NOTIFY_COMPONENTS – The deployment notifies each component before it      stops and updates that component. Components can use the SubscribeToComponentUpdates IPC operation to receive these notifications. Then,      components can respond with the DeferComponentUpdate IPC operation. For more information, see the Create deployments in the AWS IoT Greengrass V2 Developer       Guide.        SKIP_NOTIFY_COMPONENTS – The deployment doesn't notify components or wait      for them to be safe to update.
    ///
    /// Default: NOTIFY_COMPONENTS
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: Option<String>,

    ///
    /// The amount of time in seconds that each component on a device has to report that it's safe    to update. If the component waits for longer than this timeout, then the deployment proceeds    on the device.
    ///
    /// Default: 60
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for DeploymentComponentUpdatePolicy {
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

/// Contains information about how long a component on a core device can validate its    configuration updates before it times out. Components can use the SubscribeToValidateConfigurationUpdates IPC operation to receive notifications when    a deployment specifies a configuration update. Then, components can respond with the SendConfigurationValidityReport IPC operation. For more information, see the Create deployments in the AWS IoT Greengrass V2 Developer    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentConfigurationValidationPolicy {
    ///
    /// The amount of time in seconds that a component can validate its configuration updates. If    the validation time exceeds this timeout, then the deployment proceeds for the device.
    ///
    /// Default: 30
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for DeploymentConfigurationValidationPolicy {
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

/// Contains information about an AWS IoT job configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentIoTJobConfiguration {
    ///
    /// The stop configuration for the job. This configuration defines when and how to stop a job    rollout.
    ///
    /// Required: No
    ///
    /// Type: IoTJobAbortConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AbortConfig")]
    pub abort_config: Option<IoTJobAbortConfig>,

    ///
    /// The rollout configuration for the job. This configuration defines the rate at which the    job rolls out to the fleet of target devices.
    ///
    /// Required: No
    ///
    /// Type: IoTJobExecutionsRolloutConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobExecutionsRolloutConfig")]
    pub job_executions_rollout_config: Option<IoTJobExecutionsRolloutConfig>,

    ///
    /// The timeout configuration for the job. This configuration defines the amount of time each    device has to complete the job.
    ///
    /// Required: No
    ///
    /// Type: IoTJobTimeoutConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutConfig")]
    pub timeout_config: Option<IoTJobTimeoutConfig>,
}

impl cfn_resources::CfnResource for DeploymentIoTJobConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.abort_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.job_executions_rollout_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timeout_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about policies that define how a deployment updates components and    handles failure.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentPolicies {
    ///
    /// The component update policy for the configuration deployment. This policy defines when    it's safe to deploy the configuration to devices.
    ///
    /// Required: No
    ///
    /// Type: DeploymentComponentUpdatePolicy
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentUpdatePolicy")]
    pub component_update_policy: Option<DeploymentComponentUpdatePolicy>,

    ///
    /// The configuration validation policy for the configuration deployment. This policy defines    how long each component has to validate its configure updates.
    ///
    /// Required: No
    ///
    /// Type: DeploymentConfigurationValidationPolicy
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationValidationPolicy")]
    pub configuration_validation_policy: Option<DeploymentConfigurationValidationPolicy>,

    ///
    /// The failure handling policy for the configuration deployment. This policy defines what to    do if the deployment fails.
    ///
    /// Default: ROLLBACK
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FailureHandlingPolicy")]
    pub failure_handling_policy: Option<String>,
}

impl cfn_resources::CfnResource for DeploymentPolicies {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.component_update_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.configuration_validation_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains a list of criteria that define when and how to cancel a configuration    deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobAbortConfig {
    ///
    /// The list of criteria that define when and how to cancel the configuration    deployment.
    ///
    /// Required: Yes
    ///
    /// Type: List of IoTJobAbortCriteria
    ///
    /// Update requires: Replacement
    #[serde(rename = "CriteriaList")]
    pub criteria_list: Vec<IoTJobAbortCriteria>,
}

impl cfn_resources::CfnResource for IoTJobAbortConfig {
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

/// Contains criteria that define when and how to cancel a job.
///
/// The deployment stops if the following conditions are true:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobAbortCriteria {
    ///
    /// The action to perform when the criteria are met.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Action")]
    pub action: String,

    ///
    /// The type of job deployment failure that can cancel a job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FailureType")]
    pub failure_type: String,

    ///
    /// The minimum number of things that receive the configuration before the job can    cancel.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MinNumberOfExecutedThings")]
    pub min_number_of_executed_things: i64,

    ///
    /// The minimum percentage of failureType failures that occur before the job can    cancel.
    ///
    /// This parameter supports up to two digits after the decimal (for example, you can specify     10.9 or 10.99, but not 10.999).
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: f64,
}

impl cfn_resources::CfnResource for IoTJobAbortCriteria {
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

/// Contains information about the rollout configuration for a job. This configuration defines    the rate at which the job deploys a configuration to a fleet of target devices.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobExecutionsRolloutConfig {
    ///
    /// The exponential rate to increase the job rollout rate.
    ///
    /// Required: No
    ///
    /// Type: IoTJobExponentialRolloutRate
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExponentialRate")]
    pub exponential_rate: Option<IoTJobExponentialRolloutRate>,

    ///
    /// The maximum number of devices that receive a pending job notification, per minute.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaximumPerMinute")]
    pub maximum_per_minute: Option<i64>,
}

impl cfn_resources::CfnResource for IoTJobExecutionsRolloutConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.exponential_rate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about an exponential rollout rate for a configuration deployment    job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobExponentialRolloutRate {
    ///
    /// The minimum number of devices that receive a pending job notification, per minute, when    the job starts. This parameter defines the initial rollout rate of the job.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaseRatePerMinute")]
    pub base_rate_per_minute: i64,

    ///
    /// The exponential factor to increase the rollout rate for the job.
    ///
    /// This parameter supports up to one digit after the decimal (for example, you can specify     1.5, but not 1.55).
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "IncrementFactor")]
    pub increment_factor: f64,

    ///
    /// The criteria to increase the rollout rate for the job.
    ///
    /// Required: Yes
    ///
    /// Type: IoTJobRateIncreaseCriteria
    ///
    /// Update requires: Replacement
    #[serde(rename = "RateIncreaseCriteria")]
    pub rate_increase_criteria: IoTJobRateIncreaseCriteria,
}

impl cfn_resources::CfnResource for IoTJobExponentialRolloutRate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.rate_increase_criteria.validate()?;

        Ok(())
    }
}

/// Contains information about criteria to meet before a job increases its rollout rate.    Specify either numberOfNotifiedThings or    numberOfSucceededThings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobRateIncreaseCriteria {
    ///
    /// The number of devices to receive the job notification before the rollout rate    increases.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfNotifiedThings")]
    pub number_of_notified_things: Option<i64>,

    ///
    /// The number of devices to successfully run the configuration job before the rollout rate    increases.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfSucceededThings")]
    pub number_of_succeeded_things: Option<i64>,
}

impl cfn_resources::CfnResource for IoTJobRateIncreaseCriteria {
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

/// Contains information about the timeout configuration for a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IoTJobTimeoutConfig {
    ///
    /// The amount of time, in minutes, that devices have to complete the job. The timer starts    when the job status is set to IN_PROGRESS. If the job status doesn't change to a    terminal state before the time expires, then the job status is set to    TIMED_OUT.
    ///
    /// The timeout interval must be between 1 minute and 7 days (10080 minutes).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "InProgressTimeoutInMinutes")]
    pub in_progress_timeout_in_minutes: Option<i64>,
}

impl cfn_resources::CfnResource for IoTJobTimeoutConfig {
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

/// Contains information about system resource limits that the software    applies to a component's processes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SystemResourceLimits {
    ///
    /// The maximum amount of CPU time that a component's processes can use on the core device. A    core device's total CPU time is equivalent to the device's number of CPU cores. For example,    on a core device with 4 CPU cores, you can set this value to 2 to limit the component's    processes to 50 percent usage of each CPU core. On a device with 1 CPU core, you can set this    value to 0.25 to limit the component's processes to 25 percent usage of the CPU. If you set    this value to a number greater than the number of CPU cores, the AWS IoT Greengrass Core    software doesn't limit the component's CPU usage.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cpus")]
    pub cpus: Option<f64>,

    ///
    /// The maximum amount of RAM, expressed in kilobytes, that a component's processes can use on    the core device. For more information, see Configure system resource limits for components.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,
}

impl cfn_resources::CfnResource for SystemResourceLimits {
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
