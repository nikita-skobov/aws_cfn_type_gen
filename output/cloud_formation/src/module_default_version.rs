/// Specifies the default version of a module. The default version of the module will be used in CloudFormation operations for this account and Region.
///
/// To register a module version, use the AWS::CloudFormation::ModuleVersion resource.
///
/// For more information using modules, see Using modules to encapsulate and reuse resource   configurations and Registering extensions in the   AWS CloudFormation User Guide. For information on developing modules, see Developing modules in the  AWS CloudFormation CLI User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnModuleDefaultVersion {
    ///
    /// The Amazon Resource Name (ARN) of the module version to set as the default version.
    ///
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}:[0-9]{12}:type/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the module.
    ///
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 204
    ///
    /// Pattern: [A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}::[A-Za-z0-9]{2,64}(::MODULE){0,1}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<cfn_resources::StrVal>,

    ///
    /// The ID for the specific version of the module.
    ///
    /// Conditional: You must specify either Arn, or ModuleName and  VersionId.
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
    /// Update requires: Replacement
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnModuleDefaultVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::ModuleDefaultVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'arn'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.module_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 204 as _ {
                    return Err(format!(
                        "Max validation failed on field 'module_name'. {} is greater than 204",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.module_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 10 as _ {
                    return Err(format!(
                        "Min validation failed on field 'module_name'. {} is less than 10",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version_id'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
