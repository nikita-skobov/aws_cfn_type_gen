/// Creates a new Q topic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTopic {
    ///
    /// The ID of the AWS account that you want to create a topic in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Pattern: ^[0-9]{12}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,

    ///
    /// The data sets that the topic is associated with.
    ///
    /// Required: No
    ///
    /// Type: List of DatasetMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<DatasetMetadata>>,

    ///
    /// The description of the topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    /// The name of the topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    ///
    /// The ID for the topic. This ID is unique per AWS Region for each AWS account.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[A-Za-z0-9-_.\\+]*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TopicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

impl cfn_resources::CfnResource for CfnTopic {
    fn type_string(&self) -> &'static str {
        "AWS::QuickSight::Topic"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'aws_account_id'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.aws_account_id {
            if the_val.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'aws_account_id'. {} is less than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.topic_id {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'topic_id'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents the cell value synonym.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CellValueSynonym {
    ///
    /// The cell value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value: Option<String>,

    ///
    /// Other names or aliases for the cell value.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Synonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CellValueSynonym {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.cell_value {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'cell_value'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a collective constant.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CollectiveConstant {
    ///
    /// A list of values for the collective constant.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CollectiveConstant {
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

/// The order in which data is displayed for the column when     it's used in a comparative context.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComparativeOrder {
    ///
    /// The list of columns to be used in the ordering.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpecifedOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specifed_order: Option<Vec<String>>,

    ///
    /// The treat of undefined specified values. Valid values for this structure are LEAST and MOST.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LEAST | MOST
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatUndefinedSpecifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_undefined_specified_values: Option<ComparativeOrderTreatUndefinedSpecifiedValuesEnum>,

    ///
    /// The ordering type for a column. Valid values for this structure are GREATER_IS_BETTER, LESSER_IS_BETTER and SPECIFIED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GREATER_IS_BETTER | LESSER_IS_BETTER | SPECIFIED
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseOrdering")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ordering: Option<ComparativeOrderUseOrderingEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComparativeOrderTreatUndefinedSpecifiedValuesEnum {
    /// LEAST
    #[serde(rename = "LEAST")]
    Least,

    /// MOST
    #[serde(rename = "MOST")]
    Most,
}

impl Default for ComparativeOrderTreatUndefinedSpecifiedValuesEnum {
    fn default() -> Self {
        ComparativeOrderTreatUndefinedSpecifiedValuesEnum::Least
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ComparativeOrderUseOrderingEnum {
    /// GREATER_IS_BETTER
    #[serde(rename = "GREATER_IS_BETTER")]
    Greaterisbetter,

    /// LESSER_IS_BETTER
    #[serde(rename = "LESSER_IS_BETTER")]
    Lesserisbetter,

    /// SPECIFIED
    #[serde(rename = "SPECIFIED")]
    Specified,
}

impl Default for ComparativeOrderUseOrderingEnum {
    fn default() -> Self {
        ComparativeOrderUseOrderingEnum::Greaterisbetter
    }
}

impl cfn_resources::CfnResource for ComparativeOrder {
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

/// The definition of a data aggregation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataAggregation {
    ///
    /// The level of time precision that is used to aggregate DateTime values.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetRowDateGranularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_row_date_granularity: Option<DataAggregationDatasetRowDateGranularityEnum>,

    ///
    /// The column name for the default date.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultDateColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_date_column_name: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DataAggregationDatasetRowDateGranularityEnum {
    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// HOUR
    #[serde(rename = "HOUR")]
    Hour,

    /// MINUTE
    #[serde(rename = "MINUTE")]
    Minute,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// QUARTER
    #[serde(rename = "QUARTER")]
    Quarter,

    /// SECOND
    #[serde(rename = "SECOND")]
    Second,

    /// WEEK
    #[serde(rename = "WEEK")]
    Week,

    /// YEAR
    #[serde(rename = "YEAR")]
    Year,
}

impl Default for DataAggregationDatasetRowDateGranularityEnum {
    fn default() -> Self {
        DataAggregationDatasetRowDateGranularityEnum::Day
    }
}

impl cfn_resources::CfnResource for DataAggregation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.default_date_column_name {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'default_date_column_name'. {} is greater than 256", the_val.len()));
            }
        }

        Ok(())
    }
}

/// A structure that represents a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetMetadata {
    ///
    /// The list of calculated field definitions.
    ///
    /// Required: No
    ///
    /// Type: List of TopicCalculatedField
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_fields: Option<Vec<TopicCalculatedField>>,

    ///
    /// The list of column definitions.
    ///
    /// Required: No
    ///
    /// Type: List of TopicColumn
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TopicColumn>>,

    ///
    /// The definition of a data aggregation.
    ///
    /// Required: No
    ///
    /// Type: DataAggregation
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataAggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_aggregation: Option<DataAggregation>,

    ///
    /// The Amazon Resource Name (ARN) of the dataset.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: String,

    ///
    /// The description of the dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_description: Option<String>,

    ///
    /// The name of the dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,

    ///
    /// The list of filter definitions.
    ///
    /// Required: No
    ///
    /// Type: List of TopicFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TopicFilter>>,

    ///
    /// The list of named entities definitions.
    ///
    /// Required: No
    ///
    /// Type: List of TopicNamedEntity
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamedEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_entities: Option<Vec<TopicNamedEntity>>,
}

impl cfn_resources::CfnResource for DatasetMetadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_aggregation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.dataset_description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'dataset_description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.dataset_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'dataset_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a default formatting definition.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultFormatting {
    ///
    /// The display format. Valid values for this structure are AUTO,       PERCENT, CURRENCY, NUMBER, DATE, and       STRING.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | CURRENCY | DATE | NUMBER | PERCENT | STRING
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format: Option<DefaultFormattingDisplayFormatEnum>,

    ///
    /// The additional options for display formatting.
    ///
    /// Required: No
    ///
    /// Type: DisplayFormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayFormatOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format_options: Option<DisplayFormatOptions>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DefaultFormattingDisplayFormatEnum {
    /// AUTO
    #[serde(rename = "AUTO")]
    Auto,

    /// CURRENCY
    #[serde(rename = "CURRENCY")]
    Currency,

    /// DATE
    #[serde(rename = "DATE")]
    Date,

    /// NUMBER
    #[serde(rename = "NUMBER")]
    Number,

    /// PERCENT
    #[serde(rename = "PERCENT")]
    Percent,

    /// STRING
    #[serde(rename = "STRING")]
    String,
}

impl Default for DefaultFormattingDisplayFormatEnum {
    fn default() -> Self {
        DefaultFormattingDisplayFormatEnum::Auto
    }
}

impl cfn_resources::CfnResource for DefaultFormatting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.display_format_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents additional options for display formatting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DisplayFormatOptions {
    ///
    /// Determines the blank cell format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlankCellFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blank_cell_format: Option<String>,

    ///
    /// The currency symbol, such as USD.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "CurrencySymbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,

    ///
    /// Determines the DateTime format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,

    ///
    /// Determines the decimal separator.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COMMA | DOT
    ///
    /// Update requires: No interruption
    #[serde(rename = "DecimalSeparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_separator: Option<DisplayFormatOptionsDecimalSeparatorEnum>,

    ///
    /// Determines the number of fraction digits.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FractionDigits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraction_digits: Option<f64>,

    ///
    /// Determines the grouping separator.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupingSeparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_separator: Option<String>,

    ///
    /// The negative format.
    ///
    /// Required: No
    ///
    /// Type: NegativeFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_format: Option<NegativeFormat>,

    ///
    /// The prefix value for a display format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    ///
    /// The suffix value for a display format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    ///
    /// The unit scaler. Valid values for this structure are: NONE,       AUTO, THOUSANDS, MILLIONS,     BILLIONS,     and TRILLIONS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTO | BILLIONS | MILLIONS | NONE | THOUSANDS | TRILLIONS
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnitScaler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_scaler: Option<DisplayFormatOptionsUnitScalerEnum>,

    ///
    /// A Boolean value that indicates whether to use blank cell format.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBlankCellFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blank_cell_format: Option<bool>,

    ///
    /// A Boolean value that indicates whether to use grouping.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseGrouping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_grouping: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DisplayFormatOptionsDecimalSeparatorEnum {
    /// COMMA
    #[serde(rename = "COMMA")]
    Comma,

    /// DOT
    #[serde(rename = "DOT")]
    Dot,
}

impl Default for DisplayFormatOptionsDecimalSeparatorEnum {
    fn default() -> Self {
        DisplayFormatOptionsDecimalSeparatorEnum::Comma
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DisplayFormatOptionsUnitScalerEnum {
    /// AUTO
    #[serde(rename = "AUTO")]
    Auto,

    /// BILLIONS
    #[serde(rename = "BILLIONS")]
    Billions,

    /// MILLIONS
    #[serde(rename = "MILLIONS")]
    Millions,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// THOUSANDS
    #[serde(rename = "THOUSANDS")]
    Thousands,

    /// TRILLIONS
    #[serde(rename = "TRILLIONS")]
    Trillions,
}

impl Default for DisplayFormatOptionsUnitScalerEnum {
    fn default() -> Self {
        DisplayFormatOptionsUnitScalerEnum::Auto
    }
}

impl cfn_resources::CfnResource for DisplayFormatOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.blank_cell_format {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'blank_cell_format'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.currency_symbol {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'currency_symbol'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.date_format {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'date_format'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.grouping_separator {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'grouping_separator'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        self.negative_format
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.prefix {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'prefix'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.suffix {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'suffix'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a named entity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NamedEntityDefinition {
    ///
    /// The name of the entity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,

    ///
    /// The definition of a metric.
    ///
    /// Required: No
    ///
    /// Type: NamedEntityDefinitionMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<NamedEntityDefinitionMetric>,

    ///
    /// The property name to be used for the named entity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,

    ///
    /// The property role. Valid values for this structure are PRIMARY and ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ID | PRIMARY
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_role: Option<NamedEntityDefinitionPropertyRoleEnum>,

    ///
    /// The property usage. Valid values for this structure are INHERIT,       DIMENSION,     and MEASURE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIMENSION | INHERIT | MEASURE
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_usage: Option<NamedEntityDefinitionPropertyUsageEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NamedEntityDefinitionPropertyRoleEnum {
    /// ID
    #[serde(rename = "ID")]
    Id,

    /// PRIMARY
    #[serde(rename = "PRIMARY")]
    Primary,
}

impl Default for NamedEntityDefinitionPropertyRoleEnum {
    fn default() -> Self {
        NamedEntityDefinitionPropertyRoleEnum::Id
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NamedEntityDefinitionPropertyUsageEnum {
    /// DIMENSION
    #[serde(rename = "DIMENSION")]
    Dimension,

    /// INHERIT
    #[serde(rename = "INHERIT")]
    Inherit,

    /// MEASURE
    #[serde(rename = "MEASURE")]
    Measure,
}

impl Default for NamedEntityDefinitionPropertyUsageEnum {
    fn default() -> Self {
        NamedEntityDefinitionPropertyUsageEnum::Dimension
    }
}

impl cfn_resources::CfnResource for NamedEntityDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.field_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'field_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        self.metric.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.property_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'property_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a metric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NamedEntityDefinitionMetric {
    ///
    /// The aggregation of a named entity. Valid values for this structure are SUM,       MIN, MAX, COUNT, AVERAGE,       DISTINCT_COUNT, STDEV, STDEVP, VAR,       VARP, PERCENTILE,     MEDIAN,     and CUSTOM.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | CUSTOM | DISTINCT_COUNT | MAX | MEDIAN | MIN | PERCENTILE | STDEV | STDEVP | SUM | VAR | VARP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<NamedEntityDefinitionMetricAggregationEnum>,

    ///
    /// The additional parameters for an aggregation function.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunctionParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum NamedEntityDefinitionMetricAggregationEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// DISTINCT_COUNT
    #[serde(rename = "DISTINCT_COUNT")]
    Distinctcount,

    /// MAX
    #[serde(rename = "MAX")]
    Max,

    /// MEDIAN
    #[serde(rename = "MEDIAN")]
    Median,

    /// MIN
    #[serde(rename = "MIN")]
    Min,

    /// PERCENTILE
    #[serde(rename = "PERCENTILE")]
    Percentile,

    /// STDEV
    #[serde(rename = "STDEV")]
    Stdev,

    /// STDEVP
    #[serde(rename = "STDEVP")]
    Stdevp,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,

    /// VAR
    #[serde(rename = "VAR")]
    Var,

    /// VARP
    #[serde(rename = "VARP")]
    Varp,
}

impl Default for NamedEntityDefinitionMetricAggregationEnum {
    fn default() -> Self {
        NamedEntityDefinitionMetricAggregationEnum::Average
    }
}

impl cfn_resources::CfnResource for NamedEntityDefinitionMetric {
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

/// A structure that represents a negative format.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NegativeFormat {
    ///
    /// The prefix for a negative format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    ///
    /// The suffix for a negative format.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl cfn_resources::CfnResource for NegativeFormat {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.prefix {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'prefix'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.suffix {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'suffix'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The value of the constant that is used to specify the endpoints of a range filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RangeConstant {
    ///
    /// The maximum value for a range constant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,

    ///
    /// The minimum value for a range constant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "Minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
}

impl cfn_resources::CfnResource for RangeConstant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.maximum {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'maximum'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.minimum {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'minimum'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a semantic entity type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SemanticEntityType {
    ///
    /// The semantic entity sub type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<String>,

    ///
    /// The semantic entity type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    ///
    /// The semantic entity type parameters.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for SemanticEntityType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sub_type_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'sub_type_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a semantic type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SemanticType {
    ///
    /// The semantic type falsey cell value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseyCellValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falsey_cell_value: Option<String>,

    ///
    /// The other names or aliases for the false cell value.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseyCellValueSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falsey_cell_value_synonyms: Option<Vec<String>>,

    ///
    /// The semantic type sub type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<String>,

    ///
    /// The semantic type truthy cell value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruthyCellValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truthy_cell_value: Option<String>,

    ///
    /// The other names or aliases for the true cell value.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruthyCellValueSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truthy_cell_value_synonyms: Option<Vec<String>>,

    ///
    /// The semantic type name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    ///
    /// The semantic type parameters.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for SemanticType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sub_type_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'sub_type_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that represents a calculated field.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicCalculatedField {
    ///
    /// The default aggregation. Valid values for this structure are SUM,       MAX, MIN, COUNT,     DISTINCT_COUNT,     and AVERAGE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | DISTINCT_COUNT | MAX | MIN | SUM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<TopicCalculatedFieldAggregationEnum>,

    ///
    /// The list of aggregation types that are allowed for the calculated field. Valid values     for this structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP, and PERCENTILE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_aggregations: Option<Vec<String>>,

    ///
    /// The calculated field description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFieldDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_field_description: Option<String>,

    ///
    /// The calculated field name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFieldName")]
    pub calculated_field_name: String,

    ///
    /// The other names or aliases for the calculated field.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFieldSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_field_synonyms: Option<Vec<String>>,

    ///
    /// The other     names or aliases for the calculated field cell value.
    ///
    /// Required: No
    ///
    /// Type: List of CellValueSynonym
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellValueSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,

    ///
    /// The column data role for a calculated field. Valid values for this structure are DIMENSION and MEASURE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIMENSION | MEASURE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDataRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_data_role: Option<TopicCalculatedFieldColumnDataRoleEnum>,

    ///
    /// The order in which data is displayed for the calculated field when     it's used in a comparative context.
    ///
    /// Required: No
    ///
    /// Type: ComparativeOrder
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparativeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparative_order: Option<ComparativeOrder>,

    ///
    /// The default formatting definition.
    ///
    /// Required: No
    ///
    /// Type: DefaultFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultFormatting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_formatting: Option<DefaultFormatting>,

    ///
    /// The calculated field expression.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: String,

    ///
    /// A boolean value that indicates if a calculated field is included in the topic.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsIncludedInTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_included_in_topic: Option<bool>,

    ///
    /// A Boolean value that indicates whether to never aggregate calculated field in filters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NeverAggregateInFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_aggregate_in_filter: Option<bool>,

    ///
    /// The list of aggregation types that are not allowed for the calculated field. Valid     values for this structure are COUNT, DISTINCT_COUNT,       MIN, MAX, MEDIAN, SUM,       AVERAGE, STDEV, STDEVP, VAR,       VARP, and PERCENTILE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotAllowedAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_aggregations: Option<Vec<String>>,

    ///
    /// The semantic type.
    ///
    /// Required: No
    ///
    /// Type: SemanticType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<SemanticType>,

    ///
    /// The level of time precision that is used to aggregate DateTime values.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<TopicCalculatedFieldTimeGranularityEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCalculatedFieldAggregationEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// DISTINCT_COUNT
    #[serde(rename = "DISTINCT_COUNT")]
    Distinctcount,

    /// MAX
    #[serde(rename = "MAX")]
    Max,

    /// MIN
    #[serde(rename = "MIN")]
    Min,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,
}

impl Default for TopicCalculatedFieldAggregationEnum {
    fn default() -> Self {
        TopicCalculatedFieldAggregationEnum::Average
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCalculatedFieldColumnDataRoleEnum {
    /// DIMENSION
    #[serde(rename = "DIMENSION")]
    Dimension,

    /// MEASURE
    #[serde(rename = "MEASURE")]
    Measure,
}

impl Default for TopicCalculatedFieldColumnDataRoleEnum {
    fn default() -> Self {
        TopicCalculatedFieldColumnDataRoleEnum::Dimension
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCalculatedFieldTimeGranularityEnum {
    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// HOUR
    #[serde(rename = "HOUR")]
    Hour,

    /// MINUTE
    #[serde(rename = "MINUTE")]
    Minute,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// QUARTER
    #[serde(rename = "QUARTER")]
    Quarter,

    /// SECOND
    #[serde(rename = "SECOND")]
    Second,

    /// WEEK
    #[serde(rename = "WEEK")]
    Week,

    /// YEAR
    #[serde(rename = "YEAR")]
    Year,
}

impl Default for TopicCalculatedFieldTimeGranularityEnum {
    fn default() -> Self {
        TopicCalculatedFieldTimeGranularityEnum::Day
    }
}

impl cfn_resources::CfnResource for TopicCalculatedField {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.calculated_field_description {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'calculated_field_description'. {} is greater than 256", the_val.len()));
            }
        }

        let the_val = &self.calculated_field_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'calculated_field_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        self.comparative_order
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_formatting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.expression;

        if the_val.len() > 4096 as _ {
            return Err(format!(
                "Max validation failed on field 'expression'. {} is greater than 4096",
                the_val.len()
            ));
        }

        let the_val = &self.expression;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'expression'. {} is less than 1",
                the_val.len()
            ));
        }

        self.semantic_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents a category filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicCategoryFilter {
    ///
    /// The category filter function. Valid values for this structure are EXACT and CONTAINS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | EXACT
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryFilterFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter_function: Option<TopicCategoryFilterCategoryFilterFunctionEnum>,

    ///
    /// The category filter type. This element is used to specify whether a filter is a simple category filter or an inverse category filter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM_FILTER | CUSTOM_FILTER_LIST | FILTER_LIST
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryFilterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter_type: Option<TopicCategoryFilterCategoryFilterTypeEnum>,

    ///
    /// The constant used in a category filter.
    ///
    /// Required: No
    ///
    /// Type: TopicCategoryFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicCategoryFilterConstant>,

    ///
    /// A Boolean value that indicates if the filter is inverse.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCategoryFilterCategoryFilterFunctionEnum {
    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// EXACT
    #[serde(rename = "EXACT")]
    Exact,
}

impl Default for TopicCategoryFilterCategoryFilterFunctionEnum {
    fn default() -> Self {
        TopicCategoryFilterCategoryFilterFunctionEnum::Contains
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCategoryFilterCategoryFilterTypeEnum {
    /// CUSTOM_FILTER
    #[serde(rename = "CUSTOM_FILTER")]
    Customfilter,

    /// CUSTOM_FILTER_LIST
    #[serde(rename = "CUSTOM_FILTER_LIST")]
    Customfilterlist,

    /// FILTER_LIST
    #[serde(rename = "FILTER_LIST")]
    Filterlist,
}

impl Default for TopicCategoryFilterCategoryFilterTypeEnum {
    fn default() -> Self {
        TopicCategoryFilterCategoryFilterTypeEnum::Customfilter
    }
}

impl cfn_resources::CfnResource for TopicCategoryFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A constant used in a category filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicCategoryFilterConstant {
    ///
    /// A collective constant used in a category filter. This element is used to specify a list of values for the constant.
    ///
    /// Required: No
    ///
    /// Type: CollectiveConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollectiveConstant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collective_constant: Option<CollectiveConstant>,

    ///
    /// The type of category filter constant. This element is used to specify whether a constant is a singular or collective. Valid values are SINGULAR and COLLECTIVE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COLLECTIVE | RANGE | SINGULAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<TopicCategoryFilterConstantConstantTypeEnum>,

    ///
    /// A singular constant used in a category filter. This element is used to specify a single value for the constant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingularConstant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular_constant: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicCategoryFilterConstantConstantTypeEnum {
    /// COLLECTIVE
    #[serde(rename = "COLLECTIVE")]
    Collective,

    /// RANGE
    #[serde(rename = "RANGE")]
    Range,

    /// SINGULAR
    #[serde(rename = "SINGULAR")]
    Singular,
}

impl Default for TopicCategoryFilterConstantConstantTypeEnum {
    fn default() -> Self {
        TopicCategoryFilterConstantConstantTypeEnum::Collective
    }
}

impl cfn_resources::CfnResource for TopicCategoryFilterConstant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.collective_constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.singular_constant {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'singular_constant'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents a column in a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicColumn {
    ///
    /// The type of aggregation that is performed on the column data when     it's queried. Valid values for this structure are SUM, MAX,       MIN, COUNT,     DISTINCT_COUNT, and AVERAGE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | DISTINCT_COUNT | MAX | MIN | SUM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<TopicColumnAggregationEnum>,

    ///
    /// The list of aggregation types that are allowed for the column. Valid values for this     structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP,     and PERCENTILE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_aggregations: Option<Vec<String>>,

    ///
    /// The other names or aliases for the column cell value.
    ///
    /// Required: No
    ///
    /// Type: List of CellValueSynonym
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellValueSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,

    ///
    /// The role of the column in the data. Valid values are DIMENSION and MEASURE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DIMENSION | MEASURE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDataRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_data_role: Option<TopicColumnColumnDataRoleEnum>,

    ///
    /// A description of the column and its contents.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_description: Option<String>,

    ///
    /// A user-friendly name for the column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnFriendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_friendly_name: Option<String>,

    ///
    /// The name of the column.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnName")]
    pub column_name: String,

    ///
    /// The other names or aliases for the column.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_synonyms: Option<Vec<String>>,

    ///
    /// The order in which data is displayed for the column when     it's used in a comparative context.
    ///
    /// Required: No
    ///
    /// Type: ComparativeOrder
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparativeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparative_order: Option<ComparativeOrder>,

    ///
    /// The default formatting used for values in the column.
    ///
    /// Required: No
    ///
    /// Type: DefaultFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultFormatting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_formatting: Option<DefaultFormatting>,

    ///
    /// A Boolean value that indicates whether the column is included in the query results.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsIncludedInTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_included_in_topic: Option<bool>,

    ///
    /// A Boolean     value that indicates whether to aggregate the column data when     it's used in a filter context.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NeverAggregateInFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_aggregate_in_filter: Option<bool>,

    ///
    /// The list of aggregation types that are not allowed for the column. Valid values for this     structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP,     and PERCENTILE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotAllowedAggregations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_aggregations: Option<Vec<String>>,

    ///
    /// The semantic type of data contained in the column.
    ///
    /// Required: No
    ///
    /// Type: SemanticType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<SemanticType>,

    ///
    /// The level of time precision that is used to aggregate DateTime values.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<TopicColumnTimeGranularityEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicColumnAggregationEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// DISTINCT_COUNT
    #[serde(rename = "DISTINCT_COUNT")]
    Distinctcount,

    /// MAX
    #[serde(rename = "MAX")]
    Max,

    /// MIN
    #[serde(rename = "MIN")]
    Min,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,
}

impl Default for TopicColumnAggregationEnum {
    fn default() -> Self {
        TopicColumnAggregationEnum::Average
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicColumnColumnDataRoleEnum {
    /// DIMENSION
    #[serde(rename = "DIMENSION")]
    Dimension,

    /// MEASURE
    #[serde(rename = "MEASURE")]
    Measure,
}

impl Default for TopicColumnColumnDataRoleEnum {
    fn default() -> Self {
        TopicColumnColumnDataRoleEnum::Dimension
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicColumnTimeGranularityEnum {
    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// HOUR
    #[serde(rename = "HOUR")]
    Hour,

    /// MINUTE
    #[serde(rename = "MINUTE")]
    Minute,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// QUARTER
    #[serde(rename = "QUARTER")]
    Quarter,

    /// SECOND
    #[serde(rename = "SECOND")]
    Second,

    /// WEEK
    #[serde(rename = "WEEK")]
    Week,

    /// YEAR
    #[serde(rename = "YEAR")]
    Year,
}

impl Default for TopicColumnTimeGranularityEnum {
    fn default() -> Self {
        TopicColumnTimeGranularityEnum::Day
    }
}

impl cfn_resources::CfnResource for TopicColumn {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.column_description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'column_description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.column_friendly_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'column_friendly_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.column_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'column_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        self.comparative_order
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_formatting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.semantic_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A filter used to restrict data based on a range of dates or times.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicDateRangeFilter {
    ///
    /// The constant used in a date range filter.
    ///
    /// Required: No
    ///
    /// Type: TopicRangeFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicRangeFilterConstant>,

    ///
    /// A Boolean value that indicates whether the date range filter should include the boundary values. If     set to true, the filter includes the start and end dates. If set to false, the filter     excludes them.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
}

impl cfn_resources::CfnResource for TopicDateRangeFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents a filter used to select items for a topic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicFilter {
    ///
    /// The category filter that is associated with this filter.
    ///
    /// Required: No
    ///
    /// Type: TopicCategoryFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter: Option<TopicCategoryFilter>,

    ///
    /// The date range filter.
    ///
    /// Required: No
    ///
    /// Type: TopicDateRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateRangeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range_filter: Option<TopicDateRangeFilter>,

    ///
    /// The class of the filter. Valid values for this structure are       ENFORCED_VALUE_FILTER,     CONDITIONAL_VALUE_FILTER,     and NAMED_VALUE_FILTER.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONDITIONAL_VALUE_FILTER | ENFORCED_VALUE_FILTER | NAMED_VALUE_FILTER
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_class: Option<TopicFilterFilterClassEnum>,

    ///
    /// A description of the filter used to select items for a topic.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_description: Option<String>,

    ///
    /// The name of the filter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterName")]
    pub filter_name: String,

    ///
    /// The other names or aliases for the filter.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterSynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_synonyms: Option<Vec<String>>,

    ///
    /// The type of the filter. Valid values for this structure are     CATEGORY_FILTER, NUMERIC_EQUALITY_FILTER,       NUMERIC_RANGE_FILTER,     DATE_RANGE_FILTER,     and RELATIVE_DATE_FILTER.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CATEGORY_FILTER | DATE_RANGE_FILTER | NUMERIC_EQUALITY_FILTER | NUMERIC_RANGE_FILTER | RELATIVE_DATE_FILTER
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<TopicFilterFilterTypeEnum>,

    ///
    /// The numeric equality filter.
    ///
    /// Required: No
    ///
    /// Type: TopicNumericEqualityFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericEqualityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_equality_filter: Option<TopicNumericEqualityFilter>,

    ///
    /// The numeric range filter.
    ///
    /// Required: No
    ///
    /// Type: TopicNumericRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericRangeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_range_filter: Option<TopicNumericRangeFilter>,

    ///
    /// The name of the field that the filter operates on.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperandFieldName")]
    pub operand_field_name: String,

    ///
    /// The relative date filter.
    ///
    /// Required: No
    ///
    /// Type: TopicRelativeDateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_filter: Option<TopicRelativeDateFilter>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicFilterFilterClassEnum {
    /// CONDITIONAL_VALUE_FILTER
    #[serde(rename = "CONDITIONAL_VALUE_FILTER")]
    Conditionalvaluefilter,

    /// ENFORCED_VALUE_FILTER
    #[serde(rename = "ENFORCED_VALUE_FILTER")]
    Enforcedvaluefilter,

    /// NAMED_VALUE_FILTER
    #[serde(rename = "NAMED_VALUE_FILTER")]
    Namedvaluefilter,
}

impl Default for TopicFilterFilterClassEnum {
    fn default() -> Self {
        TopicFilterFilterClassEnum::Conditionalvaluefilter
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicFilterFilterTypeEnum {
    /// CATEGORY_FILTER
    #[serde(rename = "CATEGORY_FILTER")]
    Categoryfilter,

    /// DATE_RANGE_FILTER
    #[serde(rename = "DATE_RANGE_FILTER")]
    Daterangefilter,

    /// NUMERIC_EQUALITY_FILTER
    #[serde(rename = "NUMERIC_EQUALITY_FILTER")]
    Numericequalityfilter,

    /// NUMERIC_RANGE_FILTER
    #[serde(rename = "NUMERIC_RANGE_FILTER")]
    Numericrangefilter,

    /// RELATIVE_DATE_FILTER
    #[serde(rename = "RELATIVE_DATE_FILTER")]
    Relativedatefilter,
}

impl Default for TopicFilterFilterTypeEnum {
    fn default() -> Self {
        TopicFilterFilterTypeEnum::Categoryfilter
    }
}

impl cfn_resources::CfnResource for TopicFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.category_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.date_range_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.filter_description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'filter_description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.filter_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'filter_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        self.numeric_equality_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.numeric_range_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.operand_field_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'operand_field_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        self.relative_date_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents a named entity.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicNamedEntity {
    ///
    /// The definition of a named entity.
    ///
    /// Required: No
    ///
    /// Type: List of NamedEntityDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<NamedEntityDefinition>>,

    ///
    /// The description of the named entity.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_description: Option<String>,

    ///
    /// The name of the named entity.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityName")]
    pub entity_name: String,

    ///
    /// The other     names or aliases for the named entity.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitySynonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_synonyms: Option<Vec<String>>,

    ///
    /// The type of named entity that a topic represents.
    ///
    /// Required: No
    ///
    /// Type: SemanticEntityType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticEntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_entity_type: Option<SemanticEntityType>,
}

impl cfn_resources::CfnResource for TopicNamedEntity {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.entity_description {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'entity_description'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.entity_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'entity_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        self.semantic_entity_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A filter that filters topics based on the value of a numeric field. The filter includes only topics whose numeric field value matches the specified value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicNumericEqualityFilter {
    ///
    /// An aggregation function that specifies how to calculate the value of a numeric field for     a topic. Valid values for this structure are NO_AGGREGATION, SUM,       AVERAGE, COUNT, DISTINCT_COUNT, MAX,       MEDIAN, MIN, STDEV, STDEVP,       VAR,     and VARP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | DISTINCT_COUNT | MAX | MEDIAN | MIN | NO_AGGREGATION | STDEV | STDEVP | SUM | VAR | VARP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<TopicNumericEqualityFilterAggregationEnum>,

    ///
    /// The constant used in a numeric equality filter.
    ///
    /// Required: No
    ///
    /// Type: TopicSingularFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicSingularFilterConstant>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicNumericEqualityFilterAggregationEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// DISTINCT_COUNT
    #[serde(rename = "DISTINCT_COUNT")]
    Distinctcount,

    /// MAX
    #[serde(rename = "MAX")]
    Max,

    /// MEDIAN
    #[serde(rename = "MEDIAN")]
    Median,

    /// MIN
    #[serde(rename = "MIN")]
    Min,

    /// NO_AGGREGATION
    #[serde(rename = "NO_AGGREGATION")]
    Noaggregation,

    /// STDEV
    #[serde(rename = "STDEV")]
    Stdev,

    /// STDEVP
    #[serde(rename = "STDEVP")]
    Stdevp,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,

    /// VAR
    #[serde(rename = "VAR")]
    Var,

    /// VARP
    #[serde(rename = "VARP")]
    Varp,
}

impl Default for TopicNumericEqualityFilterAggregationEnum {
    fn default() -> Self {
        TopicNumericEqualityFilterAggregationEnum::Average
    }
}

impl cfn_resources::CfnResource for TopicNumericEqualityFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A filter that filters topics based on the value of a numeric field. The filter includes only topics whose numeric field value falls within the specified range.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicNumericRangeFilter {
    ///
    /// An aggregation function that specifies how to calculate the value of a numeric field for     a topic, Valid values for this structure are NO_AGGREGATION, SUM,       AVERAGE, COUNT, DISTINCT_COUNT, MAX,       MEDIAN, MIN, STDEV, STDEVP,       VAR,     and VARP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | COUNT | DISTINCT_COUNT | MAX | MEDIAN | MIN | NO_AGGREGATION | STDEV | STDEVP | SUM | VAR | VARP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Aggregation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<TopicNumericRangeFilterAggregationEnum>,

    ///
    /// The constant used in a     numeric range filter.
    ///
    /// Required: No
    ///
    /// Type: TopicRangeFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicRangeFilterConstant>,

    ///
    /// A Boolean value that indicates whether the endpoints of the numeric range are included in the filter.     If set to true, topics whose numeric field value is equal to the endpoint values will be     included in the filter. If set to false, topics whose numeric field value is equal to the     endpoint values will be excluded from the filter.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicNumericRangeFilterAggregationEnum {
    /// AVERAGE
    #[serde(rename = "AVERAGE")]
    Average,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// DISTINCT_COUNT
    #[serde(rename = "DISTINCT_COUNT")]
    Distinctcount,

    /// MAX
    #[serde(rename = "MAX")]
    Max,

    /// MEDIAN
    #[serde(rename = "MEDIAN")]
    Median,

    /// MIN
    #[serde(rename = "MIN")]
    Min,

    /// NO_AGGREGATION
    #[serde(rename = "NO_AGGREGATION")]
    Noaggregation,

    /// STDEV
    #[serde(rename = "STDEV")]
    Stdev,

    /// STDEVP
    #[serde(rename = "STDEVP")]
    Stdevp,

    /// SUM
    #[serde(rename = "SUM")]
    Sum,

    /// VAR
    #[serde(rename = "VAR")]
    Var,

    /// VARP
    #[serde(rename = "VARP")]
    Varp,
}

impl Default for TopicNumericRangeFilterAggregationEnum {
    fn default() -> Self {
        TopicNumericRangeFilterAggregationEnum::Average
    }
}

impl cfn_resources::CfnResource for TopicNumericRangeFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A constant value that is used in a range filter to specify the endpoints of the range.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicRangeFilterConstant {
    ///
    /// The data type of the constant value that is used in a range filter. Valid values for this structure are RANGE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COLLECTIVE | RANGE | SINGULAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<TopicRangeFilterConstantConstantTypeEnum>,

    ///
    /// The value of the constant that is used to specify the endpoints of a range filter.
    ///
    /// Required: No
    ///
    /// Type: RangeConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeConstant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_constant: Option<RangeConstant>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicRangeFilterConstantConstantTypeEnum {
    /// COLLECTIVE
    #[serde(rename = "COLLECTIVE")]
    Collective,

    /// RANGE
    #[serde(rename = "RANGE")]
    Range,

    /// SINGULAR
    #[serde(rename = "SINGULAR")]
    Singular,
}

impl Default for TopicRangeFilterConstantConstantTypeEnum {
    fn default() -> Self {
        TopicRangeFilterConstantConstantTypeEnum::Collective
    }
}

impl cfn_resources::CfnResource for TopicRangeFilterConstant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.range_constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents a relative date filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicRelativeDateFilter {
    ///
    /// The constant used in a     relative date filter.
    ///
    /// Required: No
    ///
    /// Type: TopicSingularFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicSingularFilterConstant>,

    ///
    /// The function to be used in a relative date filter to determine the range of dates to include in the results. Valid values for this structure are BEFORE, AFTER, and BETWEEN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: LAST | NEXT | NOW | PREVIOUS | THIS
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateFilterFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_filter_function:
        Option<TopicRelativeDateFilterRelativeDateFilterFunctionEnum>,

    ///
    /// The level of time precision that is used to aggregate DateTime values.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | QUARTER | SECOND | WEEK | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeGranularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<TopicRelativeDateFilterTimeGranularityEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicRelativeDateFilterRelativeDateFilterFunctionEnum {
    /// LAST
    #[serde(rename = "LAST")]
    Last,

    /// NEXT
    #[serde(rename = "NEXT")]
    Next,

    /// NOW
    #[serde(rename = "NOW")]
    Now,

    /// PREVIOUS
    #[serde(rename = "PREVIOUS")]
    Previous,

    /// THIS
    #[serde(rename = "THIS")]
    This,
}

impl Default for TopicRelativeDateFilterRelativeDateFilterFunctionEnum {
    fn default() -> Self {
        TopicRelativeDateFilterRelativeDateFilterFunctionEnum::Last
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicRelativeDateFilterTimeGranularityEnum {
    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// HOUR
    #[serde(rename = "HOUR")]
    Hour,

    /// MINUTE
    #[serde(rename = "MINUTE")]
    Minute,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// QUARTER
    #[serde(rename = "QUARTER")]
    Quarter,

    /// SECOND
    #[serde(rename = "SECOND")]
    Second,

    /// WEEK
    #[serde(rename = "WEEK")]
    Week,

    /// YEAR
    #[serde(rename = "YEAR")]
    Year,
}

impl Default for TopicRelativeDateFilterTimeGranularityEnum {
    fn default() -> Self {
        TopicRelativeDateFilterTimeGranularityEnum::Day
    }
}

impl cfn_resources::CfnResource for TopicRelativeDateFilter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constant
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that represents a singular filter constant, used in filters to specify a single value to match against.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicSingularFilterConstant {
    ///
    /// The type of the singular filter constant. Valid values for this structure are SINGULAR.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: COLLECTIVE | RANGE | SINGULAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstantType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<TopicSingularFilterConstantConstantTypeEnum>,

    ///
    /// The value of the singular filter constant.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingularConstant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular_constant: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TopicSingularFilterConstantConstantTypeEnum {
    /// COLLECTIVE
    #[serde(rename = "COLLECTIVE")]
    Collective,

    /// RANGE
    #[serde(rename = "RANGE")]
    Range,

    /// SINGULAR
    #[serde(rename = "SINGULAR")]
    Singular,
}

impl Default for TopicSingularFilterConstantConstantTypeEnum {
    fn default() -> Self {
        TopicSingularFilterConstantConstantTypeEnum::Collective
    }
}

impl cfn_resources::CfnResource for TopicSingularFilterConstant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.singular_constant {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'singular_constant'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
