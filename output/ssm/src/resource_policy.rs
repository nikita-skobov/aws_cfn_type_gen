

/// Creates or updates a Systems Manager resource policy. A resource policy helps you       to define the IAM entity (for example, an AWS account)       that can manage your Systems Manager resources. Currently, OpsItemGroup       is the only resource that supports Systems Manager resource policies. The resource       policy for OpsItemGroup enables AWS accounts to view and       interact with OpsCenter operational work items (OpsItems). OpsCenter is a capability of       Systems Manager.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourcePolicy {


    /// 
    /// A policy you want to associate with a resource.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policy")]
    pub policy: serde_json::Value,


    /// 
    /// Amazon Resource Name (ARN) of the resource to which you want to attach a policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}

impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string() -> &'static str {
        "AWS::SSM::ResourcePolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
