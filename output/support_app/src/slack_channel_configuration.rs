/// You can use the AWS::SupportApp::SlackChannelConfiguration resource to       specify your AWS account when you configure the AWS Support App. This resource includes the following information:
///
/// For more information, see the following topics in the AWS Support User Guide:
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSlackChannelConfiguration {
    ///
    /// The channel ID in Slack. This ID identifies a channel within a Slack workspace.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ChannelId")]
    pub channel_id: String,

    ///
    /// The channel name in Slack. This is the channel where you invite the AWS Support App.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role for this Slack channel       configuration. The AWS Support App uses this role to perform AWS Support and Service Quotas actions on your behalf.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelRoleArn")]
    pub channel_role_arn: String,

    ///
    /// Whether to get notified when a correspondence is added to your support cases.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnAddCorrespondenceToCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_add_correspondence_to_case: Option<bool>,

    ///
    /// The case severity for your support cases that you want to receive notifications. You       can specify none, all, or high.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnCaseSeverity")]
    pub notify_on_case_severity: String,

    ///
    /// Whether to get notified when your support cases are created or reopened
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnCreateOrReopenCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_create_or_reopen_case: Option<bool>,

    ///
    /// Whether to get notified when your support cases are resolved.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnResolveCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_resolve_case: Option<bool>,

    ///
    /// The team ID in Slack. This ID uniquely identifies a Slack workspace.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TeamId")]
    pub team_id: String,
}

impl cfn_resources::CfnResource for CfnSlackChannelConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::SupportApp::SlackChannelConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
