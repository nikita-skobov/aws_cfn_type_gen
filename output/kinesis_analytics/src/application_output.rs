

/// Adds an external destination to your Amazon Kinesis Analytics application.
///
/// If you want Amazon Kinesis Analytics to deliver data from an in-application stream       within your application to an external destination (such as an Amazon Kinesis stream, an       Amazon Kinesis Firehose delivery stream, or an Amazon Lambda function), you add the       relevant configuration to your application using this operation. You can configure one       or more outputs for your application. Each output configuration maps an in-application       stream and an external destination.
///
/// You can use one of the output configurations to deliver data from your       in-application error stream to an external destination so that you can analyze the       errors. For more information, see Understanding Application         Output (Destination).
///
/// Any configuration update, including adding a streaming source using this       operation, results in a new version of the application. You can use the DescribeApplication operation to find the current application       version.
///
/// For the limits on the number of application inputs and outputs       you can configure, see Limits.
///
/// This operation requires permissions to perform the kinesisanalytics:AddApplicationOutput action.
#[derive(Default, serde::Serialize)]
pub struct CfnApplicationOutput {


    /// 
    /// An array of objects, each describing one output configuration. In the output       configuration, you specify the name of an in-application stream, a destination (that is,       an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), and record the formation to use when writing to the       destination.
    /// 
    /// Required: Yes
    ///
    /// Type: Output
    ///
    /// Update requires: No interruption
    #[serde(rename = "Output")]
    pub output: Output,


    /// 
    /// Name of the application to which you want to add the output configuration.
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

}


/// When configuring application output, identifies an Amazon Kinesis Firehose delivery       stream as the destination. You provide the stream Amazon Resource Name (ARN) and an IAM       role that enables Amazon Kinesis Analytics to write to the stream on your behalf.
#[derive(Default, serde::Serialize)]
pub struct KinesisFirehoseOutput {


    /// 
    /// ARN of the destination Amazon Kinesis Firehose delivery stream to write to.
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
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination stream on your behalf. You need to grant the necessary permissions to this       role.
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


/// Describes application output configuration in which you identify an in-application       stream and a destination where you want the in-application stream data to be written.       The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery       stream.
/// 
/// For limits on how many destinations an application can write and other limitations,       see Limits.
#[derive(Default, serde::Serialize)]
pub struct Output {


    /// 
    /// Identifies an AWS Lambda function as the destination.
    /// 
    /// Required: No
    ///
    /// Type: LambdaOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaOutput")]
    pub lambda_output: Option<LambdaOutput>,


    /// 
    /// Identifies an Amazon Kinesis stream as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisStreamsOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisStreamsOutput")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,


    /// 
    /// Identifies an Amazon Kinesis Firehose delivery stream as the destination.
    /// 
    /// Required: No
    ///
    /// Type: KinesisFirehoseOutput
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisFirehoseOutput")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,


    /// 
    /// Name of the in-application stream.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Describes the data format when records are written to the destination. For more       information, see Configuring Application         Output.
    /// 
    /// Required: Yes
    ///
    /// Type: DestinationSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,

}


/// When configuring application output, identifies an Amazon Kinesis stream as the       destination. You provide the stream Amazon Resource Name (ARN) and also an IAM role ARN       that Amazon Kinesis Analytics can use to write to the stream on your behalf.
#[derive(Default, serde::Serialize)]
pub struct KinesisStreamsOutput {


    /// 
    /// ARN of the destination Amazon Kinesis stream to write to.
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
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination stream on your behalf. You need to grant the necessary permissions to this       role.
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


/// When configuring application output, identifies an AWS Lambda function       as the destination. You provide the function Amazon Resource Name (ARN) and also an IAM       role ARN that Amazon Kinesis Analytics can use to write to the function on your behalf.
#[derive(Default, serde::Serialize)]
pub struct LambdaOutput {


    /// 
    /// Amazon Resource Name (ARN) of the destination Lambda function to write to.
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


    /// 
    /// ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the       destination function on your behalf. You need to grant the necessary permissions to this       role.
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


/// Describes the data format when records are written to the destination. For more       information, see Configuring Application         Output.
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
