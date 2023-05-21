

/// Create a pipe. Amazon EventBridge Pipes connect event sources to targets and reduces the need for specialized knowledge and integration code.
#[derive(Default, serde::Serialize)]
pub struct CfnPipe {


    /// The name of the pipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// The parameters required to set up a source for your pipe.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceParameters")]
    pub source_parameters: Option<PipeSourceParameters>,


    /// The parameters required to set up enrichment on your pipe.
    ///
    /// Required: No
    ///
    /// Type: PipeEnrichmentParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnrichmentParameters")]
    pub enrichment_parameters: Option<PipeEnrichmentParameters>,


    /// The list of key-value pairs to associate with the pipe.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// The ARN of the role that allows the pipe to send data to the target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// The ARN of the enrichment resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enrichment")]
    pub enrichment: Option<String>,


    /// The ARN of the target resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: String,


    /// The ARN of the source resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: String,


    /// A description of the pipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// The state the pipe should be in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,


    /// The parameters required to set up a target for your pipe.
    ///
    /// For more information about pipe target parameters, including how to use dynamic path parameters, see Target parameters in the Amazon EventBridge User Guide.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetParameters")]
    pub target_parameters: Option<PipeTargetParameters>,

}


/// The parameters for using a Rabbit MQ broker as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceRabbitMQBrokerParameters {


    /// 
    /// The name of the virtual host associated with the source broker.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualHost")]
    pub virtual_host: Option<String>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// The name of the destination queue to consume.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueueName")]
    pub queue_name: String,


    /// The credentials needed to access the resource.
    ///
    /// Required: Yes
    ///
    /// Type: MQBrokerAccessCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: MQBrokerAccessCredentials,

}


/// This structure specifies the VPC subnets and security groups for the stream, and whether a public IP address is to be used.
#[derive(Default, serde::Serialize)]
pub struct SelfManagedKafkaAccessConfigurationVpc {


    /// 
    /// Specifies the subnets associated with the stream. These subnets must all be in the same VPC. You can specify as many as 16 subnets.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,


    /// 
    /// Specifies the security groups associated with the stream. These security groups must all be in the same VPC. You can specify as many      as five security groups. If you do not specify a security group, the default security group for the VPC is used.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroup")]
    pub security_group: Option<Vec<String>>,

}


/// The details of a capacity provider strategy. To learn more, see CapacityProviderStrategyItem in the Amazon ECS API Reference.
#[derive(Default, serde::Serialize)]
pub struct CapacityProviderStrategyItem {


    /// 
    /// The base value designates how many tasks, at a minimum, to run on the specified capacity     provider. Only one capacity provider in a capacity provider strategy can have a base defined.     If no value is specified, the default value of 0 is used.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    pub base: Option<i64>,


    /// 
    /// The weight value designates the relative percentage of the total number of tasks launched     that should use the specified capacity provider. The weight value is taken into consideration     after the base value, if defined, is satisfied.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<i64>,


    /// 
    /// The short name of the capacity provider.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: String,

}


/// A DeadLetterConfig object that contains information about a dead-letter queue configuration.
#[derive(Default, serde::Serialize)]
pub struct DeadLetterConfig {


    /// 
    /// The ARN of the Amazon SQS queue specified as the target for the dead-letter queue.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}


/// The parameters required to set up a source for your pipe.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceParameters {


    /// The parameters for using a self-managed Apache Kafka stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceSelfManagedKafkaParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelfManagedKafkaParameters")]
    pub self_managed_kafka_parameters: Option<PipeSourceSelfManagedKafkaParameters>,


    /// The parameters for using a Amazon SQS stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceSqsQueueParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqsQueueParameters")]
    pub sqs_queue_parameters: Option<PipeSourceSqsQueueParameters>,


    /// The parameters for using a Rabbit MQ broker as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceRabbitMQBrokerParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RabbitMQBrokerParameters")]
    pub rabbit_mqbroker_parameters: Option<PipeSourceRabbitMQBrokerParameters>,


    /// The parameters for using an MSK stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceManagedStreamingKafkaParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedStreamingKafkaParameters")]
    pub managed_streaming_kafka_parameters: Option<PipeSourceManagedStreamingKafkaParameters>,


    /// The parameters for using an Active MQ broker as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceActiveMQBrokerParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveMQBrokerParameters")]
    pub active_mqbroker_parameters: Option<PipeSourceActiveMQBrokerParameters>,


    /// The collection of event patterns used to filter events.
    ///
    /// To remove a filter, specify a FilterCriteria object with an empty array of Filter objects.
    ///
    /// For more information, see Events and Event     Patterns in the Amazon EventBridge User Guide.
    ///
    /// Required: No
    ///
    /// Type: FilterCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: Option<FilterCriteria>,


    /// The parameters for using a Kinesis stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceKinesisStreamParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamParameters")]
    pub kinesis_stream_parameters: Option<PipeSourceKinesisStreamParameters>,


    /// The parameters for using a DynamoDB stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeSourceDynamoDBStreamParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBStreamParameters")]
    pub dynamo_dbstream_parameters: Option<PipeSourceDynamoDBStreamParameters>,

}


/// The parameters required to set up a target for your pipe.
///
/// For more information about pipe target parameters, including how to use dynamic path parameters, see Target parameters in the Amazon EventBridge User Guide.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetParameters {


    /// 
    /// Valid JSON text passed to the target. In this case, nothing from the event itself is     passed to the target. For more information, see The JavaScript Object Notation (JSON) Data       Interchange Format.
    /// 
    /// To remove an input template, specify an empty string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputTemplate")]
    pub input_template: Option<String>,


    /// The parameters for using a SageMaker pipeline as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetSageMakerPipelineParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerPipelineParameters")]
    pub sage_maker_pipeline_parameters: Option<PipeTargetSageMakerPipelineParameters>,


    /// The parameters for using a Amazon SQS stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetSqsQueueParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqsQueueParameters")]
    pub sqs_queue_parameters: Option<PipeTargetSqsQueueParameters>,


    /// The parameters for using an Amazon ECS task as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetEcsTaskParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcsTaskParameters")]
    pub ecs_task_parameters: Option<PipeTargetEcsTaskParameters>,


    /// The parameters for using an AWS Batch job as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetBatchJobParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchJobParameters")]
    pub batch_job_parameters: Option<PipeTargetBatchJobParameters>,


    /// These are custom parameters to be used when the target is a Amazon Redshift cluster to invoke the    Amazon Redshift Data API BatchExecuteStatement.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetRedshiftDataParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftDataParameters")]
    pub redshift_data_parameters: Option<PipeTargetRedshiftDataParameters>,


    /// The parameters for using an EventBridge event bus as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetEventBridgeEventBusParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridgeEventBusParameters")]
    pub event_bridge_event_bus_parameters: Option<PipeTargetEventBridgeEventBusParameters>,


    /// These are custom parameter to be used when the target is an API Gateway REST APIs or    EventBridge ApiDestinations.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpParameters")]
    pub http_parameters: Option<PipeTargetHttpParameters>,


    /// The parameters for using an CloudWatch Logs log stream as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetCloudWatchLogsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsParameters")]
    pub cloud_watch_logs_parameters: Option<PipeTargetCloudWatchLogsParameters>,


    /// The parameters for using a Step Functions state machine as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetStateMachineParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepFunctionStateMachineParameters")]
    pub step_function_state_machine_parameters: Option<PipeTargetStateMachineParameters>,


    /// The parameters for using a Kinesis stream as a source.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetKinesisStreamParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamParameters")]
    pub kinesis_stream_parameters: Option<PipeTargetKinesisStreamParameters>,


    /// The parameters for using a Lambda function as a target.
    ///
    /// Required: No
    ///
    /// Type: PipeTargetLambdaFunctionParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaFunctionParameters")]
    pub lambda_function_parameters: Option<PipeTargetLambdaFunctionParameters>,

}


/// The retry strategy that's associated with a job. For more information, see      Automated job retries in the AWS Batch User Guide.
#[derive(Default, serde::Serialize)]
pub struct BatchRetryStrategy {


    /// 
    /// The number of times to move a job to the RUNNABLE status. If the value of attempts is greater than one, the job is retried on      failure the same number of attempts as the value.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attempts")]
    pub attempts: Option<i64>,

}


/// The parameters for using an Active MQ broker as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceActiveMQBrokerParameters {


    /// The credentials needed to access the resource.
    ///
    /// Required: Yes
    ///
    /// Type: MQBrokerAccessCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: MQBrokerAccessCredentials,


    /// The name of the destination queue to consume.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "QueueName")]
    pub queue_name: String,


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,

}


/// The parameters for using a DynamoDB stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceDynamoDBStreamParameters {


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// (Streams only) The position in a stream from which to start reading.
    ///
    /// Valid values: TRIM_HORIZON | LATEST
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPosition")]
    pub starting_position: String,


    /// Define the target queue to send dead-letter queue events to.
    ///
    /// Required: No
    ///
    /// Type: DeadLetterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,


    /// (Streams only) Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<i64>,


    /// (Streams only) The number of batches to process concurrently from each shard. The default value is 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<i64>,


    /// (Streams only) Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPartialBatchItemFailure")]
    pub on_partial_batch_item_failure: Option<String>,


    /// (Streams only) Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<i64>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,

}


/// The task placement strategy for a task or service. To learn more, see Task Placement Strategies in the Amazon Elastic Container Service Service Developer     Guide.
#[derive(Default, serde::Serialize)]
pub struct PlacementStrategy {


    /// 
    /// The field to apply the placement strategy against. For the spread placement strategy,     valid values are instanceId (or host, which has the same effect), or any platform or custom     attribute that is applied to a container instance, such as attribute:ecs.availability-zone.     For the binpack placement strategy, valid values are cpu and memory. For the random placement     strategy, this field is not used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    pub field: Option<String>,


    /// 
    /// The type of placement strategy. The random placement strategy randomly places tasks on     available candidates. The spread placement strategy spreads placement across available     candidates evenly based on the field parameter. The binpack strategy places tasks on available     candidates that have the least available amount of the resource that is specified with the     field parameter. For example, if you binpack on memory, a task is placed on the instance with     the least amount of remaining memory (but still enough to run the task).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// These are custom parameter to be used when the target is an API Gateway REST APIs or    EventBridge ApiDestinations.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetHttpParameters {


    /// 
    /// The path parameter values to be used to populate API Gateway REST API or EventBridge     ApiDestination path wildcards ("*").
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathParameterValues")]
    pub path_parameter_values: Option<Vec<String>>,


    /// 
    /// The query string keys/values that need to be sent as part of request invoking the API Gateway      REST API or EventBridge ApiDestination.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The headers that need to be sent as part of request invoking the API Gateway REST API or     EventBridge ApiDestination.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<std::collections::HashMap<String, String>>,

}


/// The overrides that are sent to a container.
#[derive(Default, serde::Serialize)]
pub struct BatchContainerOverrides {


    /// 
    /// The instance type to use for a multi-node parallel job.
    /// 
    /// NoteThis parameter isn't applicable to single-node container jobs or jobs that run on Fargate resources, and shouldn't be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The type and amount of resources to assign to a container. This overrides the settings in the job definition. The supported resources include GPU, MEMORY,      and VCPU.
    /// 
    /// Required: No
    ///
    /// Type: List of BatchResourceRequirement
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<BatchResourceRequirement>>,


    /// 
    /// The command to send to the container that overrides the default command from the Docker image or the task definition.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,


    /// 
    /// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing      environment variables from the Docker image or the task definition.
    /// 
    /// NoteEnvironment variables cannot start with "AWS Batch". This naming convention is reserved for variables that AWS Batch sets.
    /// 
    /// Required: No
    ///
    /// Type: List of BatchEnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<BatchEnvironmentVariable>>,

}


/// The type and amount of a resource to assign to a container. The supported resources include GPU, MEMORY, and VCPU.
#[derive(Default, serde::Serialize)]
pub struct BatchResourceRequirement {


    /// 
    /// The quantity of the specified resource to reserve for the container. The values vary based on the     type specified.
    /// 
    /// type="GPU"               The number of physical GPUs to reserve for the container. Make sure that the number of GPUs reserved for all          containers in a job doesn't exceed the number of available GPUs on the compute resource that the job is launched          on.        NoteGPUs aren't available for jobs that are running on Fargate resources.                   type="MEMORY"               The memory hard limit (in MiB) present to the container. This parameter is supported for jobs that are          running on EC2 resources. If your container attempts to exceed the memory specified, the container is terminated.          This parameter maps to Memory in the            Create a container section of the Docker Remote API          and the --memory option to docker run.          You must specify at least 4 MiB of memory for a job. This is required but can be specified in several places for          multi-node parallel (MNP) jobs. It must be specified for each node at least once. This parameter maps to          Memory in the            Create a container section of the Docker Remote API and the          --memory option to docker run.        NoteIf you're trying to maximize your resource utilization by providing your jobs as much memory as possible for           a particular instance type, see Memory             management in the AWS Batch User Guide.        For jobs that are running on Fargate resources, then value is the hard limit (in MiB), and          must match one of the supported values and the VCPU values must be one of the values supported for          that memory value.                                                                                                                                                                       value = 512                        VCPU = 0.25                                value = 1024                        VCPU = 0.25 or 0.5                                value = 2048                        VCPU = 0.25, 0.5, or 1                                value = 3072                        VCPU = 0.5, or 1                                value = 4096                        VCPU = 0.5, 1, or 2                                value = 5120, 6144, or 7168                        VCPU = 1 or 2                                value = 8192                        VCPU = 1, 2, 4, or 8                                value = 9216, 10240, 11264, 12288, 13312, 14336, or 15360                        VCPU = 2 or 4                                value = 16384                        VCPU = 2, 4, or 8                                value = 17408, 18432, 19456, 21504, 22528, 23552, 25600, 26624, 27648, 29696, or 30720                        VCPU = 4                                value = 20480, 24576, or 28672                        VCPU = 4 or 8                                value = 36864, 45056, 53248, or 61440                        VCPU = 8                                value = 32768, 40960, 49152, or 57344                        VCPU = 8 or 16                                value = 65536, 73728, 81920, 90112, 98304, 106496, 114688, or 122880                        VCPU = 16                                       type="VCPU"               The number of vCPUs reserved for the container. This parameter maps to CpuShares in the                     Create a container section of the Docker Remote API          and the --cpu-shares option to          docker run. Each vCPU is equivalent to 1,024 CPU shares. For EC2          resources, you must specify at least one vCPU. This is required but can be specified in several places; it must be          specified for each node at least once.        The default for the Fargate On-Demand vCPU resource count quota is 6 vCPUs. For more information about          Fargate quotas, see AWS Fargate quotas in the AWS General Reference.        For jobs that are running on Fargate resources, then value must match one of the supported          values and the MEMORY values must be one of the values supported for that VCPU value.          The supported values are 0.25, 0.5, 1, 2, 4, 8, and 16                                                                                                 value = 0.25                        MEMORY = 512, 1024, or 2048                                value = 0.5                        MEMORY = 1024, 2048, 3072, or 4096                                value = 1                        MEMORY = 2048, 3072, 4096, 5120, 6144, 7168, or 8192                                value = 2                        MEMORY = 4096, 5120, 6144, 7168, 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, or 16384                                value = 4                        MEMORY = 8192, 9216, 10240, 11264, 12288, 13312, 14336, 15360, 16384, 17408, 18432, 19456,              20480, 21504, 22528, 23552, 24576, 25600, 26624, 27648, 28672, 29696, or 30720                                value = 8                        MEMORY = 16384, 20480, 24576, 28672, 32768, 36864, 40960, 45056, 49152, 53248, 57344, or 61440                                             value = 16                        MEMORY = 32768, 40960, 49152, 57344, 65536, 73728, 81920, 90112, 98304, 106496, 114688, or 122880
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The type of resource to assign to a container. The supported resources include GPU, MEMORY, and VCPU.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// An object representing a constraint on task placement. To learn more, see Task Placement Constraints in the Amazon Elastic Container Service Developer     Guide.
#[derive(Default, serde::Serialize)]
pub struct PlacementConstraint {


    /// 
    /// The type of constraint. Use distinctInstance to ensure that each task in a particular     group is running on a different container instance. Use memberOf to restrict the selection to     a group of valid candidates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// A cluster query language expression to apply to the constraint. You cannot specify an     expression if the constraint type is distinctInstance. To learn more, see Cluster Query Language in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

}


/// The AWS Secrets Manager secret that stores your stream credentials.
#[derive(Default, serde::Serialize)]
pub struct MSKAccessCredentials {


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SaslScram512Auth")]
    pub sasl_scram512_auth: Option<String>,


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCertificateTlsAuth")]
    pub client_certificate_tls_auth: Option<String>,

}


/// The parameters for using a Amazon SQS stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetSqsQueueParameters {


    /// 
    /// The FIFO message group ID to use as the target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageGroupId")]
    pub message_group_id: Option<String>,


    /// 
    /// This parameter applies only to FIFO (first-in-first-out) queues.
    /// 
    /// The token used for deduplication of sent messages.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageDeduplicationId")]
    pub message_deduplication_id: Option<String>,

}


/// The parameters for using a Amazon SQS stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceSqsQueueParameters {


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,

}


/// The parameters for using a SageMaker pipeline as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetSageMakerPipelineParameters {


    /// 
    /// List of Parameter names and values for SageMaker Model Building Pipeline execution.
    /// 
    /// Required: No
    ///
    /// Type: List of SageMakerPipelineParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineParameterList")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,

}


/// The AWS Secrets Manager secret that stores your stream credentials.
#[derive(Default, serde::Serialize)]
pub struct SelfManagedKafkaAccessConfigurationCredentials {


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SaslScram256Auth")]
    pub sasl_scram256_auth: Option<String>,


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCertificateTlsAuth")]
    pub client_certificate_tls_auth: Option<String>,


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuth")]
    pub basic_auth: Option<String>,


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SaslScram512Auth")]
    pub sasl_scram512_auth: Option<String>,

}


/// These are custom parameters to be used when the target is a Amazon Redshift cluster to invoke the    Amazon Redshift Data API BatchExecuteStatement.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetRedshiftDataParameters {


    /// 
    /// The SQL statement text to run.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqls")]
    pub sqls: Vec<String>,


    /// 
    /// The name of the SQL statement. You can name the SQL statement when you create it to     identify the query.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatementName")]
    pub statement_name: Option<String>,


    /// 
    /// The name or ARN of the secret that enables access to the database. Required when     authenticating using Secrets Manager.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: Option<String>,


    /// 
    /// The name of the database. Required when authenticating using temporary credentials.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Database")]
    pub database: String,


    /// 
    /// The database user name. Required when authenticating using temporary credentials.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DbUser")]
    pub db_user: Option<String>,


    /// 
    /// Indicates whether to send an event back to EventBridge after the SQL statement     runs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WithEvent")]
    pub with_event: Option<bool>,

}


/// This structure specifies the VPC subnets and security groups for the task, and whether a public IP address is to be used.      This structure is relevant only for ECS tasks that use the awsvpc network mode.
#[derive(Default, serde::Serialize)]
pub struct AwsVpcConfiguration {


    /// 
    /// Specifies the security groups associated with the task. These security groups must all be in the same VPC. You can specify as many      as five security groups. If you do not specify a security group, the default security group for the VPC is used.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,


    /// 
    /// Specifies whether the task's elastic network interface receives a public IP address. You can specify ENABLED only when      LaunchType in EcsParameters is set to FARGATE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<String>,


    /// 
    /// Specifies the subnets associated with the task. These subnets must all be in the same VPC. You can specify as many as 16 subnets.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}


/// The AWS Secrets Manager secret that stores your broker credentials.
#[derive(Default, serde::Serialize)]
pub struct MQBrokerAccessCredentials {


    /// 
    /// The ARN of the Secrets Manager secret.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasicAuth")]
    pub basic_auth: String,

}


/// The parameters for using an AWS Batch job as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetBatchJobParameters {


    /// 
    /// Additional parameters passed to the job that replace parameter substitution placeholders that are set in the job definition. Parameters are specified as a key and      value pair mapping. Parameters included here override any corresponding parameter defaults from the job definition.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<std::collections::HashMap<String, String>>,


    /// The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000.      If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.
    ///
    /// Required: No
    ///
    /// Type: BatchArrayProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArrayProperties")]
    pub array_properties: Option<BatchArrayProperties>,


    /// The overrides that are sent to a container.
    ///
    /// Required: No
    ///
    /// Type: BatchContainerOverrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerOverrides")]
    pub container_overrides: Option<BatchContainerOverrides>,


    /// 
    /// The name of the job. It can be up to 128 letters long. The first character must be alphanumeric, can contain uppercase and lowercase letters, numbers, hyphens (-),      and underscores (_).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobName")]
    pub job_name: String,


    /// 
    /// The job definition used by this job. This value can be one of name, name:revision, or the Amazon Resource Name (ARN) for the job definition.      If name is specified without a revision then the latest active revision is used.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobDefinition")]
    pub job_definition: String,


    /// 
    /// The retry strategy to use for failed jobs. When a retry strategy is specified here, it overrides the retry strategy defined in the job definition.
    /// 
    /// Required: No
    ///
    /// Type: BatchRetryStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryStrategy")]
    pub retry_strategy: Option<BatchRetryStrategy>,


    /// 
    /// A list of dependencies for the job. A job can depend upon a maximum of 20 jobs. You can specify a SEQUENTIAL type dependency without      specifying a job ID for array jobs so that each child array job completes sequentially, starting at index 0. You can also specify an N_TO_N      type dependency with a job ID for array jobs. In that case, each index child of this job must wait for the corresponding index child of each      dependency to complete before it can begin.
    /// 
    /// Required: No
    ///
    /// Type: List of BatchJobDependency
    ///
    /// Update requires: No interruption
    #[serde(rename = "DependsOn")]
    pub depends_on: Option<Vec<BatchJobDependency>>,

}


/// The parameters for using a Kinesis stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceKinesisStreamParameters {


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// Define the target queue to send dead-letter queue events to.
    ///
    /// Required: No
    ///
    /// Type: DeadLetterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,


    /// 
    /// With StartingPosition set to AT_TIMESTAMP, the time from which to start reading, in Unix time seconds.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPositionTimestamp")]
    pub starting_position_timestamp: Option<String>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,


    /// (Streams only) The position in a stream from which to start reading.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPosition")]
    pub starting_position: String,


    /// (Streams only) Define how to handle item process failures. AUTOMATIC_BISECT halves each batch and retry each half until all the records are processed or there is one failed message left in the batch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnPartialBatchItemFailure")]
    pub on_partial_batch_item_failure: Option<String>,


    /// (Streams only) The number of batches to process concurrently from each shard. The default value is 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<i64>,


    /// (Streams only) Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, EventBridge never discards old records.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<i64>,


    /// (Streams only) Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, EventBridge retries failed records until the record expires in the event source.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<i64>,

}


/// A list of files containing the environment variables to pass to a container. You can     specify up to ten environment files. The file must have a .env file     extension. Each line in an environment file should contain an environment variable in     VARIABLE=VALUE format. Lines beginning with # are treated     as comments and are ignored. For more information about the environment variable file     syntax, see Declare default       environment variables in file.
///
/// If there are environment variables specified using the environment       parameter in a container definition, they take precedence over the variables contained       within an environment file. If multiple environment files are specified that contain the       same variable, they're processed from the top down. We recommend that you use unique       variable names. For more information, see Specifying environment        variables in the Amazon Elastic Container Service Developer Guide.
///
/// This parameter is only supported for tasks hosted on Fargate using the       following platform versions:
#[derive(Default, serde::Serialize)]
pub struct EcsEnvironmentFile {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon S3 object containing the environment variable file.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The file type to use. The only supported value is s3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

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


/// The array properties for the submitted job, such as the size of the array. The array size can be between 2 and 10,000.      If you specify array properties for a job, it becomes an array job. This parameter is used only if the target is an AWS Batch job.
#[derive(Default, serde::Serialize)]
pub struct BatchArrayProperties {


    /// 
    /// The size of the array, if this is an array batch job.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: Option<i64>,

}


/// These are custom parameter to be used when the target is an API Gateway REST APIs or     EventBridge ApiDestinations. In the latter case, these are merged with any     InvocationParameters specified on the Connection, with any values from the Connection taking     precedence.
#[derive(Default, serde::Serialize)]
pub struct PipeEnrichmentHttpParameters {


    /// 
    /// The path parameter values to be used to populate API Gateway REST API or EventBridge     ApiDestination path wildcards ("*").
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathParameterValues")]
    pub path_parameter_values: Option<Vec<String>>,


    /// 
    /// The headers that need to be sent as part of request invoking the API Gateway REST API or     EventBridge ApiDestination.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The query string keys/values that need to be sent as part of request invoking the API Gateway      REST API or EventBridge ApiDestination.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<std::collections::HashMap<String, String>>,

}


/// Filter events using an event pattern. For more information, see Events and Event     Patterns in the Amazon EventBridge User Guide.
#[derive(Default, serde::Serialize)]
pub struct Filter {


    /// 
    /// The event pattern.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

}


/// The parameters for using a self-managed Apache Kafka stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceSelfManagedKafkaParameters {


    /// The credentials needed to access the resource.
    ///
    /// Required: No
    ///
    /// Type: SelfManagedKafkaAccessConfigurationCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: Option<SelfManagedKafkaAccessConfigurationCredentials>,


    /// The name of the destination queue to consume.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsumerGroupID")]
    pub consumer_group_id: Option<String>,


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// (Streams only) The position in a stream from which to start reading.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<String>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,


    /// The ARN of the Secrets Manager secret used for certification.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerRootCaCertificate")]
    pub server_root_ca_certificate: Option<String>,


    /// The name of the topic that the pipe will read from.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicName")]
    pub topic_name: String,


    /// 
    /// An array of server URLs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalBootstrapServers")]
    pub additional_bootstrap_servers: Option<Vec<String>>,


    /// This structure specifies the VPC subnets and security groups for the stream, and whether a public IP address is to be used.
    ///
    /// Required: No
    ///
    /// Type: SelfManagedKafkaAccessConfigurationVpc
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vpc")]
    pub vpc: Option<SelfManagedKafkaAccessConfigurationVpc>,

}


/// The type and amount of a resource to assign to a container. The supported resource     types are GPUs and Elastic Inference accelerators. For more information, see Working with       GPUs on Amazon ECS or Working with        Amazon Elastic Inference on Amazon ECS in the     Amazon Elastic Container Service Developer Guide
#[derive(Default, serde::Serialize)]
pub struct EcsResourceRequirement {


    /// 
    /// The value for the specified resource type.
    /// 
    /// If the GPU type is used, the value is the number of physical     GPUs the Amazon ECS container agent reserves for the container. The number     of GPUs that's reserved for all containers in a task can't exceed the number of     available GPUs on the container instance that the task is launched on.
    /// 
    /// If the InferenceAccelerator type is used, the value matches     the deviceName for an InferenceAccelerator specified in a     task definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The type of resource to assign to a container. The supported values are     GPU or InferenceAccelerator.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}


/// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can      override the existing environment variables from the Docker image or the task definition. You must also specify a container name.
#[derive(Default, serde::Serialize)]
pub struct EcsEnvironmentVariable {


    /// 
    /// The name of the key-value pair. For environment variables, this is the name of the environment variable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The value of the key-value pair. For environment variables, this is the value of the environment variable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


/// The amount of ephemeral storage to allocate for the task. This parameter is used to     expand the total amount of ephemeral storage available, beyond the default amount, for     tasks hosted on Fargate. For more information, see Fargate task       storage in the Amazon ECS User Guide for Fargate.
#[derive(Default, serde::Serialize)]
pub struct EcsEphemeralStorage {


    /// 
    /// The total amount, in GiB, of ephemeral storage to set for the task. The minimum     supported value is 21 GiB and the maximum supported value is     200 GiB.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInGiB")]
    pub size_in_gi_b: i64,

}


/// The parameters for using an Amazon ECS task as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetEcsTaskParameters {


    /// 
    /// Specifies whether to enable Amazon ECS managed tags for the task. For more information,     see Tagging Your Amazon ECS Resources in the Amazon Elastic Container Service Developer     Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableECSManagedTags")]
    pub enable_ecsmanaged_tags: Option<bool>,


    /// 
    /// The metadata that you apply to the task to help you categorize and organize them. Each tag     consists of a key and an optional value, both of which you define. To learn more, see RunTask in the Amazon ECS API Reference.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the platform version for the task. Specify only the numeric portion of the     platform version, such as 1.1.0.
    /// 
    /// This structure is used only if LaunchType is FARGATE. For more     information about valid platform versions, see AWS Fargate Platform       Versions in the Amazon Elastic Container Service Developer        Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,


    /// 
    /// An array of placement constraint objects to use for the task. You can specify up to 10     constraints per task (including constraints in the task definition and those specified at     runtime).
    /// 
    /// Required: No
    ///
    /// Type: List of PlacementConstraint
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,


    /// 
    /// Specifies an Amazon ECS task group for the task. The maximum length is 255 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Group")]
    pub group: Option<String>,


    /// 
    /// The placement strategy objects to use for the task. You can specify a maximum of five     strategy rules per task.
    /// 
    /// Required: No
    ///
    /// Type: List of PlacementStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementStrategy")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,


    /// The overrides that are associated with a task.
    ///
    /// Required: No
    ///
    /// Type: EcsTaskOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "Overrides")]
    pub overrides: Option<EcsTaskOverride>,


    /// 
    /// Specifies the launch type on which your task is running. The launch type that you specify     here must match one of the launch type (compatibilities) of the target task. The     FARGATE value is supported only in the Regions where AWS Fargate with Amazon ECS     is supported. For more information, see AWS Fargate on Amazon ECS in     the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<String>,


    /// 
    /// The reference ID to use for the task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceId")]
    pub reference_id: Option<String>,


    /// 
    /// Specifies whether to propagate the tags from the task definition to the task. If no value     is specified, the tags are not propagated. Tags can only be propagated to the task during task     creation. To add tags to a task after task creation, use the TagResource API action.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<String>,


    /// 
    /// The ARN of the task definition to use if the event target is an Amazon ECS task.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: String,


    /// 
    /// The capacity provider strategy to use for the task.
    /// 
    /// If a capacityProviderStrategy is specified, the launchType     parameter must be omitted. If no capacityProviderStrategy or launchType is     specified, the defaultCapacityProviderStrategy for the cluster is used.
    /// 
    /// Required: No
    ///
    /// Type: List of CapacityProviderStrategyItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProviderStrategy")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,


    /// 
    /// Use this structure if the Amazon ECS task uses the awsvpc network mode. This     structure specifies the VPC subnets and security groups associated with the task, and whether     a public IP address is to be used. This structure is required if LaunchType is     FARGATE because the awsvpc mode is required for Fargate     tasks.
    /// 
    /// If you specify NetworkConfiguration when the target ECS task does not use the     awsvpc network mode, the task fails.
    /// 
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,


    /// 
    /// The number of tasks to create based on TaskDefinition. The default is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskCount")]
    pub task_count: Option<i64>,


    /// 
    /// Whether or not to enable the execute command functionality for the containers in this     task. If true, this enables execute command functionality on all containers in the     task.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableExecuteCommand")]
    pub enable_execute_command: Option<bool>,

}


/// The parameters for using an EventBridge event bus as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetEventBridgeEventBusParameters {


    /// 
    /// The source of the event.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Option<String>,


    /// 
    /// The URL subdomain of the endpoint. For example, if the URL for Endpoint is https://abcde.veo.endpoints.event.amazonaws.com, then the EndpointId is abcde.veo.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointId")]
    pub endpoint_id: Option<String>,


    /// 
    /// A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailType")]
    pub detail_type: Option<String>,


    /// 
    /// The time stamp of the event, per RFC3339. If no time stamp is provided, the time stamp of the PutEvents call is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Time")]
    pub time: Option<String>,


    /// 
    /// AWS resources, identified by Amazon Resource Name (ARN), which the event primarily     concerns. Any number, including zero, may be present.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,

}


/// The parameters for using a Kinesis stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetKinesisStreamParameters {


    /// 
    /// Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters      for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard.      Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this      hashing mechanism, all data records with the same partition key map to the same shard within the stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

}


/// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing      environment variables from the Docker image or the task definition.
#[derive(Default, serde::Serialize)]
pub struct BatchEnvironmentVariable {


    /// 
    /// The value of the key-value pair. For environment variables, this is the value of the environment variable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The name of the key-value pair. For environment variables, this is the name of the environment variable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// Name/Value pair of a parameter to start execution of a SageMaker Model Building     Pipeline.
#[derive(Default, serde::Serialize)]
pub struct SageMakerPipelineParameter {


    /// 
    /// Value of parameter to start execution of a SageMaker Model Building Pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// Name of parameter to start execution of a SageMaker Model Building Pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// The parameters for using a Lambda function as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetLambdaFunctionParameters {


    /// 
    /// Specify whether to invoke the function synchronously or asynchronously.
    /// 
    /// REQUEST_RESPONSE (default) - Invoke synchronously. This corresponds to the RequestResponse option in the InvocationType parameter for the Lambda Invoke API.            FIRE_AND_FORGET - Invoke asynchronously. This corresponds to the Event option in the InvocationType parameter for the Lambda Invoke API.
    /// 
    /// For more information, see Invocation types in the Amazon EventBridge User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationType")]
    pub invocation_type: Option<String>,

}


/// The overrides that are sent to a container. An empty container override can be passed in. An example of an empty      container override is {"containerOverrides": [ ] }. If a non-empty container override is specified, the name parameter must be included.
#[derive(Default, serde::Serialize)]
pub struct EcsContainerOverride {


    /// 
    /// The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can      override the existing environment variables from the Docker image or the task definition. You must also specify a container name.
    /// 
    /// Required: No
    ///
    /// Type: List of EcsEnvironmentVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    pub environment: Option<Vec<EcsEnvironmentVariable>>,


    /// 
    /// A list of files containing the environment variables to pass to a container, instead of the value from the container definition.
    /// 
    /// Required: No
    ///
    /// Type: List of EcsEnvironmentFile
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentFiles")]
    pub environment_files: Option<Vec<EcsEnvironmentFile>>,


    /// 
    /// The name of the container that receives the override. This parameter is required if any override is specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition.      If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,


    /// 
    /// The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition.      You must also specify a container name.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,


    /// 
    /// The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU.
    /// 
    /// Required: No
    ///
    /// Type: List of EcsResourceRequirement
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceRequirements")]
    pub resource_requirements: Option<Vec<EcsResourceRequirement>>,


    /// 
    /// The number of cpu units reserved for the container, instead of the default value from the task definition. You must also specify a container name.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cpu")]
    pub cpu: Option<i64>,


    /// 
    /// The command to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,

}


/// An object that represents an AWS Batch job dependency.
#[derive(Default, serde::Serialize)]
pub struct BatchJobDependency {


    /// 
    /// The type of the job dependency.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// The job ID of the AWS Batch job that's associated with this dependency.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobId")]
    pub job_id: Option<String>,

}


/// This structure specifies the network configuration for an Amazon ECS task.
#[derive(Default, serde::Serialize)]
pub struct NetworkConfiguration {


    /// 
    /// Use this structure to specify the VPC subnets and security groups for the task, and     whether a public IP address is to be used. This structure is relevant only for ECS tasks that     use the awsvpc network mode.
    /// 
    /// Required: No
    ///
    /// Type: AwsVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsvpcConfiguration")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,

}


/// The overrides that are associated with a task.
#[derive(Default, serde::Serialize)]
pub struct EcsTaskOverride {


    /// 
    /// The cpu override for the task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cpu")]
    pub cpu: Option<String>,


    /// 
    /// One or more container overrides that are sent to a task.
    /// 
    /// Required: No
    ///
    /// Type: List of EcsContainerOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerOverrides")]
    pub container_overrides: Option<Vec<EcsContainerOverride>>,


    /// 
    /// The Elastic Inference accelerator override for the task.
    /// 
    /// Required: No
    ///
    /// Type: List of EcsInferenceAcceleratorOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "InferenceAcceleratorOverrides")]
    pub inference_accelerator_overrides: Option<Vec<EcsInferenceAcceleratorOverride>>,


    /// 
    /// The ephemeral storage setting override for the task.
    /// 
    /// NoteThis parameter is only supported for tasks hosted on Fargate that       use the following platform versions:                           Linux platform version 1.4.0 or later.               Windows platform version 1.0.0 or later.
    /// 
    /// Required: No
    ///
    /// Type: EcsEphemeralStorage
    ///
    /// Update requires: No interruption
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<EcsEphemeralStorage>,


    /// 
    /// The Amazon Resource Name (ARN) of the task execution IAM role override for the task. For more     information, see Amazon ECS task       execution IAM role in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,


    /// 
    /// The memory override for the task.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Memory")]
    pub memory: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers     in this task are granted the permissions that are specified in this role. For more     information, see IAM Role for Tasks     in the Amazon Elastic Container Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskRoleArn")]
    pub task_role_arn: Option<String>,

}


/// The parameters required to set up enrichment on your pipe.
#[derive(Default, serde::Serialize)]
pub struct PipeEnrichmentParameters {


    /// 
    /// Valid JSON text passed to the enrichment. In this case, nothing from the event itself is     passed to the enrichment. For more information, see The JavaScript Object Notation (JSON) Data       Interchange Format.
    /// 
    /// To remove an input template, specify an empty string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputTemplate")]
    pub input_template: Option<String>,


    /// 
    /// Contains the HTTP parameters to use when the target is a API Gateway REST endpoint or     EventBridge ApiDestination.
    /// 
    /// If you specify an API Gateway REST API or EventBridge ApiDestination as a target, you can     use this parameter to specify headers, path parameters, and query string keys/values as part     of your target invoking request. If you're using ApiDestinations, the corresponding Connection     can also have these values configured. In case of any conflicting keys, values from the     Connection take precedence.
    /// 
    /// Required: No
    ///
    /// Type: PipeEnrichmentHttpParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpParameters")]
    pub http_parameters: Option<PipeEnrichmentHttpParameters>,

}


/// The parameters for using an CloudWatch Logs log stream as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetCloudWatchLogsParameters {


    /// 
    /// The name of the log stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,


    /// 
    /// The time the event occurred, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,

}


/// The collection of event patterns used to filter events.
///
/// To remove a filter, specify a FilterCriteria object with an empty array of Filter objects.
///
/// For more information, see Events and Event     Patterns in the Amazon EventBridge User Guide.
#[derive(Default, serde::Serialize)]
pub struct FilterCriteria {


    /// 
    /// The event patterns.
    /// 
    /// Required: No
    ///
    /// Type: List of Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,

}


/// The parameters for using an MSK stream as a source.
#[derive(Default, serde::Serialize)]
pub struct PipeSourceManagedStreamingKafkaParameters {


    /// The credentials needed to access the resource.
    ///
    /// Required: No
    ///
    /// Type: MSKAccessCredentials
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    pub credentials: Option<MSKAccessCredentials>,


    /// The name of the topic that the pipe will read from.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicName")]
    pub topic_name: String,


    /// (Streams only) The position in a stream from which to start reading.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<String>,


    /// The maximum length of a time to wait for events.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// The name of the destination queue to consume.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConsumerGroupID")]
    pub consumer_group_id: Option<String>,


    /// The maximum number of records to include in each batch.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,

}


/// The parameters for using a Step Functions state machine as a target.
#[derive(Default, serde::Serialize)]
pub struct PipeTargetStateMachineParameters {


    /// 
    /// Specify whether to invoke the Step Functions state machine synchronously or asynchronously.
    /// 
    /// REQUEST_RESPONSE (default) - Invoke synchronously. For more information, see StartSyncExecution in the AWS Step Functions API Reference.       NoteREQUEST_RESPONSE is not supported for STANDARD state machine workflows.            FIRE_AND_FORGET - Invoke asynchronously. For more information, see StartExecution in the AWS Step Functions API Reference.
    /// 
    /// For more information, see Invocation types in the Amazon EventBridge User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvocationType")]
    pub invocation_type: Option<String>,

}


/// Details on an Elastic Inference accelerator task override. This parameter is used to     override the Elastic Inference accelerator specified in the task definition. For more     information, see Working with Amazon       Elastic Inference on Amazon ECS in the     Amazon Elastic Container Service Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct EcsInferenceAcceleratorOverride {


    /// 
    /// The Elastic Inference accelerator device name to override for the task. This parameter must match a deviceName specified in the task definition.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,


    /// 
    /// The Elastic Inference accelerator type to use.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceType")]
    pub device_type: Option<String>,

}