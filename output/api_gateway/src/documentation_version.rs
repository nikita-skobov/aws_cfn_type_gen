

/// The AWS::ApiGateway::DocumentationVersion resource creates a snapshot of the documentation for an API. For more information, see Representation of API Documentation in API Gateway in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDocumentationVersion {


    /// 
    /// A description about the new documentation snapshot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The version identifier of the to-be-updated documentation version.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: String,


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

}



impl cfn_resources::CfnResource for CfnDocumentationVersion {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::DocumentationVersion"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}