

/// Use the AWS::IoT::PolicyPrincipalAttachment resource to attach an AWS IoT     policy to a principal (an X.509 certificate or other credential).
///
/// For information about working with AWS IoT policies and principals, see Authorization in the AWS IoT Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPolicyPrincipalAttachment {


    /// 
    /// The principal, which can be a certificate ARN (as returned from the       CreateCertificate operation) or an Amazon Cognito ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: String,


    /// 
    /// The name of the AWS IoT policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

}