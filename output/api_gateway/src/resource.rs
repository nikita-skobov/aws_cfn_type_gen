

/// The AWS::ApiGateway::Resource resource creates a resource in an API.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResource {


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



impl cfn_resources::CfnResource for CfnResource {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGateway::Resource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}