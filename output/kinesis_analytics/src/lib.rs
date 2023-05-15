
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationCode")]
    pub application_code: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationDescription")]
    pub application_description: Option<String>,
    /// List of Input
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Input>,

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
pub struct JSONMappingParameters {
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseInput {
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct InputSchema {
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,

}

#[derive(serde::Serialize, Default)]
pub struct InputProcessingConfiguration {
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: Option<InputLambdaProcessor>,

}

#[derive(serde::Serialize, Default)]
pub struct InputParallelism {
    #[serde(rename = "Count")]
    pub count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RecordFormat {
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct Input {
    #[serde(rename = "InputSchema")]
    pub input_schema: InputSchema,
    #[serde(rename = "KinesisStreamsInput")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    #[serde(rename = "InputParallelism")]
    pub input_parallelism: Option<InputParallelism>,
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,
    #[serde(rename = "KinesisFirehoseInput")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,

}

#[derive(serde::Serialize, Default)]
pub struct InputLambdaProcessor {
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamsInput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct MappingParameters {
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct CSVMappingParameters {
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,

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
pub struct Output {
    #[serde(rename = "LambdaOutput")]
    pub lambda_output: Option<LambdaOutput>,
    #[serde(rename = "KinesisStreamsOutput")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "KinesisFirehoseOutput")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisFirehoseOutput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamsOutput {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DestinationSchema {
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaOutput {
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

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
pub struct MappingParameters {
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct JSONMappingParameters {
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3ReferenceDataSource {
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    #[serde(rename = "FileKey")]
    pub file_key: String,
    #[serde(rename = "ReferenceRoleARN")]
    pub reference_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct CSVMappingParameters {
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceDataSource {
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    #[serde(rename = "S3ReferenceDataSource")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    #[serde(rename = "ReferenceSchema")]
    pub reference_schema: ReferenceSchema,

}

#[derive(serde::Serialize, Default)]
pub struct RecordColumn {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SqlType")]
    pub sql_type: String,
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RecordFormat {
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReferenceSchema {
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,

}


}
