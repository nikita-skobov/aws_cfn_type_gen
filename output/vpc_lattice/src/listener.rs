

/// Creates a listener for a service. Before you start using your Amazon VPC Lattice service, you must  add one or more listeners. A listener is a process that checks for connection requests to your  services. For more information, see Listeners in the  Amazon VPC Lattice User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnListener {


    /// 
    /// The action for the default rule. Each listener has a default rule. Each rule consists of a  priority, one or more actions, and one or more conditions. The default rule is the rule that's  used if no other rules match. Each rule must include exactly one of the following types of  actions: forward or fixed-response, and it must be the last action to  be performed.
    /// 
    /// Required: Yes
    ///
    /// Type: DefaultAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAction")]
    pub default_action: DefaultAction,


    /// 
    /// The name of the listener. A listener name must be unique within a service. The valid    characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last    character, or immediately after another hyphen.
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


    /// 
    /// The listener port. You can specify a value from 1 to 65535. For  HTTP, the default is 80. For HTTPS, the default is 443.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The listener protocol HTTP or HTTPS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: String,


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
    /// The tags for the listener.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnListener {
    fn type_string() -> &'static str {
        "AWS::VpcLattice::Listener"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The action for the default rule. Each listener has a default rule. Each rule consists of a  priority, one or more actions, and one or more conditions. The default rule is the rule that's  used if no other rules match. Each rule must include exactly one of the following types of  actions: forward or fixed-response, and it must be the last action to  be performed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DefaultAction {


    /// 
    /// Information about an action that returns a custom HTTP response.
    /// 
    /// Required: No
    ///
    /// Type: FixedResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "FixedResponse")]
    pub fixed_response: Option<FixedResponse>,


    /// 
    /// Describes a forward action. You can use forward actions to route requests to one or more  target groups.
    /// 
    /// Required: No
    ///
    /// Type: Forward
    ///
    /// Update requires: No interruption
    #[serde(rename = "Forward")]
    pub forward: Option<Forward>,

}




/// Information about an action that returns a custom HTTP response.
#[derive(Clone, Debug, Default, serde::Serialize)]
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




/// The forward action. Traffic that matches the rule is forwarded to the specified target  groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
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




/// Describes the weight of a target group.
#[derive(Clone, Debug, Default, serde::Serialize)]
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


