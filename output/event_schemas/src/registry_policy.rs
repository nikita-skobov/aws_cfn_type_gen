

/// Use the AWS::EventSchemas::RegistryPolicy resource to specify       resource-based policies for an EventBridge Schema Registry.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRegistryPolicy {


    /// 
    /// A resource-based policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,


    /// 
    /// The name of the registry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryName")]
    pub registry_name: String,


    /// 
    /// The revision ID of the policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevisionId")]
    pub revision_id: Option<String>,

}



impl cfn_resources::CfnResource for CfnRegistryPolicy {
    fn type_string() -> &'static str {
        "AWS::EventSchemas::RegistryPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
