

/// The AWS::AppConfig::ConfigurationProfile resource creates a configuration    profile that enables AWS AppConfig to access the configuration source. Valid    configuration sources include AWS Systems Manager (SSM) documents, SSM Parameter Store    parameters, and Amazon S3. A configuration profile includes the following    information.
///
/// AWS AppConfig requires that you create resources and deploy a configuration in the    following order:
///
/// For more information, see AWS AppConfig in the      AWS AppConfig User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConfigurationProfile {


    /// 
    /// The application ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: [a-z0-9]{4,7}
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApplicationId")]
    pub application_id: String,


    /// 
    /// A description of the configuration profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A URI to locate the configuration. You can specify the following:
    /// 
    /// For the AWS AppConfig hosted configuration store and for feature flags,        specify hosted.               For an AWS Systems Manager Parameter Store parameter, specify either the parameter name in        the format ssm-parameter://<parameter name> or the ARN.               For an AWS CodePipeline pipeline, specify the URI in the following format:        codepipeline://<pipeline name>.               For an AWS Secrets Manager secret, specify the URI in the following format:          secretsmanager://<secret name>.               For an Amazon S3 object, specify the URI in the following format:          s3://<bucket>/<objectKey> . Here is an example:          s3://my-bucket/my-app/us-east-1/my-config.json                       For an SSM document, specify either the document name in the format          ssm-document://<document name> or the Amazon Resource Name        (ARN).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocationUri")]
    pub location_uri: String,


    /// 
    /// A name for the configuration profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The ARN of an IAM role with permission to access the configuration at the specified       LocationUri.
    /// 
    /// ImportantA retrieval role ARN is not required for configurations stored in the AWS AppConfig hosted configuration store. It is required for all other sources that       store your configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^((arn):(aws|aws-cn|aws-iso|aws-iso-[a-z]{1}|aws-us-gov):(iam)::\d{12}:role[/].*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetrievalRoleArn")]
    pub retrieval_role_arn: Option<String>,


    /// 
    /// Metadata to assign to the configuration profile. Tags help organize and categorize your       AWS AppConfig resources. Each tag consists of a key and an optional value, both of     which you define.
    /// 
    /// Required: No
    ///
    /// Type: List of Tags
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tags>>,


    /// 
    /// The type of configurations contained in the profile. AWS AppConfig supports       feature flags and freeform configurations. We recommend you     create feature flag configurations to enable or disable new features and freeform     configurations to distribute configurations to an application. When calling this API, enter     one of the following values for Type:
    /// 
    /// AWS.AppConfig.FeatureFlags
    /// 
    /// AWS.Freeform
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z\.]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,


    /// 
    /// A list of methods for validating the configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of Validators
    ///
    /// Maximum: 2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validators")]
    pub validators: Option<Vec<Validators>>,

}



impl cfn_resources::CfnResource for CfnConfigurationProfile {
    fn type_string() -> &'static str {
        "AWS::AppConfig::ConfigurationProfile"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Metadata to assign to the configuration profile. Tags help organize and categorize your       AWS AppConfig resources. Each tag consists of a key and an optional value, both of     which you define.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tags {


    /// 
    /// The key-value string map. The valid character set is [a-zA-Z+-=._:/]. The tag    key can be up to 128 characters and must not start with aws:.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The tag value can be up to 256 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}




/// A validator provides a syntactic or semantic check to ensure the configuration that you     want to deploy functions as intended. To validate your application configuration data, you     provide a schema or an AWS Lambda function that runs against the configuration. The     configuration deployment or update can only proceed when the configuration data is     valid.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Validators {


    /// 
    /// Either the JSON Schema content or the Amazon Resource Name (ARN) of an Lambda     function.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 32768
    ///
    /// Update requires: No interruption
    #[serde(rename = "Content")]
    pub content: Option<String>,


    /// 
    /// AWS AppConfig supports validators of type JSON_SCHEMA and       LAMBDA
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: JSON_SCHEMA | LAMBDA
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<ValidatorsTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ValidatorsTypeEnum {

    /// JSON_SCHEMA
    #[serde(rename = "JSON_SCHEMA")]
    Jsonschema,

    /// LAMBDA
    #[serde(rename = "LAMBDA")]
    Lambda,

}

impl Default for ValidatorsTypeEnum {
    fn default() -> Self {
        ValidatorsTypeEnum::Jsonschema
    }
}

