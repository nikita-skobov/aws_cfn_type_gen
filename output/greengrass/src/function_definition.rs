

/// The     AWS::Greengrass::FunctionDefinition resource represents a function definition for AWS IoT Greengrass.   Function definitions are used to organize your function definition versions.
///
/// Function definitions can reference multiple function definition versions. All function definition versions      must be associated with a function definition. Each function definition version can contain one or more functions.
#[derive(Default, serde::Serialize)]
pub struct CfnFunctionDefinition {


    /// 
    /// Application-specific metadata to attach to the function definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    /// 
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    /// 
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// The name of the function definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The function definition version to include when the function definition is created.          A function definition version contains a list of          function property types.
    /// 
    /// NoteTo associate a function definition version after the function definition is created, 				   create an AWS::Greengrass::FunctionDefinitionVersion 				   resource and specify the ID of this function definition.
    /// 
    /// Required: No
    ///
    /// Type: FunctionDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<FunctionDefinitionVersion>,

}


/// The access identity whose permissions 		are used to run the Lambda function. This setting overrides the default access identity that's specified 		for the group (by default, ggc_user and ggc_group). You can override the user, group, or both. 	For more information, 	see Run as in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template,      RunAs is a property of the Execution 		 property type.
#[derive(Default, serde::Serialize)]
pub struct RunAs {


    /// 
    /// The user ID whose permissions are used to run the Lambda function. You can use the getent passwd 				 command on your core device to look up the user ID.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Uid")]
    pub uid: Option<i64>,


    /// 
    /// The group ID whose permissions are used to run the Lambda function. You can use the getent group 				 command on your core device to look up the group ID.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Gid")]
    pub gid: Option<i64>,

}


/// A function is a Lambda function 		that's referenced from an AWS IoT Greengrass group. The function is deployed to a Greengrass core where it runs locally. 	For more information, 	see Run Lambda Functions on the AWS IoT Greengrass Core in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Functions 		 property of the FunctionDefinitionVersion 		 property type contains a list of Function property types.
#[derive(Default, serde::Serialize)]
pub struct Function {


    /// 
    /// The Amazon Resource Name (ARN) of the alias (recommended) or version of the referenced Lambda function.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,


    /// 
    /// The group-specific settings of the Lambda function. These settings configure the function's behavior in the Greengrass group.
    /// 
    /// Required: Yes
    ///
    /// Type: FunctionConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionConfiguration")]
    pub function_configuration: FunctionConfiguration,


    /// 
    /// A descriptive or arbitrary ID for the function. This value must be unique within       the function definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,

}


/// A list of the 		resources in the group 				 that the function can access, with the corresponding read-only or read-write permissions. The maximum is 10 resources.
///
/// In an AWS CloudFormation template,      ResourceAccessPolicy is a property of the Environment 		 property type.
#[derive(Default, serde::Serialize)]
pub struct ResourceAccessPolicy {


    /// 
    /// The read-only or read-write access that the Lambda function has to the resource. 				 Valid values are ro or rw.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permission")]
    pub permission: Option<String>,


    /// 
    /// The ID of the resource. This ID is assigned to the resource when you create the resource definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: String,

}


/// A function definition version contains a list of functions.
///
/// In an AWS CloudFormation template, FunctionDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::FunctionDefinition resource.
#[derive(Default, serde::Serialize)]
pub struct FunctionDefinitionVersion {


    /// 
    /// The functions in this version.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Function
    ///
    /// Update requires: Replacement
    #[serde(rename = "Functions")]
    pub functions: Vec<Function>,


    /// 
    /// The default configuration that applies to all Lambda functions in the group. Individual Lambda functions can override these settings.
    /// 
    /// Required: No
    ///
    /// Type: DefaultConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DefaultConfig")]
    pub default_config: Option<DefaultConfig>,

}


/// The      group-specific configuration settings for a Lambda function. These settings configure the function's behavior in the Greengrass group. 		 For more information, 	see Controlling Execution of Greengrass Lambda Functions by Using Group-Specific Configuration in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template,      FunctionConfiguration is a property of the Function 		 property type.
#[derive(Default, serde::Serialize)]
pub struct FunctionConfiguration {


    /// 
    /// The execution arguments.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecArgs")]
    pub exec_args: Option<String>,


    /// 
    /// The name of the function executable.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Executable")]
    pub executable: Option<String>,


    /// 
    /// The expected encoding type of the input payload for the function. Valid values are json (default) and binary.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,


    /// 
    /// Indicates whether the function is pinned (or long-lived). Pinned functions start when the core starts and process all requests in the same container. The default value is       false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pinned")]
    pub pinned: Option<bool>,


    /// 
    /// The memory size (in KB) required by the function.
    /// 
    /// NoteThis property applies only to Lambda functions that run in a Greengrass container.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<i64>,


    /// 
    /// The allowed execution time (in seconds) after which the function should terminate. For pinned functions, this timeout applies for each request.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


    /// 
    /// The environment configuration of the function.
    /// 
    /// Required: No
    ///
    /// Type: Environment
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,

}


/// The environment configuration for a Lambda function on the AWS IoT Greengrass core.
///
/// In an AWS CloudFormation template,      Environment is a property of the FunctionConfiguration 		 property type.
#[derive(Default, serde::Serialize)]
pub struct Environment {


    /// 
    /// A list of the resources in the group 				 that the function can access, with the corresponding read-only or read-write permissions. The maximum is 10 resources.
    /// 
    /// NoteThis property applies only for Lambda functions that run in a Greengrass container.
    /// 
    /// Required: No
    ///
    /// Type: List of ResourceAccessPolicy
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceAccessPolicies")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,


    /// 
    /// Settings for the Lambda execution environment in AWS IoT Greengrass.
    /// 
    /// Required: No
    ///
    /// Type: Execution
    ///
    /// Update requires: Replacement
    #[serde(rename = "Execution")]
    pub execution: Option<Execution>,


    /// 
    /// Indicates whether the function is allowed to access the /sys directory on the core device, which allows the 				 read device information from /sys.
    /// 
    /// NoteThis property applies only to Lambda functions that run in a Greengrass container.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessSysfs")]
    pub access_sysfs: Option<bool>,


    /// 
    /// Environment variables for the Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Variables")]
    pub variables: Option<serde_json::Value>,

}


/// Configuration settings for the Lambda execution environment on the AWS IoT Greengrass core.
///
/// In an AWS CloudFormation template,      Execution is a property of the DefaultConfig property type for a function definition version and      the 		 Environment property type for a function.
#[derive(Default, serde::Serialize)]
pub struct Execution {


    /// 
    /// The containerization that the Lambda function runs in.           Valid values are GreengrassContainer or NoContainer. Typically, this is GreengrassContainer.           For more information,           see Containerization in the AWS IoT Greengrass Version 1 Developer Guide.
    /// 
    /// When set on the DefaultConfig property of a function            definition version,            this setting is used as the default containerization for all Lambda functions in the function definition version.            When set on the Environment property of a function,            this setting applies to the individual function and overrides the default.            Omit this value to run the function with the default containerization.
    /// 
    /// NoteWe recommend that you run in a Greengrass container unless your business case requires that you run without containerization.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IsolationMode")]
    pub isolation_mode: Option<String>,


    /// 
    /// The user and group permissions used to run the Lambda function. Typically, this is the ggc_user and ggc_group.           For more information,           see Run as in the AWS IoT Greengrass Version 1 Developer Guide.
    /// 
    /// When set on the DefaultConfig property of a function definition version,            this setting is used as the default access identity for all Lambda functions in the function definition version.            When set on the Environment property of a function,            this setting applies to the individual function and overrides the default. You can override the user, group, or both.            Omit this value to run the function with the default permissions.
    /// 
    /// ImportantRunning as the root user increases risks to your data and device. Do not run as root (UID/GID=0) unless            your business case requires it. For more information and requirements, see            Running a Lambda Function as Root.
    /// 
    /// Required: No
    ///
    /// Type: RunAs
    ///
    /// Update requires: Replacement
    #[serde(rename = "RunAs")]
    pub run_as: Option<RunAs>,

}


/// The default configuration 		that applies to all Lambda functions in the function definition version. Individual Lambda functions can override these settings.
///
/// In an AWS CloudFormation template,      DefaultConfig is a property of the FunctionDefinitionVersion 		 property type.
#[derive(Default, serde::Serialize)]
pub struct DefaultConfig {


    /// 
    /// Configuration settings for the Lambda execution environment on the AWS IoT Greengrass core.
    /// 
    /// Required: Yes
    ///
    /// Type: Execution
    ///
    /// Update requires: No interruption
    #[serde(rename = "Execution")]
    pub execution: Execution,

}
