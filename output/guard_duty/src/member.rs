/// You can use the AWS::GuardDuty::Member resource to add an AWS account as a          member account to the current          administrator account. If the value of the Status property is not          provided or is set to Created, a member account is created but not          invited. If the value of the Status property is set to             Invited, a member account is created and invited. An             AWS::GuardDuty::Member resource must be created with the             Status property set to Invited before the             AWS::GuardDuty::Master resource can be created in a member account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMember {
    ///
    /// The ID of the detector associated with the service to add the member to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DetectorId")]
    pub detector_id: cfn_resources::StrVal,

    ///
    /// Specifies whether or not to disable email notification for the member account that          you invite.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,

    ///
    /// The email address associated with the member account.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Email")]
    pub email: cfn_resources::StrVal,

    ///
    /// The AWS account ID of the account to designate as a          member.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemberId")]
    pub member_id: cfn_resources::StrVal,

    ///
    /// The invitation message that you want to send to the accounts that you're inviting to    GuardDuty as members.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<cfn_resources::StrVal>,

    ///
    /// You can use the Status property to update the status of the          relationship between the member account and its administrator account. Valid          values are Created and Invited when using an             AWS::GuardDuty::Member resource. If the value for this property          is not provided or set to Created, a member account is created but          not invited. If the value of this property is set to Invited, a          member account is created and invited.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnMember {
    fn type_string(&self) -> &'static str {
        "AWS::GuardDuty::Member"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
