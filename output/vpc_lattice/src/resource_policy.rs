

/// Retrieves information about the resource policy. The resource policy is an IAM policy  created on behalf of the resource owner when they share a resource.
#[derive(Default, serde::Serialize)]
pub struct CfnResourcePolicy {


    /// 
    /// An IAM policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,


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

}
