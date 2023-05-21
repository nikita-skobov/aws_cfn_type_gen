/// You can use the AWS::GuardDuty::Master resource in a member account to accept an invitation          from a administrator account. The          invitation to the member account must be sent prior to using the             AWS::GuardDuty::Master resource to accept the administrator          account's invitation. You can invite a member account by using the             InviteMembers operation of the API, or by creating an             AWS::GuardDuty::Member resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// The ID of the invitation that is sent to the account designated as a member          account. You can find the invitation ID by using the ListInvitation action of the             API.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InvitationId")]
    pub invitation_id: Option<String>,

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
}

impl cfn_resources::CfnResource for CfnMaster {
    fn type_string(&self) -> &'static str {
        "AWS::GuardDuty::Master"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.detector_id;

        if the_val.len() > 300 as _ {
            return Err(format!(
                "Max validation failed on field 'detector_id'. {} is greater than 300",
                the_val.len()
            ));
        }

        let the_val = &self.detector_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'detector_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
