

/// The AWS::ApiGatewayV2::ApiMapping resource contains an API mapping.          An API mapping relates a path of your custom domain name to a stage of your API. A          custom domain name can have multiple API mappings, but the paths can't overlap. A          custom domain can map only to APIs of the same protocol type. For more          information, see CreateApiMapping in the Amazon API Gateway V2 API             Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnApiMapping {


    /// 
    /// The identifier of the API.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiId")]
    pub api_id: String,


    /// 
    /// The API mapping key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiMappingKey")]
    pub api_mapping_key: Option<String>,


    /// 
    /// The domain name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// The API stage.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stage")]
    pub stage: String,

}



impl cfn_resources::CfnResource for CfnApiMapping {
    fn type_string() -> &'static str {
        "AWS::ApiGatewayV2::ApiMapping"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}