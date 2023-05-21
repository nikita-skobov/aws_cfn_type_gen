

/// The AWS::WAFRegional::WebACLAssociation resource associates an AWS WAF Regional web access control group (ACL) with a resource.
#[derive(Default, serde::Serialize)]
pub struct CfnWebACLAssociation {


    /// 
    /// The Amazon Resource Name (ARN) of the resource to protect with the web ACL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,


    /// 
    /// A unique identifier (ID) for the web ACL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WebACLId")]
    pub web_aclid: String,

}
