

/// The AWS::ElasticBeanstalk::ConfigurationTemplate resource is an AWS Elastic Beanstalk    resource type that specifies an Elastic Beanstalk configuration template, associated with a    specific Elastic Beanstalk application. You define application configuration settings in a    configuration template. You can then use the configuration template to deploy different    versions of the application with the same configuration settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConfigurationTemplate {


    /// 
    /// The name of the Elastic Beanstalk application to associate with this configuration    template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: String,


    /// 
    /// An optional description for this configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The ID of an environment whose settings you want to use to create the configuration    template. You must specify EnvironmentId if you don't specify     PlatformArn, SolutionStackName, or     SourceConfiguration.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnvironmentId")]
    pub environment_id: Option<String>,


    /// 
    /// Option values for the Elastic Beanstalk configuration, such as the instance type. If specified, these    values override the values obtained from the solution stack or the source configuration    template. For a complete list of Elastic Beanstalk configuration options, see Option Values in the             AWS Elastic Beanstalk Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of ConfigurationOptionSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionSettings")]
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,


    /// 
    /// The Amazon Resource Name (ARN) of the custom platform. For more information, see Custom     Platforms in the         AWS Elastic Beanstalk Developer Guide.
    /// 
    /// NoteIf you specify PlatformArn, then don't specify      SolutionStackName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlatformArn")]
    pub platform_arn: Option<String>,


    /// 
    /// The name of an Elastic Beanstalk solution stack (platform version) that this configuration uses. For    example, 64bit Amazon Linux 2013.09 running Tomcat 7 Java 7. A solution stack    specifies the operating system, runtime, and application server for a configuration template.    It also determines the set of configuration options as well as the possible and default    values. For more information, see Supported Platforms in the             AWS Elastic Beanstalk Developer Guide.
    /// 
    /// You must specify SolutionStackName if you don't specify     PlatformArn, EnvironmentId, or    SourceConfiguration.
    /// 
    /// Use the ListAvailableSolutionStacks API to obtain a list of available    solution stacks.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SolutionStackName")]
    pub solution_stack_name: Option<String>,


    /// 
    /// An Elastic Beanstalk configuration template to base this one on. If specified, Elastic Beanstalk uses the configuration values from the specified    configuration template to create a new configuration.
    /// 
    /// Values specified in OptionSettings override any values obtained from the     SourceConfiguration.
    /// 
    /// You must specify SourceConfiguration if you don't specify     PlatformArn, EnvironmentId, or    SolutionStackName.
    /// 
    /// Constraint: If both solution stack name and source configuration are specified, the    solution stack of the source configuration template must match the specified solution stack    name.
    /// 
    /// Required: Conditional
    ///
    /// Type: SourceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceConfiguration")]
    pub source_configuration: Option<SourceConfiguration>,

}



impl cfn_resources::CfnResource for CfnConfigurationTemplate {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticBeanstalk::ConfigurationTemplate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.application_name;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'application_name'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.application_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'application_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.description {

        if the_val.len() > 200 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 200", the_val.len()));
        }

        }
        
        self.source_configuration.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The ConfigurationOptionSetting property type specifies an option for an AWS Elastic Beanstalk configuration template.
///
/// The OptionSettings property of the AWS::ElasticBeanstalk::ConfigurationTemplate    resource contains a list of ConfigurationOptionSetting property types.
///
/// For a list of possible namespaces and option values, see Option Values in the     AWS Elastic Beanstalk Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationOptionSetting {


    /// 
    /// A unique namespace that identifies the option's associated AWS resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: String,


    /// 
    /// The name of the configuration option.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OptionName")]
    pub option_name: String,


    /// 
    /// A unique resource name for the option setting. Use it for a time–based scaling configuration option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,


    /// 
    /// The current value for the configuration option.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}



impl cfn_resources::CfnResource for ConfigurationOptionSetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.resource_name {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'resource_name'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.resource_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'resource_name'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// An AWS Elastic Beanstalk configuration template to base a new one on. You can use it to    define a AWS::ElasticBeanstalk::ConfigurationTemplate resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceConfiguration {


    /// 
    /// The name of the application associated with the configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationName")]
    pub application_name: String,


    /// 
    /// The name of the configuration template.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateName")]
    pub template_name: String,

}



impl cfn_resources::CfnResource for SourceConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.application_name;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'application_name'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.application_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'application_name'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.template_name;

        if the_val.len() > 100 as _ {
            return Err(format!("Max validation failed on field 'template_name'. {} is greater than 100", the_val.len()));
        }

        
        let the_val = &self.template_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'template_name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}