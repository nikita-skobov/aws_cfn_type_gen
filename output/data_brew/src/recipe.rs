/// Specifies a new AWS Glue DataBrew transformation recipe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRecipe {
    ///
    /// The description of the recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The unique name for the recipe.
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
    /// A list of steps that are defined by the recipe.
    ///
    /// Required: Yes
    ///
    /// Type: List of RecipeStep
    ///
    /// Update requires: No interruption
    #[serde(rename = "Steps")]
    pub steps: Vec<RecipeStep>,

    ///
    /// Metadata tags that have been applied to the recipe.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnRecipe {
    fn type_string(&self) -> &'static str {
        "AWS::DataBrew::Recipe"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'description'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Represents a transformation and associated parameters that are used to apply a change       to an AWS Glue DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {
    ///
    /// The name of a valid DataBrew transformation to be performed on the       data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operation")]
    pub operation: String,

    ///
    /// Contextual parameters for the transformation.
    ///
    /// Required: No
    ///
    /// Type: RecipeParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<RecipeParameters>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents an individual condition that evaluates to true or false.
///
/// Conditions are used with recipe actions. The action is only performed for column values where the       condition evaluates to true.
///
/// If a recipe requires more than one condition, then the recipe must specify multiple       ConditionExpression elements. Each condition is applied to the rows in a dataset first, before       the recipe action is performed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConditionExpression {
    ///
    /// A specific condition to apply to a recipe action. For more information, see Recipe         structure in the AWS Glue DataBrew Developer         Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[A-Z\_]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: String,

    ///
    /// A column to apply this condition to.
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
    #[serde(rename = "TargetColumn")]
    pub target_column: String,

    ///
    /// A value that the condition must evaluate to for the condition to succeed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl cfn_resources::CfnResource for ConditionExpression {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.condition;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'condition'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.condition;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'condition'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.target_column;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'target_column'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.target_column;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'target_column'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.value {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 1024",
                    the_val.len()
                ));
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

    ///
    /// Represents an Amazon location where DataBrew can store intermediate results.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<S3Location>,
}

impl cfn_resources::CfnResource for DataCatalogInputDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.catalog_id {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'catalog_id'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.catalog_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'catalog_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.database_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'database_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.database_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'database_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.table_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'table_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.table_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'table_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.temp_directory
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Input property type specifies Property description not available. for an AWS::DataBrew::Recipe.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Input {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DataCatalogInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogInputDefinition")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputDefinition")]
    pub s3_input_definition: Option<S3Location>,
}

impl cfn_resources::CfnResource for Input {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_catalog_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Parameters that are used as inputs for various recipe actions. The parameters are       specific to the context in which they're used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecipeParameters {
    ///
    /// The name of an aggregation function to apply.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregateFunction")]
    pub aggregate_function: Option<String>,

    ///
    /// The number of digits used in a counting system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    pub base: Option<String>,

    ///
    /// A case statement associated with a recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseStatement")]
    pub case_statement: Option<String>,

    ///
    /// A category map used for one-hot encoding.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryMap")]
    pub category_map: Option<String>,

    ///
    /// Characters to remove from a step that applies one-hot encoding or tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CharsToRemove")]
    pub chars_to_remove: Option<String>,

    ///
    /// Remove any non-word non-punctuation character.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollapseConsecutiveWhitespace")]
    pub collapse_consecutive_whitespace: Option<String>,

    ///
    /// The data type of the column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDataType")]
    pub column_data_type: Option<String>,

    ///
    /// A range of columns to which a step is applied.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnRange")]
    pub column_range: Option<String>,

    ///
    /// The number of times a string needs to be repeated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<String>,

    ///
    /// One or more characters that can be substituted or removed, depending on the       context.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomCharacters")]
    pub custom_characters: Option<String>,

    ///
    /// A list of words to ignore in a step that applies word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomStopWords")]
    pub custom_stop_words: Option<String>,

    ///
    /// A list of custom values to use in a step that requires that you provide a value to       finish the operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    pub custom_value: Option<String>,

    ///
    /// A list of the dataset columns included in a project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetsColumns")]
    pub datasets_columns: Option<String>,

    ///
    /// A value that specifies how many units of time to add or subtract for a date math       operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateAddValue")]
    pub date_add_value: Option<String>,

    ///
    /// A date format to apply to a date.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormat")]
    pub date_time_format: Option<String>,

    ///
    /// A set of parameters associated with a datetime.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeParameters")]
    pub date_time_parameters: Option<String>,

    ///
    /// Determines whether unmapped rows in a categorical mapping should be deleted
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOtherRows")]
    pub delete_other_rows: Option<String>,

    ///
    /// The delimiter to use when parsing separated values in a text file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

    ///
    /// The end pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndPattern")]
    pub end_pattern: Option<String>,

    ///
    /// The end position to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndPosition")]
    pub end_position: Option<String>,

    ///
    /// The end value to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndValue")]
    pub end_value: Option<String>,

    ///
    /// A list of word contractions and what they expand to. For eample:         can't; cannot; can         not.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpandContractions")]
    pub expand_contractions: Option<String>,

    ///
    /// The exponent to apply in an exponential operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exponent")]
    pub exponent: Option<String>,

    ///
    /// A value that represents FALSE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseString")]
    pub false_string: Option<String>,

    ///
    /// Specifies options to apply to the GROUP BY used in an aggregation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupByAggFunctionOptions")]
    pub group_by_agg_function_options: Option<String>,

    ///
    /// The columns to use in the GROUP BY clause.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupByColumns")]
    pub group_by_columns: Option<String>,

    ///
    /// A list of columns to hide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HiddenColumns")]
    pub hidden_columns: Option<String>,

    ///
    /// Indicates that lower and upper case letters are treated equally.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreCase")]
    pub ignore_case: Option<String>,

    ///
    /// Indicates if this column is participating in a split transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeInSplit")]
    pub include_in_split: Option<String>,

    ///
    /// The input location to load the dataset from - Amazon S3 or AWS Glue Data Catalog.
    ///
    /// Required: No
    ///
    /// Type: Input
    ///
    /// Update requires: No interruption
    #[serde(rename = "Input")]
    pub input: Option<Input>,

    ///
    /// The number of characters to split by.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<String>,

    ///
    /// Indicates if the content is text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsText")]
    pub is_text: Option<String>,

    ///
    /// The keys or columns involved in a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JoinKeys")]
    pub join_keys: Option<String>,

    ///
    /// The type of join to use, for example, INNER JOIN, OUTER       JOIN, and so on.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JoinType")]
    pub join_type: Option<String>,

    ///
    /// The columns on the left side of the join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LeftColumns")]
    pub left_columns: Option<String>,

    ///
    /// The number of times to perform split or replaceBy in a       string
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limit")]
    pub limit: Option<String>,

    ///
    /// The lower boundary for a value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowerBound")]
    pub lower_bound: Option<String>,

    ///
    /// The type of mappings to apply to construct a new dynamic frame.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapType")]
    pub map_type: Option<String>,

    ///
    /// Determines the manner in which mode value is calculated, in case there is more than       one mode value. Valid values: NONE | AVERAGE |         MINIMUM | MAXIMUM
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModeType")]
    pub mode_type: Option<String>,

    ///
    /// Specifies whether JSON input contains embedded new line characters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiLine")]
    pub multi_line: Option<bool>,

    ///
    /// The number of rows to consider in a window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRows")]
    pub num_rows: Option<String>,

    ///
    /// The number of rows to consider after the current row in a window
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRowsAfter")]
    pub num_rows_after: Option<String>,

    ///
    /// The number of rows to consider before the current row in a window
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRowsBefore")]
    pub num_rows_before: Option<String>,

    ///
    /// A column to sort the results by.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderByColumn")]
    pub order_by_column: Option<String>,

    ///
    /// The columns to sort the results by.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderByColumns")]
    pub order_by_columns: Option<String>,

    ///
    /// The value to assign to unmapped cells, in categorical mapping
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Other")]
    pub other: Option<String>,

    ///
    /// The pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,

    ///
    /// The starting pattern to split between.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOption1")]
    pub pattern_option1: Option<String>,

    ///
    /// The ending pattern to split between.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOption2")]
    pub pattern_option2: Option<String>,

    ///
    /// For splitting by multiple delimiters: A JSON-encoded string that lists the patterns in       the format. For example:       [{\"pattern\":\"1\",\"includeInSplit\":true}]
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOptions")]
    pub pattern_options: Option<String>,

    ///
    /// The size of the rolling window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: Option<String>,

    ///
    /// The character index within a string
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: Option<String>,

    ///
    /// If true, removes all of the following characters: .       .!       .,       .?
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllPunctuation")]
    pub remove_all_punctuation: Option<String>,

    ///
    /// If true, removes all single quotes and double quotes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllQuotes")]
    pub remove_all_quotes: Option<String>,

    ///
    /// If true, removes all whitespaces from the value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllWhitespace")]
    pub remove_all_whitespace: Option<String>,

    ///
    /// If true, removes all chraracters specified by         CustomCharacters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveCustomCharacters")]
    pub remove_custom_characters: Option<String>,

    ///
    /// If true, removes all chraracters specified by CustomValue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveCustomValue")]
    pub remove_custom_value: Option<String>,

    ///
    /// If true, removes the following characters if they occur at the start or       end of the value: .       !       ,       ?
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingPunctuation")]
    pub remove_leading_and_trailing_punctuation: Option<String>,

    ///
    /// If true, removes single quotes and double quotes from the beginning and       end of the value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingQuotes")]
    pub remove_leading_and_trailing_quotes: Option<String>,

    ///
    /// If true, removes all whitespaces from the beginning and end of the       value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingWhitespace")]
    pub remove_leading_and_trailing_whitespace: Option<String>,

    ///
    /// If true, removes all uppercase and lowercase alphabetic characters (A       through Z; a through z).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLetters")]
    pub remove_letters: Option<String>,

    ///
    /// If true, removes all numeric characters (0 through 9).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveNumbers")]
    pub remove_numbers: Option<String>,

    ///
    /// If true, the source column will be removed after un-nesting that column.       (Used with nested column types, such as Map, Struct, or Array.)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSourceColumn")]
    pub remove_source_column: Option<String>,

    ///
    /// If true, removes all of the following characters: ! " # $ % & '         ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSpecialCharacters")]
    pub remove_special_characters: Option<String>,

    ///
    /// The columns on the right side of a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightColumns")]
    pub right_columns: Option<String>,

    ///
    /// The number of rows in the sample.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleSize")]
    pub sample_size: Option<String>,

    ///
    /// The sampling type to apply to the dataset. Valid values: FIRST_N |         LAST_N | RANDOM
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleType")]
    pub sample_type: Option<String>,

    ///
    /// A object value to indicate the second dataset used in a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondInput")]
    pub second_input: Option<String>,

    ///
    /// A list of secondary inputs in a UNION transform
    ///
    /// Required: No
    ///
    /// Type: List of SecondaryInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryInputs")]
    pub secondary_inputs: Option<Vec<SecondaryInput>>,

    ///
    /// One or more sheet numbers in the Excel file, which will be included in a       dataset.
    ///
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetIndexes")]
    pub sheet_indexes: Option<Vec<i64>>,

    ///
    /// Oone or more named sheets in the Excel file, which will be included in a dataset.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SheetNames")]
    pub sheet_names: Option<Vec<String>>,

    ///
    /// A source column needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumn")]
    pub source_column: Option<String>,

    ///
    /// A source column needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumn1")]
    pub source_column1: Option<String>,

    ///
    /// A source column needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumn2")]
    pub source_column2: Option<String>,

    ///
    /// A list of source columns needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumns")]
    pub source_columns: Option<String>,

    ///
    /// The index number of the first column used by an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartColumnIndex")]
    pub start_column_index: Option<String>,

    ///
    /// The starting pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartPattern")]
    pub start_pattern: Option<String>,

    ///
    /// The starting position to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartPosition")]
    pub start_position: Option<String>,

    ///
    /// The starting value to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartValue")]
    pub start_value: Option<String>,

    ///
    /// Indicates this operation uses stems and lemmas (base words) for word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StemmingMode")]
    pub stemming_mode: Option<String>,

    ///
    /// The total number of transforms in this recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepCount")]
    pub step_count: Option<String>,

    ///
    /// The index ID of a step.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepIndex")]
    pub step_index: Option<String>,

    ///
    /// Indicates this operation uses stop words as part of word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StopWordsMode")]
    pub stop_words_mode: Option<String>,

    ///
    /// The resolution strategy to apply in resolving ambiguities.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Strategy")]
    pub strategy: Option<String>,

    ///
    /// The column targeted by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetColumn")]
    pub target_column: Option<String>,

    ///
    /// The names to give columns altered by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetColumnNames")]
    pub target_column_names: Option<String>,

    ///
    /// The date format to convert to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDateFormat")]
    pub target_date_format: Option<String>,

    ///
    /// The index number of an object that is targeted by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIndex")]
    pub target_index: Option<String>,

    ///
    /// The current timezone that you want to use for dates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,

    ///
    /// A regex expression to use when splitting text into terms, also called words or       tokens.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenizerPattern")]
    pub tokenizer_pattern: Option<String>,

    ///
    /// A value to use to represent TRUE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrueString")]
    pub true_string: Option<String>,

    ///
    /// The language that's used in the user-defined function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UdfLang")]
    pub udf_lang: Option<String>,

    ///
    /// Specifies a unit of time. For example: MINUTES; SECONDS;         HOURS; etc.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Units")]
    pub units: Option<String>,

    ///
    /// Cast columns as rows, so that each value is a different row in a single column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnpivotColumn")]
    pub unpivot_column: Option<String>,

    ///
    /// The upper boundary for a value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpperBound")]
    pub upper_bound: Option<String>,

    ///
    /// Create a new container to hold a dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseNewDataFrame")]
    pub use_new_data_frame: Option<String>,

    ///
    /// A static value that can be used in a comparison, a substitution, or in another       context-specific way. A Value can be a number, string, or other datatype,       depending on the recipe action in which it's used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

    ///
    /// A value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value1")]
    pub value1: Option<String>,

    ///
    /// A value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value2")]
    pub value2: Option<String>,

    ///
    /// The column that is provided as a value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueColumn")]
    pub value_column: Option<String>,

    ///
    /// The subset of rows currently available for viewing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewFrame")]
    pub view_frame: Option<String>,
}

impl cfn_resources::CfnResource for RecipeParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.input.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents a single step from a DataBrew recipe to be performed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecipeStep {
    ///
    /// The particular action to be performed in the recipe step.
    ///
    /// Required: Yes
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Action,

    ///
    /// One or more conditions that must be met for the recipe step to succeed.
    ///
    /// NoteAll of the conditions in the array must be met. In other words, all of the         conditions must be combined using a logical AND operation.
    ///
    /// Required: No
    ///
    /// Type: List of ConditionExpression
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConditionExpressions")]
    pub condition_expressions: Option<Vec<ConditionExpression>>,
}

impl cfn_resources::CfnResource for RecipeStep {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

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
    pub bucket: String,

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
}

impl cfn_resources::CfnResource for S3Location {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.bucket;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'bucket'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.bucket;

        if the_val.len() < 3 as _ {
            return Err(format!(
                "Min validation failed on field 'bucket'. {} is less than 3",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.key {
            if the_val.len() > 1280 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 1280",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.key {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents secondary inputs in a UNION transform.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecondaryInput {
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
}

impl cfn_resources::CfnResource for SecondaryInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_catalog_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
