

/// The AWS::ApiGateway::GatewayResponse resource creates a gateway response for your API. For more information, see API Gateway Responses in the API Gateway Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnGatewayResponse {


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
    /// Response parameters (paths, query strings and headers) of the GatewayResponse as a    string-to-string map of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Response templates of the GatewayResponse as a string-to-string map of key-value pairs.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseTemplates")]
    pub response_templates: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The HTTP status code for this GatewayResponse.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,


    /// 
    /// The response type of the associated GatewayResponse.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ACCESS_DENIED | API_CONFIGURATION_ERROR | AUTHORIZER_CONFIGURATION_ERROR | AUTHORIZER_FAILURE | BAD_REQUEST_BODY | BAD_REQUEST_PARAMETERS | DEFAULT_4XX | DEFAULT_5XX | EXPIRED_TOKEN | INTEGRATION_FAILURE | INTEGRATION_TIMEOUT | INVALID_API_KEY | INVALID_SIGNATURE | MISSING_AUTHENTICATION_TOKEN | QUOTA_EXCEEDED | REQUEST_TOO_LARGE | RESOURCE_NOT_FOUND | THROTTLED | UNAUTHORIZED | UNSUPPORTED_MEDIA_TYPE
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResponseType")]
    pub response_type: String,

}
