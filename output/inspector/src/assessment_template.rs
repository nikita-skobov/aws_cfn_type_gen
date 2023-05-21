

/// The AWS::Inspector::AssessmentTemplate resource creates an Amazon     Inspector assessment template, which specifies the Inspector assessment targets that will     be evaluated by an assessment run and its related configurations.
#[derive(Default, serde::Serialize)]
pub struct CfnAssessmentTemplate {


    /// 
    /// The user-defined name that identifies the assessment template that you want to     create. You can create several assessment templates for the same assessment target. The     names of the assessment templates that correspond to a particular assessment target must be     unique.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 140
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssessmentTemplateName")]
    pub assessment_template_name: Option<String>,


    /// 
    /// The duration of the assessment run in seconds.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 180
    ///
    /// Maximum: 86400
    ///
    /// Update requires: Replacement
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: i64,


    /// 
    /// The ARNs of the rules packages that you want to use in the assessment     template.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "RulesPackageArns")]
    pub rules_package_arns: Vec<String>,


    /// 
    /// The user-defined attributes that are assigned to every finding that is generated by     the assessment run that uses this assessment template. Within an assessment template, each     key must be unique.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserAttributesForFindings")]
    pub user_attributes_for_findings: Option<Vec<Tag>>,


    /// 
    /// The ARN of the assessment target to be included in the assessment template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssessmentTargetArn")]
    pub assessment_target_arn: String,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}