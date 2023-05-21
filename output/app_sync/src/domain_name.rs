

/// The AWS::AppSync::DomainName resource creates a DomainNameConfig object to     configure a custom domain.
#[derive(Default, serde::Serialize)]
pub struct CfnDomainName {


    /// 
    /// The domain name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// The decription for your domain name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the certificate. This will be an AWS Certificate Manager     certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}
