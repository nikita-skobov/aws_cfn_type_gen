
pub mod cfn_account {

#[derive(serde::Serialize, Default)]
pub struct CfnAccount {
    /// No documentation provided by AWS
    #[serde(rename = "ExpiryEventsConfiguration")]
    pub expiry_events_configuration: ExpiryEventsConfiguration,

}


#[derive(serde::Serialize, Default)]
pub struct ExpiryEventsConfiguration {
    #[serde(rename = "DaysBeforeExpiry")]
    pub days_before_expiry: Option<usize>,

}
pub type AccountId = String;

}

pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ValidationMethod")]
    pub validation_method: Option<String>,
    /// List of DomainValidationOption
    #[serde(rename = "DomainValidationOptions")]
    pub domain_validation_options: Option<Vec<DomainValidationOption>>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    pub certificate_transparency_logging_preference: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DomainValidationOption {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "ValidationDomain")]
    pub validation_domain: Option<String>,
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
