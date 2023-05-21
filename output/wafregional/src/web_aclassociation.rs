

/// The AWS::WAFRegional::WebACLAssociation resource associates an AWS WAF Regional web access control group (ACL) with a resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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



impl cfn_resources::CfnResource for CfnWebACLAssociation {
    fn type_string() -> &'static str {
        "AWS::WAFRegional::WebACLAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
