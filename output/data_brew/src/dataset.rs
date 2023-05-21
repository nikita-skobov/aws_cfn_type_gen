

/// Specifies a new DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataset {


    /// 
    /// The unique name of the dataset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The file format of a dataset that is created from an Amazon S3 file or folder.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | EXCEL | JSON | ORC | PARQUET
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<String>,


    /// 
    /// A set of options that define how DataBrew interprets the data in the       dataset.
    /// 
    /// Required: No
    ///
    /// Type: FormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatOptions")]
    pub format_options: Option<FormatOptions>,


    /// 
    /// Metadata tags that have been applied to the dataset.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Information on how DataBrew can find the dataset, in either the AWS Glue Data Catalog or Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: Input
    ///
    /// Update requires: No interruption
    #[serde(rename = "Input")]
    pub input: Input,


    /// 
    /// A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.
    /// 
    /// Required: No
    ///
    /// Type: PathOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathOptions")]
    pub path_options: Option<PathOptions>,

}

impl cfn_resources::CfnResource for CfnDataset {
    fn type_string() -> &'static str {
        "AWS::DataBrew::Dataset"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Represents a set of options that define how DataBrew will interpret a Microsoft Excel file when       creating a dataset from that file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExcelOptions {


    /// 
    /// One or more sheet numbers in the Excel file that will be included in the       dataset.
    /// 
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetIndexes")]
    pub sheet_indexes: Option<Vec<i64>>,


    /// 
    /// A variable that specifies whether the first row in the file is parsed as the       header. If this value is false, column names are auto-generated.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderRow")]
    pub header_row: Option<bool>,


    /// 
    /// One or more named sheets in the Excel file that will be included in the dataset.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetNames")]
    pub sheet_names: Option<Vec<String>>,

}


/// Represents information on how DataBrew can find data, in either the AWS Glue Data Catalog or       Amazon S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {


    /// 
    /// Connection information for dataset input files stored in a database.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseInputDefinition")]
    pub database_input_definition: Option<DatabaseInputDefinition>,


    /// 
    /// The Amazon S3 location where the data is stored.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputDefinition")]
    pub s3_input_definition: Option<S3Location>,


    /// 
    /// Contains additional resource information needed for specific datasets.
    /// 
    /// Required: No
    ///
    /// Type: Metadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,


    /// 
    /// The AWS Glue Data Catalog parameters for the data.
    /// 
    /// Required: No
    ///
    /// Type: DataCatalogInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogInputDefinition")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,

}


/// Represents a single entry in the ValuesMap of a         FilterExpression. A FilterValue associates the name of a       substitution variable in an expression to its value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterValue {


    /// 
    /// The value to be associated with the substitution variable.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The substitution variable reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueReference")]
    pub value_reference: String,

}


/// Represents a set of options that define how DataBrew selects files for a       given Amazon S3 path in a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathOptions {


    /// 
    /// If provided, this structure defines a date range for matching Amazon S3       objects based on their LastModifiedDate attribute in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: FilterExpression
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedDateCondition")]
    pub last_modified_date_condition: Option<FilterExpression>,


    /// 
    /// If provided, this structure imposes a limit on a number of files that should be       selected.
    /// 
    /// Required: No
    ///
    /// Type: FilesLimit
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilesLimit")]
    pub files_limit: Option<FilesLimit>,


    /// 
    /// A structure that maps names of parameters used in the Amazon S3 path of a       dataset to their definitions.
    /// 
    /// Required: No
    ///
    /// Type: List of PathParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<PathParameter>>,

}


/// Represents a dataset paramater that defines type and conditions for a parameter in the         Amazon S3 path of the dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetParameter {


    /// 
    /// The name of the parameter that is used in the dataset's Amazon S3       path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The type of the dataset parameter, can be one of a 'String', 'Number' or       'Datetime'.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The optional filter expression structure to apply additional matching criteria to the       parameter.
    /// 
    /// Required: No
    ///
    /// Type: FilterExpression
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    pub filter: Option<FilterExpression>,


    /// 
    /// Optional boolean value that defines whether the captured value of this parameter       should be loaded as an additional column in the dataset.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateColumn")]
    pub create_column: Option<bool>,


    /// 
    /// Additional parameter options such as a format and a timezone. Required for datetime       parameters.
    /// 
    /// Required: No
    ///
    /// Type: DatetimeOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatetimeOptions")]
    pub datetime_options: Option<DatetimeOptions>,

}


/// Connection information for dataset input files stored in a database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseInputDefinition {


    /// 
    /// Custom SQL to run against the provided AWS Glue connection. This SQL will be used as       the input for DataBrew projects and jobs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: Option<String>,


    /// 
    /// The table within the target database.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseTableName")]
    pub database_table_name: Option<String>,


    /// 
    /// The AWS Glue Connection that stores the connection information for       the target database.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueConnectionName")]
    pub glue_connection_name: String,


    /// 
    /// An Amazon location that AWS Glue Data Catalog can use as a temporary       directory.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,

}


/// Represents a set of options that define how DataBrew will read a       comma-separated value (CSV) file when creating a dataset from that file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsvOptions {


    /// 
    /// A single character that specifies the delimiter being used in the CSV file.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,


    /// 
    /// A variable that specifies whether the first row in the file is parsed as the       header. If this value is false, column names are auto-generated.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderRow")]
    pub header_row: Option<bool>,

}


/// Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read       input data, or write output from a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// 
    /// The unique name of the object in the bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1280
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The Amazon S3 bucket name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,

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


/// Represents a single entry in the path parameters of a dataset. Each         PathParameter consists of a name and a parameter definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathParameter {


    /// 
    /// The name of the path parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathParameterName")]
    pub path_parameter_name: String,


    /// 
    /// The path parameter definition.
    /// 
    /// Required: Yes
    ///
    /// Type: DatasetParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetParameter")]
    pub dataset_parameter: DatasetParameter,

}


/// Represents a limit imposed on number of Amazon S3 files that should be       selected for a dataset from a connected Amazon S3 path.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilesLimit {


    /// 
    /// The number of Amazon S3 files to select.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxFiles")]
    pub max_files: i64,


    /// 
    /// A criteria to use for Amazon S3 files sorting before their selection. By       default uses LAST_MODIFIED_DATE as a sorting criteria. Currently it's the only allowed       value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderedBy")]
    pub ordered_by: Option<String>,


    /// 
    /// A criteria to use for Amazon S3 files sorting before their selection. By       default uses DESCENDING order, i.e. most recent files are selected first.       Anotherpossible value is ASCENDING.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Order")]
    pub order: Option<String>,

}


/// Represents how metadata stored in the AWS Glue Data Catalog is defined in a DataBrew       dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataCatalogInputDefinition {


    /// 
    /// The unique identifier of the AWS account that holds the Data Catalog that stores the       data.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// The name of a database in the Data Catalog.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// An Amazon location that AWS Glue Data Catalog can use as a temporary       directory.
    /// 
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,


    /// 
    /// The name of a database table in the Data Catalog. This table corresponds to a DataBrew       dataset.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,

}


/// Represents a structure for defining parameter conditions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterExpression {


    /// 
    /// The map of substitution variable names to their values used in this filter       expression.
    /// 
    /// Required: Yes
    ///
    /// Type: List of FilterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValuesMap")]
    pub values_map: Vec<FilterValue>,


    /// 
    /// The expression which includes condition names followed by substitution variables,       possibly grouped and combined with other conditions. For example, "(starts_with :prefix1       or starts_with :prefix2) and (ends_with :suffix1 or ends_with :suffix2)". Substitution       variables should start with ':' symbol.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

}


/// Represents a set of options that define the structure of either comma-separated value (CSV),       Excel, or JSON input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormatOptions {


    /// 
    /// Options that define how Excel input is to be interpreted by DataBrew.
    /// 
    /// Required: No
    ///
    /// Type: ExcelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excel")]
    pub excel: Option<ExcelOptions>,


    /// 
    /// Options that define how JSON input is to be interpreted by DataBrew.
    /// 
    /// Required: No
    ///
    /// Type: JsonOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Json")]
    pub json: Option<JsonOptions>,


    /// 
    /// Options that define how CSV input is to be interpreted by DataBrew.
    /// 
    /// Required: No
    ///
    /// Type: CsvOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Csv")]
    pub csv: Option<CsvOptions>,

}


/// Represents additional options for correct interpretation of datetime parameters used       in the Amazon S3 path of a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatetimeOptions {


    /// 
    /// Optional value for a timezone offset of the datetime parameter value in the Amazon S3 path. Shouldn't be used if Format for this parameter includes timezone       fields. If no offset specified, UTC is assumed.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimezoneOffset")]
    pub timezone_offset: Option<String>,


    /// 
    /// Required option, that defines the datetime format used for a date parameter in the         Amazon S3 path. Should use only supported datetime specifiers and       separation characters, all litera a-z or A-Z character should be escaped with single       quotes. E.g. "MM.dd.yyyy-'at'-HH:mm".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: String,


    /// 
    /// Optional value for a non-US locale code, needed for correct interpretation of some       date formats.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocaleCode")]
    pub locale_code: Option<String>,

}


/// Contains additional resource information needed for specific datasets.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metadata {


    /// 
    /// The Amazon Resource Name (ARN) associated with the dataset. Currently, DataBrew       only supports ARNs from Amazon AppFlow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,

}


/// Represents the JSON-specific options that define how input is to be interpreted by AWS Glue DataBrew.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonOptions {


    /// 
    /// A value that specifies whether JSON input contains embedded new line       characters.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiLine")]
    pub multi_line: Option<bool>,

}
