

/// Activates a public third-party extension, making it available for use in stack templates. For more information,  see Using public   extensions in the         AWS CloudFormation User Guide.
///
/// Once you have activated a public third-party extension in your account and Region, use SetTypeConfiguration to specify configuration properties for the extension. For more information, see   Configuring extensions at   the account level in the CloudFormation User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTypeActivation {


    /// 
    /// The Amazon Resource Number (ARN) of the public extension.
    /// 
    /// Conditional: You must specify PublicTypeArn, or TypeName, Type, and   PublisherId.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws[A-Za-z0-9-]{0,64}:cloudformation:[A-Za-z0-9-]{1,64}::type/.+/[0-9a-zA-Z]{12,40}/.+
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublicTypeArn")]
    pub public_type_arn: Option<String>,


    /// 
    /// The extension type.
    /// 
    /// Conditional: You must specify PublicTypeArn, or TypeName, Type, and   PublisherId.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: HOOK | MODULE | RESOURCE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<TypeActivationTypeEnum>,


    /// 
    /// The name of the extension.
    /// 
    /// Conditional: You must specify PublicTypeArn, or TypeName, Type, and   PublisherId.
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
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,


    /// 
    /// The major version of this extension you want to activate, if multiple major versions are available. The default  is the latest major version. CloudFormation uses the latest available minor version of  the major version selected.
    /// 
    /// You can specify MajorVersion or VersionBump, but not both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<String>,


    /// 
    /// An alias to assign to the public extension, in this account and Region. If you specify an alias for the  extension, CloudFormation treats the alias as the extension type name within this account and Region. You  must use the alias to refer to the extension in your templates, API calls, and CloudFormation  console.
    /// 
    /// An extension alias must be unique within a given account and Region. You can activate the same public resource  multiple times in the same account and Region, using different type name aliases.
    /// 
    /// Required: No
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
    #[serde(rename = "TypeNameAlias")]
    pub type_name_alias: Option<String>,


    /// 
    /// Manually updates a previously-activated type to a new major or minor version, if available. You can also use  this parameter to update the value of AutoUpdate.
    /// 
    /// MAJOR: CloudFormation updates the extension to the newest major version, if one is   available.                        MINOR: CloudFormation updates the extension to the newest minor version, if one is   available.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MAJOR | MINOR
    ///
    /// Update requires: No interruption
    #[serde(rename = "VersionBump")]
    pub version_bump: Option<TypeActivationVersionBumpEnum>,


    /// 
    /// The name of the IAM execution role to use to activate the extension.
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
    /// Whether to automatically update the extension in this account and Region when a new minor  version is published by the extension publisher. Major versions released by the publisher must be manually  updated.
    /// 
    /// The default is true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoUpdate")]
    pub auto_update: Option<bool>,


    /// 
    /// The ID of the extension publisher.
    /// 
    /// Conditional: You must specify PublicTypeArn, or TypeName, Type, and   PublisherId.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 40
    ///
    /// Pattern: [0-9a-zA-Z]{12,40}
    ///
    /// Update requires: Replacement
    #[serde(rename = "PublisherId")]
    pub publisher_id: Option<String>,


    /// 
    /// Specifies logging configuration information for an extension.
    /// 
    /// Required: No
    ///
    /// Type: LoggingConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoggingConfig")]
    pub logging_config: Option<LoggingConfig>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TypeActivationTypeEnum {

    /// HOOK
    #[serde(rename = "HOOK")]
    Hook,

    /// MODULE
    #[serde(rename = "MODULE")]
    Module,

    /// RESOURCE
    #[serde(rename = "RESOURCE")]
    Resource,

}

impl Default for TypeActivationTypeEnum {
    fn default() -> Self {
        TypeActivationTypeEnum::Hook
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TypeActivationVersionBumpEnum {

    /// MAJOR
    #[serde(rename = "MAJOR")]
    Major,

    /// MINOR
    #[serde(rename = "MINOR")]
    Minor,

}

impl Default for TypeActivationVersionBumpEnum {
    fn default() -> Self {
        TypeActivationVersionBumpEnum::Major
    }
}


impl cfn_resources::CfnResource for CfnTypeActivation {
    fn type_string() -> &'static str {
        "AWS::CloudFormation::TypeActivation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains logging configuration information for an extension.
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


