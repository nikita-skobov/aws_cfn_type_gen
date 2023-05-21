

/// The AWS::ACMPCA::CertificateAuthorityActivation resource creates and       installs a CA certificate on a CA. If no status is specified, the         AWS::ACMPCA::CertificateAuthorityActivation resource status defaults to       ACTIVE. Once the CA has a CA certificate installed, you can use the resource to toggle       the CA status field between ACTIVE and DISABLED.
#[derive(Default, serde::Serialize)]
pub struct CfnCertificateAuthorityActivation {


    /// 
    /// The Amazon Resource Name (ARN) of your private CA.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,


    /// 
    /// Status of your private CA.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// The Base64 PEM-encoded certificate chain that chains up to the root CA certificate       that you used to sign your private CA certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: Option<String>,


    /// 
    /// The Base64 PEM-encoded certificate authority certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: String,

}
