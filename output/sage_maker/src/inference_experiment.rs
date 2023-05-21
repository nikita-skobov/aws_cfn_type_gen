

/// Creates an inference experiment using the configurations specified in the request.
///
/// Use this API to setup and schedule an experiment to compare model variants on a Amazon SageMaker inference endpoint. For      more information about inference experiments, see Shadow tests.
///
/// Amazon SageMaker begins your experiment at the scheduled time and routes traffic to your endpoint's model variants based      on your specified configuration.
///
/// While the experiment is in progress or after it has concluded, you can view metrics that compare your model      variants. For more information, see View, monitor, and edit shadow tests.
#[derive(Default, serde::Serialize)]
pub struct CfnInferenceExperiment {


    /// 
    /// The ARN of the IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage      Amazon SageMaker Inference endpoints for model deployment.
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
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The duration for which the inference experiment ran or will run.
    /// 
    /// The maximum duration that you can set for an inference experiment is 30 days.
    /// 
    /// Required: No
    ///
    /// Type: InferenceExperimentSchedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<InferenceExperimentSchedule>,


    /// 
    /// The type of the inference experiment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ShadowMode
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The error message for the inference experiment status result.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusReason")]
    pub status_reason: Option<String>,


    /// 
    /// The Amazon S3 location and configuration for storing inference request and response data.
    /// 
    /// Required: No
    ///
    /// Type: DataStorageConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataStorageConfig")]
    pub data_storage_config: Option<DataStorageConfig>,


    /// 
    /// An array of ModelVariantConfigSummary objects. There is one for each variant in the inference      experiment. Each ModelVariantConfigSummary object in the array describes the infrastructure      configuration for deploying the corresponding variant.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ModelVariantConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelVariants")]
    pub model_variants: Vec<ModelVariantConfig>,


    /// 
    /// The name of the inference experiment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 120
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,119}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,


    /// 
    /// The configuration of ShadowMode inference experiment type, which shows the production variant      that takes all the inference requests, and the shadow variant to which Amazon SageMaker replicates a percentage of the      inference requests. For the shadow variant it also shows the percentage of requests that Amazon SageMaker replicates.
    /// 
    /// Required: No
    ///
    /// Type: ShadowModeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModeConfig")]
    pub shadow_mode_config: Option<ShadowModeConfig>,


    /// 
    /// The AWS Key Management Service key that Amazon SageMaker uses to encrypt captured data at rest using Amazon S3      server-side encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,


    /// 
    /// The description of the inference experiment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The desired state of the experiment after stopping. The possible states are the following:
    /// 
    /// Completed: The experiment completed successfully                        Cancelled: The experiment was canceled
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Cancelled | Completed
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,

}


/// The name and sampling percentage of a shadow variant.
#[derive(Default, serde::Serialize)]
pub struct ShadowModelVariantConfig {


    /// 
    /// The percentage of inference requests that Amazon SageMaker replicates from the production variant to the shadow variant.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingPercentage")]
    pub sampling_percentage: i64,


    /// 
    /// The name of the shadow variant.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModelVariantName")]
    pub shadow_model_variant_name: String,

}


/// Configuration specifying how to treat different headers. If no headers are specified SageMaker      will by default base64 encode when capturing the data.
#[derive(Default, serde::Serialize)]
pub struct CaptureContentTypeHeader {


    /// 
    /// The list of all content type headers that SageMaker will treat as CSV and capture accordingly.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvContentTypes")]
    pub csv_content_types: Option<Vec<String>>,


    /// 
    /// The list of all content type headers that SageMaker will treat as JSON and capture accordingly.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonContentTypes")]
    pub json_content_types: Option<Vec<String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The configuration of ShadowMode inference experiment type, which specifies a production variant      to take all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the      inference requests. For the shadow variant it also specifies the percentage of requests that Amazon SageMaker replicates.
#[derive(Default, serde::Serialize)]
pub struct ShadowModeConfig {


    /// 
    /// The name of the production variant, which takes all the inference requests.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceModelVariantName")]
    pub source_model_variant_name: String,


    /// 
    /// List of shadow variant configurations.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ShadowModelVariantConfig
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModelVariants")]
    pub shadow_model_variants: Vec<ShadowModelVariantConfig>,

}


/// The infrastructure configuration for deploying the model to a real-time inference endpoint.
#[derive(Default, serde::Serialize)]
pub struct RealTimeInferenceConfig {


    /// 
    /// The instance type the model is deployed to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.xlarge | ml.c5d.18xlarge | ml.c5d.2xlarge | ml.c5d.4xlarge | ml.c5d.9xlarge | ml.c5d.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.16xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.8xlarge | ml.m5d.large | ml.m5d.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p3dn.24xlarge | ml.r5.12xlarge | ml.r5.16xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.8xlarge | ml.r5.large | ml.r5.xlarge | ml.t2.2xlarge | ml.t2.large | ml.t2.medium | ml.t2.xlarge | ml.t3.2xlarge | ml.t3.large | ml.t3.medium | ml.t3.xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The number of instances of the type specified by InstanceType.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

}


/// The configuration for the infrastructure that the model will be deployed to.
#[derive(Default, serde::Serialize)]
pub struct ModelInfrastructureConfig {


    /// 
    /// The infrastructure configuration for deploying the model to real-time inference.
    /// 
    /// Required: Yes
    ///
    /// Type: RealTimeInferenceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealTimeInferenceConfig")]
    pub real_time_inference_config: RealTimeInferenceConfig,


    /// 
    /// The inference option to which to deploy your model. Possible values are the following:
    /// 
    /// RealTime: Deploy to real-time inference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: RealTimeInference
    ///
    /// Update requires: No interruption
    #[serde(rename = "InfrastructureType")]
    pub infrastructure_type: String,

}


/// The start and end times of an inference experiment.
///
/// The maximum duration that you can set for an inference experiment is 30 days.
#[derive(Default, serde::Serialize)]
pub struct InferenceExperimentSchedule {


    /// 
    /// The timestamp at which the inference experiment ended or will end.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,


    /// 
    /// The timestamp at which the inference experiment started or will start.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

}


/// The Amazon S3 location and configuration for storing inference request and response data.
///
/// This is an optional parameter that you can use for data capture. For more information, see Capture data.
#[derive(Default, serde::Serialize)]
pub struct DataStorageConfig {


    /// 
    /// The AWS Key Management Service key that Amazon SageMaker uses to encrypt captured data at rest using Amazon S3      server-side encryption.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,


    /// 
    /// The Amazon S3 bucket where the inference request and response data is stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^(https|s3)://([^/])/?(.*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: String,


    /// 
    /// Configuration specifying how to treat different headers. If no headers are specified SageMaker will by default base64 encode when capturing the data.
    /// 
    /// Required: No
    ///
    /// Type: CaptureContentTypeHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    pub content_type: Option<CaptureContentTypeHeader>,

}


/// The metadata of the endpoint.
#[derive(Default, serde::Serialize)]
pub struct EndpointMetadata {


    /// 
    /// The name of the endpoint configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: Option<String>,


    /// 
    /// The name of the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,


    /// 
    /// The status of the endpoint. For possible values of the status of an endpoint, see       https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-endpointmetadata.html#cfn-sagemaker-inferenceexperiment-endpointmetadata-endpointstatus.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Creating | Deleting | Failed | InService | OutOfService | RollingBack | SystemUpdating | Updating
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointStatus")]
    pub endpoint_status: Option<String>,

}


/// Contains information about the deployment options of a model.
#[derive(Default, serde::Serialize)]
pub struct ModelVariantConfig {


    /// 
    /// The name of the Amazon SageMaker Model entity.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelName")]
    pub model_name: String,


    /// 
    /// The name of the variant.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariantName")]
    pub variant_name: String,


    /// 
    /// The configuration for the infrastructure that the model will be deployed to.
    /// 
    /// Required: Yes
    ///
    /// Type: ModelInfrastructureConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "InfrastructureConfig")]
    pub infrastructure_config: ModelInfrastructureConfig,

}
