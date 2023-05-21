/// Registers a resource version with the CloudFormation service. Registering a resource version makes it  available for use in CloudFormation templates in your AWS account, and includes:
///
/// For more information on how to develop resources and ready them for registration, see Creating Resource   Providers in the CloudFormation CLI User Guide.
///
/// You can have a maximum of 50 resource versions registered at a time. This maximum is per account and per  Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceVersion {
    ///
    /// The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when  invoking the resource. If your resource calls AWS APIs in any of its handlers, you must create an   IAM execution   role that includes the necessary permissions to call those AWS APIs, and  provision that execution role in your account. When CloudFormation needs to invoke the resource type  handler, CloudFormation assumes this execution role to create a temporary session token, which it then  passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.
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
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Logging configuration information for a resource.
    ///
    /// Required: No
    ///
    /// Type: LoggingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,

    ///
    /// A URL to the S3 bucket containing the resource project package that contains the necessary files for the  resource you want to register.
    ///
    /// For information on generating a schema handler package for the resource you want to register, see submit in the   CloudFormation CLI User Guide.
    ///
    /// NoteThe user registering the resource must be able to access the package in the S3 bucket. That is, the user needs   to have GetObject permissions for   the schema handler package. For more information, see Actions, Resources, and Condition Keys for Amazon S3   in the AWS Identity and Access Management User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: Replacement
    #[serde(rename = "SchemaHandlerPackage")]
    pub schema_handler_package: cfn_resources::StrVal,

    ///
    /// The name of the resource being registered.
    ///
    /// We recommend that resource names adhere to the following pattern:   company_or_organization::service::type.
    ///
    /// NoteThe following organization namespaces are reserved and can't be used in your resource names:                       Alexa      AMZN      Amazon      AWS      Custom      Dev
    ///
    /// Required: Yes
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
    #[serde(rename = "TypeName")]
    pub type_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnResourceVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::ResourceVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.execution_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'execution_role_arn'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.execution_role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'execution_role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.logging_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.schema_handler_package;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4096 as _ {
                return Err(format!("Max validation failed on field 'schema_handler_package'. {} is greater than 4096", s.len()));
            }
        }

        let the_val = &self.schema_handler_package;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'schema_handler_package'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.type_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 204 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_name'. {} is greater than 204",
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

/// Logging configuration information for a resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfig {
    ///
    /// The Amazon CloudWatch logs group to which CloudFormation sends error logging information when invoking  the type's handlers.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the role that CloudFormation should assume when sending log entries to CloudWatch  logs.
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
