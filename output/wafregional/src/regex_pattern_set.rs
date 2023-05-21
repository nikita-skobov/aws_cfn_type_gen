

/// The RegexPatternSet specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as B[a@]dB[o0]t. You can then configure AWS WAF to reject those requests.
///
/// Note that you can only create regex pattern sets using a AWS CloudFormation template. To add the regex pattern sets created through AWS CloudFormation to a RegexMatchSet, use the AWS WAF console, API, or command line interface (CLI). For more information, see      UpdateRegexMatchSet.
#[derive(Default, serde::Serialize)]
pub struct CfnRegexPatternSet {


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

}