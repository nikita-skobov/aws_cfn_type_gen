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
    pub configuration: cfn_resources::StrVal,

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
    pub configuration_alias: Option<cfn_resources::StrVal>,

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
    pub type_arn: Option<cfn_resources::StrVal>,

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
    pub type_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_configuration_arn: CfnHookTypeConfigconfigurationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnHookTypeConfigconfigurationarn;
impl CfnHookTypeConfigconfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"ConfigurationArn"#
    }
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
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 196 as _ {
                    return Err(format!(
                        "Max validation failed on field 'type_name'. {} is greater than 196",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.type_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 10 as _ {
                    return Err(format!(
                        "Min validation failed on field 'type_name'. {} is less than 10",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
