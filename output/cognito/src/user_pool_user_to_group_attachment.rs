/// Adds the specified user to the specified group.
///
/// Calling this action requires developer credentials.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserPoolUserToGroupAttachment {
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
    fn type_string(&self) -> &'static str {
        "AWS::Cognito::UserPoolUserToGroupAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.group_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'group_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.group_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'group_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.user_pool_id;

        if the_val.len() > 55 as _ {
            return Err(format!(
                "Max validation failed on field 'user_pool_id'. {} is greater than 55",
                the_val.len()
            ));
        }

        let the_val = &self.user_pool_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'user_pool_id'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'username'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.username;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'username'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
