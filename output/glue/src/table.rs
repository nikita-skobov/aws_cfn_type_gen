

/// The AWS::Glue::Table resource specifies tabular data in the AWS Glue data       catalog. For more information, see Defining Tables in the AWS Glue Data         Catalog and Table Structure in the AWS Glue Developer       Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnTable {


    /// 
    /// The name of the database where the table metadata resides.    For Hive compatibility, this must be all lowercase.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// A structure used to define a table.
    /// 
    /// Required: Yes
    ///
    /// Type: TableInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,


    /// 
    /// The ID of the Data Catalog in which to create the Table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


/// Describes the physical storage of table data.
#[derive(Default, serde::Serialize)]
pub struct StorageDescriptor {


    /// 
    /// True if the data in the table is compressed, or False if    not.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,


    /// 
    /// The output format: SequenceFileOutputFormat (binary),    or IgnoreKeyTextOutputFormat, or a custom format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputFormat")]
    pub output_format: Option<String>,


    /// 
    /// The user-supplied properties in key-value form.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// True if the table data is stored in subdirectories, or False if    not.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "StoredAsSubDirectories")]
    pub stored_as_sub_directories: Option<bool>,


    /// 
    /// The serialization/deserialization (SerDe) information.
    /// 
    /// Required: No
    ///
    /// Type: SerdeInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "SerdeInfo")]
    pub serde_info: Option<SerdeInfo>,


    /// 
    /// A list of the Columns in the table.
    /// 
    /// Required: No
    ///
    /// Type: List of Column
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<Column>>,


    /// 
    /// An object that references a schema stored in the AWS Glue Schema Registry.
    /// 
    /// Required: No
    ///
    /// Type: SchemaReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaReference")]
    pub schema_reference: Option<SchemaReference>,


    /// 
    /// A list of reducer grouping columns, clustering columns, and    bucketing columns in the table.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketColumns")]
    pub bucket_columns: Option<Vec<String>>,


    /// 
    /// The information about values that appear frequently in a column (skewed values).
    /// 
    /// Required: No
    ///
    /// Type: SkewedInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedInfo")]
    pub skewed_info: Option<SkewedInfo>,


    /// 
    /// A list specifying the sort order of each bucket in the table.
    /// 
    /// Required: No
    ///
    /// Type: List of Order
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortColumns")]
    pub sort_columns: Option<Vec<Order>>,


    /// 
    /// The input format: SequenceFileInputFormat (binary),    or TextInputFormat, or a custom format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputFormat")]
    pub input_format: Option<String>,


    /// 
    /// The physical location of the table. By default, this takes the form of the warehouse    location, followed by the database location in the warehouse, followed by the table    name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2056
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<String>,


    /// 
    /// Must be specified if the table contains any dimension columns.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfBuckets")]
    pub number_of_buckets: Option<i64>,

}


/// A structure used to define a table.
#[derive(Default, serde::Serialize)]
pub struct TableInput {


    /// 
    /// Included for Apache Hive compatibility. Not used in the normal course of AWS Glue operations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 409600
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewExpandedText")]
    pub view_expanded_text: Option<String>,


    /// 
    /// A description of the table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Included for Apache Hive compatibility. Not used in the normal course of AWS Glue operations.   If the table is a VIRTUAL_VIEW, certain Athena configuration encoded in base64.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 409600
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewOriginalText")]
    pub view_original_text: Option<String>,


    /// 
    /// A TableIdentifier structure that describes a target table for resource linking.
    /// 
    /// Required: No
    ///
    /// Type: TableIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTable")]
    pub target_table: Option<TableIdentifier>,


    /// 
    /// These key-value pairs define properties associated with the table.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// The table name. For Hive compatibility, this is folded to    lowercase when it is stored.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The retention time for this table.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Retention")]
    pub retention: Option<i64>,


    /// 
    /// The type of this table.    AWS Glue will create tables with the EXTERNAL_TABLE type.    Other services, such as Athena, may create tables with additional table types.
    /// 
    /// AWS Glue related table types:
    /// 
    /// EXTERNAL_TABLE                  Hive compatible attribute - indicates a non-Hive managed table.                       GOVERNED                  Used by AWS Lake Formation.       The AWS Glue Data Catalog understands GOVERNED.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableType")]
    pub table_type: Option<String>,


    /// 
    /// A list of columns by which the table is partitioned. Only primitive    types are supported as partition keys.
    /// 
    /// When you create a table used by Amazon Athena, and you do not specify any     partitionKeys, you must at least set the value of partitionKeys to    an empty list. For example:
    /// 
    /// "PartitionKeys": []
    /// 
    /// Required: No
    ///
    /// Type: List of Column
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionKeys")]
    pub partition_keys: Option<Vec<Column>>,


    /// 
    /// The table owner. Included for Apache Hive compatibility. Not used in the normal course of AWS Glue operations.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Owner")]
    pub owner: Option<String>,


    /// 
    /// A storage descriptor containing information about the physical storage    of this table.
    /// 
    /// Required: No
    ///
    /// Type: StorageDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageDescriptor")]
    pub storage_descriptor: Option<StorageDescriptor>,

}


/// Specifies skewed values in a table. Skewed values are those that occur with very high    frequency.
#[derive(Default, serde::Serialize)]
pub struct SkewedInfo {


    /// 
    /// A mapping of skewed values to the columns that contain them.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    pub skewed_column_value_location_maps: Option<serde_json::Value>,


    /// 
    /// A list of names of columns that contain skewed values.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnNames")]
    pub skewed_column_names: Option<Vec<String>>,


    /// 
    /// A list of values that appear so frequently as to be considered    skewed.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnValues")]
    pub skewed_column_values: Option<Vec<String>>,

}


/// A structure that describes a target table for resource linking.
#[derive(Default, serde::Serialize)]
pub struct TableIdentifier {


    /// 
    /// The name of the catalog database that contains the target table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// The name of the target table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The ID of the Data Catalog in which the table resides.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}


/// Information about a serialization/deserialization program (SerDe) that serves as an    extractor and loader.
#[derive(Default, serde::Serialize)]
pub struct SerdeInfo {


    /// 
    /// Name of the SerDe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Usually the class that implements the SerDe. An example is     org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SerializationLibrary")]
    pub serialization_library: Option<String>,


    /// 
    /// These key-value pairs define initialization parameters for the SerDe.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,

}


/// A column in a Table.
#[derive(Default, serde::Serialize)]
pub struct Column {


    /// 
    /// A free-form text comment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Comment")]
    pub comment: Option<String>,


    /// 
    /// The name of the Column.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The data type of the Column.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 131072
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


/// A structure that contains schema identity fields. Either this or the SchemaVersionId has to be provided.
#[derive(Default, serde::Serialize)]
pub struct SchemaId {


    /// 
    /// The name of the schema. One of SchemaArn or SchemaName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,


    /// 
    /// The name of the schema registry that contains the schema.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the schema. One of SchemaArn or SchemaName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,

}


/// Specifies the sort order of a sorted column.
#[derive(Default, serde::Serialize)]
pub struct Order {


    /// 
    /// The name of the column.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Column")]
    pub column: String,


    /// 
    /// Indicates that the column is sorted in ascending order    (== 1), or in descending order (==0).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

}


/// An object that references a schema stored in the AWS Glue Schema Registry.
#[derive(Default, serde::Serialize)]
pub struct SchemaReference {


    /// 
    /// A structure that contains schema identity fields. Either this or the SchemaVersionId has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: SchemaId
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaId")]
    pub schema_id: Option<SchemaId>,


    /// 
    /// The version number of the schema.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersionNumber")]
    pub schema_version_number: Option<i64>,


    /// 
    /// The unique ID assigned to a version of the schema. Either this or the SchemaId has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: Option<String>,

}