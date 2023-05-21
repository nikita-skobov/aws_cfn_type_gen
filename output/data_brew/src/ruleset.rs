

/// Specifies a new ruleset that can be used in a profile job to validate the data quality       of a dataset.
#[derive(Default, serde::Serialize)]
pub struct CfnRuleset {


    /// 
    /// The Amazon Resource Name (ARN) of a resource (dataset) that the ruleset is associated       with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetArn")]
    pub target_arn: String,


    /// 
    /// The name of the ruleset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Contains metadata about the ruleset.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see       Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The description of the ruleset.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// The threshold used with a non-aggregate check expression. The non-aggregate check       expression will be applied to each row in a specific column. Then the threshold will be       used to determine whether the validation succeeds.
#[derive(Default, serde::Serialize)]
pub struct Threshold {


    /// 
    /// The type of a threshold. Used for comparison of an actual count of rows that satisfy       the rule to the threshold value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// Unit of threshold value. Can be either a COUNT or PERCENTAGE of the full sample size       used for validation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,


    /// 
    /// The value of a threshold.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: f64,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// A key-value pair to associate an expression's substitution variable names with their       values.
#[derive(Default, serde::Serialize)]
pub struct SubstitutionValue {


    /// 
    /// Variable name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueReference")]
    pub value_reference: String,


    /// 
    /// Value or column name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// Represents a single data quality requirement that should be validated in the       scope of this dataset.
#[derive(Default, serde::Serialize)]
pub struct Rule {


    /// 
    /// The threshold used with a non-aggregate check expression. Non-aggregate check       expressions will be applied to each row in a specific column, and the threshold will be       used to determine whether the validation succeeds.
    /// 
    /// Required: No
    ///
    /// Type: Threshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: Option<Threshold>,


    /// 
    /// The expression which includes column references, condition names followed by variable       references, possibly grouped and combined with other conditions. For example,         (:col1 starts_with :prefix1 or :col1 starts_with :prefix2) and (:col1         ends_with :suffix1 or :col1 ends_with :suffix2). Column and value references       are substitution variables that should start with the ':' symbol. Depending on the       context, substitution variables' values can be either an actual value or a column name.       These values are defined in the SubstitutionMap. If a CheckExpression starts with a       column reference, then ColumnSelectors in the rule should be null. If ColumnSelectors       has been defined, then there should be no columnn reference in the left side of a       condition, for example, is_between :val1 and :val2.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CheckExpression")]
    pub check_expression: String,


    /// 
    /// List of column selectors. Selectors can be used to select columns using a name or       regular expression from the dataset. Rule will be applied to selected columns.
    /// 
    /// Required: No
    ///
    /// Type: List of ColumnSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnSelectors")]
    pub column_selectors: Option<Vec<ColumnSelector>>,


    /// 
    /// The map of substitution variable names to their values used in a check expression.       Variable names should start with a ':' (colon). Variable values can either be actual       values or column names. To differentiate between the two, column names should be       enclosed in backticks, for example, ":col1": "`Column A`".
    /// 
    /// Required: No
    ///
    /// Type: List of SubstitutionValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubstitutionMap")]
    pub substitution_map: Option<Vec<SubstitutionValue>>,


    /// 
    /// A value that specifies whether the rule is disabled. Once a rule is disabled, a       profile job will not validate it during a job run. Default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Disabled")]
    pub disabled: Option<bool>,


    /// 
    /// The name of the rule.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Selector of a column from a dataset for profile job configuration.       One selector includes either a column name or a regular expression.
#[derive(Default, serde::Serialize)]
pub struct ColumnSelector {


    /// 
    /// The name of a column from a dataset.
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A regular expression for selecting a column from a dataset.
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
    #[serde(rename = "Regex")]
    pub regex: Option<String>,

}
