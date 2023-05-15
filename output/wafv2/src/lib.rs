
pub mod cfn_ipset {

#[derive(serde::Serialize, Default)]
pub struct CfnIPSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<EntityName>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<EntityDescription>,
    /// Use CLOUDFRONT for CloudFront IPSet, use REGIONAL for Application Load Balancer and API Gateway.
    #[serde(rename = "Scope")]
    pub scope: Scope,
    /// Type of addresses in the IPSet, use IPV4 for IPV4 IP addresses, IPV6 for IPV6 address.
    #[serde(rename = "IPAddressVersion")]
    pub ipaddress_version: IPAddressVersion,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of IPAddresses.
    #[serde(rename = "Addresses")]
    pub addresses: Vec<IPAddress>,

}

pub type EntityName = String;pub type IPAddressVersion = String;pub type IPAddress = String;pub type EntityId = String;pub type ResourceArn = String;pub type Scope = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}
pub type EntityDescription = String;

}

pub mod cfn_logging_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnLoggingConfiguration {
    /// The parts of the request that you want to keep out of the logs. For example, if you redact the HEADER field, the HEADER field in the firehose will be xxx.
    #[serde(rename = "RedactedFields")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,
    /// Filtering that specifies which web requests are kept in the logs and which are dropped. You can filter on the rule action and on the web request labels that were applied by matching rules during web ACL evaluation.
    #[serde(rename = "LoggingFilter")]
    pub logging_filter: Option<()>,
    /// The Amazon Resource Names (ARNs) of the logging destinations that you want to associate with the web ACL.
    #[serde(rename = "LogDestinationConfigs")]
    pub log_destination_configs: Vec<String>,
    /// The Amazon Resource Name (ARN) of the web ACL that you want to associate with LogDestinationConfigs.
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "JsonBody")]
    pub json_body: Option<()>,
    #[serde(rename = "Method")]
    pub method: Option<()>,
    #[serde(rename = "QueryString")]
    pub query_string: Option<()>,
    #[serde(rename = "UriPath")]
    pub uri_path: Option<()>,
    #[serde(rename = "SingleHeader")]
    pub single_header: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "Behavior")]
    pub behavior: String,
    #[serde(rename = "Conditions")]
    pub conditions: Vec<Condition>,
    #[serde(rename = "Requirement")]
    pub requirement: String,

}

#[derive(serde::Serialize, Default)]
pub struct Condition {
    #[serde(rename = "LabelNameCondition")]
    pub label_name_condition: Option<()>,
    #[serde(rename = "ActionCondition")]
    pub action_condition: Option<()>,

}


}

pub mod cfn_regex_pattern_set {

#[derive(serde::Serialize, Default)]
pub struct CfnRegexPatternSet {
    /// Description of the entity.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Name of the RegexPatternSet.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RegularExpressionList")]
    pub regular_expression_list: Vec<String>,
    /// Use CLOUDFRONT for CloudFront RegexPatternSet, use REGIONAL for Application Load Balancer and API Gateway.
    #[serde(rename = "Scope")]
    pub scope: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_rule_group {

#[derive(serde::Serialize, Default)]
pub struct CfnRuleGroup {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<EntityDescription>,
    /// Collection of Rules.
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Collection of Consumed Labels.
    #[serde(rename = "ConsumedLabels")]
    pub consumed_labels: Option<Vec<LabelSummary>>,
    /// Visibility Metric of the RuleGroup.
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
    /// Custom response key and body map.
    #[serde(rename = "CustomResponseBodies")]
    pub custom_response_bodies: Option<CustomResponseBodies>,
    /// Collection of Available Labels.
    #[serde(rename = "AvailableLabels")]
    pub available_labels: Option<Vec<LabelSummary>>,
    /// Use CLOUDFRONT for CloudFront RuleGroup, use REGIONAL for Application Load Balancer and API Gateway.
    #[serde(rename = "Scope")]
    pub scope: Scope,
    /// No documentation provided by AWS
    #[serde(rename = "Capacity")]
    pub capacity: usize,

}


#[derive(serde::Serialize, Default)]
pub struct JsonBody {
    #[serde(rename = "InvalidFallbackBehavior")]
    pub invalid_fallback_behavior: Option<BodyParsingFallbackBehavior>,
    #[serde(rename = "MatchPattern")]
    pub match_pattern: JsonMatchPattern,
    #[serde(rename = "MatchScope")]
    pub match_scope: JsonMatchScope,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<OversizeHandling>,

}
pub type OversizeHandling = String;
#[derive(serde::Serialize, Default)]
pub struct CustomResponseBodies {

}

#[derive(serde::Serialize, Default)]
pub struct Cookies {
    #[serde(rename = "MatchPattern")]
    pub match_pattern: CookieMatchPattern,
    #[serde(rename = "MatchScope")]
    pub match_scope: MapMatchScope,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: OversizeHandling,

}

#[derive(serde::Serialize, Default)]
pub struct IPSetForwardedIPConfiguration {
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}
pub type SearchStringBase64 = String;pub type LabelMatchKey = String;
#[derive(serde::Serialize, Default)]
pub struct ChallengeConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}
pub type JsonMatchScope = String;pub type SensitivityLevel = String;
#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "Method")]
    pub method: Option<()>,
    #[serde(rename = "JsonBody")]
    pub json_body: Option<JsonBody>,
    #[serde(rename = "AllQueryArguments")]
    pub all_query_arguments: Option<()>,
    #[serde(rename = "Headers")]
    pub headers: Option<Headers>,
    #[serde(rename = "SingleHeader")]
    pub single_header: Option<()>,
    #[serde(rename = "SingleQueryArgument")]
    pub single_query_argument: Option<()>,
    #[serde(rename = "QueryString")]
    pub query_string: Option<()>,
    #[serde(rename = "Cookies")]
    pub cookies: Option<Cookies>,
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "UriPath")]
    pub uri_path: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct SqliMatchStatement {
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "SensitivityLevel")]
    pub sensitivity_level: Option<SensitivityLevel>,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}

#[derive(serde::Serialize, Default)]
pub struct ChallengeAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}
pub type TextTransformationType = String;pub type PositionalConstraint = String;
#[derive(serde::Serialize, Default)]
pub struct RegexPatternSetReferenceStatement {
    #[serde(rename = "Arn")]
    pub arn: ResourceArn,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,

}
pub type EntityName = String;
#[derive(serde::Serialize, Default)]
pub struct RuleAction {
    #[serde(rename = "Captcha")]
    pub captcha: Option<CaptchaAction>,
    #[serde(rename = "Challenge")]
    pub challenge: Option<ChallengeAction>,
    #[serde(rename = "Allow")]
    pub allow: Option<AllowAction>,
    #[serde(rename = "Count")]
    pub count: Option<CountAction>,
    #[serde(rename = "Block")]
    pub block: Option<BlockAction>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelMatchStatement {
    #[serde(rename = "Key")]
    pub key: LabelMatchKey,
    #[serde(rename = "Scope")]
    pub scope: LabelMatchScope,

}

#[derive(serde::Serialize, Default)]
pub struct ByteMatchStatement {
    #[serde(rename = "SearchStringBase64")]
    pub search_string_base64: Option<SearchStringBase64>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: PositionalConstraint,
    #[serde(rename = "SearchString")]
    pub search_string: Option<SearchString>,

}

#[derive(serde::Serialize, Default)]
pub struct OrStatement {
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}

#[derive(serde::Serialize, Default)]
pub struct AllowAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}
pub type LabelMatchScope = String;pub type ResponseContentType = String;pub type EntityDescription = String;pub type BodyParsingFallbackBehavior = String;
#[derive(serde::Serialize, Default)]
pub struct HeaderMatchPattern {
    #[serde(rename = "IncludedHeaders")]
    pub included_headers: Option<Vec<String>>,
    #[serde(rename = "All")]
    pub all: Option<()>,
    #[serde(rename = "ExcludedHeaders")]
    pub excluded_headers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleGroup {
    #[serde(rename = "Name")]
    pub name: Option<EntityName>,
    #[serde(rename = "Capacity")]
    pub capacity: Option<usize>,
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: Option<VisibilityConfig>,
    #[serde(rename = "Id")]
    pub id: Option<EntityId>,
    #[serde(rename = "Arn")]
    pub arn: Option<ResourceArn>,
    #[serde(rename = "Description")]
    pub description: Option<EntityDescription>,
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,

}
pub type ResponseContent = String;
#[derive(serde::Serialize, Default)]
pub struct VisibilityConfig {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    pub cloud_watch_metrics_enabled: bool,
    #[serde(rename = "SampledRequestsEnabled")]
    pub sampled_requests_enabled: bool,
    #[serde(rename = "MetricName")]
    pub metric_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "Action")]
    pub action: Option<RuleAction>,
    #[serde(rename = "Name")]
    pub name: EntityName,
    #[serde(rename = "Priority")]
    pub priority: RulePriority,
    #[serde(rename = "ChallengeConfig")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
    #[serde(rename = "Statement")]
    pub statement: Statement,
    #[serde(rename = "RuleLabels")]
    pub rule_labels: Option<Vec<Label>>,
    #[serde(rename = "CaptchaConfig")]
    pub captcha_config: Option<CaptchaConfig>,

}
pub type JsonPointerPath = String;pub type SearchString = String;pub type RulePriority = usize;
#[derive(serde::Serialize, Default)]
pub struct IPSetReferenceStatement {
    #[serde(rename = "IPSetForwardedIPConfig")]
    pub ipset_forwarded_ipconfig: Option<IPSetForwardedIPConfiguration>,
    #[serde(rename = "Arn")]
    pub arn: ResourceArn,

}

#[derive(serde::Serialize, Default)]
pub struct RateBasedStatement {
    #[serde(rename = "AggregateKeyType")]
    pub aggregate_key_type: String,
    #[serde(rename = "Limit")]
    pub limit: RateLimit,
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,
    #[serde(rename = "ScopeDownStatement")]
    pub scope_down_statement: Option<Box<Statement>>,

}

#[derive(serde::Serialize, Default)]
pub struct BlockAction {
    #[serde(rename = "CustomResponse")]
    pub custom_response: Option<CustomResponse>,

}

#[derive(serde::Serialize, Default)]
pub struct CountAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomHTTPHeader {
    #[serde(rename = "Name")]
    pub name: CustomHTTPHeaderName,
    #[serde(rename = "Value")]
    pub value: CustomHTTPHeaderValue,

}

#[derive(serde::Serialize, Default)]
pub struct CustomRequestHandling {
    #[serde(rename = "InsertHeaders")]
    pub insert_headers: Vec<CustomHTTPHeader>,

}
pub type MapMatchScope = String;
#[derive(serde::Serialize, Default)]
pub struct CaptchaAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct NotStatement {
    #[serde(rename = "Statement")]
    pub statement: Box<Statement>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomResponse {
    #[serde(rename = "ResponseCode")]
    pub response_code: ResponseStatusCode,
    #[serde(rename = "CustomResponseBodyKey")]
    pub custom_response_body_key: Option<String>,
    #[serde(rename = "ResponseHeaders")]
    pub response_headers: Option<Vec<CustomHTTPHeader>>,

}

#[derive(serde::Serialize, Default)]
pub struct SizeConstraintStatement {
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "Size")]
    pub size: f64,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,

}

#[derive(serde::Serialize, Default)]
pub struct RegexMatchStatement {
    #[serde(rename = "RegexString")]
    pub regex_string: String,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,

}
pub type TextTransformationPriority = usize;pub type RateLimit = usize;
#[derive(serde::Serialize, Default)]
pub struct TextTransformation {
    #[serde(rename = "Priority")]
    pub priority: TextTransformationPriority,
    #[serde(rename = "Type")]
    pub ty: TextTransformationType,

}

#[derive(serde::Serialize, Default)]
pub struct CaptchaConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}

#[derive(serde::Serialize, Default)]
pub struct ImmunityTimeProperty {
    #[serde(rename = "ImmunityTime")]
    pub immunity_time: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Body {
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<OversizeHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct Headers {
    #[serde(rename = "MatchPattern")]
    pub match_pattern: HeaderMatchPattern,
    #[serde(rename = "MatchScope")]
    pub match_scope: MapMatchScope,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: OversizeHandling,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AndStatement {
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}

#[derive(serde::Serialize, Default)]
pub struct CookieMatchPattern {
    #[serde(rename = "All")]
    pub all: Option<()>,
    #[serde(rename = "IncludedCookies")]
    pub included_cookies: Option<Vec<String>>,
    #[serde(rename = "ExcludedCookies")]
    pub excluded_cookies: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelSummary {
    #[serde(rename = "Name")]
    pub name: Option<LabelName>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomResponseBody {
    #[serde(rename = "ContentType")]
    pub content_type: ResponseContentType,
    #[serde(rename = "Content")]
    pub content: ResponseContent,

}
pub type LabelName = String;
#[derive(serde::Serialize, Default)]
pub struct ForwardedIPConfiguration {
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: String,
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}
pub type Scope = String;
#[derive(serde::Serialize, Default)]
pub struct Statement {
    #[serde(rename = "ByteMatchStatement")]
    pub byte_match_statement: Option<ByteMatchStatement>,
    #[serde(rename = "IPSetReferenceStatement")]
    pub ipset_reference_statement: Option<IPSetReferenceStatement>,
    #[serde(rename = "XssMatchStatement")]
    pub xss_match_statement: Option<XssMatchStatement>,
    #[serde(rename = "NotStatement")]
    pub not_statement: Option<Box<NotStatement>>,
    #[serde(rename = "RateBasedStatement")]
    pub rate_based_statement: Option<Box<RateBasedStatement>>,
    #[serde(rename = "OrStatement")]
    pub or_statement: Option<OrStatement>,
    #[serde(rename = "LabelMatchStatement")]
    pub label_match_statement: Option<LabelMatchStatement>,
    #[serde(rename = "RegexMatchStatement")]
    pub regex_match_statement: Option<RegexMatchStatement>,
    #[serde(rename = "AndStatement")]
    pub and_statement: Option<AndStatement>,
    #[serde(rename = "SizeConstraintStatement")]
    pub size_constraint_statement: Option<SizeConstraintStatement>,
    #[serde(rename = "RegexPatternSetReferenceStatement")]
    pub regex_pattern_set_reference_statement: Option<RegexPatternSetReferenceStatement>,
    #[serde(rename = "GeoMatchStatement")]
    pub geo_match_statement: Option<GeoMatchStatement>,
    #[serde(rename = "SqliMatchStatement")]
    pub sqli_match_statement: Option<SqliMatchStatement>,

}
pub type ResponseStatusCode = usize;pub type CustomHTTPHeaderValue = String;
#[derive(serde::Serialize, Default)]
pub struct XssMatchStatement {
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct GeoMatchStatement {
    #[serde(rename = "CountryCodes")]
    pub country_codes: Option<Vec<String>>,
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,

}
pub type EntityId = String;
#[derive(serde::Serialize, Default)]
pub struct Label {
    #[serde(rename = "Name")]
    pub name: LabelName,

}

#[derive(serde::Serialize, Default)]
pub struct JsonMatchPattern {
    #[serde(rename = "All")]
    pub all: Option<()>,
    #[serde(rename = "IncludedPaths")]
    pub included_paths: Option<Vec<JsonPointerPath>>,

}
pub type CustomHTTPHeaderName = String;

}

pub mod cfn_web_acl {

#[derive(serde::Serialize, Default)]
pub struct CfnWebACL {
    /// Collection of Rules.
    #[serde(rename = "Rules")]
    pub rules: Option<Vec<Rule>>,
    /// Use CLOUDFRONT for CloudFront WebACL, use REGIONAL for Application Load Balancer and API Gateway.
    #[serde(rename = "Scope")]
    pub scope: Scope,
    /// No documentation provided by AWS
    #[serde(rename = "ChallengeConfig")]
    pub challenge_config: Option<ChallengeConfig>,
    /// Visibility Metric of the WebACL.
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Default Action WebACL will take against ingress traffic when there is no matching Rule.
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,
    /// No documentation provided by AWS
    #[serde(rename = "CaptchaConfig")]
    pub captcha_config: Option<CaptchaConfig>,
    /// Custom response key and body map.
    #[serde(rename = "CustomResponseBodies")]
    pub custom_response_bodies: Option<CustomResponseBodies>,
    /// List of domains to accept in web request tokens, in addition to the domain of the protected resource.
    #[serde(rename = "TokenDomains")]
    pub token_domains: Option<TokenDomains>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<EntityDescription>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<EntityName>,

}


#[derive(serde::Serialize, Default)]
pub struct DefaultAction {
    #[serde(rename = "Block")]
    pub block: Option<BlockAction>,
    #[serde(rename = "Allow")]
    pub allow: Option<AllowAction>,

}

#[derive(serde::Serialize, Default)]
pub struct JsonBody {
    #[serde(rename = "MatchScope")]
    pub match_scope: JsonMatchScope,
    #[serde(rename = "MatchPattern")]
    pub match_pattern: JsonMatchPattern,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<OversizeHandling>,
    #[serde(rename = "InvalidFallbackBehavior")]
    pub invalid_fallback_behavior: Option<BodyParsingFallbackBehavior>,

}

#[derive(serde::Serialize, Default)]
pub struct LabelMatchStatement {
    #[serde(rename = "Scope")]
    pub scope: LabelMatchScope,
    #[serde(rename = "Key")]
    pub key: LabelMatchKey,

}

#[derive(serde::Serialize, Default)]
pub struct CountAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct AllowAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct Label {
    #[serde(rename = "Name")]
    pub name: LabelName,

}
pub type CustomHTTPHeaderValue = String;
#[derive(serde::Serialize, Default)]
pub struct ChallengeConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}

#[derive(serde::Serialize, Default)]
pub struct ResponseInspectionBodyContains {
    #[serde(rename = "SuccessStrings")]
    pub success_strings: Vec<String>,
    #[serde(rename = "FailureStrings")]
    pub failure_strings: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NotStatement {
    #[serde(rename = "Statement")]
    pub statement: Box<Statement>,

}

#[derive(serde::Serialize, Default)]
pub struct FieldToMatch {
    #[serde(rename = "AllQueryArguments")]
    pub all_query_arguments: Option<()>,
    #[serde(rename = "QueryString")]
    pub query_string: Option<()>,
    #[serde(rename = "Cookies")]
    pub cookies: Option<Cookies>,
    #[serde(rename = "Body")]
    pub body: Option<Body>,
    #[serde(rename = "JsonBody")]
    pub json_body: Option<JsonBody>,
    #[serde(rename = "UriPath")]
    pub uri_path: Option<()>,
    #[serde(rename = "SingleHeader")]
    pub single_header: Option<()>,
    #[serde(rename = "Headers")]
    pub headers: Option<Headers>,
    #[serde(rename = "Method")]
    pub method: Option<()>,
    #[serde(rename = "SingleQueryArgument")]
    pub single_query_argument: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct CookieMatchPattern {
    #[serde(rename = "IncludedCookies")]
    pub included_cookies: Option<Vec<String>>,
    #[serde(rename = "All")]
    pub all: Option<()>,
    #[serde(rename = "ExcludedCookies")]
    pub excluded_cookies: Option<Vec<String>>,

}
pub type OversizeHandling = String;
#[derive(serde::Serialize, Default)]
pub struct ForwardedIPConfiguration {
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: String,
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludedRule {
    #[serde(rename = "Name")]
    pub name: EntityName,

}

#[derive(serde::Serialize, Default)]
pub struct SingleQueryArgument {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HeaderMatchPattern {
    #[serde(rename = "IncludedHeaders")]
    pub included_headers: Option<Vec<String>>,
    #[serde(rename = "ExcludedHeaders")]
    pub excluded_headers: Option<Vec<String>>,
    #[serde(rename = "All")]
    pub all: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct OverrideAction {
    #[serde(rename = "Count")]
    pub count: Option<()>,
    #[serde(rename = "None")]
    pub none: Option<()>,

}
pub type RateLimit = usize;pub type JsonMatchScope = String;pub type JsonPointerPath = String;
#[derive(serde::Serialize, Default)]
pub struct GeoMatchStatement {
    #[serde(rename = "CountryCodes")]
    pub country_codes: Option<Vec<String>>,
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct OrStatement {
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}
pub type PositionalConstraint = String;pub type EntityName = String;pub type ResponseStatusCode = usize;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}
pub type TextTransformationPriority = usize;
#[derive(serde::Serialize, Default)]
pub struct CustomResponseBody {
    #[serde(rename = "ContentType")]
    pub content_type: ResponseContentType,
    #[serde(rename = "Content")]
    pub content: ResponseContent,

}

#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "OverrideAction")]
    pub override_action: Option<OverrideAction>,
    #[serde(rename = "ChallengeConfig")]
    pub challenge_config: Option<ChallengeConfig>,
    #[serde(rename = "RuleLabels")]
    pub rule_labels: Option<Vec<Label>>,
    #[serde(rename = "Priority")]
    pub priority: RulePriority,
    #[serde(rename = "Statement")]
    pub statement: Statement,
    #[serde(rename = "VisibilityConfig")]
    pub visibility_config: VisibilityConfig,
    #[serde(rename = "CaptchaConfig")]
    pub captcha_config: Option<CaptchaConfig>,
    #[serde(rename = "Action")]
    pub action: Option<RuleAction>,
    #[serde(rename = "Name")]
    pub name: EntityName,

}
pub type BodyParsingFallbackBehavior = String;
#[derive(serde::Serialize, Default)]
pub struct ImmunityTimeProperty {
    #[serde(rename = "ImmunityTime")]
    pub immunity_time: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ManagedRuleGroupConfig {
    #[serde(rename = "PasswordField")]
    pub password_field: Option<FieldIdentifier>,
    #[serde(rename = "AWSManagedRulesBotControlRuleSet")]
    pub awsmanaged_rules_bot_control_rule_set: Option<AWSManagedRulesBotControlRuleSet>,
    #[serde(rename = "UsernameField")]
    pub username_field: Option<FieldIdentifier>,
    #[serde(rename = "LoginPath")]
    pub login_path: Option<String>,
    #[serde(rename = "AWSManagedRulesATPRuleSet")]
    pub awsmanaged_rules_atprule_set: Option<AWSManagedRulesATPRuleSet>,
    #[serde(rename = "PayloadType")]
    pub payload_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Headers {
    #[serde(rename = "MatchPattern")]
    pub match_pattern: HeaderMatchPattern,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: OversizeHandling,
    #[serde(rename = "MatchScope")]
    pub match_scope: MapMatchScope,

}
pub type ResponseContent = String;pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct BlockAction {
    #[serde(rename = "CustomResponse")]
    pub custom_response: Option<CustomResponse>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomRequestHandling {
    #[serde(rename = "InsertHeaders")]
    pub insert_headers: Vec<CustomHTTPHeader>,

}

#[derive(serde::Serialize, Default)]
pub struct ChallengeAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomResponseBodies {

}

#[derive(serde::Serialize, Default)]
pub struct Body {
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: Option<OversizeHandling>,

}
pub type EntityId = String;
#[derive(serde::Serialize, Default)]
pub struct CaptchaAction {
    #[serde(rename = "CustomRequestHandling")]
    pub custom_request_handling: Option<CustomRequestHandling>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleAction {
    #[serde(rename = "Allow")]
    pub allow: Option<AllowAction>,
    #[serde(rename = "Block")]
    pub block: Option<BlockAction>,
    #[serde(rename = "Captcha")]
    pub captcha: Option<CaptchaAction>,
    #[serde(rename = "Challenge")]
    pub challenge: Option<ChallengeAction>,
    #[serde(rename = "Count")]
    pub count: Option<CountAction>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleGroupReferenceStatement {
    #[serde(rename = "ExcludedRules")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    #[serde(rename = "RuleActionOverrides")]
    pub rule_action_overrides: Option<Vec<RuleActionOverride>>,
    #[serde(rename = "Arn")]
    pub arn: ResourceArn,

}

#[derive(serde::Serialize, Default)]
pub struct UriPath {

}

#[derive(serde::Serialize, Default)]
pub struct IPSetReferenceStatement {
    #[serde(rename = "IPSetForwardedIPConfig")]
    pub ipset_forwarded_ipconfig: Option<IPSetForwardedIPConfiguration>,
    #[serde(rename = "Arn")]
    pub arn: ResourceArn,

}

#[derive(serde::Serialize, Default)]
pub struct IPSetForwardedIPConfiguration {
    #[serde(rename = "FallbackBehavior")]
    pub fallback_behavior: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "HeaderName")]
    pub header_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SingleHeader {
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VisibilityConfig {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    pub cloud_watch_metrics_enabled: bool,
    #[serde(rename = "SampledRequestsEnabled")]
    pub sampled_requests_enabled: bool,
    #[serde(rename = "MetricName")]
    pub metric_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TextTransformation {
    #[serde(rename = "Type")]
    pub ty: TextTransformationType,
    #[serde(rename = "Priority")]
    pub priority: TextTransformationPriority,

}

#[derive(serde::Serialize, Default)]
pub struct AWSManagedRulesATPRuleSet {
    #[serde(rename = "RequestInspection")]
    pub request_inspection: Option<RequestInspection>,
    #[serde(rename = "ResponseInspection")]
    pub response_inspection: Option<ResponseInspection>,
    #[serde(rename = "LoginPath")]
    pub login_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct RuleActionOverride {
    #[serde(rename = "ActionToUse")]
    pub action_to_use: RuleAction,
    #[serde(rename = "Name")]
    pub name: EntityName,

}

#[derive(serde::Serialize, Default)]
pub struct Rules {

}
pub type TextTransformationType = String;
#[derive(serde::Serialize, Default)]
pub struct ResponseInspectionJson {
    #[serde(rename = "SuccessValues")]
    pub success_values: Vec<String>,
    #[serde(rename = "Identifier")]
    pub identifier: String,
    #[serde(rename = "FailureValues")]
    pub failure_values: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomHTTPHeader {
    #[serde(rename = "Value")]
    pub value: CustomHTTPHeaderValue,
    #[serde(rename = "Name")]
    pub name: CustomHTTPHeaderName,

}

#[derive(serde::Serialize, Default)]
pub struct RequestInspection {
    #[serde(rename = "PayloadType")]
    pub payload_type: String,
    #[serde(rename = "UsernameField")]
    pub username_field: FieldIdentifier,
    #[serde(rename = "PasswordField")]
    pub password_field: FieldIdentifier,

}
pub type LabelName = String;pub type SensitivityLevel = String;pub type RulePriority = usize;pub type SearchString = String;pub type SearchStringBase64 = String;
#[derive(serde::Serialize, Default)]
pub struct FieldIdentifier {
    #[serde(rename = "Identifier")]
    pub identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResponseInspectionHeader {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SuccessValues")]
    pub success_values: Vec<String>,
    #[serde(rename = "FailureValues")]
    pub failure_values: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RegexMatchStatement {
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "RegexString")]
    pub regex_string: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryString {

}
pub type LabelMatchKey = String;
#[derive(serde::Serialize, Default)]
pub struct ResponseInspection {
    #[serde(rename = "StatusCode")]
    pub status_code: Option<ResponseInspectionStatusCode>,
    #[serde(rename = "Json")]
    pub json: Option<ResponseInspectionJson>,
    #[serde(rename = "Header")]
    pub header: Option<ResponseInspectionHeader>,
    #[serde(rename = "BodyContains")]
    pub body_contains: Option<ResponseInspectionBodyContains>,

}

#[derive(serde::Serialize, Default)]
pub struct ExcludedRules {

}
pub type MapMatchScope = String;
#[derive(serde::Serialize, Default)]
pub struct ByteMatchStatement {
    #[serde(rename = "SearchStringBase64")]
    pub search_string_base64: Option<SearchStringBase64>,
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "SearchString")]
    pub search_string: Option<SearchString>,
    #[serde(rename = "PositionalConstraint")]
    pub positional_constraint: PositionalConstraint,

}

#[derive(serde::Serialize, Default)]
pub struct CustomResponse {
    #[serde(rename = "CustomResponseBodyKey")]
    pub custom_response_body_key: Option<String>,
    #[serde(rename = "ResponseHeaders")]
    pub response_headers: Option<Vec<CustomHTTPHeader>>,
    #[serde(rename = "ResponseCode")]
    pub response_code: ResponseStatusCode,

}

#[derive(serde::Serialize, Default)]
pub struct SizeConstraintStatement {
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "Size")]
    pub size: f64,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,

}

#[derive(serde::Serialize, Default)]
pub struct SqliMatchStatement {
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "SensitivityLevel")]
    pub sensitivity_level: Option<SensitivityLevel>,

}
pub type CustomHTTPHeaderName = String;
#[derive(serde::Serialize, Default)]
pub struct Cookies {
    #[serde(rename = "MatchPattern")]
    pub match_pattern: CookieMatchPattern,
    #[serde(rename = "MatchScope")]
    pub match_scope: MapMatchScope,
    #[serde(rename = "OversizeHandling")]
    pub oversize_handling: OversizeHandling,

}

#[derive(serde::Serialize, Default)]
pub struct CaptchaConfig {
    #[serde(rename = "ImmunityTimeProperty")]
    pub immunity_time_property: Option<ImmunityTimeProperty>,

}

#[derive(serde::Serialize, Default)]
pub struct AndStatement {
    #[serde(rename = "Statements")]
    pub statements: Vec<Statement>,

}

#[derive(serde::Serialize, Default)]
pub struct XssMatchStatement {
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,

}
pub type LabelMatchScope = String;
#[derive(serde::Serialize, Default)]
pub struct RateBasedStatement {
    #[serde(rename = "Limit")]
    pub limit: RateLimit,
    #[serde(rename = "ForwardedIPConfig")]
    pub forwarded_ipconfig: Option<ForwardedIPConfiguration>,
    #[serde(rename = "AggregateKeyType")]
    pub aggregate_key_type: String,
    #[serde(rename = "ScopeDownStatement")]
    pub scope_down_statement: Option<Box<Statement>>,

}

#[derive(serde::Serialize, Default)]
pub struct Statement {
    #[serde(rename = "ByteMatchStatement")]
    pub byte_match_statement: Option<ByteMatchStatement>,
    #[serde(rename = "LabelMatchStatement")]
    pub label_match_statement: Option<LabelMatchStatement>,
    #[serde(rename = "IPSetReferenceStatement")]
    pub ipset_reference_statement: Option<IPSetReferenceStatement>,
    #[serde(rename = "SizeConstraintStatement")]
    pub size_constraint_statement: Option<SizeConstraintStatement>,
    #[serde(rename = "GeoMatchStatement")]
    pub geo_match_statement: Option<GeoMatchStatement>,
    #[serde(rename = "SqliMatchStatement")]
    pub sqli_match_statement: Option<SqliMatchStatement>,
    #[serde(rename = "RegexPatternSetReferenceStatement")]
    pub regex_pattern_set_reference_statement: Option<RegexPatternSetReferenceStatement>,
    #[serde(rename = "ManagedRuleGroupStatement")]
    pub managed_rule_group_statement: Option<Box<ManagedRuleGroupStatement>>,
    #[serde(rename = "RuleGroupReferenceStatement")]
    pub rule_group_reference_statement: Option<RuleGroupReferenceStatement>,
    #[serde(rename = "AndStatement")]
    pub and_statement: Option<AndStatement>,
    #[serde(rename = "RateBasedStatement")]
    pub rate_based_statement: Option<Box<RateBasedStatement>>,
    #[serde(rename = "NotStatement")]
    pub not_statement: Option<Box<NotStatement>>,
    #[serde(rename = "XssMatchStatement")]
    pub xss_match_statement: Option<XssMatchStatement>,
    #[serde(rename = "OrStatement")]
    pub or_statement: Option<OrStatement>,
    #[serde(rename = "RegexMatchStatement")]
    pub regex_match_statement: Option<RegexMatchStatement>,

}
pub type Scope = String;
#[derive(serde::Serialize, Default)]
pub struct ManagedRuleGroupStatement {
    #[serde(rename = "ManagedRuleGroupConfigs")]
    pub managed_rule_group_configs: Option<Vec<ManagedRuleGroupConfig>>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Name")]
    pub name: EntityName,
    #[serde(rename = "ExcludedRules")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    #[serde(rename = "ScopeDownStatement")]
    pub scope_down_statement: Option<Box<Statement>>,
    #[serde(rename = "RuleActionOverrides")]
    pub rule_action_overrides: Option<Vec<RuleActionOverride>>,
    #[serde(rename = "VendorName")]
    pub vendor_name: String,

}
pub type EntityDescription = String;pub type ResponseContentType = String;
#[derive(serde::Serialize, Default)]
pub struct ResponseInspectionStatusCode {
    #[serde(rename = "SuccessCodes")]
    pub success_codes: Vec<usize>,
    #[serde(rename = "FailureCodes")]
    pub failure_codes: Vec<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AWSManagedRulesBotControlRuleSet {
    #[serde(rename = "InspectionLevel")]
    pub inspection_level: String,

}
pub type TokenDomains = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct JsonMatchPattern {
    #[serde(rename = "All")]
    pub all: Option<()>,
    #[serde(rename = "IncludedPaths")]
    pub included_paths: Option<Vec<JsonPointerPath>>,

}

#[derive(serde::Serialize, Default)]
pub struct RegexPatternSetReferenceStatement {
    #[serde(rename = "TextTransformations")]
    pub text_transformations: Vec<TextTransformation>,
    #[serde(rename = "FieldToMatch")]
    pub field_to_match: FieldToMatch,
    #[serde(rename = "Arn")]
    pub arn: ResourceArn,

}


}

pub mod cfn_web_aclassociation {

#[derive(serde::Serialize, Default)]
pub struct CfnWebACLAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "WebACLArn")]
    pub web_aclarn: ResourceArn,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArn,

}

pub type ResourceArn = String;

}
