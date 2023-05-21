

/// Registers a resource version with the CloudFormation service. Registering a resource version makes it  available for use in CloudFormation templates in your AWS account, and includes:
///
/// For more information on how to develop resources and ready them for registration, see Creating Resource   Providers in the CloudFormation CLI User Guide.
///
/// You can have a maximum of 50 resource versions registered at a time. This maximum is per account and per  Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceVersion {


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
    pub type_name: String,


    /// 
    /// Logging configuration information for a resource.
    /// 
    /// Required: No
    ///
    /// Type: LoggingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggingConfig")]
    pub logging_config: Option<LoggingConfig>,


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
    pub execution_role_arn: Option<String>,


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
    pub schema_handler_package: String,

}



impl cfn_resources::CfnResource for CfnResourceVersion {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::ResourceVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Logging configuration information for a resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingConfig {


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
    pub log_role_arn: Option<String>,


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
    pub log_group_name: Option<String>,

}


