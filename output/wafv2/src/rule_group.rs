

/// Use an AWS::WAFv2::RuleGroup to define a collection of rules for inspecting and controlling web requests. You use a rule group in an AWS::WAFv2::WebACL by providing its Amazon Resource Name (ARN) to the rule statement RuleGroupReferenceStatement, when you add rules to the web ACL.
///
/// When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRuleGroup {


    /// 
    /// The labels that one or more rules in this rule group add to matching web requests. These labels are defined in the RuleLabels for a Rule.
    /// 
    /// Required: No
    ///
    /// Type: List of LabelSummary
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailableLabels")]
    pub available_labels: Option<Vec<LabelSummary>>,


    /// 
    /// The web ACL capacity units (WCUs) required for this rule group.
    /// 
    /// When you create your own rule group, you define this, and you cannot change it after creation.      When you add or modify the rules in a rule group, AWS WAF enforces this limit.
    /// 
    /// AWS WAF uses WCUs to calculate and control the operating     resources that are used to run your rules, rule groups, and web ACLs. AWS WAF     calculates capacity differently for each rule type, to reflect the relative cost of each rule.      Simple rules that cost little to run use fewer WCUs than more complex rules 				that use more processing power. 				Rule group capacity is fixed at creation, which helps users plan their      web ACL WCU usage when they use a rule group.      The WCU limit for web ACLs is 1,500.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Capacity")]
    pub capacity: i64,


    /// 
    /// The labels that one or more rules in this rule group match against in label match statements. These labels are defined in a LabelMatchStatement specification, in the Statement definition of a rule.
    /// 
    /// Required: No
    ///
    /// Type: List of LabelSummary
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsumedLabels")]
    pub consumed_labels: Option<Vec<LabelSummary>>,


    /// 
    /// A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the rule group, and then use them in the rules that you define in the rule group.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// For information about the limits on count and size for custom request and response settings, see AWS WAF quotas    in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Map of CustomResponseBody
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomResponseBodies")]
    pub custom_response_bodies: Option<std::collections::HashMap<String, CustomResponseBody>>,


    /// 
    /// A description of the rule group that helps with identification.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[\w+=:#@/\-,\.][\w+=:#@/\-,\.\s]+[\w+=:#@/\-,\.]$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the rule group. You cannot change the name of a rule group after you create it.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\w\-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The rule statements used to identify the web requests that you      want to allow, block, or count. Each rule includes one top-level statement that AWS WAF uses to identify matching      web requests, and parameters that govern how AWS WAF handles them.
    /// 
    /// Required: No
    ///
    /// Type: List of Rule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,


    /// 
    /// Specifies whether this is for an Amazon CloudFront distribution or for a regional application. A regional    application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, an AWS AppSync GraphQL API,    an Amazon Cognito user pool, or an AWS App Runner service. Valid Values are CLOUDFRONT and REGIONAL.
    /// 
    /// NoteFor CLOUDFRONT, you must create your WAFv2 resources in the US East (N. Virginia) Region, us-east-1.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Scope")]
    pub scope: String,


    /// 
    /// Key:value pairs associated with an AWS resource. The key:value pair can be anything you define. Typically, the tag key represents a category (such as "environment") and the tag value represents a specific value within that category (such as "test," "development," or "production"). You can add up to 50 tags to each AWS resource.
    /// 
    /// NoteTo modify tags on existing resources, use the AWS WAF APIs or command line interface. With AWS CloudFormation, you can only add tags to AWS WAF resources during resource creation.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Defines and enables Amazon CloudWatch metrics and web request sample collection.
    /// 
    /// Required: Yes
    ///
    /// Type: VisibilityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,

}



impl cfn_resources::CfnResource for CfnRuleGroup {
    fn type_string(&self) -> &'static str {
        "AWS::WAFv2::RuleGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.visibility_config.validate()?;

        Ok(())
    }
}

/// Specifies that AWS WAF should allow the request and optionally defines additional     custom handling for the request.
///
/// This is used in the context of other settings, for example to specify values for RuleAction and web ACL DefaultAction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AllowAction {


    /// 
    /// Defines custom handling for the web request.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomRequestHandling
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}



impl cfn_resources::CfnResource for AllowAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.custom_request_handling.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A logical rule statement used to combine other rule statements with AND logic. You provide more than one Statement within the AndStatement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AndStatement {


    /// 
    /// The statements to combine with AND logic. You can use any statements that can be nested.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Statement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}



impl cfn_resources::CfnResource for AndStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies that AWS WAF should block the request and optionally defines additional     custom handling for the response to the web request.
///
/// This is used in the context of other settings, for example to specify values for RuleAction and web ACL DefaultAction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BlockAction {


    /// 
    /// Defines a custom response for the web request.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomResponse")]
    pub custom_response: Option<CustomResponse>,

}



impl cfn_resources::CfnResource for BlockAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.custom_response.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Inspect the body of the web request. The body immediately follows the request     headers.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Body {


    /// 
    /// What AWS WAF should do if the body is larger than AWS WAF can inspect.   AWS WAF does not support inspecting the entire contents of the web request body if the body   exceeds the limit for the resource type. If the body is larger than the limit, the underlying host service   only forwards the contents that are below the limit to AWS WAF for inspection.
    /// 
    /// The default limit is 8 KB (8,192 kilobytes) for regional resources and 16 KB (16,384 kilobytes) for CloudFront distributions. For CloudFront distributions,   you can increase the limit in the web ACL AssociationConfig, for additional processing fees.
    /// 
    /// The options for oversize handling are the following:
    /// 
    /// CONTINUE - Inspect the body normally, according to the rule inspection criteria.                         MATCH - Treat the web request as matching the rule statement. AWS WAF        applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule        statement.
    /// 
    /// You can combine the MATCH or NO_MATCH    settings for oversize handling with your rule and web ACL action settings, so that you block any request whose body is over the limit.
    /// 
    /// Default: CONTINUE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE | MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<BodyOversizeHandlingEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum BodyOversizeHandlingEnum {

    /// CONTINUE
    #[serde(rename = "CONTINUE")]
    Continue,

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for BodyOversizeHandlingEnum {
    fn default() -> Self {
        BodyOversizeHandlingEnum::Continue
    }
}


impl cfn_resources::CfnResource for Body {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A rule statement that defines a string match search for AWS WAF to apply to web requests. The byte match statement provides the bytes to search for, the location in requests that you want AWS WAF to search, and other settings. The bytes to search for are typically a string that corresponds with ASCII characters. In the AWS WAF console and the developer guide, this is called a string match statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ByteMatchStatement {


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// The area within the portion of the web request that you want AWS WAF to search for       SearchString. Valid values include the following:
    /// 
    /// CONTAINS
    /// 
    /// The specified part of the web request must include the value of       SearchString, but the location doesn't matter.
    /// 
    /// CONTAINS_WORD
    /// 
    /// The specified part of the web request must include the value of       SearchString, and SearchString must contain only alphanumeric     characters or underscore (A-Z, a-z, 0-9, or _). In addition, SearchString must     be a word, which means that both of the following are true:
    /// 
    /// SearchString is at the beginning of the specified part of the web        request or is preceded by a character other than an alphanumeric character or        underscore (_). Examples include the value of a header and        ;BadBot.                        SearchString is at the end of the specified part of the web request or        is followed by a character other than an alphanumeric character or underscore (_),        for example, BadBot; and -BadBot;.
    /// 
    /// EXACTLY
    /// 
    /// The value of the specified part of the web request must exactly match the value of       SearchString.
    /// 
    /// STARTS_WITH
    /// 
    /// The value of SearchString must appear at the beginning of the specified     part of the web request.
    /// 
    /// ENDS_WITH
    /// 
    /// The value of SearchString must appear at the end of the specified part of     the web request.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTAINS | CONTAINS_WORD | ENDS_WITH | EXACTLY | STARTS_WITH
    ///
    /// Update requires: No interruption
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: ByteMatchStatementPositionalConstraintEnum,


    /// 
    /// A string value that you want AWS WAF to search for. AWS WAF searches only in        the part of web requests that you designate for inspection in FieldToMatch. The maximum length of the        value is 200 bytes. For alphabetic characters A-Z and a-z, the value is case sensitive.
    /// 
    /// Don't encode this string. Provide the value that you want AWS WAF to search for.        AWS CloudFormation automatically base64 encodes the value for you.
    /// 
    /// For example, suppose the value of Type is HEADER and the       value of Data is User-Agent. If you want to search the       User-Agent header for the value BadBot, you provide the string       BadBot in the value of SearchString.
    /// 
    /// You must specify either SearchString or SearchStringBase64 in a ByteMatchStatement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SearchString")]
    pub search_string: Option<String>,


    /// 
    /// String to search for in a web request component, base64-encoded. If you don't want to encode the string, specify the unencoded value in SearchString instead.
    /// 
    /// You must specify either SearchString or SearchStringBase64 in a ByteMatchStatement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SearchStringBase64")]
    pub search_string_base64: Option<String>,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ByteMatchStatementPositionalConstraintEnum {

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// CONTAINS_WORD
    #[serde(rename = "CONTAINS_WORD")]
    Containsword,

    /// ENDS_WITH
    #[serde(rename = "ENDS_WITH")]
    Endswith,

    /// EXACTLY
    #[serde(rename = "EXACTLY")]
    Exactly,

    /// STARTS_WITH
    #[serde(rename = "STARTS_WITH")]
    Startswith,

}

impl Default for ByteMatchStatementPositionalConstraintEnum {
    fn default() -> Self {
        ByteMatchStatementPositionalConstraintEnum::Contains
    }
}


impl cfn_resources::CfnResource for ByteMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        Ok(())
    }
}

/// Specifies that AWS WAF should run a CAPTCHA check against the request:
///
/// You can configure the expiration time         in the CaptchaConfig       ImmunityTimeProperty setting at the rule and web ACL level. The rule setting overrides the web ACL setting.
///
/// This action option is available for rules. It isn't available for web ACL default actions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptchaAction {


    /// 
    /// Defines custom handling for the web request, used when the CAPTCHA inspection determines that the request's token is valid and unexpired.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomRequestHandling
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}



impl cfn_resources::CfnResource for CaptchaAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.custom_request_handling.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies how AWS WAF should handle CAPTCHA evaluations. This is     available at the web ACL level and in each rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptchaConfig {


    /// 
    /// Determines how long a CAPTCHA timestamp in the token remains valid after the client     successfully solves a CAPTCHA puzzle.
    /// 
    /// Required: No
    ///
    /// Type: ImmunityTimeProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}



impl cfn_resources::CfnResource for CaptchaConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.immunity_time_property.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies that AWS WAF should run a Challenge check against the request to verify that the request is coming from a legitimate client session:
///
/// You can configure the expiration time      in the ChallengeConfig       ImmunityTimeProperty setting at the rule and web ACL level. The rule setting overrides the web ACL setting.
///
/// This action option is available for rules. It isn't available for web ACL default actions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChallengeAction {


    /// 
    /// Defines custom handling for the web request, used when the challenge inspection determines that the request's token is valid and unexpired.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomRequestHandling
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}



impl cfn_resources::CfnResource for ChallengeAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.custom_request_handling.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies how AWS WAF should handle Challenge evaluations. This is     available at the web ACL level and in each rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ChallengeConfig {


    /// 
    /// Determines how long a challenge timestamp in the token remains valid after the client     successfully responds to a challenge.
    /// 
    /// Required: No
    ///
    /// Type: ImmunityTimeProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}



impl cfn_resources::CfnResource for ChallengeConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.immunity_time_property.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The filter to use to identify the subset of cookies to inspect in a web request.
///
/// You must specify exactly one setting: either All, IncludedCookies, or ExcludedCookies.
///
/// Example JSON: "MatchPattern": { "IncludedCookies": {"KeyToInclude1", "KeyToInclude2", "KeyToInclude3"} }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CookieMatchPattern {


    /// 
    /// Inspect all cookies.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "All")]
    pub all: Option<serde_json::Value>,


    /// 
    /// Inspect only the cookies whose keys don't match any of the strings specified here.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 199
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedCookies")]
    pub excluded_cookies: Option<Vec<String>>,


    /// 
    /// Inspect only the cookies that have a key that matches one of the strings specified here.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 199
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedCookies")]
    pub included_cookies: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for CookieMatchPattern {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.excluded_cookies {

        if the_val.len() > 199 as _ {
            return Err(format!("Max validation failed on field 'excluded_cookies'. {} is greater than 199", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.included_cookies {

        if the_val.len() > 199 as _ {
            return Err(format!("Max validation failed on field 'included_cookies'. {} is greater than 199", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Inspect the cookies in the web request. You can specify the parts of the cookies to     inspect and you can narrow the set of cookies to inspect by including or excluding specific     keys.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
///
/// Example JSON: "Cookies": { "MatchPattern": { "All": {} }, "MatchScope": "KEY",       "OversizeHandling": "MATCH" }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Cookies {


    /// 
    /// The filter to use to identify the subset of cookies to inspect in a web request.
    /// 
    /// You must specify exactly one setting: either All, IncludedCookies, or ExcludedCookies.
    /// 
    /// Example JSON: "MatchPattern": { "IncludedCookies": {"KeyToInclude1", "KeyToInclude2", "KeyToInclude3"} }
    /// 
    /// Required: Yes
    ///
    /// Type: CookieMatchPattern
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchPattern")]
    pub match_pattern: CookieMatchPattern,


    /// 
    /// The parts of the cookies to inspect with the rule inspection criteria. If you specify       All, AWS WAF inspects both keys and values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | KEY | VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchScope")]
    pub match_scope: CookiesMatchScopeEnum,


    /// 
    /// What AWS WAF should do if the cookies of the request are larger than AWS WAF can inspect.   AWS WAF does not support inspecting the entire contents of request cookies    when they exceed 8 KB (8192 bytes) or 200 total cookies. The underlying host service forwards a maximum of 200 cookies    and at most 8 KB of cookie contents to AWS WAF.
    /// 
    /// The options for oversize handling are the following:
    /// 
    /// CONTINUE - Inspect the cookies normally, according to the rule inspection criteria.                         MATCH - Treat the web request as matching the rule statement. AWS WAF        applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule        statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE | MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: CookiesOversizeHandlingEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CookiesMatchScopeEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// KEY
    #[serde(rename = "KEY")]
    Key,

    /// VALUE
    #[serde(rename = "VALUE")]
    Value,

}

impl Default for CookiesMatchScopeEnum {
    fn default() -> Self {
        CookiesMatchScopeEnum::All
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CookiesOversizeHandlingEnum {

    /// CONTINUE
    #[serde(rename = "CONTINUE")]
    Continue,

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for CookiesOversizeHandlingEnum {
    fn default() -> Self {
        CookiesOversizeHandlingEnum::Continue
    }
}


impl cfn_resources::CfnResource for Cookies {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.match_pattern.validate()?;

        Ok(())
    }
}

/// Specifies that AWS WAF should count the request. Optionally defines additional custom     handling for the request.
///
/// This is used in the context of other settings, for example to specify values for RuleAction and web ACL DefaultAction.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CountAction {


    /// 
    /// Defines custom handling for the web request.
    /// 
    /// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: CustomRequestHandling
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}



impl cfn_resources::CfnResource for CountAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.custom_request_handling.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A custom header for custom request and response handling. This is used in CustomResponse and CustomRequestHandling
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomHTTPHeader {


    /// 
    /// The name of the custom header.
    /// 
    /// For custom request header insertion, when AWS WAF inserts the header into the request,     it prefixes this name x-amzn-waf-, to avoid confusion with the headers that     are already in the request. For example, for the header name sample, AWS WAF     inserts the header x-amzn-waf-sample.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^[a-zA-Z0-9._$-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value of the custom header.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for CustomHTTPHeader {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.value;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'value'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'value'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Custom request handling behavior that inserts custom headers into a web request. You can    add custom request handling for AWS WAF to use when the rule action doesn't block the request.      For example, CaptchaAction for requests with valid t okens, and AllowAction.
///
/// For information about customizing web requests and responses,       see Customizing web requests and responses in AWS WAF   in the         AWS WAF Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomRequestHandling {


    /// 
    /// The HTTP headers to insert into the request. Duplicate header names are not allowed.
    /// 
    /// For information about the limits on count and size for custom request and response settings, see AWS WAF quotas    in the         AWS WAF Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CustomHTTPHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsertHeaders")]
    pub insert_headers: Vec<CustomHTTPHeader>,

}



impl cfn_resources::CfnResource for CustomRequestHandling {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A custom response to send to the client. You can define a custom response for rule       actions and default web ACL actions that are set to Block.
///
/// For information about customizing web requests and responses, see Customizing web requests and responses in AWS WAF in the      AWS WAF Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomResponse {


    /// 
    /// References the response body that you want AWS WAF to return to the web request     client. You can define a custom response for a rule action or a default web ACL action that     is set to block. To do this, you first define the response body key and value in the       CustomResponseBodies setting for the AWS::WAFv2::WebACL or AWS::WAFv2::RuleGroup where you want to use it. Then, in the rule action or web ACL     default action BlockAction setting, you reference the response body using this     key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\w\-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomResponseBodyKey")]
    pub custom_response_body_key: Option<String>,


    /// 
    /// The HTTP status code to return to the client.
    /// 
    /// For a list of status codes that you can use in your custom responses, see Supported status codes for custom response    in the         AWS WAF Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 200
    ///
    /// Maximum: 599
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseCode")]
    pub response_code: i64,


    /// 
    /// The HTTP headers to use in the response. Duplicate header names are not allowed.
    /// 
    /// For information about the limits on count and size for custom request and response settings, see AWS WAF quotas    in the         AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomHTTPHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseHeaders")]
    pub response_headers: Option<Vec<CustomHTTPHeader>>,

}



impl cfn_resources::CfnResource for CustomResponse {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.custom_response_body_key {

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'custom_response_body_key'. {} is greater than 128", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.custom_response_body_key {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'custom_response_body_key'. {} is less than 1", the_val.len()));
        }

        }
        
        let the_val = &self.response_code;

        if *the_val > 599 as _ {
            return Err(format!("Max validation failed on field 'response_code'. {} is greater than 599", the_val));
        }

        
        let the_val = &self.response_code;

        if *the_val < 200 as _ {
            return Err(format!("Min validation failed on field 'response_code'. {} is less than 200", the_val));
        }

        
        Ok(())
    }
}

/// The response body to use in a custom response to a web request. This is referenced by     key from CustomResponse       CustomResponseBodyKey.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomResponseBody {


    /// 
    /// The payload of the custom response.
    /// 
    /// You can use JSON escape strings in JSON content. To do this, you must specify JSON     content in the ContentType setting.
    /// 
    /// For information about the limits on count and size for custom request and response settings, see AWS WAF quotas    in the         AWS WAF Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10240
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: String,


    /// 
    /// The type of content in the payload that you are defining in the Content     string.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: APPLICATION_JSON | TEXT_HTML | TEXT_PLAIN
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    pub content_type: CustomResponseBodyContentTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomResponseBodyContentTypeEnum {

    /// APPLICATION_JSON
    #[serde(rename = "APPLICATION_JSON")]
    Applicationjson,

    /// TEXT_HTML
    #[serde(rename = "TEXT_HTML")]
    Texthtml,

    /// TEXT_PLAIN
    #[serde(rename = "TEXT_PLAIN")]
    Textplain,

}

impl Default for CustomResponseBodyContentTypeEnum {
    fn default() -> Self {
        CustomResponseBodyContentTypeEnum::Applicationjson
    }
}


impl cfn_resources::CfnResource for CustomResponseBody {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.content;

        if the_val.len() > 10240 as _ {
            return Err(format!("Max validation failed on field 'content'. {} is greater than 10240", the_val.len()));
        }

        
        let the_val = &self.content;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'content'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// The part of the web request that you want AWS WAF to inspect. Include the single       FieldToMatch type that you want to inspect, with additional specifications     as needed, according to the type. You specify a single request component in       FieldToMatch for each rule statement that requires it. To inspect more than     one component of the web request, create a separate rule statement for each     component.
///
/// Example JSON for a QueryString field to match:
///
/// "FieldToMatch": { "QueryString": {} }
///
/// Example JSON for a Method field to match specification:
///
/// "FieldToMatch": { "Method": { "Name": "DELETE" } }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FieldToMatch {


    /// 
    /// Inspect all query arguments.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllQueryArguments")]
    pub all_query_arguments: Option<serde_json::Value>,


    /// 
    /// Inspect the request body as plain text. The request body immediately follows the request     headers. This is the part of a request that contains any additional data that you want to     send to your web server as the HTTP request body, such as data from a form.
    /// 
    /// A limited amount of the request body is forwarded to AWS WAF for    inspection by the underlying host service. For regional resources, the limit is 8 KB (8,192 kilobytes) and for CloudFront distributions, the limit is 16 KB (16,384 kilobytes). For CloudFront distributions,   you can increase the limit in the web ACL's AssociationConfig, for additional processing fees.
    /// 
    /// For information about how to handle oversized     request bodies, see the Body object configuration.
    /// 
    /// Required: No
    ///
    /// Type: Body
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    pub body: Option<Body>,


    /// 
    /// Inspect the request cookies. You must configure scope and pattern matching filters in     the Cookies object, to define the set of cookies and the parts of the cookies     that AWS WAF inspects.
    /// 
    /// Only the first 8 KB (8192 bytes) of a request's cookies and only the first 200 cookies     are forwarded to AWS WAF for inspection by the underlying host service. You must     configure how to handle any oversize cookie content in the Cookies object.     AWS WAF applies the pattern matching filters to the cookies that it receives from the     underlying host service.
    /// 
    /// Required: No
    ///
    /// Type: Cookies
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cookies")]
    pub cookies: Option<Cookies>,


    /// 
    /// Inspect the request headers. You must configure scope and pattern matching filters in     the Headers object, to define the set of headers to and the parts of the     headers that AWS WAF inspects.
    /// 
    /// Only the first 8 KB (8192 bytes) of a request's headers and only the first 200 headers     are forwarded to AWS WAF for inspection by the underlying host service. You must     configure how to handle any oversize header content in the Headers object.     AWS WAF applies the pattern matching filters to the headers that it receives from the     underlying host service.
    /// 
    /// Required: No
    ///
    /// Type: Headers
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    pub headers: Option<Headers>,


    /// 
    /// Inspect the request body as JSON. The request body immediately follows the request     headers. This is the part of a request that contains any additional data that you want to     send to your web server as the HTTP request body, such as data from a form.
    /// 
    /// A limited amount of the request body is forwarded to AWS WAF for    inspection by the underlying host service. For regional resources, the limit is 8 KB (8,192 kilobytes) and for CloudFront distributions, the limit is 16 KB (16,384 kilobytes). For CloudFront distributions,   you can increase the limit in the web ACL's AssociationConfig, for additional processing fees.
    /// 
    /// For information about how to handle oversized     request bodies, see the JsonBody object configuration.
    /// 
    /// Required: No
    ///
    /// Type: JsonBody
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonBody")]
    pub json_body: Option<JsonBody>,


    /// 
    /// Inspect the HTTP method. The method indicates the type of operation that the request is     asking the origin to perform.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Method")]
    pub method: Option<serde_json::Value>,


    /// 
    /// Inspect the query string. This is the part of a URL that appears after a ?     character, if any.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: Option<serde_json::Value>,


    /// 
    /// Inspect a single header. Provide the name of the header to inspect, for example,       User-Agent or Referer. This setting isn't case     sensitive.
    /// 
    /// Example JSON: "SingleHeader": { "Name": "haystack" }
    /// 
    /// Alternately, you can filter and inspect all headers with the Headers       FieldToMatch setting.
    /// 
    /// Required: No
    ///
    /// Type: SingleHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleHeader")]
    pub single_header: Option<SingleHeader>,


    /// 
    /// Inspect a single query argument. Provide the name of the query argument to inspect, such     as UserName or SalesRegion. The name can be up to     30 characters long and isn't case sensitive.
    /// 
    /// Example JSON: "SingleQueryArgument": { "Name": "myArgument" }
    /// 
    /// Required: No
    ///
    /// Type: SingleQueryArgument
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleQueryArgument")]
    pub single_query_argument: Option<SingleQueryArgument>,


    /// 
    /// Inspect the request URI path. This is the part of the web request that identifies a     resource, for example, /images/daily-ad.jpg.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "UriPath")]
    pub uri_path: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for FieldToMatch {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.body.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.cookies.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.headers.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.json_body.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.single_header.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.single_query_argument.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. Commonly, this is the X-Forwarded-For (XFF) header, but you can specify any header name.
///
/// This configuration is used for GeoMatchStatement and RateBasedStatement. For IPSetReferenceStatement, use IPSetForwardedIPConfig instead.
///
/// AWS WAF only evaluates the first IP address found in the specified HTTP header.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ForwardedIPConfiguration {


    /// 
    /// The match status to assign to the web request if the request doesn't have a valid IP address in the specified position.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// You can specify the following fallback behaviors:
    /// 
    /// MATCH - Treat the web request as matching the rule statement. AWS WAF applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: ForwardedIPConfigurationFallbackBehaviorEnum,


    /// 
    /// The name of the HTTP header to use for the IP address. For example, to use the X-Forwarded-For (XFF) header, set this to X-Forwarded-For.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ForwardedIPConfigurationFallbackBehaviorEnum {

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for ForwardedIPConfigurationFallbackBehaviorEnum {
    fn default() -> Self {
        ForwardedIPConfigurationFallbackBehaviorEnum::Match
    }
}


impl cfn_resources::CfnResource for ForwardedIPConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.header_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'header_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.header_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'header_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement that labels web requests by country and region and that matches against web requests based on country code. A geo match rule labels every request that it inspects regardless of whether it finds a match.
///
/// AWS WAF labels requests using the alpha-2 country and region codes from the International Organization for Standardization (ISO) 3166 standard. AWS WAF determines the codes using either the IP address in the web request origin or, if you specify it, the address in the geo match ForwardedIPConfig.
///
/// If you use the web request origin, the label formats are awswaf:clientip:geo:region:<ISO country code>-<ISO region code> and awswaf:clientip:geo:country:<ISO country code>.
///
/// If you use a forwarded IP address, the label formats are awswaf:forwardedip:geo:region:<ISO country code>-<ISO region code> and awswaf:forwardedip:geo:country:<ISO country code>.
///
/// For additional details, see Geographic match rule statement in the AWS WAF Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GeoMatchStatement {


    /// 
    /// An array of two-character country codes that you want to match against, for example, [ "US", "CN" ], from     the alpha-2 country ISO codes of the ISO 3166 international standard.
    /// 
    /// When you use a geo match statement just for the region and country labels that it adds to requests, you still have to supply a country code for the rule to evaluate. In this case, you configure the rule to only count matching requests, but it will still generate logging and count metrics for any matches. You can reduce the logging and metrics that the rule produces by specifying a country that's unlikely to be a source of traffic to your site.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CountryCodes")]
    pub country_codes: Option<Vec<String>>,


    /// 
    /// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. Commonly, this is the X-Forwarded-For (XFF) header, but you can specify any header name.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// Required: No
    ///
    /// Type: ForwardedIPConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,

}



impl cfn_resources::CfnResource for GeoMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.forwarded_ipconfig.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The filter to use to identify the subset of headers to inspect in a web request.
///
/// You must specify exactly one setting: either All, IncludedHeaders, or ExcludedHeaders.
///
/// Example JSON: "MatchPattern": { "ExcludedHeaders": {"KeyToExclude1", "KeyToExclude2"} }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HeaderMatchPattern {


    /// 
    /// Inspect all headers.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "All")]
    pub all: Option<serde_json::Value>,


    /// 
    /// Inspect only the headers whose keys don't match any of the strings specified here.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 199
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedHeaders")]
    pub excluded_headers: Option<Vec<String>>,


    /// 
    /// Inspect only the headers that have a key that matches one of the strings specified here.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 199
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedHeaders")]
    pub included_headers: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for HeaderMatchPattern {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.excluded_headers {

        if the_val.len() > 199 as _ {
            return Err(format!("Max validation failed on field 'excluded_headers'. {} is greater than 199", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.included_headers {

        if the_val.len() > 199 as _ {
            return Err(format!("Max validation failed on field 'included_headers'. {} is greater than 199", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Inspect all headers in the web request. You can specify the parts of the headers to     inspect and you can narrow the set of headers to inspect by including or excluding specific     keys.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
///
/// If you want to inspect just the value of a single header, use the       SingleHeader       FieldToMatch setting instead.
///
/// Example JSON: "Headers": { "MatchPattern": { "All": {} }, "MatchScope": "KEY",       "OversizeHandling": "MATCH" }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Headers {


    /// 
    /// The filter to use to identify the subset of headers to inspect in a web request.
    /// 
    /// You must specify exactly one setting: either All, IncludedHeaders, or ExcludedHeaders.
    /// 
    /// Example JSON: "MatchPattern": { "ExcludedHeaders": {"KeyToExclude1", "KeyToExclude2"} }
    /// 
    /// Required: Yes
    ///
    /// Type: HeaderMatchPattern
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchPattern")]
    pub match_pattern: HeaderMatchPattern,


    /// 
    /// The parts of the headers to match with the rule inspection criteria. If you specify       All, AWS WAF inspects both keys and values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | KEY | VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchScope")]
    pub match_scope: HeadersMatchScopeEnum,


    /// 
    /// What AWS WAF should do if the headers of the request are larger than AWS WAF can inspect.   AWS WAF does not support inspecting the entire contents of request headers    when they exceed 8 KB (8192 bytes) or 200 total headers. The underlying host service forwards a maximum of 200 headers    and at most 8 KB of header contents to AWS WAF.
    /// 
    /// The options for oversize handling are the following:
    /// 
    /// CONTINUE - Inspect the headers normally, according to the rule inspection criteria.                         MATCH - Treat the web request as matching the rule statement. AWS WAF        applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule        statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE | MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: HeadersOversizeHandlingEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HeadersMatchScopeEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// KEY
    #[serde(rename = "KEY")]
    Key,

    /// VALUE
    #[serde(rename = "VALUE")]
    Value,

}

impl Default for HeadersMatchScopeEnum {
    fn default() -> Self {
        HeadersMatchScopeEnum::All
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum HeadersOversizeHandlingEnum {

    /// CONTINUE
    #[serde(rename = "CONTINUE")]
    Continue,

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for HeadersOversizeHandlingEnum {
    fn default() -> Self {
        HeadersOversizeHandlingEnum::Continue
    }
}


impl cfn_resources::CfnResource for Headers {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.match_pattern.validate()?;

        Ok(())
    }
}

/// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. Commonly, this is the X-Forwarded-For (XFF) header, but you can specify any header name.
///
/// This configuration is used only for IPSetReferenceStatement. For GeoMatchStatement and RateBasedStatement, use ForwardedIPConfig instead.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IPSetForwardedIPConfiguration {


    /// 
    /// The match status to assign to the web request if the request doesn't have a valid IP address in the specified position.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// You can specify the following fallback behaviors:
    /// 
    /// MATCH - Treat the web request as matching the rule statement. AWS WAF applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule statement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: IPSetForwardedIPConfigurationFallbackBehaviorEnum,


    /// 
    /// The name of the HTTP header to use for the IP address. For example, to use the X-Forwarded-For (XFF) header, set this to X-Forwarded-For.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderName")]
    pub header_name: String,


    /// 
    /// The position in the header to search for the IP address. The header can contain IP     addresses of the original client and also of proxies. For example, the header value could     be 10.1.1.1, 127.0.0.0, 10.10.10.10 where the first IP address identifies the     original client and the rest identify proxies that the request went through.
    /// 
    /// The options for this setting are the following:
    /// 
    /// FIRST - Inspect the first IP address in the list of IP addresses in the        header. This is usually the client's original IP.               LAST - Inspect the last IP address in the list of IP addresses in the        header.               ANY - Inspect all IP addresses in the header for a match. If the header        contains more than 10 IP addresses, AWS WAF inspects the last 10.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ANY | FIRST | LAST
    ///
    /// Update requires: No interruption
    #[serde(rename = "Position")]
    pub position: IPSetForwardedIPConfigurationPositionEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum IPSetForwardedIPConfigurationFallbackBehaviorEnum {

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for IPSetForwardedIPConfigurationFallbackBehaviorEnum {
    fn default() -> Self {
        IPSetForwardedIPConfigurationFallbackBehaviorEnum::Match
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum IPSetForwardedIPConfigurationPositionEnum {

    /// ANY
    #[serde(rename = "ANY")]
    Any,

    /// FIRST
    #[serde(rename = "FIRST")]
    First,

    /// LAST
    #[serde(rename = "LAST")]
    Last,

}

impl Default for IPSetForwardedIPConfigurationPositionEnum {
    fn default() -> Self {
        IPSetForwardedIPConfigurationPositionEnum::Any
    }
}


impl cfn_resources::CfnResource for IPSetForwardedIPConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.header_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'header_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.header_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'header_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement used to detect web requests coming from particular IP addresses or address ranges. To use this, create an AWS::WAFv2::IPSet that specifies the addresses you want to detect, then use the ARN of that set in this statement.
///
/// Each IP set rule statement references an IP set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IPSetReferenceStatement {


    /// 
    /// The Amazon Resource Name (ARN) of the AWS::WAFv2::IPSet that this statement     references.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,


    /// 
    /// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. Commonly, this is the X-Forwarded-For (XFF) header, but you can specify any header name.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// Required: No
    ///
    /// Type: IPSetForwardedIPConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPSetForwardedIPConfig")]
    pub ipset_forwarded_ipconfig: Option<IPSetForwardedIPConfiguration>,

}



impl cfn_resources::CfnResource for IPSetReferenceStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.arn;

        if the_val.len() < 20 as _ {
            return Err(format!("Min validation failed on field 'arn'. {} is less than 20", the_val.len()));
        }

        
        self.ipset_forwarded_ipconfig.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Used for CAPTCHA and challenge token settings. Determines     how long a CAPTCHA or challenge timestamp remains valid after AWS WAF updates it for a successful CAPTCHA or challenge response.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImmunityTimeProperty {


    /// 
    /// The amount of time, in seconds, that a CAPTCHA or challenge timestamp is considered valid by AWS WAF. The default      setting is 300.
    /// 
    /// For the Challenge action, the minimum setting is 300.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImmunityTime")]
    pub immunity_time: i64,

}



impl cfn_resources::CfnResource for ImmunityTimeProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Inspect the body of the web request as JSON. The body immediately follows the request     headers.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
///
/// Use the specifications in this object to indicate which parts of the JSON body to     inspect using the rule's inspection criteria. AWS WAF inspects only the parts of the JSON     that result from the matches that you indicate.
///
/// Example JSON: "JsonBody": { "MatchPattern": { "All": {} }, "MatchScope": "ALL"       }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonBody {


    /// 
    /// What AWS WAF should do if it fails to completely parse the JSON body. The options are     the following:
    /// 
    /// EVALUATE_AS_STRING - Inspect the body as plain text. AWS WAF        applies the text transformations and inspection criteria that you defined for the        JSON inspection to the body text string.                        MATCH - Treat the web request as matching the rule statement.        AWS WAF applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule        statement.
    /// 
    /// If you don't provide this setting, AWS WAF parses and evaluates the content only up to the     first parsing failure that it encounters.
    /// 
    /// AWS WAF does its best to parse the entire JSON body, but might be forced to stop for     reasons such as invalid characters, duplicate keys, truncation, and any content whose root     node isn't an object or an array.
    /// 
    /// AWS WAF parses the JSON in the following examples as two valid key, value pairs:
    /// 
    /// Missing comma: {"key1":"value1""key2":"value2"}                       Missing colon: {"key1":"value1","key2""value2"}                       Extra colons: {"key1"::"value1","key2""value2"}
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EVALUATE_AS_STRING | MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "InvalidFallbackBehavior")]
    pub invalid_fallback_behavior: Option<JsonBodyInvalidFallbackBehaviorEnum>,


    /// 
    /// The patterns to look for in the JSON body. AWS WAF inspects the results of these     pattern matches against the rule inspection criteria.
    /// 
    /// Required: Yes
    ///
    /// Type: JsonMatchPattern
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchPattern")]
    pub match_pattern: JsonMatchPattern,


    /// 
    /// The parts of the JSON to match against using the MatchPattern. If you     specify All, AWS WAF matches against keys and values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALL | KEY | VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchScope")]
    pub match_scope: JsonBodyMatchScopeEnum,


    /// 
    /// What AWS WAF should do if the body is larger than AWS WAF can inspect.   AWS WAF does not support inspecting the entire contents of the web request body if the body   exceeds the limit for the resource type. If the body is larger than the limit, the underlying host service   only forwards the contents that are below the limit to AWS WAF for inspection.
    /// 
    /// The default limit is 8 KB (8,192 kilobytes) for regional resources and 16 KB (16,384 kilobytes) for CloudFront distributions. For CloudFront distributions,   you can increase the limit in the web ACL AssociationConfig, for additional processing fees.
    /// 
    /// The options for oversize handling are the following:
    /// 
    /// CONTINUE - Inspect the body normally, according to the rule inspection criteria.                         MATCH - Treat the web request as matching the rule statement. AWS WAF        applies the rule action to the request.                        NO_MATCH - Treat the web request as not matching the rule        statement.
    /// 
    /// You can combine the MATCH or NO_MATCH    settings for oversize handling with your rule and web ACL action settings, so that you block any request whose body is over the limit.
    /// 
    /// Default: CONTINUE
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONTINUE | MATCH | NO_MATCH
    ///
    /// Update requires: No interruption
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<JsonBodyOversizeHandlingEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum JsonBodyInvalidFallbackBehaviorEnum {

    /// EVALUATE_AS_STRING
    #[serde(rename = "EVALUATE_AS_STRING")]
    Evaluateasstring,

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for JsonBodyInvalidFallbackBehaviorEnum {
    fn default() -> Self {
        JsonBodyInvalidFallbackBehaviorEnum::Evaluateasstring
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JsonBodyMatchScopeEnum {

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// KEY
    #[serde(rename = "KEY")]
    Key,

    /// VALUE
    #[serde(rename = "VALUE")]
    Value,

}

impl Default for JsonBodyMatchScopeEnum {
    fn default() -> Self {
        JsonBodyMatchScopeEnum::All
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum JsonBodyOversizeHandlingEnum {

    /// CONTINUE
    #[serde(rename = "CONTINUE")]
    Continue,

    /// MATCH
    #[serde(rename = "MATCH")]
    Match,

    /// NO_MATCH
    #[serde(rename = "NO_MATCH")]
    Nomatch,

}

impl Default for JsonBodyOversizeHandlingEnum {
    fn default() -> Self {
        JsonBodyOversizeHandlingEnum::Continue
    }
}


impl cfn_resources::CfnResource for JsonBody {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.match_pattern.validate()?;

        Ok(())
    }
}

/// The patterns to look for in the JSON body. AWS WAF inspects the results of these     pattern matches against the rule inspection criteria. This is used with the FieldToMatch option JsonBody.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonMatchPattern {


    /// 
    /// Match all of the elements. See also     MatchScope in the JsonBody FieldToMatch specification.
    /// 
    /// You must specify either this setting or the IncludedPaths setting, but not     both.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "All")]
    pub all: Option<serde_json::Value>,


    /// 
    /// Match only the specified include paths. See also     MatchScope in the JsonBody FieldToMatch specification.
    /// 
    /// Provide the include paths using JSON Pointer syntax. For example, "IncludedPaths":       ["/dogs/0/name", "/dogs/1/name"]. For information about this syntax, see the     Internet Engineering Task Force (IETF) documentation JavaScript Object Notation (JSON)       Pointer.
    /// 
    /// You must specify either this setting or the All setting, but not     both.
    /// 
    /// NoteDon't use this option to include all paths. Instead, use the All       setting.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludedPaths")]
    pub included_paths: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for JsonMatchPattern {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A single label container. This is used as an element of a label array in RuleLabels inside a rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Label {


    /// 
    /// The label string.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[0-9A-Za-z_\-:]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for Label {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement to match against labels that have been added to the web request by rules that have already run in the web ACL.
///
/// The label match statement provides the label or namespace string to search for. The label string can represent a part or all of the fully qualified label name that had been added to the web request. Fully qualified labels have a prefix, optional namespaces, and label name. The prefix identifies the rule group or web ACL context of the rule that added the label. If you do not provide the fully qualified name in your label match string, AWS WAF performs the search for labels that were added in the same context as the label match statement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LabelMatchStatement {


    /// 
    /// The string to match against. The setting you provide for this depends on the match     statement's Scope setting:
    /// 
    /// If the Scope indicates LABEL, then this specification        must include the name and can include any number of preceding namespace        specifications and prefix up to providing the fully qualified label name.               If the Scope indicates NAMESPACE, then this        specification can include any number of contiguous namespace strings, and can include        the entire label namespace prefix from the rule group or web ACL where the label        originates.
    /// 
    /// Labels are case sensitive and components of a label must be separated by colon, for     example NS1:NS2:name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[0-9A-Za-z_\-:]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// Specify whether you want to match using the label name or just the namespace.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LABEL | NAMESPACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scope")]
    pub scope: LabelMatchStatementScopeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LabelMatchStatementScopeEnum {

    /// LABEL
    #[serde(rename = "LABEL")]
    Label,

    /// NAMESPACE
    #[serde(rename = "NAMESPACE")]
    Namespace,

}

impl Default for LabelMatchStatementScopeEnum {
    fn default() -> Self {
        LabelMatchStatementScopeEnum::Label
    }
}


impl cfn_resources::CfnResource for LabelMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.key;

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'key'. {} is greater than 1024", the_val.len()));
        }

        
        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'key'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// List of labels used by one or more of the rules of a AWS::WAFv2::RuleGroup. This     summary object is used for the following rule group lists:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LabelSummary {


    /// 
    /// An individual label specification.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[0-9A-Za-z_\-:]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



impl cfn_resources::CfnResource for LabelSummary {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.name {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// A logical rule statement used to negate the results of another rule statement. You provide one Statement within the NotStatement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NotStatement {


    /// 
    /// The statement to negate. You can use any statement that can be nested.
    /// 
    /// Required: Yes
    ///
    /// Type: Statement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statement")]
    pub statement: Statement,

}



impl cfn_resources::CfnResource for NotStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.statement.validate()?;

        Ok(())
    }
}

/// A logical rule statement used to combine other rule statements with OR logic. You provide more than one Statement within the OrStatement.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OrStatement {


    /// 
    /// The statements to combine with OR logic. You can use any statements that can be     nested.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Statement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}



impl cfn_resources::CfnResource for OrStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A rate-based rule counts incoming requests and rate limits requests when they are coming at too fast a rate. The rule categorizes requests according to your aggregation criteria, collects them into aggregation instances, and counts and rate limits the requests for each instance.
///
/// You can specify individual aggregation keys, like IP address or HTTP method. You can also specify aggregation key combinations, like IP address and HTTP method, or HTTP method, query argument, and cookie.
///
/// Each unique set of values for the aggregation keys that you specify is a separate aggregation instance, with the value from each key contributing to the aggregation instance definition.
///
/// For example, assume the rule evaluates web requests with the following IP address and HTTP method values:
///
/// The rule would create different aggregation instances according to your aggregation criteria, for example:
///
/// For any n-tuple of aggregation keys, each unique combination of values for the keys defines a separate aggregation instance, which AWS WAF counts and rate-limits individually.
///
/// You can optionally nest another statement inside the rate-based statement, to narrow the scope of the rule so that it only counts and rate limits requests that match the nested statement. You can use this nested scope-down statement in conjunction with your aggregation key specifications or you can just count and rate limit all requests that match the scope-down statement, without additional aggregation. When you choose to just manage all requests that match a scope-down statement, the aggregation instance is singular for the rule.
///
/// You cannot nest a RateBasedStatement inside another statement, for example inside a NotStatement or OrStatement. You can define a RateBasedStatement inside a web ACL and inside a rule group.
///
/// For additional information about the options, see Rate limiting web requests using rate-based rules   in the         AWS WAF Developer Guide.
///
/// If you only aggregate on the individual IP address or forwarded IP address, you can retrieve the list of IP addresses that AWS WAF      is currently rate limiting for a rule through the API call GetRateBasedStatementManagedKeys. This option is not available    for other aggregation configurations.
///
/// AWS WAF tracks and manages web requests separately for each instance of a rate-based rule that you use. For example, if you provide the same rate-based rule settings in two web ACLs, each of the two rule statements represents a separate instance of the rate-based rule and gets its own tracking and management by AWS WAF. If you define a rate-based rule inside a rule group, and then use that rule group in multiple places, each use creates a separate instance of the rate-based rule that gets its own tracking and management by AWS WAF.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RateBasedStatement {


    /// 
    /// Setting that indicates how to aggregate the request counts.
    /// 
    /// NoteWeb requests that are missing any of the components specified in the aggregation keys         are omitted from the rate-based rule evaluation and handling.
    /// 
    /// CONSTANT - Count and limit the requests that match the rate-based rule's scope-down        statement. With this option, the counted requests aren't further aggregated. The scope-down statement          is the only specification used. When the count of all requests that satisfy the scope-down statement         goes over the limit, AWS WAF applies the rule action to all requests that satisfy the scope-down statement.         With this option, you must configure the ScopeDownStatement property.                         CUSTOM_KEYS - Aggregate the request counts using one or more web request components as the aggregate keys.        With this option, you must specify the aggregate keys in the CustomKeys property.         To aggregate on only the IP address or only the forwarded IP address, don't use custom keys. Instead, set the aggregate         key type to IP or FORWARDED_IP.                        FORWARDED_IP - Aggregate the request counts on the first IP address in an HTTP header.         With this option, you must specify the header to use in the ForwardedIPConfig property.         To aggregate on a combination of the forwarded IP address with other aggregate keys, use CUSTOM_KEYS.                         IP - Aggregate the request counts on the IP address from the web request        origin.        To aggregate on a combination of the IP address with other aggregate keys, use CUSTOM_KEYS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONSTANT | CUSTOM_KEYS | FORWARDED_IP | IP
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregateKeyType")]
    pub aggregate_key_type: RateBasedStatementAggregateKeyTypeEnum,


    /// 
    /// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. Commonly, this is the X-Forwarded-For (XFF) header, but you can specify any header name.
    /// 
    /// NoteIf the specified header isn't present in the request, AWS WAF doesn't apply the rule to the web request at all.
    /// 
    /// This is required if you specify a forwarded IP in the rule's aggregate key settings.
    /// 
    /// Required: No
    ///
    /// Type: ForwardedIPConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,


    /// 
    /// The limit on requests per 5-minute period for a single aggregation instance for the rate-based rule.     If the rate-based statement includes a ScopeDownStatement, this limit is applied only to the     requests that match the statement.
    /// 
    /// Examples:
    /// 
    /// If you aggregate on just the IP address, this is the limit on requests from any single IP address.               If you aggregate on the HTTP method and the query argument name "city", then this is the limit on       requests for any single method, city pair.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Limit")]
    pub limit: i64,


    /// 
    /// An optional nested statement that narrows the scope of the web requests that are     evaluated by the rate-based statement. Requests are only tracked by the rate-based     statement if they match the scope-down statement. You can use any nestable statement in the scope-down statement, and you can nest statements at any     level, the same as you can for a rule statement.
    /// 
    /// Required: No
    ///
    /// Type: Statement
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScopeDownStatement")]
    pub scope_down_statement: Option<Statement>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RateBasedStatementAggregateKeyTypeEnum {

    /// CONSTANT
    #[serde(rename = "CONSTANT")]
    Constant,

    /// CUSTOM_KEYS
    #[serde(rename = "CUSTOM_KEYS")]
    Customkeys,

    /// FORWARDED_IP
    #[serde(rename = "FORWARDED_IP")]
    Forwardedip,

    /// IP
    #[serde(rename = "IP")]
    Ip,

}

impl Default for RateBasedStatementAggregateKeyTypeEnum {
    fn default() -> Self {
        RateBasedStatementAggregateKeyTypeEnum::Constant
    }
}


impl cfn_resources::CfnResource for RateBasedStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.forwarded_ipconfig.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.scope_down_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A rule statement used to search web request components for a match against a single regular expression.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RegexMatchStatement {


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// The string representing the regular expression.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegexString")]
    pub regex_string: String,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}



impl cfn_resources::CfnResource for RegexMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        let the_val = &self.regex_string;

        if the_val.len() > 512 as _ {
            return Err(format!("Max validation failed on field 'regex_string'. {} is greater than 512", the_val.len()));
        }

        
        let the_val = &self.regex_string;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'regex_string'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement used to search web request components for matches with regular expressions. To use this, create a AWS::WAFv2::RegexPatternSet that specifies the expressions that you want to detect, then use the ARN of that set in this statement. A web request matches the pattern set rule statement if the request component matches any of the patterns in the set.
///
/// Each regex pattern set rule statement references a regex pattern set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RegexPatternSetReferenceStatement {


    /// 
    /// The Amazon Resource Name (ARN) of the AWS::WAFv2::RegexPatternSet that this     statement references.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: String,


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}



impl cfn_resources::CfnResource for RegexPatternSetReferenceStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.arn;

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'arn'. {} is greater than 2048", the_val.len()));
        }

        
        let the_val = &self.arn;

        if the_val.len() < 20 as _ {
            return Err(format!("Min validation failed on field 'arn'. {} is less than 20", the_val.len()));
        }

        
        self.field_to_match.validate()?;

        Ok(())
    }
}

/// A single rule, which you can use in a AWS::WAFv2::WebACL or AWS::WAFv2::RuleGroup to identify web requests that you want to allow, block, or count.     Each rule includes one top-level Statement that AWS WAF uses to     identify matching web requests, and parameters that govern how AWS WAF handles them.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rule {


    /// 
    /// The action that AWS WAF should take on a web request when it matches the rule statement. Settings at the web ACL level can override the rule action setting.
    /// 
    /// Required: No
    ///
    /// Type: RuleAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Option<RuleAction>,


    /// 
    /// Specifies how AWS WAF should handle CAPTCHA evaluations. If you don't specify this, AWS WAF uses the CAPTCHA configuration that's defined for the web ACL.
    /// 
    /// Required: No
    ///
    /// Type: CaptchaConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaptchaConfig")]
    pub captcha_config: Option<CaptchaConfig>,


    /// 
    /// Specifies how AWS WAF should handle Challenge evaluations. If you don't specify this, AWS WAF uses the challenge configuration that's defined for the web ACL.
    /// 
    /// Required: No
    ///
    /// Type: ChallengeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChallengeConfig")]
    pub challenge_config: Option<ChallengeConfig>,


    /// 
    /// The name of the rule. You can't change the name of a Rule after you create     it.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\w\-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// If you define more than one Rule in a WebACL, AWS WAF     evaluates each request against the Rules in order based on the value of       Priority. AWS WAF processes rules with lower priority first. The priorities     don't need to be consecutive, but they must all be different.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// Labels to apply to web requests that match the rule match statement. AWS WAF applies     fully qualified labels to matching web requests. A fully qualified label is the     concatenation of a label namespace and a rule label. The rule's rule group or web ACL     defines the label namespace.
    /// 
    /// Rules that run after this rule in the web ACL can match against these labels using a       LabelMatchStatement.
    /// 
    /// For each label, provide a case-sensitive string containing optional namespaces and a     label name, according to the following guidelines:
    /// 
    /// Separate each component of the label with a colon.               Each namespace or name can have up to 128 characters.               You can specify up to 5 namespaces in a label.               Don't use the following reserved words in your label specification:          aws, waf, managed, rulegroup,          webacl, regexpatternset, or ipset.
    /// 
    /// For example, myLabelName or nameSpace1:nameSpace2:myLabelName.
    /// 
    /// Required: No
    ///
    /// Type: List of Label
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleLabels")]
    pub rule_labels: Option<Vec<Label>>,


    /// 
    /// The AWS WAF processing statement for the rule, for example ByteMatchStatement or SizeConstraintStatement.
    /// 
    /// Required: Yes
    ///
    /// Type: Statement
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statement")]
    pub statement: Statement,


    /// 
    /// Defines and enables Amazon CloudWatch metrics and web request sample collection.
    /// 
    /// Required: Yes
    ///
    /// Type: VisibilityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,

}



impl cfn_resources::CfnResource for Rule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.action.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.captcha_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.challenge_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 128", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.priority;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'priority'. {} is less than 0", the_val));
        }

        
        self.statement.validate()?;

        self.visibility_config.validate()?;

        Ok(())
    }
}

/// The action that AWS WAF should take on a web request when it matches a rule's     statement. Settings at the web ACL level can override the rule action setting.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RuleAction {


    /// 
    /// Instructs AWS WAF to allow the web request.
    /// 
    /// Required: No
    ///
    /// Type: AllowAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Allow")]
    pub allow: Option<AllowAction>,


    /// 
    /// Instructs AWS WAF to block the web request.
    /// 
    /// Required: No
    ///
    /// Type: BlockAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Block")]
    pub block: Option<BlockAction>,


    /// 
    /// Specifies that AWS WAF should run a CAPTCHA check against the request:
    /// 
    /// If the request includes a valid, unexpired CAPTCHA token,        AWS WAF allows the web request inspection to           proceed to the next rule, similar to a CountAction.               If the request doesn't include a valid, unexpired CAPTCHA token, AWS WAF           discontinues the web ACL evaluation of the request and blocks it from going to its intended destination.                  AWS WAF generates a response that it sends back to the client, which includes the following:                                                            The header x-amzn-waf-action with a value of captcha.                       The HTTP status code 405 Method Not Allowed.                       If the request contains an Accept header with a value of text/html, the response includes a CAPTCHA challenge.
    /// 
    /// You can configure the expiration time         in the CaptchaConfig       ImmunityTimeProperty setting at the rule and web ACL level. The rule setting overrides the web ACL setting.
    /// 
    /// This action option is available for rules. It isn't available for web ACL default actions.
    /// 
    /// Required: No
    ///
    /// Type: CaptchaAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Captcha")]
    pub captcha: Option<CaptchaAction>,


    /// 
    /// Instructs AWS WAF to run a Challenge check against the web request.
    /// 
    /// Required: No
    ///
    /// Type: ChallengeAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Challenge")]
    pub challenge: Option<ChallengeAction>,


    /// 
    /// Instructs AWS WAF to count the web request and then continue evaluating the request using the remaining rules in the web ACL.
    /// 
    /// Required: No
    ///
    /// Type: CountAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<CountAction>,

}



impl cfn_resources::CfnResource for RuleAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.allow.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.block.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.captcha.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.challenge.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.count.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Inspect one of the headers in the web request, identified by name, for example,       User-Agent or Referer. The name isn't case sensitive.
///
/// You can filter and inspect all headers with the FieldToMatch setting       Headers.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
///
/// Example JSON: "SingleHeader": { "Name": "haystack" }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SingleHeader {


    /// 
    /// The name of the query header to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for SingleHeader {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// Inspect one query argument in the web request, identified by name, for example       UserName or SalesRegion. The name isn't case     sensitive.
///
/// This is used to indicate the web request component to inspect, in the FieldToMatch specification.
///
/// Example JSON: "SingleQueryArgument": { "Name": "myArgument" }
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SingleQueryArgument {


    /// 
    /// The name of the query argument to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}



impl cfn_resources::CfnResource for SingleQueryArgument {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.name;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). For example, you can use a size constraint statement to look for query strings that are longer than 100 bytes.
///
/// If you configure AWS WAF to inspect the request body, AWS WAF inspects only the number of bytes of the body up to the limit for the web ACL. By default, for regional web ACLs, this limit is 8 KB (8,192 kilobytes) and for CloudFront web ACLs, this limit is 16 KB (16,384 kilobytes). For CloudFront web ACLs, you can increase the limit in the web ACL AssociationConfig, for additional fees. If you know that the request body for your web requests should never exceed the inspection limit, you could use a size constraint statement to block requests that have a larger request body size.
///
/// If you choose URI for the value of Part of the request to filter on, the slash (/) in the URI counts as one character. For example, the URI /logo.jpg is nine characters long.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SizeConstraintStatement {


    /// 
    /// The operator to use to compare the request part to the size setting.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EQ | GE | GT | LE | LT | NE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: SizeConstraintStatementComparisonOperatorEnum,


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// The size, in byte, to compare to the request part, after any transformations.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Size")]
    pub size: f64,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SizeConstraintStatementComparisonOperatorEnum {

    /// EQ
    #[serde(rename = "EQ")]
    Eq,

    /// GE
    #[serde(rename = "GE")]
    Ge,

    /// GT
    #[serde(rename = "GT")]
    Gt,

    /// LE
    #[serde(rename = "LE")]
    Le,

    /// LT
    #[serde(rename = "LT")]
    Lt,

    /// NE
    #[serde(rename = "NE")]
    Ne,

}

impl Default for SizeConstraintStatementComparisonOperatorEnum {
    fn default() -> Self {
        SizeConstraintStatementComparisonOperatorEnum::Eq
    }
}


impl cfn_resources::CfnResource for SizeConstraintStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        Ok(())
    }
}

/// A rule statement that inspects for malicious SQL code. Attackers insert malicious SQL code into web requests to do things like modify your database or extract data from it.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SqliMatchStatement {


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// The sensitivity that you want AWS WAF to use to inspect for SQL injection attacks.
    /// 
    /// HIGH detects more attacks, but might generate more false positives,     especially if your web requests frequently contain unusual strings.     For information about identifying and mitigating false positives, see       Testing and tuning in the                                                     AWS WAF Developer Guide.
    /// 
    /// LOW is generally a better choice for resources that already have other       protections against SQL injection attacks or that have a low tolerance for false positives.
    /// 
    /// Default: LOW
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HIGH | LOW
    ///
    /// Update requires: No interruption
    #[serde(rename = "SensitivityLevel")]
    pub sensitivity_level: Option<SqliMatchStatementSensitivityLevelEnum>,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SqliMatchStatementSensitivityLevelEnum {

    /// HIGH
    #[serde(rename = "HIGH")]
    High,

    /// LOW
    #[serde(rename = "LOW")]
    Low,

}

impl Default for SqliMatchStatementSensitivityLevelEnum {
    fn default() -> Self {
        SqliMatchStatementSensitivityLevelEnum::High
    }
}


impl cfn_resources::CfnResource for SqliMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        Ok(())
    }
}

/// The processing guidance for a rule, used by AWS WAF to determine whether a web request matches the rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Statement {


    /// 
    /// A logical rule statement used to combine other rule statements with AND logic. You provide more than one Statement within the AndStatement.
    /// 
    /// Required: No
    ///
    /// Type: AndStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "AndStatement")]
    pub and_statement: Option<AndStatement>,


    /// 
    /// A rule statement that defines a string match search for AWS WAF to apply to web requests. The byte match statement provides the bytes to search for, the location in requests that you want AWS WAF to search, and other settings. The bytes to search for are typically a string that corresponds with ASCII characters. In the AWS WAF console and the developer guide, this is called a string match statement.
    /// 
    /// Required: No
    ///
    /// Type: ByteMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "ByteMatchStatement")]
    pub byte_match_statement: Option<ByteMatchStatement>,


    /// 
    /// A rule statement that labels web requests by country and region and that matches against web requests based on country code. A geo match rule labels every request that it inspects regardless of whether it finds a match.
    /// 
    /// To manage requests only by country, you can use this statement by itself and specify the countries that you want to match against in the CountryCodes array.               Otherwise, configure your geo match rule with Count action so that it only labels requests. Then, add one or more label match rules to run after the geo match rule and configure them to match against the geographic labels and handle the requests as needed.
    /// 
    /// AWS WAF labels requests using the alpha-2 country and region codes from the International Organization for Standardization (ISO) 3166 standard. AWS WAF determines the codes using either the IP address in the web request origin or, if you specify it, the address in the geo match ForwardedIPConfig.
    /// 
    /// If you use the web request origin, the label formats are awswaf:clientip:geo:region:<ISO country code>-<ISO region code> and awswaf:clientip:geo:country:<ISO country code>.
    /// 
    /// If you use a forwarded IP address, the label formats are awswaf:forwardedip:geo:region:<ISO country code>-<ISO region code> and awswaf:forwardedip:geo:country:<ISO country code>.
    /// 
    /// For additional details, see Geographic match rule statement in the AWS WAF Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: GeoMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "GeoMatchStatement")]
    pub geo_match_statement: Option<GeoMatchStatement>,


    /// 
    /// A rule statement used to detect web requests coming from particular IP addresses or address ranges. To use this, create an AWS::WAFv2::IPSet that specifies the addresses you want to detect, then use the ARN of that set in this statement.
    /// 
    /// Each IP set rule statement references an IP set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.
    /// 
    /// Required: No
    ///
    /// Type: IPSetReferenceStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPSetReferenceStatement")]
    pub ipset_reference_statement: Option<IPSetReferenceStatement>,


    /// 
    /// A rule statement to match against labels that have been added to the web request by rules that have already run in the web ACL.
    /// 
    /// The label match statement provides the label or namespace string to search for. The label string can represent a part or all of the fully qualified label name that had been added to the web request. Fully qualified labels have a prefix, optional namespaces, and label name. The prefix identifies the rule group or web ACL context of the rule that added the label. If you do not provide the fully qualified name in your label match string, AWS WAF performs the search for labels that were added in the same context as the label match statement.
    /// 
    /// Required: No
    ///
    /// Type: LabelMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelMatchStatement")]
    pub label_match_statement: Option<LabelMatchStatement>,


    /// 
    /// A logical rule statement used to negate the results of another rule statement. You provide one Statement within the NotStatement.
    /// 
    /// Required: No
    ///
    /// Type: NotStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotStatement")]
    pub not_statement: Option<Box<NotStatement>>,


    /// 
    /// A logical rule statement used to combine other rule statements with OR logic. You provide more than one Statement within the OrStatement.
    /// 
    /// Required: No
    ///
    /// Type: OrStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "OrStatement")]
    pub or_statement: Option<OrStatement>,


    /// 
    /// A rate-based rule counts incoming requests and rate limits requests when they are coming at too fast a rate. The rule categorizes requests according to your aggregation criteria, collects them into aggregation instances, and counts and rate limits the requests for each instance.
    /// 
    /// You can specify individual aggregation keys, like IP address or HTTP method. You can also specify aggregation key combinations, like IP address and HTTP method, or HTTP method, query argument, and cookie.
    /// 
    /// Each unique set of values for the aggregation keys that you specify is a separate aggregation instance, with the value from each key contributing to the aggregation instance definition.
    /// 
    /// For example, assume the rule evaluates web requests with the following IP address and HTTP method values:
    /// 
    /// IP address 10.1.1.1, HTTP method POST               IP address 10.1.1.1, HTTP method GET               IP address 127.0.0.0, HTTP method POST               IP address 10.1.1.1, HTTP method GET
    /// 
    /// The rule would create different aggregation instances according to your aggregation criteria, for example:
    /// 
    /// If the aggregation criteria is just the IP address, then each individual address is an aggregation instance, and AWS WAF counts requests separately for each. The aggregation instances and request counts for our example would be the following:                                                IP address 10.1.1.1: count 3                     IP address 127.0.0.0: count 1                        If the aggregation criteria is HTTP method, then each individual HTTP method is an aggregation instance. The aggregation instances and request counts for our example would be the following:                                                HTTP method POST: count 2                     HTTP method GET: count 2                        If the aggregation criteria is IP address and HTTP method, then each IP address and each HTTP method would contribute to the combined aggregation instance. The aggregation instances and request counts for our example would be the following:                                                          IP address 10.1.1.1, HTTP method POST: count 1                     IP address 10.1.1.1, HTTP method GET: count 2                     IP address 127.0.0.0, HTTP method POST: count 1
    /// 
    /// For any n-tuple of aggregation keys, each unique combination of values for the keys defines a separate aggregation instance, which AWS WAF counts and rate-limits individually.
    /// 
    /// You can optionally nest another statement inside the rate-based statement, to narrow the scope of the rule so that it only counts and rate limits requests that match the nested statement. You can use this nested scope-down statement in conjunction with your aggregation key specifications or you can just count and rate limit all requests that match the scope-down statement, without additional aggregation. When you choose to just manage all requests that match a scope-down statement, the aggregation instance is singular for the rule.
    /// 
    /// You cannot nest a RateBasedStatement inside another statement, for example inside a NotStatement or OrStatement. You can define a RateBasedStatement inside a web ACL and inside a rule group.
    /// 
    /// For additional information about the options, see Rate limiting web requests using rate-based rules   in the         AWS WAF Developer Guide.
    /// 
    /// If you only aggregate on the individual IP address or forwarded IP address, you can retrieve the list of IP addresses that AWS WAF      is currently rate limiting for a rule through the API call GetRateBasedStatementManagedKeys. This option is not available    for other aggregation configurations.
    /// 
    /// AWS WAF tracks and manages web requests separately for each instance of a rate-based rule that you use. For example, if you provide the same rate-based rule settings in two web ACLs, each of the two rule statements represents a separate instance of the rate-based rule and gets its own tracking and management by AWS WAF. If you define a rate-based rule inside a rule group, and then use that rule group in multiple places, each use creates a separate instance of the rate-based rule that gets its own tracking and management by AWS WAF.
    /// 
    /// Required: No
    ///
    /// Type: RateBasedStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "RateBasedStatement")]
    pub rate_based_statement: Option<Box<RateBasedStatement>>,


    /// 
    /// A rule statement used to search web request components for a match against a single regular expression.
    /// 
    /// Required: No
    ///
    /// Type: RegexMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegexMatchStatement")]
    pub regex_match_statement: Option<RegexMatchStatement>,


    /// 
    /// A rule statement used to search web request components for matches with regular expressions. To use this, create a AWS::WAFv2::RegexPatternSet that specifies the expressions that you want to detect, then use the ARN of that set in this statement. A web request matches the pattern set rule statement if the request component matches any of the patterns in the set.
    /// 
    /// Each regex pattern set rule statement references a regex pattern set. You create and maintain the set independent of your rules. This allows you to use the single set in multiple rules. When you update the referenced set, AWS WAF automatically updates all rules that reference it.
    /// 
    /// Required: No
    ///
    /// Type: RegexPatternSetReferenceStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegexPatternSetReferenceStatement")]
    pub regex_pattern_set_reference_statement: Option<RegexPatternSetReferenceStatement>,


    /// 
    /// A rule statement that compares a number of bytes against the size of a request component, using a comparison operator, such as greater than (>) or less than (<). For example, you can use a size constraint statement to look for query strings that are longer than 100 bytes.
    /// 
    /// If you configure AWS WAF to inspect the request body, AWS WAF inspects only the number of bytes of the body up to the limit for the web ACL. By default, for regional web ACLs, this limit is 8 KB (8,192 kilobytes) and for CloudFront web ACLs, this limit is 16 KB (16,384 kilobytes). For CloudFront web ACLs, you can increase the limit in the web ACL AssociationConfig, for additional fees. If you know that the request body for your web requests should never exceed the inspection limit, you could use a size constraint statement to block requests that have a larger request body size.
    /// 
    /// If you choose URI for the value of Part of the request to filter on, the slash (/) in the URI counts as one character. For example, the URI /logo.jpg is nine characters long.
    /// 
    /// Required: No
    ///
    /// Type: SizeConstraintStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeConstraintStatement")]
    pub size_constraint_statement: Option<SizeConstraintStatement>,


    /// 
    /// A rule statement that inspects for malicious SQL code. Attackers insert malicious SQL code into web requests to do things like modify your database or extract data from it.
    /// 
    /// Required: No
    ///
    /// Type: SqliMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqliMatchStatement")]
    pub sqli_match_statement: Option<SqliMatchStatement>,


    /// 
    /// A rule statement that inspects for cross-site scripting (XSS) attacks. In XSS attacks, the attacker uses vulnerabilities in a benign website as a vehicle to inject malicious client-site scripts into other legitimate web browsers.
    /// 
    /// Required: No
    ///
    /// Type: XssMatchStatement
    ///
    /// Update requires: No interruption
    #[serde(rename = "XssMatchStatement")]
    pub xss_match_statement: Option<XssMatchStatement>,

}



impl cfn_resources::CfnResource for Statement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.and_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.byte_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.geo_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.ipset_reference_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.label_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.not_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.or_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.rate_based_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.regex_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.regex_pattern_set_reference_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.size_constraint_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sqli_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.xss_match_statement.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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



impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Text transformations eliminate some of the unusual formatting that attackers use in web     requests in an effort to bypass detection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TextTransformation {


    /// 
    /// Sets the relative processing order for multiple transformations.     AWS WAF processes all transformations, from lowest priority to highest,     before inspecting the transformed content. The priorities don't need to be consecutive, but     they must all be different.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// You can specify the following transformation types:
    /// 
    /// BASE64_DECODE - Decode a     Base64-encoded string.
    /// 
    /// BASE64_DECODE_EXT - Decode a     Base64-encoded string, but use a forgiving implementation that ignores     characters that aren't valid.
    /// 
    /// CMD_LINE - Command-line transformations. These are     helpful in reducing effectiveness of attackers who inject an operating system command-line      command and use unusual formatting to disguise some or all of the command.
    /// 
    /// Delete the following characters: \ " ' ^                       Delete spaces before the following characters: / (                       Replace the following characters with a space: , ;                       Replace multiple spaces with one space               Convert uppercase letters (A-Z) to lowercase (a-z)
    /// 
    /// COMPRESS_WHITE_SPACE - Replace these characters     with a space character (decimal 32):
    /// 
    /// \f, formfeed, decimal 12                        \t, tab, decimal 9                        \n, newline, decimal 10                        \r, carriage return, decimal 13                        \v, vertical tab, decimal 11               Non-breaking space, decimal 160
    /// 
    /// COMPRESS_WHITE_SPACE also replaces multiple spaces with one space.
    /// 
    /// CSS_DECODE - Decode characters that were encoded     using CSS 2.x escape rules syndata.html#characters. This function uses up to     two bytes in the decoding process, so it can help to uncover ASCII characters that were     encoded using CSS encoding that wouldn’t typically be encoded. It's also useful in     countering evasion, which is a combination of a backslash and non-hexadecimal characters.     For example, ja\vascript for javascript.
    /// 
    /// ESCAPE_SEQ_DECODE - Decode the following ANSI C     escape sequences: \a, \b, \f, \n,       \r, \t, \v, \\, \?,       \', \", \xHH (hexadecimal), \0OOO     (octal). Encodings that aren't valid remain in the output.
    /// 
    /// HEX_DECODE - Decode a string of hexadecimal     characters into a binary.
    /// 
    /// HTML_ENTITY_DECODE - Replace HTML-encoded     characters with unencoded characters. HTML_ENTITY_DECODE performs these     operations:
    /// 
    /// Replaces (ampersand)quot; with "                       Replaces (ampersand)nbsp; with a non-breaking space, decimal        160               Replaces (ampersand)lt; with a "less than" symbol               Replaces (ampersand)gt; with >                       Replaces characters that are represented in hexadecimal format,          (ampersand)#xhhhh;, with the corresponding characters               Replaces characters that are represented in decimal format,          (ampersand)#nnnn;, with the corresponding characters
    /// 
    /// JS_DECODE - Decode JavaScript escape sequences. If     a     \       u       HHHH     code is in the full-width ASCII code range of FF01-FF5E, then the higher byte     is used to detect and adjust the lower byte. If not, only the lower byte is used and the     higher byte is zeroed, causing a possible loss of information.
    /// 
    /// LOWERCASE - Convert uppercase letters (A-Z) to     lowercase (a-z).
    /// 
    /// MD5 - Calculate an MD5 hash from the data in the     input. The computed hash is in a raw binary form.
    /// 
    /// NONE - Specify NONE if you don't want     any text transformations.
    /// 
    /// NORMALIZE_PATH - Remove multiple slashes, directory     self-references, and directory back-references that are not at the beginning of the input     from an input string.
    /// 
    /// NORMALIZE_PATH_WIN - This is the same as       NORMALIZE_PATH, but first converts backslash characters to forward slashes.
    /// 
    /// REMOVE_NULLS - Remove all NULL bytes     from the input.
    /// 
    /// REPLACE_COMMENTS - Replace each occurrence of a     C-style comment (/* ... */) with a single space. Multiple consecutive     occurrences are not compressed. Unterminated comments are also replaced with a space (ASCII     0x20). However, a standalone termination of a comment (*/) is not acted upon.
    /// 
    /// REPLACE_NULLS - Replace NULL bytes in the input     with space characters (ASCII 0x20).
    /// 
    /// SQL_HEX_DECODE - Decode SQL hex data. Example       (0x414243) will be decoded to (ABC).
    /// 
    /// URL_DECODE - Decode a URL-encoded value.
    /// 
    /// URL_DECODE_UNI - Like URL_DECODE, but     with support for Microsoft-specific %u encoding. If the code is in the     full-width ASCII code range of FF01-FF5E, the higher byte is used to detect     and adjust the lower byte. Otherwise, only the lower byte is used and the higher byte is     zeroed.
    /// 
    /// UTF8_TO_UNICODE - Convert all UTF-8 character     sequences to Unicode. This helps input normalization, and minimizing false-positives and     false-negatives for non-English languages.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BASE64_DECODE | BASE64_DECODE_EXT | CMD_LINE | COMPRESS_WHITE_SPACE | CSS_DECODE | ESCAPE_SEQ_DECODE | HEX_DECODE | HTML_ENTITY_DECODE | JS_DECODE | LOWERCASE | MD5 | NONE | NORMALIZE_PATH | NORMALIZE_PATH_WIN | REMOVE_NULLS | REPLACE_COMMENTS | REPLACE_NULLS | SQL_HEX_DECODE | URL_DECODE | URL_DECODE_UNI | UTF8_TO_UNICODE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: TextTransformationTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TextTransformationTypeEnum {

    /// BASE64_DECODE
    #[serde(rename = "BASE64_DECODE")]
    Base64decode,

    /// BASE64_DECODE_EXT
    #[serde(rename = "BASE64_DECODE_EXT")]
    Base64decodeext,

    /// CMD_LINE
    #[serde(rename = "CMD_LINE")]
    Cmdline,

    /// COMPRESS_WHITE_SPACE
    #[serde(rename = "COMPRESS_WHITE_SPACE")]
    Compresswhitespace,

    /// CSS_DECODE
    #[serde(rename = "CSS_DECODE")]
    Cssdecode,

    /// ESCAPE_SEQ_DECODE
    #[serde(rename = "ESCAPE_SEQ_DECODE")]
    Escapeseqdecode,

    /// HEX_DECODE
    #[serde(rename = "HEX_DECODE")]
    Hexdecode,

    /// HTML_ENTITY_DECODE
    #[serde(rename = "HTML_ENTITY_DECODE")]
    Htmlentitydecode,

    /// JS_DECODE
    #[serde(rename = "JS_DECODE")]
    Jsdecode,

    /// LOWERCASE
    #[serde(rename = "LOWERCASE")]
    Lowercase,

    /// MD5
    #[serde(rename = "MD5")]
    Md5,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// NORMALIZE_PATH
    #[serde(rename = "NORMALIZE_PATH")]
    Normalizepath,

    /// NORMALIZE_PATH_WIN
    #[serde(rename = "NORMALIZE_PATH_WIN")]
    Normalizepathwin,

    /// REMOVE_NULLS
    #[serde(rename = "REMOVE_NULLS")]
    Removenulls,

    /// REPLACE_COMMENTS
    #[serde(rename = "REPLACE_COMMENTS")]
    Replacecomments,

    /// REPLACE_NULLS
    #[serde(rename = "REPLACE_NULLS")]
    Replacenulls,

    /// SQL_HEX_DECODE
    #[serde(rename = "SQL_HEX_DECODE")]
    Sqlhexdecode,

    /// URL_DECODE
    #[serde(rename = "URL_DECODE")]
    Urldecode,

    /// URL_DECODE_UNI
    #[serde(rename = "URL_DECODE_UNI")]
    Urldecodeuni,

    /// UTF8_TO_UNICODE
    #[serde(rename = "UTF8_TO_UNICODE")]
    Utf8tounicode,

}

impl Default for TextTransformationTypeEnum {
    fn default() -> Self {
        TextTransformationTypeEnum::Base64decode
    }
}


impl cfn_resources::CfnResource for TextTransformation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.priority;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'priority'. {} is less than 0", the_val));
        }

        
        Ok(())
    }
}

/// Defines and enables Amazon CloudWatch metrics and web request sample collection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VisibilityConfig {


    /// 
    /// A boolean indicating whether the associated resource sends metrics to Amazon CloudWatch. For the     list of available metrics, see AWS WAF       Metrics in the         AWS WAF Developer Guide.
    /// 
    /// For web ACLs, the metrics are for web requests that have the web ACL default action applied.     AWS WAF applies the default action to web requests that pass the inspection of all rules     in the web ACL without being either allowed or blocked. For more information, see The web ACL default action in the         AWS WAF Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchMetricsEnabled")]
    pub cloud_watch_metrics_enabled: bool,


    /// 
    /// A name of the Amazon CloudWatch metric dimension. The name can contain only the characters: A-Z, a-z, 0-9,     - (hyphen), and _ (underscore). The name can be from one to 128 characters long. It can't    contain whitespace or metric names that are reserved for AWS WAF, for example All and    Default_Action.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[\w#:\.\-/]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// A boolean indicating whether AWS WAF should store a sampling of the web requests that     match the rules. You can view the sampled requests through the AWS WAF console.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SampledRequestsEnabled")]
    pub sampled_requests_enabled: bool,

}



impl cfn_resources::CfnResource for VisibilityConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.metric_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'metric_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.metric_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'metric_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// A rule statement that inspects for cross-site scripting (XSS) attacks. In XSS attacks, the attacker uses vulnerabilities in a benign website as a vehicle to inject malicious client-site scripts into other legitimate web browsers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct XssMatchStatement {


    /// 
    /// The part of the web request that you want AWS WAF to inspect.
    /// 
    /// Required: Yes
    ///
    /// Type: FieldToMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,


    /// 
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. Text transformations are used in rule match statements, to transform the FieldToMatch request component before inspecting it, and they're used in rate-based rule statements, to transform request components before using them as custom aggregation keys. If you specify one or more transformations to apply, AWS WAF performs all transformations on the specified content, starting from the lowest priority setting, and then uses the component contents.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TextTransformation
    ///
    /// Update requires: No interruption
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}



impl cfn_resources::CfnResource for XssMatchStatement {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.field_to_match.validate()?;

        Ok(())
    }
}