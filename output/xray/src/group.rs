/// Use the AWS::XRay::Group resource to specify a group with a name and a filter expression.      Groups enable the collection of traces that match the filter expression, can be used to filter service graphs and traces, and to supply Amazon CloudWatch metrics.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnGroup {
    ///
    /// The filter expression defining the parameters to include traces.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<cfn_resources::StrVal>,

    ///
    /// The unique case-sensitive name of the group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: cfn_resources::StrVal,

    ///
    /// The structure containing configurations related to insights.
    ///
    /// The InsightsEnabled boolean can be set to true to enable insights for the           group or false to disable insights for the group.               The NotificationsEnabled boolean can be set to true to enable insights           notifications through Amazon EventBridge for the group.
    ///
    /// Required: No
    ///
    /// Type: InsightsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_group_arn: CfnGroupgrouparn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnGroupgrouparn;
impl CfnGroupgrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"GroupARN"#
    }
}

impl cfn_resources::CfnResource for CfnGroup {
    fn type_string(&self) -> &'static str {
        "AWS::XRay::Group"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.insights_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The structure containing configurations related to insights.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InsightsConfiguration {
    ///
    /// Set the InsightsEnabled value to true to enable insights or false to disable       insights.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_enabled: Option<bool>,

    ///
    /// Set the NotificationsEnabled value to true to enable insights notifications. Notifications can only be       enabled on a group with InsightsEnabled set to true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
}

impl cfn_resources::CfnResource for InsightsConfiguration {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
