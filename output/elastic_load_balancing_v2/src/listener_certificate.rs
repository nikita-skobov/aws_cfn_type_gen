/// Specifies an SSL server certificate to add to the certificate list for an HTTPS or TLS     listener.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnListenerCertificate {
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

    ///
    /// The Amazon Resource Name (ARN) of the listener.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ListenerArn")]
    pub listener_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnListenerCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticLoadBalancingV2::ListenerCertificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies an SSL server certificate for the certificate list of a secure     listener.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Certificate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
