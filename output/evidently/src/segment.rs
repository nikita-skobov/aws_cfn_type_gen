

/// Creates or updates a segment of your audience. A segment    is a portion of your audience that share one or more characteristics. Examples could be Chrome browser users,    users in Europe, or Firefox browser users in Europe who also fit other criteria that your application collects,    such as age.
///
/// Using a segment in an experiment limits that experiment to evaluate only the users who match the segment      criteria. Using one or more segments in a launch allow you to define different traffic splits for the different      audience segments.
///
/// For more information about segment pattern syntax, see              Segment rule pattern syntax.
///
/// The pattern that you define for a segment is matched against the value of evaluationContext, which      is passed into Evidently in the EvaluateFeature operation,      when Evidently assigns a feature variation to a user.
#[derive(Default, serde::Serialize)]
pub struct CfnSegment {


    /// 
    /// The pattern to use for the segment. For more information about pattern syntax,     see       Segment rule pattern syntax.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,


    /// An optional description for this segment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// A name for the segment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Assigns one or more tags (key-value pairs) to the feature.
    /// 
    /// Tags can help you organize and categorize your resources. You can also use them to scope user         permissions by granting a user         permission to access or change only resources with certain tag values.
    /// 
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    /// 
    /// You can associate as many as 50 tags with a feature.
    /// 
    /// For more information, see Tagging         AWS resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

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
