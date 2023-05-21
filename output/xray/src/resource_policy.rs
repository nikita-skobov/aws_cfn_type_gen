

/// Use AWS::XRay::ResourcePolicy to specify an X-Ray resource-based policy,      which grants one or more AWS services and accounts permissions      to access X-Ray. Each resource-based policy is associated with a      specific AWS account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourcePolicy {


    /// 
    /// A flag to indicate whether to bypass the resource-based policy lockout safety check.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BypassPolicyLockoutCheck")]
    pub bypass_policy_lockout_check: Option<bool>,


    /// 
    /// The resource-based policy document, which can be up to 5kb in size.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: String,


    /// 
    /// The name of the resource-based policy. Must be unique within a specific AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

}



impl cfn_resources::CfnResource for CfnResourcePolicy {
    fn type_string(&self) -> &'static str {
        "AWS::XRay::ResourcePolicy"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}