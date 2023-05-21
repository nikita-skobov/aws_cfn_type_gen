

/// A distribution configuration allows you to specify the name and description of your 			output AMI, authorize other AWS accounts to launch the AMI, and replicate the AMI to other 			AWS Regions. It also allows you to export the AMI to Amazon S3.
#[derive(Default, serde::Serialize)]
pub struct CfnDistributionConfiguration {


    /// 
    /// The name of this distribution configuration.
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
    /// The distributions of this distribution configuration formatted as an array of 			Distribution objects.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Distribution
    ///
    /// Update requires: No interruption
    #[serde(rename = "Distributions")]
    pub distributions: Vec<Distribution>,


    /// 
    /// The tags of this distribution configuration.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The description of this distribution configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// Define and configure the output AMIs of the pipeline.
#[derive(Default, serde::Serialize)]
pub struct AmiDistributionConfiguration {


    /// 
    /// The ID of an account to which you want to distribute an image.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1536
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAccountIds")]
    pub target_account_ids: Option<Vec<String>>,


    /// 
    /// The name of the output AMI.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 127
    ///
    /// Pattern: ^[-_A-Za-z0-9{][-_A-Za-z0-9\s:{}\.]+[-_A-Za-z0-9}]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The tags to apply to AMIs distributed to this Region.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmiTags")]
    pub ami_tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Launch permissions can be used to configure which AWS accounts can use the AMI to 			launch instances.
    /// 
    /// Required: No
    ///
    /// Type: LaunchPermissionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchPermissionConfiguration")]
    pub launch_permission_configuration: Option<LaunchPermissionConfiguration>,


    /// 
    /// The description of the AMI distribution configuration. Minimum and maximum length are 			in characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The KMS key identifier used to encrypt the distributed image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


/// The distribution configuration distribution defines the settings for a specific Region 			in the Distribution Configuration. You must specify whether the distribution is for an AMI 			or a container image. To do so, include exactly one of the following data types for your 			distribution:
#[derive(Default, serde::Serialize)]
pub struct Distribution {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of FastLaunchConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FastLaunchConfigurations")]
    pub fast_launch_configurations: Option<Vec<FastLaunchConfiguration>>,


    /// 
    /// The License Manager Configuration to associate with the AMI in the specified Region. 			For more information, see the 				LicenseConfiguration API.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "LicenseConfigurationArns")]
    pub license_configuration_arns: Option<Vec<String>>,


    /// 
    /// Container distribution settings for encryption, licensing, and sharing 			in a specific Region. For details, see example schema below.
    /// 
    /// Required: No
    ///
    /// Type: ContainerDistributionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerDistributionConfiguration")]
    pub container_distribution_configuration: Option<ContainerDistributionConfiguration>,


    /// 
    /// The specific AMI settings, such as launch permissions and AMI tags. For details, 			see example schema below.
    /// 
    /// Required: No
    ///
    /// Type: AmiDistributionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmiDistributionConfiguration")]
    pub ami_distribution_configuration: Option<AmiDistributionConfiguration>,


    /// 
    /// The target Region for the Distribution Configuration. For example, 			eu-west-1.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: String,


    /// 
    /// A group of launchTemplateConfiguration settings that apply to image distribution for 			specified accounts.
    /// 
    /// Required: No
    ///
    /// Type: List of LaunchTemplateConfiguration
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateConfigurations")]
    pub launch_template_configurations: Option<Vec<LaunchTemplateConfiguration>>,

}


/// The FastLaunchLaunchTemplateSpecification property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Default, serde::Serialize)]
pub struct FastLaunchLaunchTemplateSpecification {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateVersion")]
    pub launch_template_version: Option<String>,

}


/// Describes the configuration for a launch permission. The launch permission 			modification request is sent to the Amazon EC2 				ModifyImageAttribute API on behalf of the user for each Region they have 			selected to distribute the AMI. To make an AMI public, set the launch permission 			authorized accounts to all. See the examples for making an AMI public at 				Amazon EC2 				ModifyImageAttribute.
#[derive(Default, serde::Serialize)]
pub struct LaunchPermissionConfiguration {


    /// 
    /// The AWS account ID.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1536
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserIds")]
    pub user_ids: Option<Vec<String>>,


    /// 
    /// The ARN for an AWS Organizations organizational unit (OU) that you want to share your AMI with. 			For more information about key concepts for AWS Organizations, see AWS Organizations 				terminology and concepts.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationalUnitArns")]
    pub organizational_unit_arns: Option<Vec<String>>,


    /// 
    /// The name of the group.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserGroups")]
    pub user_groups: Option<Vec<String>>,


    /// 
    /// The ARN for an AWS Organization that you want to share your AMI with. For more 			information, see What is 				AWS Organizations?.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrganizationArns")]
    pub organization_arns: Option<Vec<String>>,

}


/// The container repository where the output container image is stored.
#[derive(Default, serde::Serialize)]
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
    /// Update requires: No interruption
    #[serde(rename = "Service")]
    pub service: Option<String>,


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
    /// Update requires: No interruption
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,

}


/// The FastLaunchConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Default, serde::Serialize)]
pub struct FastLaunchConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxParallelLaunches")]
    pub max_parallel_launches: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FastLaunchLaunchTemplateSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<FastLaunchLaunchTemplateSpecification>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FastLaunchSnapshotConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotConfiguration")]
    pub snapshot_configuration: Option<FastLaunchSnapshotConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,

}


/// The FastLaunchSnapshotConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Default, serde::Serialize)]
pub struct FastLaunchSnapshotConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetResourceCount")]
    pub target_resource_count: Option<i64>,

}


/// Container distribution settings for encryption, licensing, and sharing in a specific 			Region.
#[derive(Default, serde::Serialize)]
pub struct ContainerDistributionConfiguration {


    /// 
    /// The destination repository for the container distribution configuration.
    /// 
    /// Required: No
    ///
    /// Type: TargetContainerRepository
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetRepository")]
    pub target_repository: Option<TargetContainerRepository>,


    /// 
    /// The description of the container distribution configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Tags that are attached to the container distribution configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,

}


/// Identifies an Amazon EC2 launch template to use for a specific account.
#[derive(Default, serde::Serialize)]
pub struct LaunchTemplateConfiguration {


    /// 
    /// Set the specified Amazon EC2 launch template as the default launch template for the 			specified account.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetDefaultVersion")]
    pub set_default_version: Option<bool>,


    /// 
    /// The account ID that this configuration applies to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,


    /// 
    /// Identifies the Amazon EC2 launch template to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^lt-[a-z0-9-_]{17}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,

}
