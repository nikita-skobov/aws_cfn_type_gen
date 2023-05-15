
pub mod cfn_byte_match_set {

#[derive(serde::Serialize, Default)]
pub struct CfnByteMatchSet {
    /// List of ByteMatchTuple
    #[serde(rename = "ByteMatchTuples")]
    pub byte_match_tuples: Option<Vec<ByteMatchTuple>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct ByteMatchTuple {
    #[serde(rename = "TargetStringBase64")]
    pub target_string_base64: Option<String>,
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: String,
    #[serde(rename = "TargetString")]
    pub target_string: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "Data")]
    pub data: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,

}


}

pub mod cfn_geo_match_set {

#[derive(serde::Serialize, Default)]
pub struct CfnGeoMatchSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of GeoMatchConstraint
    #[serde(rename = "GeoMatchConstraints")]
    pub geo_match_constraints: Option<Vec<GeoMatchConstraint>>,

}


#[derive(serde::Serialize, Default)]
pub struct GeoMatchConstraint {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_ipset {

#[derive(serde::Serialize, Default)]
pub struct CfnIPSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of IPSetDescriptor
    #[serde(rename = "IPSetDescriptors")]
    pub ipset_descriptors: Option<Vec<IPSetDescriptor>>,

}


#[derive(serde::Serialize, Default)]
pub struct IPSetDescriptor {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub ty: String,

}


}

pub mod cfn_rate_based_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnRateBasedRule {
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RateKey")]
    pub rate_key: String,
    /// No documentation provided by AWS
    #[serde(rename = "RateLimit")]
    pub rate_limit: usize,
    /// List of Predicate
    #[serde(rename = "MatchPredicates")]
    pub match_predicates: Option<Vec<Predicate>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Predicate {
    #[serde(rename = "Negated")]
    pub negated: bool,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "DataId")]
    pub data_id: String,

}


}

pub mod cfn_regex_pattern_set {

#[derive(serde::Serialize, Default)]
pub struct CfnRegexPatternSet {
    /// No documentation provided by AWS
    #[serde(rename = "RegexPatternStrings")]
    pub regex_pattern_strings: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}



}

pub mod cfn_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnRule {
    /// List of Predicate
    #[serde(rename = "Predicates")]
    pub predicates: Option<Vec<Predicate>>,
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Predicate {
    #[serde(rename = "Negated")]
    pub negated: bool,
    #[serde(rename = "DataId")]
    pub data_id: String,
    #[serde(rename = "Type")]
    pub ty: String,

}


}

pub mod cfn_size_constraint_set {

#[derive(serde::Serialize, Default)]
pub struct CfnSizeConstraintSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of SizeConstraint
    #[serde(rename = "SizeConstraints")]
    pub size_constraints: Option<Vec<SizeConstraint>>,

}


#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Data")]
    pub data: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SizeConstraint {
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "Size")]
    pub size: usize,

}


}

pub mod cfn_sql_injection_match_set {

#[derive(serde::Serialize, Default)]
pub struct CfnSqlInjectionMatchSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of SqlInjectionMatchTuple
    #[serde(rename = "SqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Option<Vec<SqlInjectionMatchTuple>>,

}


#[derive(serde::Serialize, Default)]
pub struct SqlInjectionMatchTuple {
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,

}

#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Data")]
    pub data: Option<String>,

}


}

pub mod cfn_web_acl {

#[derive(serde::Serialize, Default)]
pub struct CfnWebACL {
    /// List of Rule
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultAction")]
    pub default_action: Action,

}


#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "Action")]
    pub action: Action,
    #[serde(rename = "Priority")]
    pub priority: usize,
    #[serde(rename = "RuleId")]
    pub rule_id: String,

}


}

pub mod cfn_web_aclassociation {

#[derive(serde::Serialize, Default)]
pub struct CfnWebACLAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "WebACLId")]
    pub web_aclid: String,

}



}

pub mod cfn_xss_match_set {

#[derive(serde::Serialize, Default)]
pub struct CfnXssMatchSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of XssMatchTuple
    #[serde(rename = "XssMatchTuples")]
    pub xss_match_tuples: Option<Vec<XssMatchTuple>>,

}


#[derive(serde::Serialize, Default)]
pub struct XssMatchTuple {
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,

}

#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "Data")]
    pub data: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,

}


}
