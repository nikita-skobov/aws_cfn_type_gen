

/// The AWS::ApiGatewayV2::Stage resource specifies a stage for an API. Each stage is a named     reference to a deployment of the API and is made available for client applications to call. To learn more, see      Working with stages for      HTTP APIs and Deploy a WebSocket API in API Gateway.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStage {


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
    /// The description for the API stage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


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
    /// The collection of tags. Each tag element is associated with a given resource.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,

}



impl cfn_resources::CfnResource for CfnStage {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::Stage"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.access_log_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.default_route_settings.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for logging access in a stage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLogSettings {


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



impl cfn_resources::CfnResource for AccessLogSettings {
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

/// Represents a collection of route settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RouteSettings {


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



impl cfn_resources::CfnResource for RouteSettings {
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