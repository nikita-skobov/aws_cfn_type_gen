/// An image build version. An image is a customized, secure, and up-to-date “golden” server     image that is pre-installed and pre-configured with software and settings to meet specific     IT standards.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImage {
    ///
    /// The Amazon Resource Name (ARN) of the container recipe that is used for this 			pipeline.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerRecipeArn")]
    pub container_recipe_arn: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the distribution configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws[^:]*:imagebuilder:[^:]+:(?:[0-9]{12}|aws):(?:image-recipe|container-recipe|infrastructure-configuration|distribution-configuration|component|image|image-pipeline|workflow\/(?:build|test|distribution))/[a-z0-9-_]+(?:/(?:(?:x|[0-9]+)\.(?:x|[0-9]+)\.(?:x|[0-9]+))(?:/[0-9]+)?)?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DistributionConfigurationArn")]
    pub distribution_configuration_arn: Option<String>,

    ///
    /// Indicates whether Image Builder collects additional information about the image, such as the 			operating system (OS) version and package list.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnhancedImageMetadataEnabled")]
    pub enhanced_image_metadata_enabled: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) of the image recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws[^:]*:imagebuilder:[^:]+:(?:[0-9]{12}|aws):(?:image-recipe|container-recipe|infrastructure-configuration|distribution-configuration|component|image|image-pipeline|workflow\/(?:build|test|distribution))/[a-z0-9-_]+(?:/(?:(?:x|[0-9]+)\.(?:x|[0-9]+)\.(?:x|[0-9]+))(?:/[0-9]+)?)?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageRecipeArn")]
    pub image_recipe_arn: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ImageScanningConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageScanningConfiguration")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,

    ///
    /// The configuration settings for your image test components, which includes 			a toggle that allows you to turn off tests, and a timeout setting.
    ///
    /// Required: No
    ///
    /// Type: ImageTestsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageTestsConfiguration")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,

    ///
    /// The Amazon Resource Name (ARN) of the infrastructure configuration associated with 			this image pipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InfrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,

    ///
    /// The tags of the image.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnImage {
    fn type_string(&self) -> &'static str {
        "AWS::ImageBuilder::Image"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.image_scanning_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.image_tests_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The EcrConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::Image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EcrConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,
}

impl cfn_resources::CfnResource for EcrConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The ImageScanningConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::Image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageScanningConfiguration {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: EcrConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EcrConfiguration")]
    pub ecr_configuration: Option<EcrConfiguration>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageScanningEnabled")]
    pub image_scanning_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for ImageScanningConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.ecr_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// When you create an image or container recipe with Image Builder, you can add the build or   		test components that are used to create the final image. You must have at least one build   		component to create a recipe, but test components are not required. If you have added tests,   		they run after the image is created, to ensure that the target image is functional and can   		be used reliably for launching Amazon EC2 instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageTestsConfiguration {
    ///
    /// Determines if tests should run after building the image. Image Builder defaults to enable tests 			to run following the image build, before image distribution.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageTestsEnabled")]
    pub image_tests_enabled: Option<bool>,

    ///
    /// The maximum time in minutes that tests are permitted to run.
    ///
    /// NoteThe timeoutMinutes attribute is not currently active. This value is 				ignored.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 60
    ///
    /// Maximum: 1440
    ///
    /// Update requires: Replacement
    #[serde(rename = "TimeoutMinutes")]
    pub timeout_minutes: Option<i64>,
}

impl cfn_resources::CfnResource for ImageTestsConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.timeout_minutes {
            if *the_val > 1440 as _ {
                return Err(format!(
                    "Max validation failed on field 'timeout_minutes'. {} is greater than 1440",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.timeout_minutes {
            if *the_val < 60 as _ {
                return Err(format!(
                    "Min validation failed on field 'timeout_minutes'. {} is less than 60",
                    the_val
                ));
            }
        }

        Ok(())
    }
}
