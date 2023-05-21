

/// Creates a new Q topic.
#[derive(Default, serde::Serialize)]
pub struct CfnTopic {


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
    pub name: Option<String>,


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
    pub description: Option<String>,


    /// 
    /// The data sets that the topic is associated with.
    /// 
    /// Required: No
    ///
    /// Type: List of DatasetMetadata
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSets")]
    pub data_sets: Option<Vec<DatasetMetadata>>,


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
    pub aws_account_id: Option<String>,


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
    pub topic_id: Option<String>,

}


/// The value of the constant that is used to specify the endpoints of a range filter.
#[derive(Default, serde::Serialize)]
pub struct RangeConstant {


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
    pub minimum: Option<String>,


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
    pub maximum: Option<String>,

}


/// A structure that represents the cell value synonym.
#[derive(Default, serde::Serialize)]
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
    pub synonyms: Option<Vec<String>>,

}


/// A structure that represents a semantic type.
#[derive(Default, serde::Serialize)]
pub struct SemanticType {


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
    pub type_name: Option<String>,


    /// 
    /// The semantic type truthy cell value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruthyCellValue")]
    pub truthy_cell_value: Option<String>,


    /// 
    /// The semantic type falsey cell value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseyCellValue")]
    pub falsey_cell_value: Option<String>,


    /// 
    /// The other names or aliases for the true cell value.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruthyCellValueSynonyms")]
    pub truthy_cell_value_synonyms: Option<Vec<String>>,


    /// 
    /// The semantic type parameters.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeParameters")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,


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
    pub sub_type_name: Option<String>,


    /// 
    /// The other names or aliases for the false cell value.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseyCellValueSynonyms")]
    pub falsey_cell_value_synonyms: Option<Vec<String>>,

}


/// A structure that represents a filter used to select items for a topic.
#[derive(Default, serde::Serialize)]
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
    pub category_filter: Option<TopicCategoryFilter>,


    /// 
    /// The other names or aliases for the filter.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterSynonyms")]
    pub filter_synonyms: Option<Vec<String>>,


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
    /// The date range filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicDateRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateRangeFilter")]
    pub date_range_filter: Option<TopicDateRangeFilter>,


    /// 
    /// The numeric range filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicNumericRangeFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericRangeFilter")]
    pub numeric_range_filter: Option<TopicNumericRangeFilter>,


    /// 
    /// The relative date filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicRelativeDateFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "RelativeDateFilter")]
    pub relative_date_filter: Option<TopicRelativeDateFilter>,


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
    pub filter_description: Option<String>,


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
    pub filter_class: Option<String>,


    /// 
    /// The numeric equality filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicNumericEqualityFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumericEqualityFilter")]
    pub numeric_equality_filter: Option<TopicNumericEqualityFilter>,


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
    pub filter_type: Option<String>,


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

}


/// A structure that represents a category filter.
#[derive(Default, serde::Serialize)]
pub struct TopicCategoryFilter {


    /// 
    /// A Boolean value that indicates if the filter is inverse.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inverse")]
    pub inverse: Option<bool>,


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
    pub category_filter_function: Option<String>,


    /// 
    /// The constant used in a category filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicCategoryFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    pub constant: Option<TopicCategoryFilterConstant>,


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
    pub category_filter_type: Option<String>,

}


/// A structure that represents a semantic entity type.
#[derive(Default, serde::Serialize)]
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
    pub sub_type_name: Option<String>,


    /// 
    /// The semantic entity type parameters.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeParameters")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,


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
    pub type_name: Option<String>,

}


/// A filter that filters topics based on the value of a numeric field. The filter includes only topics whose numeric field value matches the specified value.
#[derive(Default, serde::Serialize)]
pub struct TopicNumericEqualityFilter {


    /// 
    /// The constant used in a numeric equality filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicSingularFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    pub constant: Option<TopicSingularFilterConstant>,


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
    pub aggregation: Option<String>,

}


/// A structure that represents a named entity.
#[derive(Default, serde::Serialize)]
pub struct TopicNamedEntity {


    /// 
    /// The other     names or aliases for the named entity.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntitySynonyms")]
    pub entity_synonyms: Option<Vec<String>>,


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
    pub entity_description: Option<String>,


    /// 
    /// The type of named entity that a topic represents.
    /// 
    /// Required: No
    ///
    /// Type: SemanticEntityType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticEntityType")]
    pub semantic_entity_type: Option<SemanticEntityType>,


    /// 
    /// The definition of a named entity.
    /// 
    /// Required: No
    ///
    /// Type: List of NamedEntityDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Definition")]
    pub definition: Option<Vec<NamedEntityDefinition>>,

}


/// A structure that represents a default formatting definition.
#[derive(Default, serde::Serialize)]
pub struct DefaultFormatting {


    /// 
    /// The additional options for display formatting.
    /// 
    /// Required: No
    ///
    /// Type: DisplayFormatOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayFormatOptions")]
    pub display_format_options: Option<DisplayFormatOptions>,


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
    pub display_format: Option<String>,

}


/// A structure that represents a collective constant.
#[derive(Default, serde::Serialize)]
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
    pub value_list: Option<Vec<String>>,

}


/// A structure that represents a metric.
#[derive(Default, serde::Serialize)]
pub struct NamedEntityDefinitionMetric {


    /// 
    /// The additional parameters for an aggregation function.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationFunctionParameters")]
    pub aggregation_function_parameters: Option<std::collections::HashMap<String, String>>,


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
    pub aggregation: Option<String>,

}


/// A constant used in a category filter.
#[derive(Default, serde::Serialize)]
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
    pub collective_constant: Option<CollectiveConstant>,


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
    pub singular_constant: Option<String>,


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
    pub constant_type: Option<String>,

}


/// A structure that represents a negative format.
#[derive(Default, serde::Serialize)]
pub struct NegativeFormat {


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
    pub suffix: Option<String>,


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
    pub prefix: Option<String>,

}


/// A structure that represents additional options for display formatting.
#[derive(Default, serde::Serialize)]
pub struct DisplayFormatOptions {


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
    pub currency_symbol: Option<String>,


    /// 
    /// A Boolean value that indicates whether to use grouping.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseGrouping")]
    pub use_grouping: Option<bool>,


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
    pub blank_cell_format: Option<String>,


    /// 
    /// The negative format.
    /// 
    /// Required: No
    ///
    /// Type: NegativeFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "NegativeFormat")]
    pub negative_format: Option<NegativeFormat>,


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
    pub unit_scaler: Option<String>,


    /// 
    /// A Boolean value that indicates whether to use blank cell format.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBlankCellFormat")]
    pub use_blank_cell_format: Option<bool>,


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
    pub prefix: Option<String>,


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
    pub decimal_separator: Option<String>,


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
    pub grouping_separator: Option<String>,


    /// 
    /// Determines the number of fraction digits.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FractionDigits")]
    pub fraction_digits: Option<f64>,

}


/// Represents a column in a dataset.
#[derive(Default, serde::Serialize)]
pub struct TopicColumn {


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
    pub time_granularity: Option<String>,


    /// 
    /// The other names or aliases for the column cell value.
    /// 
    /// Required: No
    ///
    /// Type: List of CellValueSynonym
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellValueSynonyms")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,


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
    pub column_description: Option<String>,


    /// 
    /// A Boolean value that indicates whether the column is included in the query results.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsIncludedInTopic")]
    pub is_included_in_topic: Option<bool>,


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
    pub column_friendly_name: Option<String>,


    /// 
    /// The list of aggregation types that are not allowed for the column. Valid values for this     structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP,     and PERCENTILE.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotAllowedAggregations")]
    pub not_allowed_aggregations: Option<Vec<String>>,


    /// 
    /// The list of aggregation types that are allowed for the column. Valid values for this     structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP,     and PERCENTILE.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedAggregations")]
    pub allowed_aggregations: Option<Vec<String>>,


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
    pub column_data_role: Option<String>,


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
    pub aggregation: Option<String>,


    /// 
    /// The default formatting used for values in the column.
    /// 
    /// Required: No
    ///
    /// Type: DefaultFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultFormatting")]
    pub default_formatting: Option<DefaultFormatting>,


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
    /// The order in which data is displayed for the column when     it's used in a comparative context.
    /// 
    /// Required: No
    ///
    /// Type: ComparativeOrder
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparativeOrder")]
    pub comparative_order: Option<ComparativeOrder>,


    /// 
    /// A Boolean     value that indicates whether to aggregate the column data when     it's used in a filter context.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NeverAggregateInFilter")]
    pub never_aggregate_in_filter: Option<bool>,


    /// 
    /// The other names or aliases for the column.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSynonyms")]
    pub column_synonyms: Option<Vec<String>>,


    /// 
    /// The semantic type of data contained in the column.
    /// 
    /// Required: No
    ///
    /// Type: SemanticType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticType")]
    pub semantic_type: Option<SemanticType>,

}


/// The definition of a data aggregation.
#[derive(Default, serde::Serialize)]
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
    pub dataset_row_date_granularity: Option<String>,


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
    pub default_date_column_name: Option<String>,

}


/// A constant value that is used in a range filter to specify the endpoints of the range.
#[derive(Default, serde::Serialize)]
pub struct TopicRangeFilterConstant {


    /// 
    /// The value of the constant that is used to specify the endpoints of a range filter.
    /// 
    /// Required: No
    ///
    /// Type: RangeConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeConstant")]
    pub range_constant: Option<RangeConstant>,


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
    pub constant_type: Option<String>,

}


/// A structure that represents a dataset.
#[derive(Default, serde::Serialize)]
pub struct DatasetMetadata {


    /// 
    /// The list of column definitions.
    /// 
    /// Required: No
    ///
    /// Type: List of TopicColumn
    ///
    /// Update requires: No interruption
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<TopicColumn>>,


    /// 
    /// The list of calculated field definitions.
    /// 
    /// Required: No
    ///
    /// Type: List of TopicCalculatedField
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFields")]
    pub calculated_fields: Option<Vec<TopicCalculatedField>>,


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
    /// The definition of a data aggregation.
    /// 
    /// Required: No
    ///
    /// Type: DataAggregation
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataAggregation")]
    pub data_aggregation: Option<DataAggregation>,


    /// 
    /// The list of named entities definitions.
    /// 
    /// Required: No
    ///
    /// Type: List of TopicNamedEntity
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamedEntities")]
    pub named_entities: Option<Vec<TopicNamedEntity>>,


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
    pub dataset_description: Option<String>,


    /// 
    /// The list of filter definitions.
    /// 
    /// Required: No
    ///
    /// Type: List of TopicFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<TopicFilter>>,


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
    pub dataset_name: Option<String>,

}


/// A structure that represents a relative date filter.
#[derive(Default, serde::Serialize)]
pub struct TopicRelativeDateFilter {


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
    pub time_granularity: Option<String>,


    /// 
    /// The constant used in a     relative date filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicSingularFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
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
    pub relative_date_filter_function: Option<String>,

}


/// A structure that represents a singular filter constant, used in filters to specify a single value to match against.
#[derive(Default, serde::Serialize)]
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
    pub constant_type: Option<String>,


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
    pub singular_constant: Option<String>,

}


/// A filter used to restrict data based on a range of dates or times.
#[derive(Default, serde::Serialize)]
pub struct TopicDateRangeFilter {


    /// 
    /// A Boolean value that indicates whether the date range filter should include the boundary values. If     set to true, the filter includes the start and end dates. If set to false, the filter     excludes them.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Inclusive")]
    pub inclusive: Option<bool>,


    /// 
    /// The constant used in a date range filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicRangeFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
    pub constant: Option<TopicRangeFilterConstant>,

}


/// The order in which data is displayed for the column when     it's used in a comparative context.
#[derive(Default, serde::Serialize)]
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
    pub specifed_order: Option<Vec<String>>,


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
    pub use_ordering: Option<String>,


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
    pub treat_undefined_specified_values: Option<String>,

}


/// A structure that represents a calculated field.
#[derive(Default, serde::Serialize)]
pub struct TopicCalculatedField {


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
    /// A boolean value that indicates if a calculated field is included in the topic.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsIncludedInTopic")]
    pub is_included_in_topic: Option<bool>,


    /// 
    /// The list of aggregation types that are not allowed for the calculated field. Valid     values for this structure are COUNT, DISTINCT_COUNT,       MIN, MAX, MEDIAN, SUM,       AVERAGE, STDEV, STDEVP, VAR,       VARP, and PERCENTILE.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotAllowedAggregations")]
    pub not_allowed_aggregations: Option<Vec<String>>,


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
    pub column_data_role: Option<String>,


    /// 
    /// The list of aggregation types that are allowed for the calculated field. Valid values     for this structure are COUNT, DISTINCT_COUNT, MIN,       MAX, MEDIAN, SUM, AVERAGE,       STDEV, STDEVP, VAR,     VARP, and PERCENTILE.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedAggregations")]
    pub allowed_aggregations: Option<Vec<String>>,


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
    pub time_granularity: Option<String>,


    /// 
    /// The other names or aliases for the calculated field.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CalculatedFieldSynonyms")]
    pub calculated_field_synonyms: Option<Vec<String>>,


    /// 
    /// The semantic type.
    /// 
    /// Required: No
    ///
    /// Type: SemanticType
    ///
    /// Update requires: No interruption
    #[serde(rename = "SemanticType")]
    pub semantic_type: Option<SemanticType>,


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
    pub calculated_field_description: Option<String>,


    /// 
    /// The order in which data is displayed for the calculated field when     it's used in a comparative context.
    /// 
    /// Required: No
    ///
    /// Type: ComparativeOrder
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparativeOrder")]
    pub comparative_order: Option<ComparativeOrder>,


    /// 
    /// The other     names or aliases for the calculated field cell value.
    /// 
    /// Required: No
    ///
    /// Type: List of CellValueSynonym
    ///
    /// Update requires: No interruption
    #[serde(rename = "CellValueSynonyms")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,


    /// 
    /// A Boolean value that indicates whether to never aggregate calculated field in filters.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NeverAggregateInFilter")]
    pub never_aggregate_in_filter: Option<bool>,


    /// 
    /// The default formatting definition.
    /// 
    /// Required: No
    ///
    /// Type: DefaultFormatting
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultFormatting")]
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
    pub aggregation: Option<String>,

}


/// A filter that filters topics based on the value of a numeric field. The filter includes only topics whose numeric field value falls within the specified range.
#[derive(Default, serde::Serialize)]
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
    pub aggregation: Option<String>,


    /// 
    /// The constant used in a     numeric range filter.
    /// 
    /// Required: No
    ///
    /// Type: TopicRangeFilterConstant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constant")]
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
    pub inclusive: Option<bool>,

}


/// A structure that represents a named entity.
#[derive(Default, serde::Serialize)]
pub struct NamedEntityDefinition {


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
    pub property_role: Option<String>,


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
    pub property_name: Option<String>,


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
    pub metric: Option<NamedEntityDefinitionMetric>,


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
    pub property_usage: Option<String>,

}
