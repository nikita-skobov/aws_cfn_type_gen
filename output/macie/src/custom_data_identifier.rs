

/// The AWS::Macie::CustomDataIdentifier resource specifies a custom data       identifier. A custom data identifier is a set of custom criteria       for Amazon Macie to use when it inspects data sources for sensitive data.       The criteria consist of a regular expression (regex) that defines a       text pattern to match and, optionally, character sequences and a proximity rule that       refine the results. The character sequences can be:
///
/// By using custom data identifiers, you can supplement the managed data identifiers that         Macie provides and detect sensitive data that reflects your       particular scenarios, intellectual property, or proprietary data. For more information,       see Building custom data identifiers in the Amazon Macie         User Guide.
///
/// An AWS::Macie::Session resource must exist for an AWS account before you can create an         AWS::Macie::CustomDataIdentifier resource for the account. Use a DependsOn         attribute to ensure that an AWS::Macie::Session resource is       created before other Macie resources are created for an account. For       example, "DependsOn": "Session".
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCustomDataIdentifier {


    /// 
    /// A custom description of the custom data identifier. The description can contain 1-512       characters.
    /// 
    /// Avoid including sensitive data in the description. Users of the account might be able       to see the description, depending on the actions that they're allowed to perform in         Amazon Macie.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// An array of character sequences (ignore words) to exclude from       the results. If text matches the regular expression (Regex) but it contains       a string in this array, Amazon Macie ignores the text and doesn't include it       in the results.
    /// 
    /// The array can contain 1-10 ignore words. Each ignore word can contain 4-90 UTF-8       characters. Ignore words are case sensitive.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IgnoreWords")]
    pub ignore_words: Option<Vec<String>>,


    /// 
    /// An array of character sequences (keywords), one of which must       precede and be in proximity (MaximumMatchDistance) of the regular       expression (Regex) to match.
    /// 
    /// The array can contain 1-50 keywords. Each keyword can contain 3-90 UTF-8 characters.       Keywords aren't case sensitive.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Keywords")]
    pub keywords: Option<Vec<String>>,


    /// 
    /// The maximum number of characters that can exist between the end of at least one       complete character sequence specified by the Keywords array and the end of       text that matches the regular expression (Regex). If a complete keyword       precedes all the text that matches the regular expression and the keyword is within the       specified distance, Amazon Macie includes the result.
    /// 
    /// The distance can be 1-300 characters. The default value is 50.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaximumMatchDistance")]
    pub maximum_match_distance: Option<i64>,


    /// 
    /// A custom name for the custom data identifier. The name can contain 1-128       characters.
    /// 
    /// Avoid including sensitive data in the name of a custom data identifier. Users of the       account might be able to see the name, depending on the actions that they're allowed to       perform in Amazon Macie.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The regular expression (regex) that defines the text pattern to       match. The expression can contain 1-512 characters.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Regex")]
    pub regex: String,

}



impl cfn_resources::CfnResource for CfnCustomDataIdentifier {
    fn type_string() -> &'static str {
        "AWS::Macie::CustomDataIdentifier"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
