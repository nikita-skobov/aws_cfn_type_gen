

/// Use the AWS::EventSchemas::RegistryPolicy resource to specify       resource-based policies for an EventBridge Schema Registry.
#[derive(Default, serde::Serialize)]
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
    /// The revision ID of the policy.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevisionId")]
    pub revision_id: Option<String>,


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

}
