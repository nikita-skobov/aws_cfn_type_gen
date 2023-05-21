

/// Use the AWS::IoT::Policy resource to declare an AWS IoT policy. For more     information about working with AWS IoT policies, see Authorization in the       AWS IoT Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPolicy {


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

}
