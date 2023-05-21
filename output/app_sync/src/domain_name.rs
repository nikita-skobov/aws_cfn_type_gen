/// The AWS::AppSync::DomainName resource creates a DomainNameConfig object to     configure a custom domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainName {
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
    /// The domain name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl cfn_resources::CfnResource for CfnDomainName {
    fn type_string(&self) -> &'static str {
        "AWS::AppSync::DomainName"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
