

/// The AWS::ApiGateway::Deployment resource deploys an API Gateway RestApi resource to a stage so that clients can call the API over the internet. The stage acts as an environment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeployment {


    /// 
    /// The input configuration for a canary deployment.
    /// 
    /// Required: No
    ///
    /// Type: DeploymentCanarySettings
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeploymentCanarySettings")]
    pub deployment_canary_settings: Option<DeploymentCanarySettings>,


    /// 
    /// The description for the Deployment resource to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The string identifier of the associated RestApi.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RestApiId")]
    pub rest_api_id: String,


    /// 
    /// The description of the Stage resource for the Deployment resource to create. To specify a stage description, you must also provide a stage name.
    /// 
    /// Required: Conditional
    ///
    /// Type: StageDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageDescription")]
    pub stage_description: Option<StageDescription>,


    /// 
    /// The name of the Stage resource for the Deployment resource to create.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnDeployment {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::Deployment"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AccessLogSetting property type specifies settings for logging access in this stage.
///
/// AccessLogSetting is a property of the StageDescription property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLogSetting {


    /// 
    /// The Amazon Resource Name (ARN) of the CloudWatch Logs log group or Kinesis Data Firehose delivery stream to receive access logs. If you specify a Kinesis Data Firehose delivery stream, the stream name must begin with amazon-apigateway-.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,


    /// 
    /// A single line format of the access logs of data, as specified by selected $context variables. The format must include at least $context.requestId.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<String>,

}




/// The CanarySetting property type specifies settings for the canary deployment in this stage.
///
/// CanarySetting is a property of the StageDescription property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CanarySetting {


    /// 
    /// The percent (0-100) of traffic diverted to a canary deployment.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PercentTraffic")]
    pub percent_traffic: Option<f64>,


    /// 
    /// Stage variables overridden for a canary release deployment, including new stage variables introduced in the canary. These stage variables are represented as a string-to-string map between stage variable names and their values.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageVariableOverrides")]
    pub stage_variable_overrides: Option<std::collections::HashMap<String, String>>,


    /// 
    /// A Boolean flag to indicate whether the canary deployment uses the stage cache or not.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseStageCache")]
    pub use_stage_cache: Option<bool>,

}




/// The DeploymentCanarySettings property type specifies settings for the canary deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeploymentCanarySettings {


    /// 
    /// The percentage (0.0-100.0) of traffic routed to the canary deployment.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "PercentTraffic")]
    pub percent_traffic: Option<f64>,


    /// 
    /// A stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StageVariableOverrides")]
    pub stage_variable_overrides: Option<std::collections::HashMap<String, String>>,


    /// 
    /// A Boolean flag to indicate whether the canary release deployment uses the stage cache or not.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "UseStageCache")]
    pub use_stage_cache: Option<bool>,

}




/// The MethodSetting property type configures settings for all methods in a stage.
///
/// The MethodSettings property of the Amazon API Gateway Deployment StageDescription property type contains a list of MethodSetting property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MethodSetting {


    /// 
    /// Specifies whether the cached responses are encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheDataEncrypted")]
    pub cache_data_encrypted: Option<bool>,


    /// 
    /// Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheTtlInSeconds")]
    pub cache_ttl_in_seconds: Option<i64>,


    /// 
    /// Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachingEnabled")]
    pub caching_enabled: Option<bool>,


    /// 
    /// Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,


    /// 
    /// The HTTP method.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpMethod")]
    pub http_method: Option<String>,


    /// 
    /// Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. Valid values are OFF, ERROR, and INFO. Choose ERROR to write only error-level entries to CloudWatch Logs, or choose INFO to include all ERROR events as well as extra informational events.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,


    /// 
    /// Specifies whether Amazon CloudWatch metrics are enabled for this method.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: Option<bool>,


    /// 
    /// The resource path for this method. Forward slashes (/) are encoded as ~1 and the initial slash must include a forward slash. For example, the path value /resource/subresource must be encoded as /~1resource~1subresource. To specify the root path, use only a slash (/).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcePath")]
    pub resource_path: Option<String>,


    /// 
    /// Specifies the throttling burst limit.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<i64>,


    /// 
    /// Specifies the throttling rate limit.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,

}




/// StageDescription is a property of the AWS::ApiGateway::Deployment resource that configures a deployment stage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StageDescription {


    /// 
    /// Specifies settings for logging access in this stage.
    /// 
    /// Required: No
    ///
    /// Type: AccessLogSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLogSetting")]
    pub access_log_setting: Option<AccessLogSetting>,


    /// 
    /// Specifies whether a cache cluster is enabled for the stage.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheClusterEnabled")]
    pub cache_cluster_enabled: Option<bool>,


    /// 
    /// The size of the stage's cache cluster. For more information, see cacheClusterSize in the API Gateway API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheClusterSize")]
    pub cache_cluster_size: Option<String>,


    /// 
    /// Indicates whether the cached responses are encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheDataEncrypted")]
    pub cache_data_encrypted: Option<bool>,


    /// 
    /// The time-to-live (TTL) period, in seconds, that specifies how long API Gateway caches responses.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheTtlInSeconds")]
    pub cache_ttl_in_seconds: Option<i64>,


    /// 
    /// Indicates whether responses are cached and returned for requests. You must enable a cache cluster on the stage to cache responses. For more information, see Enable API Gateway Caching in a Stage to Enhance API Performance in the API Gateway Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CachingEnabled")]
    pub caching_enabled: Option<bool>,


    /// 
    /// Specifies settings for the canary deployment in this stage.
    /// 
    /// Required: No
    ///
    /// Type: CanarySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanarySetting")]
    pub canary_setting: Option<CanarySetting>,


    /// 
    /// The identifier of the client certificate that API Gateway uses to call your integration endpoints in the stage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,


    /// 
    /// Indicates whether data trace logging is enabled for methods in the stage. API Gateway pushes these logs to Amazon CloudWatch Logs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,


    /// 
    /// A description of the purpose of the stage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The version identifier of the API documentation snapshot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: Option<String>,


    /// 
    /// The logging level for this method. For valid values, see the loggingLevel property of the MethodSetting resource in the Amazon API Gateway API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,


    /// 
    /// Configures settings for all of the stage's methods.
    /// 
    /// Required: No
    ///
    /// Type: List of MethodSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "MethodSettings")]
    pub method_settings: Option<Vec<MethodSetting>>,


    /// 
    /// Indicates whether Amazon CloudWatch metrics are enabled for methods in the stage.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: Option<bool>,


    /// 
    /// An array of arbitrary tags (key-value pairs) to associate with the stage.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The target request burst rate limit. This allows more requests through for a period of time than the target rate limit. For more information, see Manage API Request Throttling in the API Gateway Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThrottlingBurstLimit")]
    pub throttling_burst_limit: Option<i64>,


    /// 
    /// The target request steady-state rate limit. For more information, see Manage API Request Throttling in the API Gateway Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThrottlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,


    /// 
    /// Specifies whether active tracing with X-ray is enabled for this stage.
    /// 
    /// For more information, see Trace API Gateway API Execution with AWS X-Ray in the API Gateway Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TracingEnabled")]
    pub tracing_enabled: Option<bool>,


    /// 
    /// A map that defines the stage variables. Variable names must consist of alphanumeric characters, and the values must match the following regular expression: [A-Za-z0-9-._~:/?#&=,]+.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    pub variables: Option<std::collections::HashMap<String, String>>,

}




/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


