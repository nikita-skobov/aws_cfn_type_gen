
pub mod cfn_microsoft_teams_channel_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnMicrosoftTeamsChannelConfiguration {
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied as a default if this is not set.
    #[serde(rename = "GuardrailPolicies")]
    pub guardrail_policies: Option<Vec<String>>,
    /// Enables use of a user role requirement in your chat configuration
    #[serde(rename = "UserRoleRequired")]
    pub user_role_required: Option<bool>,
    /// Specifies the logging level for this configuration:ERROR,INFO or NONE. This property affects the log entries pushed to Amazon CloudWatch logs
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// The id of the Microsoft Teams channel
    #[serde(rename = "TeamsChannelId")]
    pub teams_channel_id: String,
    /// The id of the Microsoft Teams tenant
    #[serde(rename = "TeamsTenantId")]
    pub teams_tenant_id: String,
    /// The id of the Microsoft Teams team
    #[serde(rename = "TeamId")]
    pub team_id: String,
    /// ARNs of SNS topics which delivers notifications to AWS Chatbot, for example CloudWatch alarm notifications.
    #[serde(rename = "SnsTopicArns")]
    pub sns_topic_arns: Option<Vec<String>>,
    /// The name of the configuration
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,

}



}

pub mod cfn_slack_channel_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnSlackChannelConfiguration {
    /// The name of the configuration
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied as a default if this is not set.
    #[serde(rename = "GuardrailPolicies")]
    pub guardrail_policies: Option<Vec<String>>,
    /// The id of the Slack workspace
    #[serde(rename = "SlackWorkspaceId")]
    pub slack_workspace_id: String,
    /// Specifies the logging level for this configuration:ERROR,INFO or NONE. This property affects the log entries pushed to Amazon CloudWatch logs
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,
    /// Enables use of a user role requirement in your chat configuration
    #[serde(rename = "UserRoleRequired")]
    pub user_role_required: Option<bool>,
    /// The id of the Slack channel
    #[serde(rename = "SlackChannelId")]
    pub slack_channel_id: String,
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,
    /// ARNs of SNS topics which delivers notifications to AWS Chatbot, for example CloudWatch alarm notifications.
    #[serde(rename = "SnsTopicArns")]
    pub sns_topic_arns: Option<Vec<String>>,

}



}
