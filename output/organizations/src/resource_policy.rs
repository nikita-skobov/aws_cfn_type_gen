/// Creates or updates a resource-based delegation policy that can be used to delegate       policy management for AWS Organizations to specified member accounts to perform       policy actions that are by default available only to the management account.
///
/// For more information about delegated policy management, see Delegated         administrator for AWS Organizations in the AWS Organizations User Guide.
///
/// You can only call this operation from the organization's management account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourcePolicy {
    ///
    /// The policy text of the organization resource policy. You can specify the resource       policy content as a JSON object or a JSON string.
    ///
    /// ImportantWhen you specify the resource policy content as a JSON string, you can't perform         drift detection on the CloudFormation stack. For this reason, we recommend         specifying the resource policy content as a JSON object instead.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40000
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: serde_json::Value,

    ///
    /// A list of tags that you want to attach to the newly created resource policy. For each       tag in the list, you must specify both a tag key and a value. You can set the value to       an empty string, but you can't set it to null. For more information about       tagging, see Tagging AWS Organizations         resources in the AWS Organizations User       Guide.
    ///
    /// NoteIf any one of the tags is not valid or if you exceed the allowed number of tags         for the resource policy, then the entire request fails and the resource policy is         not created.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::Organizations::ResourcePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
