
pub mod cfn_pipe {

#[derive(serde::Serialize, Default)]
pub struct CfnPipe {
    /// No documentation provided by AWS
    #[serde(rename = "SourceParameters")]
    pub source_parameters: Option<PipeSourceParameters>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetParameters")]
    pub target_parameters: Option<PipeTargetParameters>,
    /// No documentation provided by AWS
    #[serde(rename = "EnrichmentParameters")]
    pub enrichment_parameters: Option<PipeEnrichmentParameters>,
    /// No documentation provided by AWS
    #[serde(rename = "Target")]
    pub target: String,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// No documentation provided by AWS
    #[serde(rename = "Enrichment")]
    pub enrichment: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<RequestedPipeState>,
    /// No documentation provided by AWS
    #[serde(rename = "Source")]
    pub source: String,

}


#[derive(serde::Serialize, Default)]
pub struct PipeTargetLambdaFunctionParameters {
    #[serde(rename = "InvocationType")]
    pub invocation_type: Option<PipeTargetInvocationType>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceRabbitMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "VirtualHost")]
    pub virtual_host: Option<String>,
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    #[serde(rename = "Credentials")]
    pub credentials: MQBrokerAccessCredentials,

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceManagedStreamingKafkaParameters {
    #[serde(rename = "TopicName")]
    pub topic_name: String,
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<MSKStartPosition>,
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "ConsumerGroupID")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "Credentials")]
    pub credentials: Option<MSKAccessCredentials>,

}
pub type PipeState = String;
#[derive(serde::Serialize, Default)]
pub struct EcsContainerOverride {
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "EnvironmentFiles")]
    pub environment_files: Option<Vec<EcsEnvironmentFile>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Memory")]
    pub memory: Option<usize>,
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<EcsResourceRequirement>>,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<usize>,
    #[serde(rename = "Cpu")]
    pub cpu: Option<usize>,
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<EcsEnvironmentVariable>>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeEnrichmentParameters {
    #[serde(rename = "HttpParameters")]
    pub http_parameters: Option<PipeEnrichmentHttpParameters>,
    #[serde(rename = "InputTemplate")]
    pub input_template: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetCloudWatchLogsParameters {
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetEcsTaskParameters {
    #[serde(rename = "CapacityProviderStrategy")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "EnableECSManagedTags")]
    pub enable_ecsmanaged_tags: Option<bool>,
    #[serde(rename = "Overrides")]
    pub overrides: Option<EcsTaskOverride>,
    #[serde(rename = "TaskCount")]
    pub task_count: Option<usize>,
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: String,
    #[serde(rename = "Group")]
    pub group: Option<String>,
    #[serde(rename = "PlacementStrategy")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<LaunchType>,
    #[serde(rename = "EnableExecuteCommand")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ReferenceId")]
    pub reference_id: Option<String>,
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<PropagateTags>,
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetEventBridgeEventBusParameters {
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "EndpointId")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "DetailType")]
    pub detail_type: Option<String>,
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "Time")]
    pub time: Option<String>,

}
pub type LaunchType = String;
#[derive(serde::Serialize, Default)]
pub struct PipeSourceSqsQueueParameters {
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementConstraint {
    #[serde(rename = "Type")]
    pub ty: Option<PlacementConstraintType>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchResourceRequirement {
    #[serde(rename = "Type")]
    pub ty: BatchResourceRequirementType,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SelfManagedKafkaAccessConfigurationCredentials {

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceKinesisStreamParameters {
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<usize>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    pub on_partial_batch_item_failure: Option<OnPartialBatchItemFailureStreams>,
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<usize>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "StartingPosition")]
    pub starting_position: KinesisStreamStartPosition,
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<usize>,
    #[serde(rename = "StartingPositionTimestamp")]
    pub starting_position_timestamp: Option<String>,
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,

}
pub type PlacementStrategyType = String;
#[derive(serde::Serialize, Default)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<AssignPublicIp>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchEnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementStrategy {
    #[serde(rename = "Field")]
    pub field: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<PlacementStrategyType>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetHttpParameters {
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<QueryStringParametersMap>,
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<HeaderParametersMap>,
    #[serde(rename = "PathParameterValues")]
    pub path_parameter_values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct EcsInferenceAcceleratorOverride {
    #[serde(rename = "DeviceType")]
    pub device_type: Option<String>,
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

}
pub type EcsResourceRequirementType = String;pub type EcsEnvironmentFileType = String;pub type OnPartialBatchItemFailureStreams = String;
#[derive(serde::Serialize, Default)]
pub struct PipeTargetSageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategyItem {
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: String,
    #[serde(rename = "Base")]
    pub base: Option<usize>,
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,

}
pub type PipeTargetInvocationType = String;
#[derive(serde::Serialize, Default)]
pub struct PipeTargetParameters {
    #[serde(rename = "RedshiftDataParameters")]
    pub redshift_data_parameters: Option<PipeTargetRedshiftDataParameters>,
    #[serde(rename = "SageMakerPipelineParameters")]
    pub sage_maker_pipeline_parameters: Option<PipeTargetSageMakerPipelineParameters>,
    #[serde(rename = "SqsQueueParameters")]
    pub sqs_queue_parameters: Option<PipeTargetSqsQueueParameters>,
    #[serde(rename = "HttpParameters")]
    pub http_parameters: Option<PipeTargetHttpParameters>,
    #[serde(rename = "EventBridgeEventBusParameters")]
    pub event_bridge_event_bus_parameters: Option<PipeTargetEventBridgeEventBusParameters>,
    #[serde(rename = "BatchJobParameters")]
    pub batch_job_parameters: Option<PipeTargetBatchJobParameters>,
    #[serde(rename = "CloudWatchLogsParameters")]
    pub cloud_watch_logs_parameters: Option<PipeTargetCloudWatchLogsParameters>,
    #[serde(rename = "StepFunctionStateMachineParameters")]
    pub step_function_state_machine_parameters: Option<PipeTargetStateMachineParameters>,
    #[serde(rename = "LambdaFunctionParameters")]
    pub lambda_function_parameters: Option<PipeTargetLambdaFunctionParameters>,
    #[serde(rename = "InputTemplate")]
    pub input_template: Option<String>,
    #[serde(rename = "KinesisStreamParameters")]
    pub kinesis_stream_parameters: Option<PipeTargetKinesisStreamParameters>,
    #[serde(rename = "EcsTaskParameters")]
    pub ecs_task_parameters: Option<PipeTargetEcsTaskParameters>,

}
pub type KinesisStreamStartPosition = String;
#[derive(serde::Serialize, Default)]
pub struct FilterCriteria {
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetStateMachineParameters {
    #[serde(rename = "InvocationType")]
    pub invocation_type: Option<PipeTargetInvocationType>,

}
pub type DynamoDBStreamStartPosition = String;
#[derive(serde::Serialize, Default)]
pub struct EcsResourceRequirement {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub ty: EcsResourceRequirementType,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct BatchJobDependency {
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<BatchJobDependencyType>,

}

#[derive(serde::Serialize, Default)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EcsEnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EcsEphemeralStorage {
    #[serde(rename = "SizeInGiB")]
    pub size_in_gi_b: usize,

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceParameters {
    #[serde(rename = "KinesisStreamParameters")]
    pub kinesis_stream_parameters: Option<PipeSourceKinesisStreamParameters>,
    #[serde(rename = "RabbitMQBrokerParameters")]
    pub rabbit_mqbroker_parameters: Option<PipeSourceRabbitMQBrokerParameters>,
    #[serde(rename = "ActiveMQBrokerParameters")]
    pub active_mqbroker_parameters: Option<PipeSourceActiveMQBrokerParameters>,
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "SqsQueueParameters")]
    pub sqs_queue_parameters: Option<PipeSourceSqsQueueParameters>,
    #[serde(rename = "SelfManagedKafkaParameters")]
    pub self_managed_kafka_parameters: Option<PipeSourceSelfManagedKafkaParameters>,
    #[serde(rename = "ManagedStreamingKafkaParameters")]
    pub managed_streaming_kafka_parameters: Option<PipeSourceManagedStreamingKafkaParameters>,
    #[serde(rename = "DynamoDBStreamParameters")]
    pub dynamo_dbstream_parameters: Option<PipeSourceDynamoDBStreamParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AwsvpcConfiguration")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,

}
pub type BatchJobDependencyType = String;
#[derive(serde::Serialize, Default)]
pub struct PipeEnrichmentHttpParameters {
    #[serde(rename = "PathParameterValues")]
    pub path_parameter_values: Option<Vec<String>>,
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<QueryStringParametersMap>,
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<HeaderParametersMap>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceActiveMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    #[serde(rename = "Credentials")]
    pub credentials: MQBrokerAccessCredentials,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,

}
pub type RequestedPipeState = String;
#[derive(serde::Serialize, Default)]
pub struct EcsEnvironmentFile {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub ty: EcsEnvironmentFileType,

}

#[derive(serde::Serialize, Default)]
pub struct PipeSourceSelfManagedKafkaParameters {
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "AdditionalBootstrapServers")]
    pub additional_bootstrap_servers: Option<Vec<String>>,
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    #[serde(rename = "ServerRootCaCertificate")]
    pub server_root_ca_certificate: Option<String>,
    #[serde(rename = "Vpc")]
    pub vpc: Option<SelfManagedKafkaAccessConfigurationVpc>,
    #[serde(rename = "ConsumerGroupID")]
    pub consumer_group_id: Option<String>,
    #[serde(rename = "Credentials")]
    pub credentials: Option<SelfManagedKafkaAccessConfigurationCredentials>,
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<SelfManagedKafkaStartPosition>,
    #[serde(rename = "TopicName")]
    pub topic_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct MSKAccessCredentials {

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetBatchJobParameters {
    #[serde(rename = "JobName")]
    pub job_name: String,
    #[serde(rename = "ArrayProperties")]
    pub array_properties: Option<BatchArrayProperties>,
    #[serde(rename = "RetryStrategy")]
    pub retry_strategy: Option<BatchRetryStrategy>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<BatchParametersMap>,
    #[serde(rename = "DependsOn")]
    pub depends_on: Option<Vec<BatchJobDependency>>,
    #[serde(rename = "JobDefinition")]
    pub job_definition: String,
    #[serde(rename = "ContainerOverrides")]
    pub container_overrides: Option<BatchContainerOverrides>,

}
pub type PropagateTags = String;
#[derive(serde::Serialize, Default)]
pub struct BatchContainerOverrides {
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<BatchEnvironmentVariable>>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<BatchResourceRequirement>>,

}
pub type MSKStartPosition = String;
#[derive(serde::Serialize, Default)]
pub struct TagMap {

}

#[derive(serde::Serialize, Default)]
pub struct BatchParametersMap {

}
pub type PlacementConstraintType = String;
#[derive(serde::Serialize, Default)]
pub struct EcsTaskOverride {
    #[serde(rename = "Memory")]
    pub memory: Option<String>,
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<EcsEphemeralStorage>,
    #[serde(rename = "TaskRoleArn")]
    pub task_role_arn: Option<String>,
    #[serde(rename = "ContainerOverrides")]
    pub container_overrides: Option<Vec<EcsContainerOverride>>,
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "Cpu")]
    pub cpu: Option<String>,
    #[serde(rename = "InferenceAcceleratorOverrides")]
    pub inference_accelerator_overrides: Option<Vec<EcsInferenceAcceleratorOverride>>,

}

#[derive(serde::Serialize, Default)]
pub struct SelfManagedKafkaAccessConfigurationVpc {
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "SecurityGroup")]
    pub security_group: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetRedshiftDataParameters {
    #[serde(rename = "Database")]
    pub database: String,
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: Option<String>,
    #[serde(rename = "DbUser")]
    pub db_user: Option<String>,
    #[serde(rename = "Sqls")]
    pub sqls: Vec<String>,
    #[serde(rename = "WithEvent")]
    pub with_event: Option<bool>,
    #[serde(rename = "StatementName")]
    pub statement_name: Option<String>,

}
pub type SelfManagedKafkaStartPosition = String;
#[derive(serde::Serialize, Default)]
pub struct QueryStringParametersMap {

}

#[derive(serde::Serialize, Default)]
pub struct BatchRetryStrategy {
    #[serde(rename = "Attempts")]
    pub attempts: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchArrayProperties {
    #[serde(rename = "Size")]
    pub size: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderParametersMap {

}
pub type AssignPublicIp = String;
#[derive(serde::Serialize, Default)]
pub struct PipeSourceDynamoDBStreamParameters {
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<usize>,
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<usize>,
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<usize>,
    #[serde(rename = "StartingPosition")]
    pub starting_position: DynamoDBStreamStartPosition,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<usize>,
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<usize>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    pub on_partial_batch_item_failure: Option<OnPartialBatchItemFailureStreams>,

}

#[derive(serde::Serialize, Default)]
pub struct MQBrokerAccessCredentials {

}

#[derive(serde::Serialize, Default)]
pub struct PipeTargetKinesisStreamParameters {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

}
pub type BatchResourceRequirementType = String;
#[derive(serde::Serialize, Default)]
pub struct PipeTargetSqsQueueParameters {
    #[serde(rename = "MessageGroupId")]
    pub message_group_id: Option<String>,
    #[serde(rename = "MessageDeduplicationId")]
    pub message_deduplication_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

}


}
