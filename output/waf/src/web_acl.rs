

/// Contains the Rules that identify the requests that you want to allow, block, or count. In a WebACL, you also specify a 			default action (ALLOW or BLOCK), and the action for each Rule that you add to a 			WebACL, for example, block requests from specified IP addresses or block requests from specified referrers.             You also associate the WebACL with a Amazon CloudFront distribution to identify the requests that you want AWS WAF to filter. 			If you add more than one Rule to a WebACL, a request needs to match only one of the specifications 			to be allowed, blocked, or counted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWebACL {


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
    /// Type: List of ActivatedRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<ActivatedRule>>,


    /// 
    /// The action to perform if none of the Rules contained in the WebACL match. The action is specified by the 		     WafAction object.
    /// 
    /// Required: Yes
    ///
    /// Type: WafAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAction")]
    pub default_action: WafAction,


    /// 
    /// The name of the metrics for this WebACL. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain       whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change MetricName after you create the WebACL.
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

}



impl cfn_resources::CfnResource for CfnWebACL {
    fn type_string() -> &'static str {
        "AWS::WAF::WebACL"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// For the action that is associated with a rule in a WebACL, specifies the action that you want AWS WAF to perform when a 			web request matches all of the conditions in a rule. For the default action in a WebACL, specifies the action that you want             AWS WAF to take when a web request doesn't match all of the conditions in any of the rules in a WebACL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WafAction {


    /// 
    /// Specifies how you want AWS WAF to respond to requests that match the settings in a Rule. Valid settings include the following:
    /// 
    /// ALLOW: AWS WAF allows requests                        BLOCK: AWS WAF blocks requests                        COUNT: AWS WAF increments a counter of the requests that match all of the conditions in the rule.         AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can't specify COUNT 				for the default action for a WebACL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOW | BLOCK | COUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: WafActionTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum WafActionTypeEnum {

    /// ALLOW
    #[serde(rename = "ALLOW")]
    Allow,

    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

}

impl Default for WafActionTypeEnum {
    fn default() -> Self {
        WafActionTypeEnum::Allow
    }
}



/// The ActivatedRule object in an UpdateWebACL request specifies a Rule that you want to insert or delete, 			the priority of the Rule in the WebACL, and the action that you want AWS WAF to take when a web request matches the Rule 			(ALLOW, BLOCK, or COUNT).
///
/// To specify whether to insert or delete a Rule, use the Action parameter in the WebACLUpdate data type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ActivatedRule {


    /// 
    /// Specifies the action that Amazon CloudFront or AWS WAF takes when a web request matches the conditions in the Rule.  			Valid values for Action include the following:
    /// 
    /// ALLOW: CloudFront responds with the requested object.                        BLOCK: CloudFront responds with an HTTP 403 (Forbidden) status code.                        COUNT: AWS WAF increments a counter of requests that match the conditions in the rule and then continues to 					inspect the web request based on the remaining rules in the web ACL.
    /// 
    /// ActivatedRule|OverrideAction applies only when updating or adding a       RuleGroup to a WebACL. In this     case,     you do not use ActivatedRule|Action. For all other update requests,       ActivatedRule|Action is used instead of       ActivatedRule|OverrideAction.
    /// 
    /// Required: No
    ///
    /// Type: WafAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<WafAction>,


    /// 
    /// Specifies the order in which the Rules in a WebACL are evaluated. Rules with a lower value for 			Priority are evaluated before Rules with a higher value. The value must be a unique integer. If you add multiple 			Rules to a WebACL, the values don't need to be consecutive.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// The RuleId for a Rule. You use RuleId to get more information about a Rule, 			update a Rule, insert a Rule into a WebACL or delete a 			one from a WebACL, or delete a Rule from AWS WAF.
    /// 
    /// RuleId is returned by CreateRule and by ListRules.
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


