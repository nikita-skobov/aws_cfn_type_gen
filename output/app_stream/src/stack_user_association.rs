/// The AWS::AppStream::StackUserAssociation resource associates the specified users with the specified stacks for Amazon AppStream 2.0. Users in an AppStream 2.0 user pool cannot be assigned to stacks with fleets that are joined to an Active Directory domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub stack_name: cfn_resources::StrVal,

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
    pub user_name: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    fn type_string(&self) -> &'static str {
        "AWS::AppStream::StackUserAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.stack_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'stack_name'. {} is less than 1",
                    s.len()
                ));
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
