

/// The AWS::SageMaker::EndpointConfig resource creates a configuration       for an Amazon SageMaker endpoint. For more information, see CreateEndpointConfig       in the SageMaker Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointConfig {


    /// 
    /// Array of ProductionVariant objects. There is one for each model that you       want to host at this endpoint in shadow mode with production traffic replicated from the       model specified on ProductionVariants. If you use this field, you can only       specify one variant for ProductionVariants and one variant for         ShadowProductionVariants.
    /// 
    /// Required: No
    ///
    /// Type: List of ProductionVariant
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShadowProductionVariants")]
    pub shadow_production_variants: Option<Vec<ProductionVariant>>,


    /// 
    /// A list of ProductionVariant objects, one for each model that you want       to host at this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ProductionVariant
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,


    /// 
    /// A list of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags.
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
    /// Specifies configuration for how an endpoint performs asynchronous inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AsyncInferenceConfig")]
    pub async_inference_config: Option<AsyncInferenceConfig>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Key Management Service key       that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML       compute instance that hosts the endpoint.
    /// 
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab               Key ARN:             arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab               Alias name: alias/ExampleAlias               Alias name ARN:             arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias
    /// 
    /// The KMS key policy must grant permission to the IAM role that you specify in your         CreateEndpoint, UpdateEndpoint requests. For more       information, refer to the AWS Key Management Service section Using Key         Policies in AWS KMS
    /// 
    /// NoteCertain Nitro-based instances include local storage, dependent on the instance         type. Local storage volumes are encrypted using a hardware module on the instance.         You can't request a KmsKeyId when using an instance type with local         storage. If any of the models that you specify in the           ProductionVariants parameter use nitro-based instances with local         storage, do not specify a value for the KmsKeyId parameter. If you         specify a value for KmsKeyId when using any nitro-based instances with         local storage, the call to CreateEndpointConfig fails.For a list of instance types that support local instance storage, see Instance Store Volumes.For more information about local instance storage encryption, see SSD           Instance Store Volumes.
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
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// Specifies how to capture endpoint data for model monitor. The data capture       configuration applies to all production variants hosted at the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: DataCaptureConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCaptureConfig")]
    pub data_capture_config: Option<DataCaptureConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ExplainerConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExplainerConfig")]
    pub explainer_config: Option<ExplainerConfig>,


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
    /// Update requires: Replacement
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: Option<String>,

}

impl cfn_resources::CfnResource for CfnEndpointConfig {
    fn type_string() -> &'static str {
        "AWS::SageMaker::EndpointConfig"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The ClarifyShapConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyShapConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyTextConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TextConfig")]
    pub text_config: Option<ClarifyTextConfig>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ClarifyShapBaselineConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaselineConfig")]
    pub shap_baseline_config: ClarifyShapBaselineConfig,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfSamples")]
    pub number_of_samples: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "UseLogit")]
    pub use_logit: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Seed")]
    pub seed: Option<i64>,

}


/// The ClarifyFeatureType property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyFeatureType {

}


/// Specifies configuration for how an endpoint performs asynchronous inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceConfig {


    /// 
    /// Configures the behavior of the client used by SageMaker to interact with the model       container during asynchronous inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceClientConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<AsyncInferenceClientConfig>,


    /// 
    /// Specifies the configuration for asynchronous inference invocation outputs.
    /// 
    /// Required: Yes
    ///
    /// Type: AsyncInferenceOutputConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutputConfig")]
    pub output_config: AsyncInferenceOutputConfig,

}


/// The ClarifyInferenceConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyInferenceConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelAttribute")]
    pub label_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityIndex")]
    pub probability_index: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxRecordCount")]
    pub max_record_count: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureHeaders")]
    pub feature_headers: Option<Vec<ClarifyHeader>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelIndex")]
    pub label_index: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxPayloadInMB")]
    pub max_payload_in_mb: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyFeatureType
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureTypes")]
    pub feature_types: Option<Vec<ClarifyFeatureType>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentTemplate")]
    pub content_template: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelHeaders")]
    pub label_headers: Option<Vec<ClarifyHeader>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,

}


/// The ClarifyTextConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyTextConfig {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Language")]
    pub language: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Granularity")]
    pub granularity: String,

}


/// Specifies the configuration of your endpoint for model monitor data capture.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataCaptureConfig {


    /// 
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt the captured data at rest using Amazon S3 server-side       encryption. The KmsKeyId can be any of the following formats: Key ID:       1234abcd-12ab-34cd-56ef-1234567890ab Key ARN:       arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab Alias name:       alias/ExampleAlias Alias name ARN: arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias       If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon       S3 for your role's account. For more information, see KMS-Managed Encryption Keys       (https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html) in the Amazon       Simple Storage Service Developer Guide. The KMS key policy must grant permission to the       IAM role that you specify in your CreateModel       (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateModel.html)       request. For more information, see Using Key Policies in AWS KMS       (http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html) in the AWS Key Management Service Developer Guide.
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
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// A list of the JSON and CSV content type that the endpoint captures.
    /// 
    /// Required: No
    ///
    /// Type: CaptureContentTypeHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureContentTypeHeader")]
    pub capture_content_type_header: Option<CaptureContentTypeHeader>,


    /// 
    /// Set to True to enable data capture.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableCapture")]
    pub enable_capture: Option<bool>,


    /// 
    /// Specifies whether the endpoint captures input data to your model, output data from       your model, or both.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CaptureOption
    ///
    /// Maximum: 2
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureOptions")]
    pub capture_options: Vec<CaptureOption>,


    /// 
    /// The percentage of data to capture.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialSamplingPercentage")]
    pub initial_sampling_percentage: i64,


    /// 
    /// The S3 bucket where model monitor stores captured data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^(https|s3)://([^/])/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationS3Uri")]
    pub destination_s3_uri: String,

}


/// The ClarifyExplainerConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyExplainerConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyInferenceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceConfig")]
    pub inference_config: Option<ClarifyInferenceConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableExplanations")]
    pub enable_explanations: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ClarifyShapConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapConfig")]
    pub shap_config: ClarifyShapConfig,

}


/// Specifies the JSON and CSV content types of the data that the endpoint       captures.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptureContentTypeHeader {


    /// 
    /// A list of the JSON content types of the data that the endpoint captures. For the       endpoint to capture the data, you must also specify the content type when you invoke the       endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "JsonContentTypes")]
    pub json_content_types: Option<Vec<String>>,


    /// 
    /// A list of the CSV content types of the data that the endpoint captures. For the       endpoint to capture the data, you must also specify the content type when you invoke the       endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "CsvContentTypes")]
    pub csv_content_types: Option<Vec<String>>,

}


/// Specifies the configuration for asynchronous inference invocation outputs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceOutputConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3FailurePath")]
    pub s3_failure_path: Option<String>,


    /// 
    /// The AWS Key Management Service (AWS KMS) key that Amazon       SageMaker uses to encrypt the asynchronous inference output in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The Amazon S3 location to upload inference responses to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,


    /// 
    /// Specifies the configuration for notifications of inference results for asynchronous       inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceNotificationConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationConfig")]
    pub notification_config: Option<AsyncInferenceNotificationConfig>,

}


/// The ClarifyShapBaselineConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyShapBaselineConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaselineUri")]
    pub shap_baseline_uri: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaseline")]
    pub shap_baseline: Option<String>,

}


/// Configures the behavior of the client used by SageMaker to interact with the model       container during asynchronous inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceClientConfig {


    /// 
    /// The maximum number of concurrent requests sent by the SageMaker client to the model       container. If no value is provided, SageMaker will choose an optimal value for       you.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxConcurrentInvocationsPerInstance")]
    pub max_concurrent_invocations_per_instance: Option<i64>,

}


/// Specifies a model that you want to host and the resources to deploy for hosting it.       If you are deploying multiple models, tell Amazon SageMaker how to distribute traffic       among the models by specifying the InitialVariantWeight objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProductionVariant {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: Option<i64>,


    /// 
    /// The size of the Elastic Inference (EI) instance to use for the production variant. EI       instances provide on-demand GPU computing for inference. For more information, see         Using Elastic         Inference in Amazon SageMaker. For more information, see Using Elastic Inference in         Amazon SageMaker.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.eia1.large | ml.eia1.medium | ml.eia1.xlarge | ml.eia2.large | ml.eia2.medium | ml.eia2.xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorType")]
    pub accelerator_type: Option<String>,


    /// 
    /// Determines initial traffic distribution among all of the models that you specify in       the endpoint configuration. The traffic to a production variant is determined by the       ratio of the VariantWeight to the sum of all VariantWeight       values across all ProductionVariants. If unspecified, it defaults to 1.0.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVariantWeight")]
    pub initial_variant_weight: f64,


    /// 
    /// The serverless configuration for an endpoint. Specifies a serverless endpoint configuration instead of an instance-based endpoint configuration.
    /// 
    /// Required: No
    ///
    /// Type: ServerlessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerlessConfig")]
    pub serverless_config: Option<ServerlessConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerStartupHealthCheckTimeoutInSeconds")]
    pub container_startup_health_check_timeout_in_seconds: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelDataDownloadTimeoutInSeconds")]
    pub model_data_download_timeout_in_seconds: Option<i64>,


    /// 
    /// The ML compute instance type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.large | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.large | ml.c5.xlarge | ml.c5d.18xlarge | ml.c5d.2xlarge | ml.c5d.4xlarge | ml.c5d.9xlarge | ml.c5d.large | ml.c5d.xlarge | ml.c6g.12xlarge | ml.c6g.16xlarge | ml.c6g.2xlarge | ml.c6g.4xlarge | ml.c6g.8xlarge | ml.c6g.large | ml.c6g.xlarge | ml.c6gd.12xlarge | ml.c6gd.16xlarge | ml.c6gd.2xlarge | ml.c6gd.4xlarge | ml.c6gd.8xlarge | ml.c6gd.large | ml.c6gd.xlarge | ml.c6gn.12xlarge | ml.c6gn.16xlarge | ml.c6gn.2xlarge | ml.c6gn.4xlarge | ml.c6gn.8xlarge | ml.c6gn.large | ml.c6gn.xlarge | ml.c6i.12xlarge | ml.c6i.16xlarge | ml.c6i.24xlarge | ml.c6i.2xlarge | ml.c6i.32xlarge | ml.c6i.4xlarge | ml.c6i.8xlarge | ml.c6i.large | ml.c6i.xlarge | ml.c7g.12xlarge | ml.c7g.16xlarge | ml.c7g.2xlarge | ml.c7g.4xlarge | ml.c7g.8xlarge | ml.c7g.large | ml.c7g.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.inf1.24xlarge | ml.inf1.2xlarge | ml.inf1.6xlarge | ml.inf1.xlarge | ml.inf2.24xlarge | ml.inf2.48xlarge | ml.inf2.8xlarge | ml.inf2.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.large | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.large | ml.m5d.xlarge | ml.m6g.12xlarge | ml.m6g.16xlarge | ml.m6g.2xlarge | ml.m6g.4xlarge | ml.m6g.8xlarge | ml.m6g.large | ml.m6g.xlarge | ml.m6gd.12xlarge | ml.m6gd.16xlarge | ml.m6gd.2xlarge | ml.m6gd.4xlarge | ml.m6gd.8xlarge | ml.m6gd.large | ml.m6gd.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p4d.24xlarge | ml.p4de.24xlarge | ml.r5.12xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.large | ml.r5.xlarge | ml.r5d.12xlarge | ml.r5d.24xlarge | ml.r5d.2xlarge | ml.r5d.4xlarge | ml.r5d.large | ml.r5d.xlarge | ml.r6g.12xlarge | ml.r6g.16xlarge | ml.r6g.2xlarge | ml.r6g.4xlarge | ml.r6g.8xlarge | ml.r6g.large | ml.r6g.xlarge | ml.r6gd.12xlarge | ml.r6gd.16xlarge | ml.r6gd.2xlarge | ml.r6gd.4xlarge | ml.r6gd.8xlarge | ml.r6gd.large | ml.r6gd.xlarge | ml.t2.2xlarge | ml.t2.large | ml.t2.medium | ml.t2.xlarge | ml.trn1.2xlarge | ml.trn1.32xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableSSMAccess")]
    pub enable_ssmaccess: Option<bool>,


    /// 
    /// Number of instances to launch initially.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialInstanceCount")]
    pub initial_instance_count: Option<i64>,


    /// 
    /// The name of the production variant.
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
    #[serde(rename = "VariantName")]
    pub variant_name: String,


    /// 
    /// The name of the model that you want to host. This is the name that you specified       when creating the model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelName")]
    pub model_name: String,

}


/// Specifies the serverless configuration for an endpoint variant.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerlessConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisionedConcurrency")]
    pub provisioned_concurrency: Option<i64>,


    /// 
    /// The memory size of your serverless endpoint. Valid values are in 1 GB increments: 1024 MB, 2048 MB, 3072 MB, 4096 MB, 5120 MB, or 6144 MB.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1024
    ///
    /// Maximum: 6144
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemorySizeInMB")]
    pub memory_size_in_mb: i64,


    /// 
    /// The maximum number of concurrent invocations your serverless endpoint can process.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: i64,

}


/// Specifies the configuration for notifications of inference results for asynchronous       inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceNotificationConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IncludeInferenceResponseIn")]
    pub include_inference_response_in: Option<Vec<String>>,


    /// 
    /// Amazon SNS topic to post a notification to when an inference completes successfully.       If no topic is provided, no notification is sent on success.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SuccessTopic")]
    pub success_topic: Option<String>,


    /// 
    /// Amazon SNS topic to post a notification to when an inference fails. If no topic is       provided, no notification is sent on failure.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ErrorTopic")]
    pub error_topic: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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


/// The ClarifyHeader property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyHeader {

}


/// The ExplainerConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExplainerConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyExplainerConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClarifyExplainerConfig")]
    pub clarify_explainer_config: Option<ClarifyExplainerConfig>,

}


/// Specifies whether the endpoint captures input data or output data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptureOption {


    /// 
    /// Specifies whether the endpoint captures input data or output data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Input | Output
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureMode")]
    pub capture_mode: String,

}
