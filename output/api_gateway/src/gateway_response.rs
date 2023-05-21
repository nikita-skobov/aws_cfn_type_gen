

/// The AWS::ApiGateway::GatewayResponse resource creates a gateway response for your API. For more information, see API Gateway Responses in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGatewayResponse {


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
    pub response_type: GatewayResponseResponseTypeEnum,


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
    /// The HTTP status code for this GatewayResponse.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum GatewayResponseResponseTypeEnum {

    /// ACCESS_DENIED
    #[serde(rename = "ACCESS_DENIED")]
    Accessdenied,

    /// API_CONFIGURATION_ERROR
    #[serde(rename = "API_CONFIGURATION_ERROR")]
    Apiconfigurationerror,

    /// AUTHORIZER_CONFIGURATION_ERROR
    #[serde(rename = "AUTHORIZER_CONFIGURATION_ERROR")]
    Authorizerconfigurationerror,

    /// AUTHORIZER_FAILURE
    #[serde(rename = "AUTHORIZER_FAILURE")]
    Authorizerfailure,

    /// BAD_REQUEST_BODY
    #[serde(rename = "BAD_REQUEST_BODY")]
    Badrequestbody,

    /// BAD_REQUEST_PARAMETERS
    #[serde(rename = "BAD_REQUEST_PARAMETERS")]
    Badrequestparameters,

    /// DEFAULT_4XX
    #[serde(rename = "DEFAULT_4XX")]
    Default4xx,

    /// DEFAULT_5XX
    #[serde(rename = "DEFAULT_5XX")]
    Default5xx,

    /// EXPIRED_TOKEN
    #[serde(rename = "EXPIRED_TOKEN")]
    Expiredtoken,

    /// INTEGRATION_FAILURE
    #[serde(rename = "INTEGRATION_FAILURE")]
    Integrationfailure,

    /// INTEGRATION_TIMEOUT
    #[serde(rename = "INTEGRATION_TIMEOUT")]
    Integrationtimeout,

    /// INVALID_API_KEY
    #[serde(rename = "INVALID_API_KEY")]
    Invalidapikey,

    /// INVALID_SIGNATURE
    #[serde(rename = "INVALID_SIGNATURE")]
    Invalidsignature,

    /// MISSING_AUTHENTICATION_TOKEN
    #[serde(rename = "MISSING_AUTHENTICATION_TOKEN")]
    Missingauthenticationtoken,

    /// QUOTA_EXCEEDED
    #[serde(rename = "QUOTA_EXCEEDED")]
    Quotaexceeded,

    /// REQUEST_TOO_LARGE
    #[serde(rename = "REQUEST_TOO_LARGE")]
    Requesttoolarge,

    /// RESOURCE_NOT_FOUND
    #[serde(rename = "RESOURCE_NOT_FOUND")]
    Resourcenotfound,

    /// THROTTLED
    #[serde(rename = "THROTTLED")]
    Throttled,

    /// UNAUTHORIZED
    #[serde(rename = "UNAUTHORIZED")]
    Unauthorized,

    /// UNSUPPORTED_MEDIA_TYPE
    #[serde(rename = "UNSUPPORTED_MEDIA_TYPE")]
    Unsupportedmediatype,

}

impl Default for GatewayResponseResponseTypeEnum {
    fn default() -> Self {
        GatewayResponseResponseTypeEnum::Accessdenied
    }
}


impl cfn_resources::CfnResource for CfnGatewayResponse {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::GatewayResponse"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
