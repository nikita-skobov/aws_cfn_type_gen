

/// The AWS::ApiGatewayV2::Model resource updates data model for a          WebSocket API. For more information, see Model Selection Expressions in the API Gateway Developer             Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnModel {


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
    /// The schema for the model. For application/json models, this should be JSON schema draft 4 model.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schema")]
    pub schema: serde_json::Value,


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

}
