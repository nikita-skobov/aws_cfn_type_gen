

/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html
#[derive(Default, serde::Serialize)]
pub struct CfnProactiveEngagement {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-proactiveengagementstatus
    #[serde(rename = "ProactiveEngagementStatus")]
    pub proactive_engagement_status: String,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html#cfn-shield-proactiveengagement-emergencycontactlist
    #[serde(rename = "EmergencyContactList")]
    pub emergency_contact_list: Vec<EmergencyContact>,

}


/// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html
#[derive(Default, serde::Serialize)]
pub struct EmergencyContact {


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-phonenumber
    #[serde(rename = "PhoneNumber")]
    pub phone_number: Option<String>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-contactnotes
    #[serde(rename = "ContactNotes")]
    pub contact_notes: Option<String>,


    /// http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html#cfn-shield-proactiveengagement-emergencycontact-emailaddress
    #[serde(rename = "EmailAddress")]
    pub email_address: String,

}
