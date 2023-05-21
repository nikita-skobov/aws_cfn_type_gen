

/// The AWS::Lightsail::LoadBalancerTlsCertificate resource specifies a TLS     certificate that can be used with a Lightsail load balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoadBalancerTlsCertificate {


    /// 
    /// An array of alternative domain names and subdomain names for your SSL/TLS     certificate.
    /// 
    /// In addition to the primary domain name, you can have up to nine alternative domain names.     Wildcards (such as *.example.com) are not supported.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "CertificateAlternativeNames")]
    pub certificate_alternative_names: Option<Vec<String>>,


    /// 
    /// The domain name for the SSL/TLS certificate. For example, example.com or www.example.com.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "CertificateDomainName")]
    pub certificate_domain_name: String,


    /// 
    /// The name of the SSL/TLS certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,


    /// 
    /// A Boolean value indicating whether HTTPS redirection is enabled for the load    balancer that the TLS certificate is attached to.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpsRedirectionEnabled")]
    pub https_redirection_enabled: Option<bool>,


    /// 
    /// A Boolean value indicating whether the SSL/TLS certificate is attached to a Lightsail load balancer.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsAttached")]
    pub is_attached: Option<bool>,


    /// 
    /// The name of the load balancer that the SSL/TLS certificate is attached to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \w[\w\-]*\w
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,

}



impl cfn_resources::CfnResource for CfnLoadBalancerTlsCertificate {
    fn type_string() -> &'static str {
        "AWS::Lightsail::LoadBalancerTlsCertificate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}