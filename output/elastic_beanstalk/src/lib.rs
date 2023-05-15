
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// A name for the Elastic Beanstalk application. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the application name.
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,
    /// Your description of the application.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Specifies an application resource lifecycle configuration to prevent your application from accumulating too many versions.
    #[serde(rename = "ResourceLifecycleConfig")]
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct MaxAgeRule {
    #[serde(rename = "MaxAgeInDays")]
    pub max_age_in_days: Option<usize>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "DeleteSourceFromS3")]
    pub delete_source_from_s3: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationResourceLifecycleConfig {
    #[serde(rename = "VersionLifecycleConfig")]
    pub version_lifecycle_config: Option<ApplicationVersionLifecycleConfig>,
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MaxCountRule {
    #[serde(rename = "MaxCount")]
    pub max_count: Option<usize>,
    #[serde(rename = "DeleteSourceFromS3")]
    pub delete_source_from_s3: Option<bool>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationVersionLifecycleConfig {
    #[serde(rename = "MaxCountRule")]
    pub max_count_rule: Option<MaxCountRule>,
    #[serde(rename = "MaxAgeRule")]
    pub max_age_rule: Option<MaxAgeRule>,

}


}

pub mod cfn_application_version {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationVersion {
    /// The Amazon S3 bucket and key that identify the location of the source bundle for this version.
    #[serde(rename = "SourceBundle")]
    pub source_bundle: SourceBundle,
    /// A description of this application version.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the Elastic Beanstalk application that is associated with this application version.
    #[serde(rename = "ApplicationName")]
    pub application_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct SourceBundle {
    #[serde(rename = "S3Key")]
    pub s3_key: String,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,

}


}

pub mod cfn_configuration_template {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationTemplate {
    /// The Amazon Resource Name (ARN) of the custom platform. For more information, see [Custom Platforms](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/custom-platforms.html) in the AWS Elastic Beanstalk Developer Guide.
    #[serde(rename = "PlatformArn")]
    pub platform_arn: Option<String>,
    /// The ID of an environment whose settings you want to use to create the configuration template. You must specify EnvironmentId if you don't specify PlatformArn, SolutionStackName, or SourceConfiguration.
    #[serde(rename = "EnvironmentId")]
    pub environment_id: Option<String>,
    /// An Elastic Beanstalk configuration template to base this one on. If specified, Elastic Beanstalk uses the configuration values from the specified configuration template to create a new configuration.
    /// 
    /// Values specified in OptionSettings override any values obtained from the SourceConfiguration.
    /// 
    /// You must specify SourceConfiguration if you don't specify PlatformArn, EnvironmentId, or SolutionStackName.
    /// 
    /// Constraint: If both solution stack name and source configuration are specified, the solution stack of the source configuration template must match the specified solution stack name.
    #[serde(rename = "SourceConfiguration")]
    pub source_configuration: Option<SourceConfiguration>,
    /// The name of the Elastic Beanstalk application to associate with this configuration template.
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// The name of an Elastic Beanstalk solution stack (platform version) that this configuration uses. For example, 64bit Amazon Linux 2013.09 running Tomcat 7 Java 7. A solution stack specifies the operating system, runtime, and application server for a configuration template. It also determines the set of configuration options as well as the possible and default values. For more information, see [Supported Platforms](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/concepts.platforms.html) in the AWS Elastic Beanstalk Developer Guide.
    /// 
    /// You must specify SolutionStackName if you don't specify PlatformArn, EnvironmentId, or SourceConfiguration.
    /// 
    /// Use the ListAvailableSolutionStacks API to obtain a list of available solution stacks.
    #[serde(rename = "SolutionStackName")]
    pub solution_stack_name: Option<String>,
    /// An optional description for this configuration.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Option values for the Elastic Beanstalk configuration, such as the instance type. If specified, these values override the values obtained from the solution stack or the source configuration template. For a complete list of Elastic Beanstalk configuration options, see [Option Values](https://docs.aws.amazon.com/elasticbeanstalk/latest/dg/command-options.html) in the AWS Elastic Beanstalk Developer Guide.
    #[serde(rename = "OptionSettings")]
    pub option_settings: Option<Vec<ConfigurationOptionSetting>>,

}


#[derive(serde::Serialize, Default)]
pub struct ConfigurationOptionSetting {
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "OptionName")]
    pub option_name: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceConfiguration {
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    #[serde(rename = "TemplateName")]
    pub template_name: String,

}


}

pub mod cfn_environment {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironment {
    /// The name of the application that is associated with this environment.
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// The name of the application version to deploy.
    #[serde(rename = "VersionLabel")]
    pub version_label: Option<String>,
    /// A unique name for the environment.
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,
    /// The Amazon Resource Name (ARN) of an existing IAM role to be used as the environment's operations role.
    #[serde(rename = "OperationsRole")]
    pub operations_role: Option<String>,
    /// The Amazon Resource Name (ARN) of the custom platform to use with the environment.
    #[serde(rename = "PlatformArn")]
    pub platform_arn: Option<String>,
    /// Key-value pairs defining configuration options for this environment, such as the instance type.
    #[serde(rename = "OptionSettings")]
    pub option_settings: Option<Vec<OptionSetting>>,
    /// The name of the Elastic Beanstalk configuration template to use with the environment.
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,
    /// The name of an Elastic Beanstalk solution stack (platform version) to use with the environment.
    #[serde(rename = "SolutionStackName")]
    pub solution_stack_name: Option<String>,
    /// Specifies the tier to use in creating this environment. The environment tier that you choose determines whether Elastic Beanstalk provisions resources to support a web application that handles HTTP(S) requests or a web application that handles background-processing tasks.
    #[serde(rename = "Tier")]
    pub tier: Option<Tier>,
    /// Your description for this environment.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Specifies the tags applied to resources in the environment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// If specified, the environment attempts to use this value as the prefix for the CNAME in your Elastic Beanstalk environment URL. If not specified, the CNAME is generated automatically by appending a random alphanumeric string to the environment name.
    #[serde(rename = "CNAMEPrefix")]
    pub cnameprefix: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tier {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OptionSetting {
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "OptionName")]
    pub option_name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}
