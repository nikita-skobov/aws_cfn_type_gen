/// Describes a user's SSH information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserProfile {
    ///
    /// Whether users can specify their own SSH public key through the My Settings page. For more    information, see Managing User     Permissions.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowSelfManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<bool>,

    ///
    /// The user's IAM ARN.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: cfn_resources::StrVal,

    ///
    /// The user's SSH public key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<cfn_resources::StrVal>,

    ///
    /// The user's SSH user name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_ssh_username: CfnUserProfilesshusername,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnUserProfilesshusername;
impl CfnUserProfilesshusername {
    pub fn att_name(&self) -> &'static str {
        r#"SshUsername"#
    }
}

impl cfn_resources::CfnResource for CfnUserProfile {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorks::UserProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
