

/// Specifies a new group in the identified user pool.
///
/// Calling this action requires developer credentials.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolGroup {


    /// 
    /// A non-negative integer value that specifies the precedence of this group relative to       the other groups that a user can belong to in the user pool. Zero is the highest       precedence value. Groups with lower Precedence values take precedence over       groups with higher or null Precedence values. If a user belongs to two or       more groups, it is the group with the lowest precedence value whose role ARN is given in       the user's tokens for the cognito:roles and         cognito:preferred_role claims.
    /// 
    /// Two groups can have the same Precedence value. If this happens, neither       group takes precedence over the other. If two groups with the same         Precedence have the same role ARN, that role is used in the         cognito:preferred_role claim in tokens for users in each group. If the       two groups have different role ARNs, the cognito:preferred_role claim isn't       set in users' tokens.
    /// 
    /// The default Precedence value is null. The maximum Precedence       value is 2^31-1.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Precedence")]
    pub precedence: Option<f64>,


    /// 
    /// The role Amazon Resource Name (ARN) for the group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:[\w+=/,.@-]+:[\w+=/,.@-]+:([\w+=/,.@-]*)?:[0-9]+:[\w+=/,.@-]+(:[\w+=/,.@-]+)?(:[\w+=/,.@-]+)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The name of the group. Must be unique.
    /// 
    /// Required: No
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
    pub group_name: Option<String>,


    /// 
    /// A string containing the description of the group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


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

}

impl cfn_resources::CfnResource for CfnUserPoolGroup {
    fn type_string() -> &'static str {
        "AWS::Cognito::UserPoolGroup"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
