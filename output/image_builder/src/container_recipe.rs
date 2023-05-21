

/// Creates a new container recipe. Container recipes define how images are configured, 			tested, and assessed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnContainerRecipe {


    /// 
    /// The description of the container recipe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Identifies which KMS key is used to encrypt the container image for distribution to 			the target Region.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// Specifies the operating system version for the base image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageOsVersionOverride")]
    pub image_os_version_override: Option<String>,


    /// 
    /// The name of the container recipe.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[-_A-Za-z-0-9][-_A-Za-z0-9 ]{1,126}[-_A-Za-z-0-9]$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Build and test components that are included in the container recipe. 			Recipes require a minimum of one build component, and can 			have a maximum of 20 build and test components in any combination.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ComponentConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Components")]
    pub components: Vec<ComponentConfiguration>,


    /// 
    /// Dockerfiles are text documents that are used to build Docker containers, and ensure 			that they contain all of the elements required by the application running inside. The 			template data consists of contextual variables where Image Builder places build information or 			scripts, based on your container image recipe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DockerfileTemplateData")]
    pub dockerfile_template_data: Option<String>,


    /// 
    /// The base image for the container recipe.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentImage")]
    pub parent_image: String,


    /// 
    /// The S3 URI for the Dockerfile that will be used to build your container image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DockerfileTemplateUri")]
    pub dockerfile_template_uri: Option<String>,


    /// 
    /// Tags that are attached to the container recipe.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// A group of options that can be used to configure an instance for building and testing 			container images.
    /// 
    /// Required: No
    ///
    /// Type: InstanceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceConfiguration")]
    pub instance_configuration: Option<InstanceConfiguration>,


    /// 
    /// Specifies the operating system platform when you use a custom base image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlatformOverride")]
    pub platform_override: Option<String>,


    /// 
    /// The destination repository for the container image.
    /// 
    /// Required: Yes
    ///
    /// Type: TargetContainerRepository
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetRepository")]
    pub target_repository: TargetContainerRepository,


    /// 
    /// The semantic version of the container recipe.
    /// 
    /// NoteThe semantic version has four nodes: <major>.<minor>.<patch>/<build>. 	You can assign values for the first three, and can filter on all of them.        Assignment: For the first three nodes you can assign any positive integer value, including 	zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the 	build number to the fourth node.        Patterns: You can use any numeric pattern that adheres to the assignment requirements for 	the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or 	a date, such as 2021.01.01.        Filtering: With semantic versioning, you have the flexibility to use wildcards (x) 	to specify the most recent versions or nodes when selecting the base image or components for your 	recipe. When you use a wildcard in any node, all nodes to the right of the first wildcard must also be 	wildcards.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[0-9]+\.[0-9]+\.[0-9]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: String,


    /// 
    /// The working directory for use during build and test workflows.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,


    /// 
    /// Specifies the type of container, such as Docker.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DOCKER
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerType")]
    pub container_type: ContainerRecipeContainerTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ContainerRecipeContainerTypeEnum {

    /// DOCKER
    #[serde(rename = "DOCKER")]
    Docker,

}

impl Default for ContainerRecipeContainerTypeEnum {
    fn default() -> Self {
        ContainerRecipeContainerTypeEnum::Docker
    }
}


impl cfn_resources::CfnResource for CfnContainerRecipe {
    fn type_string() -> &'static str {
        "AWS::ImageBuilder::ContainerRecipe"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Defines a custom base AMI and block device mapping configurations of an instance    used for building and testing container images.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceConfiguration {


    /// 
    /// The AMI ID to use as the base image for a container build and test instance. If not 			specified, Image Builder will use the appropriate ECS-optimized AMI as a base image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Image")]
    pub image: Option<String>,


    /// 
    /// Defines the block devices to attach for building an instance from this Image Builder 			AMI.
    /// 
    /// Required: No
    ///
    /// Type: List of InstanceBlockDeviceMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,

}




/// Defines block device mappings for the instance used to configure your image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceBlockDeviceMapping {


    /// 
    /// Use to remove a mapping from the base image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 0
    ///
    /// Update requires: Replacement
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,


    /// 
    /// The device to which these mappings apply.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,


    /// 
    /// Use to manage Amazon EBS-specific configuration for this mapping.
    /// 
    /// Required: No
    ///
    /// Type: EbsInstanceBlockDeviceSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsInstanceBlockDeviceSpecification>,


    /// 
    /// Use to manage instance ephemeral devices.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,

}




/// Amazon EBS-specific block device mapping specifications.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbsInstanceBlockDeviceSpecification {


    /// 
    /// Use to configure delete on termination of the associated device.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// Use to override the device's volume size.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16000
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// 
    /// Use to configure device IOPS.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 100
    ///
    /// Maximum: 64000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// For GP3 volumes only – The throughput in MiB/s 			that the volume supports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 125
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Throughput")]
    pub throughput: Option<i64>,


    /// 
    /// Use to configure device encryption.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// Use to configure the KMS key to use when encrypting the device.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The snapshot that defines the device contents.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// 
    /// Use to override the device's volume type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | io2 | sc1 | st1 | standard
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<EbsInstanceBlockDeviceSpecificationVolumeTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EbsInstanceBlockDeviceSpecificationVolumeTypeEnum {

    /// gp2
    #[serde(rename = "gp2")]
    Gp2,

    /// gp3
    #[serde(rename = "gp3")]
    Gp3,

    /// io1
    #[serde(rename = "io1")]
    Io1,

    /// io2
    #[serde(rename = "io2")]
    Io2,

    /// sc1
    #[serde(rename = "sc1")]
    Sc1,

    /// st1
    #[serde(rename = "st1")]
    St1,

    /// standard
    #[serde(rename = "standard")]
    Standard,

}

impl Default for EbsInstanceBlockDeviceSpecificationVolumeTypeEnum {
    fn default() -> Self {
        EbsInstanceBlockDeviceSpecificationVolumeTypeEnum::Gp2
    }
}



/// The container repository where the output container image is stored.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetContainerRepository {


    /// 
    /// Specifies the service in which this image was registered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ECR
    ///
    /// Update requires: Replacement
    #[serde(rename = "Service")]
    pub service: Option<TargetContainerRepositoryServiceEnum>,


    /// 
    /// The name of the container repository where the output container image is stored. This 			name is prefixed by the repository location.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TargetContainerRepositoryServiceEnum {

    /// ECR
    #[serde(rename = "ECR")]
    Ecr,

}

impl Default for TargetContainerRepositoryServiceEnum {
    fn default() -> Self {
        TargetContainerRepositoryServiceEnum::Ecr
    }
}



/// Configuration details of the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ComponentParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ComponentParameter>>,


    /// 
    /// The Amazon Resource Name (ARN) of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws[^:]*:imagebuilder:[^:]+:(?:[0-9]{12}|aws):component/[a-z0-9-_]+/(?:(?:([0-9]+|x)\.([0-9]+|x)\.([0-9]+|x))|(?:[0-9]+\.[0-9]+\.[0-9]+/[0-9]+))$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,

}




/// The ComponentParameter property type specifies Property description not available. for an AWS::ImageBuilder::ContainerRecipe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentParameter {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: Vec<String>,

}


