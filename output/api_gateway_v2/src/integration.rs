

/// The AWS::ApiGatewayV2::Integration resource creates an integration          for an API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIntegration {


    /// 
    /// For WebSocket APIs, a key-value map specifying request parameters that are passed from the method request to the backend. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the backend. The method request parameter value must match the pattern of method.request.{location}.{name}                 , where                   {location}                  is querystring, path, or header; and                   {name}                  must be a valid and unique method request parameter name.
    /// 
    /// For HTTP API integrations with a specified integrationSubtype, request parameters are a key-value map specifying parameters that are passed to AWS_PROXY integrations. You can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see Working with AWS service integrations for HTTP APIs.
    /// 
    /// For HTTP API integrations without a specified integrationSubtype request parameters are a key-value map specifying how to transform HTTP requests before sending them to the backend. The key should follow the pattern <action>:<header|querystring|path>.<location> where action can be append, overwrite or remove. For values, you can provide static values, or map request data, stage variables, or context variables that are evaluated at runtime. To learn more, see Transforming API requests and responses.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestParameters")]
    pub request_parameters: Option<serde_json::Value>,


    /// 
    /// Specifies the integration's HTTP method type.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationMethod")]
    pub integration_method: Option<String>,


    /// 
    /// Supported only for HTTP APIs. You use response parameters to transform the HTTP response from a backend      integration before returning the response to clients. Specify a key-value map from a selection key to response      parameters. The selection key must be a valid HTTP status code within the range of 200-599. The value is of type ResponseParameterList. To learn more, see Transforming API requests and responses.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<serde_json::Value>,


    /// 
    /// The ID of the VPC link for a private integration. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionId")]
    pub connection_id: Option<String>,


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


    /// 
    /// The TLS configuration for a private integration. If you specify a TLS configuration, private integration traffic uses the HTTPS         protocol. Supported only for HTTP APIs.
    /// 
    /// Required: No
    ///
    /// Type: TlsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsConfig")]
    pub tls_config: Option<TlsConfig>,


    /// 
    /// For a Lambda integration, specify the URI of a Lambda function.
    /// 
    /// For an HTTP integration, specify a fully-qualified URL.
    /// 
    /// For an HTTP API private integration, specify the ARN of an Application Load           Balancer listener, Network Load Balancer listener, or AWS Cloud Map service. If           you specify the ARN of an AWS Cloud Map service, API Gateway uses             DiscoverInstances to identify resources. You can use query           parameters to target specific resources. To learn more, see DiscoverInstances. For private integrations, all resources must be           owned by the same AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationUri")]
    pub integration_uri: Option<String>,


    /// 
    /// Supported only for HTTP API AWS_PROXY integrations. Specifies the AWS service action to invoke. To learn more, see Integration subtype reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationSubtype")]
    pub integration_subtype: Option<String>,


    /// 
    /// Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:
    /// 
    /// CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.
    /// 
    /// CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.
    /// 
    /// If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentHandlingStrategy")]
    pub content_handling_strategy: Option<String>,


    /// 
    /// The integration type of an integration. One of the following:
    /// 
    /// AWS: for integrating the route or method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration. Supported only for WebSocket APIs.
    /// 
    /// AWS_PROXY: for integrating the route or method request with a Lambda function or other AWS service action. This integration is also referred to as a Lambda proxy integration.
    /// 
    /// HTTP: for integrating the route or method request with an HTTP endpoint. This integration is also referred to as the HTTP custom integration. Supported only for WebSocket APIs.
    /// 
    /// HTTP_PROXY: for integrating the route or method request with an           HTTP endpoint, with the           client request passed through as-is. This is also referred to as HTTP proxy           integration. For HTTP API private integrations, use an HTTP_PROXY           integration.
    /// 
    /// MOCK: for integrating the route or method request with API Gateway as a "loopback" endpoint without invoking any backend. Supported only for WebSocket APIs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationType")]
    pub integration_type: String,


    /// 
    /// Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value. Supported only for WebSocket APIs.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestTemplates")]
    pub request_templates: Option<serde_json::Value>,


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
    /// The template selection expression for the integration. Supported only for WebSocket APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateSelectionExpression")]
    pub template_selection_expression: Option<String>,


    /// 
    /// Specifies the format of the payload sent to an integration. Required for HTTP           APIs. For HTTP APIs, supported values for Lambda proxy integrations are             1.0 and 2.0. For all other integrations,             1.0 is the only supported value. To learn more, see           Working with AWS Lambda proxy integrations for HTTP APIs.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadFormatVersion")]
    pub payload_format_version: Option<String>,


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
    /// Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the requestTemplates property on the Integration resource. There are three valid values: WHEN_NO_MATCH, WHEN_NO_TEMPLATES, and NEVER. Supported only for WebSocket APIs.
    /// 
    /// WHEN_NO_MATCH passes the request body for unmapped content types through to the integration backend without transformation.
    /// 
    /// NEVER rejects unmapped content types with an HTTP 415 Unsupported Media Type response.
    /// 
    /// WHEN_NO_TEMPLATES allows pass-through when the integration has no content types mapped to templates. However, if there is at least one content type defined, unmapped content types will be rejected with the same HTTP 415 Unsupported Media Type response.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PassthroughBehavior")]
    pub passthrough_behavior: Option<String>,


    /// 
    /// The type of the network connection to the integration endpoint. Specify INTERNET for connections through the public routable internet or          VPC_LINK for private connections between API Gateway and resources in a VPC. The default value is INTERNET.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<String>,


    /// 
    /// Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::*:user/*. To use resource-based permissions on supported AWS services, don't specify this parameter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CredentialsArn")]
    pub credentials_arn: Option<String>,

}



impl cfn_resources::CfnResource for CfnIntegration {
    fn type_string() -> &'static str {
        "AWS::ApiGatewayV2::Integration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The TlsConfig property specifies the TLS configuration for a private       integration. If you specify a TLS configuration, private integration traffic uses the       HTTPS protocol. Supported only for HTTP APIs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsConfig {


    /// 
    /// If you specify a server name, API Gateway uses it to verify the hostname on           the integration's certificate. The server name is also included in the TLS           handshake to support Server Name Indication (SNI) or virtual hosting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerNameToVerify")]
    pub server_name_to_verify: Option<String>,

}




/// Specifies a list of response parameters for an HTTP API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResponseParameterList {


    /// 
    /// Supported only for HTTP APIs. You use response parameters to transform the HTTP response from a backend      integration before returning the response to clients. Specify a key-value map from a selection key to response      parameters. The selection key must be a valid HTTP status code within the range of 200-599. Response parameters are a key-value map. The key      must match the pattern <action>:<header>.<location> or       overwrite.statuscode. The action can be append, overwrite or       remove. The value can be a static value, or map to response data, stage variables, or context      variables that are evaluated at runtime. To learn more, see Transforming API requests and responses.
    /// 
    /// Required: No
    ///
    /// Type: List of ResponseParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<Vec<ResponseParameter>>,

}




/// Supported only for HTTP APIs. You use response parameters to transform the HTTP response from a backend      integration before returning the response to clients. Specify a key-value map from a selection key to response      parameters. The selection key must be a valid HTTP status code within the range of 200-599. Response parameters are a key-value map. The key      must match the pattern <action>:<header>.<location> or       overwrite.statuscode. The action can be append, overwrite or       remove. The value can be a static value, or map to response data, stage variables, or context      variables that are evaluated at runtime. To learn more, see Transforming API requests and responses.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResponseParameter {


    /// 
    /// Specifies the location of the response to modify, and how to modify it. To learn more, see Transforming API requests and responses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: String,


    /// 
    /// Specifies the data to update the parameter with. To learn more, see Transforming API requests and responses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: String,

}


