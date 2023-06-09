/// The AWS::AppSync::FunctionConfiguration resource defines the functions in GraphQL APIs to     perform certain operations. You can use pipeline resolvers to attach functions. For more information, see       Pipeline Resolvers     in the AWS AppSync Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnFunctionConfiguration {
    ///
    /// The AWS AppSync GraphQL API that you want to attach using this function.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// The resolver code that contains the request and response functions. When code is used, the       runtime is required. The runtime value must be APPSYNC_JS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon S3 endpoint.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CodeS3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_s3_location: Option<cfn_resources::StrVal>,

    ///
    /// The name of data source this function will attach.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceName")]
    pub data_source_name: cfn_resources::StrVal,

    ///
    /// The Function description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The version of the request mapping template. Currently, only the 2018-05-29 version of     the template is supported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<cfn_resources::StrVal>,

    ///
    /// The maximum number of resolver request inputs that will be sent to a single AWS Lambda     function in a BatchInvoke operation.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxBatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i64>,

    ///
    /// The name of the function.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The Function request mapping template. Functions support only the     2018-05-29 version of the request mapping template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<cfn_resources::StrVal>,

    ///
    /// Describes a Sync configuration for a resolver.
    ///
    /// Contains information on which Conflict Detection, as well as Resolution strategy, should be performed when     the resolver is invoked.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestMappingTemplateS3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template_s3_location: Option<cfn_resources::StrVal>,

    ///
    /// The Function response mapping template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<cfn_resources::StrVal>,

    ///
    /// The location of a response mapping template in an Amazon S3 bucket. Use this if you want to     provision with a template file in Amazon S3 rather than embedding it in your CloudFormation template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseMappingTemplateS3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template_s3_location: Option<cfn_resources::StrVal>,

    ///
    /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync     function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must     also be specified.
    ///
    /// Required: No
    ///
    /// Type: AppSyncRuntime
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,

    ///
    /// Describes a Sync configuration for a resolver.
    ///
    /// Specifies which Conflict Detection strategy and Resolution strategy to use when the     resolver is invoked.
    ///
    /// Required: No
    ///
    /// Type: SyncConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,

    #[serde(skip_serializing)]
    pub att_data_source_name: CfnFunctionConfigurationdatasourcename,

    #[serde(skip_serializing)]
    pub att_function_arn: CfnFunctionConfigurationfunctionarn,

    #[serde(skip_serializing)]
    pub att_function_id: CfnFunctionConfigurationfunctionid,

    #[serde(skip_serializing)]
    pub att_name: CfnFunctionConfigurationname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionConfigurationdatasourcename;
impl CfnFunctionConfigurationdatasourcename {
    pub fn att_name(&self) -> &'static str {
        r#"DataSourceName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionConfigurationfunctionarn;
impl CfnFunctionConfigurationfunctionarn {
    pub fn att_name(&self) -> &'static str {
        r#"FunctionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionConfigurationfunctionid;
impl CfnFunctionConfigurationfunctionid {
    pub fn att_name(&self) -> &'static str {
        r#"FunctionId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFunctionConfigurationname;
impl CfnFunctionConfigurationname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnFunctionConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::FunctionConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.runtime.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sync_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync     function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must     also be specified.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AppSyncRuntime {
    ///
    /// The name of the runtime to use. Currently, the only allowed value is     APPSYNC_JS.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The version of the runtime to use. Currently, the only allowed version is     1.0.0.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuntimeVersion")]
    pub runtime_version: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AppSyncRuntime {
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

/// The LambdaConflictHandlerConfig object when configuring LAMBDA     as the Conflict Handler.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LambdaConflictHandlerConfig {
    ///
    /// The Amazon Resource Name (ARN) for the Lambda function to use as the     Conflict Handler.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaConflictHandlerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_conflict_handler_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaConflictHandlerConfig {
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

/// Describes a Sync configuration for a resolver.
///
/// Specifies which Conflict Detection strategy and Resolution strategy to use when the     resolver is invoked.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SyncConfig {
    ///
    /// The Conflict Detection strategy to use.
    ///
    /// VERSION: Detect conflicts based on object        versions for this resolver.                        NONE: Do not detect conflicts when invoking        this resolver.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConflictDetection")]
    pub conflict_detection: cfn_resources::StrVal,

    ///
    /// The Conflict Resolution strategy to perform in the event of a conflict.
    ///
    /// OPTIMISTIC_CONCURRENCY: Resolve conflicts by        rejecting mutations when versions don't match the latest version at the        server.                        AUTOMERGE: Resolve conflicts with the        Automerge conflict resolution strategy.                        LAMBDA: Resolve conflicts with an AWS Lambda function supplied in the        LambdaConflictHandlerConfig.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConflictHandler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_handler: Option<cfn_resources::StrVal>,

    ///
    /// The LambdaConflictHandlerConfig when configuring LAMBDA as the     Conflict Handler.
    ///
    /// Required: No
    ///
    /// Type: LambdaConflictHandlerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaConflictHandlerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_conflict_handler_config: Option<LambdaConflictHandlerConfig>,
}

impl cfn_resources::CfnResource for SyncConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda_conflict_handler_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
