/// The AWS::ACMPCA::CertificateAuthorityActivation resource creates and       installs a CA certificate on a CA. If no status is specified, the         AWS::ACMPCA::CertificateAuthorityActivation resource status defaults to       ACTIVE. Once the CA has a CA certificate installed, you can use the resource to toggle       the CA status field between ACTIVE and DISABLED.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificateAuthorityActivation {
    ///
    /// The Base64 PEM-encoded certificate authority certificate.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: cfn_resources::StrVal,

    ///
    /// The Amazon Resource Name (ARN) of your private CA.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: cfn_resources::StrVal,

    ///
    /// The Base64 PEM-encoded certificate chain that chains up to the root CA certificate       that you used to sign your private CA certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<cfn_resources::StrVal>,

    ///
    /// Status of your private CA.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_complete_certificate_chain: CfnCertificateAuthorityActivationcompletecertificatechain,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificateAuthorityActivationcompletecertificatechain;
impl CfnCertificateAuthorityActivationcompletecertificatechain {
    pub fn att_name(&self) -> &'static str {
        r#"CompleteCertificateChain"#
    }
}

impl cfn_resources::CfnResource for CfnCertificateAuthorityActivation {
    fn type_string(&self) -> &'static str {
        "AWS::ACMPCA::CertificateAuthorityActivation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
