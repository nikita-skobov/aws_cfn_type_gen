
pub mod cfn_account_alias {

#[derive(serde::Serialize, Default)]
pub struct CfnAccountAlias {
    /// An account alias associated with a customer's account.
    #[serde(rename = "AccountAlias")]
    pub account_alias: String,

}



}

pub mod cfn_slack_channel_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnSlackChannelConfiguration {
    /// Whether to notify when a correspondence is added to a case.
    #[serde(rename = "NotifyOnAddCorrespondenceToCase")]
    pub notify_on_add_correspondence_to_case: Option<bool>,
    /// The channel ID in Slack, which identifies a channel within a workspace.
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
    /// The team ID in Slack, which uniquely identifies a workspace.
    #[serde(rename = "TeamId")]
    pub team_id: String,
    /// The channel name in Slack.
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
    /// Whether to notify when a case is created or reopened.
    #[serde(rename = "NotifyOnCreateOrReopenCase")]
    pub notify_on_create_or_reopen_case: Option<bool>,
    /// The severity level of a support case that a customer wants to get notified for.
    #[serde(rename = "NotifyOnCaseSeverity")]
    pub notify_on_case_severity: String,
    /// The Amazon Resource Name (ARN) of an IAM role that grants the AWS Support App access to perform operations for AWS services.
    #[serde(rename = "ChannelRoleArn")]
    pub channel_role_arn: String,
    /// Whether to notify when a case is resolved.
    #[serde(rename = "NotifyOnResolveCase")]
    pub notify_on_resolve_case: Option<bool>,

}



}

pub mod cfn_slack_workspace_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnSlackWorkspaceConfiguration {
    /// An identifier used to update an existing Slack workspace configuration in AWS CloudFormation.
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
    /// The team ID in Slack, which uniquely identifies a workspace.
    #[serde(rename = "TeamId")]
    pub team_id: String,

}



}
