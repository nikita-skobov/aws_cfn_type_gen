

/// The AWS::ApiGateway::DomainName resource specifies a custom domain name for your API in API Gateway.
///
/// You can use a custom domain name to provide a URL that's more intuitive and easier to recall. For more information about using custom domain names, see Set up Custom Domain Name for an API in API Gateway in the API Gateway Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnDomainName {


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
    pub security_policy: Option<String>,


    /// 
    /// The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,


    /// 
    /// The endpoint configuration of this DomainName showing the endpoint types of the domain name.
    /// 
    /// Required: No
    ///
    /// Type: EndpointConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: Option<EndpointConfiguration>,


    /// 
    /// The custom domain name as an API host name, for example, my-api.example.com.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,


    /// 
    /// The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway    performs two-way authentication between the client and the server. Clients must present a    trusted certificate to access your API.
    /// 
    /// Required: No
    ///
    /// Type: MutualTlsAuthentication
    ///
    /// Update requires: No interruption
    #[serde(rename = "MutualTlsAuthentication")]
    pub mutual_tls_authentication: Option<MutualTlsAuthentication>,


    /// 
    /// The collection of tags. Each tag element is associated with a given resource.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// The ARN of the public certificate issued by ACM to validate ownership of your custom domain. Only required when configuring mutual TLS and using an ACM imported or private CA certificate ARN as the RegionalCertificateArn.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OwnershipVerificationCertificateArn")]
    pub ownership_verification_certificate_arn: Option<String>,


    /// 
    /// The reference to an AWS-managed certificate that will be used for validating the regional domain name. AWS Certificate Manager is the only supported source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionalCertificateArn")]
    pub regional_certificate_arn: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


/// The mutual TLS authentication configuration for a custom domain name. If specified, API Gateway    performs two-way authentication between the client and the server. Clients must present a    trusted certificate to access your API.
#[derive(Default, serde::Serialize)]
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
    pub truststore_uri: Option<String>,


    /// 
    /// The version of the S3 object that contains your truststore. To specify a version, you must have versioning enabled for the S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TruststoreVersion")]
    pub truststore_version: Option<String>,

}


/// The EndpointConfiguration property type specifies the endpoint types of an Amazon API Gateway domain name.
///
/// EndpointConfiguration is a property of the AWS::ApiGateway::DomainName resource.
#[derive(Default, serde::Serialize)]
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
    pub types: Option<Vec<String>>,

}