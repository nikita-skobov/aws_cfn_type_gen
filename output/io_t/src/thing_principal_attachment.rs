

/// Use the AWS::IoT::ThingPrincipalAttachment resource to attach a principal     (an X.509 certificate or another credential) to a thing.
///
/// For more information about working with AWS IoT things and principals, see Authorization in the AWS IoT Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnThingPrincipalAttachment {


    /// 
    /// The name of the AWS IoT thing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingName")]
    pub thing_name: String,


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

}
