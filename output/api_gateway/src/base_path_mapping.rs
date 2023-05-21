

/// The AWS::ApiGateway::BasePathMapping resource creates a base path that clients who call your API must use in the invocation URL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBasePathMapping {


    /// 
    /// The base path name that callers of the API must provide as part of the URL after the domain name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "BasePath")]
    pub base_path: Option<String>,


    /// 
    /// The domain name of the BasePathMapping resource to be described.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: Option<String>,


    /// 
    /// The string identifier of the associated RestApi.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestApiId")]
    pub rest_api_id: Option<String>,


    /// 
    /// The name of the associated stage.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    pub stage: Option<String>,

}



impl cfn_resources::CfnResource for CfnBasePathMapping {
    fn type_string() -> &'static str {
        "AWS::ApiGateway::BasePathMapping"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
