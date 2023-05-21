

/// The AWS::ApiGatewayV2::Api resource creates an API. WebSocket APIs          and HTTP APIs are supported. For more information about          WebSocket APIs, see About WebSocket APIs in API Gateway in the API Gateway             Developer Guide. For more information about HTTP APIs, see HTTP APIs in the API Gateway Developer          Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnApi {


    /// 
    /// The route selection expression for the API. For HTTP APIs, the routeSelectionExpression must be ${request.method} ${request.path}. If not provided, this will be the default for HTTP APIs. This property is required for WebSocket APIs.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteSelectionExpression")]
    pub route_selection_expression: Option<String>,


    /// 
    /// The description of the API.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The OpenAPI definition. Supported only for HTTP APIs. To import an HTTP API, you must specify a Body or BodyS3Location. If you specify        a Body or BodyS3Location, don't specify CloudFormation resources such as AWS::ApiGatewayV2::Authorizer or AWS::ApiGatewayV2::Route.        API Gateway doesn't support the combination of OpenAPI and CloudFormation resources.
    /// 
    /// Required: Conditional
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Body")]
    pub body: Option<serde_json::Value>,


    /// 
    /// Avoid validating models when creating a deployment. Supported only for WebSocket APIs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableSchemaValidation")]
    pub disable_schema_validation: Option<bool>,


    /// 
    /// Specifies how to interpret the base path of the API during import. Valid values are ignore, prepend, and          split. The default value is ignore. To learn more, see Set the OpenAPI basePath          Property. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,


    /// 
    /// An API key selection expression. Supported only for WebSocket APIs. See API Key Selection Expressions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeySelectionExpression")]
    pub api_key_selection_expression: Option<String>,


    /// 
    /// Specifies whether clients can invoke your API by using the default             execute-api endpoint. By default, clients can invoke your API           with the default https://{api_id}.execute-api.{region}.amazonaws.com endpoint.           To require that clients use a custom domain name to invoke your API, disable the           default endpoint.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableExecuteApiEndpoint")]
    pub disable_execute_api_endpoint: Option<bool>,


    /// 
    /// Specifies whether to rollback the API creation when a warning is encountered. By default, API creation continues if a warning is encountered.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailOnWarnings")]
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
    pub name: Option<String>,


    /// 
    /// The API protocol. Valid values are WEBSOCKET or HTTP. Required unless you specify an OpenAPI definition for Body or S3BodyLocation.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<String>,


    /// 
    /// The S3 location of an OpenAPI definition. Supported only for HTTP APIs. To import an HTTP API, you must specify a Body or BodyS3Location. If you specify      a Body or BodyS3Location, don't specify CloudFormation resources such as AWS::ApiGatewayV2::Authorizer or AWS::ApiGatewayV2::Route.      API Gateway doesn't support the combination of OpenAPI and CloudFormation resources.
    /// 
    /// Required: Conditional
    ///
    /// Type: BodyS3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "BodyS3Location")]
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
    pub cors_configuration: Option<Cors>,


    /// 
    /// A version identifier for the API.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The collection of tags. Each tag element is associated with a given resource.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// This property is part of quick create. If you don't specify a          routeKey, a default route of $default is created. The          $default route acts as a catch-all for any request made to your API,        for a particular stage. The $default route key can't be modified. You        can add routes after creating the API, and you can update the route keys of        additional routes. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteKey")]
    pub route_key: Option<String>,


    /// 
    /// This property is part of quick create. It specifies the credentials required for        the integration, if any. For a Lambda integration, three options are available. To        specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name        (ARN). To require that the caller's identity be passed through from the request,        specify arn:aws:iam::*:user/*. To use resource-based permissions on        supported AWS services, specify null. Currently, this property is not used for HTTP        integrations. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialsArn")]
    pub credentials_arn: Option<String>,


    /// 
    /// This property is part of quick create. Quick create produces an API with an        integration, a default catch-all route, and a default stage which is configured to        automatically deploy changes. For HTTP integrations, specify a fully qualified URL.        For Lambda integrations, specify a function ARN. The type of the integration will be        HTTP_PROXY or AWS_PROXY, respectively. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Option<String>,

}


/// The Cors property specifies a CORS configuration for an API.          Supported only for HTTP APIs. See Configuring CORS for more information.
#[derive(Default, serde::Serialize)]
pub struct Cors {


    /// 
    /// The number of seconds that the browser should cache preflight request results. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxAge")]
    pub max_age: Option<i64>,


    /// 
    /// Represents a collection of allowed origins. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowOrigins")]
    pub allow_origins: Option<Vec<String>>,


    /// 
    /// Specifies whether credentials are included in the CORS request. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowCredentials")]
    pub allow_credentials: Option<bool>,


    /// 
    /// Represents a collection of exposed headers. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExposeHeaders")]
    pub expose_headers: Option<Vec<String>>,


    /// 
    /// Represents a collection of allowed HTTP methods. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowMethods")]
    pub allow_methods: Option<Vec<String>>,


    /// 
    /// Represents a collection of allowed headers. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowHeaders")]
    pub allow_headers: Option<Vec<String>>,

}


/// The BodyS3Location property specifies an S3 location from which to          import an OpenAPI definition. Supported only for HTTP APIs.
#[derive(Default, serde::Serialize)]
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
    pub bucket: Option<String>,


    /// 
    /// The version of the S3 object.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// The Etag of the S3 object.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Etag")]
    pub etag: Option<String>,


    /// 
    /// The key of the S3 object. Required if you specify a BodyS3Location for an API.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,

}