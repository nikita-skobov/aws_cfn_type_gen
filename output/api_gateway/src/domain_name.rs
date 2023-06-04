/// The AWS::ApiGateway::DomainName resource specifies a custom domain name for your API in API Gateway.
///
/// You can use a custom domain name to provide a URL that's more intuitive and easier to recall. For more information about using custom domain names, see Set up Custom Domain Name for an API in API Gateway in the API Gateway Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDomainName {
    ///
    /// The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.
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
    /// The custom domain name as an API host name, for example, my-api.example.com.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// The endpoint configuration of this DomainName showing the endpoint types of the domain name.
    ///
    /// Required: No
    ///
    /// Type: EndpointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub endpoint_configuration: Option<EndpointConfiguration>,

    ///
    /// The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway    performs two-way authentication between the client and the server. Clients must present a    trusted certificate to access your API.
    ///
    /// Required: No
    ///
    /// Type: MutualTlsAuthentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "MutualTlsAuthentication")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,

    /// The ARN of the public certificate issued by ACM to validate ownership of your custom domain. Only required when configuring mutual TLS and using an ACM imported or private CA certificate ARN as the RegionalCertificateArn.
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
    /// The reference to an AWS-managed certificate that will be used for validating the regional domain name. AWS Certificate Manager is the only supported source.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionalCertificateArn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub regional_certificate_arn: Option<cfn_resources::StrVal>,

    ///
    /// The Transport Layer Security (TLS) version + cipher suite for this DomainName. The valid values are TLS_1_0 and TLS_1_2.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: TLS_1_0 | TLS_1_2
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityPolicy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_policy: Option<DomainNameSecurityPolicyEnum>,

    ///
    /// The collection of tags. Each tag element is associated with a given resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_distribution_domain_name: CfnDomainNamedistributiondomainname,

    #[serde(skip_serializing)]
    pub att_distribution_hosted_zone_id: CfnDomainNamedistributionhostedzoneid,

    #[serde(skip_serializing)]
    pub att_regional_domain_name: CfnDomainNameregionaldomainname,

    #[serde(skip_serializing)]
    pub att_regional_hosted_zone_id: CfnDomainNameregionalhostedzoneid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DomainNameSecurityPolicyEnum {
    /// TLS_1_0
    #[serde(rename = "TLS_1_0")]
    Tls10,

    /// TLS_1_2
    #[serde(rename = "TLS_1_2")]
    Tls12,
}

impl Default for DomainNameSecurityPolicyEnum {
    fn default() -> Self {
        DomainNameSecurityPolicyEnum::Tls10
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNamedistributiondomainname;
impl CfnDomainNamedistributiondomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DistributionDomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainNamedistributionhostedzoneid;
impl CfnDomainNamedistributionhostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"DistributionHostedZoneId"#
    }
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
        "AWS::ApiGateway::DomainName"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.endpoint_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mutual_tls_authentication
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The EndpointConfiguration property type specifies the endpoint types of an Amazon API Gateway domain name.
///
/// EndpointConfiguration is a property of the AWS::ApiGateway::DomainName resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EndpointConfiguration {
    ///
    /// A list of endpoint types of an API (RestApi) or its custom domain name (DomainName). For an edge-optimized API and its custom domain name, the endpoint type is "EDGE". For a regional API and its custom domain name, the endpoint type is REGIONAL. For a private API, the endpoint type is PRIVATE.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Types")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub types: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for EndpointConfiguration {
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

/// The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway    performs two-way authentication between the client and the server. Clients must present a    trusted certificate to access your API.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MutualTlsAuthentication {
    ///
    /// An Amazon S3 URL that specifies the truststore for mutual TLS authentication, for example    s3://bucket-name/key-name. The truststore can contain certificates from public or private    certificate authorities. To update the truststore, upload a new version to S3, and then update    your custom domain name to use the new version. To update the truststore, you must have    permissions to access the S3 object.
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
    /// The version of the S3 object that contains your truststore. To specify a version, you must have versioning enabled for the S3 bucket.
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
