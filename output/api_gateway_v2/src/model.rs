/// The AWS::ApiGatewayV2::Model resource updates data model for a          WebSocket API. For more information, see Model Selection Expressions in the API Gateway Developer             Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub api_id: cfn_resources::StrVal,

    ///
    /// The content-type for the model, for example, "application/json".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<cfn_resources::StrVal>,

    ///
    /// The description of the model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the model.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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

    #[serde(skip_serializing)]
    pub att_model_id: CfnModelmodelid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnModelmodelid;
impl CfnModelmodelid {
    pub fn att_name(&self) -> &'static str {
        r#"ModelId"#
    }
}

impl cfn_resources::CfnResource for CfnModel {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::Model"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
