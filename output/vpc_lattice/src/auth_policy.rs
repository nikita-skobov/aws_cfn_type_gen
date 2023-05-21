/// Creates or updates the auth policy. The policy string in JSON must not contain newlines or  blank lines.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAuthPolicy {
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
}

impl cfn_resources::CfnResource for CfnAuthPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::AuthPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
