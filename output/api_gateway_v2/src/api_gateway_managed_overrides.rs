/// The AWS::ApiGatewayV2::ApiGatewayManagedOverrides resource overrides the       default properties of API Gateway-managed resources that are implicitly configured for       you when you use quick create. When you create an API by using quick create, an         AWS::ApiGatewayV2::Route, AWS::ApiGatewayV2::Integration,       and AWS::ApiGatewayV2::Stage are created for you and associated with your         AWS::ApiGatewayV2::Api. The         AWS::ApiGatewayV2::ApiGatewayManagedOverrides resource enables you to       set, or override the properties of these implicit resources. Supported only for HTTP       APIs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApiGatewayManagedOverrides {
    ///
    /// The ID of the API for which to override the configuration of API Gateway-managed resources.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiId")]
    pub api_id: String,

    ///
    /// Overrides the integration configuration for an API Gateway-managed integration.
    ///
    /// Required: No
    ///
    /// Type: IntegrationOverrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "Integration")]
    pub integration: Option<IntegrationOverrides>,

    ///
    /// Overrides the route configuration for an API Gateway-managed route.
    ///
    /// Required: No
    ///
    /// Type: RouteOverrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "Route")]
    pub route: Option<RouteOverrides>,

    ///
    /// Overrides the stage configuration for an API Gateway-managed stage.
    ///
    /// Required: No
    ///
    /// Type: StageOverrides
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    pub stage: Option<StageOverrides>,
}

impl cfn_resources::CfnResource for CfnApiGatewayManagedOverrides {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::ApiGatewayManagedOverrides"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.integration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.route.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.stage.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AccessLogSettings property overrides the access log settings for an API Gateway-managed stage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLogSettings {
    ///
    /// The ARN of the CloudWatch Logs log group to receive access logs.
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

/// The IntegrationOverrides property overrides the integration settings for       an API Gateway-managed integration. If you remove this property, API Gateway restores the default values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IntegrationOverrides {
    ///
    /// The description of the integration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// Specifies the integration's HTTP method type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationMethod")]
    pub integration_method: Option<String>,

    ///
    /// Specifies the format of the payload sent to an integration. Required for HTTP           APIs. For HTTP APIs, supported values for Lambda proxy integrations are             1.0 and 2.0. For all other integrations,             1.0 is the only supported value. To learn more, see           Working with AWS Lambda proxy integrations for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadFormatVersion")]
    pub payload_format_version: Option<String>,

    ///
    /// Custom timeout between 50 and 29,000 milliseconds for WebSocket APIs and           between 50 and 30,000 milliseconds for HTTP APIs. The default timeout is 29           seconds for WebSocket APIs and 30 seconds for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMillis")]
    pub timeout_in_millis: Option<i64>,
}

impl cfn_resources::CfnResource for IntegrationOverrides {
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

/// The RouteOverrides property overrides the route configuration for an API       Gateway-managed route. If you remove this property, API Gateway restores the default values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RouteOverrides {
    ///
    /// The authorization scopes supported by this route.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationScopes")]
    pub authorization_scopes: Option<Vec<String>>,

    ///
    /// The authorization type for the route. To learn more, see AuthorizationType.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: Option<String>,

    ///
    /// The identifier of the Authorizer resource to be associated with this route. The authorizer identifier is generated by API Gateway when you created the authorizer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerId")]
    pub authorizer_id: Option<String>,

    ///
    /// The operation name for the route.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperationName")]
    pub operation_name: Option<String>,

    ///
    /// For HTTP integrations, specify a fully qualified URL.        For Lambda integrations, specify a function ARN. The type of the integration will be        HTTP_PROXY or AWS_PROXY, respectively.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Option<String>,
}

impl cfn_resources::CfnResource for RouteOverrides {
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

/// The RouteSettings property overrides the route settings for an API Gateway-managed route.
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

/// The StageOverrides property overrides the stage configuration for an API       Gateway-managed stage. If you remove this property, API Gateway restores the default values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StageOverrides {
    ///
    /// Settings for logging access in a stage.
    ///
    /// Required: No
    ///
    /// Type: AccessLogSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLogSettings")]
    pub access_log_settings: Option<AccessLogSettings>,

    ///
    /// Specifies whether updates to an API automatically trigger a new deployment. The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoDeploy")]
    pub auto_deploy: Option<bool>,

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
    /// A map that defines the stage variables for a Stage. Variable names can have alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&=,]+.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageVariables")]
    pub stage_variables: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for StageOverrides {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_log_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_route_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
