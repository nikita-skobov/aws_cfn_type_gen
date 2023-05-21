

/// The AWS::Glue::Partition resource creates an AWS Glue partition, which       represents a slice of table data. For more information, see CreatePartition Action and Partition Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPartition {


    /// 
    /// The name of the catalog database in which to create the partition.
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
    /// The AWS account ID of the catalog in which the partion is to be created.
    /// 
    /// NoteTo specify the account ID, you can use the Ref intrinsic function         with the AWS::AccountId pseudo parameter. For example: !Ref           AWS::AccountId
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


    /// 
    /// The name of the metadata table in which the partition is to be created.
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
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The structure used to create and update a partition.
    /// 
    /// Required: Yes
    ///
    /// Type: PartitionInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,

}



impl cfn_resources::CfnResource for CfnPartition {
    fn type_string() -> &'static str {
        "AWS::Glue::Partition"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The structure used to create and update a partition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PartitionInput {


    /// 
    /// The values of the partition. Although this parameter is not required by the SDK, you must specify this parameter for a valid input.
    /// 
    /// The values for the keys for the new partition must be passed as an array of String objects that must be ordered in the same order as the partition keys appearing in the Amazon S3 prefix. Otherwise AWS Glue will add the values to the wrong keys.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Values")]
    pub values: Vec<String>,


    /// 
    /// These key-value pairs define partition parameters.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// Provides information about the physical    location where the partition is stored.
    /// 
    /// Required: No
    ///
    /// Type: StorageDescriptor
    ///
    /// Update requires: No interruption
    #[serde(rename = "StorageDescriptor")]
    pub storage_descriptor: Option<StorageDescriptor>,

}




/// An object that references a schema stored in the AWS Glue Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SchemaReference {


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




/// A structure that contains schema identity fields. Either this or the SchemaVersionId has to be provided.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// The Amazon Resource Name (ARN) of the schema. One of SchemaArn or SchemaName has to be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,


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

}




/// Describes the physical storage of table data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StorageDescriptor {


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
    /// The number of buckets.
    /// 
    /// You must specify this property if the partition contains any dimension columns.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfBuckets")]
    pub number_of_buckets: Option<i64>,


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
    /// True if the data in the table is compressed, or False if    not.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,

}




/// Information about a serialization/deserialization program (SerDe) that serves as an    extractor and loader.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SerdeInfo {


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
    /// Name of the SerDe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}




/// A column in a Table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Column {


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




/// Specifies skewed values in a table. Skewed values are those that occur with very high    frequency.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SkewedInfo {


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

}




/// Specifies the sort order of a sorted column.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<i64>,

}


