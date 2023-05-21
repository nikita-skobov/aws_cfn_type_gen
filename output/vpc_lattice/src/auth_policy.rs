

/// Creates or updates the auth policy. The policy string in JSON must not contain newlines or  blank lines.
#[derive(Default, serde::Serialize)]
pub struct CfnAuthPolicy {


    /// 
    /// The ID or Amazon Resource Name (ARN) of the service network or service for which the policy  is created.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: String,


    /// 
    /// The auth policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,

}
