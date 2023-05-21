

/// The AWS::Chatbot::SlackChannelConfiguration resource configures a Slack channel to allow users to use AWS Chatbot with AWS CloudFormation templates.
///
/// This resource requires some setup to be done in the AWS Chatbot console. To provide the required Slack workspace ID, you must perform the initial authorization flow with       Slack in the AWS Chatbot console, then copy and paste the workspace ID from the console.       For more details, see steps 1-4 in Setting Up AWS Chatbot with Slack in the AWS Chatbot User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSlackChannelConfiguration {


    /// 
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied as a default if this is not set.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GuardrailPolicies")]
    pub guardrail_policies: Option<Vec<String>>,


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
    pub slack_channel_id: String,


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
    pub slack_workspace_id: String,


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
    pub iam_role_arn: String,


    /// 
    /// The name of the configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,


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
    pub logging_level: Option<String>,


    /// 
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArns")]
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
    pub user_role_required: Option<bool>,

}

impl cfn_resources::CfnResource for CfnSlackChannelConfiguration {
    fn type_string() -> &'static str {
        "AWS::Chatbot::SlackChannelConfiguration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
