

/// Creates a listener rule. Each listener has a default rule for checking connection requests,  but you can define additional rules. Each rule consists of a priority, one or more actions, and  one or more conditions. For more information, see Listener rules in the   Amazon VPC Lattice User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnRule {


    /// 
    /// The ID or Amazon Resource Name (ARN) of the listener.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListenerIdentifier")]
    pub listener_identifier: Option<String>,


    /// 
    /// Describes the action for a rule. Each rule must include exactly one of the following types  of actions: forward or fixed-response, and it must be the last action  to be performed.
    /// 
    /// Required: Yes
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: Action,


    /// 
    /// The tags for the rule.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The priority assigned to the rule. Each rule for a specific listener must have a unique  priority. The lower the priority number the higher the priority.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


    /// 
    /// The ID or Amazon Resource Name (ARN) of the service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceIdentifier")]
    pub service_identifier: Option<String>,


    /// 
    /// The rule match.
    /// 
    /// Required: Yes
    ///
    /// Type: Match
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Match,


    /// 
    /// The name of the rule. The name must be unique within the listener. The valid characters    are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or    immediately after another hyphen.
    /// 
    /// If you don't specify a name, CloudFormation generates one. However, if    you specify a name, and later want to replace the resource, you must specify a new    name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The forward action. Traffic that matches the rule is forwarded to the specified target  groups.
#[derive(Default, serde::Serialize)]
pub struct Forward {


    /// 
    /// The target groups. Traffic matching the rule is forwarded to the specified target groups.  With forward actions, you can assign a weight that controls the prioritization and selection of  each target group. This means that requests are distributed to individual target groups based on  their weights. For example, if two target groups have the same weight, each target group receives  half of the traffic.
    /// 
    /// The default value is 1. This means that if only one target group is provided, there is no  need to set the weight; 100% of traffic will go to that target group.
    /// 
    /// Required: Yes
    ///
    /// Type: List of WeightedTargetGroup
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroups")]
    pub target_groups: Vec<WeightedTargetGroup>,

}


/// Describes a header match type. Only one can be provided.
#[derive(Default, serde::Serialize)]
pub struct HeaderMatchType {


    /// 
    /// Specifies a prefix type match. Matches the value with the prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// Specifies an exact type match.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,


    /// 
    /// Specifies a contains type match.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Contains")]
    pub contains: Option<String>,

}


/// Describes a rule match.
#[derive(Default, serde::Serialize)]
pub struct Match {


    /// 
    /// The HTTP criteria that a rule must match.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpMatch")]
    pub http_match: HttpMatch,

}


/// Describes the conditions that can be applied when matching a path for incoming  requests.
#[derive(Default, serde::Serialize)]
pub struct PathMatch {


    /// 
    /// Indicates whether the match is case sensitive. Defaults to false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,


    /// 
    /// The type of path match.
    /// 
    /// Required: Yes
    ///
    /// Type: PathMatchType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: PathMatchType,

}


/// Information about an action that returns a custom HTTP response.
#[derive(Default, serde::Serialize)]
pub struct FixedResponse {


    /// 
    /// The HTTP response code.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: i64,

}


/// Describes a path match type. Each rule can include only one of the following types of  paths.
#[derive(Default, serde::Serialize)]
pub struct PathMatchType {


    /// 
    /// A prefix match of the path.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// An exact match of the path.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


/// Describes criteria that can be applied to incoming requests.
#[derive(Default, serde::Serialize)]
pub struct HttpMatch {


    /// 
    /// The HTTP method type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Method")]
    pub method: Option<String>,


    /// 
    /// The header matches. Matches incoming requests with rule based on request header value before  applying rule action.
    /// 
    /// Required: No
    ///
    /// Type: List of HeaderMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeaderMatches")]
    pub header_matches: Option<Vec<HeaderMatch>>,


    /// 
    /// The path match.
    /// 
    /// Required: No
    ///
    /// Type: PathMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathMatch")]
    pub path_match: Option<PathMatch>,

}


/// Describes the weight of a target group.
#[derive(Default, serde::Serialize)]
pub struct WeightedTargetGroup {


    /// 
    /// The ID of the target group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupIdentifier")]
    pub target_group_identifier: String,


    /// 
    /// Only required if you specify multiple target groups for a forward action. The "weight"  determines how requests are distributed to the target group. For example, if you specify two  target groups, each with a weight of 10, each target group receives half the requests. If you  specify two target groups, one with a weight of 10 and the other with a weight of 20, the target  group with a weight of 20 receives twice as many requests as the other target group. If there's  only one target group specified, then the default value is 100.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    pub weight: Option<i64>,

}


/// Describes the constraints for a header match. Matches incoming requests with rule based on  request header value before applying rule action.
#[derive(Default, serde::Serialize)]
pub struct HeaderMatch {


    /// 
    /// Indicates whether the match is case sensitive. Defaults to false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,


    /// 
    /// The header match type.
    /// 
    /// Required: Yes
    ///
    /// Type: HeaderMatchType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: HeaderMatchType,


    /// 
    /// The name of the header.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// Describes the action for a rule. Each rule must include exactly one of the following types  of actions: forward or fixed-response, and it must be the last action  to be performed.
#[derive(Default, serde::Serialize)]
pub struct Action {


    /// 
    /// The forward action. Traffic that matches the rule is forwarded to the specified target  groups.
    /// 
    /// Required: No
    ///
    /// Type: Forward
    ///
    /// Update requires: No interruption
    #[serde(rename = "Forward")]
    pub forward: Option<Forward>,


    /// 
    /// Describes the rule action that returns a custom HTTP response.
    /// 
    /// Required: No
    ///
    /// Type: FixedResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedResponse")]
    pub fixed_response: Option<FixedResponse>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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
