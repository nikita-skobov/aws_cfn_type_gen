/// A distribution configuration allows you to specify the name and description of your 			output AMI, authorize other AWS accounts to launch the AMI, and replicate the AMI to other 			AWS Regions. It also allows you to export the AMI to Amazon S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDistributionConfiguration {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    pub name: cfn_resources::StrVal,

    ///
    /// The tags of this distribution configuration.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnDistributionConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::ImageBuilder::DistributionConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Define and configure the output AMIs of the pipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmiDistributionConfiguration {
    ///
    /// The tags to apply to AMIs distributed to this Region.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmiTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_tags: Option<std::collections::HashMap<String, String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Launch permissions can be used to configure which AWS accounts can use the AMI to 			launch instances.
    ///
    /// Required: No
    ///
    /// Type: LaunchPermissionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchPermissionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_permission_configuration: Option<LaunchPermissionConfiguration>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for AmiDistributionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.launch_permission_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 127 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 127",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.target_account_ids {
            if the_val.len() > 1536 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_account_ids'. {} is greater than 1536",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Container distribution settings for encryption, licensing, and sharing in a specific 			Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ContainerDistributionConfiguration {
    ///
    /// Tags that are attached to the container distribution configuration.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_tags: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The destination repository for the container distribution configuration.
    ///
    /// Required: No
    ///
    /// Type: TargetContainerRepository
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetRepository")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_repository: Option<TargetContainerRepository>,
}

impl cfn_resources::CfnResource for ContainerDistributionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.target_repository
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The distribution configuration distribution defines the settings for a specific Region 			in the Distribution Configuration. You must specify whether the distribution is for an AMI 			or a container image. To do so, include exactly one of the following data types for your 			distribution:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Distribution {
    ///
    /// The specific AMI settings, such as launch permissions and AMI tags. For details, 			see example schema below.
    ///
    /// Required: No
    ///
    /// Type: AmiDistributionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmiDistributionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_distribution_configuration: Option<AmiDistributionConfiguration>,

    ///
    /// Container distribution settings for encryption, licensing, and sharing 			in a specific Region. For details, see example schema below.
    ///
    /// Required: No
    ///
    /// Type: ContainerDistributionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerDistributionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_distribution_configuration: Option<ContainerDistributionConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of FastLaunchConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FastLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_launch_configurations: Option<Vec<FastLaunchConfiguration>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_configurations: Option<Vec<LaunchTemplateConfiguration>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arns: Option<Vec<String>>,

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
    pub region: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Distribution {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ami_distribution_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.container_distribution_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.launch_template_configurations {
            if the_val.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'launch_template_configurations'. {} is greater than 100", the_val.len()));
            }
        }

        if let Some(the_val) = &self.license_configuration_arns {
            if the_val.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'license_configuration_arns'. {} is greater than 50", the_val.len()));
            }
        }

        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'region'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'region'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The FastLaunchConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FastLaunchConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FastLaunchLaunchTemplateSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<FastLaunchLaunchTemplateSpecification>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxParallelLaunches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_launches: Option<i64>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FastLaunchSnapshotConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_configuration: Option<FastLaunchSnapshotConfiguration>,
}

impl cfn_resources::CfnResource for FastLaunchConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.launch_template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.snapshot_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FastLaunchLaunchTemplateSpecification property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FastLaunchLaunchTemplateSpecification {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FastLaunchLaunchTemplateSpecification {
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

/// The FastLaunchSnapshotConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::DistributionConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FastLaunchSnapshotConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetResourceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_count: Option<i64>,
}

impl cfn_resources::CfnResource for FastLaunchSnapshotConfiguration {
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

/// Describes the configuration for a launch permission. The launch permission 			modification request is sent to the Amazon EC2 				ModifyImageAttribute API on behalf of the user for each Region they have 			selected to distribute the AMI. To make an AMI public, set the launch permission 			authorized accounts to all. See the examples for making an AMI public at 				Amazon EC2 				ModifyImageAttribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchPermissionConfiguration {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arns: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for LaunchPermissionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.organization_arns {
            if the_val.len() > 25 as _ {
                return Err(format!(
                    "Max validation failed on field 'organization_arns'. {} is greater than 25",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.organizational_unit_arns {
            if the_val.len() > 25 as _ {
                return Err(format!("Max validation failed on field 'organizational_unit_arns'. {} is greater than 25", the_val.len()));
            }
        }

        if let Some(the_val) = &self.user_ids {
            if the_val.len() > 1536 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_ids'. {} is greater than 1536",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Identifies an Amazon EC2 launch template to use for a specific account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplateConfiguration {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<cfn_resources::StrVal>,

    ///
    /// Set the specified Amazon EC2 launch template as the default launch template for the 			specified account.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_default_version: Option<bool>,
}

impl cfn_resources::CfnResource for LaunchTemplateConfiguration {
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

/// The container repository where the output container image is stored.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetContainerRepository {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<TargetContainerRepositoryServiceEnum>,
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

impl cfn_resources::CfnResource for TargetContainerRepository {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.repository_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'repository_name'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.repository_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'repository_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
