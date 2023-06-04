/// Use the AWS::IoT::ThingPrincipalAttachment resource to attach a principal     (an X.509 certificate or another credential) to a thing.
///
/// For more information about working with AWS IoT things and principals, see Authorization in the AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnThingPrincipalAttachment {
    ///
    /// The principal, which can be a certificate ARN (as returned from the       CreateCertificate operation) or an Amazon Cognito ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: cfn_resources::StrVal,

    ///
    /// The name of the AWS IoT thing.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingName")]
    pub thing_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnThingPrincipalAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::ThingPrincipalAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
