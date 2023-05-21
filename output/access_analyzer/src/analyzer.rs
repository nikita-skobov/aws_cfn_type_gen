

/// The AWS::AccessAnalyzer::Analyzer resource specifies a new analyzer. The analyzer is an object that represents the IAM Access Analyzer feature. An analyzer is required      for Access Analyzer to become operational.
#[derive(Default, serde::Serialize)]
pub struct CfnAnalyzer {


    /// 
    /// The name of the analyzer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AnalyzerName")]
    pub analyzer_name: Option<String>,


    /// 
    /// The tags to apply to the analyzer.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the archive rules to add for the analyzer.
    ///
    /// Required: No
    ///
    /// Type: List of ArchiveRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveRules")]
    pub archive_rules: Option<Vec<ArchiveRule>>,


    /// 
    /// The type represents the zone of trust for the analyzer.
    ///
    /// Allowed Values: ACCOUNT | ORGANIZATION
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

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


/// The criteria for an archive rule.
#[derive(Default, serde::Serialize)]
pub struct ArchiveRule {


    /// The name of the archive rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleName")]
    pub rule_name: String,


    /// The criteria for the rule.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filter")]
    pub filter: Vec<Filter>,

}


/// The criteria that defines the rule.
///
/// To learn about filter keys that you can use to create an archive rule, see AWS Identity and Access Management Access Analyzer filter keys in the     AWS Identity and Access Management User Guide.
#[derive(Default, serde::Serialize)]
pub struct Filter {


    /// A "not equal" condition to match for the rule.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Neq")]
    pub neq: Option<Vec<String>>,


    /// The property used to define the criteria in the filter for the rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Property")]
    pub property: String,


    /// A "contains" condition to match for the rule.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Contains")]
    pub contains: Option<Vec<String>>,


    /// An "exists" condition to match for the rule.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exists")]
    pub exists: Option<bool>,


    /// An "equals" condition to match for the rule.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Eq")]
    pub eq: Option<Vec<String>>,

}
