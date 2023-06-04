/// Creates a new instance profile. For information about instance profiles, see Using       instance profiles.
///
/// For information about the number of instance profiles you can create, see IAM object quotas in the IAM User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceProfile {
    ///
    /// The name of the instance profile to create.
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<cfn_resources::StrVal>,

    ///
    /// The path to the instance profile. For more information about paths, see IAM         Identifiers in the IAM User Guide.
    ///
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: (\u002F)|(\u002F[\u0021-\u007E]+\u002F)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// The name of the role to associate with the instance profile. Only one role can be     assigned to an EC2 instance at a time, and all applications on the instance share the same     role and permissions.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Roles")]
    pub roles: Vec<String>,

    #[serde(skip_serializing)]
    pub att_arn: CfnInstanceProfilearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnInstanceProfilearn;
impl CfnInstanceProfilearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnInstanceProfile {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::InstanceProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.instance_profile_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'instance_profile_name'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.instance_profile_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'instance_profile_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
