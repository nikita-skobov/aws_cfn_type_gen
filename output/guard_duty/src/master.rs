

/// You can use the AWS::GuardDuty::Master resource in a member account to accept an invitation          from a administrator account. The          invitation to the member account must be sent prior to using the             AWS::GuardDuty::Master resource to accept the administrator          account's invitation. You can invite a member account by using the             InviteMembers operation of the API, or by creating an             AWS::GuardDuty::Member resource.
#[derive(Default, serde::Serialize)]
pub struct CfnMaster {


    /// 
    /// The unique ID of the detector of the GuardDuty member account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: Replacement
    #[serde(rename = "DetectorId")]
    pub detector_id: String,


    /// 
    /// The AWS account ID of the account designated as the administrator account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MasterId")]
    pub master_id: String,


    /// 
    /// The ID of the invitation that is sent to the account designated as a member          account. You can find the invitation ID by using the ListInvitation action of the             API.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InvitationId")]
    pub invitation_id: Option<String>,

}
