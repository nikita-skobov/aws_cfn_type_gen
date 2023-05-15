
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// No documentation provided by AWS
    #[serde(rename = "EngineType")]
    pub engine_type: EngineType,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// The ID or the Amazon Resource Name (ARN) of the customer managed KMS Key used for encrypting application-related resources.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Definition")]
    pub definition: Definition,

}

pub type EngineType = String;
#[derive(serde::Serialize, Default)]
pub struct TagMap {

}

#[derive(serde::Serialize, Default)]
pub struct Definition {

}


}

pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// The name of the environment.
    #[serde(rename = "Name")]
    pub name: String,
    /// The ID or the Amazon Resource Name (ARN) of the customer managed KMS Key used for encrypting environment-related resources.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// Tags associated to this environment.
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// The list of security groups for the VPC associated with this environment.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// The unique identifiers of the subnets assigned to this runtime environment.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    /// The target platform for the environment.
    #[serde(rename = "EngineType")]
    pub engine_type: EngineType,
    /// The type of instance underlying the environment.
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// The version of the runtime engine for the environment.
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// The storage configurations defined for the runtime environment.
    #[serde(rename = "StorageConfigurations")]
    pub storage_configurations: Option<Vec<StorageConfiguration>>,
    /// Specifies whether the environment is publicly accessible.
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,
    /// Configures a desired maintenance window for the environment. If you do not provide a value, a random system-generated value will be assigned.
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The description of the environment.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Defines the details of a high availability configuration.
    #[serde(rename = "HighAvailabilityConfig")]
    pub high_availability_config: Option<HighAvailabilityConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct EfsStorageConfiguration {
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "MountPoint")]
    pub mount_point: String,

}

#[derive(serde::Serialize, Default)]
pub struct HighAvailabilityConfig {
    #[serde(rename = "DesiredCapacity")]
    pub desired_capacity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct FsxStorageConfiguration {
    #[serde(rename = "MountPoint")]
    pub mount_point: String,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct StorageConfiguration {

}

#[derive(serde::Serialize, Default)]
pub struct TagMap {

}
pub type EngineType = String;

}
