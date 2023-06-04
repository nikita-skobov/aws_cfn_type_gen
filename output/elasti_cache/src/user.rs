/// For Redis engine version 6.0 onwards: Creates a Redis user. For more information, see Using Role Based Access Control (RBAC).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUser {
    ///
    /// Access permissions string used for this user.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the authentication mode to use. Below is an example of the possible JSON values:
    ///
    /// { Type: <iam | no-password-required | password> Passwords: ["*****", "******"] // If Type is password. }
    ///
    /// Required: No
    ///
    /// Type: AuthenticationMode
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthenticationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,

    ///
    /// The current supported value is redis.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-zA-Z]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Engine")]
    pub engine: cfn_resources::StrVal,

    ///
    /// Indicates a password is not required for this user.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoPasswordRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_password_required: Option<bool>,

    ///
    /// Passwords used for this user. You can create up to two passwords for each user.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Passwords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the user.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Pattern: [a-zA-Z][a-zA-Z0-9\-]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserId")]
    pub user_id: cfn_resources::StrVal,

    ///
    /// The username of the user.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnUserarn,

    #[serde(skip_serializing)]
    pub att_status: CfnUserstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserarn;
impl CfnUserarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserstatus;
impl CfnUserstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnUser {
    fn type_string(&self) -> &'static str {
        "AWS::ElastiCache::User"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authentication_mode
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.user_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_id'. {} is less than 1",
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

/// Specifies the authentication mode to use.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthenticationMode {
    ///
    /// Specifies the passwords to use for authentication if Type is set to password.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Passwords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<Vec<String>>,

    ///
    /// Specifies the authentication type. Possible options are IAM authentication, password and no password.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: iam | no-password-required | password
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: AuthenticationModeTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AuthenticationModeTypeEnum {
    /// iam
    #[serde(rename = "iam")]
    Iam,

    /// no-password-required
    #[serde(rename = "no-password-required")]
    Nopasswordrequired,

    /// password
    #[serde(rename = "password")]
    Password,
}

impl Default for AuthenticationModeTypeEnum {
    fn default() -> Self {
        AuthenticationModeTypeEnum::Iam
    }
}

impl cfn_resources::CfnResource for AuthenticationMode {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
