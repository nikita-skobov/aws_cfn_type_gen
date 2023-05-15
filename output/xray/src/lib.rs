
pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// The case-sensitive name of the new group. Names must be unique.
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "InsightsConfiguration")]
    pub insights_configuration: Option<InsightsConfiguration>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The filter expression defining criteria by which to group traces.
    #[serde(rename = "FilterExpression")]
    pub filter_expression: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct InsightsConfiguration {
    #[serde(rename = "NotificationsEnabled")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "InsightsEnabled")]
    pub insights_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// The resource policy document, which can be up to 5kb in size.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: String,
    /// The name of the resource policy. Must be unique within a specific AWS account.
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// A flag to indicate whether to bypass the resource policy lockout safety check
    #[serde(rename = "BypassPolicyLockoutCheck")]
    pub bypass_policy_lockout_check: Option<bool>,

}



}

pub mod cfn_sampling_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnSamplingRule {
    /// No documentation provided by AWS
    #[serde(rename = "SamplingRule")]
    pub sampling_rule: Option<SamplingRule>,
    /// The ARN of the sampling rule. Specify a rule by either name or ARN, but not both.
    #[serde(rename = "RuleName")]
    pub rule_name: Option<RuleName>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// No documentation provided by AWS
    #[serde(rename = "SamplingRuleRecord")]
    pub sampling_rule_record: Option<Box<SamplingRuleRecord>>,
    /// No documentation provided by AWS
    #[serde(rename = "SamplingRuleUpdate")]
    pub sampling_rule_update: Option<SamplingRuleUpdate>,

}


#[derive(serde::Serialize, Default)]
pub struct SamplingRuleRecord {
    #[serde(rename = "SamplingRule")]
    pub sampling_rule: Option<SamplingRule>,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    #[serde(rename = "ModifiedAt")]
    pub modified_at: Option<String>,

}
pub type RuleName = String;
#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type RuleARN = String;
#[derive(serde::Serialize, Default)]
pub struct SamplingRule {
    #[serde(rename = "FixedRate")]
    pub fixed_rate: f64,
    #[serde(rename = "Priority")]
    pub priority: usize,
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "RuleARN")]
    pub rule_arn: Option<RuleARN>,
    #[serde(rename = "RuleName")]
    pub rule_name: Option<RuleName>,
    #[serde(rename = "HTTPMethod")]
    pub httpmethod: String,
    #[serde(rename = "ServiceType")]
    pub service_type: String,
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "Version")]
    pub version: Option<usize>,
    #[serde(rename = "ReservoirSize")]
    pub reservoir_size: usize,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    #[serde(rename = "URLPath")]
    pub urlpath: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SamplingRuleUpdate {
    #[serde(rename = "RuleARN")]
    pub rule_arn: Option<RuleARN>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,
    #[serde(rename = "ReservoirSize")]
    pub reservoir_size: Option<usize>,
    #[serde(rename = "ResourceARN")]
    pub resource_arn: Option<String>,
    #[serde(rename = "URLPath")]
    pub urlpath: Option<String>,
    #[serde(rename = "RuleName")]
    pub rule_name: Option<RuleName>,
    #[serde(rename = "HTTPMethod")]
    pub httpmethod: Option<String>,
    #[serde(rename = "FixedRate")]
    pub fixed_rate: Option<f64>,
    #[serde(rename = "ServiceType")]
    pub service_type: Option<String>,
    #[serde(rename = "Host")]
    pub host: Option<String>,

}


}
