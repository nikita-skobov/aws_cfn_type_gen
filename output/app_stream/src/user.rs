/// The AWS::AppStream::User resource creates a new user in the AppStream 2.0 user pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUser {
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
    pub authentication_type: UserAuthenticationTypeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action: Option<UserMessageActionEnum>,

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
    pub user_name: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserAuthenticationTypeEnum {
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

impl Default for UserAuthenticationTypeEnum {
    fn default() -> Self {
        UserAuthenticationTypeEnum::Api
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum UserMessageActionEnum {
    /// RESEND
    #[serde(rename = "RESEND")]
    Resend,

    /// SUPPRESS
    #[serde(rename = "SUPPRESS")]
    Suppress,
}

impl Default for UserMessageActionEnum {
    fn default() -> Self {
        UserMessageActionEnum::Resend
    }
}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::User"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.first_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'first_name'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.last_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'last_name'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.user_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
