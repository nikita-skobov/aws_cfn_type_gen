

/// A versioned model that can be deployed for SageMaker inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModelPackage {


    /// 
    /// The version number of a versioned model.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageVersion")]
    pub model_package_version: Option<i64>,


    /// 
    /// The name of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<String>,


    /// 
    /// An array of additional Inference Specification objects.
    /// 
    /// Required: No
    ///
    /// Type: List of AdditionalInferenceSpecificationDefinition
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInferenceSpecifications")]
    pub additional_inference_specifications: Option<Vec<AdditionalInferenceSpecificationDefinition>>,


    /// 
    /// Metadata properties of the tracking entity, trial, or trial component.
    /// 
    /// Required: No
    ///
    /// Type: MetadataProperties
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetadataProperties")]
    pub metadata_properties: Option<MetadataProperties>,


    /// 
    /// Information about the user who created or modified an experiment, trial, trial component, lineage group, or project.
    /// 
    /// Required: No
    ///
    /// Type: UserContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<UserContext>,


    /// 
    /// Information about the user who created or modified an experiment, trial, trial component, lineage group, or project.
    /// 
    /// Required: No
    ///
    /// Type: UserContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedBy")]
    pub last_modified_by: Option<UserContext>,


    /// 
    /// The environment variables to set in the Docker container. Each key and value in the       Environment string to string map can have length of up to 1024. We       support up to 16 entries in the map.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<std::collections::HashMap<String, String>>,


    /// 
    /// An array of additional Inference Specification objects to be added to the existing       array. The total number of additional Inference Specification objects cannot exceed 15.       Each additional Inference Specification object specifies artifacts based on this model       package that can be used on inference endpoints. Generally used with SageMaker Neo to       store the compiled artifacts.
    /// 
    /// Required: No
    ///
    /// Type: List of AdditionalInferenceSpecificationDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInferenceSpecificationsToAdd")]
    pub additional_inference_specifications_to_add: Option<Vec<AdditionalInferenceSpecificationDefinition>>,


    /// 
    /// Represents the drift check baselines that can be used when the model monitor is set using the model package.
    /// 
    /// Required: No
    ///
    /// Type: DriftCheckBaselines
    ///
    /// Update requires: Replacement
    #[serde(rename = "DriftCheckBaselines")]
    pub drift_check_baselines: Option<DriftCheckBaselines>,


    /// 
    /// A list of the tags associated with the model package. For more information, see Tagging AWS       resources in the         AWS General Reference Guide.
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
    /// Specifies the validation and image scan statuses of the model package.
    /// 
    /// Required: No
    ///
    /// Type: ModelPackageStatusDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageStatusDetails")]
    pub model_package_status_details: Option<ModelPackageStatusDetails>,


    /// 
    /// The machine learning domain of your model package and its components. Common       machine learning domains include computer vision and natural language processing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    pub domain: Option<String>,


    /// 
    /// The approval status of the model. This can be one of the following values.
    /// 
    /// APPROVED - The model is approved                        REJECTED - The model is rejected.                        PENDING_MANUAL_APPROVAL - The model is waiting for manual           approval.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Approved | PendingManualApproval | Rejected
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelApprovalStatus")]
    pub model_approval_status: Option<String>,


    /// 
    /// The machine learning task your model package accomplishes. Common machine    learning tasks include object detection and image classification.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Task")]
    pub task: Option<String>,


    /// 
    /// A structure of additional Inference Specification. Additional Inference Specification       specifies details about inference jobs that can be run with models based on       this model package
    /// 
    /// Required: No
    ///
    /// Type: AdditionalInferenceSpecificationDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInferenceSpecificationDefinition")]
    pub additional_inference_specification_definition: Option<AdditionalInferenceSpecificationDefinition>,


    /// 
    /// A list of algorithms that were used to create a model package.
    /// 
    /// Required: No
    ///
    /// Type: SourceAlgorithmSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAlgorithmSpecification")]
    pub source_algorithm_specification: Option<SourceAlgorithmSpecification>,


    /// 
    /// Whether the model package is to be certified to be listed on AWS Marketplace. For       information about listing model packages on AWS Marketplace, see List Your         Algorithm or Model Package on AWS Marketplace.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertifyForMarketplace")]
    pub certify_for_marketplace: Option<bool>,


    /// 
    /// Metrics for the model.
    /// 
    /// Required: No
    ///
    /// Type: ModelMetrics
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelMetrics")]
    pub model_metrics: Option<ModelMetrics>,


    /// 
    /// The metadata properties for the model package.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerMetadataProperties")]
    pub customer_metadata_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Represents the overall status of a model package.
    /// 
    /// Required: No
    ///
    /// Type: ModelPackageStatusItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelPackageStatusItem")]
    pub model_package_status_item: Option<ModelPackageStatusItem>,


    /// 
    /// The model group to which the model belongs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: Option<String>,


    /// 
    /// The last time the model package was modified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: Option<String>,


    /// 
    /// The description of the model package.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelPackageDescription")]
    pub model_package_description: Option<String>,


    /// 
    /// A description provided when the model approval is set.
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
    #[serde(rename = "ApprovalDescription")]
    pub approval_description: Option<String>,


    /// 
    /// Defines how to perform inference generation after a training job is run.
    /// 
    /// Required: No
    ///
    /// Type: InferenceSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceSpecification")]
    pub inference_specification: Option<InferenceSpecification>,


    /// 
    /// The Amazon Simple Storage Service path where the sample payload are stored. This path must point to       a single gzip compressed tar archive (.tar.gz suffix).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SamplePayloadUrl")]
    pub sample_payload_url: Option<String>,


    /// 
    /// Specifies batch transform jobs that SageMaker runs to validate your model package.
    /// 
    /// Required: No
    ///
    /// Type: ValidationSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationSpecification")]
    pub validation_specification: Option<ValidationSpecification>,


    /// 
    /// A unique token that guarantees that the call to this API is idempotent.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientToken")]
    pub client_token: Option<String>,

}

impl cfn_resources::CfnResource for CfnModelPackage {
    fn type_string() -> &'static str {
        "AWS::SageMaker::ModelPackage"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Information about the user who created or modified an experiment, trial, trial    component, lineage group, project, or model card.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserContext {


    /// 
    /// The Amazon Resource Name (ARN) of the user's profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileArn")]
    pub user_profile_arn: Option<String>,


    /// 
    /// The name of the user's profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,


    /// 
    /// The domain associated with the user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainId")]
    pub domain_id: Option<String>,

}


/// A structure of additional Inference Specification. Additional Inference Specification       specifies details about inference jobs that can be run with models based on       this model package
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdditionalInferenceSpecificationDefinition {


    /// 
    /// A list of the instance types that are used to generate inferences in real-time.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedRealtimeInferenceInstanceTypes")]
    pub supported_realtime_inference_instance_types: Option<Vec<String>>,


    /// 
    /// A unique name to identify the additional inference specification. The name must       be unique within the list of your additional inference specifications for a       particular model package.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The supported MIME types for the output data.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedResponseMIMETypes")]
    pub supported_response_mimetypes: Option<Vec<String>>,


    /// 
    /// The Amazon ECR registry path of the Docker image that contains the inference code.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ModelPackageContainerDefinition
    ///
    /// Maximum: 15
    ///
    /// Update requires: No interruption
    #[serde(rename = "Containers")]
    pub containers: Vec<ModelPackageContainerDefinition>,


    /// 
    /// The supported MIME types for the input data.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Option<Vec<String>>,


    /// 
    /// A list of the instance types on which a transformation job can be run       or on which an endpoint can be deployed.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SupportedTransformInstanceTypes")]
    pub supported_transform_instance_types: Option<Vec<String>>,


    /// 
    /// A description of the additional Inference specification
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// Defines the input needed to run a transform job using the inference specification       specified in the algorithm.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformJobDefinition {


    /// 
    /// The maximum number of parallel requests that can be sent to each instance in a       transform job. The default value is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxConcurrentTransforms")]
    pub max_concurrent_transforms: Option<i64>,


    /// 
    /// The environment variables to set in the Docker container. We support up to 16 key and       values entries in the map.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    pub environment: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Identifies the ML compute instances for the transform job.
    /// 
    /// Required: Yes
    ///
    /// Type: TransformResources
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransformResources")]
    pub transform_resources: TransformResources,


    /// 
    /// A string that determines the number of records included in a single mini-batch.
    /// 
    /// SingleRecord means only one record is used per mini-batch.         MultiRecord means a mini-batch is set to contain as many records that       can fit within the MaxPayloadInMB limit.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MultiRecord | SingleRecord
    ///
    /// Update requires: Replacement
    #[serde(rename = "BatchStrategy")]
    pub batch_strategy: Option<String>,


    /// 
    /// The maximum payload size allowed, in MB. A payload is the data portion of a record       (without metadata).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxPayloadInMB")]
    pub max_payload_in_mb: Option<i64>,


    /// 
    /// A description of the input source and the way the transform job consumes it.
    /// 
    /// Required: Yes
    ///
    /// Type: TransformInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransformInput")]
    pub transform_input: TransformInput,


    /// 
    /// Identifies the Amazon S3 location where you want Amazon SageMaker to save the results       from the transform job.
    /// 
    /// Required: Yes
    ///
    /// Type: TransformOutput
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransformOutput")]
    pub transform_output: TransformOutput,

}


/// Describes the S3 data source.
///
/// Your input bucket must be in the same AWS region as your training job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3DataSource {


    /// 
    /// If you choose S3Prefix, S3Uri identifies a key name prefix.       SageMaker uses all objects that match the specified key name prefix for model training.
    /// 
    /// If you choose ManifestFile, S3Uri identifies an object that       is a manifest file containing a list of object keys that you want SageMaker to use for model       training.
    /// 
    /// If you choose AugmentedManifestFile, S3Uri identifies an object that is       an augmented manifest file in JSON lines format. This file contains the data you want to       use for model training. AugmentedManifestFile can only be used if the       Channel's input mode is Pipe.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AugmentedManifestFile | ManifestFile | S3Prefix
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,


    /// 
    /// Depending on the value specified for the S3DataType, identifies either       a key name prefix or a manifest. For example:
    /// 
    /// A key name prefix might look like this:             s3://bucketname/exampleprefix                       A manifest might look like this:             s3://bucketname/example.manifest                 A manifest is an S3 object which is a JSON file consisting of an array of           elements. The first element is a prefix which is followed by one or more           suffixes. SageMaker appends the suffix elements to the prefix to get a full set           of S3Uri. Note that the prefix must be a valid non-empty             S3Uri that precludes users from specifying a manifest whose           individual S3Uri is sourced from different S3 buckets.         The following code example shows a valid manifest format:                  [ {"prefix": "s3://customer_bucket/some/prefix/"},                          "relative/path/to/custdata-1",                          "relative/path/custdata-2",                          ...                          "relative/path/custdata-N"                          ]                 This JSON is equivalent to the following S3Uri           list:                  s3://customer_bucket/some/prefix/relative/path/to/custdata-1                          s3://customer_bucket/some/prefix/relative/path/custdata-2                          ...                          s3://customer_bucket/some/prefix/relative/path/custdata-N                The complete set of S3Uri in this manifest is the input data           for the channel for this data source. The object that each S3Uri           points to must be readable by the IAM role that SageMaker uses to perform tasks on           your behalf.
    /// 
    /// Your input bucket must be located in same AWS region as your training job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}


/// Input object for the model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelInput {


    /// 
    /// The input configuration object for the model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\S\s]+
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DataInputConfig")]
    pub data_input_config: String,

}


/// Describes the input source of a transform job and the way the transform job consumes       it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformInput {


    /// 
    /// The method to use to split the transform job's data files into smaller batches.       Splitting is necessary when the total size of each object is too large to fit in a       single request. You can also use data splitting to improve performance by processing       multiple concurrent mini-batches. The default value for SplitType is         None, which indicates that input data files are not split, and request       payloads contain the entire contents of an input object. Set the value of this parameter       to Line to split records on a newline character boundary.         SplitType also supports a number of record-oriented binary data       formats. Currently, the supported record formats are:
    /// 
    /// RecordIO               TFRecord
    /// 
    /// When splitting is enabled, the size of a mini-batch depends on the values of the         BatchStrategy and MaxPayloadInMB parameters. When the       value of BatchStrategy is MultiRecord, Amazon SageMaker sends the maximum       number of records in each request, up to the MaxPayloadInMB limit. If the       value of BatchStrategy is SingleRecord, Amazon SageMaker sends individual       records in each request.
    /// 
    /// NoteSome data formats represent a record as a binary payload wrapped with extra         padding bytes. When splitting is applied to a binary data format, padding is removed         if the value of BatchStrategy is set to SingleRecord.         Padding is not removed if the value of BatchStrategy is set to           MultiRecord.For more information about RecordIO, see Create a Dataset Using           RecordIO in the MXNet documentation. For more information about           TFRecord, see Consuming TFRecord data in the TensorFlow documentation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Line | None | RecordIO | TFRecord
    ///
    /// Update requires: Replacement
    #[serde(rename = "SplitType")]
    pub split_type: Option<String>,


    /// 
    /// Describes the location of       the       channel data, which is, the S3 location of the input data that the       model can consume.
    /// 
    /// Required: Yes
    ///
    /// Type: DataSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSource")]
    pub data_source: DataSource,


    /// 
    /// The multipurpose internet mail extension       (MIME)       type of the data. Amazon SageMaker uses the MIME type with each http call to       transfer data to the transform job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,


    /// 
    /// If your transform data       is       compressed, specify the compression type. Amazon SageMaker automatically       decompresses the data for the transform job accordingly. The default value is         None.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Gzip | None
    ///
    /// Update requires: Replacement
    #[serde(rename = "CompressionType")]
    pub compression_type: Option<String>,

}


/// Metadata properties of the tracking entity, trial, or trial component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetadataProperties {


    /// 
    /// The project ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProjectId")]
    pub project_id: Option<String>,


    /// 
    /// The commit ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CommitId")]
    pub commit_id: Option<String>,


    /// 
    /// The entity this entity was generated by.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "GeneratedBy")]
    pub generated_by: Option<String>,


    /// 
    /// The repository.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Repository")]
    pub repository: Option<String>,

}


/// Data quality constraints and statistics for a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelDataQuality {


    /// 
    /// Data quality constraints for a model.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,


    /// 
    /// Data quality statistics for a model.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,

}


/// Describes the Docker container for the model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelPackageContainerDefinition {


    /// 
    /// A structure with Model Input details.
    /// 
    /// Required: No
    ///
    /// Type: ModelInput
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ModelInput")]
    pub model_input: Option<ModelInput>,


    /// 
    /// The Amazon S3 path where the model artifacts, which result from model training, are stored.       This path must point to a single gzip compressed tar archive       (.tar.gz suffix).
    /// 
    /// NoteThe model artifacts must be in an S3 bucket that is in the same region as the         model package.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,


    /// 
    /// The machine learning framework of the model package container image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Framework")]
    pub framework: Option<String>,


    /// 
    /// An MD5 hash of the training algorithm that identifies the Docker image used for       training.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 72
    ///
    /// Pattern: ^[Ss][Hh][Aa]256:[0-9a-fA-F]{64}$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ImageDigest")]
    pub image_digest: Option<String>,


    /// 
    /// The DNS host name for the Docker container.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ContainerHostname")]
    pub container_hostname: Option<String>,


    /// 
    /// The framework version of the Model Package Container Image.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 10
    ///
    /// Pattern: [0-9]\.[A-Za-z0-9.-]+
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: Option<String>,


    /// 
    /// The name of a pre-trained machine learning benchmarked by       Amazon SageMaker Inference Recommender model that matches your model.       You can find a list of benchmarked models by calling ListModelMetadata.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NearestModelName")]
    pub nearest_model_name: Option<String>,


    /// 
    /// The Amazon EC2 Container Registry (Amazon ECR) path where inference code is stored.
    /// 
    /// If you are using your own custom algorithm instead of an algorithm provided by SageMaker,       the inference code must meet SageMaker requirements. SageMaker supports both       registry/repository[:tag] and registry/repository[@digest]       image path formats. For more information, see Using Your Own Algorithms with Amazon         SageMaker.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\S]+
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Image")]
    pub image: String,


    /// 
    /// The environment variables to set in the Docker container. Each key and value in the       Environment string to string map can have length of up to 1024. We       support up to 16 entries in the map.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Environment")]
    pub environment: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The AWS Marketplace product ID of the model package.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

}


/// Describes the resources, including ML instance types and ML instance count, to use for       transform job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformResources {


    /// 
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt model data on the storage volume       attached to the ML compute instance(s) that run the batch transform job.
    /// 
    /// NoteCertain Nitro-based instances include local storage, dependent on the instance         type. Local storage volumes are encrypted using a hardware module on the instance.         You can't request a VolumeKmsKeyId when using an instance type with         local storage.For a list of instance types that support local instance storage, see Instance Store Volumes.For more information about local instance storage encryption, see SSD         Instance Store Volumes.
    /// 
    /// The VolumeKmsKeyId can be any of the following formats:
    /// 
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab                       Key ARN:             arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab                       Alias name: alias/ExampleAlias                       Alias name ARN:             arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias
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
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,


    /// 
    /// The ML compute instance type for the transform job. If you are using built-in       algorithms to       transform       moderately sized datasets, we recommend using ml.m4.xlarge or       ml.m5.largeinstance types.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.large | ml.m5.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The number of       ML       compute instances to use in the transform job. The default value is         1, and the maximum is 100. For distributed transform jobs,       specify a value greater than 1.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

}


/// Contains details regarding the file source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileSource {


    /// 
    /// The digest of the file source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 72
    ///
    /// Pattern: ^[Ss][Hh][Aa]256:[0-9a-fA-F]{64}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentDigest")]
    pub content_digest: Option<String>,


    /// 
    /// The Amazon S3 URI for the file source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,


    /// 
    /// The type of content stored in the file source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

}


/// Specifies the validation and image scan statuses of the model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelPackageStatusDetails {


    /// 
    /// The status of the scan of the Docker image container for the model package.
    /// 
    /// Required: No
    ///
    /// Type: List of ModelPackageStatusItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageScanStatuses")]
    pub image_scan_statuses: Option<Vec<ModelPackageStatusItem>>,


    /// 
    /// The validation status of the model package.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ModelPackageStatusItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationStatuses")]
    pub validation_statuses: Vec<ModelPackageStatusItem>,

}


/// Contains bias metrics for a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Bias {


    /// 
    /// The bias report for a model
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Report")]
    pub report: Option<MetricsSource>,


    /// 
    /// The pre-training bias report for a model.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreTrainingReport")]
    pub pre_training_report: Option<MetricsSource>,


    /// 
    /// The post-training bias report for a model.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "PostTrainingReport")]
    pub post_training_report: Option<MetricsSource>,

}


/// Details about the metrics source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricsSource {


    /// 
    /// The metric source content type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentType")]
    pub content_type: String,


    /// 
    /// The S3 URI for the metrics source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,


    /// 
    /// The hash key used for the metrics source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 72
    ///
    /// Pattern: ^[Ss][Hh][Aa]256:[0-9a-fA-F]{64}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentDigest")]
    pub content_digest: Option<String>,

}


/// Specifies an algorithm that was used to create the model package. The algorithm must       be either an algorithm resource in your SageMaker account or an algorithm in AWS Marketplace that you are subscribed to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceAlgorithm {


    /// 
    /// The name of an algorithm that was used to create the model package. The algorithm must       be either an algorithm resource in your SageMaker account or an algorithm in AWS Marketplace that you are subscribed to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 170
    ///
    /// Pattern: (arn:aws[a-z\-]*:sagemaker:[a-z0-9\-]*:[0-9]{12}:[a-z\-]*\/)?([a-zA-Z0-9]([a-zA-Z0-9-]){0,62})(?<!-)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,


    /// 
    /// The Amazon S3 path where the model artifacts, which result from model training, are stored.       This path must point to a single gzip compressed tar archive         (.tar.gz suffix).
    /// 
    /// NoteThe model artifacts must be in an S3 bucket that is in the same AWS region as the         algorithm.
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

}


/// Defines how to perform inference generation after a training job is run.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InferenceSpecification {


    /// 
    /// The supported MIME types for the output data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SupportedResponseMIMETypes")]
    pub supported_response_mimetypes: Vec<String>,


    /// 
    /// The supported MIME types for the input data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Vec<String>,


    /// 
    /// A list of the instance types on which a transformation job can be run or on which an       endpoint can be deployed.
    /// 
    /// This parameter is required for unversioned models, and optional for versioned       models.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SupportedTransformInstanceTypes")]
    pub supported_transform_instance_types: Option<Vec<String>>,


    /// 
    /// A list of the instance types that are used to generate inferences in real-time.
    /// 
    /// This parameter is required for unversioned models, and optional for versioned       models.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SupportedRealtimeInferenceInstanceTypes")]
    pub supported_realtime_inference_instance_types: Option<Vec<String>>,


    /// 
    /// The Amazon ECR registry path of the Docker image that contains the inference code.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ModelPackageContainerDefinition
    ///
    /// Maximum: 15
    ///
    /// Update requires: Replacement
    #[serde(rename = "Containers")]
    pub containers: Vec<ModelPackageContainerDefinition>,

}


/// Specifies batch transform jobs that SageMaker runs to validate your model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValidationSpecification {


    /// 
    /// The IAM roles to be used for the validation of the model package.
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
    #[serde(rename = "ValidationRole")]
    pub validation_role: String,


    /// 
    /// An array of ModelPackageValidationProfile objects, each of which       specifies a batch transform job that SageMaker runs to validate your model package.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ValidationProfile
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationProfiles")]
    pub validation_profiles: Vec<ValidationProfile>,

}


/// Contains explainability metrics for a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Explainability {


    /// 
    /// The explainability report for a model.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Report")]
    pub report: Option<MetricsSource>,

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


/// Represents the drift check model quality baselines that can be used when the model monitor is set using       the model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DriftCheckModelQuality {


    /// 
    /// The drift check model quality statistics.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,


    /// 
    /// The drift check model quality constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}


/// Represents the drift check explainability baselines that can be used when the model monitor is set       using the model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DriftCheckExplainability {


    /// 
    /// The explainability config file for the model.
    /// 
    /// Required: No
    ///
    /// Type: FileSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<FileSource>,


    /// 
    /// The drift check explainability constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}


/// A list of algorithms that were used to create a model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceAlgorithmSpecification {


    /// 
    /// A list of the algorithms that were used to create a model package.
    /// 
    /// Required: Yes
    ///
    /// Type: List of SourceAlgorithm
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAlgorithms")]
    pub source_algorithms: Vec<SourceAlgorithm>,

}


/// Describes the results of a transform job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransformOutput {


    /// 
    /// The Amazon S3 path where you want Amazon SageMaker to store the results of the transform job. For       example, s3://bucket-name/key-name-prefix.
    /// 
    /// For every S3 object used as input for the transform job, batch transform stores the       transformed data with an .out suffix in a corresponding subfolder in the       location in the output prefix. For example, for the input data stored at         s3://bucket-name/input-name-prefix/dataset01/data.csv, batch transform       stores the transformed data at         s3://bucket-name/output-name-prefix/input-name-prefix/data.csv.out.       Batch transform doesn't upload partially processed objects. For an input S3 object that       contains multiple records, it creates an .out file only if the transform       job succeeds on the entire file. When the input contains multiple S3 objects, the batch       transform job processes the listed S3 objects and uploads only the output for       successfully processed objects. If any object fails in the transform job batch transform       marks the job as failed to prompt investigation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,


    /// 
    /// Defines how to assemble the results of the transform job as a single S3 object. Choose       a format that is most convenient to you. To concatenate the results in binary format,       specify None. To add a newline character at the end of every transformed       record, specify       Line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Line | None
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssembleWith")]
    pub assemble_with: Option<String>,


    /// 
    /// The MIME type used to specify the output data. Amazon SageMaker uses the MIME type with each http       call to transfer data from the transform job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Accept")]
    pub accept: Option<String>,


    /// 
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using       Amazon S3 server-side encryption. The KmsKeyId can be any of the following       formats:
    /// 
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab                       Key ARN:             arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab                       Alias name: alias/ExampleAlias                       Alias name ARN:             arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias
    /// 
    /// If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon S3 for your       role's account. For more information, see KMS-Managed Encryption Keys in the         Amazon Simple Storage Service         Developer Guide.
    /// 
    /// The KMS key policy must grant permission to the IAM role that you specify in your 	CreateModel 		request. For more information, see Using           Key Policies in AWS KMS in the         AWS Key Management Service Developer         Guide.
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

}


/// Represents the drift check data quality baselines that can be used when the model monitor is set using       the model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DriftCheckModelDataQuality {


    /// 
    /// The drift check model data quality statistics.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,


    /// 
    /// The drift check model data quality constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}


/// Contains metrics captured from a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelMetrics {


    /// 
    /// Metrics that measure the quality of a model.
    /// 
    /// Required: No
    ///
    /// Type: ModelQuality
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQuality")]
    pub model_quality: Option<ModelQuality>,


    /// 
    /// Metrics that measure bais in a model.
    /// 
    /// Required: No
    ///
    /// Type: Bias
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bias")]
    pub bias: Option<Bias>,


    /// 
    /// Metrics that measure the quality of the input data for a model.
    /// 
    /// Required: No
    ///
    /// Type: ModelDataQuality
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelDataQuality")]
    pub model_data_quality: Option<ModelDataQuality>,


    /// 
    /// Metrics that help explain a model.
    /// 
    /// Required: No
    ///
    /// Type: Explainability
    ///
    /// Update requires: Replacement
    #[serde(rename = "Explainability")]
    pub explainability: Option<Explainability>,

}


/// Model quality statistics and constraints.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelQuality {


    /// 
    /// Model quality constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,


    /// 
    /// Model quality statistics.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,

}


/// Represents the overall status of a model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelPackageStatusItem {


    /// 
    /// if the overall status is Failed, the reason for the failure.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<String>,


    /// 
    /// The name of the model package for which the overall status is being reported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The current status.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Completed | Failed | InProgress | NotStarted
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,

}


/// Represents the drift check baselines that can be used when the model monitor is set using the model       package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DriftCheckBaselines {


    /// 
    /// Represents the drift check model data quality baselines that can be used when the model monitor is set       using the model package.
    /// 
    /// Required: No
    ///
    /// Type: DriftCheckModelDataQuality
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelDataQuality")]
    pub model_data_quality: Option<DriftCheckModelDataQuality>,


    /// 
    /// Represents the drift check bias baselines that can be used when the model monitor is set using the model       package.
    /// 
    /// Required: No
    ///
    /// Type: DriftCheckBias
    ///
    /// Update requires: Replacement
    #[serde(rename = "Bias")]
    pub bias: Option<DriftCheckBias>,


    /// 
    /// Represents the drift check explainability baselines that can be used when the model monitor is set using       the model package.
    /// 
    /// Required: No
    ///
    /// Type: DriftCheckExplainability
    ///
    /// Update requires: Replacement
    #[serde(rename = "Explainability")]
    pub explainability: Option<DriftCheckExplainability>,


    /// 
    /// Represents the drift check model quality baselines that can be used when the model monitor is set using       the model package.
    /// 
    /// Required: No
    ///
    /// Type: DriftCheckModelQuality
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQuality")]
    pub model_quality: Option<DriftCheckModelQuality>,

}


/// Represents the drift check bias baselines that can be used when the model monitor is set using the       model package.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DriftCheckBias {


    /// 
    /// The post-training constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "PostTrainingConstraints")]
    pub post_training_constraints: Option<MetricsSource>,


    /// 
    /// The bias config file for a model.
    /// 
    /// Required: No
    ///
    /// Type: FileSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<FileSource>,


    /// 
    /// The pre-training constraints.
    /// 
    /// Required: No
    ///
    /// Type: MetricsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreTrainingConstraints")]
    pub pre_training_constraints: Option<MetricsSource>,

}


/// Describes the location of the channel data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSource {


    /// 
    /// The S3 location of the data source that is associated with a channel.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DataSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: S3DataSource,

}


/// Contains data, such as the inputs and targeted instance types that are used in the       process of validating the model package.
///
/// The data provided in the validation profile is made available to your buyers on AWS       Marketplace.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ValidationProfile {


    /// 
    /// The name of the profile for the model package.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileName")]
    pub profile_name: String,


    /// 
    /// The TransformJobDefinition object that describes the transform job used       for the validation of the model package.
    /// 
    /// Required: Yes
    ///
    /// Type: TransformJobDefinition
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransformJobDefinition")]
    pub transform_job_definition: TransformJobDefinition,

}
