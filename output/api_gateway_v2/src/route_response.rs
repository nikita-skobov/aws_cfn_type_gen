

/// The AWS::ApiGatewayV2::RouteResponse resource creates a route          response for a WebSocket API. For more information, see Set up Route Responses for a WebSocket API in API Gateway in the             API Gateway Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnRouteResponse {


    /// 
    /// The route response parameters.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseParameters")]
    pub response_parameters: Option<serde_json::Value>,


    /// 
    /// The route response key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteResponseKey")]
    pub route_response_key: String,


    /// 
    /// The response models for the route response.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseModels")]
    pub response_models: Option<serde_json::Value>,


    /// 
    /// The model selection expression for the route response. Supported only for WebSocket APIs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelSelectionExpression")]
    pub model_selection_expression: Option<String>,


    /// 
    /// The route ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RouteId")]
    pub route_id: String,


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

}


/// Specifies whether the parameter is required.
#[derive(Default, serde::Serialize)]
pub struct ParameterConstraints {


    /// 
    /// Specifies whether the parameter is required.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Required")]
    pub required: bool,

}