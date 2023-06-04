/// The AWS::Chatbot::SlackChannelConfiguration resource configures a Slack channel to allow users to use AWS Chatbot with AWS CloudFormation templates.
///
/// This resource requires some setup to be done in the AWS Chatbot console. To provide the required Slack workspace ID, you must perform the initial authorization flow with       Slack in the AWS Chatbot console, then copy and paste the workspace ID from the console.       For more details, see steps 1-4 in Setting Up AWS Chatbot with Slack in the AWS Chatbot User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSlackChannelConfiguration {
    ///
    /// The name of the configuration.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: cfn_resources::StrVal,

    ///
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied as a default if this is not set.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GuardrailPolicies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub guardrail_policies: Option<Vec<String>>,

    ///
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot.
    ///
    /// This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: cfn_resources::StrVal,

    ///
    /// Specifies the logging level for this configuration. This property affects the log entries pushed to Amazon CloudWatch Logs.
    ///
    /// Logging levels include ERROR, INFO, or NONE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingLevel")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub logging_level: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the Slack channel.
    ///
    /// To get the ID, open Slack, right click on the channel name in the left pane, then choose Copy Link. The channel ID is the 9-character string at the end of the URL. For example, ABCBBLZZZ.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SlackChannelId")]
    pub slack_channel_id: cfn_resources::StrVal,

    ///
    /// The ID of the Slack workspace authorized with AWS Chatbot.
    ///
    /// To get the workspace ID, you must perform the initial authorization flow with Slack in the AWS Chatbot console. Then you can copy and paste the workspace ID from the console.       For more details, see steps 1-4 in Setting Up AWS Chatbot with Slack in the AWS Chatbot User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SlackWorkspaceId")]
    pub slack_workspace_id: cfn_resources::StrVal,

    ///
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArns")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub sns_topic_arns: Option<Vec<String>>,

    ///
    /// Enables use of a user role requirement in your chat configuration.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserRoleRequired")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub user_role_required: Option<bool>,

    #[serde(skip_serializing)]
    pub att_arn: CfnSlackChannelConfigurationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSlackChannelConfigurationarn;
impl CfnSlackChannelConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnSlackChannelConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::Chatbot::SlackChannelConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
