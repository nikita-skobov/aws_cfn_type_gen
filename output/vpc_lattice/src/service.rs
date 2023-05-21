

/// Creates a service. A service is any software application that can run on instances  containers, or serverless functions within an account or virtual private cloud (VPC).
///
/// For more information, see Services in the  Amazon VPC Lattice User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnService {


    /// 
    /// The type of IAM policy.
    /// 
    /// NONE: The resource does not use an IAM policy. This is the default.     AWS_IAM: The resource uses an IAM policy. When this type is used, auth is enabled and an auth policy is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,


    /// 
    /// The custom domain name of the service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomDomainName")]
    pub custom_domain_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DnsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsEntry")]
    pub dns_entry: Option<DnsEntry>,


    /// 
    /// The name of the service. The name must be unique within the account. The valid characters    are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or    immediately after another hyphen.
    /// 
    /// If you don't specify a name, CloudFormation generates one. However, if    you specify a name, and later want to replace the resource, you must specify a new    name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,


    /// 
    /// The tags for the service.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

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


/// Describes the DNS information of a service.
#[derive(Default, serde::Serialize)]
pub struct DnsEntry {


    /// 
    /// The ID of the hosted zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,


    /// 
    /// The domain name of the service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

}