/// Creates or updates the auth policy. The policy string in JSON must not contain newlines or  blank lines.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub resource_identifier: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_state: CfnAuthPolicystate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAuthPolicystate;
impl CfnAuthPolicystate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnAuthPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::AuthPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
