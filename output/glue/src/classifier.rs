

/// The AWS::Glue::Classifier resource creates an AWS Glue classifier that       categorizes data sources and specifies schemas. For more information, see Adding Classifiers to         a Crawler and Classifier Structure in the AWS Glue Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClassifier {


    /// 
    /// A classifier for JSON content.
    /// 
    /// Required: Conditional
    ///
    /// Type: JsonClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonClassifier")]
    pub json_classifier: Option<JsonClassifier>,


    /// 
    /// A classifier that uses grok.
    /// 
    /// Required: Conditional
    ///
    /// Type: GrokClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrokClassifier")]
    pub grok_classifier: Option<GrokClassifier>,


    /// 
    /// A classifier for XML content.
    /// 
    /// Required: Conditional
    ///
    /// Type: XMLClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "XMLClassifier")]
    pub xmlclassifier: Option<XMLClassifier>,


    /// 
    /// A classifier for comma-separated values (CSV).
    /// 
    /// Required: Conditional
    ///
    /// Type: CsvClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvClassifier")]
    pub csv_classifier: Option<CsvClassifier>,

}

impl cfn_resources::CfnResource for CfnClassifier {
    fn type_string() -> &'static str {
        "AWS::Glue::Classifier"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A classifier that uses grok patterns.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrokClassifier {


    /// 
    /// The name of the classifier.
    /// 
    /// Required: No
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Optional custom grok patterns defined by this classifier. For more information, see       custom patterns in Writing Custom       Classifiers.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 16000
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomPatterns")]
    pub custom_patterns: Option<String>,


    /// 
    /// An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, and    so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classification")]
    pub classification: String,


    /// 
    /// The grok pattern applied to a data store by this classifier. For more information, see       built-in patterns in Writing Custom       Classifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,

}


/// A classifier for custom CSV content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsvClassifier {


    /// 
    /// A list of strings representing column names.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    pub header: Option<Vec<String>>,


    /// 
    /// A custom symbol to denote what separates each column entry in the row.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Pattern: [^\r\n]
    ///
    /// Update requires: No interruption
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,


    /// 
    /// Indicates whether the CSV file contains a header.
    /// 
    /// A value of UNKNOWN specifies that the classifier will detect whether the CSV file contains headings.
    /// 
    /// A value of PRESENT specifies that the CSV file contains headings.
    /// 
    /// A value of ABSENT specifies that the CSV file does not contain headings.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ABSENT | PRESENT | UNKNOWN
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<String>,


    /// 
    /// Specifies not to trim values before identifying the type of column values. The default    value is true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableValueTrimming")]
    pub disable_value_trimming: Option<bool>,


    /// 
    /// A custom symbol to denote what combines content into a single column value. It must be    different from the column delimiter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1
    ///
    /// Pattern: [^\r\n]
    ///
    /// Update requires: No interruption
    #[serde(rename = "QuoteSymbol")]
    pub quote_symbol: Option<String>,


    /// 
    /// Enables the processing of files that contain only one column.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowSingleColumn")]
    pub allow_single_column: Option<bool>,


    /// 
    /// The name of the classifier.
    /// 
    /// Required: No
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
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// A classifier for JSON content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonClassifier {


    /// 
    /// The name of the classifier.
    /// 
    /// Required: No
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A JsonPath string defining the JSON data for the classifier to classify.       AWS Glue supports a subset of JsonPath, as described in Writing JsonPath         Custom Classifiers.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonPath")]
    pub json_path: String,

}


/// A classifier for XML content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct XMLClassifier {


    /// 
    /// The XML tag designating the element that contains each record in an XML document being    parsed. This can't identify a self-closing element (closed by />). An empty    row element that contains only attributes can be parsed as long as it ends with a closing tag    (for example, <row item_a="A" item_b="B"></row> is okay, but     <row item_a="A" item_b="B" /> is not).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowTag")]
    pub row_tag: String,


    /// 
    /// The name of the classifier.
    /// 
    /// Required: No
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
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// An identifier of the data format that the classifier matches.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classification")]
    pub classification: String,

}
