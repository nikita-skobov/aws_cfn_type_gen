
pub mod cfn_alias {

#[derive(serde::Serialize, Default)]
pub struct CfnAlias {
    /// No documentation provided by AWS
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionVersion")]
    pub function_version: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisionedConcurrencyConfig")]
    pub provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "RoutingConfig")]
    pub routing_config: Option<AliasRoutingConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct VersionWeight {
    #[serde(rename = "FunctionWeight")]
    pub function_weight: f64,
    #[serde(rename = "FunctionVersion")]
    pub function_version: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisionedConcurrencyConfiguration {
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: usize,

}

#[derive(serde::Serialize, Default)]
pub struct AliasRoutingConfiguration {
    #[serde(rename = "AdditionalVersionWeights")]
    pub additional_version_weights: Vec<VersionWeight>,

}


}

pub mod cfn_code_signing_config {

#[derive(serde::Serialize, Default)]
pub struct CfnCodeSigningConfig {
    /// A description of the CodeSigningConfig
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Policies to control how to act if a signature is invalid
    #[serde(rename = "CodeSigningPolicies")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    /// When the CodeSigningConfig is later on attached to a function, the function code will be expected to be signed by profiles from this list
    #[serde(rename = "AllowedPublishers")]
    pub allowed_publishers: AllowedPublishers,

}


#[derive(serde::Serialize, Default)]
pub struct AllowedPublishers {
    #[serde(rename = "SigningProfileVersionArns")]
    pub signing_profile_version_arns: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CodeSigningPolicies {
    #[serde(rename = "UntrustedArtifactOnDeployment")]
    pub untrusted_artifact_on_deployment: String,

}


}

pub mod cfn_event_invoke_config {

#[derive(serde::Serialize, Default)]
pub struct CfnEventInvokeConfig {
    /// No documentation provided by AWS
    #[serde(rename = "Qualifier")]
    pub qualifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "MaximumEventAgeInSeconds")]
    pub maximum_event_age_in_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationConfig")]
    pub destination_config: Option<DestinationConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct OnFailure {
    #[serde(rename = "Destination")]
    pub destination: String,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationConfig {
    #[serde(rename = "OnFailure")]
    pub on_failure: Option<OnFailure>,
    #[serde(rename = "OnSuccess")]
    pub on_success: Option<OnSuccess>,

}

#[derive(serde::Serialize, Default)]
pub struct OnSuccess {
    #[serde(rename = "Destination")]
    pub destination: String,

}


}

pub mod cfn_event_source_mapping {

#[derive(serde::Serialize, Default)]
pub struct CfnEventSourceMapping {
    /// Specific configuration settings for an MSK event source.
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,
    /// The filter criteria to control event filtering.
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: Option<FilterCriteria>,
    /// The name of the Lambda function.
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// A list of SourceAccessConfiguration.
    #[serde(rename = "SourceAccessConfigurations")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    /// The scaling configuration for the event source.
    #[serde(rename = "ScalingConfig")]
    pub scaling_config: Option<ScalingConfig>,
    /// The Amazon Resource Name (ARN) of the event source.
    #[serde(rename = "EventSourceArn")]
    pub event_source_arn: Option<String>,
    /// (Streams) If the function returns an error, split the batch in two and retry.
    #[serde(rename = "BisectBatchOnFunctionError")]
    pub bisect_batch_on_function_error: Option<bool>,
    /// (Streams) An Amazon SQS queue or Amazon SNS topic destination for discarded records.
    #[serde(rename = "DestinationConfig")]
    pub destination_config: Option<DestinationConfig>,
    /// Self-managed event source endpoints.
    #[serde(rename = "SelfManagedEventSource")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    /// Document db event source config.
    #[serde(rename = "DocumentDBEventSourceConfig")]
    pub document_dbevent_source_config: Option<DocumentDBEventSourceConfig>,
    /// (Streams) Tumbling window (non-overlapping time window) duration to perform aggregations.
    #[serde(rename = "TumblingWindowInSeconds")]
    pub tumbling_window_in_seconds: Option<usize>,
    /// (Streams) The maximum age of a record that Lambda sends to a function for processing.
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<usize>,
    /// (Streams) A list of response types supported by the function.
    #[serde(rename = "FunctionResponseTypes")]
    pub function_response_types: Option<Vec<String>>,
    /// (Streams) The number of batches to process from each shard concurrently.
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<usize>,
    /// (Kafka) A list of Kafka topics.
    #[serde(rename = "Topics")]
    pub topics: Option<Vec<String>>,
    /// Specific configuration settings for a Self-Managed Apache Kafka event source.
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,
    /// The maximum number of items to retrieve in a single batch.
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    /// Disables the event source mapping to pause polling and invocation.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// (Streams) The maximum amount of time to gather records before invoking the function, in seconds.
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    /// With StartingPosition set to AT_TIMESTAMP, the time from which to start reading, in Unix time seconds.
    #[serde(rename = "StartingPositionTimestamp")]
    pub starting_position_timestamp: Option<f64>,
    /// (Streams) The maximum number of times to retry when the function returns an error.
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<usize>,
    /// The position in a stream from which to start reading. Required for Amazon Kinesis and Amazon DynamoDB Streams sources.
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<String>,
    /// (ActiveMQ) A list of ActiveMQ queues.
    #[serde(rename = "Queues")]
    pub queues: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct DocumentDBEventSourceConfig {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "CollectionName")]
    pub collection_name: Option<String>,
    #[serde(rename = "FullDocument")]
    pub full_document: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Endpoints {
    #[serde(rename = "KafkaBootstrapServers")]
    pub kafka_bootstrap_servers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationConfig {
    #[serde(rename = "OnFailure")]
    pub on_failure: Option<OnFailure>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceAccessConfiguration {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "URI")]
    pub uri: Option<String>,

}
pub type MaximumConcurrency = usize;
#[derive(serde::Serialize, Default)]
pub struct FilterCriteria {
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,

}

#[derive(serde::Serialize, Default)]
pub struct OnFailure {
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SelfManagedEventSource {
    #[serde(rename = "Endpoints")]
    pub endpoints: Option<Endpoints>,

}
pub type ConsumerGroupId = String;
#[derive(serde::Serialize, Default)]
pub struct ScalingConfig {
    #[serde(rename = "MaximumConcurrency")]
    pub maximum_concurrency: Option<MaximumConcurrency>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    pub consumer_group_id: Option<ConsumerGroupId>,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SelfManagedKafkaEventSourceConfig {
    #[serde(rename = "ConsumerGroupId")]
    pub consumer_group_id: Option<ConsumerGroupId>,

}


}

pub mod cfn_function {

#[derive(serde::Serialize, Default)]
pub struct CfnFunction {
    /// A description of the function.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The number of simultaneous executions to reserve for the function.
    #[serde(rename = "ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: Option<usize>,
    /// The name of the Lambda function, up to 64 characters in length. If you don't specify a name, AWS CloudFormation generates one.
    #[serde(rename = "FunctionName")]
    pub function_name: Option<String>,
    /// The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    /// Connection settings for an Amazon EFS file system. To connect a function to a file system, a mount target must be available in every Availability Zone that your function connects to. If your template contains an AWS::EFS::MountTarget resource, you must also specify a DependsOn attribute to ensure that the mount target is created or updated before the function.
    #[serde(rename = "FileSystemConfigs")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    /// The name of the method within your code that Lambda calls to execute your function. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime
    #[serde(rename = "Handler")]
    pub handler: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Architectures")]
    pub architectures: Option<Vec<String>>,
    /// RuntimeManagementConfig
    #[serde(rename = "RuntimeManagementConfig")]
    pub runtime_management_config: Option<RuntimeManagementConfig>,
    /// The Amazon Resource Name (ARN) of the function's execution role.
    #[serde(rename = "Role")]
    pub role: String,
    /// A function's ephemeral storage settings.
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    /// Environment variables that are accessible from function code during execution.
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,
    /// The amount of memory that your function has access to. Increasing the function's memory also increases its CPU allocation. The default value is 128 MB. The value must be a multiple of 64 MB.
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<usize>,
    /// The amount of time that Lambda allows a function to run before stopping it. The default is 3 seconds. The maximum allowed value is 900 seconds.
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    /// The SnapStart setting of your function
    #[serde(rename = "SnapStart")]
    pub snap_start: Option<SnapStart>,
    /// A dead letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing.
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC.
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    /// Set Mode to Active to sample and trace a subset of incoming requests with AWS X-Ray.
    #[serde(rename = "TracingConfig")]
    pub tracing_config: Option<TracingConfig>,
    /// A list of function layers to add to the function's execution environment. Specify each layer by its ARN, including the version.
    #[serde(rename = "Layers")]
    pub layers: Option<Vec<String>>,
    /// PackageType.
    #[serde(rename = "PackageType")]
    pub package_type: Option<String>,
    /// The code for the function.
    #[serde(rename = "Code")]
    pub code: Code,
    /// The identifier of the function's runtime.
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    /// A list of tags to apply to the function.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// ImageConfig
    #[serde(rename = "ImageConfig")]
    pub image_config: Option<ImageConfig>,
    /// A unique Arn for CodeSigningConfig resource
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ImageConfig {
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "EntryPoint")]
    pub entry_point: Option<Vec<String>>,
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DeadLetterConfig {
    #[serde(rename = "TargetArn")]
    pub target_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub variables: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct SnapStart {
    #[serde(rename = "ApplyOn")]
    pub apply_on: String,

}

#[derive(serde::Serialize, Default)]
pub struct Code {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "ImageUri")]
    pub image_uri: Option<String>,
    #[serde(rename = "S3Key")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "ZipFile")]
    pub zip_file: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TracingConfig {
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RuntimeManagementConfig {
    #[serde(rename = "RuntimeVersionArn")]
    pub runtime_version_arn: Option<String>,
    #[serde(rename = "UpdateRuntimeOn")]
    pub update_runtime_on: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FileSystemConfig {
    #[serde(rename = "LocalMountPath")]
    pub local_mount_path: String,
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct EphemeralStorage {
    #[serde(rename = "Size")]
    pub size: usize,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct SnapStartResponse {
    #[serde(rename = "OptimizationStatus")]
    pub optimization_status: Option<String>,
    #[serde(rename = "ApplyOn")]
    pub apply_on: Option<String>,

}


}

pub mod cfn_layer_version {

#[derive(serde::Serialize, Default)]
pub struct CfnLayerVersion {
    /// No documentation provided by AWS
    #[serde(rename = "Content")]
    pub content: Content,
    /// No documentation provided by AWS
    #[serde(rename = "LicenseInfo")]
    pub license_info: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LayerName")]
    pub layer_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CompatibleRuntimes")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "CompatibleArchitectures")]
    pub compatible_architectures: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Content {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}


}

pub mod cfn_layer_version_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnLayerVersionPermission {
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: String,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationId")]
    pub organization_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: String,
    /// No documentation provided by AWS
    #[serde(rename = "LayerVersionArn")]
    pub layer_version_arn: String,

}



}

pub mod cfn_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnPermission {
    /// No documentation provided by AWS
    #[serde(rename = "FunctionUrlAuthType")]
    pub function_url_auth_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceAccount")]
    pub source_account: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: String,
    /// No documentation provided by AWS
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PrincipalOrgID")]
    pub principal_org_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: String,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EventSourceToken")]
    pub event_source_token: Option<String>,

}



}

pub mod cfn_url {

#[derive(serde::Serialize, Default)]
pub struct CfnUrl {
    /// No documentation provided by AWS
    #[serde(rename = "Cors")]
    pub cors: Option<Cors>,
    /// The Amazon Resource Name (ARN) of the function associated with the Function URL.
    #[serde(rename = "TargetFunctionArn")]
    pub target_function_arn: String,
    /// The invocation mode for the function’s URL. Set to BUFFERED if you want to buffer responses before returning them to the client. Set to RESPONSE_STREAM if you want to stream responses, allowing faster time to first byte and larger response payload sizes. If not set, defaults to BUFFERED.
    #[serde(rename = "InvokeMode")]
    pub invoke_mode: Option<String>,
    /// Can be either AWS_IAM if the requests are authorized via IAM, or NONE if no authorization is configured on the Function URL.
    #[serde(rename = "AuthType")]
    pub auth_type: String,
    /// The alias qualifier for the target function. If TargetFunctionArn is unqualified then Qualifier must be passed.
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,

}

pub type AllowHeaders = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct Cors {
    #[serde(rename = "AllowCredentials")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: Option<ExposeHeaders>,
    #[serde(rename = "MaxAge")]
    pub max_age: Option<usize>,
    #[serde(rename = "AllowMethods")]
    pub allow_methods: Option<AllowMethods>,
    #[serde(rename = "AllowOrigins")]
    pub allow_origins: Option<AllowOrigins>,
    #[serde(rename = "AllowHeaders")]
    pub allow_headers: Option<AllowHeaders>,

}
pub type AllowOrigins = Vec<String>;pub type ExposeHeaders = Vec<String>;pub type AllowMethods = Vec<String>;

}

pub mod cfn_version {

#[derive(serde::Serialize, Default)]
pub struct CfnVersion {
    /// No documentation provided by AWS
    #[serde(rename = "CodeSha256")]
    pub code_sha256: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisionedConcurrencyConfig")]
    pub provisioned_concurrency_config: Option<ProvisionedConcurrencyConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct ProvisionedConcurrencyConfiguration {
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: usize,

}


}
