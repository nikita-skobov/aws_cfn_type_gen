

/// The AWS::DMS::Certificate resource creates an Secure Sockets Layer (SSL) certificate that       encrypts connections between AWS DMS endpoints and the replication instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificate {


    /// 
    /// The location of an imported Oracle Wallet certificate for use with SSL. An example       is: filebase64("${path.root}/rds-ca-2019-root.sso")
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateWallet")]
    pub certificate_wallet: Option<String>,


    /// 
    /// The contents of a .pem file, which contains an X.509 certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificatePem")]
    pub certificate_pem: Option<String>,


    /// 
    /// A customer-assigned name for the certificate. Identifiers must begin with a letter and     must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or     contain two consecutive hyphens.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateIdentifier")]
    pub certificate_identifier: Option<String>,

}



impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string() -> &'static str {
        "AWS::DMS::Certificate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
