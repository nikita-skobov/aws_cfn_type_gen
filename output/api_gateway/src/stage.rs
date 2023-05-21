/// The AWS::ApiGateway::Stage resource creates a stage for a deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStage {
    ///
    /// Access log settings, including the access log format and access log destination ARN.
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
    /// The stage's cache capacity in GB. For more information about choosing a cache size, see Enabling API caching to enhance responsiveness.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: 0.5 | 1.6 | 118 | 13.5 | 237 | 28.4 | 58.2 | 6.1
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheClusterSize")]
    pub cache_cluster_size: Option<StageCacheClusterSizeEnum>,

    ///
    /// Settings for the canary deployment in this stage.
    ///
    /// Required: No
    ///
    /// Type: CanarySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "CanarySetting")]
    pub canary_setting: Option<CanarySetting>,

    ///
    /// The identifier of a client certificate for an API stage.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,

    ///
    /// The identifier of the Deployment that the stage points to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,

    ///
    /// The stage's description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The version of the associated API documentation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: Option<String>,

    ///
    /// A map that defines the method settings for a Stage resource. Keys (designated as /{method_setting_key below) are method paths defined as {resource_path}/{http_method} for an individual method override, or /\*/\* for overriding all methods in the stage.
    ///
    /// Required: No
    ///
    /// Type: List of MethodSetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "MethodSettings")]
    pub method_settings: Option<Vec<MethodSetting>>,

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
    /// The name of the stage is the first path segment in the Uniform Resource Identifier (URI) of a call to API Gateway. Stage names can only contain alphanumeric characters, hyphens, and underscores. Maximum length is 128 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StageName")]
    pub stage_name: Option<String>,

    ///
    /// The collection of tags. Each tag element is associated with a given resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// Specifies whether active tracing with X-ray is enabled for the Stage.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "TracingEnabled")]
    pub tracing_enabled: Option<bool>,

    ///
    /// A map (string-to-string map) that defines the stage variables, where the variable name is the key and the variable value is the value. Variable names are limited to alphanumeric characters. Values must match the following regular expression: [A-Za-z0-9-._~:/?#&=,]+.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variables")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum StageCacheClusterSizeEnum {
    /// 0.5
    #[serde(rename = "0.5")]
    E05,

    /// 1.6
    #[serde(rename = "1.6")]
    E16,

    /// 118
    #[serde(rename = "118")]
    E118,

    /// 13.5
    #[serde(rename = "13.5")]
    E135,

    /// 237
    #[serde(rename = "237")]
    E237,

    /// 28.4
    #[serde(rename = "28.4")]
    E284,

    /// 58.2
    #[serde(rename = "58.2")]
    E582,

    /// 6.1
    #[serde(rename = "6.1")]
    E61,
}

impl Default for StageCacheClusterSizeEnum {
    fn default() -> Self {
        StageCacheClusterSizeEnum::E05
    }
}

impl cfn_resources::CfnResource for CfnStage {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Stage"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_log_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.canary_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AccessLogSetting property type specifies settings for logging access in this stage.
///
/// AccessLogSetting is a property of the AWS::ApiGateway::Stage resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLogSetting {
    ///
    /// The Amazon Resource Name (ARN) of the CloudWatch Logs log group or Kinesis Data Firehose delivery stream to receive access logs. If you specify a Kinesis Data Firehose delivery stream, the stream name must begin with amazon-apigateway-. This parameter is required to enable access logging.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

    ///
    /// A single line format of the access logs of data, as specified by selected $context variables. The format must include at least $context.requestId. This parameter is required to enable access logging.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<String>,
}

impl cfn_resources::CfnResource for AccessLogSetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Configuration settings of a canary deployment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CanarySetting {
    ///
    /// The ID of the canary deployment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,

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

impl cfn_resources::CfnResource for CanarySetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The MethodSetting property type configures settings for all methods in a stage.
///
/// The MethodSettings property of the AWS::ApiGateway::Stage resource contains a list of MethodSetting property types.
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
    /// The HTTP method. To apply settings to multiple resources and methods, specify an asterisk (*) for the HttpMethod and /* for the ResourcePath. This parameter is required when you specify a MethodSetting.
    ///
    /// Required: Conditional
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
    /// The resource path for this method. Forward slashes (/) are encoded as ~1 and the initial slash must include a forward slash. For example, the path value /resource/subresource must be encoded as /~1resource~1subresource. To specify the root path, use only a slash (/). To apply settings to multiple resources and methods, specify an asterisk (*) for the HttpMethod and /* for the ResourcePath.      This parameter is required when you specify a MethodSetting.
    ///
    /// Required: Conditional
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

impl cfn_resources::CfnResource for MethodSetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
