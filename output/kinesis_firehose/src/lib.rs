
pub mod cfn_delivery_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnDeliveryStream {
    /// No documentation provided by AWS
    #[serde(rename = "RedshiftDestinationConfiguration")]
    pub redshift_destination_configuration: Option<RedshiftDestinationConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "S3DestinationConfiguration")]
    pub s3_destination_configuration: Option<S3DestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "KinesisStreamSourceConfiguration")]
    pub kinesis_stream_source_configuration: Option<KinesisStreamSourceConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AmazonopensearchserviceDestinationConfiguration")]
    pub amazonopensearchservice_destination_configuration: Option<AmazonopensearchserviceDestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    pub delivery_stream_encryption_configuration_input: Option<DeliveryStreamEncryptionConfigurationInput>,
    /// No documentation provided by AWS
    #[serde(rename = "AmazonOpenSearchServerlessDestinationConfiguration")]
    pub amazon_open_search_serverless_destination_configuration: Option<AmazonOpenSearchServerlessDestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "SplunkDestinationConfiguration")]
    pub splunk_destination_configuration: Option<SplunkDestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "HttpEndpointDestinationConfiguration")]
    pub http_endpoint_destination_configuration: Option<HttpEndpointDestinationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DeliveryStreamType")]
    pub delivery_stream_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct OrcSerDe {
    #[serde(rename = "DictionaryKeyThreshold")]
    pub dictionary_key_threshold: Option<f64>,
    #[serde(rename = "EnablePadding")]
    pub enable_padding: Option<bool>,
    #[serde(rename = "BloomFilterColumns")]
    pub bloom_filter_columns: Option<Vec<String>>,
    #[serde(rename = "FormatVersion")]
    pub format_version: Option<String>,
    #[serde(rename = "PaddingTolerance")]
    pub padding_tolerance: Option<f64>,
    #[serde(rename = "BlockSizeBytes")]
    pub block_size_bytes: Option<usize>,
    #[serde(rename = "StripeSizeBytes")]
    pub stripe_size_bytes: Option<usize>,
    #[serde(rename = "RowIndexStride")]
    pub row_index_stride: Option<usize>,
    #[serde(rename = "Compression")]
    pub compression: Option<String>,
    #[serde(rename = "BloomFilterFalsePositiveProbability")]
    pub bloom_filter_false_positive_probability: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<usize>,
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ParquetSerDe {
    #[serde(rename = "PageSizeBytes")]
    pub page_size_bytes: Option<usize>,
    #[serde(rename = "EnableDictionaryCompression")]
    pub enable_dictionary_compression: Option<bool>,
    #[serde(rename = "WriterVersion")]
    pub writer_version: Option<String>,
    #[serde(rename = "BlockSizeBytes")]
    pub block_size_bytes: Option<usize>,
    #[serde(rename = "MaxPaddingBytes")]
    pub max_padding_bytes: Option<usize>,
    #[serde(rename = "Compression")]
    pub compression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamSourceConfiguration {
    #[serde(rename = "KinesisStreamARN")]
    pub kinesis_stream_arn: String,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DocumentIdOptions {
    #[serde(rename = "DefaultDocumentIdFormat")]
    pub default_document_id_format: String,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamicPartitioningConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RetryOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonopensearchserviceDestinationConfiguration {
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<AmazonopensearchserviceBufferingHints>,
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,
    #[serde(rename = "DocumentIdOptions")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "IndexRotationPeriod")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "ClusterEndpoint")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DomainARN")]
    pub domain_arn: Option<String>,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<AmazonopensearchserviceRetryOptions>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonopensearchserviceBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<usize>,
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<usize>,
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RedshiftDestinationConfiguration {
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "ClusterJDBCURL")]
    pub cluster_jdbcurl: String,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "S3BackupConfiguration")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "CopyCommand")]
    pub copy_command: CopyCommand,
    #[serde(rename = "Password")]
    pub password: String,

}

#[derive(serde::Serialize, Default)]
pub struct RedshiftRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchDestinationConfiguration {
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "IndexRotationPeriod")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    #[serde(rename = "ClusterEndpoint")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,
    #[serde(rename = "DocumentIdOptions")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    pub domain_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonOpenSearchServerlessDestinationConfiguration {
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<AmazonOpenSearchServerlessBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "CollectionEndpoint")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<AmazonOpenSearchServerlessRetryOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonopensearchserviceRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SplunkRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DataFormatConversionConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "SchemaConfiguration")]
    pub schema_configuration: Option<SchemaConfiguration>,
    #[serde(rename = "InputFormatConfiguration")]
    pub input_format_configuration: Option<InputFormatConfiguration>,
    #[serde(rename = "OutputFormatConfiguration")]
    pub output_format_configuration: Option<OutputFormatConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpEndpointCommonAttribute {
    #[serde(rename = "AttributeValue")]
    pub attribute_value: String,
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct KMSEncryptionConfig {
    #[serde(rename = "AWSKMSKeyARN")]
    pub awskmskey_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeliveryStreamEncryptionConfigurationInput {
    #[serde(rename = "KeyARN")]
    pub key_arn: Option<String>,
    #[serde(rename = "KeyType")]
    pub key_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct HttpEndpointRequestConfiguration {
    #[serde(rename = "ContentEncoding")]
    pub content_encoding: Option<String>,
    #[serde(rename = "CommonAttributes")]
    pub common_attributes: Option<Vec<HttpEndpointCommonAttribute>>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KMSEncryptionConfig")]
    pub kmsencryption_config: Option<KMSEncryptionConfig>,
    #[serde(rename = "NoEncryptionConfig")]
    pub no_encryption_config: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLoggingOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpEndpointConfiguration {
    #[serde(rename = "AccessKey")]
    pub access_key: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Url")]
    pub url: String,

}

#[derive(serde::Serialize, Default)]
pub struct OutputFormatConfiguration {
    #[serde(rename = "Serializer")]
    pub serializer: Option<Serializer>,

}

#[derive(serde::Serialize, Default)]
pub struct SplunkDestinationConfiguration {
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "HECToken")]
    pub hectoken: String,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "HECEndpoint")]
    pub hecendpoint: String,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<SplunkRetryOptions>,
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    pub hecacknowledgment_timeout_in_seconds: Option<usize>,
    #[serde(rename = "HECEndpointType")]
    pub hecendpoint_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct Processor {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ProcessorParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonOpenSearchServerlessRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpEndpointDestinationConfiguration {
    #[serde(rename = "RequestConfiguration")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: HttpEndpointConfiguration,
    #[serde(rename = "RetryOptions")]
    pub retry_options: Option<RetryOptions>,
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct CopyCommand {
    #[serde(rename = "DataTableColumns")]
    pub data_table_columns: Option<String>,
    #[serde(rename = "DataTableName")]
    pub data_table_name: String,
    #[serde(rename = "CopyOptions")]
    pub copy_options: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Serializer {
    #[serde(rename = "OrcSerDe")]
    pub orc_ser_de: Option<OrcSerDe>,
    #[serde(rename = "ParquetSerDe")]
    pub parquet_ser_de: Option<ParquetSerDe>,

}

#[derive(serde::Serialize, Default)]
pub struct Deserializer {
    #[serde(rename = "OpenXJsonSerDe")]
    pub open_xjson_ser_de: Option<OpenXJsonSerDe>,
    #[serde(rename = "HiveJsonSerDe")]
    pub hive_json_ser_de: Option<HiveJsonSerDe>,

}

#[derive(serde::Serialize, Default)]
pub struct AmazonOpenSearchServerlessBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    pub interval_in_seconds: Option<usize>,
    #[serde(rename = "SizeInMBs")]
    pub size_in_mbs: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HiveJsonSerDe {
    #[serde(rename = "TimestampFormats")]
    pub timestamp_formats: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct S3DestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "ErrorOutputPrefix")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct ProcessingConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Processors")]
    pub processors: Option<Vec<Processor>>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaConfiguration {
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProcessorParameter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExtendedS3DestinationConfiguration {
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<String>,
    #[serde(rename = "S3BackupMode")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3BackupConfiguration")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "DynamicPartitioningConfiguration")]
    pub dynamic_partitioning_configuration: Option<DynamicPartitioningConfiguration>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "DataFormatConversionConfiguration")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "ProcessingConfiguration")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct OpenXJsonSerDe {
    #[serde(rename = "ColumnToJsonKeyMappings")]
    pub column_to_json_key_mappings: Option<()>,
    #[serde(rename = "ConvertDotsInJsonKeysToUnderscores")]
    pub convert_dots_in_json_keys_to_underscores: Option<bool>,
    #[serde(rename = "CaseInsensitive")]
    pub case_insensitive: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct InputFormatConfiguration {
    #[serde(rename = "Deserializer")]
    pub deserializer: Option<Deserializer>,

}


}
