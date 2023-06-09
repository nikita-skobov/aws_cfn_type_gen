/// The AWS::ApiGateway::Method resource creates API Gateway methods that define the parameters and body that clients must send in their requests.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMethod {
    ///
    /// A boolean flag specifying whether a valid ApiKey is required to invoke this method.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,

    ///
    /// A list of authorization scopes configured on the method. The scopes are used with a COGNITO_USER_POOLS authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,

    ///
    /// The method's authorization type. This parameter is required. For valid values, see Method in the API Gateway API Reference.
    ///
    /// NoteIf you specify the AuthorizerId property, specify CUSTOM or COGNITO_USER_POOLS for this property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<cfn_resources::StrVal>,

    ///
    /// The identifier of an authorizer to use on this method. The method's authorization type must be CUSTOM or COGNITO_USER_POOLS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<cfn_resources::StrVal>,

    ///
    /// The method's HTTP verb.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpMethod")]
    pub http_method: cfn_resources::StrVal,

    ///
    /// Represents an HTTP, HTTP_PROXY, AWS, AWS_PROXY, or Mock integration.
    ///
    /// Required: No
    ///
    /// Type: Integration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Integration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<Integration>,

    ///
    /// Gets a method response associated with a given HTTP status code.
    ///
    /// Required: No
    ///
    /// Type: List of MethodResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "MethodResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_responses: Option<Vec<MethodResponse>>,

    ///
    /// A human-friendly operation identifier for the method. For example, you can assign the operationName of ListPets for the GET /pets method in the PetStore example.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<cfn_resources::StrVal>,

    ///
    /// A key-value map specifying data schemas, represented by Model resources, (as the mapped value) of the request payloads of given content types (as the mapping key).
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<std::collections::HashMap<String, String>>,

    ///
    /// A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key is a method request parameter name matching the pattern of method.request.{location}.{name}, where location is querystring, path, or header and name is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (true) or optional (false). The method request parameter names defined here are available in Integration to be mapped to integration request parameters or templates.
    ///
    /// Required: No
    ///
    /// Type: Map of Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, bool>>,

    ///
    /// The identifier of a RequestValidator for request validation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestValidatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<cfn_resources::StrVal>,

    ///
    /// The Resource identifier for the MethodResponse resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: cfn_resources::StrVal,

    ///
    /// The string identifier of the associated RestApi.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestApiId")]
    pub rest_api_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnMethod {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Method"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.integration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Integration is a property of the AWS::ApiGateway::Method resource that specifies information about the target backend that a method calls.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Integration {
    ///
    /// A list of request parameters whose values API Gateway caches. To be valid values for cacheKeyParameters, these parameters must also be specified for Method requestParameters.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheKeyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,

    ///
    /// Specifies a group of related cached parameters. By default, API Gateway uses the resource ID as the cacheNamespace. You can specify the same cacheNamespace across resources to return the same cached data for requests to different resources.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the VpcLink used for the integration when connectionType=VPC_LINK and undefined, otherwise.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<cfn_resources::StrVal>,

    ///
    /// The type of the network connection to the integration endpoint. The valid value is INTERNET for connections through the public routable internet or VPC_LINK for private connections between API Gateway and a network load balancer in a VPC. The default value is INTERNET.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: INTERNET | VPC_LINK
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<IntegrationConnectionTypeEnum>,

    ///
    /// Specifies how to handle request payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:
    ///
    /// If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the passthroughBehavior is configured to support payload pass-through.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONVERT_TO_BINARY | CONVERT_TO_TEXT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<IntegrationContentHandlingEnum>,

    ///
    /// Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string arn:aws:iam::\*:user/\*. To use resource-based permissions on supported AWS services, specify null.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the integration's HTTP method type. For the Type property, if you specify MOCK, this property is optional. For Lambda integrations, you must set the integration method to POST. For all other types, you must specify this property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationHttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_http_method: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the integration's responses.
    ///
    /// Required: No
    ///
    /// Type: List of IntegrationResponse
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_responses: Option<Vec<IntegrationResponse>>,

    ///
    /// Specifies how the method request body of an unmapped content type will be passed through    the integration request to the back end without transformation. A content type is unmapped if    no mapping template is defined in the integration or the content type does not match any of    the mapped content types, as specified in requestTemplates. The valid value is one of the    following: WHEN_NO_MATCH: passes the method request body through the integration request to    the back end without transformation when the method request content type does not match any    content type associated with the mapping templates defined in the integration request.    WHEN_NO_TEMPLATES: passes the method request body through the integration request to the back    end without transformation when no mapping template is defined in the integration request. If    a template is defined when this option is selected, the method request of an unmapped    content-type will be rejected with an HTTP 415 Unsupported Media Type response. NEVER: rejects    the method request with an HTTP 415 Unsupported Media Type response when either the method    request content type does not match any content type associated with the mapping templates    defined in the integration request or no mapping template is defined in the integration    request.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PassthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<cfn_resources::StrVal>,

    ///
    /// A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of method.request.{location}.{name}, where location is querystring, path, or header and name must be a valid and unique method request parameter name.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<std::collections::HashMap<String, String>>,

    ///
    /// Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<std::collections::HashMap<String, String>>,

    ///
    /// Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,

    ///
    /// Specifies an API method integration type. The valid value is one of the following:
    ///
    /// For the HTTP and HTTP proxy integrations, each integration can specify a protocol (http/https), port and path. Standard 80 and 443 ports are supported as well as custom ports above 1024. An HTTP or HTTP proxy integration with a connectionType of VPC_LINK is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AWS | AWS_PROXY | HTTP | HTTP_PROXY | MOCK
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<IntegrationTypeEnum>,

    ///
    /// Specifies Uniform Resource Identifier (URI) of the integration endpoint.
    ///
    /// For HTTP or HTTP_PROXY integrations, the URI must be a fully formed, encoded HTTP(S) URL 	  according to the RFC-3986 specification for standard integrations. If connectionType is VPC_LINK specify the Network Load Balancer DNS name.    For AWS or AWS_PROXY integrations, the URI is of    the form arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service_api}.    Here, {Region} is the API Gateway region (e.g., us-east-1); {service} is the name of the    integrated AWS service (e.g., s3); and {subdomain} is a designated subdomain supported by    certain AWS service for fast host-name lookup. action can be used for an AWS service    action-based API, using an Action={name}&{p1}={v1}&p2={v2}... query string. The ensuing    {service_api} refers to a supported action {name} plus any required input parameters.    Alternatively, path can be used for an AWS service path-based API. The ensuing service_api    refers to the path to an AWS service resource, including the region of the integrated AWS    service, if applicable. For example, for integration with the S3 API of GetObject, the uri can    be either arn:aws:apigateway:us-west-2:s3:action/GetObject&Bucket={bucket}&Key={key} or    arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IntegrationConnectionTypeEnum {
    /// INTERNET
    #[serde(rename = "INTERNET")]
    Internet,

    /// VPC_LINK
    #[serde(rename = "VPC_LINK")]
    Vpclink,
}

impl Default for IntegrationConnectionTypeEnum {
    fn default() -> Self {
        IntegrationConnectionTypeEnum::Internet
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IntegrationContentHandlingEnum {
    /// CONVERT_TO_BINARY
    #[serde(rename = "CONVERT_TO_BINARY")]
    Converttobinary,

    /// CONVERT_TO_TEXT
    #[serde(rename = "CONVERT_TO_TEXT")]
    Converttotext,
}

impl Default for IntegrationContentHandlingEnum {
    fn default() -> Self {
        IntegrationContentHandlingEnum::Converttobinary
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IntegrationTypeEnum {
    /// AWS
    #[serde(rename = "AWS")]
    Aws,

    /// AWS_PROXY
    #[serde(rename = "AWS_PROXY")]
    Awsproxy,

    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// HTTP_PROXY
    #[serde(rename = "HTTP_PROXY")]
    Httpproxy,

    /// MOCK
    #[serde(rename = "MOCK")]
    Mock,
}

impl Default for IntegrationTypeEnum {
    fn default() -> Self {
        IntegrationTypeEnum::Aws
    }
}

impl cfn_resources::CfnResource for Integration {
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

/// IntegrationResponse is a property of the Amazon API Gateway Method Integration property type that specifies the response that API Gateway sends after a method's backend finishes processing a request.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct IntegrationResponse {
    ///
    /// Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:
    ///
    /// If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONVERT_TO_BINARY | CONVERT_TO_TEXT
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<IntegrationResponseContentHandlingEnum>,

    ///
    /// A key-value map specifying response parameters that are passed to the method response from the back end.       The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,

    ///
    /// Specifies the templates used to transform the integration response body. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,

    ///
    /// Specifies the regular expression (regex) pattern used to choose an integration response based on the response from the back end. For example, if the success response returns nothing and the error response returns some string, you could use the .+ regex to match error response. However, make sure that the error response does not contain any newline (\n) character in such cases. If the back end is an AWS Lambda function, the AWS Lambda function error header is matched. For all other HTTP and AWS back ends, the HTTP status code is matched.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SelectionPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the status code that is used to map the integration response to an existing MethodResponse.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum IntegrationResponseContentHandlingEnum {
    /// CONVERT_TO_BINARY
    #[serde(rename = "CONVERT_TO_BINARY")]
    Converttobinary,

    /// CONVERT_TO_TEXT
    #[serde(rename = "CONVERT_TO_TEXT")]
    Converttotext,
}

impl Default for IntegrationResponseContentHandlingEnum {
    fn default() -> Self {
        IntegrationResponseContentHandlingEnum::Converttobinary
    }
}

impl cfn_resources::CfnResource for IntegrationResponse {
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

/// Represents a method response of a given HTTP status code returned to the client. The method response is passed from the back end through the associated integration response that can be transformed using a mapping template.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MethodResponse {
    ///
    /// Specifies the Model resources used for the response's content-type. Response models are represented as a key/value map, with a content-type as the key and a Model name as the value.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<std::collections::HashMap<String, String>>,

    ///
    /// A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header and the value specifies whether the associated method response header is required or not. The expression of the key must match the pattern method.response.header.{name}, where name is a valid and unique header name. API Gateway passes certain integration response data to the method response headers specified here according to the mapping you prescribe in the API's IntegrationResponse. The integration response data that can be mapped include an integration response header expressed in integration.response.header.{name}, a static value enclosed within a pair of single quotes (e.g., 'application/json'), or a JSON expression from the back-end response payload in the form of integration.response.body.{JSON-expression}, where JSON-expression is a valid JSON expression without the $ prefix.)
    ///
    /// Required: No
    ///
    /// Type: Map of Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<std::collections::HashMap<String, bool>>,

    ///
    /// The method response's status code.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MethodResponse {
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
