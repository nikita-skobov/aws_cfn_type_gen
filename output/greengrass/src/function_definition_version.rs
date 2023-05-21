/// The     AWS::Greengrass::FunctionDefinitionVersion resource represents a function definition version for AWS IoT Greengrass.     A function definition version contains contain a list of functions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFunctionDefinitionVersion {
    ///
    /// The default configuration that applies to all Lambda functions in the group. Individual Lambda functions can override these settings.
    ///
    /// Required: No
    ///
    /// Type: DefaultConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DefaultConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<DefaultConfig>,

    ///
    /// The ID of the function definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for CfnFunctionDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::FunctionDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The default configuration 		that applies to all Lambda functions in the function definition version. Individual Lambda functions can override these settings.
///
/// In an AWS CloudFormation template,      DefaultConfig is a property of the AWS::Greengrass::FunctionDefinitionVersion resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for DefaultConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.execution.validate()?;

        Ok(())
    }
}

/// The environment configuration for a Lambda function on the AWS IoT Greengrass core.
///
/// In an AWS CloudFormation template,      Environment is a property of the FunctionConfiguration 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Environment {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_sysfs: Option<bool>,

    ///
    /// Settings for the Lambda execution environment in AWS IoT Greengrass.
    ///
    /// Required: No
    ///
    /// Type: Execution
    ///
    /// Update requires: Replacement
    #[serde(rename = "Execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<Execution>,

    ///
    /// A list of the resources in the group 				 that the function can access, with the corresponding read-only or read-write permissions. The maximum is 10 resources.
    ///
    /// NoteThis property applies only to Lambda functions that run in a Greengrass container.
    ///
    /// Required: No
    ///
    /// Type: List of ResourceAccessPolicy
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceAccessPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,

    ///
    /// Environment variables for the Lambda function.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for Environment {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.execution
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Configuration settings for the Lambda execution environment on the AWS IoT Greengrass core.
///
/// In an AWS CloudFormation template,      Execution is a property of the DefaultConfig property type for a function definition version and      the 		 Environment property type for a function.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Execution {
    ///
    /// The containerization that the Lambda function runs in.           Valid values are GreengrassContainer or NoContainer. Typically, this is GreengrassContainer. 	For more information, 	see Containerization in the AWS IoT Greengrass Version 1 Developer Guide.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_mode: Option<cfn_resources::StrVal>,

    ///
    /// The user and group permissions used to run the Lambda function. Typically, this is the ggc_user and ggc_group. 	For more information, 	see Run as in the AWS IoT Greengrass Version 1 Developer Guide.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as: Option<RunAs>,
}

impl cfn_resources::CfnResource for Execution {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.run_as.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A function is a Lambda function 		that's referenced from an AWS IoT Greengrass group. The function is deployed to a Greengrass core where it runs locally. 	For more information, 	see Run Lambda Functions on the AWS IoT Greengrass Core in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Functions 		 property of the AWS::Greengrass::FunctionDefinitionVersion resource contains a      list of Function property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub function_arn: cfn_resources::StrVal,

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
    pub id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Function {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.function_configuration.validate()?;

        Ok(())
    }
}

/// The      group-specific configuration settings for a Lambda function. These settings configure the function's behavior in the Greengrass group. 		 For more information, 	see Controlling Execution of Greengrass Lambda Functions by Using Group-Specific Configuration in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template,      FunctionConfiguration is a property of the Function property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FunctionConfiguration {
    ///
    /// The expected encoding type of the input payload for the function. Valid values are json (default) and binary.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<cfn_resources::StrVal>,

    ///
    /// The environment configuration of the function.
    ///
    /// Required: No
    ///
    /// Type: Environment
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,

    ///
    /// The execution arguments.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExecArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_args: Option<cfn_resources::StrVal>,

    ///
    /// The name of the function executable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Executable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,

    ///
    /// Indicates whether the function is pinned (or long-lived). Pinned functions start when the core starts and process all requests in the same container. The default value is       false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,

    ///
    /// The allowed execution time (in seconds) after which the function should terminate. For pinned functions, this timeout applies for each request.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl cfn_resources::CfnResource for FunctionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.environment
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A list of the 		resources in the group 				 that the function can access, with the corresponding read-only or read-write permissions. The maximum is 10 resources.
///
/// In an AWS CloudFormation template,      ResourceAccessPolicy is a property of the Environment 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the resource. This ID is assigned to the resource when you create the resource definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ResourceAccessPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The user and group permissions 		used to run the Lambda function. This setting overrides the default access identity that's specified 		for the group (by default, ggc_user and ggc_group). You can override the user, group, or both. 	For more information, 	see Run as in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template,      RunAs is a property of the Execution 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RunAs {
    ///
    /// The group ID whose permissions are used to run the Lambda function. You can use the getent group 				 command on your core device to look up the group ID.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Gid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,

    ///
    /// The user ID whose permissions are used to run the Lambda function. You can use the getent passwd 				 command on your core device to look up the user ID.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

impl cfn_resources::CfnResource for RunAs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
