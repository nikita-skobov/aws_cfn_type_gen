
pub mod cfn_compute_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnComputeEnvironment {
    /// No documentation provided by AWS
    #[serde(rename = "UpdatePolicy")]
    pub update_policy: Option<UpdatePolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "UnmanagedvCpus")]
    pub unmanagedv_cpus: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ComputeResources")]
    pub compute_resources: Option<ComputeResources>,
    /// No documentation provided by AWS
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ComputeEnvironmentName")]
    pub compute_environment_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplaceComputeEnvironment")]
    pub replace_compute_environment: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EksConfiguration")]
    pub eks_configuration: Option<EksConfiguration>,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,

}


#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EksConfiguration {
    #[serde(rename = "EksClusterArn")]
    pub eks_cluster_arn: String,
    #[serde(rename = "KubernetesNamespace")]
    pub kubernetes_namespace: String,

}

#[derive(serde::Serialize, Default)]
pub struct Ec2ConfigurationObject {
    #[serde(rename = "ImageIdOverride")]
    pub image_id_override: Option<String>,
    #[serde(rename = "ImageKubernetesVersion")]
    pub image_kubernetes_version: Option<String>,
    #[serde(rename = "ImageType")]
    pub image_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct UpdatePolicy {
    #[serde(rename = "JobExecutionTimeoutMinutes")]
    pub job_execution_timeout_minutes: Option<usize>,
    #[serde(rename = "TerminateJobsOnUpdate")]
    pub terminate_jobs_on_update: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ComputeResources {
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,
    #[serde(rename = "PlacementGroup")]
    pub placement_group: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "BidPercentage")]
    pub bid_percentage: Option<usize>,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "InstanceRole")]
    pub instance_role: Option<String>,
    #[serde(rename = "UpdateToLatestImageVersion")]
    pub update_to_latest_image_version: Option<bool>,
    #[serde(rename = "MinvCpus")]
    pub minv_cpus: Option<usize>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Ec2KeyPair")]
    pub ec2_key_pair: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "Ec2Configuration")]
    pub ec2_configuration: Option<Vec<Ec2ConfigurationObject>>,
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,
    #[serde(rename = "MaxvCpus")]
    pub maxv_cpus: usize,
    #[serde(rename = "SpotIamFleetRole")]
    pub spot_iam_fleet_role: Option<String>,
    #[serde(rename = "DesiredvCpus")]
    pub desiredv_cpus: Option<usize>,

}


}

pub mod cfn_job_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnJobDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "Timeout")]
    pub timeout: Option<Timeout>,
    /// No documentation provided by AWS
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RetryStrategy")]
    pub retry_strategy: Option<RetryStrategy>,
    /// No documentation provided by AWS
    #[serde(rename = "NodeProperties")]
    pub node_properties: Option<NodeProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "PlatformCapabilities")]
    pub platform_capabilities: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EksProperties")]
    pub eks_properties: Option<EksProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "SchedulingPriority")]
    pub scheduling_priority: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ContainerProperties")]
    pub container_properties: Option<ContainerProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,

}


#[derive(serde::Serialize, Default)]
pub struct RetryStrategy {
    #[serde(rename = "Attempts")]
    pub attempts: Option<usize>,
    #[serde(rename = "EvaluateOnExit")]
    pub evaluate_on_exit: Option<Vec<EvaluateOnExit>>,

}

#[derive(serde::Serialize, Default)]
pub struct FargatePlatformConfiguration {
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LinuxParameters {
    #[serde(rename = "Swappiness")]
    pub swappiness: Option<usize>,
    #[serde(rename = "MaxSwap")]
    pub max_swap: Option<usize>,
    #[serde(rename = "InitProcessEnabled")]
    pub init_process_enabled: Option<bool>,
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<Vec<Tmpfs>>,
    #[serde(rename = "SharedMemorySize")]
    pub shared_memory_size: Option<usize>,
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<Device>>,

}

#[derive(serde::Serialize, Default)]
pub struct EksVolume {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "HostPath")]
    pub host_path: Option<EksHostPath>,
    #[serde(rename = "Secret")]
    pub secret: Option<EksSecret>,
    #[serde(rename = "EmptyDir")]
    pub empty_dir: Option<EksEmptyDir>,

}

#[derive(serde::Serialize, Default)]
pub struct EphemeralStorage {
    #[serde(rename = "SizeInGiB")]
    pub size_in_gi_b: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Tmpfs {
    #[serde(rename = "ContainerPath")]
    pub container_path: String,
    #[serde(rename = "Size")]
    pub size: usize,
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<Vec<String>>,

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
pub struct Environment {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NodeRangeProperty {
    #[serde(rename = "Container")]
    pub container: Option<ContainerProperties>,
    #[serde(rename = "TargetNodes")]
    pub target_nodes: String,

}

#[derive(serde::Serialize, Default)]
pub struct NodeProperties {
    #[serde(rename = "NodeRangeProperties")]
    pub node_range_properties: Vec<NodeRangeProperty>,
    #[serde(rename = "MainNode")]
    pub main_node: usize,
    #[serde(rename = "NumNodes")]
    pub num_nodes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct EksProperties {
    #[serde(rename = "PodProperties")]
    pub pod_properties: Option<PodProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct EfsVolumeConfiguration {
    #[serde(rename = "AuthorizationConfig")]
    pub authorization_config: Option<AuthorizationConfig>,
    #[serde(rename = "TransitEncryption")]
    pub transit_encryption: Option<String>,
    #[serde(rename = "RootDirectory")]
    pub root_directory: Option<String>,
    #[serde(rename = "TransitEncryptionPort")]
    pub transit_encryption_port: Option<usize>,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct EksContainerSecurityContext {
    #[serde(rename = "RunAsGroup")]
    pub run_as_group: Option<usize>,
    #[serde(rename = "RunAsNonRoot")]
    pub run_as_non_root: Option<bool>,
    #[serde(rename = "RunAsUser")]
    pub run_as_user: Option<usize>,
    #[serde(rename = "ReadOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Volumes {
    #[serde(rename = "EfsVolumeConfiguration")]
    pub efs_volume_configuration: Option<EfsVolumeConfiguration>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Host")]
    pub host: Option<VolumesHost>,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerProperties {
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,
    #[serde(rename = "MountPoints")]
    pub mount_points: Option<Vec<MountPoints>>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<Volumes>>,
    #[serde(rename = "FargatePlatformConfiguration")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "ReadonlyRootFilesystem")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "Memory")]
    pub memory: Option<usize>,
    #[serde(rename = "JobRoleArn")]
    pub job_role_arn: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "LinuxParameters")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<Environment>>,
    #[serde(rename = "Vcpus")]
    pub vcpus: Option<usize>,
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<EphemeralStorage>,

}

#[derive(serde::Serialize, Default)]
pub struct EksContainer {
    #[serde(rename = "Env")]
    pub env: Option<Vec<EksContainerEnvironmentVariable>>,
    #[serde(rename = "ImagePullPolicy")]
    pub image_pull_policy: Option<String>,
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "VolumeMounts")]
    pub volume_mounts: Option<Vec<EksContainerVolumeMount>>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Resources")]
    pub resources: Option<EksContainerResourceRequirements>,
    #[serde(rename = "SecurityContext")]
    pub security_context: Option<EksContainerSecurityContext>,

}

#[derive(serde::Serialize, Default)]
pub struct Timeout {
    #[serde(rename = "AttemptDurationSeconds")]
    pub attempt_duration_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VolumesHost {
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Metadata {
    #[serde(rename = "Labels")]
    pub labels: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct EvaluateOnExit {
    #[serde(rename = "OnStatusReason")]
    pub on_status_reason: Option<String>,
    #[serde(rename = "OnExitCode")]
    pub on_exit_code: Option<String>,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "OnReason")]
    pub on_reason: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MountPoints {
    #[serde(rename = "ContainerPath")]
    pub container_path: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "SourceVolume")]
    pub source_volume: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EksContainerResourceRequirements {
    #[serde(rename = "Requests")]
    pub requests: Option<()>,
    #[serde(rename = "Limits")]
    pub limits: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct EksContainerEnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct PodProperties {
    #[serde(rename = "HostNetwork")]
    pub host_network: Option<bool>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<EksVolume>>,
    #[serde(rename = "ServiceAccountName")]
    pub service_account_name: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<EksContainer>>,
    #[serde(rename = "DnsPolicy")]
    pub dns_policy: Option<String>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthorizationConfig {
    #[serde(rename = "AccessPointId")]
    pub access_point_id: Option<String>,
    #[serde(rename = "Iam")]
    pub iam: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Device {
    #[serde(rename = "ContainerPath")]
    pub container_path: Option<String>,
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "HostPath")]
    pub host_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceRequirement {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Secret {
    #[serde(rename = "ValueFrom")]
    pub value_from: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct EksEmptyDir {
    #[serde(rename = "SizeLimit")]
    pub size_limit: Option<String>,
    #[serde(rename = "Medium")]
    pub medium: Option<String>,

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
pub struct EksSecret {
    #[serde(rename = "SecretName")]
    pub secret_name: String,
    #[serde(rename = "Optional")]
    pub optional: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct EksContainerVolumeMount {
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "MountPath")]
    pub mount_path: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EksHostPath {
    #[serde(rename = "Path")]
    pub path: Option<String>,

}


}

pub mod cfn_job_queue {

#[derive(serde::Serialize, Default)]
pub struct CfnJobQueue {
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// List of ComputeEnvironmentOrder
    #[serde(rename = "ComputeEnvironmentOrder")]
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    /// No documentation provided by AWS
    #[serde(rename = "Priority")]
    pub priority: usize,
    /// No documentation provided by AWS
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "JobQueueName")]
    pub job_queue_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SchedulingPolicyArn")]
    pub scheduling_policy_arn: Option<ResourceArn>,

}

pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct ComputeEnvironmentOrder {
    #[serde(rename = "Order")]
    pub order: usize,
    #[serde(rename = "ComputeEnvironment")]
    pub compute_environment: String,

}


}

pub mod cfn_scheduling_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnSchedulingPolicy {
    /// Fair Share Policy for the Job Queue.
    #[serde(rename = "FairsharePolicy")]
    pub fairshare_policy: Option<FairsharePolicy>,
    /// A key-value pair to associate with a resource.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// Name of Scheduling Policy.
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ShareAttributes {
    #[serde(rename = "ShareIdentifier")]
    pub share_identifier: Option<String>,
    #[serde(rename = "WeightFactor")]
    pub weight_factor: Option<f64>,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct FairsharePolicy {
    #[serde(rename = "ShareDecaySeconds")]
    pub share_decay_seconds: Option<f64>,
    #[serde(rename = "ShareDistribution")]
    pub share_distribution: Option<Vec<ShareAttributes>>,
    #[serde(rename = "ComputeReservation")]
    pub compute_reservation: Option<f64>,

}


}
