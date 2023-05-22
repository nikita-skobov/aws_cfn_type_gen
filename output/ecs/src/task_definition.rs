/// The details of a task definition which describes the container and volume definitions 			of an Amazon Elastic Container Service task. You can specify which Docker images to use, the required 			resources, and other configurations related to launching the task definition through an 			Amazon ECS service or task.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTaskDefinition {
    ///
    /// A list of container definitions in JSON format that describe the different containers 			that make up your task. For more information about container definition parameters and 			defaults, see Amazon ECS Task 				Definitions in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of ContainerDefinition
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,

    ///
    /// The number of cpu units used by the task. If you use the EC2 launch type, 			this field is optional. Any value can be used. If you use the Fargate launch type, this 			field is required. You must use one of the following values. The value that you choose 			determines your range of valid values for the memory parameter.
    ///
    /// The CPU units cannot be less than 1 vCPU when you use Windows containers on 			Fargate.
    ///
    /// 256 (.25 vCPU) - Available memory values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)               512 (.5 vCPU) - Available memory values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)               1024 (1 vCPU) - Available memory values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)               2048 (2 vCPU) - Available memory values: 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)               4096 (4 vCPU) - Available memory values: 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)               8192 (8 vCPU) - Available memory values: 16 GB and 60 GB in 4 GB increments        This option requires Linux platform 1.4.0 or                     later.               16384 (16vCPU) - Available memory values: 32GB and 120 GB in 8 GB increments        This option requires Linux platform 1.4.0 or                     later.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<cfn_resources::StrVal>,

    ///
    /// The ephemeral storage settings to use for tasks run with the task definition.
    ///
    /// Required: No
    ///
    /// Type: EphemeralStorage
    ///
    /// Update requires: Replacement
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,

    ///
    /// The Amazon Resource Name (ARN) of the task execution role that grants the Amazon ECS container agent       permission to make AWS API calls on your behalf. The task execution IAM role is required       depending on the requirements of your task. For more information, see Amazon ECS task         execution IAM role in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of a family that this task definition is registered to. Up to 255 letters (uppercase and lowercase),  numbers, hyphens, and underscores are allowed.
    ///
    /// A family groups multiple versions of a task definition. Amazon ECS gives the first task definition that you  registered to a family a revision number of 1. Amazon ECS gives sequential revision numbers to each task definition  that you add.
    ///
    /// NoteTo use revision numbers when you update a task definition, specify this property. If you don't specify a value,   AWS CloudFormation generates a new task definition each time that you update it.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<cfn_resources::StrVal>,

    ///
    /// The Elastic Inference accelerators to use for the containers in the task.
    ///
    /// Required: No
    ///
    /// Type: List of InferenceAccelerator
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceAccelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,

    ///
    /// The IPC resource namespace to use for the containers in the task. The valid values are         host, task, or none. If host is       specified, then all containers within the tasks that specified the host IPC       mode on the same container instance share the same IPC resources with the host Amazon EC2       instance. If task is specified, all containers within the specified task       share the same IPC resources. If none is specified, then IPC resources       within the containers of a task are private and not shared with other containers in a       task or on the container instance. If no value is specified, then the IPC resource       namespace sharing depends on the Docker daemon setting on the container instance. For       more information, see IPC         settings in the Docker run reference.
    ///
    /// If the host IPC mode is used, be aware that there is a heightened risk of       undesired IPC namespace expose. For more information, see Docker       security.
    ///
    /// If you are setting namespaced kernel parameters using systemControls for       the containers in the task, the following will apply to your IPC resource namespace. For       more information, see System         Controls in the Amazon Elastic Container Service Developer Guide.
    ///
    /// For tasks that use the host IPC mode, IPC namespace related             systemControls are not supported.               For tasks that use the task IPC mode, IPC namespace related             systemControls will apply to all containers within a           task.
    ///
    /// NoteThis parameter is not supported for Windows containers or tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: host | none | task
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<TaskDefinitionIpcModeEnum>,

    ///
    /// The amount (in MiB) of memory used by the task.
    ///
    /// If your tasks runs on Amazon EC2 instances, you must specify either a task-level memory 			value or a container-level memory value. This field is optional and any value can be 			used. If a task-level memory value is specified, the container-level memory value is 			optional. For more information regarding container-level memory and memory reservation, 			see ContainerDefinition.
    ///
    /// If your tasks runs on AWS Fargate, this field is required. You must use one of the 			following values. The value you choose determines your range of valid values for the 				cpu parameter.
    ///
    /// 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available cpu values: 256 (.25 vCPU)               1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available cpu values: 512 (.5 vCPU)               2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available cpu values: 1024 (1 vCPU)               Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available cpu values: 2048 (2 vCPU)               Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available cpu values: 4096 (4 vCPU)               Between 16 GB and 60 GB in 4 GB increments - Available cpu values: 8192 (8 vCPU)        This option requires Linux platform 1.4.0 or                     later.               Between 32GB and 120 GB in 8 GB increments - Available cpu values: 16384 (16 vCPU)        This option requires Linux platform 1.4.0 or                     later.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<cfn_resources::StrVal>,

    ///
    /// The Docker networking mode to use for the containers in the task. The valid values are         none, bridge, awsvpc, and host.       If no network mode is specified, the default is bridge.
    ///
    /// For Amazon ECS tasks on Fargate, the awsvpc network mode is required.       For Amazon ECS tasks on Amazon EC2 Linux instances, any network mode can be used. For Amazon ECS tasks on Amazon EC2 Windows instances, <default> or awsvpc can be used. If the network       mode is set to none, you cannot specify port mappings in your container       definitions, and the tasks containers do not have external connectivity. The         host and awsvpc network modes offer the highest networking       performance for containers because they use the EC2 network stack instead of the       virtualized network stack provided by the bridge mode.
    ///
    /// With the host and awsvpc network modes, exposed container       ports are mapped directly to the corresponding host port (for the host       network mode) or the attached elastic network interface port (for the         awsvpc network mode), so you cannot take advantage of dynamic host port       mappings.
    ///
    /// ImportantWhen using the host network mode, you should not run               containers using the root user (UID 0). It is considered best practice               to use a non-root user.
    ///
    /// If the network mode is awsvpc, the task is allocated an elastic network       interface, and you must specify a NetworkConfiguration value when you create       a service or run a task with the task definition. For more information, see Task Networking in the         Amazon Elastic Container Service Developer Guide.
    ///
    /// If the network mode is host, you cannot run multiple instantiations of the       same task on a single container instance when port mappings are used.
    ///
    /// For more information, see Network         settings in the Docker run reference.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: awsvpc | bridge | host | none
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<TaskDefinitionNetworkModeEnum>,

    ///
    /// The process namespace to use for the containers in the task. The valid               values are host or task. If host               is specified, then all containers within the tasks that specified the                 host PID mode on the same container instance share the               same process namespace with the host Amazon EC2 instance. If task is               specified, all containers within the specified task share the same               process namespace. If no value is specified, the default is a private               namespace. For more information, see PID settings in the Docker run                 reference.
    ///
    /// If the host PID mode is used, be aware that there is a               heightened risk of undesired process namespace expose. For more               information, see Docker                 security.
    ///
    /// NoteThis parameter is not supported for Windows containers or tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: host | task
    ///
    /// Update requires: Replacement
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<TaskDefinitionPidModeEnum>,

    ///
    /// An array of placement constraint objects to use for tasks.
    ///
    /// NoteThis parameter isn't supported for tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: List of TaskDefinitionPlacementConstraint
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,

    ///
    /// The configuration details for the App Mesh proxy.
    ///
    /// Your Amazon ECS container instances require at least version 1.26.0 of the container agent 			and at least version 1.26.0-1 of the ecs-init package to use a proxy 			configuration. If your container instances are launched from the Amazon ECS optimized AMI 			version 20190301 or later, they contain the required versions of the 			container agent and ecs-init. For more information, see Amazon ECS-optimized Linux AMI in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ProxyConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProxyConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<ProxyConfiguration>,

    ///
    /// The task launch types the task definition was validated against. For more information, see Amazon ECS launch types 			in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RequiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,

    ///
    /// The operating system that your tasks definitions run on. A platform family is 			specified only for tasks using the Fargate launch type.
    ///
    /// When you specify a task definition in a service, this value must match the 				runtimePlatform value of the service.
    ///
    /// Required: No
    ///
    /// Type: RuntimePlatform
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuntimePlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,

    ///
    /// The metadata that you apply to the task definition to help you categorize and organize 			them. Each tag consists of a key and an optional value. You define both of them.
    ///
    /// The following basic restrictions apply to tags:
    ///
    /// Maximum number of tags per resource - 50               For each resource, each tag key must be unique, and each tag key can have only           one value.               Maximum key length - 128 Unicode characters in UTF-8               Maximum value length - 256 Unicode characters in UTF-8               If your tagging schema is used across multiple services and resources,           remember that other services may have restrictions on allowed characters.           Generally allowed characters are: letters, numbers, and spaces representable in           UTF-8, and the following characters: + - = . _ : / @.               Tag keys and values are case-sensitive.               Do not use aws:, AWS:, or any upper or lowercase           combination of such as a prefix for either keys or values as it is reserved for           AWS use. You cannot edit or delete tag keys or values with this prefix. Tags with           this prefix do not count against your tags per resource limit.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The short name or full Amazon Resource Name (ARN) of the AWS Identity and Access Management role that grants containers in the 			task permission to call AWS APIs on your behalf. For more information, see Amazon ECS 				Task Role in the Amazon Elastic Container Service Developer Guide.
    ///
    /// IAM roles for tasks on Windows require that the -EnableTaskIAMRole option 			is set when you launch the Amazon ECS-optimized Windows AMI. Your containers must also run some 			configuration code to use the feature. For more information, see Windows IAM roles 				for tasks in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TaskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The list of data volume definitions for the task. For more information, see Using data volumes in tasks in the Amazon Elastic Container Service Developer Guide.
    ///
    /// NoteThe host and sourcePath parameters aren't supported for 				tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: List of Volume
    ///
    /// Update requires: Replacement
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,

    #[serde(skip_serializing)]
    pub att_task_definition_arn: CfnTaskDefinitiontaskdefinitionarn,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskDefinitionIpcModeEnum {
    /// host
    #[serde(rename = "host")]
    Host,

    /// none
    #[serde(rename = "none")]
    None,

    /// task
    #[serde(rename = "task")]
    Task,
}

impl Default for TaskDefinitionIpcModeEnum {
    fn default() -> Self {
        TaskDefinitionIpcModeEnum::Host
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskDefinitionNetworkModeEnum {
    /// awsvpc
    #[serde(rename = "awsvpc")]
    Awsvpc,

    /// bridge
    #[serde(rename = "bridge")]
    Bridge,

    /// host
    #[serde(rename = "host")]
    Host,

    /// none
    #[serde(rename = "none")]
    None,
}

impl Default for TaskDefinitionNetworkModeEnum {
    fn default() -> Self {
        TaskDefinitionNetworkModeEnum::Awsvpc
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskDefinitionPidModeEnum {
    /// host
    #[serde(rename = "host")]
    Host,

    /// task
    #[serde(rename = "task")]
    Task,
}

impl Default for TaskDefinitionPidModeEnum {
    fn default() -> Self {
        TaskDefinitionPidModeEnum::Host
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTaskDefinitiontaskdefinitionarn;
impl CfnTaskDefinitiontaskdefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"TaskDefinitionArn"#
    }
}

impl cfn_resources::CfnResource for CfnTaskDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::ECS::TaskDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ephemeral_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.proxy_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.runtime_platform
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The authorization configuration details for the Amazon EFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthorizationConfig {
    ///
    /// The Amazon EFS access point ID to use. If an access point is specified, the root directory 			value specified in the EFSVolumeConfiguration must either be omitted or set 			to / which will enforce the path set on the EFS access point. If an access 			point is used, transit encryption must be on in the 				EFSVolumeConfiguration. For more information, see Working with Amazon 				EFS access points in the Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<cfn_resources::StrVal>,

    ///
    /// Determines whether to use the Amazon ECS task role defined in a task definition when 			mounting the Amazon EFS file system. If it is turned on, transit encryption must be turned on in the 				EFSVolumeConfiguration. If this parameter is omitted, the default value 			of DISABLED is used. For more information, see Using 				Amazon EFS access points in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: Replacement
    #[serde(rename = "IAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<AuthorizationConfigIAMEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AuthorizationConfigIAMEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for AuthorizationConfigIAMEnum {
    fn default() -> Self {
        AuthorizationConfigIAMEnum::Disabled
    }
}

impl cfn_resources::CfnResource for AuthorizationConfig {
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

/// The ContainerDefinition property specifies a container definition. Container definitions are used  in task definitions to describe the different containers that are launched as part of a task.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerDefinition {
    ///
    /// The command that's passed to the container. This parameter maps to Cmd in 			the Create a container section of the Docker Remote API and the 				COMMAND parameter to docker 				run. For more information, see https://docs.docker.com/engine/reference/builder/#cmd. If there are multiple arguments, each 			argument is a separated string in the array.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    ///
    /// The number of cpu units reserved for the container. This parameter maps 			to CpuShares in the Create a container section of the 			Docker Remote API and the --cpu-shares option to docker run.
    ///
    /// This field is optional for tasks using the Fargate launch type, and the 			only requirement is that the total amount of CPU reserved for all containers within a 			task be lower than the task-level cpu value.
    ///
    /// NoteYou can determine the number of CPU units that are available per EC2 instance type 				by multiplying the vCPUs listed for that instance type on the Amazon EC2 Instances detail page 				by 1,024.
    ///
    /// Linux containers share unallocated CPU units with other containers on the container 			instance with the same ratio as their allocated amount. For example, if you run a 			single-container task on a single-core instance type with 512 CPU units specified for 			that container, and that's the only task running on the container instance, that 			container could use the full 1,024 CPU unit share at any given time. However, if you 			launched another copy of the same task on that container instance, each task is 			guaranteed a minimum of 512 CPU units when needed. Moreover, each container could float 			to higher CPU usage if the other container was not using it. If both tasks were 100% 			active all of the time, they would be limited to 512 CPU units.
    ///
    /// On Linux container instances, the Docker daemon on the container instance uses the CPU 			value to calculate the relative CPU share ratios for running containers. For more 			information, see CPU share 				constraint in the Docker documentation. The minimum valid CPU share value 			that the Linux kernel allows is 2. However, the CPU parameter isn't required, and you 			can use CPU values below 2 in your container definitions. For CPU values below 2 			(including null), the behavior varies based on your Amazon ECS container agent 			version:
    ///
    /// Agent versions less than or equal to 1.1.0: 					Null and zero CPU values are passed to Docker as 0, which Docker then converts 					to 1,024 CPU shares. CPU values of 1 are passed to Docker as 1, which the Linux 					kernel converts to two CPU shares.                        Agent versions greater than or equal to 1.2.0: 					Null, zero, and CPU values of 1 are passed to Docker as 2.
    ///
    /// On Windows container instances, the CPU limit is enforced as an absolute limit, or a 			quota. Windows containers only have access to the specified amount of CPU that's 			described in the task definition. A null or zero CPU value is passed to Docker as 				0, which Windows interprets as 1% of one CPU.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,

    ///
    /// The dependencies defined for container startup and shutdown. A container can contain multiple dependencies. When  a dependency is defined for container startup, for container shutdown it is reversed.
    ///
    /// For tasks using the EC2 launch type, the container instances require at least version 1.26.0 of the container  agent to turn on container dependencies. However, we recommend using the latest container agent version. For  information about checking your agent version and updating to the latest version, see Updating the Amazon ECS Container Agent in the   Amazon Elastic Container Service Developer Guide. If you're using an Amazon ECS-optimized Linux  AMI, your instance needs at least version 1.26.0-1 of the ecs-init package. If your container instances  are launched from version 20190301 or later, then they contain the required versions of the container  agent and ecs-init. For more information, see Amazon ECS-optimized Linux AMI in the   Amazon Elastic Container Service Developer Guide.
    ///
    /// For tasks using the Fargate launch type, the task or service requires the following platforms:
    ///
    /// Linux platform version 1.3.0 or later.     Windows platform version 1.0.0 or later.
    ///
    /// If the task definition is used in a blue/green deployment that uses AWS::CodeDeploy::DeploymentGroup BlueGreenDeploymentConfiguration, the dependsOn parameter is  not supported. For more information see Issue #680 on the on  the GitHub website.
    ///
    /// Required: No
    ///
    /// Type: List of ContainerDependency
    ///
    /// Update requires: Replacement
    #[serde(rename = "DependsOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<ContainerDependency>>,

    ///
    /// When this parameter is true, networking is off within the container. This parameter maps to 				NetworkDisabled in the Create a container section of 			the Docker Remote API.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DisableNetworking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,

    ///
    /// A list of DNS search domains that are presented to the container. This parameter maps 			to DnsSearch in the Create a container section of the 			Docker Remote API and the --dns-search option to docker run.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DnsSearchDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,

    ///
    /// A list of DNS servers that are presented to the container. This parameter maps to 				Dns in the Create a container section of the 			Docker Remote API and the --dns option to docker run.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DnsServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,

    ///
    /// A key/value map of labels to add to the container. This parameter maps to 				Labels in the Create a container section of the 			Docker Remote API and the --label option to docker run. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DockerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<std::collections::HashMap<String, String>>,

    ///
    /// A list of strings to provide custom configuration for multiple 			security systems. For more information about valid values, see Docker Run Security Configuration. This field isn't valid 			for containers in tasks using the Fargate launch 			type.
    ///
    /// For Linux tasks on EC2, this parameter can be used to reference custom 			labels for SELinux and AppArmor multi-level security systems.
    ///
    /// For any tasks on EC2, this parameter can be used to reference a 			credential spec file that configures a container for Active Directory 			authentication. For more 			information, see Using gMSAs for Windows 				Containers and Using gMSAs for Linux 					Containers in the Amazon Elastic Container Service Developer Guide.
    ///
    /// This parameter maps to SecurityOpt in the 			Create a container section of the Docker Remote API and the 				--security-opt option to docker 				run.
    ///
    /// NoteThe Amazon ECS container agent running on a container instance must register with the 					ECS_SELINUX_CAPABLE=true or ECS_APPARMOR_CAPABLE=true 				environment variables before containers placed on that instance can use these 				security options. For more information, see Amazon ECS Container 					Agent Configuration in the Amazon Elastic Container Service Developer Guide.
    ///
    /// For more information about valid values, see Docker 				Run Security Configuration.
    ///
    /// Valid values: "no-new-privileges" | "apparmor:PROFILE" | "label:value" | 			"credentialspec:CredentialSpecFilePath"
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DockerSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,

    ///
    /// ImportantEarly versions of the Amazon ECS container agent don't properly handle 					entryPoint parameters. If you have problems using 					entryPoint, update your container agent or enter your commands and 				arguments as command array items instead.
    ///
    /// The entry point that's passed to the container. This parameter maps to 				Entrypoint in the Create a container section of the 			Docker Remote API and the --entrypoint option to docker run. For more information, see https://docs.docker.com/engine/reference/builder/#entrypoint.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,

    ///
    /// The environment variables to pass to a container. This parameter maps to 				Env in the Create a container section of the 			Docker Remote API and the --env option to docker run.
    ///
    /// ImportantWe don't recommend that you use plaintext environment variables for sensitive 				information, such as credential data.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,

    ///
    /// A list of files containing the environment variables to pass to a container. This 			parameter maps to the --env-file option to docker run.
    ///
    /// You can specify up to ten environment files. The file must have a .env 			file extension. Each line in an environment file contains an environment variable in 				VARIABLE=VALUE format. Lines beginning with # are treated 			as comments and are ignored. For more information about the environment variable file 			syntax, see Declare default 				environment variables in file.
    ///
    /// If there are environment variables specified using the environment 			parameter in a container definition, they take precedence over the variables contained 			within an environment file. If multiple environment files are specified that contain the 			same variable, they're processed from the top down. We recommend that you use unique 			variable names. For more information, see Specifying Environment 				Variables in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of EnvironmentFile
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EnvironmentFile>>,

    ///
    /// If the essential parameter of a container is marked as true, 			and that container fails or stops for any reason, all other containers that are part of 			the task are stopped. If the essential parameter of a container is marked 			as false, its failure doesn't affect the rest of the containers in a task. 			If this parameter is omitted, a container is assumed to be essential.
    ///
    /// All tasks must have at least one essential container. If you have an application 			that's composed of multiple containers, group containers that are used for a common 			purpose into components, and separate the different components into multiple task 			definitions. For more information, see Application 				Architecture in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Essential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,

    ///
    /// A list of hostnames and IP address mappings to append to the /etc/hosts 			file on the container. This parameter maps to ExtraHosts in the 			Create a container section of the Docker Remote API and the 				--add-host option to docker 				run.
    ///
    /// NoteThis parameter isn't supported for Windows containers or tasks that use the 					awsvpc network mode.
    ///
    /// Required: No
    ///
    /// Type: List of HostEntry
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<HostEntry>>,

    ///
    /// The FireLens configuration for the container. This is used to specify and configure a 			log router for container logs. For more information, see Custom Log Routing 			in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: FirelensConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirelensConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,

    ///
    /// The container health check command and associated configuration parameters for the 			container. This parameter maps to HealthCheck in the 			Create a container section of the Docker Remote API and the 				HEALTHCHECK parameter of docker 				run.
    ///
    /// Required: No
    ///
    /// Type: HealthCheck
    ///
    /// Update requires: Replacement
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,

    ///
    /// The hostname to use for your container. This parameter maps to Hostname 			in the Create a container section of the Docker Remote API and the 				--hostname option to docker 				run.
    ///
    /// NoteThe hostname parameter is not supported if you're using the 					awsvpc network mode.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<cfn_resources::StrVal>,

    ///
    /// The image used to start a container. This string is passed directly to the Docker 			daemon. By default, images in the Docker Hub registry are available. Other repositories 			are specified with either         repository-url/image:tag       or         repository-url/image@digest       . Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to Image in the 			Create a container section of the Docker Remote API and the 				IMAGE parameter of docker 				run.
    ///
    /// When a new task starts, the Amazon ECS container agent pulls the latest version of 					the specified image and tag for the container to use. However, subsequent 					updates to a repository image aren't propagated to already running tasks.               Images in Amazon ECR repositories can be specified by either using the full 						registry/repository:tag or 						registry/repository@digest. For example, 						012345678910.dkr.ecr.<region-name>.amazonaws.com/<repository-name>:latest 					or 						012345678910.dkr.ecr.<region-name>.amazonaws.com/<repository-name>@sha256:94afd1f2e64d908bc90dbca0035a5b567EXAMPLE. 				               Images in official repositories on Docker Hub use a single name (for example, 						ubuntu or mongo).               Images in other repositories on Docker Hub are qualified with an organization 					name (for example, amazon/amazon-ecs-agent).               Images in other online repositories are qualified further by a domain name 					(for example, quay.io/assemblyline/ubuntu).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Image")]
    pub image: cfn_resources::StrVal,

    ///
    /// When this parameter is true, you can deploy containerized applications 			that require stdin or a tty to be allocated. This parameter 			maps to OpenStdin in the Create a container section of the 			Docker Remote API and the --interactive option to docker run.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Interactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,

    ///
    /// The links parameter allows containers to communicate with each other 			without the need for port mappings. This parameter is only supported if the network mode 			of a task definition is bridge. The name:internalName 			construct is analogous to name:alias in Docker links. 			Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. For more information about linking Docker containers, go to 				Legacy container links 			in the Docker documentation. This parameter maps to Links in the 			Create a container section of the Docker Remote API and the 				--link option to docker 			run.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// ImportantContainers that are collocated on a single container instance may be able to 				communicate with each other without requiring links or host port mappings. Network 				isolation is achieved on the container instance using security groups and VPC 				settings.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,

    ///
    /// Linux-specific modifications that are applied to the container, such as Linux kernel capabilities. For more  information see KernelCapabilities.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: LinuxParameters
    ///
    /// Update requires: Replacement
    #[serde(rename = "LinuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,

    ///
    /// The log configuration specification for the container.
    ///
    /// This parameter maps to LogConfig in the Create a container section of the   Docker Remote API and the --log-driver  option to docker run. By default, containers use  the same logging driver that the Docker daemon uses. However, the container may use a different logging driver than  the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different  logging driver for a container, the log system must be configured properly on the container instance (or on a  different log server for remote logging options). For more information on the options for different supported log  drivers, see Configure logging drivers in  the Docker documentation.
    ///
    /// NoteAmazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the   LogConfiguration   data type). Additional log drivers may be available in future releases of the Amazon ECS container agent.
    ///
    /// This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check  the Docker Remote API version on your container instance, log in to your container instance and run the following  command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// NoteThe Amazon ECS container agent running on a container instance must register the logging drivers available on   that instance with the ECS_AVAILABLE_LOGGING_DRIVERS environment variable before containers placed on   that instance can use these log configuration options. For more information, see Amazon ECS Container Agent Configuration in the   Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,

    ///
    /// The amount (in MiB) of memory to present to the container. If your container attempts to exceed the memory  specified here, the container is killed. The total amount of memory reserved for all containers within a task must be  lower than the task memory value, if one is specified. This parameter maps to Memory in the   Create a container section  of the Docker Remote API and the --memory  option to docker  run.
    ///
    /// If using the Fargate launch type, this parameter is optional.
    ///
    /// If using the EC2 launch type, you must specify either a task-level memory value or a container-level memory  value. If you specify both a container-level memory and memoryReservation value,   memory must be greater than memoryReservation. If you specify   memoryReservation, then that value is subtracted from the available memory resources for the container  instance where the container is placed. Otherwise, the value of memory is used.
    ///
    /// The Docker 20.10.0 or later daemon reserves a minimum of 6 MiB of memory for a container, so you should not  specify fewer than 6 MiB of memory for your containers.
    ///
    /// The Docker 19.03.13-ce or earlier daemon reserves a minimum of 4 MiB of memory for a container, so you should  not specify fewer than 4 MiB of memory for your containers.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,

    ///
    /// The soft limit (in MiB) of memory to reserve for the container. When system memory is 			under heavy contention, Docker attempts to keep the container memory to this soft limit. 			However, your container can consume more memory when it needs to, up to either the hard 			limit specified with the memory parameter (if applicable), or all of the 			available memory on the container instance, whichever comes first. This parameter maps 			to MemoryReservation in the Create a container section of 			the Docker Remote API and the --memory-reservation option to docker run.
    ///
    /// If a task-level memory value is not specified, you must specify a non-zero integer for 			one or both of memory or memoryReservation in a container 			definition. If you specify both, memory must be greater than 				memoryReservation. If you specify memoryReservation, then 			that value is subtracted from the available memory resources for the container instance 			where the container is placed. Otherwise, the value of memory is 			used.
    ///
    /// For example, if your container normally uses 128 MiB of memory, but occasionally 			bursts to 256 MiB of memory for short periods of time, you can set a 				memoryReservation of 128 MiB, and a memory hard limit of 			300 MiB. This configuration would allow the container to only reserve 128 MiB of memory 			from the remaining resources on the container instance, but also allow the container to 			consume more memory resources when needed.
    ///
    /// The Docker 20.10.0 or later daemon reserves a minimum of 6 MiB of memory for a 			container. So, don't specify less than 6 MiB of memory for your containers.
    ///
    /// The Docker 19.03.13-ce or earlier daemon reserves a minimum of 4 MiB of memory for a 			container. So, don't specify less than 4 MiB of memory for your containers.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,

    ///
    /// The mount points for data volumes in your container.
    ///
    /// This parameter maps to Volumes in the Create a container 			section of the Docker Remote API and the --volume option to docker run.
    ///
    /// Windows containers can mount whole directories on the same drive as 				$env:ProgramData. Windows containers can't mount directories on a 			different drive, and mount point can't be across drives.
    ///
    /// Required: No
    ///
    /// Type: List of MountPoint
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,

    ///
    /// The name of a container. If you're linking multiple containers together in a task 			definition, the name of one container can be entered in the 				links of another container to connect the containers. 			Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. This parameter maps to name in the 			Create a container section of the Docker Remote API and the 				--name option to docker 			run.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The list of port mappings for the container. Port mappings allow containers to access ports on the host  container instance to send or receive traffic.
    ///
    /// For task definitions that use the awsvpc network mode, you should only specify the   containerPort. The hostPort can be left blank or it must be the same value as the   containerPort.
    ///
    /// Port mappings on Windows use the NetNAT gateway address rather than localhost. There  is no loopback for port mappings on Windows, so you cannot access a container's mapped port from the host itself.
    ///
    /// This parameter maps to PortBindings in the Create a container section of the   Docker Remote API and the --publish  option to docker run. If the network mode of a  task definition is set to none, then you can't specify port mappings. If the network mode of a task  definition is set to host, then host ports must either be undefined or they must match the container  port in the port mapping.
    ///
    /// NoteAfter a task reaches the RUNNING status, manual and automatic host and container port assignments   are visible in the Network Bindings section of a container description for a   selected task in the Amazon ECS console. The assignments are also visible in the networkBindings   section DescribeTasks   responses.
    ///
    /// Required: No
    ///
    /// Type: List of PortMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "PortMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,

    ///
    /// When this parameter is true, the container is given elevated privileges on the host 			container instance (similar to the root user). This parameter maps to 				Privileged in the Create a container section of the 			Docker Remote API and the --privileged option to docker run.
    ///
    /// NoteThis parameter is not supported for Windows containers or tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    ///
    /// When this parameter is true, a TTY is allocated. This parameter maps to 				Tty in the Create a container section of the 			Docker Remote API and the --tty option to docker run.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PseudoTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,

    ///
    /// When this parameter is true, the container is given read-only access to its root file 			system. This parameter maps to ReadonlyRootfs in the 			Create a container section of the Docker Remote API and the 				--read-only option to docker 				run.
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReadonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,

    ///
    /// The private repository authentication credentials to use.
    ///
    /// Required: No
    ///
    /// Type: RepositoryCredentials
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,

    ///
    /// The type and amount of a resource to assign to a container. The only supported 			resource is a GPU.
    ///
    /// Required: No
    ///
    /// Type: List of ResourceRequirement
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,

    ///
    /// The secrets to pass to the container. For more information, see Specifying 				Sensitive Data in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Secret
    ///
    /// Update requires: Replacement
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,

    ///
    /// Time duration (in seconds) to wait before giving up on resolving dependencies for a 			container. For example, you specify two containers in a task definition with containerA 			having a dependency on containerB reaching a COMPLETE, 			SUCCESS, or HEALTHY status. If a startTimeout 			value is specified for containerB and it doesn't reach the desired status within that 			time then containerA gives up and not start. This results in the task transitioning to a 				STOPPED state.
    ///
    /// NoteWhen the ECS_CONTAINER_START_TIMEOUT container agent configuration 				variable is used, it's enforced independently from this start timeout value.
    ///
    /// For tasks using the Fargate launch type, the task or service requires 			the following platforms:
    ///
    /// Linux platform version 1.3.0 or later.               Windows platform version 1.0.0 or later.
    ///
    /// For tasks using the EC2 launch type, your container instances require at 			least version 1.26.0 of the container agent to use a container start 			timeout value. However, we recommend using the latest container agent version. For 			information about checking your agent version and updating to the latest version, see 				Updating the Amazon ECS 				Container Agent in the Amazon Elastic Container Service Developer Guide. If you're using an Amazon ECS-optimized Linux AMI, 			your instance needs at least version 1.26.0-1 of the ecs-init 			package. If your container instances are launched from version 20190301 or 			later, then they contain the required versions of the container agent and 				ecs-init. For more information, see Amazon ECS-optimized Linux AMI 			in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i64>,

    ///
    /// Time duration (in seconds) to wait before the container is forcefully killed if it 			doesn't exit normally on its own.
    ///
    /// For tasks using the Fargate launch type, the task or service requires 			the following platforms:
    ///
    /// Linux platform version 1.3.0 or later.               Windows platform version 1.0.0 or later.
    ///
    /// The max stop timeout value is 120 seconds and if the parameter is not specified, the 			default value of 30 seconds is used.
    ///
    /// For tasks that use the EC2 launch type, if the stopTimeout 			parameter isn't specified, the value set for the Amazon ECS container agent configuration 			variable ECS_CONTAINER_STOP_TIMEOUT is used. If neither the 				stopTimeout parameter or the ECS_CONTAINER_STOP_TIMEOUT 			agent configuration variable are set, then the default values of 30 seconds for Linux 			containers and 30 seconds on Windows containers are used. Your container instances 			require at least version 1.26.0 of the container agent to use a container stop timeout 			value. However, we recommend using the latest container agent version. For information 			about checking your agent version and updating to the latest version, see Updating the Amazon ECS Container Agent in the Amazon Elastic Container Service Developer Guide. If you're using 			an Amazon ECS-optimized Linux AMI, your instance needs at least version 1.26.0-1 of the 				ecs-init package. If your container instances are launched from version 				20190301 or later, then they contain the required versions of the 			container agent and ecs-init. For more information, see Amazon ECS-optimized Linux AMI in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,

    ///
    /// A list of namespaced kernel parameters to set in the container. This parameter maps to 				Sysctls in the Create a container section of the 			Docker Remote API and the --sysctl option to docker run.
    ///
    /// NoteWe don't recommended that you specify network-related systemControls 				parameters for multiple containers in a single task that also uses either the 					awsvpc or host network modes. For tasks that use the 					awsvpc network mode, the container that's started last determines 				which systemControls parameters take effect. For tasks that use the 					host network mode, it changes the container instance's namespaced 				kernel parameters as well as the containers.
    ///
    /// Required: No
    ///
    /// Type: List of SystemControl
    ///
    /// Update requires: Replacement
    #[serde(rename = "SystemControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<SystemControl>>,

    ///
    /// A list of ulimits to set in the container. This parameter maps to Ulimits in the   Create a container section  of the Docker Remote API and the --ulimit  option to docker run. Valid naming values are  displayed in the Ulimit data  type. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check  the Docker Remote API version on your container instance, log in to your container instance and run the following  command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: List of Ulimit
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,

    ///
    /// The user to use inside the container. This parameter maps to User in the 			Create a container section of the Docker Remote API and the 				--user option to docker 			run.
    ///
    /// ImportantWhen running tasks using the host network mode, don't run containers 				using the root user (UID 0). We recommend using a non-root user for better 				security.
    ///
    /// You can specify the user using the following formats. If specifying a UID 			or GID, you must specify it as a positive integer.
    ///
    /// user                                user:group                                uid                                uid:gid                                user:gid                                uid:group
    ///
    /// NoteThis parameter is not supported for Windows containers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<cfn_resources::StrVal>,

    ///
    /// Data volumes to mount from another container. This parameter maps to 				VolumesFrom in the Create a container section of the 			Docker Remote API and the --volumes-from option to docker run.
    ///
    /// Required: No
    ///
    /// Type: List of VolumeFrom
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<VolumeFrom>>,

    ///
    /// The working directory to run commands inside the container in. This parameter maps to 				WorkingDir in the Create a container section of the 			Docker Remote API and the --workdir option to docker run.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ContainerDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.firelens_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.health_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.linux_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.log_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.repository_credentials
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The ContainerDependency property specifies the dependencies defined for container startup and  shutdown. A container can contain multiple dependencies. When a dependency is defined for container startup, for  container shutdown it is reversed.
///
/// Your Amazon ECS container instances require at least version 1.26.0 of the container agent to enable container  dependencies. However, we recommend using the latest container agent version. For information about checking your  agent version and updating to the latest version, see Updating the Amazon ECS Container Agent in the   Amazon Elastic Container Service Developer Guide. If you are using an Amazon ECS-optimized  Linux AMI, your instance needs at least version 1.26.0-1 of the ecs-init package. If your container  instances are launched from version 20190301 or later, then they contain the required versions of the  container agent and ecs-init. For more information, see Amazon ECS-optimized Linux AMI in the   Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerDependency {
    ///
    /// The dependency condition of the container. The following are the available conditions 			and their behavior:
    ///
    /// START - This condition emulates the behavior of links and 					volumes today. It validates that a dependent container is started before 					permitting other containers to start.                        COMPLETE - This condition validates that a dependent 					container runs to completion (exits) before permitting other containers to 					start. This can be useful for nonessential containers that run a script and then 					exit. This condition can't be set on an essential container.                        SUCCESS - This condition is the same as 						COMPLETE, but it also requires that the container exits with a 						zero status. This condition can't be set on an essential 					container.                        HEALTHY - This condition validates that the dependent 					container passes its Docker health check before permitting other containers to 					start. This requires that the dependent container has health checks configured. 					This condition is confirmed only at task startup.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMPLETE | HEALTHY | START | SUCCESS
    ///
    /// Update requires: Replacement
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ContainerDependencyConditionEnum>,

    ///
    /// The name of a container.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ContainerDependencyConditionEnum {
    /// COMPLETE
    #[serde(rename = "COMPLETE")]
    Complete,

    /// HEALTHY
    #[serde(rename = "HEALTHY")]
    Healthy,

    /// START
    #[serde(rename = "START")]
    Start,

    /// SUCCESS
    #[serde(rename = "SUCCESS")]
    Success,
}

impl Default for ContainerDependencyConditionEnum {
    fn default() -> Self {
        ContainerDependencyConditionEnum::Complete
    }
}

impl cfn_resources::CfnResource for ContainerDependency {
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

/// The Device property specifies an object representing a container instance host device.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Device {
    ///
    /// The path inside the container at which to expose the host device.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<cfn_resources::StrVal>,

    ///
    /// The path for the device on the host container instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<cfn_resources::StrVal>,

    ///
    /// The explicit permissions to provide to the container for the device. By default, the 			container has permissions for read, write, and 				mknod for the device.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for Device {
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

/// The DockerVolumeConfiguration property specifies a Docker volume configuration and is used when you  use Docker volumes. Docker volumes are only supported when you are using the EC2 launch type. Windows containers only  support the use of the local driver. To use bind mounts, specify a host instead.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DockerVolumeConfiguration {
    ///
    /// If this value is true, the Docker volume is created if it doesn't already 			exist.
    ///
    /// NoteThis field is only used if the scope is shared.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Autoprovision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,

    ///
    /// The Docker volume driver to use. The driver value must match the driver name provided 			by Docker because it is used for task placement. If the driver was installed using the 			Docker plugin CLI, use docker plugin ls to retrieve the driver name from 			your container instance. If the driver was installed using another method, use Docker 			plugin discovery to retrieve the driver name. For more information, see Docker 				plugin discovery. This parameter maps to Driver in the 			Create a volume section of the Docker Remote API and the 				xxdriver option to docker 				volume create.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<cfn_resources::StrVal>,

    ///
    /// A map of Docker driver-specific options passed through. This parameter maps to 				DriverOpts in the Create a volume section of the 			Docker Remote API and the xxopt option to docker 				volume create.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,

    ///
    /// Custom metadata to add to your Docker volume. This parameter maps to 				Labels in the Create a volume section of the 			Docker Remote API and the xxlabel option to docker 				volume create.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    ///
    /// The scope for the Docker volume that determines its lifecycle. Docker volumes that are 			scoped to a task are automatically provisioned when the task starts and 			destroyed when the task stops. Docker volumes that are scoped as shared 			persist after the task stops.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: shared | task
    ///
    /// Update requires: Replacement
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<DockerVolumeConfigurationScopeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DockerVolumeConfigurationScopeEnum {
    /// shared
    #[serde(rename = "shared")]
    Shared,

    /// task
    #[serde(rename = "task")]
    Task,
}

impl Default for DockerVolumeConfigurationScopeEnum {
    fn default() -> Self {
        DockerVolumeConfigurationScopeEnum::Shared
    }
}

impl cfn_resources::CfnResource for DockerVolumeConfiguration {
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

/// This parameter is specified when you're using an Amazon Elastic File System file system for task 			storage. For more information, see Amazon EFS volumes in the 			Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EFSVolumeConfiguration {
    ///
    /// The authorization configuration details for the Amazon EFS file system.
    ///
    /// Required: No
    ///
    /// Type: AuthorizationConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthorizationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<AuthorizationConfig>,

    ///
    /// The Amazon EFS file system ID to use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilesystemId")]
    pub filesystem_id: cfn_resources::StrVal,

    ///
    /// The directory within the Amazon EFS file system to mount as the root directory inside the 			host. If this parameter is omitted, the root of the Amazon EFS volume will be used. 			Specifying / will have the same effect as omitting this parameter.
    ///
    /// ImportantIf an EFS access point is specified in the authorizationConfig, the 				root directory parameter must either be omitted or set to / which will 				enforce the path set on the EFS access point.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<cfn_resources::StrVal>,

    ///
    /// Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host 			and the Amazon EFS server. Transit encryption must be turned on if Amazon EFS IAM authorization is 			used. If this parameter is omitted, the default value of DISABLED is used. 			For more information, see Encrypting data in transit in 			the Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<EFSVolumeConfigurationTransitEncryptionEnum>,

    ///
    /// The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS 			server. If you do not specify a transit encryption port, it will use the port selection 			strategy that the Amazon EFS mount helper uses. For more information, see EFS mount 				helper in the Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitEncryptionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EFSVolumeConfigurationTransitEncryptionEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for EFSVolumeConfigurationTransitEncryptionEnum {
    fn default() -> Self {
        EFSVolumeConfigurationTransitEncryptionEnum::Disabled
    }
}

impl cfn_resources::CfnResource for EFSVolumeConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authorization_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A list of files containing the environment variables to pass to a container. You can 			specify up to ten environment files. The file must have a .env file 			extension. Each line in an environment file should contain an environment variable in 				VARIABLE=VALUE format. Lines beginning with # are treated 			as comments and are ignored. For more information about the environment variable file 			syntax, see Declare default 				environment variables in file.
///
/// If there are environment variables specified using the environment 			parameter in a container definition, they take precedence over the variables contained 			within an environment file. If multiple environment files are specified that contain the 			same variable, they're processed from the top down. We recommend that you use unique 			variable names. For more information, see Specifying environment 				variables in the Amazon Elastic Container Service Developer Guide.
///
/// This parameter is only supported for tasks hosted on Fargate using the 			following platform versions:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EnvironmentFile {
    ///
    /// The file type to use. The only supported value is s3.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: s3
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<EnvironmentFileTypeEnum>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon S3 object containing the environment 			variable file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EnvironmentFileTypeEnum {
    /// s3
    #[serde(rename = "s3")]
    S3,
}

impl Default for EnvironmentFileTypeEnum {
    fn default() -> Self {
        EnvironmentFileTypeEnum::S3
    }
}

impl cfn_resources::CfnResource for EnvironmentFile {
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

/// The amount of ephemeral storage to allocate for the task. This parameter is used to 			expand the total amount of ephemeral storage available, beyond the default amount, for 			tasks hosted on AWS Fargate. For more information, see Fargate task 				storage in the Amazon ECS User Guide for AWS Fargate       .
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EphemeralStorage {
    ///
    /// The total amount, in GiB, of ephemeral storage to set for the task. The minimum 			supported value is 21 GiB and the maximum supported value is 				200 GiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "SizeInGiB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gi_b: Option<i64>,
}

impl cfn_resources::CfnResource for EphemeralStorage {
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

/// The FireLens configuration for the container. This is used to specify and configure a 			log router for container logs. For more information, see Custom log routing 			in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FirelensConfiguration {
    ///
    /// The options to use when configuring the log router. This field is optional and can be used to add additional  metadata, such as the task, task definition, cluster, and container instance details to the log event.
    ///
    /// If specified, valid option keys are:
    ///
    /// enable-ecs-log-metadata, which can be true or false     config-file-type, which can be s3 or file     config-file-value, which is either an S3 ARN or a file path
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,

    ///
    /// The log router to use. The valid values are fluentd or 				fluentbit.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: fluentbit | fluentd
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<FirelensConfigurationTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FirelensConfigurationTypeEnum {
    /// fluentbit
    #[serde(rename = "fluentbit")]
    Fluentbit,

    /// fluentd
    #[serde(rename = "fluentd")]
    Fluentd,
}

impl Default for FirelensConfigurationTypeEnum {
    fn default() -> Self {
        FirelensConfigurationTypeEnum::Fluentbit
    }
}

impl cfn_resources::CfnResource for FirelensConfiguration {
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

/// The HealthCheck property specifies an object representing a container health    check. Health check parameters that are specified in a container definition override any    Docker health checks that exist in the container image (such as those specified in a parent    image or from the image's Dockerfile). This configuration maps to the HEALTHCHECK    parameter of docker    run.
///
/// If a task is run manually, and not as part of a service, the task will continue its   lifecycle regardless of its health status. For tasks that are part of a service, if the   task reports as unhealthy then the task will be stopped and the service scheduler will   replace it.
///
/// The following are notes about container health check support:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheck {
    ///
    /// A string array representing the command that the container runs to determine if it is 			healthy. The string array must start with CMD to run the command arguments 			directly, or CMD-SHELL to run the command with the container's default 			shell.
    ///
    /// When you use the AWS Management Console JSON panel, the AWS Command Line Interface, or the APIs, enclose the list of 			commands in double quotes and brackets.
    ///
    /// [ "CMD-SHELL", "curl -f http://localhost/ || exit 1" ]
    ///
    /// You don't include the double quotes and brackets when you use the AWS Management Console.
    ///
    /// CMD-SHELL, curl -f http://localhost/ || exit 1
    ///
    /// An exit code of 0 indicates success, and non-zero exit code indicates failure. For 			more information, see HealthCheck in the Create a container 			section of the Docker Remote API.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    ///
    /// The time period in seconds between each health check execution. You may specify 			between 5 and 300 seconds. The default value is 30 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,

    ///
    /// The number of times to retry a failed health check before the container is considered 			unhealthy. You may specify between 1 and 10 retries. The default value is 3.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,

    ///
    /// The optional grace period to provide containers time to bootstrap before failed health 			checks count towards the maximum number of retries. You can specify between 0 and 300 			seconds. By default, the startPeriod is off.
    ///
    /// NoteIf a health check succeeds within the startPeriod, then the container 				is considered healthy and any subsequent failures count toward the maximum number of 				retries.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,

    ///
    /// The time period in seconds to wait for a health check to succeed before it is 			considered a failure. You may specify between 2 and 60 seconds. The default value is 			5.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl cfn_resources::CfnResource for HealthCheck {
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

/// The HostEntry property specifies a hostname and an IP address that are added to the   /etc/hosts file of a container through the extraHosts parameter of its   ContainerDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HostEntry {
    ///
    /// The hostname to use in the /etc/hosts entry.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<cfn_resources::StrVal>,

    ///
    /// The IP address to use in the /etc/hosts entry.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HostEntry {
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

/// The HostVolumeProperties property specifies details on a container instance bind mount host  volume.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HostVolumeProperties {
    ///
    /// When the host parameter is used, specify a sourcePath to 			declare the path on the host container instance that's presented to the container. If 			this parameter is empty, then the Docker daemon has assigned a host path for you. If the 				host parameter contains a sourcePath file location, then 			the data volume persists at the specified location on the host container instance until 			you delete it manually. If the sourcePath value doesn't exist on the host 			container instance, the Docker daemon creates it. If the location does exist, the 			contents of the source path folder are exported.
    ///
    /// If you're using the Fargate launch type, the sourcePath 			parameter is not supported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HostVolumeProperties {
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

/// Details on an Elastic Inference accelerator. For more information, see Working with Amazon Elastic Inference on 				Amazon ECS in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InferenceAccelerator {
    ///
    /// The Elastic Inference accelerator device name. The deviceName must also 			be referenced in a container definition as a ResourceRequirement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<cfn_resources::StrVal>,

    ///
    /// The Elastic Inference accelerator type to use.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InferenceAccelerator {
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

/// The KernelCapabilities property specifies the Linux capabilities for the container that are added  to or dropped from the default configuration that is provided by Docker. For more information on the default  capabilities and the non-default available capabilities, see Runtime privilege and   Linux capabilities in the Docker run reference. For more detailed information on these  Linux capabilities, see the capabilities(7) Linux manual page.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KernelCapabilities {
    ///
    /// The Linux capabilities for the container that have been added to the default 			configuration provided by Docker. This parameter maps to CapAdd in the 			Create a container section of the Docker Remote API and the 				--cap-add option to docker 				run.
    ///
    /// NoteTasks launched on AWS Fargate only support adding the SYS_PTRACE kernel 				capability.
    ///
    /// Valid values: "ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | 				"CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | 				"IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | 				"MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" 				| "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | 				"SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | 				"SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | 			"WAKE_ALARM"
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,

    ///
    /// The Linux capabilities for the container that have been removed from the default 			configuration provided by Docker. This parameter maps to CapDrop in the 			Create a container section of the Docker Remote API and the 				--cap-drop option to docker 				run.
    ///
    /// Valid values: "ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | 				"CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | 				"IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | 				"MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" 				| "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | 				"SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | 				"SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | 			"WAKE_ALARM"
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for KernelCapabilities {
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

/// A key-value pair object.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeyValuePair {
    ///
    /// The name of the key-value pair. For environment variables, this is the name of the 			environment variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The value of the key-value pair. For environment variables, this is the value of the 			environment variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for KeyValuePair {
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

/// The Linux-specific options that are applied to the container, such as Linux KernelCapabilities.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LinuxParameters {
    ///
    /// The Linux capabilities for the container that are added to or dropped from the default 			configuration provided by Docker.
    ///
    /// NoteFor tasks that use the Fargate launch type, 					capabilities is supported for all platform versions but the 					add parameter is only supported if using platform version 1.4.0 or 				later.
    ///
    /// Required: No
    ///
    /// Type: KernelCapabilities
    ///
    /// Update requires: Replacement
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KernelCapabilities>,

    ///
    /// Any host devices to expose to the container. This parameter maps to 				Devices in the Create a container section of the 			Docker Remote API and the --device option to docker run.
    ///
    /// NoteIf you're using tasks that use the Fargate launch type, the 					devices parameter isn't supported.
    ///
    /// Required: No
    ///
    /// Type: List of Device
    ///
    /// Update requires: Replacement
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    ///
    /// Run an init process inside the container that forwards signals and reaps 			processes. This parameter maps to the --init option to docker run. This parameter requires version 1.25 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,

    ///
    /// The total amount of swap memory (in MiB) a container can use. This parameter will be 			translated to the --memory-swap option to docker run where the value would be the sum of 			the container memory plus the maxSwap value.
    ///
    /// If a maxSwap value of 0 is specified, the container will not 			use swap. Accepted values are 0 or any positive integer. If the 				maxSwap parameter is omitted, the container will use the swap 			configuration for the container instance it is running on. A maxSwap value 			must be set for the swappiness parameter to be used.
    ///
    /// NoteIf you're using tasks that use the Fargate launch type, the 					maxSwap parameter isn't supported.If you're using tasks on Amazon Linux 2023 the swappiness parameter isn't supported.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxSwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i64>,

    ///
    /// The value for the size (in MiB) of the /dev/shm volume. This parameter 			maps to the --shm-size option to docker 				run.
    ///
    /// NoteIf you are using tasks that use the Fargate launch type, the 					sharedMemorySize parameter is not supported.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "SharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,

    ///
    /// This allows you to tune a container's memory swappiness behavior. A 				swappiness value of 0 will cause swapping to not happen 			unless absolutely necessary. A swappiness value of 100 will 			cause pages to be swapped very aggressively. Accepted values are whole numbers between 				0 and 100. If the swappiness parameter is not 			specified, a default value of 60 is used. If a value is not specified for 				maxSwap then this parameter is ignored. This parameter maps to the 				--memory-swappiness option to docker run.
    ///
    /// NoteIf you're using tasks that use the Fargate launch type, the 					swappiness parameter isn't supported.If you're using tasks on Amazon Linux 2023 the swappiness parameter isn't supported.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,

    ///
    /// The container path, mount options, and size (in MiB) of the tmpfs mount. This 			parameter maps to the --tmpfs option to docker run.
    ///
    /// NoteIf you're using tasks that use the Fargate launch type, the 					tmpfs parameter isn't supported.
    ///
    /// Required: No
    ///
    /// Type: List of Tmpfs
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

impl cfn_resources::CfnResource for LinuxParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.capabilities
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The LogConfiguration property specifies log configuration options to send to a custom log driver  for the container.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogConfiguration {
    ///
    /// The log driver to use for the container.
    ///
    /// For tasks on AWS Fargate, the supported log drivers are awslogs, 				splunk, and awsfirelens.
    ///
    /// For tasks hosted on Amazon EC2 instances, the supported log drivers are 				awslogs, fluentd, gelf, 				json-file, journald, 				logentries,syslog, splunk, and 				awsfirelens.
    ///
    /// For more information about using the awslogs log driver, see Using 				the awslogs log driver in the Amazon Elastic Container Service Developer Guide.
    ///
    /// For more information about using the awsfirelens log driver, see Custom log routing in the Amazon Elastic Container Service Developer Guide.
    ///
    /// NoteIf you have a custom driver that isn't listed, you can fork the Amazon ECS container 				agent project that's available 					on GitHub and customize it to work with that driver. We encourage you to 				submit pull requests for changes that you would like to have included. However, we 				don't currently provide support for running modified copies of this software.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: awsfirelens | awslogs | fluentd | gelf | journald | json-file | splunk | syslog
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDriver")]
    pub log_driver: LogConfigurationLogDriverEnum,

    ///
    /// The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version --format '{{.Server.APIVersion}}'
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,

    ///
    /// The secrets to pass to the log configuration. For more information, see Specifying 				sensitive data in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Secret
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LogConfigurationLogDriverEnum {
    /// awsfirelens
    #[serde(rename = "awsfirelens")]
    Awsfirelens,

    /// awslogs
    #[serde(rename = "awslogs")]
    Awslogs,

    /// fluentd
    #[serde(rename = "fluentd")]
    Fluentd,

    /// gelf
    #[serde(rename = "gelf")]
    Gelf,

    /// journald
    #[serde(rename = "journald")]
    Journald,

    /// json-file
    #[serde(rename = "json-file")]
    Jsonfile,

    /// splunk
    #[serde(rename = "splunk")]
    Splunk,

    /// syslog
    #[serde(rename = "syslog")]
    Syslog,
}

impl Default for LogConfigurationLogDriverEnum {
    fn default() -> Self {
        LogConfigurationLogDriverEnum::Awsfirelens
    }
}

impl cfn_resources::CfnResource for LogConfiguration {
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

/// The details for a volume mount point that's used in a container definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MountPoint {
    ///
    /// The path on the container to mount the host volume at.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<cfn_resources::StrVal>,

    ///
    /// If this value is true, the container has read-only access to the volume. 			If this value is false, then the container can write to the volume. The 			default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    ///
    /// The name of the volume to mount. Must be a volume name referenced in the 				name parameter of task definition volume.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MountPoint {
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

/// The PortMapping property specifies a port mapping. Port mappings allow containers to access ports  on the host container instance to send or receive traffic. Port mappings are specified as part of the container  definition.
///
/// If you are using containers in a task with the awsvpc or host network mode, exposed  ports should be specified using containerPort. The hostPort can be left blank or it must be  the same value as the containerPort.
///
/// After a task reaches the RUNNING status, manual and automatic host and container port assignments  are visible in the networkBindings section of DescribeTasks API responses.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortMapping {
    ///
    /// The application protocol that's used for the port mapping. This parameter only applies 			to Service Connect. We recommend that you set this parameter to be consistent with the 			protocol that your application uses. If you set this parameter, Amazon ECS adds 			protocol-specific connection handling to the Service Connect proxy. If you set this 			parameter, Amazon ECS adds protocol-specific telemetry in the Amazon ECS console and CloudWatch.
    ///
    /// If you don't set a value for this parameter, then TCP is used. However, Amazon ECS doesn't 			add protocol-specific telemetry for TCP.
    ///
    /// Tasks that run in a namespace can use short names to connect 	to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. 	Tasks connect through a managed proxy container 	that collects logs and metrics for increased visibility. 	Only the tasks that Amazon ECS services create are supported with Service Connect. 	For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: grpc | http | http2
    ///
    /// Update requires: Replacement
    #[serde(rename = "AppProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<PortMappingAppProtocolEnum>,

    ///
    /// The port number on the container that's bound to the user-specified or automatically 			assigned host port.
    ///
    /// If you use containers in a task with the awsvpc or host 			network mode, specify the exposed ports using containerPort.
    ///
    /// If you use containers in a task with the bridge network mode and you 			specify a container port and not a host port, your container automatically receives a 			host port in the ephemeral port range. For more information, see hostPort. 			Port mappings that are automatically assigned in this way do not count toward the 100 			reserved ports limit of a container instance.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,

    ///
    /// The port number range on the container that's bound to the dynamically mapped host port 			range.
    ///
    /// The following rules apply when you specify a containerPortRange:
    ///
    /// You must use either the bridge network mode or the awsvpc 					network mode.               This parameter is available for both the EC2 and AWS Fargate launch types.               This parameter is available for both the Linux and Windows operating systems.               The container instance must have at least version 1.67.0 of the container agent 					and at least version 1.67.0-1 of the ecs-init package               You can specify a maximum of 100 port ranges per container.               You do not specify a hostPortRange. The value of the hostPortRange is set 					as follows:                                               For containers in a task with the awsvpc network mode, 							the hostPort is set to the same value as the 								containerPort. This is a static mapping 							strategy.                     For containers in a task with the bridge network mode, the Amazon ECS agent finds open host ports from the default ephemeral range and passes it to docker to bind them to the container ports.                        The containerPortRange valid values are between 1 and 					65535.               A port can only be included in one port mapping per container.               You cannot specify overlapping port ranges.               The first port in the range must be less than last port in the range.               Docker recommends that you turn off the docker-proxy in the Docker daemon config file when you have a large number of ports.        For more information, see Issue #11185 on the Github website.        For information about how to turn off the docker-proxy in the Docker daemon config file, see Docker daemon in the Amazon ECS Developer Guide.
    ///
    /// You can call DescribeTasks to view the hostPortRange which 			are the host ports that are bound to the container ports.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port_range: Option<cfn_resources::StrVal>,

    ///
    /// The port number on the container instance to reserve for your container.
    ///
    /// If you specify a containerPortRange, leave this field empty and the value of 			the hostPort is set as follows:
    ///
    /// For containers in a task with the awsvpc network mode, the 						hostPort is set to the same value as the 						containerPort. This is a static mapping strategy.               For containers in a task with the bridge network mode, the Amazon ECS agent finds 					open ports on the host and automatically binds them to the container ports. This 					is a dynamic mapping strategy.
    ///
    /// If you use containers in a task with the awsvpc or host 			network mode, the hostPort can either be left blank or set to the same 			value as the containerPort.
    ///
    /// If you use containers in a task with the bridge network mode, you can 			specify a non-reserved host port for your container port mapping, or you can omit the 				hostPort (or set it to 0) while specifying a 				containerPort and your container automatically receives a port in the 			ephemeral port range for your container instance operating system and Docker 			version.
    ///
    /// The default ephemeral port range for Docker version 1.6.0 and later is listed on the 			instance under /proc/sys/net/ipv4/ip_local_port_range. If this kernel 			parameter is unavailable, the default ephemeral port range from 49153 through 65535 is 			used. Do not attempt to specify a host port in the ephemeral port range as these are 			reserved for automatic assignment. In general, ports below 32768 are outside of the 			ephemeral port range.
    ///
    /// The default reserved ports are 22 for SSH, the Docker ports 2375 and 2376, and the 			Amazon ECS container agent ports 51678-51680. Any host port that was previously specified in 			a running task is also reserved while the task is running. That is, after a task stops, 			the host port is released. The current reserved ports are displayed in the 			remainingResources of DescribeContainerInstances 			output. A container instance can have up to 100 reserved ports at a time. This number 			includes the default reserved ports. Automatically assigned ports aren't included in the 			100 reserved ports quota.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,

    ///
    /// The name that's used for the port mapping. This parameter only applies to 			Service Connect. This parameter is the name that you use in the 				serviceConnectConfiguration of a service. The name can include up to 64 			characters. The characters can include lowercase letters, numbers, underscores (_), and 			hyphens (-). The name can't start with a hyphen.
    ///
    /// For more information, see Service Connect in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The protocol used for the port mapping. Valid values are tcp and 				udp. The default is tcp.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: tcp | udp
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<PortMappingProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PortMappingAppProtocolEnum {
    /// grpc
    #[serde(rename = "grpc")]
    Grpc,

    /// http
    #[serde(rename = "http")]
    Http,

    /// http2
    #[serde(rename = "http2")]
    Http2,
}

impl Default for PortMappingAppProtocolEnum {
    fn default() -> Self {
        PortMappingAppProtocolEnum::Grpc
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PortMappingProtocolEnum {
    /// tcp
    #[serde(rename = "tcp")]
    Tcp,

    /// udp
    #[serde(rename = "udp")]
    Udp,
}

impl Default for PortMappingProtocolEnum {
    fn default() -> Self {
        PortMappingProtocolEnum::Tcp
    }
}

impl cfn_resources::CfnResource for PortMapping {
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

/// The configuration details for the App Mesh proxy.
///
/// For tasks that use the EC2 launch type, the container instances require 			at least version 1.26.0 of the container agent and at least version 1.26.0-1 of the 				ecs-init package to use a proxy configuration. If your container 			instances are launched from the Amazon ECS optimized AMI version 20190301 or 			later, then they contain the required versions of the container agent and 				ecs-init. For more information, see Amazon ECS-optimized Linux AMI
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProxyConfiguration {
    ///
    /// The name of the container that will serve as the App Mesh proxy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerName")]
    pub container_name: cfn_resources::StrVal,

    ///
    /// The set of network configuration parameters to provide the Container Network Interface 			(CNI) plugin, specified as key-value pairs.
    ///
    /// IgnoredUID - (Required) The user ID (UID) of the proxy 					container as defined by the user parameter in a container 					definition. This is used to ensure the proxy ignores its own traffic. If 						IgnoredGID is specified, this field can be empty.                        IgnoredGID - (Required) The group ID (GID) of the proxy 					container as defined by the user parameter in a container 					definition. This is used to ensure the proxy ignores its own traffic. If 						IgnoredUID is specified, this field can be empty.                        AppPorts - (Required) The list of ports that the 					application uses. Network traffic to these ports is forwarded to the 						ProxyIngressPort and ProxyEgressPort.                        ProxyIngressPort - (Required) Specifies the port that 					incoming traffic to the AppPorts is directed to.                        ProxyEgressPort - (Required) Specifies the port that 					outgoing traffic from the AppPorts is directed to.                        EgressIgnoredPorts - (Required) The egress traffic going to 					the specified ports is ignored and not redirected to the 						ProxyEgressPort. It can be an empty list.                        EgressIgnoredIPs - (Required) The egress traffic going to 					the specified IP addresses is ignored and not redirected to the 						ProxyEgressPort. It can be an empty list.
    ///
    /// Required: No
    ///
    /// Type: List of KeyValuePair
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProxyConfigurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_properties: Option<Vec<KeyValuePair>>,

    ///
    /// The proxy type. The only supported value is APPMESH.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: APPMESH
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<ProxyConfigurationTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ProxyConfigurationTypeEnum {
    /// APPMESH
    #[serde(rename = "APPMESH")]
    Appmesh,
}

impl Default for ProxyConfigurationTypeEnum {
    fn default() -> Self {
        ProxyConfigurationTypeEnum::Appmesh
    }
}

impl cfn_resources::CfnResource for ProxyConfiguration {
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

/// The repository credentials for private registry authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RepositoryCredentials {
    ///
    /// The Amazon Resource Name (ARN) of the secret containing the private repository 			credentials.
    ///
    /// NoteWhen you use the Amazon ECS API, AWS CLI, or AWS SDK, if the secret exists in the same 				Region as the task that you're launching then you can use either the full ARN or 				the name of the secret. When you use the AWS Management Console, you must specify the full ARN 				of the secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CredentialsParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_parameter: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RepositoryCredentials {
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

/// The type and amount of a resource to assign to a container. The supported resource types are 			GPUs and Elastic Inference accelerators. For more information, see Working with 				GPUs on Amazon ECS or Working with 				Amazon Elastic Inference on Amazon ECS in the Amazon Elastic Container Service Developer Guide
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceRequirement {
    ///
    /// The type of resource to assign to a container. The supported values are 				GPU or InferenceAccelerator.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GPU | InferenceAccelerator
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ResourceRequirementTypeEnum,

    ///
    /// The value for the specified resource type.
    ///
    /// If the GPU type is used, the value is the number of physical 				GPUs the Amazon ECS container agent reserves for the container. The number 			of GPUs that's reserved for all containers in a task can't exceed the number of 			available GPUs on the container instance that the task is launched on.
    ///
    /// If the InferenceAccelerator type is used, the value matches 			the deviceName for an InferenceAccelerator specified in a 			task definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceRequirementTypeEnum {
    /// GPU
    #[serde(rename = "GPU")]
    Gpu,

    /// InferenceAccelerator
    #[serde(rename = "InferenceAccelerator")]
    Inferenceaccelerator,
}

impl Default for ResourceRequirementTypeEnum {
    fn default() -> Self {
        ResourceRequirementTypeEnum::Gpu
    }
}

impl cfn_resources::CfnResource for ResourceRequirement {
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

/// Information about the platform for the Amazon ECS service or task.
///
/// For more information about RuntimePlatform, see RuntimePlatform in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuntimePlatform {
    ///
    /// The CPU architecture.
    ///
    /// You can run your Linux tasks on an ARM-based platform by setting the value to 				ARM64. This option is available for tasks that run on Linux Amazon EC2 			instance or Linux containers on Fargate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ARM64 | X86_64
    ///
    /// Update requires: Replacement
    #[serde(rename = "CpuArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_architecture: Option<RuntimePlatformCpuArchitectureEnum>,

    ///
    /// The operating system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LINUX | WINDOWS_SERVER_2004_CORE | WINDOWS_SERVER_2016_FULL | WINDOWS_SERVER_2019_CORE | WINDOWS_SERVER_2019_FULL | WINDOWS_SERVER_2022_CORE | WINDOWS_SERVER_2022_FULL | WINDOWS_SERVER_20H2_CORE
    ///
    /// Update requires: Replacement
    #[serde(rename = "OperatingSystemFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_family: Option<RuntimePlatformOperatingSystemFamilyEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RuntimePlatformCpuArchitectureEnum {
    /// ARM64
    #[serde(rename = "ARM64")]
    Arm64,

    /// X86_64
    #[serde(rename = "X86_64")]
    X8664,
}

impl Default for RuntimePlatformCpuArchitectureEnum {
    fn default() -> Self {
        RuntimePlatformCpuArchitectureEnum::Arm64
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RuntimePlatformOperatingSystemFamilyEnum {
    /// LINUX
    #[serde(rename = "LINUX")]
    Linux,

    /// WINDOWS_SERVER_2004_CORE
    #[serde(rename = "WINDOWS_SERVER_2004_CORE")]
    Windowsserver2004core,

    /// WINDOWS_SERVER_2016_FULL
    #[serde(rename = "WINDOWS_SERVER_2016_FULL")]
    Windowsserver2016full,

    /// WINDOWS_SERVER_2019_CORE
    #[serde(rename = "WINDOWS_SERVER_2019_CORE")]
    Windowsserver2019core,

    /// WINDOWS_SERVER_2019_FULL
    #[serde(rename = "WINDOWS_SERVER_2019_FULL")]
    Windowsserver2019full,

    /// WINDOWS_SERVER_2022_CORE
    #[serde(rename = "WINDOWS_SERVER_2022_CORE")]
    Windowsserver2022core,

    /// WINDOWS_SERVER_2022_FULL
    #[serde(rename = "WINDOWS_SERVER_2022_FULL")]
    Windowsserver2022full,

    /// WINDOWS_SERVER_20H2_CORE
    #[serde(rename = "WINDOWS_SERVER_20H2_CORE")]
    Windowsserver20h2core,
}

impl Default for RuntimePlatformOperatingSystemFamilyEnum {
    fn default() -> Self {
        RuntimePlatformOperatingSystemFamilyEnum::Linux
    }
}

impl cfn_resources::CfnResource for RuntimePlatform {
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

/// An object representing the secret to expose to your container. Secrets can be exposed 			to a container in the following ways:
///
/// For more information, see Specifying 				sensitive data in the Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Secret {
    ///
    /// The name of the secret.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The secret to expose to the container. The supported values are either the full ARN 			of the AWS Secrets Manager secret or the full ARN of the parameter in the SSM Parameter 			Store.
    ///
    /// For information about the require AWS Identity and Access Management permissions, see Required IAM permissions for Amazon ECS secrets (for Secrets Manager) or Required IAM permissions for Amazon ECS secrets (for Systems Manager Parameter 			store) in the Amazon Elastic Container Service Developer Guide.
    ///
    /// NoteIf the SSM Parameter Store parameter exists in the same Region as the task 				you're launching, then you can use either the full ARN or name of the parameter. 				If the parameter exists in a different Region, then the full ARN must be 				specified.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValueFrom")]
    pub value_from: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Secret {
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

/// A list of namespaced kernel parameters to set in the container. This parameter maps to 				Sysctls in the Create a container section of the 			Docker Remote API and the --sysctl option to docker run.
///
/// We don't recommend that you specify network-related systemControls 			parameters for multiple containers in a single task. This task also uses either the 				awsvpc or host network mode. It does it for the following 			reasons.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SystemControl {
    ///
    /// The namespaced kernel parameter to set a value for.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,

    ///
    /// The value for the namespaced kernel parameter that's specified in 				namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SystemControl {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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

/// The constraint on task placement in the task definition. For more 			information, see Task placement constraints in the 			Amazon Elastic Container Service Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TaskDefinitionPlacementConstraint {
    ///
    /// A cluster query language expression to apply to the constraint. For more information, 			see Cluster query language in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<cfn_resources::StrVal>,

    ///
    /// The type of constraint. The MemberOf constraint restricts selection to be 			from a group of valid candidates.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: memberOf
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: TaskDefinitionPlacementConstraintTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskDefinitionPlacementConstraintTypeEnum {
    /// memberOf
    #[serde(rename = "memberOf")]
    Memberof,
}

impl Default for TaskDefinitionPlacementConstraintTypeEnum {
    fn default() -> Self {
        TaskDefinitionPlacementConstraintTypeEnum::Memberof
    }
}

impl cfn_resources::CfnResource for TaskDefinitionPlacementConstraint {
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

/// The container path, mount options, and size of the tmpfs mount.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tmpfs {
    ///
    /// The absolute file path where the tmpfs volume is to be mounted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<cfn_resources::StrVal>,

    ///
    /// The list of tmpfs volume mount options.
    ///
    /// Valid values: "defaults" | "ro" | "rw" | "suid" | "nosuid" | "dev" | "nodev" | 				"exec" | "noexec" | "sync" | "async" | "dirsync" | "remount" | "mand" | "nomand" | 				"atime" | "noatime" | "diratime" | "nodiratime" | "bind" | "rbind" | "unbindable" | 				"runbindable" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | 				"relatime" | "norelatime" | "strictatime" | "nostrictatime" | "mode" | "uid" | "gid" 				| "nr_inodes" | "nr_blocks" | "mpol"
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,

    ///
    /// The maximum size (in MiB) of the tmpfs volume.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Size")]
    pub size: i64,
}

impl cfn_resources::CfnResource for Tmpfs {
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

/// The ulimit settings to pass to the container.
///
/// Amazon ECS tasks hosted on AWS Fargate use the default 							resource limit values set by the operating system with the exception of 							the nofile resource limit parameter which AWS Fargate 							overrides. The nofile resource limit sets a restriction on 							the number of open files that a container can use. The default 								nofile soft limit is 1024 and the default hard limit 							is 4096.
///
/// You can specify the ulimit settings for a container in a task 			definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ulimit {
    ///
    /// The hard limit for the ulimit type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "HardLimit")]
    pub hard_limit: i64,

    ///
    /// The type of the ulimit.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: core | cpu | data | fsize | locks | memlock | msgqueue | nice | nofile | nproc | rss | rtprio | rttime | sigpending | stack
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: UlimitNameEnum,

    ///
    /// The soft limit for the ulimit type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "SoftLimit")]
    pub soft_limit: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum UlimitNameEnum {
    /// core
    #[serde(rename = "core")]
    Core,

    /// cpu
    #[serde(rename = "cpu")]
    Cpu,

    /// data
    #[serde(rename = "data")]
    Data,

    /// fsize
    #[serde(rename = "fsize")]
    Fsize,

    /// locks
    #[serde(rename = "locks")]
    Locks,

    /// memlock
    #[serde(rename = "memlock")]
    Memlock,

    /// msgqueue
    #[serde(rename = "msgqueue")]
    Msgqueue,

    /// nice
    #[serde(rename = "nice")]
    Nice,

    /// nofile
    #[serde(rename = "nofile")]
    Nofile,

    /// nproc
    #[serde(rename = "nproc")]
    Nproc,

    /// rss
    #[serde(rename = "rss")]
    Rss,

    /// rtprio
    #[serde(rename = "rtprio")]
    Rtprio,

    /// rttime
    #[serde(rename = "rttime")]
    Rttime,

    /// sigpending
    #[serde(rename = "sigpending")]
    Sigpending,

    /// stack
    #[serde(rename = "stack")]
    Stack,
}

impl Default for UlimitNameEnum {
    fn default() -> Self {
        UlimitNameEnum::Core
    }
}

impl cfn_resources::CfnResource for Ulimit {
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

/// The Volume property specifies a data volume used in a task definition. For tasks that use a Docker  volume, specify a DockerVolumeConfiguration. For tasks that use a bind mount host volume, specify a   host and optional sourcePath. For more information about host and optional   sourcePath, see Volumes and Using Data Volumes in   Tasks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Volume {
    ///
    /// This parameter is specified when you use Docker volumes.
    ///
    /// Windows containers only support the use of the local driver. To use bind 			mounts, specify the host parameter instead.
    ///
    /// NoteDocker volumes aren't supported by tasks run on AWS Fargate.
    ///
    /// Required: No
    ///
    /// Type: DockerVolumeConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "DockerVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,

    ///
    /// This parameter is specified when you use an Amazon Elastic File System file system for task 			storage.
    ///
    /// Required: No
    ///
    /// Type: EFSVolumeConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EFSVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efsvolume_configuration: Option<EFSVolumeConfiguration>,

    ///
    /// This parameter is specified when you use bind mount host volumes. The contents of the 				host parameter determine whether your bind mount host volume persists 			on the host container instance and where it's stored. If the host parameter 			is empty, then the Docker daemon assigns a host path for your data volume. However, the 			data isn't guaranteed to persist after the containers that are associated with it stop 			running.
    ///
    /// Windows containers can mount whole directories on the same drive as 				$env:ProgramData. Windows containers can't mount directories on a 			different drive, and mount point can't be across drives. For example, you can mount 				C:\my\path:C:\my\path and D:\:D:\, but not 				D:\my\path:C:\my\path or D:\:C:\my\path.
    ///
    /// Required: No
    ///
    /// Type: HostVolumeProperties
    ///
    /// Update requires: Replacement
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostVolumeProperties>,

    ///
    /// The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, underscores, and hyphens are allowed. This name is referenced in the 				sourceVolume parameter of container definition 			mountPoints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Volume {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.docker_volume_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.efsvolume_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.host.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Details on a data volume from another container in the same task definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VolumeFrom {
    ///
    /// If this value is true, the container has read-only access to the volume. 			If this value is false, then the container can write to the volume. The 			default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    ///
    /// The name of another container within the same task definition to mount volumes 			from.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for VolumeFrom {
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
