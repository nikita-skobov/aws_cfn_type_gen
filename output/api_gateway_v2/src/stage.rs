

/// The AWS::ApiGatewayV2::Stage resource specifies a stage for an API. Each stage is a named     reference to a deployment of the API and is made available for client applications to call. To learn more, see      Working with stages for      HTTP APIs and Deploy a WebSocket API in API Gateway.
#[derive(Default, serde::Serialize)]
pub struct CfnStage {


    /// 
    /// This parameter is not currently supported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicyId")]
    pub access_policy_id: Option<String>,


    /// 
    /// Route settings for the stage.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteSettings")]
    pub route_settings: Option<serde_json::Value>,


    /// 
    /// The API identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,


    /// 
    /// The deployment identifier for the API stage. Can't be updated if autoDeploy is enabled.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeploymentId")]
    pub deployment_id: Option<String>,


    /// 
    /// Specifies whether updates to an API automatically trigger a new deployment. The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoDeploy")]
    pub auto_deploy: Option<bool>,


    /// 
    /// A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&=,]+.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageVariables")]
    pub stage_variables: Option<serde_json::Value>,


    /// 
    /// The identifier of a client certificate for a Stage. Supported only for WebSocket APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientCertificateId")]
    pub client_certificate_id: Option<String>,


    /// 
    /// The stage name. Stage names can contain only alphanumeric characters, hyphens, and underscores, or be $default. Maximum length is 128 characters.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StageName")]
    pub stage_name: String,


    /// 
    /// Settings for logging access in this stage.
    /// 
    /// Required: No
    ///
    /// Type: AccessLogSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLogSettings")]
    pub access_log_settings: Option<AccessLogSettings>,


    /// 
    /// The collection of tags. Each tag element is associated with a given resource.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


    /// 
    /// The default route settings for the stage.
    /// 
    /// Required: No
    ///
    /// Type: RouteSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRouteSettings")]
    pub default_route_settings: Option<RouteSettings>,


    /// 
    /// The description for the API stage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// Settings for logging access in a stage.
#[derive(Default, serde::Serialize)]
pub struct AccessLogSettings {


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


    /// 
    /// The ARN of the CloudWatch Logs log group to receive access logs. This parameter is required to enable access logging.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}


/// Represents a collection of route settings.
#[derive(Default, serde::Serialize)]
pub struct RouteSettings {


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


    /// 
    /// Specifies the logging level for this route: INFO, ERROR, or OFF. This property affects the log entries pushed to Amazon CloudWatch Logs. Supported only for WebSocket APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingLevel")]
    pub logging_level: Option<String>,


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
    /// Specifies whether detailed metrics are enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailedMetricsEnabled")]
    pub detailed_metrics_enabled: Option<bool>,


    /// 
    /// Specifies whether (true) or not (false) data trace logging is enabled for this route. This property affects the log entries pushed to Amazon CloudWatch Logs. Supported only for WebSocket APIs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,

}