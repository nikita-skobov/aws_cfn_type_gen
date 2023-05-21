/// Specifies a new DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataset {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DatasetFormatEnum>,

    ///
    /// A set of options that define how DataBrew interprets the data in the       dataset.
    ///
    /// Required: No
    ///
    /// Type: FormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "FormatOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_options: Option<FormatOptions>,

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
    pub name: cfn_resources::StrVal,

    ///
    /// A set of options that defines how DataBrew interprets an Amazon S3 path of the dataset.
    ///
    /// Required: No
    ///
    /// Type: PathOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_options: Option<PathOptions>,

    ///
    /// Metadata tags that have been applied to the dataset.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DatasetFormatEnum {
    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// EXCEL
    #[serde(rename = "EXCEL")]
    Excel,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// ORC
    #[serde(rename = "ORC")]
    Orc,

    /// PARQUET
    #[serde(rename = "PARQUET")]
    Parquet,
}

impl Default for DatasetFormatEnum {
    fn default() -> Self {
        DatasetFormatEnum::Csv
    }
}

impl cfn_resources::CfnResource for CfnDataset {
    fn type_string(&self) -> &'static str {
        "AWS::DataBrew::Dataset"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.format_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.input.validate()?;

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

        self.path_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<cfn_resources::StrVal>,

    ///
    /// A variable that specifies whether the first row in the file is parsed as the       header. If this value is false, column names are auto-generated.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,
}

impl cfn_resources::CfnResource for CsvOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.delimiter {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1 as _ {
                    return Err(format!(
                        "Max validation failed on field 'delimiter'. {} is greater than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.delimiter {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'delimiter'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<cfn_resources::StrVal>,

    ///
    /// An Amazon location that AWS Glue Data Catalog can use as a temporary       directory.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<S3Location>,
}

impl cfn_resources::CfnResource for DataCatalogInputDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.catalog_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'catalog_id'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.catalog_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'catalog_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.database_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'database_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.database_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'database_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.table_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'table_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.table_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'table_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.temp_directory
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Connection information for dataset input files stored in a database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseInputDefinition {
    ///
    /// The table within the target database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseTableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_table_name: Option<cfn_resources::StrVal>,

    ///
    /// The AWS Glue Connection that stores the connection information for       the target database.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueConnectionName")]
    pub glue_connection_name: cfn_resources::StrVal,

    ///
    /// Custom SQL to run against the provided AWS Glue connection. This SQL will be used as       the input for DataBrew projects and jobs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<cfn_resources::StrVal>,

    ///
    /// An Amazon location that AWS Glue Data Catalog can use as a temporary       directory.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_directory: Option<S3Location>,
}

impl cfn_resources::CfnResource for DatabaseInputDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.temp_directory
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents a dataset paramater that defines type and conditions for a parameter in the         Amazon S3 path of the dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetParameter {
    ///
    /// Optional boolean value that defines whether the captured value of this parameter       should be loaded as an additional column in the dataset.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_options: Option<DatetimeOptions>,

    ///
    /// The optional filter expression structure to apply additional matching criteria to the       parameter.
    ///
    /// Required: No
    ///
    /// Type: FilterExpression
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterExpression>,

    ///
    /// The name of the parameter that is used in the dataset's Amazon S3       path.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The type of the dataset parameter, can be one of a 'String', 'Number' or       'Datetime'.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DatasetParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.datetime_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.filter.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents additional options for correct interpretation of datetime parameters used       in the Amazon S3 path of a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatetimeOptions {
    ///
    /// Required option, that defines the datetime format used for a date parameter in the         Amazon S3 path. Should use only supported datetime specifiers and       separation characters, all litera a-z or A-Z character should be escaped with single       quotes. E.g. "MM.dd.yyyy-'at'-HH:mm".
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: cfn_resources::StrVal,

    ///
    /// Optional value for a non-US locale code, needed for correct interpretation of some       date formats.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<cfn_resources::StrVal>,

    ///
    /// Optional value for a timezone offset of the datetime parameter value in the Amazon S3 path. Shouldn't be used if Format for this parameter includes timezone       fields. If no offset specified, UTC is assumed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimezoneOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DatetimeOptions {
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

/// Represents a set of options that define how DataBrew will interpret a Microsoft Excel file when       creating a dataset from that file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExcelOptions {
    ///
    /// A variable that specifies whether the first row in the file is parsed as the       header. If this value is false, column names are auto-generated.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_indexes: Option<Vec<i64>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_names: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for ExcelOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sheet_indexes {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'sheet_indexes'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sheet_names {
            if the_val.len() > 1 as _ {
                return Err(format!(
                    "Max validation failed on field 'sheet_names'. {} is greater than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
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
    /// A criteria to use for Amazon S3 files sorting before their selection. By       default uses DESCENDING order, i.e. most recent files are selected first.       Anotherpossible value is ASCENDING.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<cfn_resources::StrVal>,

    ///
    /// A criteria to use for Amazon S3 files sorting before their selection. By       default uses LAST_MODIFIED_DATE as a sorting criteria. Currently it's the only allowed       value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered_by: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FilesLimit {
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

/// Represents a structure for defining parameter conditions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterExpression {
    ///
    /// The expression which includes condition names followed by substitution variables,       possibly grouped and combined with other conditions. For example, "(starts_with :prefix1       or starts_with :prefix2) and (ends_with :suffix1 or ends_with :suffix2)". Substitution       variables should start with ':' symbol.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for FilterExpression {
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
    pub value: cfn_resources::StrVal,

    ///
    /// The substitution variable reference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueReference")]
    pub value_reference: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for FilterValue {
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

/// Represents a set of options that define the structure of either comma-separated value (CSV),       Excel, or JSON input.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FormatOptions {
    ///
    /// Options that define how CSV input is to be interpreted by DataBrew.
    ///
    /// Required: No
    ///
    /// Type: CsvOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CsvOptions>,

    ///
    /// Options that define how Excel input is to be interpreted by DataBrew.
    ///
    /// Required: No
    ///
    /// Type: ExcelOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Excel")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonOptions>,
}

impl cfn_resources::CfnResource for FormatOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csv.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.excel.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.json.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents information on how DataBrew can find data, in either the AWS Glue Data Catalog or       Amazon S3.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {
    ///
    /// The AWS Glue Data Catalog parameters for the data.
    ///
    /// Required: No
    ///
    /// Type: DataCatalogInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogInputDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,

    ///
    /// Connection information for dataset input files stored in a database.
    ///
    /// Required: No
    ///
    /// Type: DatabaseInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseInputDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_input_definition: Option<DatabaseInputDefinition>,

    ///
    /// Contains additional resource information needed for specific datasets.
    ///
    /// Required: No
    ///
    /// Type: Metadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    ///
    /// The Amazon S3 location where the data is stored.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_definition: Option<S3Location>,
}

impl cfn_resources::CfnResource for Input {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_catalog_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.database_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.metadata
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_line: Option<bool>,
}

impl cfn_resources::CfnResource for JsonOptions {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Metadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.source_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'source_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.source_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'source_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Represents a set of options that define how DataBrew selects files for a       given Amazon S3 path in a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathOptions {
    ///
    /// If provided, this structure imposes a limit on a number of files that should be       selected.
    ///
    /// Required: No
    ///
    /// Type: FilesLimit
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilesLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_limit: Option<FilesLimit>,

    ///
    /// If provided, this structure defines a date range for matching Amazon S3       objects based on their LastModifiedDate attribute in Amazon S3.
    ///
    /// Required: No
    ///
    /// Type: FilterExpression
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedDateCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date_condition: Option<FilterExpression>,

    ///
    /// A structure that maps names of parameters used in the Amazon S3 path of a       dataset to their definitions.
    ///
    /// Required: No
    ///
    /// Type: List of PathParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<PathParameter>>,
}

impl cfn_resources::CfnResource for PathOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.files_limit
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.last_modified_date_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents a single entry in the path parameters of a dataset. Each         PathParameter consists of a name and a parameter definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathParameter {
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

    ///
    /// The name of the path parameter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathParameterName")]
    pub path_parameter_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PathParameter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dataset_parameter.validate()?;

        Ok(())
    }
}

/// Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read       input data, or write output from a job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {
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
    pub bucket: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Location {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'bucket'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.bucket;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'bucket'. {} is less than 3",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1280 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 1280",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
