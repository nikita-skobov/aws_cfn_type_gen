

/// The AWS::AppStream::User resource creates a new user in the AppStream 2.0 user pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUser {


    /// 
    /// The email address of the user.
    /// 
    /// Users' email addresses are case-sensitive. During login, if they specify an email address that doesn't use the same capitalization as the email address specified when their user pool account was created, a "user does not exist" error message displays.
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


    /// 
    /// The authentication type for the user. You must specify USERPOOL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: API | AWS_AD | SAML | USERPOOL
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,


    /// 
    /// The last name, or surname, of the user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[A-Za-z0-9_\-\s]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,


    /// 
    /// The action to take for the welcome email that is sent to a user after the user is created in the user pool. If you specify SUPPRESS, no email is sent. If you specify RESEND, do not specify the first name or last name of the user. If the value is null, the email is sent.
    /// 
    /// NoteThe temporary password in the welcome email is valid for only 7 days. If users don’t set their passwords within 7 days, you must send them a new welcome email.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: RESEND | SUPPRESS
    ///
    /// Update requires: Replacement
    #[serde(rename = "MessageAction")]
    pub message_action: Option<String>,


    /// 
    /// The first name, or given name, of the user.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[A-Za-z0-9_\-\s]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,

}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string() -> &'static str {
        "AWS::AppStream::User"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
