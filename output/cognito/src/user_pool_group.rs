/// Specifies a new group in the identified user pool.
///
/// Calling this action requires developer credentials.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserPoolGroup {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

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
    pub user_pool_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnUserPoolGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.group_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'group_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.group_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'group_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.precedence {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'precedence'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 20",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 55 as _ {
                return Err(format!(
                    "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                    s.len()
                ));
            }
        }

        let the_val = &self.user_pool_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'user_pool_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
