
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
pub struct FieldToMatch {
    #[serde(rename = "Data")]
    pub data: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct ByteMatchTuple {
    #[serde(rename = "TextTransformation")]
    pub text_transformation: String,
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: String,
    #[serde(rename = "TargetString")]
    pub target_string: Option<String>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TargetStringBase64")]
    pub target_string_base64: Option<String>,

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
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnRule {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// List of Predicate
    #[serde(rename = "Predicates")]
    pub predicates: Option<Vec<Predicate>>,

}


#[derive(serde::Serialize, Default)]
pub struct Predicate {
    #[serde(rename = "DataId")]
    pub data_id: String,
    #[serde(rename = "Negated")]
    pub negated: bool,
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
    pub size_constraints: Vec<SizeConstraint>,

}


#[derive(serde::Serialize, Default)]
pub struct SizeConstraint {
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "Size")]
    pub size: usize,
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
    /// List of ActivatedRule
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<ActivatedRule>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultAction")]
    pub default_action: WafAction,

}


#[derive(serde::Serialize, Default)]
pub struct WafAction {
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct ActivatedRule {
    #[serde(rename = "Action")]
    pub action: Option<WafAction>,
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    #[serde(rename = "Priority")]
    pub priority: usize,

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
    pub xss_match_tuples: Vec<XssMatchTuple>,

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
