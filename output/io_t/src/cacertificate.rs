

/// Specifies a CA certificate.
#[derive(Default, serde::Serialize)]
pub struct CfnCACertificate {


    /// 
    /// Information about the registration configuration.
    /// 
    /// Required: No
    ///
    /// Type: RegistrationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistrationConfig")]
    pub registration_config: Option<RegistrationConfig>,


    /// 
    /// The certificate data in PEM format.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CACertificatePem")]
    pub cacertificate_pem: String,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The private key verification certificate.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VerificationCertificatePem")]
    pub verification_certificate_pem: Option<String>,


    /// 
    /// The status of the CA certificate.
    /// 
    /// Valid values are "ACTIVE" and "INACTIVE".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,


    /// 
    /// The mode of the CA.
    /// 
    /// All the device certificates that are registered using this CA will be registered      in the same mode as the CA. For more information about certificate mode for device certificates, see certificate mode.
    /// 
    /// Valid values are "DEFAULT" and "SNI_ONLY".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateMode")]
    pub certificate_mode: Option<String>,


    /// 
    /// If true, removes auto registration.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAutoRegistration")]
    pub remove_auto_registration: Option<bool>,


    /// 
    /// Whether the CA certificate is configured for auto registration of device certificates.     Valid values are "ENABLE" and "DISABLE".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoRegistrationStatus")]
    pub auto_registration_status: Option<String>,

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


/// The registration configuration.
#[derive(Default, serde::Serialize)]
pub struct RegistrationConfig {


    /// 
    /// The template body.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,


    /// 
    /// The ARN of the role.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The name of the provisioning template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,

}
