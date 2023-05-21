/// The AWS::Glue::Classifier resource creates an AWS Glue classifier that       categorizes data sources and specifies schemas. For more information, see Adding Classifiers to         a Crawler and Classifier Structure in the AWS Glue Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClassifier {
    ///
    /// A classifier for comma-separated values (CSV).
    ///
    /// Required: Conditional
    ///
    /// Type: CsvClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_classifier: Option<CsvClassifier>,

    ///
    /// A classifier that uses grok.
    ///
    /// Required: Conditional
    ///
    /// Type: GrokClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrokClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<GrokClassifier>,

    ///
    /// A classifier for JSON content.
    ///
    /// Required: Conditional
    ///
    /// Type: JsonClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_classifier: Option<JsonClassifier>,

    ///
    /// A classifier for XML content.
    ///
    /// Required: Conditional
    ///
    /// Type: XMLClassifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "XMLClassifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmlclassifier: Option<XMLClassifier>,
}

impl cfn_resources::CfnResource for CfnClassifier {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Classifier"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csv_classifier
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.grok_classifier
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.json_classifier
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.xmlclassifier
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A classifier for custom CSV content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsvClassifier {
    ///
    /// Enables the processing of files that contain only one column.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowSingleColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_single_column: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<CsvClassifierContainsHeaderEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<cfn_resources::StrVal>,

    ///
    /// Specifies not to trim values before identifying the type of column values. The default    value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableValueTrimming")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_value_trimming: Option<bool>,

    ///
    /// A list of strings representing column names.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_symbol: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CsvClassifierContainsHeaderEnum {
    /// ABSENT
    #[serde(rename = "ABSENT")]
    Absent,

    /// PRESENT
    #[serde(rename = "PRESENT")]
    Present,

    /// UNKNOWN
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for CsvClassifierContainsHeaderEnum {
    fn default() -> Self {
        CsvClassifierContainsHeaderEnum::Absent
    }
}

impl cfn_resources::CfnResource for CsvClassifier {
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

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.quote_symbol {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1 as _ {
                    return Err(format!(
                        "Max validation failed on field 'quote_symbol'. {} is greater than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.quote_symbol {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'quote_symbol'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A classifier that uses grok patterns.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrokClassifier {
    ///
    /// An identifier of the data format that the classifier matches, such as Twitter, JSON, Omniture logs, and    so on.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classification")]
    pub classification: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_patterns: Option<cfn_resources::StrVal>,

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
    pub grok_pattern: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GrokClassifier {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_patterns {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 16000 as _ {
                    return Err(format!("Max validation failed on field 'custom_patterns'. {} is greater than 16000", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.custom_patterns {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_patterns'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.grok_pattern;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'grok_pattern'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.grok_pattern;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'grok_pattern'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A classifier for JSON content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonClassifier {
    ///
    /// A JsonPath string defining the JSON data for the classifier to classify.       AWS Glue supports a subset of JsonPath, as described in Writing JsonPath         Custom Classifiers.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonPath")]
    pub json_path: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for JsonClassifier {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A classifier for XML content.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct XMLClassifier {
    ///
    /// An identifier of the data format that the classifier matches.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Classification")]
    pub classification: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The XML tag designating the element that contains each record in an XML document being    parsed. This can't identify a self-closing element (closed by />). An empty    row element that contains only attributes can be parsed as long as it ends with a closing tag    (for example, <row item_a="A" item_b="B"></row> is okay, but     <row item_a="A" item_b="B" /> is not).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RowTag")]
    pub row_tag: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for XMLClassifier {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
