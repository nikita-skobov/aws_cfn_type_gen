/// The AWS::AppSync::Resolver resource defines the logical GraphQL resolver that you attach to     fields in a schema. Request and response templates for resolvers are written in Apache Velocity Template     Language (VTL) format. For more information about resolvers, see Resolver Mapping Template       Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResolver {
    ///
    /// The AWS AppSync GraphQL API to which you want to attach this resolver.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: cfn_resources::StrVal,

    ///
    /// The caching configuration for the resolver.
    ///
    /// Required: No
    ///
    /// Type: CachingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_config: Option<CachingConfig>,

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
    /// The resolver data source name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<cfn_resources::StrVal>,

    ///
    /// The GraphQL field on a type that invokes the resolver.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FieldName")]
    pub field_name: cfn_resources::StrVal,

    ///
    /// The resolver type.
    ///
    /// UNIT: A UNIT resolver type. A UNIT resolver is        the default resolver type. You can use a UNIT resolver to run a GraphQL query against        a single data source.                        PIPELINE: A PIPELINE resolver type. You can        use a PIPELINE resolver to invoke a series of Function objects in a        serial manner. You can use a pipeline resolver to run a GraphQL query against        multiple data sources.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<cfn_resources::StrVal>,

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
    /// Functions linked with the pipeline resolver.
    ///
    /// Required: No
    ///
    /// Type: PipelineConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,

    ///
    /// The request mapping template.
    ///
    /// Request mapping templates are optional when using a Lambda data source. For all other data sources, a     request mapping template is required.
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
    /// The location of a request mapping template in an Amazon S3 bucket. Use this if you want to     provision with a template file in Amazon S3 rather than embedding it in your CloudFormation template.
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
    /// The response mapping template.
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
    /// The SyncConfig for a resolver attached to a versioned data source.
    ///
    /// Required: No
    ///
    /// Type: SyncConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SyncConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,

    ///
    /// The GraphQL type that invokes this resolver.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TypeName")]
    pub type_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_field_name: CfnResolverfieldname,

    #[serde(skip_serializing)]
    pub att_resolver_arn: CfnResolverresolverarn,

    #[serde(skip_serializing)]
    pub att_type_name: CfnResolvertypename,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverfieldname;
impl CfnResolverfieldname {
    pub fn att_name(&self) -> &'static str {
        r#"FieldName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverresolverarn;
impl CfnResolverresolverarn {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolvertypename;
impl CfnResolvertypename {
    pub fn att_name(&self) -> &'static str {
        r#"TypeName"#
    }
}

impl cfn_resources::CfnResource for CfnResolver {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::Resolver"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.caching_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.pipeline_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
    /// The version of the runtime to use. Currently, the only allowed version is 1.0.0.
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

/// The caching configuration for a resolver that has caching activated.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CachingConfig {
    ///
    /// The caching keys for a resolver that has caching activated.
    ///
    /// Valid values are entries from the $context.arguments,       $context.source, and $context.identity maps.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachingKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_keys: Option<Vec<String>>,

    ///
    /// The TTL in seconds for a resolver that has caching activated.
    ///
    /// Valid values are 1–3,600 seconds.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ttl")]
    pub ttl: f64,
}

impl cfn_resources::CfnResource for CachingConfig {
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

/// The LambdaConflictHandlerConfig when configuring LAMBDA as the Conflict Handler.
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

/// Use the PipelineConfig property type to specify PipelineConfig for an AWS AppSync resolver.
///
/// PipelineConfig is a property of the AWS::AppSync::Resolver     resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PipelineConfig {
    ///
    /// A list of Function objects.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for PipelineConfig {
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
