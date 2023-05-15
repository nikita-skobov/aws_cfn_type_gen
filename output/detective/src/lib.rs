
pub mod cfn_graph {

#[derive(serde::Serialize, Default)]
pub struct CfnGraph {
    /// Indicates whether to automatically enable new organization accounts as member accounts in the organization behavior graph.
    #[serde(rename = "AutoEnableMembers")]
    pub auto_enable_members: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_member_invitation {

#[derive(serde::Serialize, Default)]
pub struct CfnMemberInvitation {
    /// The root email address for the account to be invited, for validation. Updating this field has no effect.
    #[serde(rename = "MemberEmailAddress")]
    pub member_email_address: String,
    /// A message to be included in the email invitation sent to the invited account. Updating this field has no effect.
    #[serde(rename = "Message")]
    pub message: Option<String>,
    /// The AWS account ID to be invited to join the graph as a member
    #[serde(rename = "MemberId")]
    pub member_id: String,
    /// When set to true, invitation emails are not sent to the member accounts. Member accounts must still accept the invitation before they are added to the behavior graph. Updating this field has no effect.
    #[serde(rename = "DisableEmailNotification")]
    pub disable_email_notification: Option<bool>,
    /// The ARN of the graph to which the member account will be invited
    #[serde(rename = "GraphArn")]
    pub graph_arn: String,

}



}
