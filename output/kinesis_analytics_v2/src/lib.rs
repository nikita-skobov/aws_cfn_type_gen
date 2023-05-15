
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// The name of the application.
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,
    /// To create a Kinesis Data Analytics Studio notebook, you must set the mode to `INTERACTIVE`. However, for a Kinesis Data Analytics for Apache Flink application, the mode is optional.
    #[serde(rename = "ApplicationMode")]
    pub application_mode: Option<String>,
    /// Specifies run configuration (start parameters) of a Kinesis Data Analytics application. Evaluated on update for RUNNING applications an only.
    #[serde(rename = "RunConfiguration")]
    pub run_configuration: Option<RunConfiguration>,
    /// The runtime environment for the application.
    #[serde(rename = "RuntimeEnvironment")]
    pub runtime_environment: String,
    /// The description of the application.
    #[serde(rename = "ApplicationDescription")]
    pub application_description: Option<String>,
    /// Use this parameter to configure the application.
    #[serde(rename = "ApplicationConfiguration")]
    pub application_configuration: Option<ApplicationConfiguration>,
    /// Specifies the IAM role that the application uses to access external resources.
    #[serde(rename = "ServiceExecutionRole")]
    pub service_execution_role: Arn,
    /// A list of one or more tags to assign to the application. A tag is a key-value pair that identifies an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Used to configure start of maintenance window.
    #[serde(rename = "ApplicationMaintenanceConfiguration")]
    pub application_maintenance_configuration: Option<ApplicationMaintenanceConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct FlinkApplicationConfiguration {
    #[serde(rename = "MonitoringConfiguration")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "CheckpointConfiguration")]
    pub checkpoint_configuration: Option<CheckpointConfiguration>,
    #[serde(rename = "ParallelismConfiguration")]
    pub parallelism_configuration: Option<ParallelismConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct JSONMappingParameters {
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct GlueDataCatalogConfiguration {
    #[serde(rename = "DatabaseARN")]
    pub database_arn: Option<Arn>,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationSnapshotConfiguration {
    #[serde(rename = "SnapshotsEnabled")]
    pub snapshots_enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationConfiguration {
    #[serde(rename = "VpcConfigurations")]
    pub vpc_configurations: Option<VpcConfigurations>,
    #[serde(rename = "ZeppelinApplicationConfiguration")]
    pub zeppelin_application_configuration: Option<ZeppelinApplicationConfiguration>,
    #[serde(rename = "FlinkApplicationConfiguration")]
    pub flink_application_configuration: Option<FlinkApplicationConfiguration>,
    #[serde(rename = "SqlApplicationConfiguration")]
    pub sql_application_configuration: Option<SqlApplicationConfiguration>,
    #[serde(rename = "ApplicationCodeConfiguration")]
    pub application_code_configuration: Option<ApplicationCodeConfiguration>,
    #[serde(rename = "ApplicationSnapshotConfiguration")]
    pub application_snapshot_configuration: Option<ApplicationSnapshotConfiguration>,
    #[serde(rename = "EnvironmentProperties")]
    pub environment_properties: Option<EnvironmentProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomArtifactsConfiguration {

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationRestoreConfiguration {
    #[serde(rename = "SnapshotName")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "ApplicationRestoreType")]
    pub application_restore_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct CodeContent {
    #[serde(rename = "TextContent")]
    pub text_content: Option<String>,
    #[serde(rename = "ZipFileContent")]
    pub zip_file_content: Option<String>,
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: Option<S3ContentLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct SqlApplicationConfiguration {
    #[serde(rename = "Inputs")]
    pub inputs: Option<Vec<Input>>,

}

#[derive(serde::Serialize, Default)]
pub struct ParallelismConfiguration {
    #[serde(rename = "Parallelism")]
    pub parallelism: Option<usize>,
    #[serde(rename = "AutoScalingEnabled")]
    pub auto_scaling_enabled: Option<bool>,
    #[serde(rename = "ParallelismPerKPU")]
    pub parallelism_per_kpu: Option<usize>,
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct Input {
    #[serde(rename = "InputSchema")]
    pub input_schema: InputSchema,
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,
    #[serde(rename = "KinesisStreamsInput")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    #[serde(rename = "InputParallelism")]
    pub input_parallelism: Option<InputParallelism>,
    #[serde(rename = "KinesisFirehoseInput")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct InputSchema {
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamsInput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct CustomArtifactConfiguration {
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: Option<S3ContentLocation>,
    #[serde(rename = "MavenReference")]
    pub maven_reference: Option<MavenReference>,
    #[serde(rename = "ArtifactType")]
    pub artifact_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationMaintenanceConfiguration {
    #[serde(rename = "ApplicationMaintenanceWindowStartTime")]
    pub application_maintenance_window_start_time: String,

}

#[derive(serde::Serialize, Default)]
pub struct CheckpointConfiguration {
    #[serde(rename = "MinPauseBetweenCheckpoints")]
    pub min_pause_between_checkpoints: Option<usize>,
    #[serde(rename = "CheckpointInterval")]
    pub checkpoint_interval: Option<usize>,
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: String,
    #[serde(rename = "CheckpointingEnabled")]
    pub checkpointing_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PropertyGroup {
    #[serde(rename = "PropertyGroupId")]
    pub property_group_id: Option<String>,
    #[serde(rename = "PropertyMap")]
    pub property_map: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct RunConfiguration {
    #[serde(rename = "FlinkRunConfiguration")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,
    #[serde(rename = "ApplicationRestoreConfiguration")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct CSVMappingParameters {
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct RecordColumn {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,
    #[serde(rename = "SqlType")]
    pub sql_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct MappingParameters {
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct EnvironmentProperties {
    #[serde(rename = "PropertyGroups")]
    pub property_groups: Option<Vec<PropertyGroup>>,

}

#[derive(serde::Serialize, Default)]
pub struct DeployAsApplicationConfiguration {
    #[serde(rename = "S3ContentLocation")]
    pub s3_content_location: S3ContentBaseLocation,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfigurations {

}

#[derive(serde::Serialize, Default)]
pub struct MavenReference {
    #[serde(rename = "GroupId")]
    pub group_id: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ArtifactId")]
    pub artifact_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringConfiguration {
    #[serde(rename = "ConfigurationType")]
    pub configuration_type: String,
    #[serde(rename = "MetricsLevel")]
    pub metrics_level: Option<String>,
    #[serde(rename = "LogLevel")]
    pub log_level: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseInput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct InputLambdaProcessor {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct ZeppelinApplicationConfiguration {
    #[serde(rename = "DeployAsApplicationConfiguration")]
    pub deploy_as_application_configuration: Option<DeployAsApplicationConfiguration>,
    #[serde(rename = "MonitoringConfiguration")]
    pub monitoring_configuration: Option<ZeppelinMonitoringConfiguration>,
    #[serde(rename = "CustomArtifactsConfiguration")]
    pub custom_artifacts_configuration: Option<CustomArtifactsConfiguration>,
    #[serde(rename = "CatalogConfiguration")]
    pub catalog_configuration: Option<CatalogConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationCodeConfiguration {
    #[serde(rename = "CodeContentType")]
    pub code_content_type: String,
    #[serde(rename = "CodeContent")]
    pub code_content: CodeContent,

}

#[derive(serde::Serialize, Default)]
pub struct FlinkRunConfiguration {
    #[serde(rename = "AllowNonRestoredState")]
    pub allow_non_restored_state: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct InputProcessingConfiguration {
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: Option<InputLambdaProcessor>,

}

#[derive(serde::Serialize, Default)]
pub struct RecordFormat {
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct InputParallelism {
    #[serde(rename = "Count")]
    pub count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct S3ContentLocation {
    #[serde(rename = "FileKey")]
    pub file_key: String,
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,
    #[serde(rename = "BucketARN")]
    pub bucket_arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct CatalogConfiguration {
    #[serde(rename = "GlueDataCatalogConfiguration")]
    pub glue_data_catalog_configuration: Option<GlueDataCatalogConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ZeppelinMonitoringConfiguration {
    #[serde(rename = "LogLevel")]
    pub log_level: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3ContentBaseLocation {
    #[serde(rename = "BucketARN")]
    pub bucket_arn: Arn,
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,

}


}

pub mod cfn_application_cloud_watch_logging_option {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationCloudWatchLoggingOption {
    /// No documentation provided by AWS
    #[serde(rename = "CloudWatchLoggingOption")]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct CloudWatchLoggingOption {
    #[serde(rename = "LogStreamARN")]
    pub log_stream_arn: String,

}


}

pub mod cfn_application_output {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationOutput {
    /// No documentation provided by AWS
    #[serde(rename = "Output")]
    pub output: Output,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct KinesisStreamsOutput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaOutput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseOutput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationSchema {
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Output {
    #[serde(rename = "KinesisStreamsOutput")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    #[serde(rename = "KinesisFirehoseOutput")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,
    #[serde(rename = "LambdaOutput")]
    pub lambda_output: Option<LambdaOutput>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


}

pub mod cfn_application_reference_data_source {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationReferenceDataSource {
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,

}


#[derive(serde::Serialize, Default)]
pub struct RecordColumn {
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,
    #[serde(rename = "SqlType")]
    pub sql_type: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct RecordFormat {
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct S3ReferenceDataSource {
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    #[serde(rename = "FileKey")]
    pub file_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MappingParameters {
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct JSONMappingParameters {
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct CSVMappingParameters {
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceSchema {
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceDataSource {
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    #[serde(rename = "ReferenceSchema")]
    pub reference_schema: ReferenceSchema,
    #[serde(rename = "S3ReferenceDataSource")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,

}


}
