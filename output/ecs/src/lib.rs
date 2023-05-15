
pub mod cfn_capacity_provider {

#[derive(serde::Serialize, Default)]
pub struct CfnCapacityProvider {
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingGroupProvider")]
    pub auto_scaling_group_provider: AutoScalingGroupProvider,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ManagedScaling {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "MinimumScalingStepSize")]
    pub minimum_scaling_step_size: Option<usize>,
    #[serde(rename = "TargetCapacity")]
    pub target_capacity: Option<usize>,
    #[serde(rename = "InstanceWarmupPeriod")]
    pub instance_warmup_period: Option<usize>,
    #[serde(rename = "MaximumScalingStepSize")]
    pub maximum_scaling_step_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoScalingGroupProvider {
    #[serde(rename = "ManagedScaling")]
    pub managed_scaling: Option<ManagedScaling>,
    #[serde(rename = "AutoScalingGroupArn")]
    pub auto_scaling_group_arn: String,
    #[serde(rename = "ManagedTerminationProtection")]
    pub managed_termination_protection: Option<String>,

}


}

pub mod cfn_cluster {

#[derive(serde::Serialize, Default)]
pub struct CfnCluster {
    /// List of ClusterSettings
    #[serde(rename = "ClusterSettings")]
    pub cluster_settings: Option<Vec<ClusterSettings>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<ClusterConfiguration>,
    /// A user-generated string that you use to identify your cluster. If you don't specify a name, AWS CloudFormation generates a unique physical ID for the name.
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,
    /// List of CapacityProviderStrategyItem
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    pub default_capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// No documentation provided by AWS
    #[serde(rename = "CapacityProviders")]
    pub capacity_providers: Option<Vec<String>>,
    /// Service Connect Configuration default for all services or tasks within this cluster
    #[serde(rename = "ServiceConnectDefaults")]
    pub service_connect_defaults: Option<ServiceConnectDefaults>,

}


#[derive(serde::Serialize, Default)]
pub struct ServiceConnectDefaults {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategyItem {
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "Base")]
    pub base: Option<usize>,
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ExecuteCommandLogConfiguration {
    #[serde(rename = "CloudWatchEncryptionEnabled")]
    pub cloud_watch_encryption_enabled: Option<bool>,
    #[serde(rename = "S3EncryptionEnabled")]
    pub s3_encryption_enabled: Option<bool>,
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "CloudWatchLogGroupName")]
    pub cloud_watch_log_group_name: Option<String>,
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterSettings {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfiguration {
    #[serde(rename = "ExecuteCommandConfiguration")]
    pub execute_command_configuration: Option<ExecuteCommandConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ExecuteCommandConfiguration {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Logging")]
    pub logging: Option<String>,
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<ExecuteCommandLogConfiguration>,

}


}

pub mod cfn_cluster_capacity_provider_associations {

#[derive(serde::Serialize, Default)]
pub struct CfnClusterCapacityProviderAssociations {
    /// List of capacity providers to associate with the cluster
    #[serde(rename = "CapacityProviders")]
    pub capacity_providers: CapacityProviders,
    /// List of capacity providers to associate with the cluster
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    pub default_capacity_provider_strategy: DefaultCapacityProviderStrategy,
    /// The name of the cluster
    #[serde(rename = "Cluster")]
    pub cluster: Cluster,

}


#[derive(serde::Serialize, Default)]
pub struct DefaultCapacityProviderStrategy {

}
pub type Cluster = String;
#[derive(serde::Serialize, Default)]
pub struct CapacityProviders {

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategy {
    #[serde(rename = "Base")]
    pub base: Option<usize>,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: CapacityProvider,

}
pub type CapacityProvider = String;

}

pub mod cfn_primary_task_set {

#[derive(serde::Serialize, Default)]
pub struct CfnPrimaryTaskSet {
    /// The short name or full Amazon Resource Name (ARN) of the service to create the task set in.
    #[serde(rename = "Service")]
    pub service: String,
    /// The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to create the task set in.
    #[serde(rename = "Cluster")]
    pub cluster: String,
    /// The ID or full Amazon Resource Name (ARN) of the task set.
    #[serde(rename = "TaskSetId")]
    pub task_set_id: String,

}



}

pub mod cfn_service {

#[derive(serde::Serialize, Default)]
pub struct CfnService {
    /// No documentation provided by AWS
    #[serde(rename = "EnableExecuteCommand")]
    pub enable_execute_command: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SchedulingStrategy")]
    pub scheduling_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<String>,
    /// List of CapacityProviderStrategyItem
    #[serde(rename = "CapacityProviderStrategy")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    /// List of LoadBalancer
    #[serde(rename = "LoadBalancers")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// List of PlacementConstraint
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// No documentation provided by AWS
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckGracePeriodSeconds")]
    pub health_check_grace_period_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Role")]
    pub role: Option<String>,
    /// List of PlacementStrategy
    #[serde(rename = "PlacementStrategies")]
    pub placement_strategies: Option<Vec<PlacementStrategy>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentController")]
    pub deployment_controller: Option<DeploymentController>,
    /// List of ServiceRegistry
    #[serde(rename = "ServiceRegistries")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// No documentation provided by AWS
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Cluster")]
    pub cluster: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskDefinition")]
    pub task_definition: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DesiredCount")]
    pub desired_count: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentConfiguration")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableECSManagedTags")]
    pub enable_ecsmanaged_tags: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceConnectConfiguration")]
    pub service_connect_configuration: Option<ServiceConnectConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DeploymentCircuitBreaker {
    #[serde(rename = "Rollback")]
    pub rollback: bool,
    #[serde(rename = "Enable")]
    pub enable: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Secret {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ValueFrom")]
    pub value_from: String,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategyItem {
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: Option<String>,
    #[serde(rename = "Base")]
    pub base: Option<usize>,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentConfiguration {
    #[serde(rename = "MinimumHealthyPercent")]
    pub minimum_healthy_percent: Option<usize>,
    #[serde(rename = "DeploymentCircuitBreaker")]
    pub deployment_circuit_breaker: Option<DeploymentCircuitBreaker>,
    #[serde(rename = "MaximumPercent")]
    pub maximum_percent: Option<usize>,
    #[serde(rename = "Alarms")]
    pub alarms: Option<DeploymentAlarms>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadBalancer {
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AwsvpcConfiguration")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceConnectService {
    #[serde(rename = "PortName")]
    pub port_name: String,
    #[serde(rename = "DiscoveryName")]
    pub discovery_name: Option<String>,
    #[serde(rename = "ClientAliases")]
    pub client_aliases: Option<Vec<ServiceConnectClientAlias>>,
    #[serde(rename = "IngressPortOverride")]
    pub ingress_port_override: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceRegistry {
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "RegistryArn")]
    pub registry_arn: Option<String>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentAlarms {
    #[serde(rename = "AlarmNames")]
    pub alarm_names: Vec<String>,
    #[serde(rename = "Rollback")]
    pub rollback: bool,
    #[serde(rename = "Enable")]
    pub enable: bool,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentController {
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceConnectConfiguration {
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Services")]
    pub services: Option<Vec<ServiceConnectService>>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementConstraint {
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct LogConfiguration {
    #[serde(rename = "LogDriver")]
    pub log_driver: Option<String>,
    #[serde(rename = "SecretOptions")]
    pub secret_options: Option<Vec<Secret>>,
    #[serde(rename = "Options")]
    pub options: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementStrategy {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Field")]
    pub field: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceConnectClientAlias {
    #[serde(rename = "DnsName")]
    pub dns_name: Option<String>,
    #[serde(rename = "Port")]
    pub port: usize,

}


}

pub mod cfn_task_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnTaskDefinition {
    /// List of ContainerDefinition
    #[serde(rename = "ContainerDefinitions")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,
    /// No documentation provided by AWS
    #[serde(rename = "RuntimePlatform")]
    pub runtime_platform: Option<RuntimePlatform>,
    /// No documentation provided by AWS
    #[serde(rename = "RequiresCompatibilities")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Cpu")]
    pub cpu: Option<String>,
    /// List of Volume
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<Volume>>,
    /// List of InferenceAccelerator
    #[serde(rename = "InferenceAccelerators")]
    pub inference_accelerators: Option<Vec<InferenceAccelerator>>,
    /// No documentation provided by AWS
    #[serde(rename = "ProxyConfiguration")]
    pub proxy_configuration: Option<ProxyConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskRoleArn")]
    pub task_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Family")]
    pub family: Option<String>,
    /// List of TaskDefinitionPlacementConstraint
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    /// No documentation provided by AWS
    #[serde(rename = "Memory")]
    pub memory: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct HostVolumeProperties {
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheck {
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<usize>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,
    #[serde(rename = "Retries")]
    pub retries: Option<usize>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct LinuxParameters {
    #[serde(rename = "MaxSwap")]
    pub max_swap: Option<usize>,
    #[serde(rename = "Swappiness")]
    pub swappiness: Option<usize>,
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<KernelCapabilities>,
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<Vec<Tmpfs>>,
    #[serde(rename = "SharedMemorySize")]
    pub shared_memory_size: Option<usize>,
    #[serde(rename = "InitProcessEnabled")]
    pub init_process_enabled: Option<bool>,
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<Device>>,

}

#[derive(serde::Serialize, Default)]
pub struct FirelensConfiguration {
    #[serde(rename = "Options")]
    pub options: Option<()>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProxyConfiguration {
    #[serde(rename = "ProxyConfigurationProperties")]
    pub proxy_configuration_properties: Option<Vec<KeyValuePair>>,
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LogConfiguration {
    #[serde(rename = "Options")]
    pub options: Option<()>,
    #[serde(rename = "SecretOptions")]
    pub secret_options: Option<Vec<Secret>>,
    #[serde(rename = "LogDriver")]
    pub log_driver: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Device {
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "ContainerPath")]
    pub container_path: Option<String>,
    #[serde(rename = "HostPath")]
    pub host_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MountPoint {
    #[serde(rename = "SourceVolume")]
    pub source_volume: Option<String>,
    #[serde(rename = "ContainerPath")]
    pub container_path: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct RuntimePlatform {
    #[serde(rename = "OperatingSystemFamily")]
    pub operating_system_family: Option<String>,
    #[serde(rename = "CpuArchitecture")]
    pub cpu_architecture: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Secret {
    #[serde(rename = "ValueFrom")]
    pub value_from: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct VolumeFrom {
    #[serde(rename = "SourceContainer")]
    pub source_container: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerDependency {
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InferenceAccelerator {
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,
    #[serde(rename = "DeviceType")]
    pub device_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DockerVolumeConfiguration {
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<()>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<()>,
    #[serde(rename = "Autoprovision")]
    pub autoprovision: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PortMapping {
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "ContainerPortRange")]
    pub container_port_range: Option<String>,
    #[serde(rename = "HostPort")]
    pub host_port: Option<usize>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "AppProtocol")]
    pub app_protocol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Volume {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "DockerVolumeConfiguration")]
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,
    #[serde(rename = "Host")]
    pub host: Option<HostVolumeProperties>,
    #[serde(rename = "EFSVolumeConfiguration")]
    pub efsvolume_configuration: Option<EFSVolumeConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct Ulimit {
    #[serde(rename = "SoftLimit")]
    pub soft_limit: usize,
    #[serde(rename = "HardLimit")]
    pub hard_limit: usize,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct AuthorizationConfig {
    #[serde(rename = "IAM")]
    pub iam: Option<String>,
    #[serde(rename = "AccessPointId")]
    pub access_point_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RepositoryCredentials {
    #[serde(rename = "CredentialsParameter")]
    pub credentials_parameter: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tmpfs {
    #[serde(rename = "Size")]
    pub size: usize,
    #[serde(rename = "ContainerPath")]
    pub container_path: Option<String>,
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct EphemeralStorage {
    #[serde(rename = "SizeInGiB")]
    pub size_in_gi_b: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceRequirement {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct KernelCapabilities {
    #[serde(rename = "Add")]
    pub add: Option<Vec<String>>,
    #[serde(rename = "Drop")]
    pub drop: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct HostEntry {
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TaskDefinitionPlacementConstraint {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyValuePair {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerDefinition {
    #[serde(rename = "DnsServers")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(rename = "SystemControls")]
    pub system_controls: Option<Vec<SystemControl>>,
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "Memory")]
    pub memory: Option<usize>,
    #[serde(rename = "PortMappings")]
    pub port_mappings: Option<Vec<PortMapping>>,
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<usize>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EnvironmentFiles")]
    pub environment_files: Option<Vec<EnvironmentFile>>,
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Option<Vec<VolumeFrom>>,
    #[serde(rename = "MountPoints")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(rename = "DockerLabels")]
    pub docker_labels: Option<()>,
    #[serde(rename = "StartTimeout")]
    pub start_timeout: Option<usize>,
    #[serde(rename = "PseudoTerminal")]
    pub pseudo_terminal: Option<bool>,
    #[serde(rename = "DnsSearchDomains")]
    pub dns_search_domains: Option<Vec<String>>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "DependsOn")]
    pub depends_on: Option<Vec<ContainerDependency>>,
    #[serde(rename = "Interactive")]
    pub interactive: Option<bool>,
    #[serde(rename = "DisableNetworking")]
    pub disable_networking: Option<bool>,
    #[serde(rename = "EntryPoint")]
    pub entry_point: Option<Vec<String>>,
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheck>,
    #[serde(rename = "Cpu")]
    pub cpu: Option<usize>,
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,
    #[serde(rename = "ReadonlyRootFilesystem")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: Option<usize>,
    #[serde(rename = "Essential")]
    pub essential: Option<bool>,
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "RepositoryCredentials")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "FirelensConfiguration")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Option<Vec<HostEntry>>,
    #[serde(rename = "LinuxParameters")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "DockerSecurityOptions")]
    pub docker_security_options: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct EnvironmentFile {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EFSVolumeConfiguration {
    #[serde(rename = "AuthorizationConfig")]
    pub authorization_config: Option<AuthorizationConfig>,
    #[serde(rename = "FilesystemId")]
    pub filesystem_id: String,
    #[serde(rename = "TransitEncryptionPort")]
    pub transit_encryption_port: Option<usize>,
    #[serde(rename = "TransitEncryption")]
    pub transit_encryption: Option<String>,
    #[serde(rename = "RootDirectory")]
    pub root_directory: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SystemControl {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_task_set {

#[derive(serde::Serialize, Default)]
pub struct CfnTaskSet {
    /// List of LoadBalancer
    #[serde(rename = "LoadBalancers")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// An object representing the network configuration for a task or service.
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// The platform version that the tasks in the task set should use. A platform version is specified only for tasks using the Fargate launch type. If one isn't specified, the LATEST platform version is used by default.
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,
    /// The details of the service discovery registries to assign to this task set. For more information, see https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html.
    #[serde(rename = "ServiceRegistries")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// A floating-point percentage of the desired number of tasks to place and keep running in the task set.
    #[serde(rename = "Scale")]
    pub scale: Option<Scale>,
    /// The launch type that new tasks in the task set will use. For more information, see https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html in the Amazon Elastic Container Service Developer Guide.
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<String>,
    /// The short name or full Amazon Resource Name (ARN) of the service to create the task set in.
    #[serde(rename = "Service")]
    pub service: String,
    /// The short name or full Amazon Resource Name (ARN) of the task definition for the tasks in the task set to use.
    #[serde(rename = "TaskDefinition")]
    pub task_definition: String,
    /// An optional non-unique tag that identifies this task set in external systems. If the task set is associated with a service discovery registry, the tasks in this task set will have the ECS_TASK_SET_EXTERNAL_ID AWS Cloud Map attribute set to the provided value.
    #[serde(rename = "ExternalId")]
    pub external_id: Option<String>,
    /// The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to create the task set in.
    #[serde(rename = "Cluster")]
    pub cluster: String,

}


#[derive(serde::Serialize, Default)]
pub struct LoadBalancer {
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "TargetGroupArn")]
    pub target_group_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AwsVpcConfiguration")]
    pub aws_vpc_configuration: Option<AwsVpcConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct Scale {
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceRegistry {
    #[serde(rename = "RegistryArn")]
    pub registry_arn: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,
    #[serde(rename = "ContainerPort")]
    pub container_port: Option<usize>,
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}


}
