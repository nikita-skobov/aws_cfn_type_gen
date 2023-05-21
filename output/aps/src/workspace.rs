/// The AWS::APS::Workspace type specifies an Amazon Managed Service for Prometheus         (Amazon Managed Service for Prometheus) workspace. A workspace is a logical and       isolated Prometheus server dedicated to Prometheus resources such as metrics. You can       have one or more workspaces in each Region in your account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnWorkspace {
    ///
    /// The alert manager definition for the workspace, as a string. For more information, see         Alert manager and templating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlertManagerDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_manager_definition: Option<cfn_resources::StrVal>,

    ///
    /// An alias that you assign to this workspace to help you identify it. It does not need       to be unique.
    ///
    /// The alias can be as many as 100 characters and can include any type of characters.         Amazon Managed Service for Prometheus automatically strips any blank spaces from the beginning and end       of the alias that you specify.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<cfn_resources::StrVal>,

    ///
    /// The LoggingConfiguration attribute is used to set the logging configuration for the workspace.
    ///
    /// Required: No
    ///
    /// Type: LoggingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,

    ///
    /// A list of tag keys and values to associate with the workspace.
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

impl cfn_resources::CfnResource for CfnWorkspace {
    fn type_string(&self) -> &'static str {
        "AWS::APS::Workspace"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.logging_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The LoggingConfiguration attribute sets the logging configuration for the workspace.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfiguration {
    ///
    /// The Amazon Resource Name (ARN) of the CloudWatch log group the logs are emitted to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LoggingConfiguration {
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
