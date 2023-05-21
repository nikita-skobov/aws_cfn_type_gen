

/// Specifies a runtime environment for a given runtime engine.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEnvironment {


    /// 
    /// The target platform for the runtime environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: bluage | microfocus
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineType")]
    pub engine_type: EnvironmentEngineTypeEnum,


    /// 
    /// Defines the storage configuration for a runtime environment.
    /// 
    /// Required: No
    ///
    /// Type: List of StorageConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "StorageConfigurations")]
    pub storage_configurations: Option<Vec<StorageConfiguration>>,


    /// 
    /// Defines the details of a high availability configuration.
    /// 
    /// Required: No
    ///
    /// Type: HighAvailabilityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HighAvailabilityConfig")]
    pub high_availability_config: Option<HighAvailabilityConfig>,


    /// 
    /// The version of the runtime engine.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,10}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The list of subnets associated with the VPC for this runtime environment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,


    /// 
    /// The list of security groups for the VPC associated with this runtime environment.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The description of the runtime environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the runtime environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [A-Za-z0-9][A-Za-z0-9_\-]{1,59}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Configures the maintenance window you want for the runtime environment. If you do not     provide a value, a random system-generated value will be assigned.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,50}
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,


    /// 
    /// The identifier of a customer managed key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// Specifies whether the runtime environment is publicly accessible.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: Option<bool>,


    /// 
    /// The instance type of the runtime environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,20}
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EnvironmentEngineTypeEnum {

    /// bluage
    #[serde(rename = "bluage")]
    Bluage,

    /// microfocus
    #[serde(rename = "microfocus")]
    Microfocus,

}

impl Default for EnvironmentEngineTypeEnum {
    fn default() -> Self {
        EnvironmentEngineTypeEnum::Bluage
    }
}


impl cfn_resources::CfnResource for CfnEnvironment {
    fn type_string() -> &'static str {
        "AWS::M2::Environment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Defines the storage configuration for a runtime environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StorageConfiguration {


    /// 
    /// Defines the storage configuration for an Amazon FSx file system.
    /// 
    /// Required: No
    ///
    /// Type: FsxStorageConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Fsx")]
    pub fsx: Option<FsxStorageConfiguration>,


    /// 
    /// Defines the storage configuration for an Amazon EFS file system.
    /// 
    /// Required: No
    ///
    /// Type: EfsStorageConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Efs")]
    pub efs: Option<EfsStorageConfiguration>,

}




/// Defines the details of a high availability configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HighAvailabilityConfig {


    /// 
    /// The number of instances in a high availability configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredCapacity")]
    pub desired_capacity: i64,

}




/// Defines the storage configuration for an Amazon EFS file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EfsStorageConfiguration {


    /// 
    /// The file system identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,200}
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,


    /// 
    /// The mount point for the file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,200}
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountPoint")]
    pub mount_point: String,

}




/// Defines the storage configuration for an Amazon FSx file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FsxStorageConfiguration {


    /// 
    /// The mount point for the file system.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,200}
    ///
    /// Update requires: Replacement
    #[serde(rename = "MountPoint")]
    pub mount_point: String,


    /// 
    /// The file system identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \S{1,200}
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,

}


