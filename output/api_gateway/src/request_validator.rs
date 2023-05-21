/// The AWS::ApiGateway::RequestValidator resource sets up basic validation rules for incoming requests to your API. For more information, see Enable Basic Request Validation for an API in API Gateway in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRequestValidator {
    ///
    /// The name of this RequestValidator
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

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
    /// A Boolean flag to indicate whether to validate a request body according to the configured Model schema.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidateRequestBody")]
    pub validate_request_body: Option<bool>,

    ///
    /// A Boolean flag to indicate whether to validate request parameters (true) or not (false).
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidateRequestParameters")]
    pub validate_request_parameters: Option<bool>,
}

impl cfn_resources::CfnResource for CfnRequestValidator {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::RequestValidator"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
