/// The HookTypeConfig resource specifies the configuration of a hook.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnHookTypeConfig {
    ///
    /// Specifies the activated hook type configuration, in this AWS account and AWS Region.
    ///
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: String,

    ///
    /// Specifies the activated hook type configuration, in this AWS account and AWS Region.
    ///
    /// Defaults to default alias. Hook types currently support default configuration alias.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_alias: Option<String>,

    ///
    /// The Amazon Resource Number (ARN) for the hook to set Configuration for.
    ///
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,

    ///
    /// The unique name for your hook. Specifies a three-part namespace for your hook, with a recommended pattern of   Organization::Service::Hook.
    ///
    /// You must specify either TypeName and Configuration or TypeARN and   Configuration.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 196
    ///
    /// Update requires: No interruption
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

impl cfn_resources::CfnResource for CfnHookTypeConfig {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::HookTypeConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.type_name {
            if the_val.len() > 196 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 196",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.type_name {
            if the_val.len() < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'type_name'. {} is less than 10",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}
