/// Specifies a domain configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDomainConfiguration {
    ///
    /// An object that specifies the authorization service for a domain.
    ///
    /// Required: No
    ///
    /// Type: AuthorizerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_config: Option<AuthorizerConfig>,

    ///
    /// The name of the domain configuration. This value must be unique to a region.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_name: Option<cfn_resources::StrVal>,

    ///
    /// The status to which the domain configuration should be updated.
    ///
    /// Valid values: ENABLED | DISABLED
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_status: Option<cfn_resources::StrVal>,

    ///
    /// The name of the domain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// The ARNs of the certificates that AWS IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN.    This value is not required for AWS-managed domains.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerCertificateArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_arns: Option<Vec<String>>,

    ///
    /// The type of service delivered by the endpoint.
    ///
    /// Note        AWS IoT Core currently supports only the DATA service type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<cfn_resources::StrVal>,

    ///
    /// Metadata which can be used to manage the domain configuration.
    ///
    /// NoteFor URI Request parameters use format: ...key1=value1&key2=value2...For the CLI command-line parameter use format: &&tags       "key1=value1&key2=value2..."For the cli-input-json file use format: "tags":       "key1=value1&key2=value2..."
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TlsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,

    ///
    /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority.      This value is not required for AWS-managed domains.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_certificate_arn: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnDomainConfigurationarn,

    #[serde(skip_serializing)]
    pub att_domain_type: CfnDomainConfigurationdomaintype,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainConfigurationarn;
impl CfnDomainConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainConfigurationdomaintype;
impl CfnDomainConfigurationdomaintype {
    pub fn att_name(&self) -> &'static str {
        r#"DomainType"#
    }
}

impl cfn_resources::CfnResource for CfnDomainConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::DomainConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.authorizer_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.tls_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that specifies the authorization service for a domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AuthorizerConfig {
    ///
    /// A Boolean that specifies whether the domain configuration's authorization service can be overridden.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowAuthorizerOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_authorizer_override: Option<bool>,

    ///
    /// The name of the authorization service for a domain configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAuthorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authorizer_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AuthorizerConfig {
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

/// An object that contains information about a server certificate.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ServerCertificateSummary {
    ///
    /// The ARN of the server certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The status of the server certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status: Option<cfn_resources::StrVal>,

    ///
    /// Details that explain the status of the server certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateStatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status_detail: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ServerCertificateSummary {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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

/// The TlsConfig property type specifies Property description not available. for an AWS::IoT::DomainConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TlsConfig {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TlsConfig {
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
