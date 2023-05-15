
pub mod cfn_analyzer {

#[derive(serde::Serialize, Default)]
pub struct CfnAnalyzer {
    /// The type of the analyzer, must be ACCOUNT or ORGANIZATION
    #[serde(rename = "Type")]
    pub ty: String,
    /// Analyzer name
    #[serde(rename = "AnalyzerName")]
    pub analyzer_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of ArchiveRule
    #[serde(rename = "ArchiveRules")]
    pub archive_rules: Option<Vec<ArchiveRule>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveRule {
    #[serde(rename = "Filter")]
    pub filter: Vec<Filter>,
    #[serde(rename = "RuleName")]
    pub rule_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "Exists")]
    pub exists: Option<bool>,
    #[serde(rename = "Neq")]
    pub neq: Option<Vec<String>>,
    #[serde(rename = "Contains")]
    pub contains: Option<Vec<String>>,
    #[serde(rename = "Eq")]
    pub eq: Option<Vec<String>>,
    #[serde(rename = "Property")]
    pub property: String,

}


}
