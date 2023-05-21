/// The RegexPatternSet specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as B[a@]dB[o0]t. You can then configure AWS WAF to reject those requests.
///
/// Note that you can only create regex pattern sets using a AWS CloudFormation template. To add the regex pattern sets created through AWS CloudFormation to a RegexMatchSet, use the AWS WAF console, API, or command line interface (CLI). For more information, see      UpdateRegexMatchSet.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRegexPatternSet {
    ///
    /// A friendly name or description of the AWS::WAFRegional::RegexPatternSet. You can't change Name after you create a RegexPatternSet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Specifies the regular expression (regex) patterns that you want AWS WAF to search for, such as B[a@]dB[o0]t.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegexPatternStrings")]
    pub regex_pattern_strings: Vec<String>,
}

impl cfn_resources::CfnResource for CfnRegexPatternSet {
    fn type_string(&self) -> &'static str {
        "AWS::WAFRegional::RegexPatternSet"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 128",
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

        let the_val = &self.regex_pattern_strings;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'regex_pattern_strings'. {} is greater than 10",
                the_val.len()
            ));
        }

        Ok(())
    }
}
