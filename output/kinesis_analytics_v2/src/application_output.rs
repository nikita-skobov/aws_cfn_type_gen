

/// Adds an external destination to your SQL-based Amazon Kinesis Data Analytics       application.
///
/// If you want Kinesis Data Analytics to deliver data from an in-application stream       within your application to an external destination (such as an Kinesis data stream, a       Kinesis Data Firehose delivery stream, or an Amazon Lambda function), you add the relevant       configuration to your application using this operation. You can configure one or more       outputs for your application. Each output configuration maps an in-application stream       and an external destination.
///
/// You can use one of the output configurations to deliver data from your in-application       error stream to an external destination so that you can analyze the errors.
///
/// Any configuration update, including adding a streaming source using this operation,       results in a new version of the application. You can use the DescribeApplication operation to find the current application       version.
#[derive(Default, serde::Serialize)]
pub struct CfnApplicationOutput {


    /// 
    /// The name of the application.
    /// 
    /// Required: Yes
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
    pub application_name: String,


    /// 
    /// Describes a SQL-based Kinesis Data Analytics application's output configuration,    in which you identify an in-application stream and a destination where you want the    in-application stream data to be written. The destination can be a Kinesis data stream or a    Kinesis Data Firehose delivery stream.
    /// 
    /// 
    /// 
    /// Required: Yes
    ///
    /// Type: Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Output")]
    pub output: Output,

}


/// When you configure a SQL-based Kinesis Data Analytics application's output,    identifies an Amazon Lambda function as the destination. You provide the function Amazon Resource    Name (ARN) of the Lambda function.
#[derive(Default, serde::Serialize)]
pub struct LambdaOutput {


    /// 
    /// The Amazon Resource Name (ARN) of the destination Lambda function to write to.
    /// 
    /// NoteTo specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see Example ARNs: Amazon Lambda
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


/// Describes a SQL-based Kinesis Data Analytics application's output configuration,    in which you identify an in-application stream and a destination where you want the    in-application stream data to be written. The destination can be a Kinesis data stream or a    Kinesis Data Firehose delivery stream.
/// 
#[derive(Default, serde::Serialize)]
pub struct Output {


    /// 
    /// Identifies a Kinesis Data Firehose delivery stream as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisFirehoseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseOutput")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,


    /// 
    /// Describes the data format when records are written to the destination.
    /// 
    /// Required: Yes
    ///
    /// Type: DestinationSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,


    /// 
    /// Identifies a Kinesis data stream    as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamsOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsOutput")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,


    /// 
    /// Identifies an Amazon Lambda function as the destination.
    /// 
    /// Required: No
    ///
    /// Type: LambdaOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaOutput")]
    pub lambda_output: Option<LambdaOutput>,


    /// 
    /// The name of the in-application stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Pattern: [^-\s<>&]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// For a SQL-based Kinesis Data Analytics application, when configuring application    output, identifies a Kinesis Data Firehose delivery stream as the destination. You provide the    stream Amazon Resource Name (ARN) of the delivery stream.
#[derive(Default, serde::Serialize)]
pub struct KinesisFirehoseOutput {


    /// 
    /// The ARN of the destination delivery stream to write to.
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


/// Describes the data format when records are written to the destination in a SQL-based Kinesis Data Analytics application.
#[derive(Default, serde::Serialize)]
pub struct DestinationSchema {


    /// 
    /// Specifies the format of the records on the output stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: Option<String>,

}


/// When you configure a SQL-based Kinesis Data Analytics application's output,    identifies a Kinesis data stream as the destination. You provide the stream Amazon Resource    Name (ARN).
#[derive(Default, serde::Serialize)]
pub struct KinesisStreamsOutput {


    /// 
    /// The ARN of the destination Kinesis data stream to write to.
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
