/// The HookDefaultVersion resource specifies the default version of the hook. The default version of  the hook is used in CloudFormation operations for this AWS account and AWS Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnHookDefaultVersion {
    ///
    /// The name of the hook.
    ///
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,

    ///
    /// The version ID of the type configuration.
    ///
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [A-Za-z0-9-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeVersionArn")]
    pub type_version_arn: Option<String>,

    ///
    /// The version ID of the type specified.
    ///
    /// You must specify either TypeVersionArn, or TypeName and VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [A-Za-z0-9-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
}

impl cfn_resources::CfnResource for CfnHookDefaultVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::HookDefaultVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.type_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'type_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_version_arn {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_version_arn'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_version_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'type_version_arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.version_id {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'version_id'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.version_id {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'version_id'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
