
pub mod cfn_custom_resource {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomResource {
    /// No documentation provided by AWS
    #[serde(rename = "ServiceToken")]
    pub service_token: String,

}



}

pub mod cfn_hook_default_version {

#[derive(serde::Serialize, Default)]
pub struct CfnHookDefaultVersion {
    /// The Amazon Resource Name (ARN) of the type version.
    #[serde(rename = "TypeVersionArn")]
    pub type_version_arn: Option<String>,
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    /// The ID of an existing version of the hook to set as the default.
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,

}



}

pub mod cfn_hook_type_config {

#[derive(serde::Serialize, Default)]
pub struct CfnHookTypeConfig {
    /// The configuration data for the extension, in this account and region.
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    /// The Amazon Resource Name (ARN) of the type without version number.
    #[serde(rename = "TypeArn")]
    pub type_arn: Option<String>,
    /// An alias by which to refer to this extension configuration data.
    #[serde(rename = "ConfigurationAlias")]
    pub configuration_alias: Option<String>,

}



}

pub mod cfn_hook_version {

#[derive(serde::Serialize, Default)]
pub struct CfnHookVersion {
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: String,
    /// Specifies logging configuration information for a type.
    #[serde(rename = "LoggingConfig")]
    pub logging_config: Option<LoggingConfig>,
    /// A url to the S3 bucket containing the schema handler package that contains the schema, event handlers, and associated files for the type you want to register.
    /// 
    /// For information on generating a schema handler package for the type you want to register, see submit in the CloudFormation CLI User Guide.
    #[serde(rename = "SchemaHandlerPackage")]
    pub schema_handler_package: String,
    /// The Amazon Resource Name (ARN) of the IAM execution role to use to register the type. If your resource type calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. CloudFormation then assumes that execution role to provide your resource type with the appropriate credentials.
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct LoggingConfig {
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogRoleArn")]
    pub log_role_arn: Option<String>,

}


}

pub mod cfn_macro {

#[derive(serde::Serialize, Default)]
pub struct CfnMacro {
    /// No documentation provided by AWS
    #[serde(rename = "LogRoleARN")]
    pub log_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}



}

pub mod cfn_module_default_version {

#[derive(serde::Serialize, Default)]
pub struct CfnModuleDefaultVersion {
    /// The name of a module existing in the registry.
    #[serde(rename = "ModuleName")]
    pub module_name: Option<String>,
    /// The Amazon Resource Name (ARN) of the module version to set as the default version.
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    /// The ID of an existing version of the named module to set as the default.
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,

}



}

pub mod cfn_module_version {

#[derive(serde::Serialize, Default)]
pub struct CfnModuleVersion {
    /// The name of the module being registered.
    /// 
    /// Recommended module naming pattern: company_or_organization::service::type::MODULE.
    #[serde(rename = "ModuleName")]
    pub module_name: String,
    /// The url to the S3 bucket containing the schema and template fragment for the module you want to register.
    #[serde(rename = "ModulePackage")]
    pub module_package: String,

}



}

pub mod cfn_public_type_version {

#[derive(serde::Serialize, Default)]
pub struct CfnPublicTypeVersion {
    /// The version number of a public third-party extension
    #[serde(rename = "PublicVersionNumber")]
    pub public_version_number: Option<String>,
    /// A url to the S3 bucket where logs for the testType run will be available
    #[serde(rename = "LogDeliveryBucket")]
    pub log_delivery_bucket: Option<String>,
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    /// The kind of extension
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}



}

pub mod cfn_publisher {

#[derive(serde::Serialize, Default)]
pub struct CfnPublisher {
    /// Whether you accept the terms and conditions for publishing extensions in the CloudFormation registry. You must accept the terms and conditions in order to publish public extensions to the CloudFormation registry. The terms and conditions can be found at https://cloudformation-registry-documents.s3.amazonaws.com/Terms_and_Conditions_for_AWS_CloudFormation_Registry_Publishers.pdf
    #[serde(rename = "AcceptTermsAndConditions")]
    pub accept_terms_and_conditions: bool,
    /// If you are using a Bitbucket or GitHub account for identity verification, the Amazon Resource Name (ARN) for your connection to that account.
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,

}



}

pub mod cfn_resource_default_version {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceDefaultVersion {
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    /// The Amazon Resource Name (ARN) of the type version.
    #[serde(rename = "TypeVersionArn")]
    pub type_version_arn: Option<String>,
    /// The ID of an existing version of the resource to set as the default.
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,

}



}

pub mod cfn_resource_version {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceVersion {
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: String,
    /// The Amazon Resource Name (ARN) of the IAM execution role to use to register the type. If your resource type calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. CloudFormation then assumes that execution role to provide your resource type with the appropriate credentials.
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    /// A url to the S3 bucket containing the schema handler package that contains the schema, event handlers, and associated files for the type you want to register.
    /// 
    /// For information on generating a schema handler package for the type you want to register, see submit in the CloudFormation CLI User Guide.
    #[serde(rename = "SchemaHandlerPackage")]
    pub schema_handler_package: String,
    /// Specifies logging configuration information for a type.
    #[serde(rename = "LoggingConfig")]
    pub logging_config: Option<LoggingConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct LoggingConfig {
    #[serde(rename = "LogRoleArn")]
    pub log_role_arn: Option<String>,
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}


}

pub mod cfn_stack {

#[derive(serde::Serialize, Default)]
pub struct CfnStack {
    /// No documentation provided by AWS
    #[serde(rename = "TimeoutInMinutes")]
    pub timeout_in_minutes: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateURL")]
    pub template_url: String,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationARNs")]
    pub notification_arns: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_stack_set {

#[derive(serde::Serialize, Default)]
pub struct CfnStackSet {
    /// The Amazon Resource Number (ARN) of the IAM role to use to create this stack set. Specify an IAM role only if you are using customized administrator roles to control which users or groups can manage specific stack sets within the same administrator account.
    #[serde(rename = "AdministrationRoleARN")]
    pub administration_role_arn: Option<String>,
    /// Describes whether StackSets automatically deploys to AWS Organizations accounts that are added to the target organization or organizational unit (OU). Specify only if PermissionModel is SERVICE_MANAGED.
    #[serde(rename = "AutoDeployment")]
    pub auto_deployment: Option<AutoDeployment>,
    /// In some cases, you must explicitly acknowledge that your stack set template contains certain capabilities in order for AWS CloudFormation to create the stack set and related stack instances.
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<Vec<Capability>>,
    /// The structure that contains the template body, with a minimum length of 1 byte and a maximum length of 51,200 bytes.
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,
    /// Specifies the AWS account that you are acting from. By default, SELF is specified. For self-managed permissions, specify SELF; for service-managed permissions, if you are signed in to the organization's management account, specify SELF. If you are signed in to a delegated administrator account, specify DELEGATED_ADMIN.
    #[serde(rename = "CallAs")]
    pub call_as: Option<String>,
    /// The user-specified preferences for how AWS CloudFormation performs a stack set operation.
    #[serde(rename = "OperationPreferences")]
    pub operation_preferences: Option<OperationPreferences>,
    /// The key-value pairs to associate with this stack set and the stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the stacks. A maximum number of 50 tags can be specified.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A description of the stack set. You can use the description to identify the stack set's purpose or other important information.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the IAM execution role to use to create the stack set. If you do not specify an execution role, AWS CloudFormation uses the AWSCloudFormationStackSetExecutionRole role for the stack set operation.
    #[serde(rename = "ExecutionRoleName")]
    pub execution_role_name: Option<String>,
    /// Describes how the IAM roles required for stack set operations are created. By default, SELF-MANAGED is specified.
    #[serde(rename = "PermissionModel")]
    pub permission_model: String,
    /// Location of file containing the template body. The URL must point to a template (max size: 460,800 bytes) that is located in an Amazon S3 bucket.
    #[serde(rename = "TemplateURL")]
    pub template_url: Option<String>,
    /// The name to associate with the stack set. The name must be unique in the Region where you create your stack set.
    #[serde(rename = "StackSetName")]
    pub stack_set_name: String,
    /// Describes whether StackSets performs non-conflicting operations concurrently and queues conflicting operations.
    #[serde(rename = "ManagedExecution")]
    pub managed_execution: Option<()>,
    /// A group of stack instances with parameters in some specific accounts and regions.
    #[serde(rename = "StackInstancesGroup")]
    pub stack_instances_group: Option<Vec<StackInstances>>,
    /// The input parameters for the stack set template.
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<Parameter>>,

}

pub type Capability = String;pub type RegionConcurrencyType = String;
#[derive(serde::Serialize, Default)]
pub struct OperationPreferences {
    #[serde(rename = "RegionOrder")]
    pub region_order: Option<Vec<Region>>,
    #[serde(rename = "RegionConcurrencyType")]
    pub region_concurrency_type: Option<RegionConcurrencyType>,
    #[serde(rename = "FailureTolerancePercentage")]
    pub failure_tolerance_percentage: Option<usize>,
    #[serde(rename = "MaxConcurrentCount")]
    pub max_concurrent_count: Option<usize>,
    #[serde(rename = "MaxConcurrentPercentage")]
    pub max_concurrent_percentage: Option<usize>,
    #[serde(rename = "FailureToleranceCount")]
    pub failure_tolerance_count: Option<usize>,

}
pub type OrganizationalUnitId = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type Account = String;
#[derive(serde::Serialize, Default)]
pub struct DeploymentTargets {
    #[serde(rename = "OrganizationalUnitIds")]
    pub organizational_unit_ids: Option<Vec<OrganizationalUnitId>>,
    #[serde(rename = "AccountFilterType")]
    pub account_filter_type: Option<String>,
    #[serde(rename = "Accounts")]
    pub accounts: Option<Vec<Account>>,

}

#[derive(serde::Serialize, Default)]
pub struct AutoDeployment {
    #[serde(rename = "RetainStacksOnAccountRemoval")]
    pub retain_stacks_on_account_removal: Option<bool>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}
pub type Active = bool;
#[derive(serde::Serialize, Default)]
pub struct StackInstances {
    #[serde(rename = "ParameterOverrides")]
    pub parameter_overrides: Option<Vec<Parameter>>,
    #[serde(rename = "DeploymentTargets")]
    pub deployment_targets: DeploymentTargets,
    #[serde(rename = "Regions")]
    pub regions: Vec<Region>,

}
pub type Region = String;
#[derive(serde::Serialize, Default)]
pub struct Parameter {
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
    #[serde(rename = "ParameterKey")]
    pub parameter_key: String,

}


}

pub mod cfn_type_activation {

#[derive(serde::Serialize, Default)]
pub struct CfnTypeActivation {
    /// The Amazon Resource Name (ARN) of the IAM execution role to use to register the type. If your resource type calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. CloudFormation then assumes that execution role to provide your resource type with the appropriate credentials.
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    /// Specifies logging configuration information for a type.
    #[serde(rename = "LoggingConfig")]
    pub logging_config: Option<LoggingConfig>,
    /// Whether to automatically update the extension in this account and region when a new minor version is published by the extension publisher. Major versions released by the publisher must be manually updated.
    #[serde(rename = "AutoUpdate")]
    pub auto_update: Option<bool>,
    /// The Major Version of the type you want to enable
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<String>,
    /// An alias to assign to the public extension in this account and region. If you specify an alias for the extension, you must then use the alias to refer to the extension in your templates.
    #[serde(rename = "TypeNameAlias")]
    pub type_name_alias: Option<String>,
    /// The name of the type being registered.
    /// 
    /// We recommend that type names adhere to the following pattern: company_or_organization::service::type.
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
    /// Manually updates a previously-enabled type to a new major or minor version, if available. You can also use this parameter to update the value of AutoUpdateEnabled
    #[serde(rename = "VersionBump")]
    pub version_bump: Option<String>,
    /// The Amazon Resource Number (ARN) assigned to the public extension upon publication
    #[serde(rename = "PublicTypeArn")]
    pub public_type_arn: Option<String>,
    /// The publisher id assigned by CloudFormation for publishing in this region.
    #[serde(rename = "PublisherId")]
    pub publisher_id: Option<String>,
    /// The kind of extension
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct LoggingConfig {
    #[serde(rename = "LogRoleArn")]
    pub log_role_arn: Option<String>,
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,

}


}

pub mod cfn_wait_condition {

#[derive(serde::Serialize, Default)]
pub struct CfnWaitCondition {
    /// No documentation provided by AWS
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Handle")]
    pub handle: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Timeout")]
    pub timeout: Option<String>,

}



}

pub mod cfn_wait_condition_handle {

#[derive(serde::Serialize, Default)]
pub struct CfnWaitConditionHandle {

}



}
