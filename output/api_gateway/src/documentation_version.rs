

/// The AWS::ApiGateway::DocumentationVersion resource creates a snapshot of the documentation for an API. For more information, see Representation of API Documentation in API Gateway in the API Gateway Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnDocumentationVersion {


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
    /// A description about the new documentation snapshot.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}
