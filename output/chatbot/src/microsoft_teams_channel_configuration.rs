/// The AWS::Chatbot::MicrosoftTeamsChannelConfiguration resource configures a Microsoft Teams channel to allow users to use AWS Chatbot with AWS CloudFormation templates.
///
/// This resource requires some setup to be done in the AWS Chatbot console. To provide the required Microsoft Teams team and tenant IDs, you must perform the initial authorization flow with       Microsoft Teams in the AWS Chatbot console, then copy and paste the IDs from the console.       For more details, see steps 1-4 in Setting Up AWS Chatbot with Microsoft Teams in the AWS Chatbot Administrator Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMicrosoftTeamsChannelConfiguration {
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
    /// The ID of the Microsoft Team authorized with AWS Chatbot.
    ///
    /// To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.       For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TeamId")]
    pub team_id: cfn_resources::StrVal,

    ///
    /// The ID of the Microsoft Teams channel.
    ///
    /// To get the channel ID, open Microsoft Teams, right click on the channel name in the left pane, then choose Copy. An example of the channel ID syntax is: 19%3ab6ef35dc342d56ba5654e6fc6d25a071%40thread.tacv2.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TeamsChannelId")]
    pub teams_channel_id: cfn_resources::StrVal,

    ///
    /// The ID of the Microsoft Teams tenant.
    ///
    /// To get the tenant ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the tenant ID from the console.       For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TeamsTenantId")]
    pub teams_tenant_id: cfn_resources::StrVal,

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
    pub att_arn: CfnMicrosoftTeamsChannelConfigurationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMicrosoftTeamsChannelConfigurationarn;
impl CfnMicrosoftTeamsChannelConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnMicrosoftTeamsChannelConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::Chatbot::MicrosoftTeamsChannelConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
