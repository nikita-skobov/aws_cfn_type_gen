

/// Adds a reference data source to an existing SQL-based Kinesis Data Analytics application.
///
/// Kinesis Data Analytics reads reference data (that is, an Amazon S3 object) and creates an    in-application table within your application. In the request, you provide the source (S3    bucket name and object key name), name of the in-application table to create, and the    necessary mapping information that describes how data in an Amazon S3 object maps to columns    in the resulting in-application table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApplicationReferenceDataSource {


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
    /// For a SQL-based Kinesis Data Analytics application, describes the reference data    source by providing the source information (Amazon S3 bucket name and object key name), the    resulting in-application table name that is created, and the necessary schema to map the data    elements in the Amazon S3 object to the in-application table.
    /// 
    /// Required: Yes
    ///
    /// Type: ReferenceDataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,

}

impl cfn_resources::CfnResource for CfnApplicationReferenceDataSource {
    fn type_string() -> &'static str {
        "AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// When you configure a SQL-based Kinesis Data Analytics application's input at the    time of creating or updating an application, provides additional mapping information specific    to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the    streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MappingParameters {


    /// 
    /// Provides additional mapping information when JSON is the record format on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: JSONMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "JSONMappingParameters")]
    pub jsonmapping_parameters: Option<JSONMappingParameters>,


    /// 
    /// Provides additional mapping information when the record format uses delimiters    (for example, CSV).
    /// 
    /// Required: No
    ///
    /// Type: CSVMappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "CSVMappingParameters")]
    pub csvmapping_parameters: Option<CSVMappingParameters>,

}


/// For a SQL-based Kinesis Data Analytics application, describes the format of the    data in the streaming source, and how each data element maps to corresponding columns created    in the in-application stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferenceSchema {


    /// 
    /// Specifies the encoding of the records in the streaming source. For example, UTF-8.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 5
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

}


/// For a SQL-based Kinesis Data Analytics application, describes the record format    and relevant mapping information that should be applied to schematize the records on the    stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordFormat {


    /// 
    /// When you configure application input at the time of creating or updating an application,    provides additional mapping information specific to the record format (such as JSON, CSV, or    record fields delimited by some delimiter) on the streaming source.
    /// 
    /// Required: No
    ///
    /// Type: MappingParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "MappingParameters")]
    pub mapping_parameters: Option<MappingParameters>,


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

}


/// For a SQL-based Kinesis Data Analytics application, describes the reference data    source by providing the source information (Amazon S3 bucket name and object key name), the    resulting in-application table name that is created, and the necessary schema to map the data    elements in the Amazon S3 object to the in-application table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReferenceDataSource {


    /// 
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.
    /// 
    /// Required: Yes
    ///
    /// Type: ReferenceSchema
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceSchema")]
    pub reference_schema: ReferenceSchema,


    /// 
    /// The name of the in-application table to create.
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
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,


    /// 
    /// Identifies the S3 bucket and object that contains the reference data. A Kinesis Data       Analytics application loads reference data only once. If the data changes, you call the         UpdateApplication operation to trigger reloading of data into your       application.
    /// 
    /// Required: No
    ///
    /// Type: S3ReferenceDataSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ReferenceDataSource")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,

}


/// For an SQL-based Amazon Kinesis Data Analytics application, identifies the Amazon S3       bucket and object that contains the reference data.
///
/// A Kinesis Data Analytics application loads reference data only once. If the data       changes, you call the UpdateApplication operation to trigger reloading of data into your       application.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3ReferenceDataSource {


    /// 
    /// The object key name containing the reference data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileKey")]
    pub file_key: String,


    /// 
    /// The Amazon Resource Name (ARN) of the S3 bucket.
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

}


/// For a SQL-based Kinesis Data Analytics application, provides additional mapping information when the record    format uses delimiters, such as CSV. For example, the following sample records use CSV format,    where the records use the '\n' as the row delimiter and a comma (",") as    the column delimiter:
///
/// "name1", "address1"
///
/// "name2", "address2"
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CSVMappingParameters {


    /// 
    /// The row delimiter. For example, in a CSV format, '\n' is the typical    row delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,


    /// 
    /// The column delimiter. For example, in a CSV format, a comma (",") is the typical column    delimiter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,

}


/// For a SQL-based Kinesis Data Analytics application, describes the mapping of each    data element in the streaming source to the corresponding column in the in-application    stream.
///
/// Also used to describe the format of the reference data source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecordColumn {


    /// 
    /// The name of the column that is created in the in-application input stream or reference    table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [^-\s<>&]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The type of column created in the in-application input stream or reference table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqlType")]
    pub sql_type: String,


    /// 
    /// A reference to the data element in the streaming input or the reference data    source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mapping")]
    pub mapping: Option<String>,

}


/// For a SQL-based Kinesis Data Analytics application, provides additional mapping    information when JSON is the record format on the streaming source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JSONMappingParameters {


    /// 
    /// The path to the top-level parent that contains the records.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Pattern: ^(?=^\$)(?=^\S+$).*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,

}
