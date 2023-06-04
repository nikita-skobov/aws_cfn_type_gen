/// The AWS::ApiGatewayV2::Api resource creates an API. WebSocket APIs          and HTTP APIs are supported. For more information about          WebSocket APIs, see About WebSocket APIs in API Gateway in the API Gateway             Developer Guide. For more information about HTTP APIs, see HTTP APIs in the API Gateway Developer          Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApi {
    ///
    /// An API key selection expression. Supported only for WebSocket APIs. See API Key Selection Expressions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<cfn_resources::StrVal>,

    ///
    /// Specifies how to interpret the base path of the API during import. Valid values are ignore, prepend, and          split. The default value is ignore. To learn more, see Set the OpenAPI basePath          Property. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<cfn_resources::StrVal>,

    ///
    /// The OpenAPI definition. Supported only for HTTP APIs. To import an HTTP API, you must specify a Body or BodyS3Location. If you specify        a Body or BodyS3Location, don't specify CloudFormation resources such as AWS::ApiGatewayV2::Authorizer or AWS::ApiGatewayV2::Route.        API Gateway doesn't support the combination of OpenAPI and CloudFormation resources.
    ///
    /// Required: Conditional
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,

    ///
    /// The S3 location of an OpenAPI definition. Supported only for HTTP APIs. To import an HTTP API, you must specify a Body or BodyS3Location. If you specify      a Body or BodyS3Location, don't specify CloudFormation resources such as AWS::ApiGatewayV2::Authorizer or AWS::ApiGatewayV2::Route.      API Gateway doesn't support the combination of OpenAPI and CloudFormation resources.
    ///
    /// Required: Conditional
    ///
    /// Type: BodyS3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyS3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_s3_location: Option<BodyS3Location>,

    ///
    /// A CORS configuration. Supported only for HTTP APIs. See Configuring CORS for more information.
    ///
    /// Required: No
    ///
    /// Type: Cors
    ///
    /// Update requires: No interruption
    #[serde(rename = "CorsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<Cors>,

    ///
    /// This property is part of quick create. It specifies the credentials required for        the integration, if any. For a Lambda integration, three options are available. To        specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name        (ARN). To require that the caller's identity be passed through from the request,        specify arn:aws:iam::*:user/*. To use resource-based permissions on        supported AWS services, specify null. Currently, this property is not used for HTTP        integrations. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialsArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<cfn_resources::StrVal>,

    ///
    /// The description of the API.
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
    /// Specifies whether clients can invoke your API by using the default             execute-api endpoint. By default, clients can invoke your API           with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint.           To require that clients use a custom domain name to invoke your API, disable the           default endpoint.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableExecuteApiEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_execute_api_endpoint: Option<bool>,

    ///
    /// Avoid validating models when creating a deployment. Supported only for WebSocket APIs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableSchemaValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_schema_validation: Option<bool>,

    ///
    /// Specifies whether to rollback the API creation when a warning is encountered. By default, API creation continues if a warning is encountered.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,

    ///
    /// The name of the API. Required unless you specify an OpenAPI definition for Body or S3BodyLocation.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The API protocol. Valid values are WEBSOCKET or HTTP. Required unless you specify an OpenAPI definition for Body or S3BodyLocation.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<cfn_resources::StrVal>,

    ///
    /// This property is part of quick create. If you don't specify a          routeKey, a default route of $default is created. The          $default route acts as a catch-all for any request made to your API,        for a particular stage. The $default route key can't be modified. You        can add routes after creating the API, and you can update the route keys of        additional routes. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_key: Option<cfn_resources::StrVal>,

    ///
    /// The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<cfn_resources::StrVal>,

    ///
    /// The collection of tags. Each tag element is associated with a given resource.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// This property is part of quick create. Quick create produces an API with an        integration, a default catch-all route, and a default stage which is configured to        automatically deploy changes. For HTTP integrations, specify a fully qualified URL.        For Lambda integrations, specify a function ARN. The type of the integration will be        HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<cfn_resources::StrVal>,

    ///
    /// A version identifier for the API.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_api_endpoint: CfnApiapiendpoint,

    #[serde(skip_serializing)]
    pub att_api_id: CfnApiapiid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApiapiendpoint;
impl CfnApiapiendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"ApiEndpoint"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnApiapiid;
impl CfnApiapiid {
    pub fn att_name(&self) -> &'static str {
        r#"ApiId"#
    }
}

impl cfn_resources::CfnResource for CfnApi {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::Api"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.body_s3_location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cors_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The BodyS3Location property specifies an S3 location from which to          import an OpenAPI definition. Supported only for HTTP APIs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BodyS3Location {
    ///
    /// The S3 bucket that contains the OpenAPI definition to import. Required if you specify a BodyS3Location for an API.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<cfn_resources::StrVal>,

    ///
    /// The Etag of the S3 object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Etag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<cfn_resources::StrVal>,

    ///
    /// The key of the S3 object. Required if you specify a BodyS3Location for an API.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The version of the S3 object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BodyS3Location {
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

/// The Cors property specifies a CORS configuration for an API.          Supported only for HTTP APIs. See Configuring CORS for more information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Cors {
    ///
    /// Specifies whether credentials are included in the CORS request. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,

    ///
    /// Represents a collection of allowed headers. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,

    ///
    /// Represents a collection of allowed HTTP methods. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,

    ///
    /// Represents a collection of allowed origins. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowOrigins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,

    ///
    /// Represents a collection of exposed headers. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExposeHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,

    ///
    /// The number of seconds that the browser should cache preflight request results. Supported only for HTTP APIs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
}

impl cfn_resources::CfnResource for Cors {
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
