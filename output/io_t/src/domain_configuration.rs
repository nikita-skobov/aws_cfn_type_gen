

/// Specifies a domain configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDomainConfiguration {


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
    pub service_type: Option<String>,


    /// 
    /// The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority.      This value is not required for AWS-managed domains.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationCertificateArn")]
    pub validation_certificate_arn: Option<String>,


    /// 
    /// The ARNs of the certificates that AWS IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN.    This value is not required for AWS-managed domains.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerCertificateArns")]
    pub server_certificate_arns: Option<Vec<String>>,


    /// 
    /// The name of the domain configuration. This value must be unique to a region.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainConfigurationName")]
    pub domain_configuration_name: Option<String>,


    /// 
    /// The name of the domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,


    /// 
    /// An object that specifies the authorization service for a domain.
    /// 
    /// Required: No
    ///
    /// Type: AuthorizerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizerConfig")]
    pub authorizer_config: Option<AuthorizerConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: TlsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsConfig")]
    pub tls_config: Option<TlsConfig>,


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
    pub tags: Option<Vec<Tag>>,


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
    pub domain_configuration_status: Option<String>,

}

impl cfn_resources::CfnResource for CfnDomainConfiguration {
    fn type_string() -> &'static str {
        "AWS::IoT::DomainConfiguration"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// An object that contains information about a server certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerCertificateSummary {


    /// 
    /// Details that explain the status of the server certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateStatusDetail")]
    pub server_certificate_status_detail: Option<String>,


    /// 
    /// The status of the server certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateStatus")]
    pub server_certificate_status: Option<String>,


    /// 
    /// The ARN of the server certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerCertificateArn")]
    pub server_certificate_arn: Option<String>,

}


/// The TlsConfig property type specifies Property description not available. for an AWS::IoT::DomainConfiguration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityPolicy")]
    pub security_policy: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// An object that specifies the authorization service for a domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AuthorizerConfig {


    /// 
    /// The name of the authorization service for a domain configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultAuthorizerName")]
    pub default_authorizer_name: Option<String>,


    /// 
    /// A Boolean that specifies whether the domain configuration's authorization service can be overridden.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowAuthorizerOverride")]
    pub allow_authorizer_override: Option<bool>,

}
