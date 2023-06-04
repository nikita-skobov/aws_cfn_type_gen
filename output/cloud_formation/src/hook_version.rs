/// The HookVersion resource publishes new or first hook version to the AWS CloudFormation  registry.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnHookVersion {
    ///
    /// The Amazon Resource Name (ARN) of the task execution role that grants the hook permission.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub execution_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Contains logging configuration information for an extension.
    ///
    /// Required: No
    ///
    /// Type: LoggingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggingConfig")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub logging_config: Option<LoggingConfig>,

    ///
    /// A URL to the Amazon S3 bucket containing the hook project package that contains the necessary files for the hook  you want to register.
    ///
    /// For information on generating a schema handler package for the resource you want to register, see submit in the   CloudFormation CLI User Guide for Extension Development.
    ///
    /// NoteThe user registering the resource must be able to access the package in the S3 bucket. That's, the user must   have GetObject permissions for the   schema handler package. For more information, see Actions, Resources, and Condition Keys for Amazon S3   in the AWS Identity and Access Management User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaHandlerPackage")]
    pub schema_handler_package: cfn_resources::StrVal,

    ///
    /// The unique name for your hook. Specifies a three-part namespace for your hook, with a recommended pattern of   Organization::Service::Hook.
    ///
    /// NoteThe following organization namespaces are reserved and can't be used in your hook type names:                                                                                       Alexa                                      AMZN                                      Amazon                                      ASK                                      AWS                                      Custom                                      Dev
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 196
    ///
    /// Update requires: Replacement
    #[serde(rename = "TypeName")]
    pub type_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnHookVersionarn,

    #[serde(skip_serializing)]
    pub att_type_arn: CfnHookVersiontypearn,

    #[serde(skip_serializing)]
    pub att_version_id: CfnHookVersionversionid,

    #[serde(skip_serializing)]
    pub att_visibility: CfnHookVersionvisibility,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnHookVersionarn;
impl CfnHookVersionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnHookVersiontypearn;
impl CfnHookVersiontypearn {
    pub fn att_name(&self) -> &'static str {
        r#"TypeArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnHookVersionversionid;
impl CfnHookVersionversionid {
    pub fn att_name(&self) -> &'static str {
        r#"VersionId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnHookVersionvisibility;
impl CfnHookVersionvisibility {
    pub fn att_name(&self) -> &'static str {
        r#"Visibility"#
    }
}

impl cfn_resources::CfnResource for CfnHookVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::HookVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.logging_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.type_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 196 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 196",
                    s.len()
                ));
            }
        }

        let the_val = &self.type_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'type_name'. {} is less than 10",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The LoggingConfig property type specifies logging configuration information for an  extension.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    ///
    /// The Amazon CloudWatch Logs group to which CloudFormation sends error logging information when invoking  the extension's handlers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub log_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the role that CloudFormation should assume when sending log entries  to CloudWatch Logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:.+:iam::[0-9]{12}:role/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogRoleArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub log_role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LoggingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.log_group_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'log_group_name'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.log_group_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'log_group_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.log_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'log_role_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.log_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'log_role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
