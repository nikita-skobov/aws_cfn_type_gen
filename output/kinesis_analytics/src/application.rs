

/// The AWS::KinesisAnalytics::Application resource creates an Amazon Kinesis Data Analytics application. For more information, see the Amazon Kinesis Data Analytics Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplication {


    /// 
    /// Name of your Amazon Kinesis Analytics application (for example,         sample-app).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9_.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,


    /// 
    /// Use this parameter to configure the application input.
    /// 
    /// You can configure your application to receive input from a single streaming source. In       this configuration, you map this streaming source to an in-application stream that is       created. Your application code can then query the in-application stream like a table       (you can think of it as a constantly updating table).
    /// 
    /// For the streaming source, you provide its Amazon Resource Name (ARN) and format of       data on the stream (for example, JSON, CSV, etc.). You also must provide an IAM role       that Amazon Kinesis Analytics can assume to read this stream on your behalf.
    /// 
    /// To create the in-application stream, you need to specify a schema to transform your       data into a schematized version used in SQL. In the schema, you provide the necessary       mapping of the data elements in the streaming source to record columns in the in-app       stream.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Input
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Input>,


    /// 
    /// Summary description of the application.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationDescription")]
    pub application_description: Option<String>,


    /// 
    /// One or more SQL statements that read input data, transform it, and generate output.       For example, you can write a SQL statement that reads data from one in-application       stream, generates a running average of the number of advertisement clicks by vendor, and       insert resulting rows in another in-application stream using pumps. For more information       about the typical pattern, see Application         Code.
    /// 
    /// You can provide such series of SQL statements, where output of one statement can be       used as the input for the next statement. You store intermediate results by creating       in-application streams and pumps.
    /// 
    /// Note that the application code must create the streams with names specified in the         Outputs. For example, if your Outputs defines output       streams named ExampleOutputStream1 and ExampleOutputStream2,       then your application code must create these streams.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 102400
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationCode")]
    pub application_code: Option<String>,

}

impl cfn_resources::CfnResource for CfnApplication {
    fn type_string() -> &'static str {
        "AWS::KinesisAnalytics::Application"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes the mapping of each data element in the streaming source to the       corresponding column in the in-application stream.
///
/// Also used to describe the format of the reference data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordColumn {


    /// 
    /// Name of the column created in the in-application input stream or reference       table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Reference to the data element in the streaming input or the reference data source.       This element is required if the RecordFormatType is JSON.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,


    /// 
    /// Type of column created in the in-application input stream or reference table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlType")]
    pub sql_type: String,

}


/// Provides a description of a processor that is used to preprocess the records in the       stream before being processed by your application code. Currently, the only input       processor available is AWS Lambda.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputProcessingConfiguration {


    /// 
    /// The InputLambdaProcessor that is used to preprocess the records       in the stream before being processed by your application code.
    /// 
    /// Required: No
    ///
    /// Type: InputLambdaProcessor
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: Option<InputLambdaProcessor>,

}


/// Provides additional mapping information when the record format uses delimiters, such       as CSV. For example, the following sample records use CSV format, where the records use       the '\n' as the row delimiter and a comma (",") as the column       delimiter:
///
/// "name1", "address1"
///
/// "name2", "address2"
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CSVMappingParameters {


    /// 
    /// Row delimiter. For example, in a CSV format, '\n' is the typical       row delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,


    /// 
    /// Column delimiter. For example, in a CSV format, a comma (",") is the typical column       delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,

}


/// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.
///
/// Also used to describe the format of the reference data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputSchema {


    /// 
    /// Specifies the format of the records on the streaming source.
    /// 
    /// Required: Yes
    ///
    /// Type: RecordFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,


    /// 
    /// Specifies the encoding of the records in the streaming source. For example, UTF-8.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: UTF-8
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordEncoding")]
    pub record_encoding: Option<String>,


    /// 
    /// A list of RecordColumn objects.
    /// 
    /// Required: Yes
    ///
    /// Type: List of RecordColumn
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,

}


/// Provides additional mapping information when JSON is the record format on the       streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JSONMappingParameters {


    /// 
    /// Path to the top-level parent that contains the records.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}


/// Identifies an Amazon Kinesis Firehose delivery stream as the streaming source. You       provide the delivery stream's Amazon Resource Name (ARN) and an IAM role ARN that       enables Amazon Kinesis Analytics to access the stream on your behalf.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisFirehoseInput {


    /// 
    /// ARN of the input delivery stream.
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
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,


    /// 
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on       your behalf. You need to make sure that the role has the necessary permissions to access       the stream.
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
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}


/// An object that contains the Amazon Resource Name (ARN) of the AWS Lambda function that is used to preprocess records in the       stream, and the ARN of the IAM role that is used to access the AWS Lambda       function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputLambdaProcessor {


    /// 
    /// The ARN of the IAM role that is used to access the AWS Lambda       function.
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
    #[serde(rename = "RoleARN")]
    pub role_arn: String,


    /// 
    /// The ARN of the AWS Lambda function that operates on records in the       stream.
    /// 
    /// NoteTo specify an earlier version of the Lambda function than the latest, include the         Lambda function version in the Lambda function ARN. For more information about         Lambda ARNs, see Example           ARNs: AWS Lambda
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
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,

}


/// When configuring application input at the time of creating or updating an application,       provides additional mapping information specific to the record format (such as JSON,       CSV, or record fields delimited by some delimiter) on the streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MappingParameters {


    /// 
    /// Provides additional mapping information when the record format uses delimiters (for       example, CSV).
    /// 
    /// Required: No
    ///
    /// Type: CSVMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,


    /// 
    /// Provides additional mapping information when JSON is the record format on the       streaming source.
    /// 
    /// Required: No
    ///
    /// Type: JSONMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,

}


/// Identifies an Amazon Kinesis stream as the streaming source. You provide the stream's       Amazon Resource Name (ARN) and an IAM role ARN that enables Amazon Kinesis Analytics to       access the stream on your behalf.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisStreamsInput {


    /// 
    /// ARN of the input Amazon Kinesis stream to read.
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
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,


    /// 
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on       your behalf. You need to grant the necessary permissions to this role.
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
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}


/// Describes the record format and relevant mapping information that should be applied       to schematize the records on the stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordFormat {


    /// 
    /// The type of record format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,


    /// 
    /// When configuring application input at the time of creating or updating an application,       provides additional mapping information specific to the record format (such as JSON,       CSV, or record fields delimited by some delimiter) on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: MappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,

}


/// Describes the number of in-application streams to create for a given streaming source.       For information about parallelism, see Configuring Application         Input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InputParallelism {


    /// 
    /// Number of in-application streams to create. For more information, see Limits.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,

}


/// When you configure the application input, you specify the streaming source, the       in-application stream name that is created, and the mapping between the two. For more       information, see Configuring Application         Input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {


    /// 
    /// If the streaming source is an Amazon Kinesis stream, identifies the stream's Amazon       Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to access the       stream on your behalf.
    /// 
    /// Note: Either KinesisStreamsInput or KinesisFirehoseInput is       required.
    /// 
    /// Required: Conditional
    ///
    /// Type: KinesisStreamsInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsInput")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,


    /// 
    /// If the streaming source is an Amazon Kinesis Firehose delivery stream, identifies the       delivery stream's ARN and an IAM role that enables Amazon Kinesis Analytics to access       the stream on your behalf.
    /// 
    /// Note: Either KinesisStreamsInput or KinesisFirehoseInput is       required.
    /// 
    /// Required: Conditional
    ///
    /// Type: KinesisFirehoseInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseInput")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,


    /// 
    /// Describes the format of the data in the streaming source, and how each data element       maps to corresponding columns in the in-application stream that is being created.
    /// 
    /// Also used to describe the format of the reference data source.
    /// 
    /// Required: Yes
    ///
    /// Type: InputSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputSchema")]
    pub input_schema: InputSchema,


    /// 
    /// The InputProcessingConfiguration for the input. An input       processor transforms records as they are received from the stream, before the       application's SQL code executes. Currently, the only input processing configuration       available is InputLambdaProcessor.
    /// 
    /// Required: No
    ///
    /// Type: InputProcessingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,


    /// 
    /// Name prefix to use when creating an in-application stream. Suppose that you specify a       prefix "MyInApplicationStream." Amazon Kinesis Analytics then creates one or more (as       per the InputParallelism count you specified) in-application streams with       names "MyInApplicationStream_001," "MyInApplicationStream_002," and so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,


    /// 
    /// Describes the number of in-application streams to create.
    /// 
    /// Data from your source is routed to these in-application input streams.
    /// 
    /// See Configuring Application Input.
    /// 
    /// Required: No
    ///
    /// Type: InputParallelism
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputParallelism")]
    pub input_parallelism: Option<InputParallelism>,

}
