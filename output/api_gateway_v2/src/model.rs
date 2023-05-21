

/// The AWS::ApiGatewayV2::Model resource updates data model for a          WebSocket API. For more information, see Model Selection Expressions in the API Gateway Developer             Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnModel {


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
    /// The content-type for the model, for example, "application/json".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,


    /// 
    /// The description of the model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The schema for the model. For application/json models, this should be JSON schema draft 4 model.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: serde_json::Value,

}



impl cfn_resources::CfnResource for CfnModel {
    fn type_string() -> &'static str {
        "AWS::ApiGatewayV2::Model"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
