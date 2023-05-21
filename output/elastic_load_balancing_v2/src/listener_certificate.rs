

/// Specifies an SSL server certificate to add to the certificate list for an HTTPS or TLS     listener.
#[derive(Default, serde::Serialize)]
pub struct CfnListenerCertificate {


    /// 
    /// The Amazon Resource Name (ARN) of the listener.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListenerArn")]
    pub listener_arn: String,


    /// 
    /// The certificate. You can specify one certificate per resource.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Certificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificates")]
    pub certificates: Vec<Certificate>,

}


/// Specifies an SSL server certificate for the certificate list of a secure     listener.
#[derive(Default, serde::Serialize)]
pub struct Certificate {


    /// 
    /// The Amazon Resource Name (ARN) of the certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,

}