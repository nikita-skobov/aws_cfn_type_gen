

/// Use the AWS::XRay::SamplingRule resource to specify a sampling rule, which controls sampling behavior for instrumented applications.    Include a SamplingRule entity to create or update a sampling rule.
///
/// Services retrieve rules with GetSamplingRules, and evaluate each rule in ascending    order of priority for each request. If a rule matches, the service records a trace, borrowing it from the reservoir size. After 10 seconds, the service    reports back to X-Ray with GetSamplingTargets to get updated versions of    each in-use rule. The updated rule contains a trace quota that the service can use instead of borrowing from the reservoir.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSamplingRule {


    /// 
    /// The sampling rule to be created or updated.
    /// 
    /// Required: No
    ///
    /// Type: SamplingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingRule")]
    pub sampling_rule: Option<Box<SamplingRule>>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnSamplingRule {
    fn type_string() -> &'static str {
        "AWS::XRay::SamplingRule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A sampling rule that services use to decide whether to instrument a request. Rule    fields can match properties of the service, or properties of a request. The service can ignore    rules that don't match its properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SamplingRule {


    /// 
    /// Matches attributes derived from the request.
    /// 
    /// Map Entries: Maximum number of 5 items.
    /// 
    /// Key Length Constraints: Minimum length of 1. Maximum length of 32.
    /// 
    /// Value Length Constraints: Minimum length of 1. Maximum length of 32.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The percentage of matching requests to instrument, after the reservoir is    exhausted.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedRate")]
    pub fixed_rate: f64,


    /// 
    /// Matches the HTTP method of a request.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTPMethod")]
    pub httpmethod: String,


    /// 
    /// Matches the hostname from a request URL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "Host")]
    pub host: String,


    /// 
    /// The priority of the sampling rule.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 9999
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// A fixed number of matching requests to instrument per second, prior to applying the    fixed rate. The reservoir is not used directly by services, but applies to all services using the rule collectively.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReservoirSize")]
    pub reservoir_size: i64,


    /// 
    /// Matches the ARN of the AWS resource on which the service runs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 500
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,


    /// 
    /// The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.
    /// 
    /// NoteSpecifying a sampling rule by name is recommended, as specifying by      ARN will be deprecated in future.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleARN")]
    pub rule_arn: Option<String>,


    /// 
    /// The name of the sampling rule. Specify a rule by either name or ARN, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,


    /// 
    /// Matches the name that the service uses to identify itself in segments.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: String,


    /// 
    /// Matches the origin that the service uses to identify its type in segments.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceType")]
    pub service_type: String,


    /// 
    /// Matches the path from a request URL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "URLPath")]
    pub urlpath: String,


    /// 
    /// The version of the sampling rule. Version can only be set when creating a new sampling rule.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: Option<i64>,

}




/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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


