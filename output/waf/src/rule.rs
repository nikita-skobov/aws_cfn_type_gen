

/// A combination of ByteMatchSet, IPSet, and/or SqlInjectionMatchSet objects that identify the web requests that you 			want to allow, block, or count. For example, you might create a Rule that includes the following predicates:
///
/// To match the settings in this Rule, a request must originate from 192.0.2.44 AND include a User-Agent 			header for which the value is BadBot.
#[derive(Default, serde::Serialize)]
pub struct CfnRule {


    /// 
    /// The name of the metrics for this Rule. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain     whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change MetricName after you create the Rule.
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
    /// The Predicates object contains one Predicate element for each ByteMatchSet, IPSet, or      SqlInjectionMatchSet object that you want to include in a Rule.
    /// 
    /// Required: No
    ///
    /// Type: List of Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicates")]
    pub predicates: Option<Vec<Predicate>>,


    /// 
    /// The friendly name or description for the Rule. You can't change the name of a Rule after you create it.
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


/// Specifies the ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, and SizeConstraintSet objects 			that you want to add to a Rule and, for each object, indicates whether you want to negate the settings, for example, requests that do 			NOT originate from the IP address 192.0.2.44.
#[derive(Default, serde::Serialize)]
pub struct Predicate {


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
    /// Set Negated to False if you want AWS WAF to allow, block, or count requests based on the settings in the 		     specified ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, or SizeConstraintSet. 			For example, if an IPSet includes the IP address 192.0.2.44, AWS WAF will allow or block requests based on that IP address.
    /// 
    /// Set Negated to True if you want AWS WAF to allow or block a request based on the negation 		     of the settings in the ByteMatchSet, IPSet, SqlInjectionMatchSet, XssMatchSet, RegexMatchSet, GeoMatchSet, or SizeConstraintSet. 			For example, if an IPSet includes the IP address 192.0.2.44, AWS WAF will allow, block, or count requests based on 			all IP addresses except       192.0.2.44.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Negated")]
    pub negated: bool,

}
