/// Creates or updates a Contributor Insights rule. Rules evaluate log events in a CloudWatch Logs log group, enabling you to find contributor data     for the log events in that log group. For more information, see       Using Contributor Insights to Analyze High-Cardinality Data in the Amazon CloudWatch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnInsightRule {
    /// The definition of the rule, as a JSON object.     For details about the syntax, see       Contributor Insights Rule Syntax in the Amazon CloudWatch User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleBody")]
    pub rule_body: cfn_resources::StrVal,

    ///
    /// The name of the rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleName")]
    pub rule_name: cfn_resources::StrVal,

    ///
    /// The current state of the rule. Valid values are ENABLED and DISABLED.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleState")]
    pub rule_state: cfn_resources::StrVal,

    ///
    /// A list of key-value pairs to associate with the Contributor Insights rule. You can       associate as many as 50 tags with a rule.
    ///
    /// Tags can help you organize and categorize your resources. For more information,       see         Tagging Your Amazon CloudWatch Resources.
    ///
    /// To be able to associate tags with a rule, you must have the cloudwatch:TagResource permission in addition to the cloudwatch:PutInsightRule permission.
    ///
    /// Required: No
    ///
    /// Type: Tags
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(skip_serializing)]
    pub att_arn: CfnInsightRulearn,

    #[serde(skip_serializing)]
    pub att_rule_name: CfnInsightRulerulename,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInsightRulearn;
impl CfnInsightRulearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInsightRulerulename;
impl CfnInsightRulerulename {
    pub fn att_name(&self) -> &'static str {
        r#"RuleName"#
    }
}

impl cfn_resources::CfnResource for CfnInsightRule {
    fn type_string(&self) -> &'static str {
        "AWS::CloudWatch::InsightRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.tags.as_ref().map_or(Ok(()), |val| val.validate())?;

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

/// A list of key-value pairs to associate with the Contributor Insights rule. You can     associate as many as 50 tags with a rule.
///
/// Tags can help you organize and categorize your resources. For more information,       see         Tagging Your Amazon CloudWatch Resources.
///
/// To be able to associate tags with a rule, you must have the cloudwatch:TagResource permission in addition to the cloudwatch:PutInsightRule permission.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tags {}

impl cfn_resources::CfnResource for Tags {
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
