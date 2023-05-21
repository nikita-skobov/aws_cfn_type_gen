

/// Describes a user's SSH information.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub iam_user_arn: String,


    /// 
    /// The user's SSH public key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshPublicKey")]
    pub ssh_public_key: Option<String>,


    /// 
    /// The user's SSH user name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SshUsername")]
    pub ssh_username: Option<String>,

}



impl cfn_resources::CfnResource for CfnUserProfile {
    fn type_string() -> &'static str {
        "AWS::OpsWorks::UserProfile"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
