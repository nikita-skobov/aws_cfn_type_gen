/// The AWS::Glue::Partition resource creates an AWS Glue partition, which       represents a slice of table data. For more information, see CreatePartition Action and Partition Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPartition {
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
    pub catalog_id: cfn_resources::StrVal,

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
    pub database_name: cfn_resources::StrVal,

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
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnPartition {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Partition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.catalog_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'catalog_id'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.catalog_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'catalog_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.database_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'database_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.database_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'database_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.partition_input.validate()?;

        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'table_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.table_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'table_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A column in a Table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<cfn_resources::StrVal>,

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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Column {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.comment {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'comment'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.comment {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'comment'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.cfn_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 131072 as _ {
                    return Err(format!(
                        "Max validation failed on field 'cfn_type'. {} is greater than 131072",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.cfn_type {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'cfn_type'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies the sort order of a sorted column.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub column: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl cfn_resources::CfnResource for Order {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.column;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'column'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.column;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'column'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.sort_order {
            if *the_val > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'sort_order'. {} is greater than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.sort_order {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'sort_order'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The structure used to create and update a partition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PartitionInput {
    ///
    /// These key-value pairs define partition parameters.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_descriptor: Option<StorageDescriptor>,

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
}

impl cfn_resources::CfnResource for PartitionInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.storage_descriptor
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that contains schema identity fields. Either this or the SchemaVersionId has to be provided.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SchemaId {
    ///
    /// The name of the schema registry that contains the schema.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the schema. One of SchemaArn or SchemaName has to be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the schema. One of SchemaArn or SchemaName has to be provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SchemaId {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An object that references a schema stored in the AWS Glue Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_id: Option<cfn_resources::StrVal>,

    ///
    /// The version number of the schema.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version_number: Option<i64>,
}

impl cfn_resources::CfnResource for SchemaReference {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.schema_id
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information about a serialization/deserialization program (SerDe) that serves as an    extractor and loader.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// These key-value pairs define initialization parameters for the SerDe.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_library: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SerdeInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies skewed values in a table. Skewed values are those that occur with very high    frequency.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SkewedInfo {
    ///
    /// A list of names of columns that contain skewed values.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_names: Option<Vec<String>>,

    ///
    /// A mapping of skewed values to the columns that contain them.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_value_location_maps: Option<serde_json::Value>,

    ///
    /// A list of values that appear so frequently as to be considered    skewed.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedColumnValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skewed_column_values: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for SkewedInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes the physical storage of table data.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StorageDescriptor {
    ///
    /// A list of reducer grouping columns, clustering columns, and    bucketing columns in the table.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_columns: Option<Vec<String>>,

    ///
    /// A list of the Columns in the table.
    ///
    /// Required: No
    ///
    /// Type: List of Column
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,

    ///
    /// True if the data in the table is compressed, or False if    not.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compressed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_buckets: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<cfn_resources::StrVal>,

    ///
    /// The user-supplied properties in key-value form.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    ///
    /// An object that references a schema stored in the AWS Glue Schema Registry.
    ///
    /// Required: No
    ///
    /// Type: SchemaReference
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchemaReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_reference: Option<SchemaReference>,

    ///
    /// The serialization/deserialization (SerDe) information.
    ///
    /// Required: No
    ///
    /// Type: SerdeInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "SerdeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde_info: Option<SerdeInfo>,

    ///
    /// The information about values that appear frequently in a column (skewed values).
    ///
    /// Required: No
    ///
    /// Type: SkewedInfo
    ///
    /// Update requires: No interruption
    #[serde(rename = "SkewedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_columns: Option<Vec<Order>>,

    ///
    /// True if the table data is stored in subdirectories, or False if    not.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "StoredAsSubDirectories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_as_sub_directories: Option<bool>,
}

impl cfn_resources::CfnResource for StorageDescriptor {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.input_format {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'input_format'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.location {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2056 as _ {
                    return Err(format!(
                        "Max validation failed on field 'location'. {} is greater than 2056",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.output_format {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'output_format'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        self.schema_reference
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.serde_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.skewed_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
