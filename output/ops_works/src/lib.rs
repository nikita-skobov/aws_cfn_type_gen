
pub mod cfn_app {

#[derive(serde::Serialize, Default)]
pub struct CfnApp {
    /// No documentation provided by AWS
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// List of DataSource
    #[serde(rename = "DataSources")]
    pub data_sources: Option<Vec<DataSource>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EnableSsl")]
    pub enable_ssl: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SslConfiguration")]
    pub ssl_configuration: Option<SslConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AppSource")]
    pub app_source: Option<Source>,
    /// No documentation provided by AWS
    #[serde(rename = "Domains")]
    pub domains: Option<Vec<String>>,
    /// List of EnvironmentVariable
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<EnvironmentVariable>>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "Shortname")]
    pub shortname: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Secure")]
    pub secure: Option<bool>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataSource {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SslConfiguration {
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,
    #[serde(rename = "Chain")]
    pub chain: Option<String>,
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "Url")]
    pub url: Option<String>,
    #[serde(rename = "Revision")]
    pub revision: Option<String>,
    #[serde(rename = "SshKey")]
    pub ssh_key: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "Username")]
    pub username: Option<String>,

}


}

pub mod cfn_elastic_load_balancer_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnElasticLoadBalancerAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "LayerId")]
    pub layer_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,

}



}

pub mod cfn_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnInstance {
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AmiId")]
    pub ami_id: Option<String>,
    /// List of BlockDeviceMapping
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// No documentation provided by AWS
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RootDeviceType")]
    pub root_device_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SshKeyName")]
    pub ssh_key_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstallUpdatesOnBoot")]
    pub install_updates_on_boot: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "LayerIds")]
    pub layer_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Os")]
    pub os: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualizationType")]
    pub virtualization_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeBasedAutoScaling")]
    pub time_based_auto_scaling: Option<TimeBasedAutoScaling>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingType")]
    pub auto_scaling_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AgentVersion")]
    pub agent_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticIps")]
    pub elastic_ips: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct BlockDeviceMapping {
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsBlockDevice>,
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TimeBasedAutoScaling {
    #[serde(rename = "Tuesday")]
    pub tuesday: Option<()>,
    #[serde(rename = "Saturday")]
    pub saturday: Option<()>,
    #[serde(rename = "Sunday")]
    pub sunday: Option<()>,
    #[serde(rename = "Monday")]
    pub monday: Option<()>,
    #[serde(rename = "Friday")]
    pub friday: Option<()>,
    #[serde(rename = "Thursday")]
    pub thursday: Option<()>,
    #[serde(rename = "Wednesday")]
    pub wednesday: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct EbsBlockDevice {
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,

}


}

pub mod cfn_layer {

#[derive(serde::Serialize, Default)]
pub struct CfnLayer {
    /// No documentation provided by AWS
    #[serde(rename = "AutoAssignPublicIps")]
    pub auto_assign_public_ips: bool,
    /// No documentation provided by AWS
    #[serde(rename = "CustomRecipes")]
    pub custom_recipes: Option<Recipes>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableAutoHealing")]
    pub enable_auto_healing: bool,
    /// No documentation provided by AWS
    #[serde(rename = "StackId")]
    pub stack_id: String,
    /// List of VolumeConfiguration
    #[serde(rename = "VolumeConfigurations")]
    pub volume_configurations: Option<Vec<VolumeConfiguration>>,
    /// No documentation provided by AWS
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomJson")]
    pub custom_json: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomSecurityGroupIds")]
    pub custom_security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomInstanceProfileArn")]
    pub custom_instance_profile_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Shortname")]
    pub shortname: String,
    /// No documentation provided by AWS
    #[serde(rename = "LoadBasedAutoScaling")]
    pub load_based_auto_scaling: Option<LoadBasedAutoScaling>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: bool,
    /// No documentation provided by AWS
    #[serde(rename = "LifecycleEventConfiguration")]
    pub lifecycle_event_configuration: Option<LifecycleEventConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "InstallUpdatesOnBoot")]
    pub install_updates_on_boot: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "UseEbsOptimizedInstances")]
    pub use_ebs_optimized_instances: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Packages")]
    pub packages: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Recipes {
    #[serde(rename = "Setup")]
    pub setup: Option<Vec<String>>,
    #[serde(rename = "Configure")]
    pub configure: Option<Vec<String>>,
    #[serde(rename = "Shutdown")]
    pub shutdown: Option<Vec<String>>,
    #[serde(rename = "Deploy")]
    pub deploy: Option<Vec<String>>,
    #[serde(rename = "Undeploy")]
    pub undeploy: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ShutdownEventConfiguration {
    #[serde(rename = "ExecutionTimeout")]
    pub execution_timeout: Option<usize>,
    #[serde(rename = "DelayUntilElbConnectionsDrained")]
    pub delay_until_elb_connections_drained: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoScalingThresholds {
    #[serde(rename = "CpuThreshold")]
    pub cpu_threshold: Option<f64>,
    #[serde(rename = "LoadThreshold")]
    pub load_threshold: Option<f64>,
    #[serde(rename = "MemoryThreshold")]
    pub memory_threshold: Option<f64>,
    #[serde(rename = "ThresholdsWaitTime")]
    pub thresholds_wait_time: Option<usize>,
    #[serde(rename = "InstanceCount")]
    pub instance_count: Option<usize>,
    #[serde(rename = "IgnoreMetricsTime")]
    pub ignore_metrics_time: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadBasedAutoScaling {
    #[serde(rename = "Enable")]
    pub enable: Option<bool>,
    #[serde(rename = "UpScaling")]
    pub up_scaling: Option<AutoScalingThresholds>,
    #[serde(rename = "DownScaling")]
    pub down_scaling: Option<AutoScalingThresholds>,

}

#[derive(serde::Serialize, Default)]
pub struct LifecycleEventConfiguration {
    #[serde(rename = "ShutdownEventConfiguration")]
    pub shutdown_event_configuration: Option<ShutdownEventConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct VolumeConfiguration {
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "NumberOfDisks")]
    pub number_of_disks: Option<usize>,
    #[serde(rename = "RaidLevel")]
    pub raid_level: Option<usize>,
    #[serde(rename = "Size")]
    pub size: Option<usize>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "MountPoint")]
    pub mount_point: Option<String>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

}


}

pub mod cfn_stack {

#[derive(serde::Serialize, Default)]
pub struct CfnStack {
    /// No documentation provided by AWS
    #[serde(rename = "ConfigurationManager")]
    pub configuration_manager: Option<StackConfigurationManager>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceStackId")]
    pub source_stack_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UseCustomCookbooks")]
    pub use_custom_cookbooks: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "UseOpsworksSecurityGroups")]
    pub use_opsworks_security_groups: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ClonePermissions")]
    pub clone_permissions: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultSubnetId")]
    pub default_subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultOs")]
    pub default_os: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultSshKeyName")]
    pub default_ssh_key_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomJson")]
    pub custom_json: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EcsClusterArn")]
    pub ecs_cluster_arn: Option<String>,
    /// List of ElasticIp
    #[serde(rename = "ElasticIps")]
    pub elastic_ips: Option<Vec<ElasticIp>>,
    /// No documentation provided by AWS
    #[serde(rename = "CloneAppIds")]
    pub clone_app_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ChefConfiguration")]
    pub chef_configuration: Option<ChefConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultAvailabilityZone")]
    pub default_availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRootDeviceType")]
    pub default_root_device_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostnameTheme")]
    pub hostname_theme: Option<String>,
    /// List of RdsDbInstance
    #[serde(rename = "RdsDbInstances")]
    pub rds_db_instances: Option<Vec<RdsDbInstance>>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomCookbooksSource")]
    pub custom_cookbooks_source: Option<Source>,
    /// No documentation provided by AWS
    #[serde(rename = "AgentVersion")]
    pub agent_version: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct StackConfigurationManager {
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticIp {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Ip")]
    pub ip: String,

}

#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "Username")]
    pub username: Option<String>,
    #[serde(rename = "SshKey")]
    pub ssh_key: Option<String>,
    #[serde(rename = "Revision")]
    pub revision: Option<String>,
    #[serde(rename = "Password")]
    pub password: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Url")]
    pub url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct RdsDbInstance {
    #[serde(rename = "RdsDbInstanceArn")]
    pub rds_db_instance_arn: String,
    #[serde(rename = "DbUser")]
    pub db_user: String,
    #[serde(rename = "DbPassword")]
    pub db_password: String,

}

#[derive(serde::Serialize, Default)]
pub struct ChefConfiguration {
    #[serde(rename = "ManageBerkshelf")]
    pub manage_berkshelf: Option<bool>,
    #[serde(rename = "BerkshelfVersion")]
    pub berkshelf_version: Option<String>,

}


}

pub mod cfn_user_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnUserProfile {
    /// No documentation provided by AWS
    #[serde(rename = "SshPublicKey")]
    pub ssh_public_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SshUsername")]
    pub ssh_username: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowSelfManagement")]
    pub allow_self_management: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: String,

}



}

pub mod cfn_volume {

#[derive(serde::Serialize, Default)]
pub struct CfnVolume {
    /// No documentation provided by AWS
    #[serde(rename = "Ec2VolumeId")]
    pub ec2_volume_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MountPoint")]
    pub mount_point: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StackId")]
    pub stack_id: String,

}



}
