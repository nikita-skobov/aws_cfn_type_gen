

/// Creates a definition for a job that monitors model quality and drift. For information       about model monitor, see Amazon SageMaker Model       Monitor.
#[derive(Default, serde::Serialize)]
pub struct CfnModelQualityJobDefinition {


    /// 
    /// A list of the inputs that are monitored. Currently endpoints are supported.
    /// 
    /// Required: Yes
    ///
    /// Type: ModelQualityJobInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQualityJobInput")]
    pub model_quality_job_input: ModelQualityJobInput,


    /// 
    /// Specifies the network configuration for the monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: NetworkConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,


    /// 
    /// A time limit for how long the monitoring job is allowed to run before stopping.
    /// 
    /// Required: No
    ///
    /// Type: StoppingCondition
    ///
    /// Update requires: Replacement
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,


    /// 
    /// The output configuration for monitoring jobs.
    /// 
    /// Required: Yes
    ///
    /// Type: MonitoringOutputConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQualityJobOutputConfig")]
    pub model_quality_job_output_config: MonitoringOutputConfig,


    /// 
    /// Container image configuration object for the monitoring job.
    /// 
    /// Required: Yes
    ///
    /// Type: ModelQualityAppSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQualityAppSpecification")]
    pub model_quality_app_specification: ModelQualityAppSpecification,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to       perform tasks on your behalf.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the monitoring job definition.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<String>,


    /// 
    /// Identifies the resources to deploy for a monitoring job.
    /// 
    /// Required: Yes
    ///
    /// Type: MonitoringResources
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,


    /// 
    /// Specifies the constraints and baselines for the monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: ModelQualityBaselineConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelQualityBaselineConfig")]
    pub model_quality_baseline_config: Option<ModelQualityBaselineConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<String>,

}


/// The output object for a monitoring job.
#[derive(Default, serde::Serialize)]
pub struct MonitoringOutput {


    /// 
    /// The Amazon S3 storage location where the results of a monitoring job are saved.
    /// 
    /// Required: Yes
    ///
    /// Type: S3Output
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}


/// The DatasetFormat property type specifies Property description not available. for an AWS::SageMaker::ModelQualityJobDefinition.
#[derive(Default, serde::Serialize)]
pub struct DatasetFormat {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Csv
    ///
    /// Update requires: Replacement
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parquet")]
    pub parquet: Option<bool>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Json")]
    pub json: Option<Json>,

}


/// Networking options for a job, such as network traffic encryption between containers,     whether to allow inbound and outbound network calls to and from containers, and the VPC     subnets and security groups to use for VPC-enabled jobs.
#[derive(Default, serde::Serialize)]
pub struct NetworkConfig {


    /// 
    /// Whether to encrypt all communications between distributed processing jobs. Choose       True to encrypt communications. Encryption provides greater security for distributed       processing jobs, but the processing might take longer.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,


    /// 
    /// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,


    /// 
    /// Whether to allow inbound and outbound network calls to and from the containers used for     the processing job.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,

}


/// The output configuration for monitoring jobs.
#[derive(Default, serde::Serialize)]
pub struct MonitoringOutputConfig {


    /// 
    /// Monitoring outputs for monitoring jobs. This is where the output of the periodic     monitoring jobs is uploaded.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MonitoringOutput
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,


    /// 
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model     artifacts at rest using Amazon S3 server-side encryption.
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


/// Configuration for monitoring constraints and monitoring statistics. These baseline     resources are compared against the results of the current job from the series of jobs     scheduled to collect data periodically.
#[derive(Default, serde::Serialize)]
pub struct ModelQualityBaselineConfig {


    /// 
    /// The name of the job that performs baselining for the monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaseliningJobName")]
    pub baselining_job_name: Option<String>,


    /// 
    /// The constraints resource for a monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: ConstraintsResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,

}


/// The Csv property type specifies Property description not available. for an AWS::SageMaker::ModelQualityJobDefinition.
#[derive(Default, serde::Serialize)]
pub struct Csv {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}


/// Container image configuration object for the monitoring job.
#[derive(Default, serde::Serialize)]
pub struct ModelQualityAppSpecification {


    /// 
    /// An Amazon S3 URI to a script that is called per row prior to running analysis. It can     base64 decode the payload and convert it into a flatted json so that the built-in container     can use the converted data. Applicable only for the built-in (first party)     containers.
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
    #[serde(rename = "RecordPreprocessorSourceUri")]
    pub record_preprocessor_source_uri: Option<String>,


    /// 
    /// An Amazon S3 URI to a script that is called after analysis has been performed.     Applicable only for the built-in (first party) containers.
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
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    pub post_analytics_processor_source_uri: Option<String>,


    /// 
    /// Sets the environment variables in the container that the monitoring job runs.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    pub environment: Option<std::collections::HashMap<String, String>>,


    /// 
    /// An array of arguments for the container used to run the monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerArguments")]
    pub container_arguments: Option<Vec<String>>,


    /// 
    /// The machine learning problem type of the model that the monitoring job monitors.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BinaryClassification | MulticlassClassification | Regression
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProblemType")]
    pub problem_type: String,


    /// 
    /// The address of the container image that the monitoring job runs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageUri")]
    pub image_uri: String,


    /// 
    /// Specifies the entrypoint for a container that the monitoring job runs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,

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


/// The Amazon S3 storage location where the results of a monitoring job are saved.
#[derive(Default, serde::Serialize)]
pub struct S3Output {


    /// 
    /// A URI that identifies the Amazon S3 storage location where Amazon SageMaker saves the       results of a monitoring job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,


    /// 
    /// The local path to the Amazon S3 storage location where Amazon SageMaker saves the       results of a monitoring job. LocalPath is an absolute path for the output data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalPath")]
    pub local_path: String,


    /// 
    /// Whether to upload the results of the monitoring job continuously or after the job       completes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,

}


/// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC. For more       information, see Protect Endpoints by Using an Amazon Virtual Private Cloud and Protect Training Jobs         by Using an Amazon Virtual Private Cloud.
#[derive(Default, serde::Serialize)]
pub struct VpcConfig {


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

}


/// Identifies the resources to deploy for a monitoring job.
#[derive(Default, serde::Serialize)]
pub struct MonitoringResources {


    /// 
    /// The configuration for the cluster resources used to run the processing job.
    /// 
    /// Required: Yes
    ///
    /// Type: ClusterConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}


/// Input object for the endpoint
#[derive(Default, serde::Serialize)]
pub struct EndpointInput {


    /// 
    /// If specified, monitoring jobs substract this time from the end time. For information     about using offsets for scheduling monitoring jobs, see Schedule Model       Quality Monitoring Jobs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^.?P.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<String>,


    /// 
    /// The attribute of the input data that represents the ground truth label.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,


    /// 
    /// In a classification problem, the attribute that represents the class probability.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,


    /// 
    /// Whether the Pipe or File is used as the input mode for     transferring data for the monitoring job. Pipe mode is recommended for large     datasets. File mode is useful for small files that fit in memory. Defaults to       File.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: File | Pipe
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,


    /// 
    /// Path to the filesystem where the endpoint data is available to the container.
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
    #[serde(rename = "LocalPath")]
    pub local_path: String,


    /// 
    /// If specified, monitoring jobs substract this time from the start time. For information     about using offsets for scheduling monitoring jobs, see Schedule Model       Quality Monitoring Jobs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^.?P.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<String>,


    /// 
    /// The threshold for the class probability to be evaluated as a positive result.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,


    /// 
    /// Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key.     Defaults to FullyReplicated
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FullyReplicated | ShardedByS3Key
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,


    /// 
    /// An endpoint in customer's account which has enabled DataCaptureConfig     enabled.
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

}


/// The input for the model quality monitoring job. Currently endponts are supported for     input for model quality monitoring jobs.
#[derive(Default, serde::Serialize)]
pub struct ModelQualityJobInput {


    /// 
    /// The ground truth label provided for the model.
    /// 
    /// Required: Yes
    ///
    /// Type: MonitoringGroundTruthS3Input
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroundTruthS3Input")]
    pub ground_truth_s3_input: MonitoringGroundTruthS3Input,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: BatchTransformInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,


    /// 
    /// Input object for the endpoint
    /// 
    /// Required: No
    ///
    /// Type: EndpointInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,

}


/// Specifies a limit to how long a model training job or model compilation job can run.       It also specifies how long a managed spot training job has to complete. When the job       reaches the time limit, SageMaker ends the training or compilation job. Use this API to cap       model training costs.
///
/// To stop a training job, SageMaker sends the algorithm the SIGTERM signal,       which delays job termination for 120 seconds. Algorithms can use this 120-second window       to save the model artifacts, so the results of training are not lost.
///
/// The training algorithms provided by SageMaker automatically save the intermediate results       of a model training job when possible. This attempt to save artifacts is only a best       effort case as model might not be in a state from which it can be saved. For example, if       training has just started, the model might not be ready to save. When saved, this       intermediate data is a valid model artifact. You can use it to create a model with         CreateModel.
#[derive(Default, serde::Serialize)]
pub struct StoppingCondition {


    /// 
    /// The maximum length of time, in seconds, that a training or compilation job can run       before it is stopped.
    /// 
    /// For compilation jobs, if the job does not complete during this time, a         TimeOut error is generated. We recommend starting with 900 seconds and       increasing as necessary based on your model.
    /// 
    /// For all other jobs, if the job does not complete during this time, SageMaker ends the job.       When RetryStrategy is specified in the job request,         MaxRuntimeInSeconds specifies the maximum time for all of the attempts       in total, not each individual attempt. The default value is 1 day. The maximum value is       28 days.
    /// 
    /// The maximum time that a TrainingJob can run in total, including any time       spent publishing metrics or archiving and uploading models after it has been stopped, is       30 days.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: i64,

}


/// The Json property type specifies Property description not available. for an AWS::SageMaker::ModelQualityJobDefinition.
#[derive(Default, serde::Serialize)]
pub struct Json {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}


/// The ground truth labels for the dataset used for the monitoring job.
#[derive(Default, serde::Serialize)]
pub struct MonitoringGroundTruthS3Input {


    /// 
    /// The address of the Amazon S3 location of the ground truth labels.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}


/// The configuration for the cluster of resources used to run the processing job.
#[derive(Default, serde::Serialize)]
pub struct ClusterConfig {


    /// 
    /// The ML compute instance type for the processing job.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute       instance(s) that run the model monitoring job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,


    /// 
    /// The number of ML compute instances to use in the model monitoring job. For distributed       processing jobs, specify a value greater than 1. The default value is 1.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,


    /// 
    /// The size of the ML storage volume, in gigabytes, that you want to provision. You must       specify sufficient ML storage for your scenario.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,

}


/// The BatchTransformInput property type specifies Property description not available. for an AWS::SageMaker::ModelQualityJobDefinition.
#[derive(Default, serde::Serialize)]
pub struct BatchTransformInput {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: DatasetFormat
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalPath")]
    pub local_path: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,

}


/// The constraints resource for a monitoring job.
#[derive(Default, serde::Serialize)]
pub struct ConstraintsResource {


    /// 
    /// The Amazon S3 URI for the constraints resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<String>,

}
