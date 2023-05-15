
pub mod cfn_component {

#[derive(serde::Serialize, Default)]
pub struct CfnComponent {
    /// The operating system (OS) version supported by the component.
    #[serde(rename = "SupportedOsVersions")]
    pub supported_os_versions: Option<Vec<String>>,
    /// The data of the component.
    #[serde(rename = "Data")]
    pub data: Option<String>,
    /// The description of the component.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The version of the component.
    #[serde(rename = "Version")]
    pub version: String,
    /// The KMS key identifier used to encrypt the component.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// The tags associated with the component.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The change description of the component.
    #[serde(rename = "ChangeDescription")]
    pub change_description: Option<String>,
    /// The name of the component.
    #[serde(rename = "Name")]
    pub name: String,
    /// The platform of the component.
    #[serde(rename = "Platform")]
    pub platform: String,
    /// The uri of the component.
    #[serde(rename = "Uri")]
    pub uri: Option<String>,

}



}

pub mod cfn_container_recipe {

#[derive(serde::Serialize, Default)]
pub struct CfnContainerRecipe {
    /// Specifies the type of container, such as Docker.
    #[serde(rename = "ContainerType")]
    pub container_type: Option<String>,
    /// The S3 URI for the Dockerfile that will be used to build your container image.
    #[serde(rename = "DockerfileTemplateUri")]
    pub dockerfile_template_uri: Option<String>,
    /// The semantic version of the container recipe (<major>.<minor>.<patch>).
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// The name of the container recipe.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Specifies the operating system platform when you use a custom source image.
    #[serde(rename = "PlatformOverride")]
    pub platform_override: Option<String>,
    /// The description of the container recipe.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// A group of options that can be used to configure an instance for building and testing container images.
    #[serde(rename = "InstanceConfiguration")]
    pub instance_configuration: Option<InstanceConfiguration>,
    /// The destination repository for the container image.
    #[serde(rename = "TargetRepository")]
    pub target_repository: Option<TargetContainerRepository>,
    /// Components for build and test that are included in the container recipe.
    #[serde(rename = "Components")]
    pub components: Option<Vec<ComponentConfiguration>>,
    /// The source image for the container recipe.
    #[serde(rename = "ParentImage")]
    pub parent_image: Option<String>,
    /// Identifies which KMS key is used to encrypt the container image.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// Tags that are attached to the container recipe.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The working directory to be used during build and test workflows.
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
    /// Dockerfiles are text documents that are used to build Docker containers, and ensure that they contain all of the elements required by the application running inside. The template data consists of contextual variables where Image Builder places build information or scripts, based on your container image recipe.
    #[serde(rename = "DockerfileTemplateData")]
    pub dockerfile_template_data: Option<String>,
    /// Specifies the operating system version for the source image.
    #[serde(rename = "ImageOsVersionOverride")]
    pub image_os_version_override: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EbsInstanceBlockDeviceSpecification {
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentConfiguration {
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ComponentParameter>>,
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceBlockDeviceMapping {
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsInstanceBlockDeviceSpecification>,
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceConfiguration {
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetContainerRepository {
    #[serde(rename = "Service")]
    pub service: Option<String>,
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,

}


}

pub mod cfn_distribution_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnDistributionConfiguration {
    /// The name of the distribution configuration.
    #[serde(rename = "Name")]
    pub name: String,
    /// The description of the distribution configuration.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The distributions of the distribution configuration.
    #[serde(rename = "Distributions")]
    pub distributions: Vec<Distribution>,
    /// The tags associated with the component.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct TargetContainerRepository {
    #[serde(rename = "Service")]
    pub service: Option<String>,
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FastLaunchLaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateVersion")]
    pub launch_template_version: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchPermissionConfiguration {
    #[serde(rename = "UserIds")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "OrganizationalUnitArns")]
    pub organizational_unit_arns: Option<Vec<String>>,
    #[serde(rename = "UserGroups")]
    pub user_groups: Option<Vec<String>>,
    #[serde(rename = "OrganizationArns")]
    pub organization_arns: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Distribution {
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "ContainerDistributionConfiguration")]
    pub container_distribution_configuration: Option<ContainerDistributionConfiguration>,
    #[serde(rename = "AmiDistributionConfiguration")]
    pub ami_distribution_configuration: Option<AmiDistributionConfiguration>,
    #[serde(rename = "LicenseConfigurationArns")]
    pub license_configuration_arns: Option<Vec<LicenseConfigurationArn>>,
    #[serde(rename = "LaunchTemplateConfigurations")]
    pub launch_template_configurations: Option<Vec<LaunchTemplateConfiguration>>,
    #[serde(rename = "FastLaunchConfigurations")]
    pub fast_launch_configurations: Option<Vec<FastLaunchConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerDistributionConfiguration {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,
    #[serde(rename = "TargetRepository")]
    pub target_repository: Option<TargetContainerRepository>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateConfiguration {
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "SetDefaultVersion")]
    pub set_default_version: Option<bool>,

}
pub type LicenseConfigurationArn = String;
#[derive(serde::Serialize, Default)]
pub struct FastLaunchSnapshotConfiguration {
    #[serde(rename = "TargetResourceCount")]
    pub target_resource_count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AmiDistributionConfiguration {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LaunchPermissionConfiguration")]
    pub launch_permission_configuration: Option<LaunchPermissionConfiguration>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "AmiTags")]
    pub ami_tags: Option<()>,
    #[serde(rename = "TargetAccountIds")]
    pub target_account_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct FastLaunchConfiguration {
    #[serde(rename = "SnapshotConfiguration")]
    pub snapshot_configuration: Option<FastLaunchSnapshotConfiguration>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<FastLaunchLaunchTemplateSpecification>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "MaxParallelLaunches")]
    pub max_parallel_launches: Option<usize>,

}


}

pub mod cfn_image {

#[derive(serde::Serialize, Default)]
pub struct CfnImage {
    /// The image tests configuration used when creating this image.
    #[serde(rename = "ImageTestsConfiguration")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// The Amazon Resource Name (ARN) of the image recipe that defines how images are configured, tested, and assessed.
    #[serde(rename = "ImageRecipeArn")]
    pub image_recipe_arn: Option<String>,
    /// The Amazon Resource Name (ARN) of the distribution configuration.
    #[serde(rename = "DistributionConfigurationArn")]
    pub distribution_configuration_arn: Option<String>,
    /// Contains settings for vulnerability scans.
    #[serde(rename = "ImageScanningConfiguration")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// The tags associated with the image.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The Amazon Resource Name (ARN) of the container recipe that defines how images are configured and tested.
    #[serde(rename = "ContainerRecipeArn")]
    pub container_recipe_arn: Option<String>,
    /// The Amazon Resource Name (ARN) of the infrastructure configuration.
    #[serde(rename = "InfrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: Option<String>,
    /// Collects additional information about the image being created, including the operating system (OS) version and package list.
    #[serde(rename = "EnhancedImageMetadataEnabled")]
    pub enhanced_image_metadata_enabled: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct EcrConfiguration {
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ImageScanningConfiguration {
    #[serde(rename = "ImageScanningEnabled")]
    pub image_scanning_enabled: Option<bool>,
    #[serde(rename = "EcrConfiguration")]
    pub ecr_configuration: Option<EcrConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ImageTestsConfiguration {
    #[serde(rename = "TimeoutMinutes")]
    pub timeout_minutes: Option<usize>,
    #[serde(rename = "ImageTestsEnabled")]
    pub image_tests_enabled: Option<bool>,

}


}

pub mod cfn_image_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnImagePipeline {
    /// Contains settings for vulnerability scans.
    #[serde(rename = "ImageScanningConfiguration")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    /// The Amazon Resource Name (ARN) of the container recipe that defines how images are configured and tested.
    #[serde(rename = "ContainerRecipeArn")]
    pub container_recipe_arn: Option<String>,
    /// The schedule of the image pipeline.
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,
    /// The Amazon Resource Name (ARN) of the distribution configuration associated with this image pipeline.
    #[serde(rename = "DistributionConfigurationArn")]
    pub distribution_configuration_arn: Option<String>,
    /// The Amazon Resource Name (ARN) of the infrastructure configuration associated with this image pipeline.
    #[serde(rename = "InfrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: Option<String>,
    /// The status of the image pipeline.
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// The name of the image pipeline.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The description of the image pipeline.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The tags of this image pipeline.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The image tests configuration of the image pipeline.
    #[serde(rename = "ImageTestsConfiguration")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,
    /// Collects additional information about the image being created, including the operating system (OS) version and package list.
    #[serde(rename = "EnhancedImageMetadataEnabled")]
    pub enhanced_image_metadata_enabled: Option<bool>,
    /// The Amazon Resource Name (ARN) of the image recipe that defines how images are configured, tested, and assessed.
    #[serde(rename = "ImageRecipeArn")]
    pub image_recipe_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ImageTestsConfiguration {
    #[serde(rename = "TimeoutMinutes")]
    pub timeout_minutes: Option<usize>,
    #[serde(rename = "ImageTestsEnabled")]
    pub image_tests_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Schedule {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "PipelineExecutionStartCondition")]
    pub pipeline_execution_start_condition: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ImageScanningConfiguration {
    #[serde(rename = "ImageScanningEnabled")]
    pub image_scanning_enabled: Option<bool>,
    #[serde(rename = "EcrConfiguration")]
    pub ecr_configuration: Option<EcrConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct EcrConfiguration {
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,

}


}

pub mod cfn_image_recipe {

#[derive(serde::Serialize, Default)]
pub struct CfnImageRecipe {
    /// The version of the image recipe.
    #[serde(rename = "Version")]
    pub version: String,
    /// The components of the image recipe.
    #[serde(rename = "Components")]
    pub components: Vec<ComponentConfiguration>,
    /// The name of the image recipe.
    #[serde(rename = "Name")]
    pub name: String,
    /// The block device mappings to apply when creating images from this recipe.
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,
    /// The parent image of the image recipe.
    #[serde(rename = "ParentImage")]
    pub parent_image: String,
    /// Specify additional settings and launch scripts for your build instances.
    #[serde(rename = "AdditionalInstanceConfiguration")]
    pub additional_instance_configuration: Option<AdditionalInstanceConfiguration>,
    /// The tags of the image recipe.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The working directory to be used during build and test workflows.
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
    /// The description of the image recipe.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ComponentParameter {
    #[serde(rename = "Value")]
    pub value: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct EbsInstanceBlockDeviceSpecification {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceBlockDeviceMapping {
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsInstanceBlockDeviceSpecification>,
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ComponentConfiguration {
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ComponentParameter>>,
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalInstanceConfiguration {
    #[serde(rename = "SystemsManagerAgent")]
    pub systems_manager_agent: Option<SystemsManagerAgent>,
    #[serde(rename = "UserDataOverride")]
    pub user_data_override: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SystemsManagerAgent {
    #[serde(rename = "UninstallAfterBuild")]
    pub uninstall_after_build: Option<bool>,

}


}

pub mod cfn_infrastructure_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnInfrastructureConfiguration {
    /// The instance types of the infrastructure configuration.
    #[serde(rename = "InstanceTypes")]
    pub instance_types: Option<Vec<String>>,
    /// The terminate instance on failure configuration of the infrastructure configuration.
    #[serde(rename = "TerminateInstanceOnFailure")]
    pub terminate_instance_on_failure: Option<bool>,
    /// The subnet ID of the infrastructure configuration.
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// The name of the infrastructure configuration.
    #[serde(rename = "Name")]
    pub name: String,
    /// The EC2 key pair of the infrastructure configuration..
    #[serde(rename = "KeyPair")]
    pub key_pair: Option<String>,
    /// The SNS Topic Amazon Resource Name (ARN) of the infrastructure configuration.
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<String>,
    /// The logging configuration of the infrastructure configuration.
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,
    /// The tags associated with the component.
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// The description of the infrastructure configuration.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The security group IDs of the infrastructure configuration.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// The instance profile of the infrastructure configuration.
    #[serde(rename = "InstanceProfileName")]
    pub instance_profile_name: String,
    /// The instance metadata option settings for the infrastructure configuration.
    #[serde(rename = "InstanceMetadataOptions")]
    pub instance_metadata_options: Option<InstanceMetadataOptions>,
    /// The tags attached to the resource created by Image Builder.
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct InstanceMetadataOptions {
    #[serde(rename = "HttpTokens")]
    pub http_tokens: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TagMap {
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Logging {
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<S3Logs>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Logs {
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

}


}
