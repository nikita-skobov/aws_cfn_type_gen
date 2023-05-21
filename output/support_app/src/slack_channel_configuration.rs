

/// You can use the AWS::SupportApp::SlackChannelConfiguration resource to       specify your AWS account when you configure the AWS Support App. This resource includes the following information:
///
/// For more information, see the following topics in the AWS Support User Guide:
#[derive(Default, serde::Serialize)]
pub struct CfnSlackChannelConfiguration {


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


    /// 
    /// Whether to get notified when your support cases are created or reopened
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnCreateOrReopenCase")]
    pub notify_on_create_or_reopen_case: Option<bool>,


    /// 
    /// The channel name in Slack. This is the channel where you invite the AWS Support App.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,


    /// 
    /// Whether to get notified when a correspondence is added to your support cases.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnAddCorrespondenceToCase")]
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
    /// Whether to get notified when your support cases are resolved.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyOnResolveCase")]
    pub notify_on_resolve_case: Option<bool>,


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

}
