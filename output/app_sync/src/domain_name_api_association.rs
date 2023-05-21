

/// The AWS::AppSync::DomainNameApiAssociation resource represents the mapping of your custom     domain name to the assigned API URL.
#[derive(Default, serde::Serialize)]
pub struct CfnDomainNameApiAssociation {


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

}
