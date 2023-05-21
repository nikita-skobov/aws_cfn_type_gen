

/// The AWS::SageMaker::Model resource to create a model to host at an Amazon       SageMaker endpoint. For more information, see Deploying a Model on Amazon         SageMaker Hosting Services in the Amazon SageMaker Developer         Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnModel {


    /// 
    /// A list of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags in the AWS Billing and Cost         Management User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies details of how containers in a multi-container endpoint are called.
    /// 
    /// Required: No
    ///
    /// Type: InferenceExecutionConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceExecutionConfig")]
    pub inference_execution_config: Option<InferenceExecutionConfig>,


    /// 
    /// The location of the primary docker image containing inference code, associated       artifacts, and custom environment map that the inference code uses when the model is       deployed for predictions.
    /// 
    /// Required: No
    ///
    /// Type: ContainerDefinition
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrimaryContainer")]
    pub primary_container: Option<ContainerDefinition>,


    /// 
    /// A VpcConfig object that specifies the VPC that you want your model to connect       to. Control access to and from your model container by configuring the VPC.         VpcConfig is used in hosting services and in batch transform. For more       information, see Protect Endpoints by Using an Amazon Virtual Private Cloud and Protect Data in Batch         Transform Jobs by Using an Amazon Virtual Private Cloud.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,


    /// 
    /// Isolates the model container. No inbound or outbound network calls can be made to or       from the model container.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,


    /// 
    /// Specifies the containers in the inference pipeline.
    /// 
    /// Required: No
    ///
    /// Type: List of ContainerDefinition
    ///
    /// Maximum: 15
    ///
    /// Update requires: Replacement
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<ContainerDefinition>>,


    /// 
    /// The name of the new model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that SageMaker can assume to access model       artifacts and docker image for deployment on ML compute instances or for batch transform       jobs. Deploying on ML compute instances is part of model hosting. For more information,       see SageMaker         Roles.
    /// 
    /// NoteTo be able to pass this role to SageMaker, the caller of this API must have the           iam:PassRole permission.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,

}


/// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC. For more       information, see Protect Endpoints by Using an Amazon Virtual Private Cloud and Protect Training Jobs         by Using an Amazon Virtual Private Cloud.
#[derive(Default, serde::Serialize)]
pub struct VpcConfig {


    /// 
    /// The ID of the subnets in the VPC to which you want to connect your training job or       model. For information about the availability of specific instance types, see Supported         Instance Types and Availability Zones.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,


    /// 
    /// The VPC security group IDs, in the form sg-xxxxxxxx. Specify the security groups for       the VPC that is specified in the Subnets field.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}


/// Specifies an authentication configuration for the private docker registry where your       model image is hosted. Specify a value for this property only if you specified         Vpc as the value for the RepositoryAccessMode field of the         ImageConfig object that you passed to a call to         CreateModel and the private Docker registry where the model image is       hosted requires authentication.
#[derive(Default, serde::Serialize)]
pub struct RepositoryAuthConfig {


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Lambda function that provides       credentials to authenticate to the private Docker registry where your model image is       hosted. For information about how to create an AWS Lambda function, see         Create a Lambda function         with the console in the         AWS Lambda Developer         Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryCredentialsProviderArn")]
    pub repository_credentials_provider_arn: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// Describes the container, as part of model definition.
#[derive(Default, serde::Serialize)]
pub struct ContainerDefinition {


    /// 
    /// The inference specification name in the model package version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceSpecificationName")]
    pub inference_specification_name: Option<String>,


    /// 
    /// Whether the container hosts a single model or multiple models.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MultiModel | SingleModel
    ///
    /// Update requires: Replacement
    #[serde(rename = "Mode")]
    pub mode: Option<String>,


    /// 
    /// The environment variables to set in the Docker container. Each key and value in the         Environment string to string map can have length of up to 1024. We       support up to 16 entries in the map.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    pub environment: Option<serde_json::Value>,


    /// 
    /// The name or Amazon Resource Name (ARN) of the model package to use to create the       model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 176
    ///
    /// Pattern: (arn:aws[a-z\-]*:sagemaker:[a-z0-9\-]*:[0-9]{12}:[a-z\-]*\/)?([a-zA-Z0-9]([a-zA-Z0-9-]){0,62})(?<!-)(\/[0-9]{1,5})?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<String>,


    /// 
    /// The S3 path where the model artifacts, which result from model training, are stored.       This path must point to a single gzip compressed tar archive (.tar.gz suffix). The S3       path is required for SageMaker built-in algorithms, but not if you use your own algorithms.       For more information on built-in algorithms, see Common         Parameters.
    /// 
    /// NoteThe model artifacts must be in an S3 bucket that is in the same region as the         model or endpoint you are creating.
    /// 
    /// If you provide a value for this parameter, SageMaker uses AWS Security Token       Service to download model artifacts from the S3 path you provide. AWS STS       is activated in your AWS account by default. If you previously       deactivated AWS STS for a region, you need to reactivate AWS STS for that region. For more information, see Activating and         Deactivating AWS STS in an AWS Region in the                   AWS Identity and Access Management User         Guide.
    /// 
    /// ImportantIf you use a built-in algorithm to create a model, SageMaker requires that you provide         a S3 path to the model artifacts in ModelDataUrl.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,


    /// 
    /// Specifies additional configuration for multi-model endpoints.
    /// 
    /// Required: No
    ///
    /// Type: MultiModelConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "MultiModelConfig")]
    pub multi_model_config: Option<MultiModelConfig>,


    /// 
    /// This parameter is ignored for models that contain only a       PrimaryContainer.
    /// 
    /// When a ContainerDefinition is part of an inference pipeline, the value of       the parameter uniquely identifies the container for the purposes of logging and metrics.       For information, see Use Logs and Metrics         to Monitor an Inference Pipeline. If you don't specify a value for this       parameter for a ContainerDefinition that is part of an inference pipeline,       a unique name is automatically assigned based on the position of the         ContainerDefinition in the pipeline. If you specify a value for the         ContainerHostName for any ContainerDefinition that is part       of an inference pipeline, you must specify a value for the         ContainerHostName parameter of every ContainerDefinition       in that pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerHostname")]
    pub container_hostname: Option<String>,


    /// 
    /// The path where inference code is stored. This can be either in Amazon EC2 Container Registry or in a       Docker registry that is accessible from the same VPC that you configure for your       endpoint. If you are using your own custom algorithm instead of an algorithm provided by       SageMaker, the inference code must meet SageMaker requirements. SageMaker supports both         registry/repository[:tag] and registry/repository[@digest]       image path formats. For more information, see Using Your Own Algorithms with       Amazon SageMaker.
    /// 
    /// NoteThe model artifacts in an Amazon S3 bucket and the Docker image for inference container         in Amazon EC2 Container Registry must be in the same region as the model or endpoint you are         creating.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Image")]
    pub image: Option<String>,


    /// 
    /// Specifies whether the model container is in Amazon ECR or a private Docker registry       accessible from your Amazon Virtual Private Cloud (VPC). For information about storing containers in a       private Docker registry, see Use a         Private Docker Registry for Real-Time Inference Containers.
    /// 
    /// NoteThe model artifacts in an Amazon S3 bucket and the Docker image for inference container         in Amazon EC2 Container Registry must be in the same region as the model or endpoint you are         creating.
    /// 
    /// Required: No
    ///
    /// Type: ImageConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageConfig")]
    pub image_config: Option<ImageConfig>,

}


/// Specifies whether the model container is in Amazon ECR or a private Docker registry       accessible from your Amazon Virtual Private Cloud (VPC).
#[derive(Default, serde::Serialize)]
pub struct ImageConfig {


    /// 
    /// (Optional) Specifies an authentication configuration for the private docker registry       where your model image is hosted. Specify a value for this property only if you       specified Vpc as the value for the RepositoryAccessMode field,       and the private Docker registry where the model image is hosted requires       authentication.
    /// 
    /// Required: No
    ///
    /// Type: RepositoryAuthConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryAuthConfig")]
    pub repository_auth_config: Option<RepositoryAuthConfig>,


    /// 
    /// Set this to one of the following values:
    /// 
    /// Platform - The model image is hosted in Amazon ECR.                        Vpc - The model image is hosted in a private Docker registry in           your VPC.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Platform | Vpc
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryAccessMode")]
    pub repository_access_mode: String,

}


/// Specifies additional configuration for hosting multi-model endpoints.
#[derive(Default, serde::Serialize)]
pub struct MultiModelConfig {


    /// 
    /// Whether to cache models for a multi-model endpoint. By default, multi-model endpoints       cache models so that a model does not have to be loaded into memory each time it is       invoked. Some use cases do not benefit from model caching. For example, if an endpoint       hosts a large number of models that are each invoked infrequently, the endpoint might       perform better if you disable model caching. To disable model caching, set the value of       this parameter to Disabled.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelCacheSetting")]
    pub model_cache_setting: Option<String>,

}


/// Specifies details about how containers in a multi-container endpoint are run.
#[derive(Default, serde::Serialize)]
pub struct InferenceExecutionConfig {


    /// 
    /// How containers in a multi-container are run. The following values are valid.
    /// 
    /// Serial - Containers run as a serial pipeline.               Direct - Only the individual container that you specify is           run.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Mode")]
    pub mode: String,

}
