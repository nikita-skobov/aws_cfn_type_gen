/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProactiveEngagement {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-emergencycontactlist
    #[serde(rename = "EmergencyContactList")]
    pub emergency_contact_list: Vec<EmergencyContact>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-proactiveengagementstatus
    #[serde(rename = "ProactiveEngagementStatus")]
    pub proactive_engagement_status: String,
}

impl cfn_resources::CfnResource for CfnProactiveEngagement {
    fn type_string(&self) -> &'static str {
        "AWS::Shield::ProactiveEngagement"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EmergencyContact {
    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-contactnotes
    #[serde(rename = "ContactNotes")]
    pub contact_notes: Option<String>,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-emailaddress
    #[serde(rename = "EmailAddress")]
    pub email_address: String,

    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-phonenumber
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Option<String>,
}

impl cfn_resources::CfnResource for EmergencyContact {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
