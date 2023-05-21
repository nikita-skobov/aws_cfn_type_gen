

/// Creates a data access policy for OpenSearch Serverless. Access policies limit access to collections       and the resources within them, and allow a user to access that data irrespective of the       access mechanism or network source. For more information, see Data access         control for Amazon OpenSearch Serverless.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPolicy {


    /// 
    /// The description of the policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The name of the policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The JSON policy document without any whitespaces.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: String,


    /// 
    /// The type of access policy. Currently the only option is data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

}



impl cfn_resources::CfnResource for CfnAccessPolicy {
    fn type_string() -> &'static str {
        "AWS::OpenSearchServerless::AccessPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
