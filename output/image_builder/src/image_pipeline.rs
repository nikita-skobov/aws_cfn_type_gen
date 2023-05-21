

/// An image pipeline is the automation configuration for building secure OS images on AWS.     The Image Builder image pipeline is associated with an image recipe that defines the build,     validation, and test phases for an image build lifecycle. An image pipeline can be     associated with an infrastructure configuration that defines where your image is built. You     can define attributes, such as instance type, subnets, security groups, logging, and other     infrastructure-related configurations. You can also associate your image pipeline with a     distribution configuration to define how you would like to deploy your image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImagePipeline {


    /// 
    /// The Amazon Resource Name (ARN) of the container recipe that is used for this 			pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerRecipeArn")]
    pub container_recipe_arn: Option<String>,


    /// 
    /// The description of this image pipeline.
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
    /// The Amazon Resource Name (ARN) of the distribution configuration associated with this 			image pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DistributionConfigurationArn")]
    pub distribution_configuration_arn: Option<String>,


    /// 
    /// Collects additional information about the image being created, including the operating 			system (OS) version and package list. This information is used to enhance the overall 			experience of using EC2 Image Builder. Enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnhancedImageMetadataEnabled")]
    pub enhanced_image_metadata_enabled: Option<bool>,


    /// 
    /// The Amazon Resource Name (ARN) of the image recipe associated with this image 			pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageRecipeArn")]
    pub image_recipe_arn: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ImageScanningConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageScanningConfiguration")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,


    /// 
    /// The configuration of the image tests that run after image creation to ensure the  			quality of the image that was created.
    /// 
    /// Required: No
    ///
    /// Type: ImageTestsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageTestsConfiguration")]
    pub image_tests_configuration: Option<ImageTestsConfiguration>,


    /// 
    /// The Amazon Resource Name (ARN) of the infrastructure configuration associated with 			this image pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InfrastructureConfigurationArn")]
    pub infrastructure_configuration_arn: String,


    /// 
    /// The name of the image pipeline.
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
    /// The schedule of the image pipeline. A schedule configures how often and when a pipeline 			automatically creates a new image.
    /// 
    /// Required: No
    ///
    /// Type: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,


    /// 
    /// The status of the image pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<ImagePipelineStatusEnum>,


    /// 
    /// The tags of this image pipeline.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ImagePipelineStatusEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

}

impl Default for ImagePipelineStatusEnum {
    fn default() -> Self {
        ImagePipelineStatusEnum::Disabled
    }
}


impl cfn_resources::CfnResource for CfnImagePipeline {
    fn type_string() -> &'static str {
        "AWS::ImageBuilder::ImagePipeline"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The EcrConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::ImagePipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EcrConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerTags")]
    pub container_tags: Option<Vec<String>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryName")]
    pub repository_name: Option<String>,

}




/// The ImageScanningConfiguration property type specifies Property description not available. for an AWS::ImageBuilder::ImagePipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageScanningConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: EcrConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcrConfiguration")]
    pub ecr_configuration: Option<EcrConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageScanningEnabled")]
    pub image_scanning_enabled: Option<bool>,

}




/// When you create an image or container recipe with Image Builder, you can add the build or   		test components that your image pipeline uses to create the final image. You must   		have at least one build component to create a recipe, but test components are not required.   		Your pipeline runs tests after it builds the image, to ensure that the target image is   		functional and can be used reliably for launching Amazon EC2 instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImageTestsConfiguration {


    /// 
    /// Defines if tests should be executed when building this image. For example,       true or false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "TimeoutMinutes")]
    pub timeout_minutes: Option<i64>,

}




/// A schedule configures how often and when a pipeline will automatically create a new 			image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schedule {


    /// 
    /// The condition configures when the pipeline should trigger a new image build. When the 	    pipelineExecutionStartCondition is set to 	    EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE, and you use semantic version 			filters on the base image or components in your image recipe, Image Builder will build a 			new image only when there are new versions of the image or components in your recipe that 			match the semantic version filter. When it is set to EXPRESSION_MATCH_ONLY, it 			will build a new image every time the CRON expression matches the current time. For semantic 			version syntax, see CreateComponent      	in the Image Builder API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE | EXPRESSION_MATCH_ONLY
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineExecutionStartCondition")]
    pub pipeline_execution_start_condition: Option<SchedulePipelineExecutionStartConditionEnum>,


    /// 
    /// The cron expression determines how often EC2 Image Builder evaluates your 				pipelineExecutionStartCondition.
    /// 
    /// For information on how to format a cron expression in Image Builder, see Use 				cron expressions in EC2 Image Builder.
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
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SchedulePipelineExecutionStartConditionEnum {

    /// EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE
    #[serde(rename = "EXPRESSION_MATCH_AND_DEPENDENCY_UPDATES_AVAILABLE")]
    Expressionmatchanddependencyupdatesavailable,

    /// EXPRESSION_MATCH_ONLY
    #[serde(rename = "EXPRESSION_MATCH_ONLY")]
    Expressionmatchonly,

}

impl Default for SchedulePipelineExecutionStartConditionEnum {
    fn default() -> Self {
        SchedulePipelineExecutionStartConditionEnum::Expressionmatchanddependencyupdatesavailable
    }
}

