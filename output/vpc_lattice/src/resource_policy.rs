/// Retrieves information about the resource policy. The resource policy is an IAM policy  created on behalf of the resource owner when they share a resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourcePolicy {
    ///
    /// The Amazon Resource Name (ARN) of the service network or service.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,

    ///
    /// An IAM policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::ResourcePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
