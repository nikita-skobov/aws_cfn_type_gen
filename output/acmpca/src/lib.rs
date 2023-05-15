
pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// The time after which the Certificate will be valid.
    #[serde(rename = "ValidityNotBefore")]
    pub validity_not_before: Option<Validity>,
    /// Specifies a custom configuration template to use when issuing a certificate. If this parameter is not provided, ACM Private CA defaults to the EndEntityCertificate/V1 template.
    #[serde(rename = "TemplateArn")]
    pub template_arn: Option<Arn>,
    /// The Amazon Resource Name (ARN) for the private CA to issue the certificate.
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: Arn,
    /// These are fields to be overridden in a certificate at the time of issuance. These requires an API_Passthrough template be used or they will be ignored.
    #[serde(rename = "ApiPassthrough")]
    pub api_passthrough: Option<ApiPassthrough>,
    /// The time before which the Certificate will be valid.
    #[serde(rename = "Validity")]
    pub validity: Validity,
    /// The name of the algorithm that will be used to sign the Certificate.
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
    /// The certificate signing request (CSR) for the Certificate.
    #[serde(rename = "CertificateSigningRequest")]
    pub certificate_signing_request: String,

}


#[derive(serde::Serialize, Default)]
pub struct ExtendedKeyUsageList {

}

#[derive(serde::Serialize, Default)]
pub struct PolicyInformation {
    #[serde(rename = "PolicyQualifiers")]
    pub policy_qualifiers: Option<PolicyQualifierInfoList>,
    #[serde(rename = "CertPolicyId")]
    pub cert_policy_id: CustomObjectIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct CustomExtensionList {

}
pub type CustomObjectIdentifier = String;pub type DnsName = String;
#[derive(serde::Serialize, Default)]
pub struct EdiPartyName {
    #[serde(rename = "PartyName")]
    pub party_name: String,
    #[serde(rename = "NameAssigner")]
    pub name_assigner: String,

}

#[derive(serde::Serialize, Default)]
pub struct GeneralNameList {

}

#[derive(serde::Serialize, Default)]
pub struct Validity {
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct PolicyQualifierInfoList {

}

#[derive(serde::Serialize, Default)]
pub struct ExtendedKeyUsage {
    #[serde(rename = "ExtendedKeyUsageObjectIdentifier")]
    pub extended_key_usage_object_identifier: Option<CustomObjectIdentifier>,
    #[serde(rename = "ExtendedKeyUsageType")]
    pub extended_key_usage_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Extensions {
    #[serde(rename = "ExtendedKeyUsage")]
    pub extended_key_usage: Option<ExtendedKeyUsageList>,
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<GeneralNameList>,
    #[serde(rename = "CustomExtensions")]
    pub custom_extensions: Option<CustomExtensionList>,
    #[serde(rename = "CertificatePolicies")]
    pub certificate_policies: Option<CertificatePolicyList>,
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<KeyUsage>,

}

#[derive(serde::Serialize, Default)]
pub struct Qualifier {
    #[serde(rename = "CpsUri")]
    pub cps_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct CustomAttribute {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "ObjectIdentifier")]
    pub object_identifier: CustomObjectIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct OtherName {
    #[serde(rename = "TypeId")]
    pub type_id: CustomObjectIdentifier,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type UniformResourceIdentifier = String;
#[derive(serde::Serialize, Default)]
pub struct CustomExtension {
    #[serde(rename = "Critical")]
    pub critical: Option<bool>,
    #[serde(rename = "ObjectIdentifier")]
    pub object_identifier: CustomObjectIdentifier,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type IpAddress = String;
#[derive(serde::Serialize, Default)]
pub struct ApiPassthrough {
    #[serde(rename = "Subject")]
    pub subject: Option<Subject>,
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct CertificatePolicyList {

}

#[derive(serde::Serialize, Default)]
pub struct CustomAttributeList {

}

#[derive(serde::Serialize, Default)]
pub struct GeneralName {
    #[serde(rename = "UniformResourceIdentifier")]
    pub uniform_resource_identifier: Option<UniformResourceIdentifier>,
    #[serde(rename = "EdiPartyName")]
    pub edi_party_name: Option<EdiPartyName>,
    #[serde(rename = "Rfc822Name")]
    pub rfc822_name: Option<Rfc822Name>,
    #[serde(rename = "RegisteredId")]
    pub registered_id: Option<CustomObjectIdentifier>,
    #[serde(rename = "OtherName")]
    pub other_name: Option<OtherName>,
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<Subject>,
    #[serde(rename = "DnsName")]
    pub dns_name: Option<DnsName>,
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<IpAddress>,

}

#[derive(serde::Serialize, Default)]
pub struct KeyUsage {
    #[serde(rename = "KeyAgreement")]
    pub key_agreement: Option<bool>,
    #[serde(rename = "KeyEncipherment")]
    pub key_encipherment: Option<bool>,
    #[serde(rename = "KeyCertSign")]
    pub key_cert_sign: Option<bool>,
    #[serde(rename = "CRLSign")]
    pub crlsign: Option<bool>,
    #[serde(rename = "DecipherOnly")]
    pub decipher_only: Option<bool>,
    #[serde(rename = "DigitalSignature")]
    pub digital_signature: Option<bool>,
    #[serde(rename = "NonRepudiation")]
    pub non_repudiation: Option<bool>,
    #[serde(rename = "EncipherOnly")]
    pub encipher_only: Option<bool>,
    #[serde(rename = "DataEncipherment")]
    pub data_encipherment: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PolicyQualifierInfo {
    #[serde(rename = "PolicyQualifierId")]
    pub policy_qualifier_id: String,
    #[serde(rename = "Qualifier")]
    pub qualifier: Qualifier,

}

#[derive(serde::Serialize, Default)]
pub struct Subject {
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Surname")]
    pub surname: Option<String>,
    #[serde(rename = "CustomAttributes")]
    pub custom_attributes: Option<CustomAttributeList>,
    #[serde(rename = "GivenName")]
    pub given_name: Option<String>,
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "Initials")]
    pub initials: Option<String>,
    #[serde(rename = "Locality")]
    pub locality: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Organization")]
    pub organization: Option<String>,
    #[serde(rename = "DistinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(rename = "GenerationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(rename = "OrganizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "Pseudonym")]
    pub pseudonym: Option<String>,

}
pub type Rfc822Name = String;

}

pub mod cfn_certificate_authority {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificateAuthority {
    /// Public key algorithm and size, in bits, of the key pair that your CA creates when it issues a certificate.
    #[serde(rename = "KeyAlgorithm")]
    pub key_algorithm: String,
    /// Structure that contains X.500 distinguished name information for your CA.
    #[serde(rename = "Subject")]
    pub subject: Subject,
    /// KeyStorageSecurityStadard defines a cryptographic key management compliance standard used for handling CA keys.
    #[serde(rename = "KeyStorageSecurityStandard")]
    pub key_storage_security_standard: Option<String>,
    /// Usage mode of the ceritificate authority.
    #[serde(rename = "UsageMode")]
    pub usage_mode: Option<String>,
    /// Structure that contains CSR pass through extension information used by the CreateCertificateAuthority action.
    #[serde(rename = "CsrExtensions")]
    pub csr_extensions: Option<CsrExtensions>,
    /// Certificate revocation information used by the CreateCertificateAuthority and UpdateCertificateAuthority actions.
    #[serde(rename = "RevocationConfiguration")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The type of the certificate authority.
    #[serde(rename = "Type")]
    pub ty: String,
    /// Algorithm your CA uses to sign certificate requests.
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,

}


#[derive(serde::Serialize, Default)]
pub struct KeyUsage {
    #[serde(rename = "DigitalSignature")]
    pub digital_signature: Option<bool>,
    #[serde(rename = "CRLSign")]
    pub crlsign: Option<bool>,
    #[serde(rename = "KeyCertSign")]
    pub key_cert_sign: Option<bool>,
    #[serde(rename = "DataEncipherment")]
    pub data_encipherment: Option<bool>,
    #[serde(rename = "NonRepudiation")]
    pub non_repudiation: Option<bool>,
    #[serde(rename = "KeyEncipherment")]
    pub key_encipherment: Option<bool>,
    #[serde(rename = "EncipherOnly")]
    pub encipher_only: Option<bool>,
    #[serde(rename = "DecipherOnly")]
    pub decipher_only: Option<bool>,
    #[serde(rename = "KeyAgreement")]
    pub key_agreement: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomAttribute {
    #[serde(rename = "ObjectIdentifier")]
    pub object_identifier: CustomObjectIdentifier,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type IpAddress = String;
#[derive(serde::Serialize, Default)]
pub struct EdiPartyName {
    #[serde(rename = "NameAssigner")]
    pub name_assigner: String,
    #[serde(rename = "PartyName")]
    pub party_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct RevocationConfiguration {
    #[serde(rename = "OcspConfiguration")]
    pub ocsp_configuration: Option<OcspConfiguration>,
    #[serde(rename = "CrlConfiguration")]
    pub crl_configuration: Option<CrlConfiguration>,

}
pub type AccessMethodType = String;
#[derive(serde::Serialize, Default)]
pub struct CrlConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "CustomCname")]
    pub custom_cname: Option<String>,
    #[serde(rename = "ExpirationInDays")]
    pub expiration_in_days: Option<usize>,
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3ObjectAcl")]
    pub s3_object_acl: Option<String>,

}
pub type UniformResourceIdentifier = String;
#[derive(serde::Serialize, Default)]
pub struct OtherName {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "TypeId")]
    pub type_id: CustomObjectIdentifier,

}

#[derive(serde::Serialize, Default)]
pub struct CustomAttributeList {

}

#[derive(serde::Serialize, Default)]
pub struct OcspConfiguration {
    #[serde(rename = "OcspCustomCname")]
    pub ocsp_custom_cname: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Subject {
    #[serde(rename = "GenerationQualifier")]
    pub generation_qualifier: Option<String>,
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,
    #[serde(rename = "Organization")]
    pub organization: Option<String>,
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    #[serde(rename = "Locality")]
    pub locality: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "Pseudonym")]
    pub pseudonym: Option<String>,
    #[serde(rename = "CustomAttributes")]
    pub custom_attributes: Option<CustomAttributeList>,
    #[serde(rename = "DistinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(rename = "OrganizationalUnit")]
    pub organizational_unit: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Surname")]
    pub surname: Option<String>,
    #[serde(rename = "Initials")]
    pub initials: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "GivenName")]
    pub given_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GeneralName {
    #[serde(rename = "DnsName")]
    pub dns_name: Option<DnsName>,
    #[serde(rename = "EdiPartyName")]
    pub edi_party_name: Option<EdiPartyName>,
    #[serde(rename = "UniformResourceIdentifier")]
    pub uniform_resource_identifier: Option<UniformResourceIdentifier>,
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<Subject>,
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<IpAddress>,
    #[serde(rename = "Rfc822Name")]
    pub rfc822_name: Option<Rfc822Name>,
    #[serde(rename = "RegisteredId")]
    pub registered_id: Option<CustomObjectIdentifier>,
    #[serde(rename = "OtherName")]
    pub other_name: Option<OtherName>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}
pub type Arn = String;pub type CustomObjectIdentifier = String;
#[derive(serde::Serialize, Default)]
pub struct AccessDescription {
    #[serde(rename = "AccessLocation")]
    pub access_location: GeneralName,
    #[serde(rename = "AccessMethod")]
    pub access_method: AccessMethod,

}

#[derive(serde::Serialize, Default)]
pub struct SubjectInformationAccess {

}

#[derive(serde::Serialize, Default)]
pub struct AccessMethod {
    #[serde(rename = "CustomObjectIdentifier")]
    pub custom_object_identifier: Option<CustomObjectIdentifier>,
    #[serde(rename = "AccessMethodType")]
    pub access_method_type: Option<AccessMethodType>,

}
pub type Rfc822Name = String;pub type DnsName = String;
#[derive(serde::Serialize, Default)]
pub struct CsrExtensions {
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<KeyUsage>,
    #[serde(rename = "SubjectInformationAccess")]
    pub subject_information_access: Option<SubjectInformationAccess>,

}


}

pub mod cfn_certificate_authority_activation {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificateAuthorityActivation {
    /// The status of the Certificate Authority.
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// Certificate Authority certificate that will be installed in the Certificate Authority.
    #[serde(rename = "Certificate")]
    pub certificate: String,
    /// Arn of the Certificate Authority.
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,

}



}

pub mod cfn_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnPermission {
    /// The AWS service or identity that receives the permission. At this time, the only valid principal is acm.amazonaws.com.
    #[serde(rename = "Principal")]
    pub principal: String,
    /// The ID of the calling account.
    #[serde(rename = "SourceAccount")]
    pub source_account: Option<String>,
    /// The Amazon Resource Name (ARN) of the Private Certificate Authority that grants the permission.
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,
    /// The actions that the specified AWS service principal can use. Actions IssueCertificate, GetCertificate and ListPermissions must be provided.
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,

}



}
