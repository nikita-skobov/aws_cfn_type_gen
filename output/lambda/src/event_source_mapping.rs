

/// The AWS::Lambda::EventSourceMapping resource creates a mapping between an event source and    an AWS Lambda function. Lambda reads items from the event source and triggers the function.
///
/// For details about each event source type, see the following topics. In particular, each of the topics    describes the required and optional parameters for the specific event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventSourceMapping {


    /// 
    /// Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.
    /// 
    /// Required: No
    ///
    /// Type: AmazonManagedKafkaEventSourceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AmazonManagedKafkaEventSourceConfig")]
    pub amazon_managed_kafka_event_source_config: Option<AmazonManagedKafkaEventSourceConfig>,


    /// 
    /// The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation  (6 MB).
    /// 
    /// Amazon Kinesis – Default 100. Max 10,000.                        Amazon DynamoDB Streams – Default 100. Max 10,000.                        Amazon Simple Queue Service – Default 10. For standard queues the max is 10,000. For FIFO queues the max is 10.                        Amazon Managed Streaming for Apache Kafka – Default 100. Max 10,000.                        Self-managed Apache Kafka – Default 100. Max 10,000.                        Amazon MQ (ActiveMQ and RabbitMQ) – Default 100. Max 10,000.                        DocumentDB – Default 100. Max 10,000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: Option<i64>,


    /// 
    /// (Kinesis and DynamoDB Streams only) If the function returns an error, split the batch in two and retry. The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BisectBatchOnFunctionError")]
    pub bisect_batch_on_function_error: Option<bool>,


    /// 
    /// (Kinesis and DynamoDB Streams only) An Amazon SQS queue or Amazon SNS topic destination for discarded records.
    /// 
    /// Required: No
    ///
    /// Type: DestinationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationConfig")]
    pub destination_config: Option<DestinationConfig>,


    /// 
    /// Specific configuration settings for a DocumentDB event source.
    /// 
    /// Required: No
    ///
    /// Type: DocumentDBEventSourceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentDBEventSourceConfig")]
    pub document_dbevent_source_config: Option<DocumentDBEventSourceConfig>,


    /// 
    /// When true, the event source mapping is active. When false, Lambda pauses polling and invocation.
    /// 
    /// Default: True
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The Amazon Resource Name (ARN) of the event source.
    /// 
    /// Amazon Kinesis – The ARN of the data stream or a stream consumer.                        Amazon DynamoDB Streams – The ARN of the stream.                        Amazon Simple Queue Service – The ARN of the queue.                        Amazon Managed Streaming for Apache Kafka – The ARN of the cluster.                        Amazon MQ – The ARN of the broker.                        Amazon DocumentDB – The ARN of the DocumentDB change stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)
    ///
    /// Update requires: Replacement
    #[serde(rename = "EventSourceArn")]
    pub event_source_arn: Option<String>,


    /// 
    /// An object that defines the filter criteria that   determine whether Lambda should process an event. For more information, see Lambda event filtering.
    /// 
    /// Required: No
    ///
    /// Type: FilterCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterCriteria")]
    pub filter_criteria: Option<FilterCriteria>,


    /// 
    /// The name of the Lambda function.
    /// 
    /// Name formats                                                   Function name – MyFunction.                        Function ARN – arn:aws:lambda:us-west-2:123456789012:function:MyFunction.                        Version or Alias ARN – arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD.                        Partial ARN – 123456789012:function:MyFunction.
    /// 
    /// The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64    characters in length.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Pattern: (arn:(aws[a-zA-Z-]*)?:lambda:)?([a-z]{2}(-gov)?-[a-z]+-\d{1}:)?(\d{12}:)?(function:)?([a-zA-Z0-9-_]+)(:(\$LATEST|[a-zA-Z0-9-_]+))?
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionName")]
    pub function_name: String,


    /// 
    /// (Streams and SQS) A list of current response type enums applied to the event source mapping.
    /// 
    /// Valid Values: ReportBatchItemFailures
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionResponseTypes")]
    pub function_response_types: Option<Vec<String>>,


    /// 
    /// The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function.
    /// 
    /// Default (Kinesis, DynamoDB, Amazon SQS event sources): 0
    /// 
    /// Default (Amazon MSK, Kafka, Amazon MQ, Amazon DocumentDB event sources): 500 ms
    /// 
    /// Related setting: For Amazon SQS event sources, when you set BatchSize    to a value greater than 10, you must set MaximumBatchingWindowInSeconds to at least 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    pub maximum_batching_window_in_seconds: Option<i64>,


    /// 
    /// (Kinesis and DynamoDB Streams only) Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, Lambda never discards old records.
    /// 
    /// NoteThe minimum valid value for maximum record age is 60s. Although values less than 60 and greater than -1 fall within the parameter's absolute range, they are not allowed
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: -1
    ///
    /// Maximum: 604800
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    pub maximum_record_age_in_seconds: Option<i64>,


    /// 
    /// (Kinesis and DynamoDB Streams only) Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, Lambda retries failed records until the record expires in the event source.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: -1
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<i64>,


    /// 
    /// (Kinesis and DynamoDB Streams only) The number of batches to process concurrently from each shard. The default value is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelizationFactor")]
    pub parallelization_factor: Option<i64>,


    /// 
    /// (Amazon MQ) The name of the Amazon MQ broker destination queue to consume.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Queues")]
    pub queues: Option<Vec<String>>,


    /// 
    /// (Amazon SQS only) The scaling configuration for the event source. For more information, see Configuring maximum concurrency for Amazon SQS event sources.
    /// 
    /// Required: No
    ///
    /// Type: ScalingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingConfig")]
    pub scaling_config: Option<ScalingConfig>,


    /// 
    /// The self-managed Apache Kafka cluster for your event source.
    /// 
    /// Required: No
    ///
    /// Type: SelfManagedEventSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "SelfManagedEventSource")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,


    /// 
    /// Specific configuration settings for a self-managed Apache Kafka event source.
    /// 
    /// Required: No
    ///
    /// Type: SelfManagedKafkaEventSourceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "SelfManagedKafkaEventSourceConfig")]
    pub self_managed_kafka_event_source_config: Option<SelfManagedKafkaEventSourceConfig>,


    /// 
    /// An array of the authentication protocol, VPC components, or virtual host to secure and define your event source.
    /// 
    /// Required: No
    ///
    /// Type: List of SourceAccessConfiguration
    ///
    /// Maximum: 22
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAccessConfigurations")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,


    /// 
    /// The position in a stream from which to start reading. Required for Amazon Kinesis and Amazon DynamoDB.
    /// 
    /// LATEST - Read only new records.        TRIM_HORIZON - Process all available records.        AT_TIMESTAMP - Specify a time from which to start reading records.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPosition")]
    pub starting_position: Option<String>,


    /// 
    /// With StartingPosition set to AT_TIMESTAMP, the time from which to start    reading, in Unix time seconds.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "StartingPositionTimestamp")]
    pub starting_position_timestamp: Option<f64>,


    /// 
    /// The name of the Kafka topic.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topics")]
    pub topics: Option<Vec<String>>,


    /// 
    /// (Kinesis and DynamoDB Streams only) The duration in seconds of a processing window for DynamoDB and Kinesis Streams event sources. A value of 0 seconds indicates no tumbling window.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 900
    ///
    /// Update requires: No interruption
    #[serde(rename = "TumblingWindowInSeconds")]
    pub tumbling_window_in_seconds: Option<i64>,

}



impl cfn_resources::CfnResource for CfnEventSourceMapping {
    fn type_string() -> &'static str {
        "AWS::Lambda::EventSourceMapping"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonManagedKafkaEventSourceConfig {


    /// 
    /// The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources.  After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see  Customizable consumer group ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: [a-zA-Z0-9-\/*:_+=.@-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConsumerGroupId")]
    pub consumer_group_id: Option<String>,

}




/// A configuration object that specifies the destination of an event after Lambda processes it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationConfig {


    /// 
    /// The destination configuration for failed invocations.
    /// 
    /// Required: No
    ///
    /// Type: OnFailure
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnFailure")]
    pub on_failure: Option<OnFailure>,

}




/// Specific configuration settings for a DocumentDB event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentDBEventSourceConfig {


    /// 
    /// The name of the collection to consume within the database. If you do not specify a collection, Lambda consumes all collections.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 57
    ///
    /// Pattern: (^(?!(system\x2e)))(^[_a-zA-Z0-9])([^$]*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectionName")]
    pub collection_name: Option<String>,


    /// 
    /// The name of the database to consume within the DocumentDB cluster.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: [^ /\.$\x22]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// Determines what DocumentDB sends to your event stream during document update operations. If set to UpdateLookup, DocumentDB sends a delta describing the changes, along with a copy of the entire document. Otherwise, DocumentDB sends only a partial document that contains the changes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Default | UpdateLookup
    ///
    /// Update requires: No interruption
    #[serde(rename = "FullDocument")]
    pub full_document: Option<DocumentDBEventSourceConfigFullDocumentEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DocumentDBEventSourceConfigFullDocumentEnum {

    /// Default
    #[serde(rename = "Default")]
    Default,

    /// UpdateLookup
    #[serde(rename = "UpdateLookup")]
    Updatelookup,

}

impl Default for DocumentDBEventSourceConfigFullDocumentEnum {
    fn default() -> Self {
        DocumentDBEventSourceConfigFullDocumentEnum::Default
    }
}



/// The list of bootstrap servers for your Kafka brokers in the following format: "KafkaBootstrapServers": ["abc.xyz.com:xxxx","abc2.xyz.com:xxxx"].
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Endpoints {


    /// 
    /// The list of bootstrap servers for your Kafka brokers in the following format: "KafkaBootstrapServers": ["abc.xyz.com:xxxx","abc2.xyz.com:xxxx"].
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaBootstrapServers")]
    pub kafka_bootstrap_servers: Option<Vec<String>>,

}




/// A structure within a FilterCriteria object that defines an event filtering pattern.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Filter {


    /// 
    /// A filter pattern. For more information on the syntax of a filter pattern, see        Filter rule syntax.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 4096
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

}




/// An object that contains the filters for an event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterCriteria {


    /// 
    /// A list of filters.
    /// 
    /// Required: No
    ///
    /// Type: List of Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,

}




/// A destination for events that failed processing.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnFailure {


    /// 
    /// The Amazon Resource Name (ARN) of the destination resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 350
    ///
    /// Pattern: ^$|arn:(aws[a-zA-Z0-9-]*):([a-zA-Z0-9\-])+:([a-z]{2}(-gov)?-[a-z]+-\d{1})?:(\d{12})?:(.*)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

}




/// (Amazon SQS only) The scaling configuration for the event source. To remove the configuration, pass an empty value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingConfig {


    /// 
    /// Limits the number of concurrent instances that the Amazon SQS event source can invoke.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumConcurrency")]
    pub maximum_concurrency: Option<i64>,

}




/// The self-managed Apache Kafka cluster for your event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SelfManagedEventSource {


    /// 
    /// The list of bootstrap servers for your Kafka brokers in the following format: "KafkaBootstrapServers": ["abc.xyz.com:xxxx","abc2.xyz.com:xxxx"].
    /// 
    /// Required: No
    ///
    /// Type: Endpoints
    ///
    /// Update requires: Replacement
    #[serde(rename = "Endpoints")]
    pub endpoints: Option<Endpoints>,

}




/// Specific configuration settings for a self-managed Apache Kafka event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SelfManagedKafkaEventSourceConfig {


    /// 
    /// The identifier for the Kafka consumer group to join. The consumer group ID must be unique among all your Kafka event sources.  After creating a Kafka event source mapping with the consumer group ID specified, you cannot update this value. For more information, see  Customizable consumer group ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: [a-zA-Z0-9-\/*:_+=.@-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConsumerGroupId")]
    pub consumer_group_id: Option<String>,

}




/// An array of the authentication protocol, VPC components, or virtual host to secure and define your event source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceAccessConfiguration {


    /// 
    /// The type of authentication protocol, VPC components, or virtual host for your event source. For example: "Type":"SASL_SCRAM_512_AUTH".
    /// 
    /// BASIC_AUTH – (Amazon MQ) The AWS Secrets Manager secret that stores your broker credentials.                        BASIC_AUTH – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL/PLAIN authentication of your Apache Kafka brokers.                        VPC_SUBNET – (Self-managed Apache Kafka) The subnets associated with your VPC. Lambda connects to these subnets to fetch data from your self-managed Apache Kafka cluster.                        VPC_SECURITY_GROUP – (Self-managed Apache Kafka) The VPC security group used to manage access to your self-managed Apache Kafka brokers.                        SASL_SCRAM_256_AUTH – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL SCRAM-256 authentication of your self-managed Apache Kafka brokers.                        SASL_SCRAM_512_AUTH – (Amazon MSK, Self-managed Apache Kafka) The Secrets Manager ARN of your secret key used for SASL SCRAM-512 authentication of your self-managed Apache Kafka brokers.                        VIRTUAL_HOST –- (RabbitMQ) The name of the virtual host in your RabbitMQ broker. Lambda uses this RabbitMQ host as the event source.  This property cannot be specified in an UpdateEventSourceMapping API call.                        CLIENT_CERTIFICATE_TLS_AUTH – (Amazon MSK, self-managed Apache Kafka) The Secrets Manager ARN of your secret key containing the certificate chain (X.509 PEM),  private key (PKCS#8 PEM), and private key password (optional) used for mutual TLS authentication of your MSK/Apache Kafka brokers.                        SERVER_ROOT_CA_CERTIFICATE – (Self-managed Apache Kafka) The Secrets Manager ARN of your secret key containing the root CA certificate (X.509 PEM) used for TLS encryption of your Apache Kafka brokers.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BASIC_AUTH | CLIENT_CERTIFICATE_TLS_AUTH | SASL_SCRAM_256_AUTH | SASL_SCRAM_512_AUTH | SERVER_ROOT_CA_CERTIFICATE | VIRTUAL_HOST | VPC_SECURITY_GROUP | VPC_SUBNET
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<SourceAccessConfigurationTypeEnum>,


    /// 
    /// The value for your chosen configuration in Type. For example: "URI": "arn:aws:secretsmanager:us-east-1:01234567890:secret:MyBrokerSecretName".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: [a-zA-Z0-9-\/*:_+=.@-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "URI")]
    pub uri: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceAccessConfigurationTypeEnum {

    /// BASIC_AUTH
    #[serde(rename = "BASIC_AUTH")]
    Basicauth,

    /// CLIENT_CERTIFICATE_TLS_AUTH
    #[serde(rename = "CLIENT_CERTIFICATE_TLS_AUTH")]
    Clientcertificatetlsauth,

    /// SASL_SCRAM_256_AUTH
    #[serde(rename = "SASL_SCRAM_256_AUTH")]
    Saslscram256auth,

    /// SASL_SCRAM_512_AUTH
    #[serde(rename = "SASL_SCRAM_512_AUTH")]
    Saslscram512auth,

    /// SERVER_ROOT_CA_CERTIFICATE
    #[serde(rename = "SERVER_ROOT_CA_CERTIFICATE")]
    Serverrootcacertificate,

    /// VIRTUAL_HOST
    #[serde(rename = "VIRTUAL_HOST")]
    Virtualhost,

    /// VPC_SECURITY_GROUP
    #[serde(rename = "VPC_SECURITY_GROUP")]
    Vpcsecuritygroup,

    /// VPC_SUBNET
    #[serde(rename = "VPC_SUBNET")]
    Vpcsubnet,

}

impl Default for SourceAccessConfigurationTypeEnum {
    fn default() -> Self {
        SourceAccessConfigurationTypeEnum::Basicauth
    }
}

