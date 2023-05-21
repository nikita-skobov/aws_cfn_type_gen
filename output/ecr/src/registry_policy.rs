

/// The AWS::ECR::RegistryPolicy resource creates or updates the permissions       policy for a private registry.
///
/// A private registry policy is used to specify permissions for another AWS account and is used when configuring cross-account replication. For       more information, see Registry         permissions in the Amazon Elastic Container Registry User         Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnRegistryPolicy {


    /// 
    /// The JSON policy text for your registry.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyText")]
    pub policy_text: serde_json::Value,

}
