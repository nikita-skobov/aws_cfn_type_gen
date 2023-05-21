

/// The HookVersion resource publishes new or first hook version to the AWS CloudFormation  registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub execution_role_arn: Option<String>,


    /// 
    /// Contains logging configuration information for an extension.
    /// 
    /// Required: No
    ///
    /// Type: LoggingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggingConfig")]
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
    pub schema_handler_package: String,


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
    pub type_name: String,

}



impl cfn_resources::CfnResource for CfnHookVersion {
    fn type_string(&self) -> &'static str {
        "AWS::CloudFormation::HookVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.logging_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.type_name;

        if the_val.len() > 196 as _ {
            return Err(format!("Max validation failed on field 'type_name'. {} is greater than 196", the_val.len()));
        }

        
        let the_val = &self.type_name;

        if the_val.len() < 10 as _ {
            return Err(format!("Min validation failed on field 'type_name'. {} is less than 10", the_val.len()));
        }

        
        Ok(())
    }
}

/// The LoggingConfig property type specifies logging configuration information for an  extension.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub log_group_name: Option<String>,


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
    pub log_role_arn: Option<String>,

}



impl cfn_resources::CfnResource for LoggingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.log_group_name {

        if the_val.len() > 512 as _ {
            return Err(format!("Max validation failed on field 'log_group_name'. {} is greater than 512", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.log_group_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'log_group_name'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.log_role_arn {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'log_role_arn'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.log_role_arn {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'log_role_arn'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}