/// The AWS::ApiGatewayV2::IntegrationResponse resource updates an          integration response for an WebSocket API. For more information, see Set up WebSocket API Integration Responses in API Gateway in the             API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIntegrationResponse {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling_strategy: Option<String>,

    ///
    /// The integration ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IntegrationId")]
    pub integration_id: String,

    ///
    /// The integration response key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegrationResponseKey")]
    pub integration_response_key: String,

    ///
    /// A key-value map specifying response parameters that are passed to the method          response from the backend. The key is a method response header parameter name and          the mapped value is an integration response header value, a static value enclosed          within a pair of single quotes, or a JSON expression from the integration response          body. The mapping key must match the pattern of                method.response.header.{name}          , where name is a valid and unique header name. The mapped non-static value          must match the pattern of                integration.response.header.{name}          or integration.response.body.{JSON-expression}          , where             {name}          is a valid and unique response header name and             {JSON-expression}          is a valid JSON expression without the $ prefix.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<serde_json::Value>,

    ///
    /// The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<serde_json::Value>,

    ///
    /// The template selection expression for the integration response. Supported only for WebSocket APIs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateSelectionExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_selection_expression: Option<String>,
}

impl cfn_resources::CfnResource for CfnIntegrationResponse {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::IntegrationResponse"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
