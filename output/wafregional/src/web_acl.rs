

/// Contains the Rules that identify the requests that you want to allow, block, or count. In a WebACL, you also specify a      default action (ALLOW or BLOCK), and the action for each Rule that you add to a      WebACL, for example, block requests from specified IP addresses or block requests from specified referrers.      If you add more than one Rule to a WebACL, a request needs to match only one of the specifications      to be allowed, blocked, or counted.
///
/// To identify the requests that you want AWS WAF to filter, you associate the WebACL with an API Gateway API or an Application Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWebACL {


    /// 
    /// The action to perform if none of the Rules contained in the WebACL match. The action is specified by the      WafAction object.
    /// 
    /// Required: Yes
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAction")]
    pub default_action: Action,


    /// 
    /// A name for the metrics for this WebACL. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain     whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change MetricName after you create the WebACL.
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
    /// A friendly name or description of the WebACL. You can't change the name of a WebACL after you create it.
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
    /// An array that contains the action for each Rule in a WebACL, the priority of the Rule, 			and the ID of the Rule.
    /// 
    /// Required: No
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,

}



impl cfn_resources::CfnResource for CfnWebACL {
    fn type_string() -> &'static str {
        "AWS::WAFRegional::WebACL"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Specifies the action AWS WAF takes when a web request matches or doesn't match all rule conditions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {


    /// 
    /// For actions that are associated with a rule, the action that AWS WAF takes when a web request matches all conditions in a rule.
    /// 
    /// For the default action of a web access control list (ACL), the action that AWS WAF takes when a web request doesn't match all conditions in any rule.
    /// 
    /// Valid settings include the following:
    /// 
    /// ALLOW: AWS WAF allows requests                   BLOCK: AWS WAF blocks requests                      COUNT: AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can't specify COUNT for the default action for a WebACL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}




/// A combination of ByteMatchSet, IPSet, and/or SqlInjectionMatchSet objects that identify the web requests that you      want to allow, block, or count. For example, you might create a Rule that includes the following predicates:
///
/// To match the settings in this Rule, a request must originate from 192.0.2.44 AND include a User-Agent     header for which the value is BadBot.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rule {


    /// 
    /// The action that AWS WAF takes when a web request matches all conditions in the rule, such as allow, block, or count the request.
    /// 
    /// Required: Yes
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Action,


    /// 
    /// The order in which AWS WAF evaluates the rules in a web ACL. AWS WAF evaluates rules with a lower value before rules with a higher value. The value must be a unique integer. If you have multiple rules in a web ACL, the priority numbers do not need to be consecutive.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// The ID of an AWS WAF Regional rule to associate with a web ACL.
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
    #[serde(rename = "RuleId")]
    pub rule_id: String,

}


