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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub certificate_domain_name: cfn_resources::StrVal,

    ///
    /// The name of the SSL/TLS certificate.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateName")]
    pub certificate_name: cfn_resources::StrVal,

    ///
    /// A Boolean value indicating whether HTTPS redirection is enabled for the load    balancer that the TLS certificate is attached to.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpsRedirectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub load_balancer_name: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_load_balancer_tls_certificate_arn:
        CfnLoadBalancerTlsCertificateloadbalancertlscertificatearn,

    #[serde(skip_serializing)]
    pub att_status: CfnLoadBalancerTlsCertificatestatus,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoadBalancerTlsCertificateloadbalancertlscertificatearn;
impl CfnLoadBalancerTlsCertificateloadbalancertlscertificatearn {
    pub fn att_name(&self) -> &'static str {
        r#"LoadBalancerTlsCertificateArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoadBalancerTlsCertificatestatus;
impl CfnLoadBalancerTlsCertificatestatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnLoadBalancerTlsCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::LoadBalancerTlsCertificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
