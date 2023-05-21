/// The AWS::Batch::JobDefinition resource specifies the parameters for an AWS Batch job  definition. For more information, see Job Definitions in the AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnJobDefinition {
    ///
    /// An object with various properties specific to Amazon ECS based jobs. Valid values are   containerProperties, eksProperties, and nodeProperties.  Only one can be specified.
    ///
    /// Required: No
    ///
    /// Type: ContainerProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,

    ///
    /// An object with various properties that are specific to Amazon EKS based jobs. Valid values are   containerProperties, eksProperties, and nodeProperties.  Only one can be specified.
    ///
    /// Required: No
    ///
    /// Type: EksProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "EksProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties: Option<EksProperties>,

    ///
    /// The name of the job definition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,

    ///
    /// An object with various properties that are specific to multi-node parallel jobs. Valid  values are containerProperties, eksProperties, and   nodeProperties. Only one can be specified.
    ///
    /// NoteIf the job runs on Fargate resources, don't specify nodeProperties. Use   containerProperties instead.
    ///
    /// Required: No
    ///
    /// Type: NodeProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,

    ///
    /// Default parameters or parameter substitution placeholders that are set in the job  definition. Parameters are specified as a key-value pair mapping. Parameters in a   SubmitJob request override any corresponding parameter defaults from the job  definition. For more information about specifying parameters, see Job definition parameters in the           AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    ///
    /// The platform capabilities required by the job definition. If no value is specified, it  defaults to EC2. Jobs run on Fargate resources specify  FARGATE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<String>>,

    ///
    /// Specifies whether to propagate the tags from the job or job definition to the corresponding  Amazon ECS task. If no value is specified, the tags aren't propagated. Tags can only be propagated to  the tasks when the tasks are created. For tags with the same name, job tags are given priority  over job definitions tags. If the total number of combined tags from the job and job definition  is over 50, the job is moved to the FAILED state.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,

    ///
    /// The retry strategy to use for failed jobs that are submitted with this job  definition.
    ///
    /// Required: No
    ///
    /// Type: RetryStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,

    ///
    /// The scheduling priority of the job definition. This only affects jobs in job queues with a  fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower  scheduling priority.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchedulingPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i64>,

    ///
    /// The tags that are applied to the job definition.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// The timeout time for jobs that are submitted with this job definition. After the amount of  time you specify passes, AWS Batch terminates your jobs if they aren't finished.
    ///
    /// Required: No
    ///
    /// Type: Timeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timeout>,

    ///
    /// The type of job definition. For more information about multi-node parallel jobs, see Creating a multi-node parallel job definition in the           AWS Batch User Guide.
    ///
    /// NoteIf the job is run on Fargate resources, then multinode isn't supported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: container | multinode
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: JobDefinitionTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JobDefinitionTypeEnum {
    /// container
    #[serde(rename = "container")]
    Container,

    /// multinode
    #[serde(rename = "multinode")]
    Multinode,
}

impl Default for JobDefinitionTypeEnum {
    fn default() -> Self {
        JobDefinitionTypeEnum::Container
    }
}

impl cfn_resources::CfnResource for CfnJobDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Batch::JobDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.eks_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.node_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.retry_strategy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timeout.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The authorization configuration details for the Amazon EFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthorizationConfig {
    ///
    /// The Amazon EFS access point ID to use. If an access point is specified, the root directory value  specified in the EFSVolumeConfiguration must either be omitted or set to   / which enforces the path set on the EFS access point. If an access point is used,  transit encryption must be enabled in the EFSVolumeConfiguration. For more  information, see Working   with Amazon EFS access points in the Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,

    ///
    /// Whether or not to use the AWS Batch job IAM role defined in a job definition when mounting the  Amazon EFS file system. If enabled, transit encryption must be enabled in the   EFSVolumeConfiguration. If this parameter is omitted, the default value of   DISABLED is used. For more information, see Using Amazon EFS access points in  the         AWS Batch User Guide. EFS IAM authorization requires that   TransitEncryption be ENABLED and that a JobRoleArn is  specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<AuthorizationConfigIamEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AuthorizationConfigIamEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for AuthorizationConfigIamEnum {
    fn default() -> Self {
        AuthorizationConfigIamEnum::Disabled
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

/// Container properties are used  for  Amazon ECS based job definitions. These properties to describe the container that's  launched as part of a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerProperties {
    ///
    /// The command that's passed to the container. This parameter maps to Cmd in the  Create a container section of the Docker Remote API and the COMMAND  parameter to docker run. For more information, see   https://docs.docker.com/engine/reference/builder/#cmd.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    ///
    /// The environment variables to pass to a container. This parameter maps to Env in  the Create a container section of the Docker Remote API and the   --env option to docker run.
    ///
    /// ImportantWe don't recommend using plaintext environment variables for sensitive information, such as   credential data.
    ///
    /// NoteEnvironment variables cannot start with "AWS_BATCH". This naming convention is reserved for variables that AWS Batch sets.
    ///
    /// Required: No
    ///
    /// Type: List of Environment
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<Environment>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: EphemeralStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "EphemeralStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,

    ///
    /// The Amazon Resource Name (ARN) of the execution role that AWS Batch can assume. For jobs that run on Fargate  resources, you must provide an execution role. For more information, see AWS Batch execution IAM role  in the         AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,

    ///
    /// The platform configuration for jobs that are running on Fargate resources. Jobs that are  running on EC2 resources must not specify this parameter.
    ///
    /// Required: No
    ///
    /// Type: FargatePlatformConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FargatePlatformConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,

    ///
    /// The image used to start a container. This string is passed directly to the Docker daemon.  Images in the Docker Hub registry are available by default. Other repositories are specified with           repository-url/image:tag       .  It can be 255 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), underscores (_), colons (:), periods (.), forward slashes (/), and number signs (#). This parameter maps to Image in the  Create a container section of the Docker Remote API and the IMAGE  parameter of docker run.
    ///
    /// NoteDocker image architecture must match the processor architecture of the compute resources   that they're scheduled on. For example, ARM-based Docker images can only run on ARM-based   compute resources.
    ///
    /// Images in Amazon ECR Public repositories use the full registry/repository[:tag] or    registry/repository[@digest] naming conventions. For example,    public.ecr.aws/registry_alias/my-web-app:latest          .               Images in Amazon ECR repositories use the full registry and repository URI (for example,    123456789012.dkr.ecr.<region-name>.amazonaws.com/<repository-name>).               Images in official repositories on Docker Hub use a single name (for example,    ubuntu or mongo).               Images in other repositories on Docker Hub are qualified with an organization name (for   example, amazon/amazon-ecs-agent).               Images in other online repositories are qualified further by a domain name (for example,    quay.io/assemblyline/ubuntu).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: String,

    ///
    /// The instance type to use for a multi-node parallel job. All node groups in a multi-node  parallel job must use the same instance type.
    ///
    /// NoteThis parameter isn't applicable to single-node container jobs or jobs that run on Fargate   resources, and shouldn't be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that the container can assume for AWS permissions. For more  information, see IAM roles for tasks in the   Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,

    ///
    /// Linux-specific modifications that are applied to the container, such as details for device  mappings.
    ///
    /// Required: No
    ///
    /// Type: LinuxParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "LinuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,

    ///
    /// The log configuration specification for the container.
    ///
    /// This parameter maps to LogConfig in the Create a container  section of the Docker Remote API and the --log-driver option to docker run. By default, containers use the same logging  driver that the Docker daemon uses. However the container might use a different logging driver  than the Docker daemon by specifying a log driver with this parameter in the container  definition. To use a different logging driver for a container, the log system must be configured  properly on the container instance (or on a different log server for remote logging options). For  more information on the options for different supported log drivers, see Configure logging drivers  in the Docker documentation.
    ///
    /// Note        AWS Batch currently supports a subset of the logging drivers available to the Docker daemon   (shown in the LogConfiguration data type).
    ///
    /// This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version | grep "Server API version"
    ///
    /// NoteThe Amazon ECS container agent running on a container instance must register the logging drivers   available on that instance with the ECS_AVAILABLE_LOGGING_DRIVERS environment   variable before containers placed on that instance can use these log configuration options. For   more information, see Amazon ECS container agent   configuration in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: LogConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,

    ///
    /// This parameter is deprecated, use resourceRequirements to specify the memory  requirements for the job definition. It's not supported for jobs running on Fargate resources.  For jobs that run on EC2 resources, it specifies the memory hard limit (in MiB) for a container.  If your container attempts to exceed the specified number, it's terminated. You must specify at  least 4 MiB of memory for a job using this parameter. The memory hard limit can be specified in  several places. It must be specified for each node at least once.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,

    ///
    /// The mount points for data volumes in your container. This parameter maps to   Volumes in the Create a container section of the Docker Remote API  and the --volume option to docker  run.
    ///
    /// Required: No
    ///
    /// Type: List of MountPoints
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoints>>,

    ///
    /// The network configuration for jobs that are running on Fargate resources. Jobs that are  running on EC2 resources must not specify this parameter.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    ///
    /// When this parameter is true, the container is given elevated permissions on the host  container instance (similar to the root user). This parameter maps to   Privileged in the Create a container section of the  Docker Remote API and the --privileged option to docker run. The default value is false.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources and   shouldn't be provided, or specified as false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    ///
    /// When this parameter is true, the container is given read-only access to its root file  system. This parameter maps to ReadonlyRootfs in the  Create a container section of the Docker Remote API and the   --read-only option to docker run.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,

    ///
    /// The type and amount of resources to assign to a container. The supported resources include   GPU, MEMORY, and VCPU.
    ///
    /// Required: No
    ///
    /// Type: List of ResourceRequirement
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,

    ///
    /// The secrets for the container. For more information, see Specifying sensitive data in the           AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Secret
    ///
    /// Update requires: No interruption
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,

    ///
    /// A list of ulimits to set in the container. This parameter maps to   Ulimits in the Create a container section of the Docker Remote API  and the --ulimit option to docker  run.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources and   shouldn't be provided.
    ///
    /// Required: No
    ///
    /// Type: List of Ulimit
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,

    ///
    /// The user name to use inside the container. This parameter maps to User in the  Create a container section of the Docker Remote API and the --user  option to docker run.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    ///
    /// This parameter is deprecated, use resourceRequirements to specify the vCPU  requirements for the job definition. It's not supported for jobs running on Fargate resources.  For jobs running on EC2 resources, it specifies the number of vCPUs reserved for the job.
    ///
    /// Each vCPU is equivalent to 1,024 CPU shares. This parameter maps to CpuShares  in the Create a container section of the Docker Remote API and the   --cpu-shares option to docker run. The  number of vCPUs must be specified but can be specified in several places. You must specify it at  least once for each node.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vcpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i64>,

    ///
    /// A list of data volumes used in a job.
    ///
    /// Required: No
    ///
    /// Type: List of Volumes
    ///
    /// Update requires: No interruption
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volumes>>,
}

impl cfn_resources::CfnResource for ContainerProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ephemeral_storage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.fargate_platform_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.linux_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.log_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a container instance host device.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Device {
    ///
    /// The path inside the container that's used to expose the host device. By default, the   hostPath value is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,

    ///
    /// The path for the device on the host container instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<String>,

    ///
    /// The explicit permissions to provide to the container for the device. By default, the  container has permissions for read, write, and mknod for  the device.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
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

/// This is used when you're using an Amazon Elastic File System file system for job storage. For more  information, see Amazon EFS   Volumes in the         AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EfsVolumeConfiguration {
    ///
    /// The authorization configuration details for the Amazon EFS file system.
    ///
    /// Required: No
    ///
    /// Type: AuthorizationConfig
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,

    ///
    /// The directory within the Amazon EFS file system to mount as the root directory inside the host.  If this parameter is omitted, the root of the Amazon EFS volume is used instead. Specifying   / has the same effect as omitting this parameter. The maximum length is 4,096  characters.
    ///
    /// ImportantIf an EFS access point is specified in the authorizationConfig, the root   directory parameter must either be omitted or set to /, which enforces the path set   on the Amazon EFS access point.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,

    ///
    /// Determines whether to enable encryption for Amazon EFS data in transit between the Amazon ECS host and  the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If  this parameter is omitted, the default value of DISABLED is used. For more  information, see Encrypting data in transit in the Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,

    ///
    /// The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If  you don't specify a transit encryption port, it uses the port selection strategy that the Amazon EFS  mount helper uses. The value must be between 0 and 65,535. For more information, see EFS mount helper in the   Amazon Elastic File System User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitEncryptionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i64>,
}

impl cfn_resources::CfnResource for EfsVolumeConfiguration {
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

/// EKS container properties are used in job definitions for Amazon EKS based job definitions to  describe the properties for a container node in the pod that's launched as part of a job. This  can't be specified for Amazon ECS based job definitions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksContainer {
    ///
    /// An array of arguments to the entrypoint. If this isn't specified, the CMD of  the container image is used. This corresponds to the args member in the Entrypoint portion of the Pod  in Kubernetes. Environment variable references are expanded using the container's environment.
    ///
    /// If the referenced environment variable doesn't exist, the reference in the command isn't  changed. For example, if the reference is to "$(NAME1)" and the NAME1  environment variable doesn't exist, the command string will remain "$(NAME1)."   $$ is replaced with $, and the resulting string isn't expanded. For  example, $$(VAR_NAME) is passed as $(VAR_NAME) whether or not the   VAR_NAME environment variable exists. For more information, see CMD in the   Dockerfile reference and Define a command and arguments for a pod in the Kubernetes   documentation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    ///
    /// The entrypoint for the container. This isn't run within a shell. If this isn't specified,  the ENTRYPOINT of the container image is used. Environment variable references are  expanded using the container's environment.
    ///
    /// If the referenced environment variable doesn't exist, the reference in the command isn't  changed. For example, if the reference is to "$(NAME1)" and the NAME1  environment variable doesn't exist, the command string will remain "$(NAME1)."   $$ is replaced with $ and the resulting string isn't expanded. For  example, $$(VAR_NAME) will be passed as $(VAR_NAME) whether or not the   VAR_NAME environment variable exists. The entrypoint can't be updated. For more  information, see ENTRYPOINT in the Dockerfile reference and Define a command and arguments for a container and Entrypoint in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,

    ///
    /// The environment variables to pass to a container.
    ///
    /// NoteEnvironment variables cannot start with "AWS_BATCH". This naming convention is reserved for variables that AWS Batch sets.
    ///
    /// Required: No
    ///
    /// Type: List of EksContainerEnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<EksContainerEnvironmentVariable>>,

    ///
    /// The Docker image used to start the container.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Image")]
    pub image: String,

    ///
    /// The image pull policy for the container. Supported values are Always,   IfNotPresent, and Never. This parameter defaults to   IfNotPresent. However, if the :latest tag is specified, it defaults to   Always. For more information, see Updating   images in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImagePullPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,

    ///
    /// The name of the container. If the name isn't specified, the default name   "Default" is used. Each container in a pod must have a unique name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///
    /// The type and amount of resources to assign to a container. The supported resources include   memory, cpu, and nvidia.com/gpu. For more information,  see Resource management for pods and containers in the Kubernetes   documentation.
    ///
    /// Required: No
    ///
    /// Type: EksContainerResourceRequirements
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EksContainerResourceRequirements>,

    ///
    /// The security context for a job. For more information, see Configure a   security context for a pod or container in the Kubernetes   documentation.
    ///
    /// Required: No
    ///
    /// Type: EksContainerSecurityContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<EksContainerSecurityContext>,

    ///
    /// The volume mounts for the container. AWS Batch supports emptyDir,   hostPath, and secret volume types. For more information about volumes  and volume mounts in Kubernetes, see Volumes in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: List of EksContainerVolumeMount
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeMounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<EksContainerVolumeMount>>,
}

impl cfn_resources::CfnResource for EksContainer {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.resources
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.security_context
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An environment variable.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksContainerEnvironmentVariable {
    ///
    /// The name of the environment variable.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The value of the environment variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl cfn_resources::CfnResource for EksContainerEnvironmentVariable {
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

/// The EksContainerResourceRequirements property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksContainerResourceRequirements {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Requests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for EksContainerResourceRequirements {
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

/// The EksContainerSecurityContext property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksContainerSecurityContext {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunAsGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunAsNonRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunAsUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
}

impl cfn_resources::CfnResource for EksContainerSecurityContext {
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

/// The volume mounts for a container for an Amazon EKS job. For more information about volumes and  volume mounts in Kubernetes, see Volumes in the Kubernetes documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksContainerVolumeMount {
    ///
    /// The path on the container where the volume is mounted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,

    ///
    /// The name the volume mount. This must match the name of one of the volumes in the  pod.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///
    /// If this value is true, the container has read-only access to the volume.  Otherwise, the container can write to the volume. The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl cfn_resources::CfnResource for EksContainerVolumeMount {
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

/// The EksEmptyDir property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksEmptyDir {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Medium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
}

impl cfn_resources::CfnResource for EksEmptyDir {
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

/// The EksHostPath property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksHostPath {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl cfn_resources::CfnResource for EksHostPath {
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

/// An object that contains the properties for the Kubernetes resources of a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksProperties {
    ///
    /// The properties for the Kubernetes pod resources of a job.
    ///
    /// Required: No
    ///
    /// Type: PodProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "PodProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_properties: Option<PodProperties>,
}

impl cfn_resources::CfnResource for EksProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.pod_properties
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The EksSecret property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksSecret {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Optional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl cfn_resources::CfnResource for EksSecret {
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

/// Specifies an Amazon EKS volume for a job definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EksVolume {
    ///
    /// Specifies the configuration of a Kubernetes emptyDir volume. For more information,  see emptyDir  in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: EksEmptyDir
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmptyDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EksEmptyDir>,

    ///
    /// Specifies the configuration of a Kubernetes hostPath volume. For more information,  see hostPath  in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: EksHostPath
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<EksHostPath>,

    ///
    /// The name of the volume. The name must be allowed as a DNS subdomain name. For more  information, see DNS subdomain names in the Kubernetes documentation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Specifies the configuration of a Kubernetes secret volume. For more information, see   secret in the   Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: EksSecret
    ///
    /// Update requires: No interruption
    #[serde(rename = "Secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<EksSecret>,
}

impl cfn_resources::CfnResource for EksVolume {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.empty_dir
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.host_path
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.secret.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Environment property type specifies environment variables to use in a job definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Environment {
    ///
    /// The name of the environment variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///
    /// The value of the environment variable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl cfn_resources::CfnResource for Environment {
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

/// The EphemeralStorage property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EphemeralStorage {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInGiB")]
    pub size_in_gi_b: i64,
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

/// Specifies an array of up to 5 conditions to be met, and an action to take   (RETRY or EXIT) if all conditions are met. If none of the   EvaluateOnExit conditions in a RetryStrategy match, then the job is  retried.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EvaluateOnExit {
    ///
    /// Specifies the action to take if all of the specified conditions  (onStatusReason, onReason, and onExitCode) are met. The  values aren't case sensitive.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EXIT | RETRY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: EvaluateOnExitActionEnum,

    ///
    /// Contains a glob pattern to match against the decimal representation of the   ExitCode returned for a job. The pattern can be up to 512 characters long. It can  contain only numbers, and can end with an asterisk (*) so that only the start of the string needs  to be an exact match.
    ///
    /// The string can contain up to 512 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit_code: Option<String>,

    ///
    /// Contains a glob pattern to match against the Reason returned for a job. The  pattern can contain up to 512 characters. It can contain letters, numbers, periods (.), colons  (:), and white space (including spaces and tabs). It can optionally end with an asterisk (*) so  that only the start of the string needs to be an exact match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_reason: Option<String>,

    ///
    /// Contains a glob pattern to match against the StatusReason returned for a job.  The pattern can contain up to 512 characters. It can contain letters, numbers, periods (.),  colons (:), and white spaces (including spaces or tabs).  It can  optionally end with an asterisk (*) so that only the start of the string needs to be an exact  match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_status_reason: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EvaluateOnExitActionEnum {
    /// EXIT
    #[serde(rename = "EXIT")]
    Exit,

    /// RETRY
    #[serde(rename = "RETRY")]
    Retry,
}

impl Default for EvaluateOnExitActionEnum {
    fn default() -> Self {
        EvaluateOnExitActionEnum::Exit
    }
}

impl cfn_resources::CfnResource for EvaluateOnExit {
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

/// The platform configuration for jobs that are running on Fargate resources. Jobs that run  on EC2 resources must not specify this parameter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FargatePlatformConfiguration {
    ///
    /// The AWS Fargate platform version where the jobs are running. A platform version is  specified only for jobs that are running on Fargate resources. If one isn't specified, the   LATEST platform version is used by default. This uses a recent, approved version of  the AWS Fargate platform for compute resources. For more information, see AWS Fargate   platform versions in the Amazon Elastic Container Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
}

impl cfn_resources::CfnResource for FargatePlatformConfiguration {
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

/// Linux-specific modifications that are applied to the container, such as details for device  mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LinuxParameters {
    ///
    /// Any of the host devices to expose to the container. This parameter maps to   Devices in the Create a container section of the Docker Remote API  and the --device option to docker  run.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't   provide it for these jobs.
    ///
    /// Required: No
    ///
    /// Type: List of Device
    ///
    /// Update requires: No interruption
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    ///
    /// If true, run an init process inside the container that forwards signals and  reaps processes. This parameter maps to the --init option to docker run. This parameter requires version 1.25 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version | grep "Server API version"
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InitProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,

    ///
    /// The total amount of swap memory (in MiB) a container can use. This parameter is translated  to the --memory-swap option to docker   run where the value is the sum of the container memory plus the maxSwap  value. For more information, see --memory-swap details in the Docker documentation.
    ///
    /// If a maxSwap value of 0 is specified, the container doesn't use  swap. Accepted values are 0 or any positive integer. If the maxSwap  parameter is omitted, the container doesn't use the swap configuration for the container instance  that it's running on. A maxSwap value must be set for the swappiness  parameter to be used.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't   provide it for these jobs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i64>,

    ///
    /// The value for the size (in MiB) of the /dev/shm volume. This parameter maps to  the --shm-size option to docker  run.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't   provide it for these jobs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,

    ///
    /// You can use this parameter to tune a container's memory swappiness behavior. A   swappiness value of 0 causes swapping to not occur unless absolutely  necessary. A swappiness value of 100 causes pages to be swapped  aggressively. Valid values are whole numbers between 0 and 100. If the   swappiness parameter isn't specified, a default value of 60 is used.  If a value isn't specified for maxSwap, then this parameter is ignored. If   maxSwap is set to 0, the container doesn't use swap. This parameter maps to the   --memory-swappiness option to docker   run.
    ///
    /// Consider the following when you use a per-container swap configuration.
    ///
    /// Swap space must be enabled and allocated on the container instance for the containers to   use.        NoteBy default, the Amazon ECS optimized AMIs don't have swap enabled. You must enable swap on the    instance to use this feature. For more information, see Instance store swap    volumes in the Amazon EC2 User Guide for Linux Instances or How do I    allocate memory to work as swap space in an Amazon EC2 instance by using a swap    file?                        The swap space parameters are only supported for job definitions using EC2   resources.               If the maxSwap and swappiness parameters are omitted from a job   definition, each container has a default swappiness value of 60. Moreover, the   total swap usage is limited to two times the memory reservation of the container.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't   provide it for these jobs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Swappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i64>,

    ///
    /// The container path, mount options, and size (in MiB) of the tmpfs mount. This  parameter maps to the --tmpfs option to docker   run.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources. Don't   provide this parameter for this resource type.
    ///
    /// Required: No
    ///
    /// Type: List of Tmpfs
    ///
    /// Update requires: No interruption
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
        Ok(())
    }
}

/// Log configuration options to send to a custom log driver for the container.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LogConfiguration {
    ///
    /// The log driver to use for the container. The valid values that are listed for this parameter  are log drivers that the Amazon ECS container agent can communicate with by default.
    ///
    /// The supported log drivers are awslogs, fluentd, gelf,   json-file, journald, logentries, syslog, and   splunk.
    ///
    /// NoteJobs that are running on Fargate resources are restricted to the awslogs and   splunk log drivers.
    ///
    /// awslogs                  Specifies the Amazon CloudWatch Logs logging driver. For more information, see Using the awslogs log driver    in the             AWS Batch User Guide and Amazon CloudWatch Logs logging    driver in the Docker documentation.                       fluentd                  Specifies the Fluentd logging driver. For more information including usage and options,    see Fluentd logging    driver in the Docker documentation.                       gelf                  Specifies the Graylog Extended Format (GELF) logging driver. For more information    including usage and options, see Graylog Extended Format logging    driver in the Docker documentation.                       journald                  Specifies the journald logging driver. For more information including usage and options,    see Journald logging    driver in the Docker documentation.                       json-file                  Specifies the JSON file logging driver. For more information including usage and options,    see JSON File    logging driver in the Docker documentation.                       splunk                  Specifies the Splunk logging driver. For more information including usage and options,    see Splunk logging    driver in the Docker documentation.                       syslog                  Specifies the syslog logging driver. For more information including usage and options,    see Syslog logging    driver in the Docker documentation.
    ///
    /// NoteIf you have a custom driver that's not listed earlier that you want to work with the Amazon ECS   container agent, you can fork the Amazon ECS container agent project that's available on GitHub and customize it to   work with that driver. We encourage you to submit pull requests for changes that you want to   have included. However, Amazon Web Services doesn't currently support running modified copies of this   software.
    ///
    /// This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version | grep "Server API version"
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: awslogs | fluentd | gelf | journald | json-file | splunk | syslog
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDriver")]
    pub log_driver: LogConfigurationLogDriverEnum,

    ///
    /// The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: sudo docker version | grep "Server API version"
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,

    ///
    /// The secrets to pass to the log configuration. For more information, see Specifying sensitive   data in the         AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Secret
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LogConfigurationLogDriverEnum {
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
        LogConfigurationLogDriverEnum::Awslogs
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

/// The Metadata property type specifies Property description not available. for an AWS::Batch::JobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metadata {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for Metadata {
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

/// Details for a Docker volume mount point that's used in a job's container properties. This  parameter maps to Volumes in the Create a container section of the Docker Remote API and the   --volume option to docker run.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MountPoints {
    ///
    /// The path on the container where the host volume is mounted.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,

    ///
    /// If this value is true, the container has read-only access to the volume.  Otherwise, the container can write to the volume. The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    ///
    /// The name of the volume to mount.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

impl cfn_resources::CfnResource for MountPoints {
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

/// The network configuration for jobs that are running on Fargate resources. Jobs that are  running on EC2 resources must not specify this parameter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfiguration {
    ///
    /// Indicates whether the job has a public IP address. For a job that's running on Fargate  resources in a private subnet to send outbound traffic to the internet (for example, to pull  container images), the private subnet requires a NAT gateway be attached to route requests to the  internet. For more information, see Amazon ECS task networking in the   Amazon Elastic Container Service Developer Guide. The default value is "DISABLED".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<NetworkConfigurationAssignPublicIpEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NetworkConfigurationAssignPublicIpEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for NetworkConfigurationAssignPublicIpEnum {
    fn default() -> Self {
        NetworkConfigurationAssignPublicIpEnum::Disabled
    }
}

impl cfn_resources::CfnResource for NetworkConfiguration {
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

/// An object that represents the node properties of a multi-node parallel job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeProperties {
    ///
    /// Specifies the node index for the main node of a multi-node parallel job. This node index  value must be fewer than the number of nodes.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MainNode")]
    pub main_node: i64,

    ///
    /// A list of node ranges and their properties that are associated with a multi-node parallel  job.
    ///
    /// Required: Yes
    ///
    /// Type: List of NodeRangeProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "NodeRangeProperties")]
    pub node_range_properties: Vec<NodeRangeProperty>,

    ///
    /// The number of nodes that are associated with a multi-node parallel job.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumNodes")]
    pub num_nodes: i64,
}

impl cfn_resources::CfnResource for NodeProperties {
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

/// An object that represents the properties of the node range for a multi-node parallel  job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NodeRangeProperty {
    ///
    /// The container details for the node range.
    ///
    /// Required: No
    ///
    /// Type: ContainerProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerProperties>,

    ///
    /// The range of nodes, using node index values. A range of 0:3 indicates nodes  with index values of 0 through 3. If the starting range value is  omitted (:n), then 0 is used to start the range. If the ending range  value is omitted (n:), then the highest possible node index is used to end the  range. Your accumulative node ranges must account for all nodes (0:n). You can nest  node ranges (for example, 0:10 and 4:5). In this case, the   4:5 range properties override the 0:10 properties.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetNodes")]
    pub target_nodes: String,
}

impl cfn_resources::CfnResource for NodeRangeProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.container
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The properties for the pod.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PodProperties {
    ///
    /// The properties of the container that's used on the Amazon EKS pod.
    ///
    /// Required: No
    ///
    /// Type: List of EksContainer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<EksContainer>>,

    ///
    /// The DNS policy for the pod. The default value is ClusterFirst. If the   hostNetwork parameter is not specified, the default is   ClusterFirstWithHostNet. ClusterFirst indicates that any DNS query  that does not match the configured cluster domain suffix is forwarded to the upstream nameserver  inherited from the node. If no value was specified for dnsPolicy in the RegisterJobDefinition API operation, then no value will be returned for   dnsPolicy by either of DescribeJobDefinitions  or DescribeJobs API operations. The pod spec setting will contain either   ClusterFirst or ClusterFirstWithHostNet, depending on the value of the   hostNetwork parameter. For more information, see Pod's DNS policy in the Kubernetes documentation.
    ///
    /// Valid values: Default | ClusterFirst |   ClusterFirstWithHostNet
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,

    ///
    /// Indicates if the pod uses the hosts' network IP address. The default value is   true. Setting this to false enables the Kubernetes pod networking model.  Most AWS Batch workloads are egress-only and don't require the overhead of IP allocation for each  pod for incoming connections. For more information, see Host   namespaces and Pod networking  in the Kubernetes documentation.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Metadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    ///
    /// The name of the service account that's used to run the pod. For more information, see   Kubernetes service   accounts and Configure a Kubernetes service account   to assume an IAM role in the Amazon EKS User Guide and Configure service accounts for pods in the Kubernetes  documentation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceAccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    ///
    /// Specifies the volumes for a job definition that uses Amazon EKS resources.
    ///
    /// Required: No
    ///
    /// Type: List of EksVolume
    ///
    /// Update requires: No interruption
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<EksVolume>>,
}

impl cfn_resources::CfnResource for PodProperties {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The type and amount of a resource to assign to a container. The supported resources include   GPU, MEMORY, and VCPU.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceRequirement {
    ///
    /// The type of resource to assign to a container. The supported resources include   GPU, MEMORY, and VCPU.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GPU | MEMORY | VCPU
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<ResourceRequirementTypeEnum>,

    ///
    /// The quantity of the specified resource to reserve for the container. The values vary based  on the type specified.
    ///
    /// type="GPU"                  The number of physical GPUs to reserve for the container. Make sure that the number of    GPUs reserved for all containers in a job doesn't exceed the number of available GPUs on the    compute resource that the job is launched on.          NoteGPUs aren't available for jobs that are running on Fargate resources.                       type="MEMORY"                  The memory hard limit (in MiB) present to the container. This parameter is supported for    jobs that are running on EC2 resources. If your container attempts to exceed the memory    specified, the container is terminated. This parameter maps to Memory in the    Create a container section of the Docker Remote API and the    --memory option to docker run. You    must specify at least 4 MiB of memory for a job. This is required but can be specified in    several places for multi-node parallel (MNP) jobs. It must be specified for each node at least    once. This parameter maps to Memory in the Create a container    section of the Docker Remote API and the --memory option to docker run.          NoteIf you're trying to maximize your resource utilization by providing your jobs as much    memory as possible for a particular instance type, see Memory management in the                   AWS Batch User Guide.          For jobs that are running on Fargate resources, then value is the hard    limit (in MiB), and must match one of the supported values and the VCPU values    must be one of the values supported for that memory value.                                                                                                                                                                                                 value = 512                                          VCPU = 0.25                                     value = 1024                                          VCPU = 0.25 or 0.5                                     value = 2048                                          VCPU = 0.25, 0.5, or 1                                     value = 3072                                          VCPU = 0.5, or 1                                     value = 4096                                          VCPU = 0.5, 1, or 2                                     value = 5120, 6144, or 7168                                          VCPU = 1 or 2                                     value = 8192                                          VCPU = 1, 2, or 4                                     value = 9216, 10240, 11264, 12288, 13312, 14336, or 15360                                          VCPU = 2 or 4                                     value = 16384                                          VCPU = 2, 4, or 8                                     value = 17408, 18432, 19456, 21504, 22528, 23552, 25600, 26624, 27648, 29696, or 30720                                          VCPU = 4                                     value = 20480, 24576, or 28672                                          VCPU = 4 or 8                                     value = 36864, 45056, 53248, or 61440                                          VCPU = 8                                     value = 32768, 40960, 49152, or 57344                                          VCPU = 8 or 16                                     value = 65536, 73728, 81920, 90112, 98304, 106496, 114688, or 122880                                          VCPU = 16                                               type="VCPU"                  The number of vCPUs reserved for the container. This parameter maps to    CpuShares in the Create a container section of the    Docker Remote API and the --cpu-shares option to docker run. Each vCPU is equivalent to 1,024 CPU shares.    For EC2 resources, you must specify at least one vCPU. This is required but can be specified    in several places; it must be specified for each node at least once.          The default for the Fargate On-Demand vCPU resource count quota is 6 vCPUs. For more    information about Fargate quotas, see AWS Fargate quotas    in the             AWS General Reference.          For jobs that are running on Fargate resources, then value must match one    of the supported values and the MEMORY values must be one of the values supported    for that VCPU value. The supported values are 0.25, 0.5, 1, 2, 4, 8, and    16                                                                                                                value = 0.25                                          MEMORY = 512, 1024, or 2048                                     value = 0.5                                          MEMORY = 1024, 2048, 3072, or 4096                                     value = 1                                          MEMORY = 2048, 3072, 4096, 5120, 6144, 7168, or 8192                                     value = 2                                          MEMORY = 4096, 5120, 6144, 7168, 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, or 16384                                     value = 4                                          MEMORY = 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, 16384, 17408, 18432, 19456,   20480, 21504, 22528, 23552, 24576, 25600, 26624, 27648, 28672, 29696, or 30720                                     value = 8                                          MEMORY = 16384, 20480, 24576, 28672, 32768, 36864, 40960, 45056, 49152, 53248, 57344, or 61440                                     value = 16                                          MEMORY = 32768, 40960, 49152, 57344, 65536, 73728, 81920, 90112, 98304, 106496, 114688, or 122880
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ResourceRequirementTypeEnum {
    /// GPU
    #[serde(rename = "GPU")]
    Gpu,

    /// MEMORY
    #[serde(rename = "MEMORY")]
    Memory,

    /// VCPU
    #[serde(rename = "VCPU")]
    Vcpu,
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

/// The retry strategy that's associated with a job. For more information, see Automated job retries in the           AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetryStrategy {
    ///
    /// The number of times to move a job to the RUNNABLE status. You can specify  between 1 and 10 attempts. If the value of attempts is greater than one, the job is  retried on failure the same number of attempts as the value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,

    ///
    /// Array of up to 5 objects that specify the conditions where jobs are retried or failed. If  this parameter is specified, then the attempts parameter must also be specified. If  none of the listed conditions match, then the job is retried.
    ///
    /// Required: No
    ///
    /// Type: List of EvaluateOnExit
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluateOnExit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_on_exit: Option<Vec<EvaluateOnExit>>,
}

impl cfn_resources::CfnResource for RetryStrategy {
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

/// An object that represents the secret to expose to your container. Secrets can be exposed to  a container in the following ways:
///
/// For more information, see Specifying sensitive data in the           AWS Batch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Secret {
    ///
    /// The name of the secret.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The secret to expose to the container. The supported values are either the full Amazon Resource Name (ARN) of  the AWS Secrets Manager secret or the full ARN of the parameter in the AWS Systems Manager Parameter Store.
    ///
    /// NoteIf the AWS Systems Manager Parameter Store parameter exists in the same Region as the job you're   launching, then you can use either the full Amazon Resource Name (ARN) or name of the parameter. If the parameter   exists in a different Region, then the full ARN must be specified.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueFrom")]
    pub value_from: String,
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

/// An object that represents a job timeout configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Timeout {
    ///
    /// The job timeout time (in seconds) that's measured from the job attempt's   startedAt timestamp. After this time passes, AWS Batch terminates your jobs if they  aren't finished. The minimum value for the timeout is 60 seconds.
    ///
    /// For array jobs, the timeout applies to the child jobs, not to the parent array job.
    ///
    /// For multi-node parallel (MNP) jobs, the timeout applies to the whole job, not to the  individual nodes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttemptDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for Timeout {
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
    /// The absolute file path in the container where the tmpfs volume is  mounted.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerPath")]
    pub container_path: String,

    ///
    /// The list of tmpfs volume mount options.
    ///
    /// Valid values: "defaults" | "ro" | "rw" |   "suid" | "nosuid" | "dev" | "nodev" |   "exec" | "noexec" | "sync" | "async" |   "dirsync" | "remount" | "mand" | "nomand" |   "atime" | "noatime" | "diratime" |   "nodiratime" | "bind" | "rbind" | "unbindable" | "runbindable" |   "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | "relatime" |   "norelatime" | "strictatime" | "nostrictatime" |   "mode" | "uid" | "gid" | "nr_inodes" |   "nr_blocks" | "mpol"
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,

    ///
    /// The size (in MiB) of the tmpfs volume.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Ulimit {
    ///
    /// The hard limit for the ulimit type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HardLimit")]
    pub hard_limit: i64,

    ///
    /// The type of the ulimit.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The soft limit for the ulimit type.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SoftLimit")]
    pub soft_limit: i64,
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

/// A list of volumes that are associated with the job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Volumes {
    ///
    /// This is used when you're using an Amazon Elastic File System file system for job storage. For more  information, see Amazon EFS   Volumes in the         AWS Batch User Guide.
    ///
    /// Required: No
    ///
    /// Type: EfsVolumeConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EfsVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<EfsVolumeConfiguration>,

    ///
    /// The contents of the host parameter determine whether your data volume persists  on the host container instance and where it's stored. If the host parameter is empty, then the  Docker daemon assigns a host path for your data volume. However, the data isn't guaranteed to  persist after the containers that are associated with it stop running.
    ///
    /// NoteThis parameter isn't applicable to jobs that are running on Fargate resources and   shouldn't be provided.
    ///
    /// Required: No
    ///
    /// Type: VolumesHost
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<VolumesHost>,

    ///
    /// The name of the volume. It can be up to 255 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_). This name is referenced in the   sourceVolume parameter of container definition mountPoints.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl cfn_resources::CfnResource for Volumes {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.efs_volume_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.host.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Determine whether your data volume persists on the host container instance and where it's  stored. If this parameter is empty, then the Docker daemon assigns a host path for your data  volume. However, the data isn't guaranteed to persist after the containers that are associated  with it stop running.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VolumesHost {
    ///
    /// The path on the host container instance that's presented to the container. If this parameter  is empty, then the Docker daemon has assigned a host path for you. If this parameter contains a  file location, then the data volume persists at the specified location on the host container  instance until you delete it manually. If the source path location doesn't exist on the host  container instance, the Docker daemon creates it. If the location does exist, the contents of the  source path folder are exported.
    ///
    /// NoteThis parameter isn't applicable to jobs that run on Fargate resources. Don't provide this   for these jobs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

impl cfn_resources::CfnResource for VolumesHost {
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
