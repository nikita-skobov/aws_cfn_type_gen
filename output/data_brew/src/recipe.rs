/// Specifies a new AWS Glue DataBrew transformation recipe.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnRecipe {
    fn type_string(&self) -> &'static str {
        "AWS::DataBrew::Recipe"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
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

        Ok(())
    }
}

/// Represents a transformation and associated parameters that are used to apply a change       to an AWS Glue DataBrew dataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub operation: cfn_resources::StrVal,

    ///
    /// Contextual parameters for the transformation.
    ///
    /// Required: No
    ///
    /// Type: RecipeParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RecipeParameters>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub condition: cfn_resources::StrVal,

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
    pub target_column: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ConditionExpression {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.condition;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'condition'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.condition;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'condition'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_column;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_column'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_column;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_column'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'value'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Represents how metadata stored in the AWS Glue Data Catalog is defined in a DataBrew       dataset.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    /// Represents an Amazon location where DataBrew can store intermediate results.
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

/// The Input property type specifies Property description not available. for an AWS::DataBrew::Recipe.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Input {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DataCatalogInputDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCatalogInputDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_catalog_input_definition: Option<DataCatalogInputDefinition>,

    /// Property description not available.
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

        self.s3_input_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Parameters that are used as inputs for various recipe actions. The parameters are       specific to the context in which they're used.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_function: Option<cfn_resources::StrVal>,

    ///
    /// The number of digits used in a counting system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<cfn_resources::StrVal>,

    ///
    /// A case statement associated with a recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseStatement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_statement: Option<cfn_resources::StrVal>,

    ///
    /// A category map used for one-hot encoding.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CategoryMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_map: Option<cfn_resources::StrVal>,

    ///
    /// Characters to remove from a step that applies one-hot encoding or tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CharsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chars_to_remove: Option<cfn_resources::StrVal>,

    ///
    /// Remove any non-word non-punctuation character.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CollapseConsecutiveWhitespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_consecutive_whitespace: Option<cfn_resources::StrVal>,

    ///
    /// The data type of the column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnDataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_data_type: Option<cfn_resources::StrVal>,

    ///
    /// A range of columns to which a step is applied.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_range: Option<cfn_resources::StrVal>,

    ///
    /// The number of times a string needs to be repeated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<cfn_resources::StrVal>,

    ///
    /// One or more characters that can be substituted or removed, depending on the       context.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_characters: Option<cfn_resources::StrVal>,

    ///
    /// A list of words to ignore in a step that applies word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomStopWords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_stop_words: Option<cfn_resources::StrVal>,

    ///
    /// A list of custom values to use in a step that requires that you provide a value to       finish the operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<cfn_resources::StrVal>,

    ///
    /// A list of the dataset columns included in a project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetsColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets_columns: Option<cfn_resources::StrVal>,

    ///
    /// A value that specifies how many units of time to add or subtract for a date math       operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateAddValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_add_value: Option<cfn_resources::StrVal>,

    ///
    /// A date format to apply to a date.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<cfn_resources::StrVal>,

    ///
    /// A set of parameters associated with a datetime.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DateTimeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_parameters: Option<cfn_resources::StrVal>,

    ///
    /// Determines whether unmapped rows in a categorical mapping should be deleted
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOtherRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_other_rows: Option<cfn_resources::StrVal>,

    ///
    /// The delimiter to use when parsing separated values in a text file.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<cfn_resources::StrVal>,

    ///
    /// The end pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_pattern: Option<cfn_resources::StrVal>,

    ///
    /// The end position to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_position: Option<cfn_resources::StrVal>,

    ///
    /// The end value to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<cfn_resources::StrVal>,

    ///
    /// A list of word contractions and what they expand to. For eample:         can't; cannot; can         not.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpandContractions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_contractions: Option<cfn_resources::StrVal>,

    ///
    /// The exponent to apply in an exponential operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exponent: Option<cfn_resources::StrVal>,

    ///
    /// A value that represents FALSE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FalseString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub false_string: Option<cfn_resources::StrVal>,

    ///
    /// Specifies options to apply to the GROUP BY used in an aggregation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupByAggFunctionOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_agg_function_options: Option<cfn_resources::StrVal>,

    ///
    /// The columns to use in the GROUP BY clause.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupByColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_columns: Option<cfn_resources::StrVal>,

    ///
    /// A list of columns to hide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HiddenColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_columns: Option<cfn_resources::StrVal>,

    ///
    /// Indicates that lower and upper case letters are treated equally.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IgnoreCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<cfn_resources::StrVal>,

    ///
    /// Indicates if this column is participating in a split transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeInSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_split: Option<cfn_resources::StrVal>,

    ///
    /// The input location to load the dataset from - Amazon S3 or AWS Glue Data Catalog.
    ///
    /// Required: No
    ///
    /// Type: Input
    ///
    /// Update requires: No interruption
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<cfn_resources::StrVal>,

    ///
    /// Indicates if the content is text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_text: Option<cfn_resources::StrVal>,

    ///
    /// The keys or columns involved in a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JoinKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_keys: Option<cfn_resources::StrVal>,

    ///
    /// The type of join to use, for example, INNER JOIN, OUTER       JOIN, and so on.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JoinType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_type: Option<cfn_resources::StrVal>,

    ///
    /// The columns on the left side of the join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LeftColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_columns: Option<cfn_resources::StrVal>,

    ///
    /// The number of times to perform split or replaceBy in a       string
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<cfn_resources::StrVal>,

    ///
    /// The lower boundary for a value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<cfn_resources::StrVal>,

    ///
    /// The type of mappings to apply to construct a new dynamic frame.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MapType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_type: Option<cfn_resources::StrVal>,

    ///
    /// Determines the manner in which mode value is calculated, in case there is more than       one mode value. Valid values: NONE | AVERAGE |         MINIMUM | MAXIMUM
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether JSON input contains embedded new line characters.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MultiLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows: Option<cfn_resources::StrVal>,

    ///
    /// The number of rows to consider after the current row in a window
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRowsAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows_after: Option<cfn_resources::StrVal>,

    ///
    /// The number of rows to consider before the current row in a window
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumRowsBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_rows_before: Option<cfn_resources::StrVal>,

    ///
    /// A column to sort the results by.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderByColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by_column: Option<cfn_resources::StrVal>,

    ///
    /// The columns to sort the results by.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrderByColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by_columns: Option<cfn_resources::StrVal>,

    ///
    /// The value to assign to unmapped cells, in categorical mapping
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Other")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<cfn_resources::StrVal>,

    ///
    /// The pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<cfn_resources::StrVal>,

    ///
    /// The starting pattern to split between.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOption1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_option1: Option<cfn_resources::StrVal>,

    ///
    /// The ending pattern to split between.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOption2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_option2: Option<cfn_resources::StrVal>,

    ///
    /// For splitting by multiple delimiters: A JSON-encoded string that lists the patterns in       the format. For example:       [{\"pattern\":\"1\",\"includeInSplit\":true}]
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PatternOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_options: Option<cfn_resources::StrVal>,

    ///
    /// The size of the rolling window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<cfn_resources::StrVal>,

    ///
    /// The character index within a string
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all of the following characters: .       .!       .,       .?
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllPunctuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_punctuation: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all single quotes and double quotes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllQuotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_quotes: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all whitespaces from the value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAllWhitespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_whitespace: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all chraracters specified by         CustomCharacters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveCustomCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_custom_characters: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all chraracters specified by CustomValue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveCustomValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_custom_value: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes the following characters if they occur at the start or       end of the value: .       !       ,       ?
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingPunctuation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_leading_and_trailing_punctuation: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes single quotes and double quotes from the beginning and       end of the value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingQuotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_leading_and_trailing_quotes: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all whitespaces from the beginning and end of the       value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLeadingAndTrailingWhitespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_leading_and_trailing_whitespace: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all uppercase and lowercase alphabetic characters (A       through Z; a through z).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveLetters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_letters: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all numeric characters (0 through 9).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_numbers: Option<cfn_resources::StrVal>,

    ///
    /// If true, the source column will be removed after un-nesting that column.       (Used with nested column types, such as Map, Struct, or Array.)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSourceColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_source_column: Option<cfn_resources::StrVal>,

    ///
    /// If true, removes all of the following characters: ! " # $ % & '         ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSpecialCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_special_characters: Option<cfn_resources::StrVal>,

    ///
    /// The columns on the right side of a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RightColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_columns: Option<cfn_resources::StrVal>,

    ///
    /// The number of rows in the sample.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<cfn_resources::StrVal>,

    ///
    /// The sampling type to apply to the dataset. Valid values: FIRST_N |         LAST_N | RANDOM
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_type: Option<cfn_resources::StrVal>,

    ///
    /// A object value to indicate the second dataset used in a join.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_input: Option<cfn_resources::StrVal>,

    ///
    /// A list of secondary inputs in a UNION transform
    ///
    /// Required: No
    ///
    /// Type: List of SecondaryInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecondaryInputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<cfn_resources::StrVal>,

    ///
    /// A source column needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumn1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column1: Option<cfn_resources::StrVal>,

    ///
    /// A source column needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumn2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column2: Option<cfn_resources::StrVal>,

    ///
    /// A list of source columns needed for an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_columns: Option<cfn_resources::StrVal>,

    ///
    /// The index number of the first column used by an operation, step, or transform.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartColumnIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column_index: Option<cfn_resources::StrVal>,

    ///
    /// The starting pattern to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pattern: Option<cfn_resources::StrVal>,

    ///
    /// The starting position to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<cfn_resources::StrVal>,

    ///
    /// The starting value to locate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<cfn_resources::StrVal>,

    ///
    /// Indicates this operation uses stems and lemmas (base words) for word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StemmingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stemming_mode: Option<cfn_resources::StrVal>,

    ///
    /// The total number of transforms in this recipe.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_count: Option<cfn_resources::StrVal>,

    ///
    /// The index ID of a step.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_index: Option<cfn_resources::StrVal>,

    ///
    /// Indicates this operation uses stop words as part of word tokenization.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StopWordsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words_mode: Option<cfn_resources::StrVal>,

    ///
    /// The resolution strategy to apply in resolving ambiguities.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<cfn_resources::StrVal>,

    ///
    /// The column targeted by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_column: Option<cfn_resources::StrVal>,

    ///
    /// The names to give columns altered by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetColumnNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_column_names: Option<cfn_resources::StrVal>,

    ///
    /// The date format to convert to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date_format: Option<cfn_resources::StrVal>,

    ///
    /// The index number of an object that is targeted by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_index: Option<cfn_resources::StrVal>,

    ///
    /// The current timezone that you want to use for dates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<cfn_resources::StrVal>,

    ///
    /// A regex expression to use when splitting text into terms, also called words or       tokens.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TokenizerPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenizer_pattern: Option<cfn_resources::StrVal>,

    ///
    /// A value to use to represent TRUE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrueString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_string: Option<cfn_resources::StrVal>,

    ///
    /// The language that's used in the user-defined function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UdfLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udf_lang: Option<cfn_resources::StrVal>,

    ///
    /// Specifies a unit of time. For example: MINUTES; SECONDS;         HOURS; etc.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Units")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<cfn_resources::StrVal>,

    ///
    /// Cast columns as rows, so that each value is a different row in a single column.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnpivotColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpivot_column: Option<cfn_resources::StrVal>,

    ///
    /// The upper boundary for a value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UpperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<cfn_resources::StrVal>,

    ///
    /// Create a new container to hold a dataset.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseNewDataFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_new_data_frame: Option<cfn_resources::StrVal>,

    ///
    /// A static value that can be used in a comparison, a substitution, or in another       context-specific way. A Value can be a number, string, or other datatype,       depending on the recipe action in which it's used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,

    ///
    /// A value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value1: Option<cfn_resources::StrVal>,

    ///
    /// A value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value2: Option<cfn_resources::StrVal>,

    ///
    /// The column that is provided as a value that's used by this operation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_column: Option<cfn_resources::StrVal>,

    ///
    /// The subset of rows currently available for viewing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ViewFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_frame: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RecipeParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.input.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Represents a single step from a DataBrew recipe to be performed.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expressions: Option<Vec<ConditionExpression>>,
}

impl cfn_resources::CfnResource for RecipeStep {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action.validate()?;

        Ok(())
    }
}

/// Represents an Amazon S3 location (bucket name, bucket owner, and object key) where DataBrew can read       input data, or write output from a job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

/// Represents secondary inputs in a UNION transform.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_definition: Option<S3Location>,
}

impl cfn_resources::CfnResource for SecondaryInput {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
