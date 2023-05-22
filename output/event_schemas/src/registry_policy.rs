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
    pub registry_name: cfn_resources::StrVal,

    ///
    /// The revision ID of the policy.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_id: CfnRegistryPolicyid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRegistryPolicyid;
impl CfnRegistryPolicyid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnRegistryPolicy {
    fn type_string(&self) -> &'static str {
        "AWS::EventSchemas::RegistryPolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
