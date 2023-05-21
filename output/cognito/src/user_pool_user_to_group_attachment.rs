

/// Adds the specified user to the specified group.
///
/// Calling this action requires developer credentials.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolUserToGroupAttachment {


    /// 
    /// The user pool ID for the user pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 55
    ///
    /// Pattern: [\w-]+_[0-9a-zA-Z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: String,


    /// 
    /// The group name.
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
    #[serde(rename = "GroupName")]
    pub group_name: String,


    /// 
    /// The username for the user.
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
    #[serde(rename = "Username")]
    pub username: String,

}



impl cfn_resources::CfnResource for CfnUserPoolUserToGroupAttachment {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolUserToGroupAttachment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
