
pub mod cfn_dataset {

#[derive(serde::Serialize, Default)]
pub struct CfnDataset {
    /// Dataset name
    #[serde(rename = "Name")]
    pub name: String,
    /// Format options for dataset
    #[serde(rename = "FormatOptions")]
    pub format_options: Option<FormatOptions>,
    /// Input
    #[serde(rename = "Input")]
    pub input: Input,
    /// PathOptions
    #[serde(rename = "PathOptions")]
    pub path_options: Option<PathOptions>,
    /// Dataset format
    #[serde(rename = "Format")]
    pub format: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct FilterExpression {
    #[serde(rename = "ValuesMap")]
    pub values_map: Vec<FilterValue>,
    #[serde(rename = "Expression")]
    pub expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct Metadata {
    #[serde(rename = "SourceArn")]
    pub source_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterValue {
    #[serde(rename = "ValueReference")]
    pub value_reference: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataCatalogInputDefinition {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Input {
    #[serde(rename = "DataCatalogInputDefinition")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,
    #[serde(rename = "DatabaseInputDefinition")]
    pub database_input_definition: Option<DatabaseInputDefinition>,
    #[serde(rename = "S3InputDefinition")]
    pub s3_input_definition: Option<S3Location>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,

}

#[derive(serde::Serialize, Default)]
pub struct CsvOptions {
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "HeaderRow")]
    pub header_row: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct FilesLimit {
    #[serde(rename = "MaxFiles")]
    pub max_files: usize,
    #[serde(rename = "OrderedBy")]
    pub ordered_by: Option<String>,
    #[serde(rename = "Order")]
    pub order: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JsonOptions {
    #[serde(rename = "MultiLine")]
    pub multi_line: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PathOptions {
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<PathParameter>>,
    #[serde(rename = "FilesLimit")]
    pub files_limit: Option<FilesLimit>,
    #[serde(rename = "LastModifiedDateCondition")]
    pub last_modified_date_condition: Option<FilterExpression>,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetParameter {
    #[serde(rename = "Name")]
    pub name: PathParameterName,
    #[serde(rename = "DatetimeOptions")]
    pub datetime_options: Option<DatetimeOptions>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Filter")]
    pub filter: Option<FilterExpression>,
    #[serde(rename = "CreateColumn")]
    pub create_column: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExcelOptions {
    #[serde(rename = "SheetIndexes")]
    pub sheet_indexes: Option<Vec<usize>>,
    #[serde(rename = "HeaderRow")]
    pub header_row: Option<bool>,
    #[serde(rename = "SheetNames")]
    pub sheet_names: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct PathParameter {
    #[serde(rename = "PathParameterName")]
    pub path_parameter_name: PathParameterName,
    #[serde(rename = "DatasetParameter")]
    pub dataset_parameter: DatasetParameter,

}
pub type PathParameterName = String;
#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FormatOptions {
    #[serde(rename = "Json")]
    pub json: Option<JsonOptions>,
    #[serde(rename = "Excel")]
    pub excel: Option<ExcelOptions>,
    #[serde(rename = "Csv")]
    pub csv: Option<CsvOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct DatabaseInputDefinition {
    #[serde(rename = "GlueConnectionName")]
    pub glue_connection_name: String,
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,
    #[serde(rename = "DatabaseTableName")]
    pub database_table_name: Option<String>,
    #[serde(rename = "QueryString")]
    pub query_string: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DatetimeOptions {
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "LocaleCode")]
    pub locale_code: Option<String>,
    #[serde(rename = "TimezoneOffset")]
    pub timezone_offset: Option<String>,

}


}

pub mod cfn_job {

#[derive(serde::Serialize, Default)]
pub struct CfnJob {
    /// Encryption mode
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<String>,
    /// List of Output
    #[serde(rename = "Outputs")]
    pub outputs: Option<Vec<Output>>,
    /// Encryption Key Arn
    #[serde(rename = "EncryptionKeyArn")]
    pub encryption_key_arn: Option<String>,
    /// Max retries
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<usize>,
    /// Dataset name
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,
    /// List of DataCatalogOutput
    #[serde(rename = "DataCatalogOutputs")]
    pub data_catalog_outputs: Option<Vec<DataCatalogOutput>>,
    /// Job name
    #[serde(rename = "Name")]
    pub name: String,
    /// Log subscription
    #[serde(rename = "LogSubscription")]
    pub log_subscription: Option<String>,
    /// Output location
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<OutputLocation>,
    /// Project name
    #[serde(rename = "ProjectName")]
    pub project_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Recipe")]
    pub recipe: Option<Recipe>,
    /// Job type
    #[serde(rename = "Type")]
    pub ty: String,
    /// Role arn
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Job Sample
    #[serde(rename = "JobSample")]
    pub job_sample: Option<JobSample>,
    /// Profile Job configuration
    #[serde(rename = "ProfileConfiguration")]
    pub profile_configuration: Option<ProfileConfiguration>,
    /// Max capacity
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<usize>,
    /// Data quality rules configuration
    #[serde(rename = "ValidationConfigurations")]
    pub validation_configurations: Option<Vec<ValidationConfiguration>>,
    /// Timeout
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    /// List of DatabaseOutput
    #[serde(rename = "DatabaseOutputs")]
    pub database_outputs: Option<Vec<DatabaseOutput>>,

}


#[derive(serde::Serialize, Default)]
pub struct DatabaseOutput {
    #[serde(rename = "GlueConnectionName")]
    pub glue_connection_name: String,
    #[serde(rename = "DatabaseOutputMode")]
    pub database_output_mode: Option<String>,
    #[serde(rename = "DatabaseOptions")]
    pub database_options: DatabaseTableOutputOptions,

}
pub type ValidationMode = String;
#[derive(serde::Serialize, Default)]
pub struct AllowedStatistics {
    #[serde(rename = "Statistics")]
    pub statistics: Vec<Statistic>,

}

#[derive(serde::Serialize, Default)]
pub struct S3TableOutputOptions {
    #[serde(rename = "Location")]
    pub location: S3Location,

}

#[derive(serde::Serialize, Default)]
pub struct EntityDetectorConfiguration {
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<String>,
    #[serde(rename = "AllowedStatistics")]
    pub allowed_statistics: Option<AllowedStatistics>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type SampleMode = String;
#[derive(serde::Serialize, Default)]
pub struct JobSample {
    #[serde(rename = "Size")]
    pub size: Option<JobSize>,
    #[serde(rename = "Mode")]
    pub mode: Option<SampleMode>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StatisticsConfiguration {
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<StatisticOverride>>,
    #[serde(rename = "IncludedStatistics")]
    pub included_statistics: Option<Vec<Statistic>>,

}

#[derive(serde::Serialize, Default)]
pub struct StatisticOverride {
    #[serde(rename = "Statistic")]
    pub statistic: Statistic,
    #[serde(rename = "Parameters")]
    pub parameters: ParameterMap,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnSelector {
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
pub type Statistic = String;
#[derive(serde::Serialize, Default)]
pub struct DatabaseTableOutputOptions {
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,
    #[serde(rename = "TableName")]
    pub table_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct OutputFormatOptions {
    #[serde(rename = "Csv")]
    pub csv: Option<CsvOutputOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct CsvOutputOptions {
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputLocation {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProfileConfiguration {
    #[serde(rename = "ProfileColumns")]
    pub profile_columns: Option<Vec<ColumnSelector>>,
    #[serde(rename = "DatasetStatisticsConfiguration")]
    pub dataset_statistics_configuration: Option<StatisticsConfiguration>,
    #[serde(rename = "ColumnStatisticsConfigurations")]
    pub column_statistics_configurations: Option<Vec<ColumnStatisticsConfiguration>>,
    #[serde(rename = "EntityDetectorConfiguration")]
    pub entity_detector_configuration: Option<EntityDetectorConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct ValidationConfiguration {
    #[serde(rename = "RulesetArn")]
    pub ruleset_arn: String,
    #[serde(rename = "ValidationMode")]
    pub validation_mode: Option<ValidationMode>,

}

#[derive(serde::Serialize, Default)]
pub struct Recipe {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterMap {

}

#[derive(serde::Serialize, Default)]
pub struct Output {
    #[serde(rename = "FormatOptions")]
    pub format_options: Option<OutputFormatOptions>,
    #[serde(rename = "Overwrite")]
    pub overwrite: Option<bool>,
    #[serde(rename = "Location")]
    pub location: S3Location,
    #[serde(rename = "MaxOutputFiles")]
    pub max_output_files: Option<usize>,
    #[serde(rename = "CompressionFormat")]
    pub compression_format: Option<String>,
    #[serde(rename = "PartitionColumns")]
    pub partition_columns: Option<Vec<String>>,
    #[serde(rename = "Format")]
    pub format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnStatisticsConfiguration {
    #[serde(rename = "Selectors")]
    pub selectors: Option<Vec<ColumnSelector>>,
    #[serde(rename = "Statistics")]
    pub statistics: StatisticsConfiguration,

}
pub type JobSize = usize;
#[derive(serde::Serialize, Default)]
pub struct DataCatalogOutput {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "S3Options")]
    pub s3_options: Option<S3TableOutputOptions>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseOptions")]
    pub database_options: Option<DatabaseTableOutputOptions>,
    #[serde(rename = "Overwrite")]
    pub overwrite: Option<bool>,
    #[serde(rename = "TableName")]
    pub table_name: String,

}


}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Sample
    #[serde(rename = "Sample")]
    pub sample: Option<Sample>,
    /// Project name
    #[serde(rename = "Name")]
    pub name: String,
    /// Recipe name
    #[serde(rename = "RecipeName")]
    pub recipe_name: String,
    /// Dataset name
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// Role arn
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Sample {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Size")]
    pub size: Option<usize>,

}


}

pub mod cfn_recipe {

#[derive(serde::Serialize, Default)]
pub struct CfnRecipe {
    /// Description of the recipe
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Recipe name
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of RecipeStep
    #[serde(rename = "Steps")]
    pub steps: Vec<RecipeStep>,

}


#[derive(serde::Serialize, Default)]
pub struct SecondaryInput {
    #[serde(rename = "S3InputDefinition")]
    pub s3_input_definition: Option<S3Location>,
    #[serde(rename = "DataCatalogInputDefinition")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterMap {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionExpression {
    #[serde(rename = "Condition")]
    pub condition: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "TargetColumn")]
    pub target_column: String,

}

#[derive(serde::Serialize, Default)]
pub struct RecipeParameters {
    #[serde(rename = "ExpandContractions")]
    pub expand_contractions: Option<String>,
    #[serde(rename = "Other")]
    pub other: Option<String>,
    #[serde(rename = "RemoveSpecialCharacters")]
    pub remove_special_characters: Option<String>,
    #[serde(rename = "Value1")]
    pub value1: Option<String>,
    #[serde(rename = "LeftColumns")]
    pub left_columns: Option<String>,
    #[serde(rename = "TargetColumn")]
    pub target_column: Option<String>,
    #[serde(rename = "Base")]
    pub base: Option<String>,
    #[serde(rename = "MapType")]
    pub map_type: Option<String>,
    #[serde(rename = "RemoveLeadingAndTrailingWhitespace")]
    pub remove_leading_and_trailing_whitespace: Option<String>,
    #[serde(rename = "SampleType")]
    pub sample_type: Option<String>,
    #[serde(rename = "SecondaryInputs")]
    pub secondary_inputs: Option<Vec<SecondaryInput>>,
    #[serde(rename = "SourceColumns")]
    pub source_columns: Option<String>,
    #[serde(rename = "TargetDateFormat")]
    pub target_date_format: Option<String>,
    #[serde(rename = "StartValue")]
    pub start_value: Option<String>,
    #[serde(rename = "CollapseConsecutiveWhitespace")]
    pub collapse_consecutive_whitespace: Option<String>,
    #[serde(rename = "ViewFrame")]
    pub view_frame: Option<String>,
    #[serde(rename = "IsText")]
    pub is_text: Option<String>,
    #[serde(rename = "SampleSize")]
    pub sample_size: Option<String>,
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "IgnoreCase")]
    pub ignore_case: Option<String>,
    #[serde(rename = "CustomStopWords")]
    pub custom_stop_words: Option<String>,
    #[serde(rename = "RemoveAllQuotes")]
    pub remove_all_quotes: Option<String>,
    #[serde(rename = "ColumnDataType")]
    pub column_data_type: Option<String>,
    #[serde(rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "CustomCharacters")]
    pub custom_characters: Option<String>,
    #[serde(rename = "Exponent")]
    pub exponent: Option<String>,
    #[serde(rename = "NumRowsAfter")]
    pub num_rows_after: Option<String>,
    #[serde(rename = "GroupByAggFunctionOptions")]
    pub group_by_agg_function_options: Option<String>,
    #[serde(rename = "Period")]
    pub period: Option<String>,
    #[serde(rename = "EndPosition")]
    pub end_position: Option<String>,
    #[serde(rename = "LowerBound")]
    pub lower_bound: Option<String>,
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,
    #[serde(rename = "PatternOptions")]
    pub pattern_options: Option<String>,
    #[serde(rename = "RightColumns")]
    pub right_columns: Option<String>,
    #[serde(rename = "CategoryMap")]
    pub category_map: Option<String>,
    #[serde(rename = "Interval")]
    pub interval: Option<String>,
    #[serde(rename = "StepCount")]
    pub step_count: Option<String>,
    #[serde(rename = "SheetNames")]
    pub sheet_names: Option<Vec<String>>,
    #[serde(rename = "DatasetsColumns")]
    pub datasets_columns: Option<String>,
    #[serde(rename = "UdfLang")]
    pub udf_lang: Option<String>,
    #[serde(rename = "RemoveSourceColumn")]
    pub remove_source_column: Option<String>,
    #[serde(rename = "AggregateFunction")]
    pub aggregate_function: Option<String>,
    #[serde(rename = "Value2")]
    pub value2: Option<String>,
    #[serde(rename = "DateTimeParameters")]
    pub date_time_parameters: Option<String>,
    #[serde(rename = "RemoveAllPunctuation")]
    pub remove_all_punctuation: Option<String>,
    #[serde(rename = "StepIndex")]
    pub step_index: Option<String>,
    #[serde(rename = "TargetIndex")]
    pub target_index: Option<String>,
    #[serde(rename = "StartColumnIndex")]
    pub start_column_index: Option<String>,
    #[serde(rename = "CaseStatement")]
    pub case_statement: Option<String>,
    #[serde(rename = "GroupByColumns")]
    pub group_by_columns: Option<String>,
    #[serde(rename = "StemmingMode")]
    pub stemming_mode: Option<String>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "TokenizerPattern")]
    pub tokenizer_pattern: Option<String>,
    #[serde(rename = "SecondInput")]
    pub second_input: Option<String>,
    #[serde(rename = "Units")]
    pub units: Option<String>,
    #[serde(rename = "DeleteOtherRows")]
    pub delete_other_rows: Option<String>,
    #[serde(rename = "Strategy")]
    pub strategy: Option<String>,
    #[serde(rename = "RemoveLeadingAndTrailingQuotes")]
    pub remove_leading_and_trailing_quotes: Option<String>,
    #[serde(rename = "ValueColumn")]
    pub value_column: Option<String>,
    #[serde(rename = "Input")]
    pub input: Option<()>,
    #[serde(rename = "TrueString")]
    pub true_string: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,
    #[serde(rename = "SourceColumn1")]
    pub source_column1: Option<String>,
    #[serde(rename = "DateAddValue")]
    pub date_add_value: Option<String>,
    #[serde(rename = "FalseString")]
    pub false_string: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<String>,
    #[serde(rename = "OrderByColumn")]
    pub order_by_column: Option<String>,
    #[serde(rename = "HiddenColumns")]
    pub hidden_columns: Option<String>,
    #[serde(rename = "RemoveNumbers")]
    pub remove_numbers: Option<String>,
    #[serde(rename = "StartPosition")]
    pub start_position: Option<String>,
    #[serde(rename = "SheetIndexes")]
    pub sheet_indexes: Option<Vec<usize>>,
    #[serde(rename = "RemoveLetters")]
    pub remove_letters: Option<String>,
    #[serde(rename = "JoinType")]
    pub join_type: Option<String>,
    #[serde(rename = "MultiLine")]
    pub multi_line: Option<bool>,
    #[serde(rename = "RemoveCustomCharacters")]
    pub remove_custom_characters: Option<String>,
    #[serde(rename = "Count")]
    pub count: Option<String>,
    #[serde(rename = "PatternOption1")]
    pub pattern_option1: Option<String>,
    #[serde(rename = "NumRowsBefore")]
    pub num_rows_before: Option<String>,
    #[serde(rename = "RemoveAllWhitespace")]
    pub remove_all_whitespace: Option<String>,
    #[serde(rename = "RemoveCustomValue")]
    pub remove_custom_value: Option<String>,
    #[serde(rename = "StartPattern")]
    pub start_pattern: Option<String>,
    #[serde(rename = "NumRows")]
    pub num_rows: Option<String>,
    #[serde(rename = "CharsToRemove")]
    pub chars_to_remove: Option<String>,
    #[serde(rename = "UseNewDataFrame")]
    pub use_new_data_frame: Option<String>,
    #[serde(rename = "ModeType")]
    pub mode_type: Option<String>,
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,
    #[serde(rename = "ColumnRange")]
    pub column_range: Option<String>,
    #[serde(rename = "RemoveLeadingAndTrailingPunctuation")]
    pub remove_leading_and_trailing_punctuation: Option<String>,
    #[serde(rename = "SourceColumn")]
    pub source_column: Option<String>,
    #[serde(rename = "TargetColumnNames")]
    pub target_column_names: Option<String>,
    #[serde(rename = "IncludeInSplit")]
    pub include_in_split: Option<String>,
    #[serde(rename = "UpperBound")]
    pub upper_bound: Option<String>,
    #[serde(rename = "JoinKeys")]
    pub join_keys: Option<String>,
    #[serde(rename = "StopWordsMode")]
    pub stop_words_mode: Option<String>,
    #[serde(rename = "EndValue")]
    pub end_value: Option<String>,
    #[serde(rename = "PatternOption2")]
    pub pattern_option2: Option<String>,
    #[serde(rename = "SourceColumn2")]
    pub source_column2: Option<String>,
    #[serde(rename = "UnpivotColumn")]
    pub unpivot_column: Option<String>,
    #[serde(rename = "EndPattern")]
    pub end_pattern: Option<String>,
    #[serde(rename = "OrderByColumns")]
    pub order_by_columns: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataCatalogInputDefinition {
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "Operation")]
    pub operation: String,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct RecipeStep {
    #[serde(rename = "Action")]
    pub action: Action,
    #[serde(rename = "ConditionExpressions")]
    pub condition_expressions: Option<Vec<ConditionExpression>>,

}


}

pub mod cfn_ruleset {

#[derive(serde::Serialize, Default)]
pub struct CfnRuleset {
    /// Arn of the target resource (dataset) to apply the ruleset to
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Name of the Ruleset
    #[serde(rename = "Name")]
    pub name: String,
    /// List of the data quality rules in the ruleset
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
    /// Description of the Ruleset
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ValuesMap {

}
pub type ThresholdValue = f64;
#[derive(serde::Serialize, Default)]
pub struct ColumnSelector {
    #[serde(rename = "Regex")]
    pub regex: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}
pub type ThresholdType = String;
#[derive(serde::Serialize, Default)]
pub struct Threshold {
    #[serde(rename = "Unit")]
    pub unit: Option<ThresholdUnit>,
    #[serde(rename = "Value")]
    pub value: ThresholdValue,
    #[serde(rename = "Type")]
    pub ty: Option<ThresholdType>,

}
pub type Disabled = bool;
#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "ColumnSelectors")]
    pub column_selectors: Option<Vec<ColumnSelector>>,
    #[serde(rename = "Threshold")]
    pub threshold: Option<Threshold>,
    #[serde(rename = "Disabled")]
    pub disabled: Option<Disabled>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CheckExpression")]
    pub check_expression: Expression,
    #[serde(rename = "SubstitutionMap")]
    pub substitution_map: Option<ValuesMap>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type ThresholdUnit = String;pub type Expression = String;
#[derive(serde::Serialize, Default)]
pub struct SubstitutionValue {
    #[serde(rename = "ValueReference")]
    pub value_reference: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_schedule {

#[derive(serde::Serialize, Default)]
pub struct CfnSchedule {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of JobName
    #[serde(rename = "JobNames")]
    pub job_names: Option<Vec<JobName>>,
    /// Schedule cron
    #[serde(rename = "CronExpression")]
    pub cron_expression: String,
    /// Schedule Name
    #[serde(rename = "Name")]
    pub name: String,

}

pub type JobName = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
