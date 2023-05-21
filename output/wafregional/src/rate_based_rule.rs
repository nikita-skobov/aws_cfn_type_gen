

/// A RateBasedRule is identical to a regular Rule, with     one addition: a RateBasedRule counts the number of requests that arrive from a     specified IP address every five minutes. For example, based on recent requests that you've     seen from an attacker, you might create a RateBasedRule that includes the     following conditions:
///
/// In the rule, you also define the rate limit as 15,000.
///
/// Requests that meet both of these conditions and exceed 15,000 requests every five     minutes trigger the rule's action (block or count), which is defined in the web     ACL.
///
/// Note you can only create rate-based rules using an AWS CloudFormation template. To add the rate-based rules created through AWS CloudFormation to a web ACL, use the AWS WAF console, API, or command line interface (CLI). For more information, see    UpdateWebACL.
#[derive(Default, serde::Serialize)]
pub struct CfnRateBasedRule {


    /// 
    /// The field that AWS WAF uses to determine if requests are likely arriving from single     source and thus subject to rate monitoring. The only valid value for RateKey     is IP. IP indicates that requests arriving from the same IP     address are subject to the RateLimit that is specified in the       RateBasedRule.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: IP
    ///
    /// Update requires: Replacement
    #[serde(rename = "RateKey")]
    pub rate_key: String,


    /// 
    /// A name for the metrics for a RateBasedRule. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain    whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the       RateBasedRule.
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
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// A friendly name or description for a RateBasedRule. You can't change the     name of a RateBasedRule after you create it.
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
    /// The maximum number of requests, which have an identical value in the field specified     by the RateKey, allowed in a five-minute period. If the number of requests     exceeds the RateLimit and the other predicates specified in the rule are also       met, AWS WAF triggers the action that is specified for this rule.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateLimit")]
    pub rate_limit: i64,


    /// 
    /// The Predicates object contains one Predicate element for      each ByteMatchSet, IPSet, or SqlInjectionMatchSet> object that you want to include in a       RateBasedRule.
    /// 
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchPredicates")]
    pub match_predicates: Option<Vec<Predicate>>,

}


/// Specifies the ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, and SizeConstraintSet objects 			that you want to add to a Rule and, for each object, indicates whether you want to negate the settings, for example, requests that do 			NOT originate from the IP address 192.0.2.44.
#[derive(Default, serde::Serialize)]
pub struct Predicate {


    /// 
    /// Set Negated to False if you want AWS WAF to allow, block, or count requests based on the settings in the 		     specified ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, or SizeConstraintSet.               For example, if an IPSet includes the IP address 192.0.2.44, AWS WAF will allow or block requests based on that IP address.
    /// 
    /// Set Negated to True if you want AWS WAF to allow or block a request based on the negation 		     of the settings in the ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, or SizeConstraintSet>.             For example, if an IPSet includes the IP address 192.0.2.44, AWS WAF will allow, block, or count requests based on 			all IP addresses except       192.0.2.44.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Negated")]
    pub negated: bool,


    /// 
    /// A unique identifier for a predicate in a Rule, such as ByteMatchSetId or IPSetId. 			The ID is returned by the corresponding Create or List command.
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
    /// Update requires: No interruption
    #[serde(rename = "DataId")]
    pub data_id: String,


    /// 
    /// The type of predicate in a Rule, such as ByteMatch or IPSet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ByteMatch | GeoMatch | IPMatch | RegexMatch | SizeConstraint | SqlInjectionMatch | XssMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}
