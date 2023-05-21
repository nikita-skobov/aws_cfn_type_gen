

/// The AWS::AppSync::DomainNameApiAssociation resource represents the mapping of your custom     domain name to the assigned API URL.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainNameApiAssociation {


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
    /// The API ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiId")]
    pub api_id: String,

}



impl cfn_resources::CfnResource for CfnDomainNameApiAssociation {
    fn type_string() -> &'static str {
        "AWS::AppSync::DomainNameApiAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
