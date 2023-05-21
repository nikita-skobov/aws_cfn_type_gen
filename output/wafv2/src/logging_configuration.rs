/// Defines an association between logging destinations and a web ACL resource, for logging     from AWS WAF. As part of the association, you can specify parts of the standard logging     fields to keep out of the logs and you can specify filters so that you log only a subset of     the logging records.
///
/// You can access information about the traffic that AWS WAF inspects using the following     steps:
///
/// When you successfully enable logging using a PutLoggingConfiguration      request, AWS WAF creates an additional role or policy that is required to write        logs to the logging destination. For an Amazon CloudWatch Logs log group, AWS WAF creates a resource policy on the log group.      For an Amazon S3 bucket, AWS WAF creates a bucket policy. For an Amazon Kinesis Data Firehose, AWS WAF creates a service-linked role.
///
/// For additional information about web ACL logging, see       Logging web ACL traffic information         in the         AWS WAF Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoggingConfiguration {
    ///
    /// The logging destination configuration that you want to associate with the web     ACL.
    ///
    /// NoteYou can associate one logging destination to a web ACL.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<String>,

    ///
    /// Filtering that specifies which web requests are kept in the logs and which are dropped.     You can filter on the rule action and on the web request labels that were applied by     matching rules during web ACL evaluation.
    ///
    /// Required: No
    ///
    /// Type: LoggingFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingFilter")]
    pub logging_filter: Option<LoggingFilter>,

    ///
    /// The parts of the request that you want to keep out of the logs. For example, if you     redact the SingleHeader field, the HEADER field in the logs will     be REDACTED.
    ///
    /// NoteYou can specify only the following fields for redaction: UriPath,        QueryString, SingleHeader, Method, and        JsonBody.
    ///
    /// Required: No
    ///
    /// Type: List of FieldToMatch
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "RedactedFields")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,

    ///
    /// The Amazon Resource Name (ARN) of the web ACL that you want to associate with       LogDestinationConfigs.
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
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

impl cfn_resources::CfnResource for CfnLoggingConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::WAFv2::LoggingConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.log_destination_configs;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'log_destination_configs'. {} is greater than 100",
                the_val.len()
            ));
        }

        self.logging_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.redacted_fields {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'redacted_fields'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.resource_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'resource_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.resource_arn;

        if the_val.len() < 20 as _ {
            return Err(format!(
                "Min validation failed on field 'resource_arn'. {} is less than 20",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// A single action condition for a condition in a logging filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ActionCondition {
    ///
    /// The action setting that a log record must contain in order to meet the condition. This is the action that AWS WAF applied to the web request.
    ///
    /// For rule groups, this is either the configured rule action setting, or if you've applied a rule action override to the rule, it's the override action.     The value EXCLUDED_AS_COUNT matches on     excluded rules and also on rules that have a rule action override of Count.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOW | BLOCK | CAPTCHA | CHALLENGE | COUNT | EXCLUDED_AS_COUNT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: ActionConditionActionEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ActionConditionActionEnum {
    /// ALLOW
    #[serde(rename = "ALLOW")]
    Allow,

    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,

    /// CAPTCHA
    #[serde(rename = "CAPTCHA")]
    Captcha,

    /// CHALLENGE
    #[serde(rename = "CHALLENGE")]
    Challenge,

    /// COUNT
    #[serde(rename = "COUNT")]
    Count,

    /// EXCLUDED_AS_COUNT
    #[serde(rename = "EXCLUDED_AS_COUNT")]
    Excludedascount,
}

impl Default for ActionConditionActionEnum {
    fn default() -> Self {
        ActionConditionActionEnum::Allow
    }
}

impl cfn_resources::CfnResource for ActionCondition {
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

/// A single match condition for a log filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Condition {
    ///
    /// A single action condition. This is the action setting that a log record must contain in order to meet the condition.
    ///
    /// Required: No
    ///
    /// Type: ActionCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionCondition")]
    pub action_condition: Option<ActionCondition>,

    ///
    /// A single label name condition. This is the fully qualified label name that a log record must contain in order to meet the condition.     Fully qualified labels have a prefix, optional namespaces, and label name. The prefix identifies the rule group or web ACL context of the rule that added the label.
    ///
    /// Required: No
    ///
    /// Type: LabelNameCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "LabelNameCondition")]
    pub label_name_condition: Option<LabelNameCondition>,
}

impl cfn_resources::CfnResource for Condition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.action_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.label_name_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The parts of the request that you want to keep out of the logs. This is used in the logging configuration RedactedFields specification.
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
    /// Redact the request body JSON.
    ///
    /// Required: No
    ///
    /// Type: JsonBody
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonBody")]
    pub json_body: Option<JsonBody>,

    ///
    /// Redact the indicated HTTP method. The method indicates the type of operation that the request is     asking the origin to perform.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Method")]
    pub method: Option<serde_json::Value>,

    ///
    /// Redact the query string. This is the part of a URL that appears after a ?     character, if any.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryString")]
    pub query_string: Option<serde_json::Value>,

    ///
    /// Redact a single header. Provide the name of the header to inspect, for example,       User-Agent or Referer. This setting isn't case     sensitive.
    ///
    /// Example JSON: "SingleHeader": { "Name": "haystack" }
    ///
    /// Required: No
    ///
    /// Type: SingleHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "SingleHeader")]
    pub single_header: Option<SingleHeader>,

    ///
    /// Redact the request URI path. This is the part of the web request that identifies a     resource, for example, /images/daily-ad.jpg.
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
        self.json_body
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.single_header
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A single logging filter, used in LoggingFilter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Filter {
    ///
    /// How to handle logs that satisfy the filter's conditions and requirement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DROP | KEEP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Behavior")]
    pub behavior: FilterBehaviorEnum,

    ///
    /// Match conditions for the filter.
    ///
    /// Required: Yes
    ///
    /// Type: List of Condition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Condition>,

    ///
    /// Logic to apply to the filtering conditions. You can specify that, in order to satisfy     the filter, a log must match all conditions or must match at least one condition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MEETS_ALL | MEETS_ANY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Requirement")]
    pub requirement: FilterRequirementEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FilterBehaviorEnum {
    /// DROP
    #[serde(rename = "DROP")]
    Drop,

    /// KEEP
    #[serde(rename = "KEEP")]
    Keep,
}

impl Default for FilterBehaviorEnum {
    fn default() -> Self {
        FilterBehaviorEnum::Drop
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FilterRequirementEnum {
    /// MEETS_ALL
    #[serde(rename = "MEETS_ALL")]
    Meetsall,

    /// MEETS_ANY
    #[serde(rename = "MEETS_ANY")]
    Meetsany,
}

impl Default for FilterRequirementEnum {
    fn default() -> Self {
        FilterRequirementEnum::Meetsall
    }
}

impl cfn_resources::CfnResource for Filter {
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
    /// Type: MatchPattern
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchPattern")]
    pub match_pattern: MatchPattern,

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

/// A single label name condition for a condition in a logging     filter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LabelNameCondition {
    ///
    /// The label name that a log record must contain in order to meet the condition. This must     be a fully qualified label name. Fully qualified labels have a prefix, optional namespaces, and label name. The prefix identifies the rule group or web ACL context of the rule that added the label.
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
    #[serde(rename = "LabelName")]
    pub label_name: String,
}

impl cfn_resources::CfnResource for LabelNameCondition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.label_name;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'label_name'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.label_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'label_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Filtering that specifies which web requests are kept in the logs and which are dropped,       defined for a web ACL's LoggingConfiguration.
///
/// You can filter on the rule action and on the web request labels that were applied by     matching rules during web ACL evaluation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingFilter {
    ///
    /// Default handling for logs that don't match any of the specified filtering conditions.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DROP | KEEP
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultBehavior")]
    pub default_behavior: LoggingFilterDefaultBehaviorEnum,

    ///
    /// The filters that you want to apply to the logs.
    ///
    /// Required: Yes
    ///
    /// Type: List of Filter
    ///
    /// Update requires: No interruption
    #[serde(rename = "Filters")]
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum LoggingFilterDefaultBehaviorEnum {
    /// DROP
    #[serde(rename = "DROP")]
    Drop,

    /// KEEP
    #[serde(rename = "KEEP")]
    Keep,
}

impl Default for LoggingFilterDefaultBehaviorEnum {
    fn default() -> Self {
        LoggingFilterDefaultBehaviorEnum::Drop
    }
}

impl cfn_resources::CfnResource for LoggingFilter {
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

/// The patterns to look for in the JSON body. AWS WAF inspects the results of these     pattern matches against the rule inspection criteria.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MatchPattern {
    ///
    /// Match all of the elements.
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
    /// Match only the specified include paths.
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

impl cfn_resources::CfnResource for MatchPattern {
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
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
