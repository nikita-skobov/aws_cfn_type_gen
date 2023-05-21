

/// The AWS::AppStream::StackUserAssociation resource associates the specified users with the specified stacks for Amazon AppStream 2.0. Users in an AppStream 2.0 user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStackUserAssociation {


    /// 
    /// The authentication type for the user who is associated with the stack. You must specify USERPOOL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: API | AWS_AD | SAML | USERPOOL
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: StackUserAssociationAuthenticationTypeEnum,


    /// 
    /// Specifies whether a welcome email is sent to a user after the user is created in the user pool.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SendEmailNotification")]
    pub send_email_notification: Option<bool>,


    /// 
    /// The name of the stack that is associated with the user.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "StackName")]
    pub stack_name: String,


    /// 
    /// The email address of the user who is associated with the stack.
    /// 
    /// NoteUsers' email addresses are case-sensitive.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\p{L}\p{M}\p{S}\p{N}\p{P}]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StackUserAssociationAuthenticationTypeEnum {

    /// API
    #[serde(rename = "API")]
    Api,

    /// AWS_AD
    #[serde(rename = "AWS_AD")]
    Awsad,

    /// SAML
    #[serde(rename = "SAML")]
    Saml,

    /// USERPOOL
    #[serde(rename = "USERPOOL")]
    Userpool,

}

impl Default for StackUserAssociationAuthenticationTypeEnum {
    fn default() -> Self {
        StackUserAssociationAuthenticationTypeEnum::Api
    }
}


impl cfn_resources::CfnResource for CfnStackUserAssociation {
    fn type_string() -> &'static str {
        "AWS::AppStream::StackUserAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
