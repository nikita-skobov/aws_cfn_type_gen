

/// Use the AWS::IoT::Policy resource to declare an AWS IoT policy. For more     information about working with AWS IoT policies, see Authorization in the       AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPolicy {


    /// 
    /// The policy name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,


    /// 
    /// The JSON document that describes the policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    pub policy_document: serde_json::Value,

}



impl cfn_resources::CfnResource for CfnPolicy {
    fn type_string() -> &'static str {
        "AWS::IoT::Policy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
