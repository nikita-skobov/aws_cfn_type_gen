

/// The AWS::KinesisFirehose::DeliveryStream resource specifies an Amazon     Kinesis Data Firehose (Kinesis Data Firehose) delivery stream that delivers real-time     streaming data to an Amazon Simple Storage Service (Amazon S3), Amazon Redshift, or Amazon     Elasticsearch Service (Amazon ES) destination. For more information, see Creating an Amazon       Kinesis Data Firehose Delivery Stream in the Amazon Kinesis Data       Firehose Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeliveryStream {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AmazonOpenSearchServerlessDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonOpenSearchServerlessDestinationConfiguration")]
    pub amazon_open_search_serverless_destination_configuration: Option<AmazonOpenSearchServerlessDestinationConfiguration>,


    /// 
    /// The destination in Amazon OpenSearch Service. You can specify only one     destination.
    /// 
    /// Required: Conditional
    ///
    /// Type: AmazonopensearchserviceDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AmazonopensearchserviceDestinationConfiguration")]
    pub amazonopensearchservice_destination_configuration: Option<AmazonopensearchserviceDestinationConfiguration>,


    /// 
    /// Specifies the type and Amazon Resource Name (ARN) of the CMK to use for Server-Side     Encryption (SSE).
    /// 
    /// Required: No
    ///
    /// Type: DeliveryStreamEncryptionConfigurationInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    pub delivery_stream_encryption_configuration_input: Option<DeliveryStreamEncryptionConfigurationInput>,


    /// 
    /// The name of the delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: Option<String>,


    /// 
    /// The delivery stream type. This can be one of the following values:
    /// 
    /// DirectPut: Provider applications access the delivery stream        directly.                        KinesisStreamAsSource: The delivery stream uses a Kinesis data        stream as a source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DirectPut | KinesisStreamAsSource
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeliveryStreamType")]
    pub delivery_stream_type: Option<DeliveryStreamDeliveryStreamTypeEnum>,


    /// 
    /// An Amazon ES destination for the delivery stream.
    /// 
    /// Conditional. You must specify only one destination configuration.
    /// 
    /// If you change the delivery stream destination from an Amazon ES destination to an     Amazon S3 or Amazon Redshift destination, update requires some interruptions.
    /// 
    /// Required: Conditional
    ///
    /// Type: ElasticsearchDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,


    /// 
    /// An Amazon S3 destination for the delivery stream.
    /// 
    /// Conditional. You must specify only one destination configuration.
    /// 
    /// If you change the delivery stream destination from an Amazon Extended S3 destination     to an Amazon ES destination, update requires some interruptions.
    /// 
    /// Required: Conditional
    ///
    /// Type: ExtendedS3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,


    /// 
    /// Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint     destination. You can specify only one destination.
    /// 
    /// Required: No
    ///
    /// Type: HttpEndpointDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpEndpointDestinationConfiguration")]
    pub http_endpoint_destination_configuration: Option<HttpEndpointDestinationConfiguration>,


    /// 
    /// When a Kinesis stream is used as the source for the delivery stream, a KinesisStreamSourceConfiguration containing the Kinesis stream ARN and the role     ARN for the source stream.
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamSourceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "KinesisStreamSourceConfiguration")]
    pub kinesis_stream_source_configuration: Option<KinesisStreamSourceConfiguration>,


    /// 
    /// An Amazon Redshift destination for the delivery stream.
    /// 
    /// Conditional. You must specify only one destination configuration.
    /// 
    /// If you change the delivery stream destination from an Amazon Redshift destination to     an Amazon ES destination, update requires some interruptions.
    /// 
    /// Required: Conditional
    ///
    /// Type: RedshiftDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedshiftDestinationConfiguration")]
    pub redshift_destination_configuration: Option<RedshiftDestinationConfiguration>,


    /// 
    /// The S3DestinationConfiguration property type specifies an Amazon Simple     Storage Service (Amazon S3) destination to which Amazon Kinesis Data Firehose (Kinesis Data     Firehose) delivers data.
    /// 
    /// Conditional. You must specify only one destination configuration.
    /// 
    /// If you change the delivery stream destination from an Amazon S3 destination to an     Amazon ES destination, update requires some interruptions.
    /// 
    /// Required: Conditional
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3DestinationConfiguration")]
    pub s3_destination_configuration: Option<S3DestinationConfiguration>,


    /// 
    /// The configuration of a destination in Splunk for the delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: SplunkDestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SplunkDestinationConfiguration")]
    pub splunk_destination_configuration: Option<SplunkDestinationConfiguration>,


    /// 
    /// A set of tags to assign to the delivery stream. A tag is a key-value pair that you can     define and assign to AWS resources. Tags are metadata. For example, you can     add friendly names and descriptions or other types of information that can help you     distinguish the delivery stream. For more information about tags, see Using       Cost Allocation Tags in the AWS Billing and Cost Management User     Guide.
    /// 
    /// You can specify up to 50 tags when creating a delivery stream.
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

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DeliveryStreamDeliveryStreamTypeEnum {

    /// DirectPut
    #[serde(rename = "DirectPut")]
    Directput,

    /// KinesisStreamAsSource
    #[serde(rename = "KinesisStreamAsSource")]
    Kinesisstreamassource,

}

impl Default for DeliveryStreamDeliveryStreamTypeEnum {
    fn default() -> Self {
        DeliveryStreamDeliveryStreamTypeEnum::Directput
    }
}


impl cfn_resources::CfnResource for CfnDeliveryStream {
    fn type_string() -> &'static str {
        "AWS::KinesisFirehose::DeliveryStream"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AmazonOpenSearchServerlessBufferingHints property type specifies Property description not available. for an AWS::KinesisFirehose::DeliveryStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonOpenSearchServerlessBufferingHints {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<i64>,

}




/// The AmazonOpenSearchServerlessDestinationConfiguration property type specifies Property description not available. for an AWS::KinesisFirehose::DeliveryStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonOpenSearchServerlessDestinationConfiguration {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AmazonOpenSearchServerlessBufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<AmazonOpenSearchServerlessBufferingHints>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectionEndpoint")]
    pub collection_endpoint: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: AmazonOpenSearchServerlessRetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<AmazonOpenSearchServerlessRetryOptions>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,

}




/// The AmazonOpenSearchServerlessRetryOptions property type specifies Property description not available. for an AWS::KinesisFirehose::DeliveryStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonOpenSearchServerlessRetryOptions {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

}




/// Describes the buffering to perform before delivering data to the Amazon OpenSearch     Service destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonopensearchserviceBufferingHints {


    /// 
    /// Buffer incoming data for the specified period of time, in seconds, before delivering it     to the destination. The default value is 300 (5 minutes).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 60
    ///
    /// Maximum: 900
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<i64>,


    /// 
    /// Buffer incoming data to the specified size, in MBs, before delivering it to the     destination. The default value is 5. We recommend setting this parameter to a value greater     than the amount of data you typically ingest into the delivery stream in 10 seconds. For     example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or     higher.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<i64>,

}




/// Describes the configuration of a destination in Amazon OpenSearch Service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonopensearchserviceDestinationConfiguration {


    /// 
    /// The buffering options. If no value is specified, the default values for     AmazonopensearchserviceBufferingHints are used.
    /// 
    /// Required: No
    ///
    /// Type: AmazonopensearchserviceBufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<AmazonopensearchserviceBufferingHints>,


    /// 
    /// Describes the Amazon CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The endpoint to use when communicating with the cluster. Specify either this     ClusterEndpoint or the DomainARN field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: https:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterEndpoint")]
    pub cluster_endpoint: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DocumentIdOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentIdOptions")]
    pub document_id_options: Option<DocumentIdOptions>,


    /// 
    /// The ARN of the Amazon OpenSearch Service domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainARN")]
    pub domain_arn: Option<String>,


    /// 
    /// The Amazon OpenSearch Service index name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 80
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// The Amazon OpenSearch Service index rotation period. Index rotation appends a timestamp     to the IndexName to facilitate the expiration of old data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NoRotation | OneDay | OneHour | OneMonth | OneWeek
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexRotationPeriod")]
    pub index_rotation_period: Option<AmazonopensearchserviceDestinationConfigurationIndexRotationPeriodEnum>,


    /// 
    /// Describes a data processing configuration.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The retry behavior in case Kinesis Data Firehose is unable to deliver documents to     Amazon OpenSearch Service. The default value is 300 (5 minutes).
    /// 
    /// Required: No
    ///
    /// Type: AmazonopensearchserviceRetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<AmazonopensearchserviceRetryOptions>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose     for calling the Amazon OpenSearch Service Configuration API and for indexing     documents.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// Defines how documents should be delivered to Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AllDocuments | FailedDocumentsOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<AmazonopensearchserviceDestinationConfigurationS3BackupModeEnum>,


    /// 
    /// Describes the configuration of a destination in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,


    /// 
    /// The Amazon OpenSearch Service type name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,


    /// 
    /// The details of the VPC of the Amazon OpenSearch Service destination.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AmazonopensearchserviceDestinationConfigurationIndexRotationPeriodEnum {

    /// NoRotation
    #[serde(rename = "NoRotation")]
    Norotation,

    /// OneDay
    #[serde(rename = "OneDay")]
    Oneday,

    /// OneHour
    #[serde(rename = "OneHour")]
    Onehour,

    /// OneMonth
    #[serde(rename = "OneMonth")]
    Onemonth,

    /// OneWeek
    #[serde(rename = "OneWeek")]
    Oneweek,

}

impl Default for AmazonopensearchserviceDestinationConfigurationIndexRotationPeriodEnum {
    fn default() -> Self {
        AmazonopensearchserviceDestinationConfigurationIndexRotationPeriodEnum::Norotation
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AmazonopensearchserviceDestinationConfigurationS3BackupModeEnum {

    /// AllDocuments
    #[serde(rename = "AllDocuments")]
    Alldocuments,

    /// FailedDocumentsOnly
    #[serde(rename = "FailedDocumentsOnly")]
    Faileddocumentsonly,

}

impl Default for AmazonopensearchserviceDestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        AmazonopensearchserviceDestinationConfigurationS3BackupModeEnum::Alldocuments
    }
}



/// Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents     to Amazon OpenSearch Service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmazonopensearchserviceRetryOptions {


    /// 
    /// After an initial failure to deliver to Amazon OpenSearch Service, the total amount of     time during which Kinesis Data Firehose retries delivery (including the first attempt).     After this time has elapsed, the failed documents are written to Amazon S3. Default value     is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 7200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

}




/// The BufferingHints property type specifies how Amazon Kinesis Data     Firehose (Kinesis Data Firehose) buffers incoming data before delivering it to the     destination. The first buffer condition that is satisfied triggers Kinesis Data Firehose to     deliver the data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BufferingHints {


    /// 
    /// The length of time, in seconds, that Kinesis Data Firehose buffers incoming data     before delivering it to the destination. For valid values, see the       IntervalInSeconds content for the BufferingHints data     type in the Amazon Kinesis Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 60
    ///
    /// Maximum: 900
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<i64>,


    /// 
    /// The size of the buffer, in MBs, that Kinesis Data Firehose uses for incoming data     before delivering it to the destination. For valid values, see the SizeInMBs     content for the BufferingHints data     type in the Amazon Kinesis Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<i64>,

}




/// The CloudWatchLoggingOptions property type specifies Amazon CloudWatch     Logs (CloudWatch Logs) logging options that Amazon Kinesis Data Firehose (Kinesis Data     Firehose) uses for the delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudWatchLoggingOptions {


    /// 
    /// Indicates whether CloudWatch Logs logging is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The name of the CloudWatch Logs log group that contains the log stream that Kinesis     Data Firehose will use.
    /// 
    /// Conditional. If you enable logging, you must specify this property.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,


    /// 
    /// The name of the CloudWatch Logs log stream that Kinesis Data Firehose uses to send     logs about data delivery.
    /// 
    /// Conditional. If you enable logging, you must specify this property.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,

}




/// The CopyCommand property type configures the Amazon Redshift       COPY command that Amazon Kinesis Data Firehose (Kinesis Data Firehose) uses     to load data into an Amazon Redshift cluster from an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CopyCommand {


    /// 
    /// Parameters to use with the Amazon Redshift COPY command. For examples, see     the CopyOptions content for the CopyCommand data type in     the Amazon Kinesis Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 204800
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyOptions")]
    pub copy_options: Option<String>,


    /// 
    /// A comma-separated list of column names.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 204800
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTableColumns")]
    pub data_table_columns: Option<String>,


    /// 
    /// The name of the target table. The table must already exist in the database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTableName")]
    pub data_table_name: String,

}




/// Specifies that you want Kinesis Data Firehose to convert data from the JSON format to     the Parquet or ORC format before writing it to Amazon S3. Kinesis Data Firehose uses the     serializer and deserializer that you specify, in addition to the column information from     the AWS Glue table, to deserialize your input data from JSON and then     serialize it to the Parquet or ORC format. For more information, see Kinesis       Data Firehose Record Format Conversion.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataFormatConversionConfiguration {


    /// 
    /// Defaults to true. Set it to false if you want to disable     format conversion while preserving the configuration details.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Specifies the deserializer that you want Kinesis Data Firehose to use to convert the     format of your data from JSON. This parameter is required if Enabled is set to     true.
    /// 
    /// Required: No
    ///
    /// Type: InputFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputFormatConfiguration")]
    pub input_format_configuration: Option<InputFormatConfiguration>,


    /// 
    /// Specifies the serializer that you want Kinesis Data Firehose to use to convert the     format of your data to the Parquet or ORC format. This parameter is required if       Enabled is set to true.
    /// 
    /// Required: No
    ///
    /// Type: OutputFormatConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputFormatConfiguration")]
    pub output_format_configuration: Option<OutputFormatConfiguration>,


    /// 
    /// Specifies the AWS Glue Data Catalog table that contains the column     information. This parameter is required if Enabled is set to true.
    /// 
    /// Required: No
    ///
    /// Type: SchemaConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaConfiguration")]
    pub schema_configuration: Option<SchemaConfiguration>,

}




/// Specifies the type and Amazon Resource Name (ARN) of the CMK to use for Server-Side     Encryption (SSE).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeliveryStreamEncryptionConfigurationInput {


    /// 
    /// If you set KeyType to CUSTOMER_MANAGED_CMK, you must specify     the Amazon Resource Name (ARN) of the CMK. If you set KeyType to         AWS_OWNED_CMK, Kinesis Data Firehose uses a service-account CMK.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyARN")]
    pub key_arn: Option<String>,


    /// 
    /// Indicates the type of customer master key (CMK) to use for encryption. The default     setting is AWS_OWNED_CMK. For more information about CMKs, see Customer       Master Keys (CMKs).
    /// 
    /// You can use a CMK of type CUSTOMER_MANAGED_CMK to encrypt up to 500 delivery     streams.
    /// 
    /// ImportantTo encrypt your delivery stream, use symmetric CMKs. Kinesis Data Firehose doesn't       support asymmetric CMKs. For information about symmetric and asymmetric CMKs, see About        Symmetric and Asymmetric CMKs in the AWS Key Management       Service developer guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_OWNED_CMK | CUSTOMER_MANAGED_CMK
    ///
    /// Update requires: No interruption
    #[serde(rename = "KeyType")]
    pub key_type: DeliveryStreamEncryptionConfigurationInputKeyTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DeliveryStreamEncryptionConfigurationInputKeyTypeEnum {

    /// AWS_OWNED_CMK
    #[serde(rename = "AWS_OWNED_CMK")]
    Awsownedcmk,

    /// CUSTOMER_MANAGED_CMK
    #[serde(rename = "CUSTOMER_MANAGED_CMK")]
    Customermanagedcmk,

}

impl Default for DeliveryStreamEncryptionConfigurationInputKeyTypeEnum {
    fn default() -> Self {
        DeliveryStreamEncryptionConfigurationInputKeyTypeEnum::Awsownedcmk
    }
}



/// The deserializer you want Kinesis Data Firehose to use for converting the input data     from JSON. Kinesis Data Firehose then serializes the data to its final format using the       Serializer. Kinesis Data Firehose supports two types of deserializers: the       Apache Hive JSON SerDe and the OpenX JSON SerDe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Deserializer {


    /// 
    /// The native Hive / HCatalog JsonSerDe. Used by Kinesis Data Firehose for deserializing     data, which means converting it from the JSON format in preparation for serializing it to     the Parquet or ORC format. This is one of two deserializers you can choose, depending on     which one offers the functionality you need. The other option is the OpenX SerDe.
    /// 
    /// Required: No
    ///
    /// Type: HiveJsonSerDe
    ///
    /// Update requires: No interruption
    #[serde(rename = "HiveJsonSerDe")]
    pub hive_json_ser_de: Option<HiveJsonSerDe>,


    /// 
    /// The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means     converting it from the JSON format in preparation for serializing it to the Parquet or ORC     format. This is one of two deserializers you can choose, depending on which one offers the     functionality you need. The other option is the native Hive / HCatalog JsonSerDe.
    /// 
    /// Required: No
    ///
    /// Type: OpenXJsonSerDe
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenXJsonSerDe")]
    pub open_xjson_ser_de: Option<OpenXJsonSerDe>,

}




/// The DocumentIdOptions property type specifies Property description not available. for an AWS::KinesisFirehose::DeliveryStream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentIdOptions {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultDocumentIdFormat")]
    pub default_document_id_format: String,

}




/// The DynamicPartitioningConfiguration property type specifies the     configuration of the dynamic partitioning mechanism that creates targeted data sets from     the streaming data by partitioning it based on partition keys.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamicPartitioningConfiguration {


    /// 
    /// Specifies whether dynamic partitioning is enabled for this Kinesis Data Firehose     delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Specifies the retry behavior in case Kinesis Data Firehose is unable to deliver data     to an Amazon S3 prefix.
    /// 
    /// Required: No
    ///
    /// Type: RetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RetryOptions>,

}




/// The ElasticsearchBufferingHints property type specifies how Amazon     Kinesis Data Firehose (Kinesis Data Firehose) buffers incoming data while delivering it to     the destination. The first buffer condition that is satisfied triggers Kinesis Data     Firehose to deliver the data.
///
/// ElasticsearchBufferingHints is the property type for the BufferingHints     property of the Amazon Kinesis Data Firehose DeliveryStream       ElasticsearchDestinationConfiguration property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchBufferingHints {


    /// 
    /// The length of time, in seconds, that Kinesis Data Firehose buffers incoming data     before delivering it to the destination. For valid values, see the       IntervalInSeconds content for the BufferingHints data     type in the Amazon Kinesis Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 60
    ///
    /// Maximum: 900
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<i64>,


    /// 
    /// The size of the buffer, in MBs, that Kinesis Data Firehose uses for incoming data     before delivering it to the destination. For valid values, see the SizeInMBs     content for the BufferingHints data     type in the Amazon Kinesis Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<i64>,

}




/// The ElasticsearchDestinationConfiguration property type specifies an Amazon     Elasticsearch Service (Amazon ES) domain that Amazon Kinesis Data Firehose (Kinesis Data     Firehose) delivers data to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchDestinationConfiguration {


    /// 
    /// Configures how Kinesis Data Firehose buffers incoming data while delivering it to the     Amazon ES domain.
    /// 
    /// Required: No
    ///
    /// Type: ElasticsearchBufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,


    /// 
    /// The Amazon CloudWatch Logs logging options for the delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The endpoint to use when communicating with the cluster. Specify either this       ClusterEndpoint or the DomainARN field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterEndpoint")]
    pub cluster_endpoint: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DocumentIdOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentIdOptions")]
    pub document_id_options: Option<DocumentIdOptions>,


    /// 
    /// The ARN of the Amazon ES domain. The IAM role must have permissions for       DescribeElasticsearchDomain, DescribeElasticsearchDomains, and       DescribeElasticsearchDomainConfig after assuming the role specified in       RoleARN.
    /// 
    /// Specify either ClusterEndpoint or DomainARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainARN")]
    pub domain_arn: Option<String>,


    /// 
    /// The name of the Elasticsearch index to which Kinesis Data Firehose adds data for     indexing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 80
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexName")]
    pub index_name: String,


    /// 
    /// The frequency of Elasticsearch index rotation. If you enable index rotation, Kinesis     Data Firehose appends a portion of the UTC arrival timestamp to the specified index name,     and rotates the appended timestamp accordingly. For more information, see Index Rotation for the Amazon ES Destination in the Amazon Kinesis       Data Firehose Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NoRotation | OneDay | OneHour | OneMonth | OneWeek
    ///
    /// Update requires: No interruption
    #[serde(rename = "IndexRotationPeriod")]
    pub index_rotation_period: Option<ElasticsearchDestinationConfigurationIndexRotationPeriodEnum>,


    /// 
    /// The data processing configuration for the Kinesis Data Firehose delivery     stream.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The retry behavior when Kinesis Data Firehose is unable to deliver data to Amazon     ES.
    /// 
    /// Required: No
    ///
    /// Type: ElasticsearchRetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<ElasticsearchRetryOptions>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose     for calling the Amazon ES Configuration API and for indexing documents. For more     information, see Controlling Access with Amazon Kinesis       Data Firehose.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// The condition under which Kinesis Data Firehose delivers data to Amazon Simple Storage     Service (Amazon S3). You can send Amazon S3 all documents (all data) or only the documents     that Kinesis Data Firehose could not deliver to the Amazon ES destination. For more     information and valid values, see the S3BackupMode content for the ElasticsearchDestinationConfiguration data type in the Amazon Kinesis       Data Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AllDocuments | FailedDocumentsOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<ElasticsearchDestinationConfigurationS3BackupModeEnum>,


    /// 
    /// The S3 bucket where Kinesis Data Firehose backs up incoming data.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,


    /// 
    /// The Elasticsearch type name that Amazon ES adds to documents when indexing     data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,


    /// 
    /// The details of the VPC of the Amazon ES destination.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ElasticsearchDestinationConfigurationIndexRotationPeriodEnum {

    /// NoRotation
    #[serde(rename = "NoRotation")]
    Norotation,

    /// OneDay
    #[serde(rename = "OneDay")]
    Oneday,

    /// OneHour
    #[serde(rename = "OneHour")]
    Onehour,

    /// OneMonth
    #[serde(rename = "OneMonth")]
    Onemonth,

    /// OneWeek
    #[serde(rename = "OneWeek")]
    Oneweek,

}

impl Default for ElasticsearchDestinationConfigurationIndexRotationPeriodEnum {
    fn default() -> Self {
        ElasticsearchDestinationConfigurationIndexRotationPeriodEnum::Norotation
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ElasticsearchDestinationConfigurationS3BackupModeEnum {

    /// AllDocuments
    #[serde(rename = "AllDocuments")]
    Alldocuments,

    /// FailedDocumentsOnly
    #[serde(rename = "FailedDocumentsOnly")]
    Faileddocumentsonly,

}

impl Default for ElasticsearchDestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        ElasticsearchDestinationConfigurationS3BackupModeEnum::Alldocuments
    }
}



/// The ElasticsearchRetryOptions property type configures the retry behavior     for when Amazon Kinesis Data Firehose (Kinesis Data Firehose) can't deliver data to Amazon     Elasticsearch Service (Amazon ES).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchRetryOptions {


    /// 
    /// After an initial failure to deliver to Amazon ES, the total amount of time during which     Kinesis Data Firehose re-attempts delivery (including the first attempt). If Kinesis Data     Firehose can't deliver the data within the specified time, it writes the data to the backup     S3 bucket. For valid values, see the DurationInSeconds content for the ElasticsearchRetryOptions data type in the Amazon Kinesis Data       Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 7200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

}




/// The EncryptionConfiguration property type specifies the encryption     settings that Amazon Kinesis Data Firehose (Kinesis Data Firehose) uses when delivering     data to Amazon Simple Storage Service (Amazon S3).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfiguration {


    /// 
    /// The AWS Key Management Service (AWS KMS) encryption     key that Amazon S3 uses to encrypt your data.
    /// 
    /// Required: No
    ///
    /// Type: KMSEncryptionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSEncryptionConfig")]
    pub kmsencryption_config: Option<KMSEncryptionConfig>,


    /// 
    /// Disables encryption. For valid values, see the NoEncryptionConfig     content for the EncryptionConfiguration data type in the Amazon Kinesis Data Firehose       API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NoEncryption
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoEncryptionConfig")]
    pub no_encryption_config: Option<EncryptionConfigurationNoEncryptionConfigEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EncryptionConfigurationNoEncryptionConfigEnum {

    /// NoEncryption
    #[serde(rename = "NoEncryption")]
    Noencryption,

}

impl Default for EncryptionConfigurationNoEncryptionConfigEnum {
    fn default() -> Self {
        EncryptionConfigurationNoEncryptionConfigEnum::Noencryption
    }
}



/// The ExtendedS3DestinationConfiguration property type configures an     Amazon S3 destination for an Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExtendedS3DestinationConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon S3 bucket. For constraints, see ExtendedS3DestinationConfiguration in the Amazon Kinesis Data       Firehose API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,


    /// 
    /// The buffering option.
    /// 
    /// Required: No
    ///
    /// Type: BufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,


    /// 
    /// The Amazon CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The compression format. If no value is specified, the default is       UNCOMPRESSED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GZIP | HADOOP_SNAPPY | Snappy | UNCOMPRESSED | ZIP
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<ExtendedS3DestinationConfigurationCompressionFormatEnum>,


    /// 
    /// The serializer, deserializer, and schema for converting data from the JSON format to     the Parquet or ORC format before writing it to Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: DataFormatConversionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataFormatConversionConfiguration")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,


    /// 
    /// The configuration of the dynamic partitioning mechanism that creates targeted data     sets from the streaming data by partitioning it based on partition keys.
    /// 
    /// Required: No
    ///
    /// Type: DynamicPartitioningConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamicPartitioningConfiguration")]
    pub dynamic_partitioning_configuration: Option<DynamicPartitioningConfiguration>,


    /// 
    /// The encryption configuration for the Kinesis Data Firehose delivery stream. The default     value is NoEncryption.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,


    /// 
    /// A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing     them to S3. This prefix appears immediately following the bucket name. For information     about how to specify this prefix, see Custom Prefixes for Amazon S3     Objects.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorOutputPrefix")]
    pub error_output_prefix: Option<String>,


    /// 
    /// The YYYY/MM/DD/HH time format prefix is automatically used for delivered     Amazon S3 files. For more information, see ExtendedS3DestinationConfiguration in the Amazon Kinesis Data       Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The data processing configuration for the Kinesis Data Firehose delivery     stream.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS credentials. For constraints,     see ExtendedS3DestinationConfiguration in the Amazon Kinesis Data       Firehose API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// The configuration for backup in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupConfiguration")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,


    /// 
    /// The Amazon S3 backup mode. After you create a delivery stream, you can update it to     enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the     delivery stream to disable it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<ExtendedS3DestinationConfigurationS3BackupModeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ExtendedS3DestinationConfigurationCompressionFormatEnum {

    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// HADOOP_SNAPPY
    #[serde(rename = "HADOOP_SNAPPY")]
    Hadoopsnappy,

    /// Snappy
    #[serde(rename = "Snappy")]
    Snappy,

    /// UNCOMPRESSED
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed,

    /// ZIP
    #[serde(rename = "ZIP")]
    Zip,

}

impl Default for ExtendedS3DestinationConfigurationCompressionFormatEnum {
    fn default() -> Self {
        ExtendedS3DestinationConfigurationCompressionFormatEnum::Gzip
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ExtendedS3DestinationConfigurationS3BackupModeEnum {

    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

}

impl Default for ExtendedS3DestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        ExtendedS3DestinationConfigurationS3BackupModeEnum::Disabled
    }
}



/// The native Hive / HCatalog JsonSerDe. Used by Kinesis Data Firehose for deserializing     data, which means converting it from the JSON format in preparation for serializing it to     the Parquet or ORC format. This is one of two deserializers you can choose, depending on     which one offers the functionality you need. The other option is the OpenX SerDe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HiveJsonSerDe {


    /// 
    /// Indicates how you want Kinesis Data Firehose to parse the date and timestamps that     may be present in your input data JSON. To specify these format strings, follow the pattern     syntax of JodaTime's DateTimeFormat format strings. For more information, see Class DateTimeFormat. You can also use the special value millis to     parse timestamps in epoch milliseconds. If you don't specify a format, Kinesis Data     Firehose uses java.sql.Timestamp::valueOf by default.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimestampFormats")]
    pub timestamp_formats: Option<Vec<String>>,

}




/// Describes the metadata that's delivered to the specified HTTP endpoint destination.     Kinesis Firehose supports any custom HTTP endpoint or HTTP endpoints owned by supported     third-party service providers, including Datadog, MongoDB, and New Relic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpEndpointCommonAttribute {


    /// 
    /// The name of the HTTP endpoint common attribute.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(?!\s*$).+
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,


    /// 
    /// The value of the HTTP endpoint common attribute.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttributeValue")]
    pub attribute_value: String,

}




/// Describes the configuration of the HTTP endpoint to which Kinesis Firehose delivers     data. Kinesis Firehose supports any custom HTTP endpoint or HTTP endpoints owned by     supported third-party service providers, including Datadog, MongoDB, and New Relic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpEndpointConfiguration {


    /// 
    /// The access key required for Kinesis Firehose to authenticate with the HTTP endpoint     selected as the destination.
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
    #[serde(rename = "AccessKey")]
    pub access_key: Option<String>,


    /// 
    /// The name of the HTTP endpoint selected as the destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^(?!\s*$).+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The URL of the HTTP endpoint selected as the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Pattern: https://.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: String,

}




/// Describes the configuration of the HTTP endpoint destination. Kinesis Firehose     supports any custom HTTP endpoint or HTTP endpoints owned by supported third-party service     providers, including Datadog, MongoDB, and New Relic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpEndpointDestinationConfiguration {


    /// 
    /// The buffering options that can be used before data is delivered to the specified     destination. Kinesis Data Firehose treats these options as hints, and it might choose to     use more optimal values. The SizeInMBs and IntervalInSeconds parameters are optional.     However, if you specify a value for one of them, you must also provide a value for the     other.
    /// 
    /// Required: No
    ///
    /// Type: BufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,


    /// 
    /// Describes the Amazon CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The configuration of the HTTP endpoint selected as the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpEndpointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: HttpEndpointConfiguration,


    /// 
    /// Describes the data processing configuration.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The configuration of the request sent to the HTTP endpoint specified as the     destination.
    /// 
    /// Required: No
    ///
    /// Type: HttpEndpointRequestConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestConfiguration")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,


    /// 
    /// Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data     to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment     of receipt from the specified HTTP endpoint destination.
    /// 
    /// Required: No
    ///
    /// Type: RetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RetryOptions>,


    /// 
    /// Kinesis Data Firehose uses this IAM role for all the permissions that the delivery     stream needs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,


    /// 
    /// Describes the S3 bucket backup options for the data that Kinesis Data Firehose     delivers to the HTTP endpoint destination. You can back up all documents (AllData) or only     the documents that Kinesis Data Firehose could not deliver to the specified HTTP endpoint     destination (FailedDataOnly).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AllData | FailedDataOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<HttpEndpointDestinationConfigurationS3BackupModeEnum>,


    /// 
    /// Describes the configuration of a destination in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HttpEndpointDestinationConfigurationS3BackupModeEnum {

    /// AllData
    #[serde(rename = "AllData")]
    Alldata,

    /// FailedDataOnly
    #[serde(rename = "FailedDataOnly")]
    Faileddataonly,

}

impl Default for HttpEndpointDestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        HttpEndpointDestinationConfigurationS3BackupModeEnum::Alldata
    }
}



/// The configuration of the HTTP endpoint request. Kinesis Firehose supports any custom     HTTP endpoint or HTTP endpoints owned by supported third-party service providers, including     Datadog, MongoDB, and New Relic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpEndpointRequestConfiguration {


    /// 
    /// Describes the metadata sent to the HTTP endpoint destination.
    /// 
    /// Required: No
    ///
    /// Type: List of HttpEndpointCommonAttribute
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "CommonAttributes")]
    pub common_attributes: Option<Vec<HttpEndpointCommonAttribute>>,


    /// 
    /// Kinesis Data Firehose uses the content encoding to compress the body of a request     before sending the request to the destination. For more information, see Content-Encoding     in MDN Web Docs, the official Mozilla documentation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GZIP | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<HttpEndpointRequestConfigurationContentEncodingEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HttpEndpointRequestConfigurationContentEncodingEnum {

    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// NONE
    #[serde(rename = "NONE")]
    None,

}

impl Default for HttpEndpointRequestConfigurationContentEncodingEnum {
    fn default() -> Self {
        HttpEndpointRequestConfigurationContentEncodingEnum::Gzip
    }
}



/// Specifies the deserializer you want to use to convert the format of the input data.     This parameter is required if Enabled is set to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputFormatConfiguration {


    /// 
    /// Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe     or the OpenX JSON SerDe. If both are non-null, the server rejects the request.
    /// 
    /// Required: No
    ///
    /// Type: Deserializer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Deserializer")]
    pub deserializer: Option<Deserializer>,

}




/// The KMSEncryptionConfig property type specifies the AWS     Key Management Service (AWS KMS) encryption key that Amazon Simple Storage     Service (Amazon S3) uses to encrypt data delivered by the Amazon Kinesis Data Firehose     (Kinesis Data Firehose) stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KMSEncryptionConfig {


    /// 
    /// The Amazon Resource Name (ARN) of the AWS KMS encryption key that     Amazon S3 uses to encrypt data delivered by the Kinesis Data Firehose stream. The key must     belong to the same region as the destination S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AWSKMSKeyARN")]
    pub awskmskey_arn: String,

}




/// The KinesisStreamSourceConfiguration property type specifies the stream and     role Amazon Resource Names (ARNs) for a Kinesis stream used as the source for a delivery     stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamSourceConfiguration {


    /// 
    /// The ARN of the source Kinesis data stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KinesisStreamARN")]
    pub kinesis_stream_arn: String,


    /// 
    /// The ARN of the role that provides access to the source Kinesis data stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}




/// The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means     converting it from the JSON format in preparation for serializing it to the Parquet or ORC     format. This is one of two deserializers you can choose, depending on which one offers the     functionality you need. The other option is the native Hive / HCatalog JsonSerDe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenXJsonSerDe {


    /// 
    /// When set to true, which is the default, Kinesis Data Firehose converts     JSON keys to lowercase before deserializing them.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseInsensitive")]
    pub case_insensitive: Option<bool>,


    /// 
    /// Maps column names to JSON keys that aren't identical to the column names. This is     useful when the JSON contains keys that are Hive keywords. For example,       timestamp is a Hive keyword. If you have a JSON key named       timestamp, set this parameter to {"ts": "timestamp"} to map     this key to a column named ts.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnToJsonKeyMappings")]
    pub column_to_json_key_mappings: Option<std::collections::HashMap<String, String>>,


    /// 
    /// When set to true, specifies that the names of the keys include dots and     that you want Kinesis Data Firehose to replace them with underscores. This is useful     because Apache Hive does not allow dots in column names. For example, if the JSON contains     a key whose name is "a.b", you can define the column name to be "a_b" when using this     option.
    /// 
    /// The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConvertDotsInJsonKeysToUnderscores")]
    pub convert_dots_in_json_keys_to_underscores: Option<bool>,

}




/// A serializer to use for converting data to the ORC format before storing it in Amazon     S3. For more information, see Apache     ORC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OrcSerDe {


    /// 
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to     copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the     minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 67108864
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockSizeBytes")]
    pub block_size_bytes: Option<i64>,


    /// 
    /// The column names for which you want Kinesis Data Firehose to create bloom filters. The     default is null.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BloomFilterColumns")]
    pub bloom_filter_columns: Option<Vec<String>>,


    /// 
    /// The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the     Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "BloomFilterFalsePositiveProbability")]
    pub bloom_filter_false_positive_probability: Option<f64>,


    /// 
    /// The compression code to use over data blocks. The default is SNAPPY.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SNAPPY | ZLIB
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compression")]
    pub compression: Option<OrcSerDeCompressionEnum>,


    /// 
    /// Represents the fraction of the total number of non-null rows. To turn off dictionary     encoding, set this fraction to a number that is less than the number of distinct keys in a     dictionary. To always use dictionary encoding, set this threshold to 1.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DictionaryKeyThreshold")]
    pub dictionary_key_threshold: Option<f64>,


    /// 
    /// Set this to true to indicate that you want stripes to be padded to the HDFS     block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS     before querying. The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnablePadding")]
    pub enable_padding: Option<bool>,


    /// 
    /// The version of the file to write. The possible values are V0_11 and       V0_12. The default is V0_12.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: V0_11 | V0_12
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatVersion")]
    pub format_version: Option<OrcSerDeFormatVersionEnum>,


    /// 
    /// A number between 0 and 1 that defines the tolerance for block padding as a decimal     fraction of stripe size. The default value is 0.05, which means 5 percent of stripe     size.
    /// 
    /// For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block     padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB     block. In such a case, if the available size within the block is more than 3.2 MiB, a new,     smaller stripe is inserted to fit within that space. This ensures that no stripe crosses     block boundaries and causes remote reads within a node-local task.
    /// 
    /// Kinesis Data Firehose ignores this parameter when EnablePadding is       false.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PaddingTolerance")]
    pub padding_tolerance: Option<f64>,


    /// 
    /// The number of rows between index entries. The default is 10,000 and the minimum is     1,000.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowIndexStride")]
    pub row_index_stride: Option<i64>,


    /// 
    /// The number of bytes in each stripe. The default is 64 MiB and the minimum is 8     MiB.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 8388608
    ///
    /// Update requires: No interruption
    #[serde(rename = "StripeSizeBytes")]
    pub stripe_size_bytes: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum OrcSerDeCompressionEnum {

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SNAPPY
    #[serde(rename = "SNAPPY")]
    Snappy,

    /// ZLIB
    #[serde(rename = "ZLIB")]
    Zlib,

}

impl Default for OrcSerDeCompressionEnum {
    fn default() -> Self {
        OrcSerDeCompressionEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum OrcSerDeFormatVersionEnum {

    /// V0_11
    #[serde(rename = "V0_11")]
    V011,

    /// V0_12
    #[serde(rename = "V0_12")]
    V012,

}

impl Default for OrcSerDeFormatVersionEnum {
    fn default() -> Self {
        OrcSerDeFormatVersionEnum::V011
    }
}



/// Specifies the serializer that you want Kinesis Data Firehose to use to convert the     format of your data before it writes it to Amazon S3. This parameter is required if       Enabled is set to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutputFormatConfiguration {


    /// 
    /// Specifies which serializer to use. You can choose either the ORC SerDe or the Parquet     SerDe. If both are non-null, the server rejects the request.
    /// 
    /// Required: No
    ///
    /// Type: Serializer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Serializer")]
    pub serializer: Option<Serializer>,

}




/// A serializer to use for converting data to the Parquet format before storing it in     Amazon S3. For more information, see Apache Parquet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ParquetSerDe {


    /// 
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to     copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the     minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 67108864
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockSizeBytes")]
    pub block_size_bytes: Option<i64>,


    /// 
    /// The compression code to use over data blocks. The possible values are       UNCOMPRESSED, SNAPPY, and GZIP, with the default     being SNAPPY. Use SNAPPY for higher decompression speed. Use       GZIP if the compression ratio is more important than speed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GZIP | SNAPPY | UNCOMPRESSED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compression")]
    pub compression: Option<ParquetSerDeCompressionEnum>,


    /// 
    /// Indicates whether to enable dictionary compression.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDictionaryCompression")]
    pub enable_dictionary_compression: Option<bool>,


    /// 
    /// The maximum amount of padding to apply. This is useful if you intend to copy the data     from Amazon S3 to HDFS before querying. The default is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxPaddingBytes")]
    pub max_padding_bytes: Option<i64>,


    /// 
    /// The Parquet page size. Column chunks are divided into pages. A page is conceptually an     indivisible unit (in terms of compression and encoding). The minimum value is 64 KiB and     the default is 1 MiB.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 65536
    ///
    /// Update requires: No interruption
    #[serde(rename = "PageSizeBytes")]
    pub page_size_bytes: Option<i64>,


    /// 
    /// Indicates the version of row format to output. The possible values are V1     and V2. The default is V1.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: V1 | V2
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriterVersion")]
    pub writer_version: Option<ParquetSerDeWriterVersionEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ParquetSerDeCompressionEnum {

    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// SNAPPY
    #[serde(rename = "SNAPPY")]
    Snappy,

    /// UNCOMPRESSED
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed,

}

impl Default for ParquetSerDeCompressionEnum {
    fn default() -> Self {
        ParquetSerDeCompressionEnum::Gzip
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ParquetSerDeWriterVersionEnum {

    /// V1
    #[serde(rename = "V1")]
    V1,

    /// V2
    #[serde(rename = "V2")]
    V2,

}

impl Default for ParquetSerDeWriterVersionEnum {
    fn default() -> Self {
        ParquetSerDeWriterVersionEnum::V1
    }
}



/// The ProcessingConfiguration property configures data processing for an     Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProcessingConfiguration {


    /// 
    /// Indicates whether data processing is enabled (true) or disabled (false).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The data processors.
    /// 
    /// Required: No
    ///
    /// Type: List of Processor
    ///
    /// Update requires: No interruption
    #[serde(rename = "Processors")]
    pub processors: Option<Vec<Processor>>,

}




/// The Processor property specifies a data processor for an Amazon Kinesis     Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Processor {


    /// 
    /// The processor parameters.
    /// 
    /// Required: No
    ///
    /// Type: List of ProcessorParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ProcessorParameter>>,


    /// 
    /// The type of processor. Valid values: Lambda.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AppendDelimiterToRecord | Lambda | MetadataExtraction | RecordDeAggregation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ProcessorTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ProcessorTypeEnum {

    /// AppendDelimiterToRecord
    #[serde(rename = "AppendDelimiterToRecord")]
    Appenddelimitertorecord,

    /// Lambda
    #[serde(rename = "Lambda")]
    Lambda,

    /// MetadataExtraction
    #[serde(rename = "MetadataExtraction")]
    Metadataextraction,

    /// RecordDeAggregation
    #[serde(rename = "RecordDeAggregation")]
    Recorddeaggregation,

}

impl Default for ProcessorTypeEnum {
    fn default() -> Self {
        ProcessorTypeEnum::Appenddelimitertorecord
    }
}



/// The ProcessorParameter property specifies a processor parameter in a data     processor for an Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProcessorParameter {


    /// 
    /// The name of the parameter. Currently the following default values are supported: 3     for NumberOfRetries and 60 for the BufferIntervalInSeconds. The       BufferSizeInMBs ranges between 0.2 MB and up to 3MB. The default buffering     hint is 1MB for all destinations, except Splunk. For Splunk, the default buffering hint is     256 KB.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BufferIntervalInSeconds | BufferSizeInMBs | Delimiter | JsonParsingEngine | LambdaArn | MetadataExtractionQuery | NumberOfRetries | RoleArn | SubRecordType
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterName")]
    pub parameter_name: ProcessorParameterParameterNameEnum,


    /// 
    /// The parameter value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5120
    ///
    /// Pattern: ^(?!\s*$).+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ProcessorParameterParameterNameEnum {

    /// BufferIntervalInSeconds
    #[serde(rename = "BufferIntervalInSeconds")]
    Bufferintervalinseconds,

    /// BufferSizeInMBs
    #[serde(rename = "BufferSizeInMBs")]
    Buffersizeinmbs,

    /// Delimiter
    #[serde(rename = "Delimiter")]
    Delimiter,

    /// JsonParsingEngine
    #[serde(rename = "JsonParsingEngine")]
    Jsonparsingengine,

    /// LambdaArn
    #[serde(rename = "LambdaArn")]
    Lambdaarn,

    /// MetadataExtractionQuery
    #[serde(rename = "MetadataExtractionQuery")]
    Metadataextractionquery,

    /// NumberOfRetries
    #[serde(rename = "NumberOfRetries")]
    Numberofretries,

    /// RoleArn
    #[serde(rename = "RoleArn")]
    Rolearn,

    /// SubRecordType
    #[serde(rename = "SubRecordType")]
    Subrecordtype,

}

impl Default for ProcessorParameterParameterNameEnum {
    fn default() -> Self {
        ProcessorParameterParameterNameEnum::Bufferintervalinseconds
    }
}



/// The RedshiftDestinationConfiguration property type specifies an Amazon     Redshift cluster to which Amazon Kinesis Data Firehose (Kinesis Data Firehose) delivers     data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftDestinationConfiguration {


    /// 
    /// The CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The connection string that Kinesis Data Firehose uses to connect to the Amazon Redshift     cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: jdbc:(redshift|postgresql)://((?!-)[A-Za-z0-9-]{1,63}(?<!-)\.)+redshift\.([a-zA-Z0-9\.]+):\d{1,5}/[a-zA-Z0-9_$-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClusterJDBCURL")]
    pub cluster_jdbcurl: String,


    /// 
    /// Configures the Amazon Redshift COPY command that Kinesis Data Firehose uses     to load data into the cluster from the Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: CopyCommand
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyCommand")]
    pub copy_command: CopyCommand,


    /// 
    /// The password for the Amazon Redshift user that you specified in the       Username property.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 6
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Password")]
    pub password: String,


    /// 
    /// The data processing configuration for the Kinesis Data Firehose delivery     stream.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The retry behavior in case Kinesis Data Firehose is unable to deliver documents to     Amazon Redshift. Default value is 3600 (60 minutes).
    /// 
    /// Required: No
    ///
    /// Type: RedshiftRetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RedshiftRetryOptions>,


    /// 
    /// The ARN of the AWS Identity and Access Management (IAM) role that     grants Kinesis Data Firehose access to your Amazon S3 bucket and AWS KMS     (if you enable data encryption). For more information, see Grant Kinesis Data       Firehose Access to an Amazon Redshift Destination in the Amazon       Kinesis Data Firehose Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// The configuration for backup in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupConfiguration")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,


    /// 
    /// The Amazon S3 backup mode. After you create a delivery stream, you can update it to     enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the     delivery stream to disable it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<RedshiftDestinationConfigurationS3BackupModeEnum>,


    /// 
    /// The S3 bucket where Kinesis Data Firehose first delivers data. After the data is in the     bucket, Kinesis Data Firehose uses the COPY command to load the data into the     Amazon Redshift cluster. For the Amazon S3 bucket's compression format, don't specify       SNAPPY or ZIP because the Amazon Redshift COPY     command doesn't support them.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,


    /// 
    /// The Amazon Redshift user that has permission to access the Amazon Redshift cluster.     This user must have INSERT privileges for copying data from the Amazon S3     bucket to the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Username")]
    pub username: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RedshiftDestinationConfigurationS3BackupModeEnum {

    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,

}

impl Default for RedshiftDestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        RedshiftDestinationConfigurationS3BackupModeEnum::Disabled
    }
}



/// Configures retry behavior in case Kinesis Data Firehose is unable to deliver     documents to Amazon Redshift.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftRetryOptions {


    /// 
    /// The length of time during which Kinesis Data Firehose retries delivery after a     failure, starting from the initial request and including the first attempt. The default     value is 3600 seconds (60 minutes). Kinesis Data Firehose does not retry if the value of       DurationInSeconds is 0 (zero) or if the first delivery attempt takes longer     than the current value.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 7200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

}




/// Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data     to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment     of receipt from the specified HTTP endpoint destination. Kinesis Firehose supports any     custom HTTP endpoint or HTTP endpoints owned by supported third-party service providers,     including Datadog, MongoDB, and New Relic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetryOptions {


    /// 
    /// The total amount of time that Kinesis Data Firehose spends on retries. This duration     starts after the initial attempt to send data to the custom destination via HTTPS endpoint     fails. It doesn't include the periods during which Kinesis Data Firehose waits for     acknowledgment from the specified destination after each attempt.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 7200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

}




/// The S3DestinationConfiguration property type specifies an Amazon Simple     Storage Service (Amazon S3) destination to which Amazon Kinesis Data Firehose (Kinesis Data     Firehose) delivers data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3DestinationConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the Amazon S3 bucket to send data to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,


    /// 
    /// Configures how Kinesis Data Firehose buffers incoming data while delivering it to the     Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: BufferingHints
    ///
    /// Update requires: No interruption
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,


    /// 
    /// The CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The type of compression that Kinesis Data Firehose uses to compress the data that it     delivers to the Amazon S3 bucket. For valid values, see the CompressionFormat     content for the S3DestinationConfiguration data type in the Amazon Kinesis Data       Firehose API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GZIP | HADOOP_SNAPPY | Snappy | UNCOMPRESSED | ZIP
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<S3DestinationConfigurationCompressionFormatEnum>,


    /// 
    /// Configures Amazon Simple Storage Service (Amazon S3) server-side encryption. Kinesis     Data Firehose uses AWS Key Management Service (AWS KMS)     to encrypt the data that it delivers to your Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,


    /// 
    /// A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing     them to S3. This prefix appears immediately following the bucket name. For information     about how to specify this prefix, see Custom Prefixes for Amazon S3     Objects.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorOutputPrefix")]
    pub error_output_prefix: Option<String>,


    /// 
    /// A prefix that Kinesis Data Firehose adds to the files that it delivers to the Amazon S3     bucket. The prefix helps you identify the files that Kinesis Data Firehose     delivered.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The ARN of an AWS Identity and Access Management (IAM) role that grants     Kinesis Data Firehose access to your Amazon S3 bucket and AWS KMS (if you     enable data encryption). For more information, see Grant Kinesis Data       Firehose Access to an Amazon S3 Destination in the Amazon Kinesis Data       Firehose Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum S3DestinationConfigurationCompressionFormatEnum {

    /// GZIP
    #[serde(rename = "GZIP")]
    Gzip,

    /// HADOOP_SNAPPY
    #[serde(rename = "HADOOP_SNAPPY")]
    Hadoopsnappy,

    /// Snappy
    #[serde(rename = "Snappy")]
    Snappy,

    /// UNCOMPRESSED
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed,

    /// ZIP
    #[serde(rename = "ZIP")]
    Zip,

}

impl Default for S3DestinationConfigurationCompressionFormatEnum {
    fn default() -> Self {
        S3DestinationConfigurationCompressionFormatEnum::Gzip
    }
}



/// Specifies the schema to which you want Kinesis Data Firehose to configure your data     before it writes it to Amazon S3. This parameter is required if Enabled is set     to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SchemaConfiguration {


    /// 
    /// The ID of the AWS Glue Data Catalog. If you don't supply this, the       AWS account ID is used by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// Specifies the name of the AWS Glue database that contains the schema for     the output data.
    /// 
    /// ImportantIf the SchemaConfiguration request parameter is used as part of invoking       the CreateDeliveryStream API, then the DatabaseName property       is required and its value must be specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// If you don't specify an AWS Region, the default is the current     Region.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: Option<String>,


    /// 
    /// The role that Kinesis Data Firehose can use to access AWS Glue. This     role must be in the same account you use for Kinesis Data Firehose. Cross-account roles     aren't allowed.
    /// 
    /// ImportantIf the SchemaConfiguration request parameter is used as part of invoking       the CreateDeliveryStream API, then the RoleARN property is       required and its value must be specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,


    /// 
    /// Specifies the AWS Glue table that contains the column information that     constitutes your data schema.
    /// 
    /// ImportantIf the SchemaConfiguration request parameter is used as part of invoking       the CreateDeliveryStream API, then the TableName property is       required and its value must be specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,


    /// 
    /// Specifies the table version for the output data schema. If you don't specify this     version ID, or if you set it to LATEST, Kinesis Data Firehose uses the most     recent version. This means that any updates to the table are automatically picked     up.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\S+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,

}




/// The serializer that you want Kinesis Data Firehose to use to convert data to the target     format before writing it to Amazon S3. Kinesis Data Firehose supports two types of     serializers: the ORC SerDe and the Parquet SerDe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Serializer {


    /// 
    /// A serializer to use for converting data to the ORC format before storing it in Amazon     S3. For more information, see Apache     ORC.
    /// 
    /// Required: No
    ///
    /// Type: OrcSerDe
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrcSerDe")]
    pub orc_ser_de: Option<OrcSerDe>,


    /// 
    /// A serializer to use for converting data to the Parquet format before storing it in     Amazon S3. For more information, see Apache Parquet.
    /// 
    /// Required: No
    ///
    /// Type: ParquetSerDe
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParquetSerDe")]
    pub parquet_ser_de: Option<ParquetSerDe>,

}




/// The SplunkDestinationConfiguration property type specifies the     configuration of a destination in Splunk for a Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SplunkDestinationConfiguration {


    /// 
    /// The Amazon CloudWatch logging options for your delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: CloudWatchLoggingOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,


    /// 
    /// The amount of time that Kinesis Data Firehose waits to receive an acknowledgment from     Splunk after it sends it data. At the end of the timeout period, Kinesis Data Firehose     either tries to send the data again or considers it an error, based on your retry     settings.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 180
    ///
    /// Maximum: 600
    ///
    /// Update requires: No interruption
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    pub hecacknowledgment_timeout_in_seconds: Option<i64>,


    /// 
    /// The HTTP Event Collector (HEC) endpoint to which Kinesis Data Firehose sends your     data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "HECEndpoint")]
    pub hecendpoint: String,


    /// 
    /// This type can be either Raw or Event.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Event | Raw
    ///
    /// Update requires: No interruption
    #[serde(rename = "HECEndpointType")]
    pub hecendpoint_type: SplunkDestinationConfigurationHECEndpointTypeEnum,


    /// 
    /// This is a GUID that you obtain from your Splunk cluster when you create a new HEC     endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "HECToken")]
    pub hectoken: String,


    /// 
    /// The data processing configuration.
    /// 
    /// Required: No
    ///
    /// Type: ProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,


    /// 
    /// The retry behavior in case Kinesis Data Firehose is unable to deliver data to Splunk,     or if it doesn't receive an acknowledgment of receipt from Splunk.
    /// 
    /// Required: No
    ///
    /// Type: SplunkRetryOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<SplunkRetryOptions>,


    /// 
    /// Defines how documents should be delivered to Amazon S3. When set to       FailedEventsOnly, Kinesis Data Firehose writes any data that could not be     indexed to the configured Amazon S3 destination. When set to AllEvents,     Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed     documents to Amazon S3. The default value is FailedEventsOnly.
    /// 
    /// You can update this backup mode from FailedEventsOnly to       AllEvents. You can't update it from AllEvents to       FailedEventsOnly.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AllEvents | FailedEventsOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<SplunkDestinationConfigurationS3BackupModeEnum>,


    /// 
    /// The configuration for the backup Amazon S3 location.
    /// 
    /// Required: Yes
    ///
    /// Type: S3DestinationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SplunkDestinationConfigurationHECEndpointTypeEnum {

    /// Event
    #[serde(rename = "Event")]
    Event,

    /// Raw
    #[serde(rename = "Raw")]
    Raw,

}

impl Default for SplunkDestinationConfigurationHECEndpointTypeEnum {
    fn default() -> Self {
        SplunkDestinationConfigurationHECEndpointTypeEnum::Event
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SplunkDestinationConfigurationS3BackupModeEnum {

    /// AllEvents
    #[serde(rename = "AllEvents")]
    Allevents,

    /// FailedEventsOnly
    #[serde(rename = "FailedEventsOnly")]
    Failedeventsonly,

}

impl Default for SplunkDestinationConfigurationS3BackupModeEnum {
    fn default() -> Self {
        SplunkDestinationConfigurationS3BackupModeEnum::Allevents
    }
}



/// The SplunkRetryOptions property type specifies retry behavior in case     Kinesis Data Firehose is unable to deliver documents to Splunk or if it doesn't receive an     acknowledgment from Splunk.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SplunkRetryOptions {


    /// 
    /// The total amount of time that Kinesis Data Firehose spends on retries. This duration     starts after the initial attempt to send data to Splunk fails. It doesn't include the     periods during which Kinesis Data Firehose waits for acknowledgment from Splunk after each     attempt.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 7200
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<i64>,

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




/// The details of the VPC of the Amazon ES destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfiguration {


    /// 
    /// The ARN of the IAM role that you want the delivery stream to use to create endpoints     in the destination VPC. You can use your existing Kinesis Data Firehose delivery role or     you can specify a new role. In either case, make sure that the role trusts the Kinesis Data     Firehose service principal and that it grants the following permissions:
    /// 
    /// ec2:DescribeVpcs            ec2:DescribeVpcAttribute            ec2:DescribeSubnets            ec2:DescribeSecurityGroups            ec2:DescribeNetworkInterfaces            ec2:CreateNetworkInterface            ec2:CreateNetworkInterfacePermission            ec2:DeleteNetworkInterface
    /// 
    /// If you revoke these permissions after you create the delivery stream, Kinesis Data     Firehose can't scale out by creating more ENIs when necessary. You might therefore see a     degradation in performance.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// The IDs of the security groups that you want Kinesis Data Firehose to use when it     creates ENIs in the VPC of the Amazon ES destination. You can use the same security group     that the Amazon ES domain uses or different ones. If you specify different security groups     here, ensure that they allow outbound HTTPS traffic to the Amazon ES domain's security     group. Also ensure that the Amazon ES domain's security group allows HTTPS traffic from the     security groups specified here. If you use the same security group for both your delivery     stream and the Amazon ES domain, make sure the security group inbound rule allows HTTPS     traffic.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,


    /// 
    /// The IDs of the subnets that Kinesis Data Firehose uses to create ENIs in the VPC of     the Amazon ES destination. Make sure that the routing tables and inbound and outbound rules     allow traffic to flow from the subnets whose IDs are specified here to the subnets that     have the destination Amazon ES endpoints. Kinesis Data Firehose creates at least one ENI in     each of the subnets that are specified here. Do not delete or modify these ENIs.
    /// 
    /// The number of ENIs that Kinesis Data Firehose creates in the subnets specified here     scales up and down automatically based on throughput. To enable Kinesis Data Firehose to     scale up the number of ENIs to match throughput, ensure that you have sufficient quota. To     help you calculate the quota you need, assume that Kinesis Data Firehose can create up to     three ENIs for this delivery stream for each of the subnets specified here.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}


