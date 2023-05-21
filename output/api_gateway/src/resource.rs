

/// The AWS::ApiGateway::Resource resource creates a resource in an API.
#[derive(Default, serde::Serialize)]
pub struct CfnResource {


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
    /// The last path segment for this resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PathPart")]
    pub path_part: String,


    /// 
    /// The parent resource's identifier.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentId")]
    pub parent_id: String,

}
