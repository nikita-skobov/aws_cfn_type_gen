

/// The AWS::Detective::MemberInvitation resource is an Amazon Detective       resource type that creates an invitation to join a Detective behavior graph. The       administrator account can choose whether to send an email notification of the invitation       to the root user email address of the AWS account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMemberInvitation {


    /// 
    /// Whether to send an invitation email to the member account. If set to true, the member account does not receive an invitation email.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableEmailNotification")]
    pub disable_email_notification: Option<bool>,


    /// 
    /// The ARN of the behavior graph to invite the account to contribute data to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GraphArn")]
    pub graph_arn: String,


    /// 
    /// The root user email address of the invited account. If the email address provided is       not the root user email address for the provided account, the invitation creation       fails.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemberEmailAddress")]
    pub member_email_address: String,


    /// 
    /// The AWS account identifier of the invited account
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemberId")]
    pub member_id: String,


    /// 
    /// Customized text to include in the invitation email message.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    pub message: Option<String>,

}



impl cfn_resources::CfnResource for CfnMemberInvitation {
    fn type_string() -> &'static str {
        "AWS::Detective::MemberInvitation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
