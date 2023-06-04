/// Creates a component. Components are software that run on AWS IoT Greengrass core devices.    After you develop and test a component on your core device, you can use this operation to    upload your component to AWS IoT Greengrass. Then, you can deploy the component to other    core devices.
///
/// You can use this operation to do the following:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnComponentVersion {
    ///
    /// The recipe to use to create the component. The recipe defines the component's metadata,    parameters, dependencies, lifecycle, artifacts, and platform compatibility.
    ///
    /// You must specify either InlineRecipe or LambdaFunction.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InlineRecipe")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub inline_recipe: Option<cfn_resources::StrVal>,

    ///
    /// The parameters to create a component from a Lambda function.
    ///
    /// You must specify either InlineRecipe or LambdaFunction.
    ///
    /// Required: No
    ///
    /// Type: LambdaFunctionRecipeSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaFunction")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_function: Option<LambdaFunctionRecipeSource>,

    ///
    /// Application-specific metadata to attach to the component version. You can use tags in     IAM policies to control access to AWS IoT Greengrass resources. You can    also use tags to categorize your resources. For more information, see Tag your AWS IoT Greengrass Version 2 resources in the AWS IoT Greengrass V2 Developer     Guide.
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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnComponentVersionarn,

    #[serde(skip_serializing)]
    pub att_component_name: CfnComponentVersioncomponentname,

    #[serde(skip_serializing)]
    pub att_component_version: CfnComponentVersioncomponentversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnComponentVersionarn;
impl CfnComponentVersionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnComponentVersioncomponentname;
impl CfnComponentVersioncomponentname {
    pub fn att_name(&self) -> &'static str {
        r#"ComponentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnComponentVersioncomponentversion;
impl CfnComponentVersioncomponentversion {
    pub fn att_name(&self) -> &'static str {
        r#"ComponentVersion"#
    }
}

impl cfn_resources::CfnResource for CfnComponentVersion {
    fn type_string(&self) -> &'static str {
        "AWS::GreengrassV2::ComponentVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_function
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about a component dependency for a Lambda function    component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComponentDependencyRequirement {
    ///
    /// The type of this dependency. Choose from the following options:
    ///
    /// SOFT – The component doesn't restart if the dependency changes      state.        HARD – The component restarts if the dependency changes state.
    ///
    /// Default: HARD
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DependencyType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub dependency_type: Option<cfn_resources::StrVal>,

    ///
    /// The component version requirement for the component dependency.
    ///
    /// AWS IoT Greengrass uses semantic version constraints. For more information, see     Semantic Versioning.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VersionRequirement")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub version_requirement: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ComponentDependencyRequirement {
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

/// Contains information about a platform that a component supports.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComponentPlatform {
    ///
    /// A dictionary of attributes for the platform. The AWS IoT Greengrass Core software defines    the os and platform by default. You can specify additional platform    attributes for a core device when you deploy the AWS IoT Greengrass nucleus component. For    more information, see the AWS IoT Greengrass     nucleus component in the AWS IoT Greengrass V2 Developer    Guide.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub attributes: Option<std::collections::HashMap<String, String>>,

    ///
    /// The friendly name of the platform. This name helps you identify the platform.
    ///
    /// If you omit this parameter, AWS IoT Greengrass creates a friendly name from the     os and architecture of the platform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ComponentPlatform {
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

/// Contains information about a container in which AWS Lambda functions run on     AWS IoT Greengrass core devices.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaContainerParams {
    ///
    /// The list of system devices that the container can access.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaDeviceMount
    ///
    /// Update requires: Replacement
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub devices: Option<Vec<LambdaDeviceMount>>,

    ///
    /// The memory size of the container, expressed in kilobytes.
    ///
    /// Default: 16384 (16 MB)
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemorySizeInKB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub memory_size_in_kb: Option<i64>,

    ///
    /// Whether or not the container can read information from the device's /sys    folder.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountROSysfs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mount_rosysfs: Option<bool>,

    ///
    /// The list of volumes that the container can access.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaVolumeMount
    ///
    /// Update requires: Replacement
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub volumes: Option<Vec<LambdaVolumeMount>>,
}

impl cfn_resources::CfnResource for LambdaContainerParams {
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

/// Contains information about a device that Linux processes in a container can access.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaDeviceMount {
    ///
    /// Whether or not to add the component's system user as an owner of the device.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AddGroupOwner")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub add_group_owner: Option<bool>,

    ///
    /// The mount path for the device in the file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// The permission to access the device: read/only (ro) or read/write     (rw).
    ///
    /// Default: ro
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub permission: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaDeviceMount {
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

/// Contains information about an event source for an AWS Lambda function. The    event source defines the topics on which this Lambda function subscribes to    receive messages that run the function.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaEventSource {
    ///
    /// The topic to which to subscribe to receive event messages.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Topic")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub topic: Option<cfn_resources::StrVal>,

    ///
    /// The type of event source. Choose from the following options:
    ///
    /// PUB_SUB – Subscribe to local publish/subscribe messages. This event      source type doesn't support MQTT wildcards (+ and #) in the      event source topic.        IOT_CORE – Subscribe to AWS IoT Core MQTT messages. This      event source type supports MQTT wildcards (+ and #) in the event      source topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaEventSource {
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

/// Contains parameters for a Lambda function that runs on AWS IoT Greengrass.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaExecutionParameters {
    ///
    /// The map of environment variables that are available to the Lambda function    when it runs.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentVariables")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,

    ///
    /// The list of event sources to which to subscribe to receive work messages. The Lambda function runs when it receives a message from an event source. You can    subscribe this function to local publish/subscribe messages and AWS IoT Core MQTT    messages.
    ///
    /// Required: No
    ///
    /// Type: List of LambdaEventSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventSources")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub event_sources: Option<Vec<LambdaEventSource>>,

    ///
    /// The list of arguments to pass to the Lambda function when it runs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecArgs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub exec_args: Option<Vec<String>>,

    ///
    /// The encoding type that the Lambda function supports.
    ///
    /// Default: json
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InputPayloadEncodingType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub input_payload_encoding_type: Option<cfn_resources::StrVal>,

    ///
    /// The parameters for the Linux process that contains the Lambda    function.
    ///
    /// Required: No
    ///
    /// Type: LambdaLinuxProcessParams
    ///
    /// Update requires: Replacement
    #[serde(rename = "LinuxProcessParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub linux_process_params: Option<LambdaLinuxProcessParams>,

    ///
    /// The maximum amount of time in seconds that a non-pinned Lambda function can    idle before the AWS IoT Greengrass Core software stops its process.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxIdleTimeInSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_idle_time_in_seconds: Option<i64>,

    ///
    /// The maximum number of instances that a non-pinned Lambda function can run at    the same time.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxInstancesCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_instances_count: Option<i64>,

    ///
    /// The maximum size of the message queue for the Lambda function component. The     AWS IoT Greengrass core device stores messages in a FIFO (first-in-first-out) queue until    it can run the Lambda function to consume each message.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxQueueSize")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_queue_size: Option<i64>,

    ///
    /// Whether or not the Lambda function is pinned, or long-lived.
    ///
    /// A pinned Lambda function starts when the AWS IoT Greengrass Core starts      and keeps running in its own container.        A non-pinned Lambda function starts only when it receives a work item      and exists after it idles for maxIdleTimeInSeconds. If the function has      multiple work items, the AWS IoT Greengrass Core software creates multiple instances of      the function.
    ///
    /// Default: true
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Pinned")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub pinned: Option<bool>,

    ///
    /// The interval in seconds at which a pinned (also known as long-lived) Lambda    function component sends status updates to the Lambda manager component.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatusTimeoutInSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status_timeout_in_seconds: Option<i64>,

    ///
    /// The maximum amount of time in seconds that the Lambda function can process a    work item.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub timeout_in_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for LambdaExecutionParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.linux_process_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about an AWS Lambda function to import to create a    component.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaFunctionRecipeSource {
    ///
    /// The component versions on which this Lambda function component    depends.
    ///
    /// Required: No
    ///
    /// Type: Map of ComponentDependencyRequirement
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentDependencies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub component_dependencies:
        Option<std::collections::HashMap<String, ComponentDependencyRequirement>>,

    ///
    /// The system and runtime parameters for the Lambda function as it runs on the     AWS IoT Greengrass core device.
    ///
    /// Required: No
    ///
    /// Type: LambdaExecutionParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentLambdaParameters")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub component_lambda_parameters: Option<LambdaExecutionParameters>,

    ///
    /// The name of the component.
    ///
    /// Defaults to the name of the Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub component_name: Option<cfn_resources::StrVal>,

    ///
    /// The platforms that the component version supports.
    ///
    /// Required: No
    ///
    /// Type: List of ComponentPlatform
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentPlatforms")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub component_platforms: Option<Vec<ComponentPlatform>>,

    ///
    /// The version of the component.
    ///
    /// Defaults to the version of the Lambda function as a semantic version. For    example, if your function version is 3, the component version becomes     3.0.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub component_version: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the Lambda function. The ARN must include the version of the    function to import. You can't use version aliases like $LATEST.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LambdaArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lambda_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaFunctionRecipeSource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.component_lambda_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains parameters for a Linux process that contains an AWS Lambda    function.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaLinuxProcessParams {
    ///
    /// The parameters for the container in which the Lambda function runs.
    ///
    /// Required: No
    ///
    /// Type: LambdaContainerParams
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerParams")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub container_params: Option<LambdaContainerParams>,

    ///
    /// The isolation mode for the process that contains the Lambda function. The    process can run in an isolated runtime environment inside the AWS IoT Greengrass    container, or as a regular process outside any container.
    ///
    /// Default: GreengrassContainer
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IsolationMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub isolation_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaLinuxProcessParams {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_params
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about a volume that Linux processes in a container can access. When    you define a volume, the AWS IoT Greengrass Core software mounts the source files to the    destination inside the container.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaVolumeMount {
    ///
    /// Whether or not to add the AWS IoT Greengrass user group as an owner of the    volume.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AddGroupOwner")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub add_group_owner: Option<bool>,

    ///
    /// The path to the logical volume in the file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPath")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub destination_path: Option<cfn_resources::StrVal>,

    ///
    /// The permission to access the volume: read/only (ro) or read/write     (rw).
    ///
    /// Default: ro
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub permission: Option<cfn_resources::StrVal>,

    ///
    /// The path to the physical volume in the file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub source_path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaVolumeMount {
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
