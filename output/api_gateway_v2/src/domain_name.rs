/// The AWS::ApiGatewayV2::DomainName resource specifies a custom domain          name for your API in Amazon API Gateway (API Gateway).
///
/// You can use a custom domain name to provide a URL that's more intuitive and easier          to recall. For more information about using custom domain names, see Set up             Custom Domain Name for an API in API Gateway in the API             Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDomainName {
    ///
    /// The custom domain name for your API in Amazon API Gateway. Uppercase letters are          not supported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// The domain name configurations.
    ///
    /// Required: No
    ///
    /// Type: List of DomainNameConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainNameConfigurations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub domain_name_configurations: Option<Vec<DomainNameConfiguration>>,

    ///
    /// The mutual TLS authentication configuration for a custom domain name.
    ///
    /// Required: No
    ///
    /// Type: MutualTlsAuthentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "MutualTlsAuthentication")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,

    ///
    /// The collection of tags associated with a domain name.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_regional_domain_name: CfnDomainNameregionaldomainname,

    #[serde(skip_serializing)]
    pub att_regional_hosted_zone_id: CfnDomainNameregionalhostedzoneid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNameregionaldomainname;
impl CfnDomainNameregionaldomainname {
    pub fn att_name(&self) -> &'static str {
        r#"RegionalDomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNameregionalhostedzoneid;
impl CfnDomainNameregionalhostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"RegionalHostedZoneId"#
    }
}

impl cfn_resources::CfnResource for CfnDomainName {
    fn type_string(&self) -> &'static str {
        "AWS::ApiGatewayV2::DomainName"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.mutual_tls_authentication
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The DomainNameConfiguration property type specifies the configuration          for a an API's domain name.
///
/// DomainNameConfiguration is a property of the AWS::ApiGatewayV2::DomainName resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DomainNameConfiguration {
    ///
    /// An AWS-managed certificate that will be used by the edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The user-friendly name of the certificate that will be used by the edge-optimized endpoint for this domain name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub certificate_name: Option<cfn_resources::StrVal>,

    ///
    /// The endpoint type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub endpoint_type: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon resource name (ARN) for the public certificate issued by AWS Certificate Manager. This ARN is used to validate custom domain ownership. It's required only if you configure mutual TLS and use either an ACM-imported or a private CA certificate ARN as the regionalCertificateArn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OwnershipVerificationCertificateArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub ownership_verification_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Transport Layer Security (TLS) version of the security policy for this domain name. The valid values are TLS_1_0 and TLS_1_2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityPolicy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_policy: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DomainNameConfiguration {
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

/// If specified, API Gateway performs two-way authentication between the client and the server. Clients must present a trusted certificate to access your API.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MutualTlsAuthentication {
    ///
    /// An Amazon S3 URL that specifies the truststore for mutual TLS           authentication, for example, s3://bucket-name/key-name           .           The truststore can contain certificates from public or private certificate           authorities. To update the truststore, upload a new version to S3, and then           update your custom domain name to use the new version. To update the truststore,           you must have permissions to access the S3 object.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruststoreUri")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub truststore_uri: Option<cfn_resources::StrVal>,

    ///
    /// The version of the S3 object that contains your truststore. To           specify a version, you must have versioning enabled for the S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruststoreVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub truststore_version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MutualTlsAuthentication {
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
