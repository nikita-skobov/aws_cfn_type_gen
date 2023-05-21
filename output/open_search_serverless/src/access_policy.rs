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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The JSON policy document without any whitespaces.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: cfn_resources::StrVal,

    ///
    /// The type of access policy. Currently the only option is data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnAccessPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::OpenSearchServerless::AccessPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
