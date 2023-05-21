

/// Imports the signing and encryption certificates that you need to create local (AS2)    profiles and partner    profiles.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificate {


    /// 
    /// Specifies whether this certificate is used for signing or encryption.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ENCRYPTION | SIGNING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Usage")]
    pub usage: CertificateUsageEnum,


    /// 
    /// Key-value pairs that can be used to group and search for certificates.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// An optional date that specifies when the certificate becomes inactive.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InactiveDate")]
    pub inactive_date: Option<String>,


    /// 
    /// The file name for the certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16384
    ///
    /// Pattern: ^[\u0009\u000A\u000D\u0020-\u00FF]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Certificate")]
    pub certificate: String,


    /// 
    /// The list of certificates that make up the chain for the certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2097152
    ///
    /// Pattern: ^[\u0009\u000A\u000D\u0020-\u00FF]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: Option<String>,


    /// 
    /// The file that contains the private key for the certificate that's being imported.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,


    /// 
    /// An optional date that specifies when the certificate becomes active.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveDate")]
    pub active_date: Option<String>,


    /// 
    /// The name or description that's used to identity the certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^[\p{Graph}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateUsageEnum {

    /// ENCRYPTION
    #[serde(rename = "ENCRYPTION")]
    Encryption,

    /// SIGNING
    #[serde(rename = "SIGNING")]
    Signing,

}

impl Default for CertificateUsageEnum {
    fn default() -> Self {
        CertificateUsageEnum::Encryption
    }
}


impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string() -> &'static str {
        "AWS::Transfer::Certificate"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
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


