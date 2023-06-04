/// Adds the specified user to the specified group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnUserToGroupAddition {
    ///
    /// The name of the group to update.
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: cfn_resources::StrVal,

    ///
    /// A list of the names of the users that you want to add to the group.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Vec<String>,
}

impl cfn_resources::CfnResource for CfnUserToGroupAddition {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::UserToGroupAddition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'group_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'group_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
