
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// The cpu architecture of an application.
    #[serde(rename = "Architecture")]
    pub architecture: Option<Architecture>,
    /// EMR release label.
    #[serde(rename = "ReleaseLabel")]
    pub release_label: String,
    /// Tag map with key and value
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Maximum allowed cumulative resources for an Application. No new resources will be created once the limit is hit.
    #[serde(rename = "MaximumCapacity")]
    pub maximum_capacity: Option<MaximumAllowedResources>,
    /// The type of the application
    #[serde(rename = "Type")]
    pub ty: String,
    /// Configuration for Auto Stop of Application.
    #[serde(rename = "AutoStopConfiguration")]
    pub auto_stop_configuration: Option<AutoStopConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkerTypeSpecifications")]
    pub worker_type_specifications: Option<WorkerTypeSpecificationInputMap>,
    /// No documentation provided by AWS
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfigurationInput>,
    /// User friendly Application name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Initial capacity initialized when an Application is started.
    #[serde(rename = "InitialCapacity")]
    pub initial_capacity: Option<InitialCapacityConfigMap>,
    /// Configuration for Auto Start of Application.
    #[serde(rename = "AutoStartConfiguration")]
    pub auto_start_configuration: Option<AutoStartConfiguration>,
    /// Network Configuration for customer VPC connectivity.
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct ImageConfigurationInput {
    #[serde(rename = "ImageUri")]
    pub image_uri: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MaximumAllowedResources {
    #[serde(rename = "Memory")]
    pub memory: MemorySize,
    #[serde(rename = "Disk")]
    pub disk: Option<DiskSize>,
    #[serde(rename = "Cpu")]
    pub cpu: CpuSize,

}

#[derive(serde::Serialize, Default)]
pub struct InitialCapacityConfigMap {

}

#[derive(serde::Serialize, Default)]
pub struct InitialCapacityConfig {
    #[serde(rename = "WorkerCount")]
    pub worker_count: usize,
    #[serde(rename = "WorkerConfiguration")]
    pub worker_configuration: WorkerConfiguration,

}
pub type DiskSize = String;pub type Architecture = String;pub type SecurityGroupId = String;
#[derive(serde::Serialize, Default)]
pub struct InitialCapacityConfigKeyValuePair {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: InitialCapacityConfig,

}

#[derive(serde::Serialize, Default)]
pub struct AutoStartConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct WorkerConfiguration {
    #[serde(rename = "Disk")]
    pub disk: Option<DiskSize>,
    #[serde(rename = "Cpu")]
    pub cpu: CpuSize,
    #[serde(rename = "Memory")]
    pub memory: MemorySize,

}
pub type SubnetId = String;pub type CpuSize = String;
#[derive(serde::Serialize, Default)]
pub struct WorkerTypeSpecificationInput {
    #[serde(rename = "ImageConfiguration")]
    pub image_configuration: Option<ImageConfigurationInput>,

}

#[derive(serde::Serialize, Default)]
pub struct WorkerTypeSpecificationInputMap {

}

#[derive(serde::Serialize, Default)]
pub struct AutoStopConfiguration {
    #[serde(rename = "IdleTimeoutMinutes")]
    pub idle_timeout_minutes: Option<usize>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}
pub type MemorySize = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<SubnetId>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<SecurityGroupId>>,

}


}
