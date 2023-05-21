

/// Use the AWS::ACMPCA::CertificateAuthority resource to create a private       CA. Once the CA exists, you can use the AWS::ACMPCA::Certificate resource       to issue a new CA certificate. Alternatively, you can issue a CA certificate using an       on-premises CA, and then use the         AWS::ACMPCA::CertificateAuthorityActivation resource to import the new       CA certificate and activate the CA.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificateAuthority {


    /// 
    /// Specifies information to be added to the extension section of the certificate signing 			request (CSR).
    /// 
    /// Required: No
    ///
    /// Type: CsrExtensions
    ///
    /// Update requires: Replacement
    #[serde(rename = "CsrExtensions")]
    pub csr_extensions: Option<CsrExtensions>,


    /// 
    /// Type of the public key algorithm and size, in bits, of the key pair that your CA 			creates when it issues a certificate. When you create a subordinate CA, you must use a 			key algorithm supported by the parent CA.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EC_prime256v1 | EC_secp384r1 | RSA_2048 | RSA_4096
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyAlgorithm")]
    pub key_algorithm: CertificateAuthorityKeyAlgorithmEnum,


    /// 
    /// Specifies a cryptographic key management compliance standard used for handling CA 			keys.
    /// 
    /// Default: FIPS_140_2_LEVEL_3_OR_HIGHER
    /// 
    /// NoteSome AWS Regions do not support the default. When creating a CA in these Regions, you 				must provide FIPS_140_2_LEVEL_2_OR_HIGHER as the argument for 					KeyStorageSecurityStandard. Failure to do this results in an 					InvalidArgsException with the message, "A certificate authority 				cannot be created in this region with the specified security standard."For information about security standard support in various Regions, see Storage 					and security compliance of AWS Private CA private keys.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FIPS_140_2_LEVEL_2_OR_HIGHER | FIPS_140_2_LEVEL_3_OR_HIGHER
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyStorageSecurityStandard")]
    pub key_storage_security_standard: Option<CertificateAuthorityKeyStorageSecurityStandardEnum>,


    /// 
    /// Certificate revocation information used by the CreateCertificateAuthority and UpdateCertificateAuthority actions. Your private certificate authority (CA)       can configure Online Certificate Status Protocol (OCSP) support and/or maintain a       certificate revocation list (CRL). OCSP returns validation information about       certificates as requested by clients, and a CRL contains an updated list of certificates       revoked by your CA. For more information, see RevokeCertificate in the AWS Private CA API         Reference and Setting up a certificate         revocation method in the AWS Private CA User         Guide.
    /// 
    /// NoteThe following requirements apply to revocation configurations.                                                     A configuration disabling CRLs or OCSP must contain only the               Enabled=False parameter, and will fail if other parameters             such as CustomCname or ExpirationInDays are             included.                   In a CRL configuration, the S3BucketName parameter must             conform to the Amazon S3 bucket               naming rules.                   A configuration containing a custom Canonical Name (CNAME) parameter for             CRLs or OCSP must conform to RFC2396 restrictions             on the use of special characters in a CNAME.                    In a CRL or OCSP configuration, the value of a CNAME parameter must not             include a protocol prefix such as "http://" or "https://".
    /// 
    /// Required: No
    ///
    /// Type: RevocationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "RevocationConfiguration")]
    pub revocation_configuration: Option<RevocationConfiguration>,


    /// 
    /// Name of the algorithm your private CA uses to sign certificate requests.
    /// 
    /// This parameter should not be confused with the SigningAlgorithm parameter 			used to sign certificates when they are issued.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SHA256WITHECDSA | SHA256WITHRSA | SHA384WITHECDSA | SHA384WITHRSA | SHA512WITHECDSA | SHA512WITHRSA
    ///
    /// Update requires: Replacement
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: CertificateAuthoritySigningAlgorithmEnum,


    /// 
    /// Structure that contains X.500 distinguished name information for your private 			CA.
    /// 
    /// Required: Yes
    ///
    /// Type: Subject
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subject")]
    pub subject: Subject,


    /// 
    /// Key-value pairs that will be attached to the new private CA. You can associate up to       50 tags with a private CA. For information using tags with IAM to manage permissions,       see Controlling Access Using IAM Tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Type of your private CA.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ROOT | SUBORDINATE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: CertificateAuthorityTypeEnum,


    /// 
    /// Specifies whether the CA issues general-purpose certificates that typically require a 			revocation mechanism, or short-lived certificates that may optionally omit revocation 			because they expire quickly. Short-lived certificate validity is limited to seven 			days.
    /// 
    /// The default value is GENERAL_PURPOSE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GENERAL_PURPOSE | SHORT_LIVED_CERTIFICATE
    ///
    /// Update requires: Replacement
    #[serde(rename = "UsageMode")]
    pub usage_mode: Option<CertificateAuthorityUsageModeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateAuthorityKeyAlgorithmEnum {

    /// EC_prime256v1
    #[serde(rename = "EC_prime256v1")]
    Ecprime256v1,

    /// EC_secp384r1
    #[serde(rename = "EC_secp384r1")]
    Ecsecp384r1,

    /// RSA_2048
    #[serde(rename = "RSA_2048")]
    Rsa2048,

    /// RSA_4096
    #[serde(rename = "RSA_4096")]
    Rsa4096,

}

impl Default for CertificateAuthorityKeyAlgorithmEnum {
    fn default() -> Self {
        CertificateAuthorityKeyAlgorithmEnum::Ecprime256v1
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateAuthorityKeyStorageSecurityStandardEnum {

    /// FIPS_140_2_LEVEL_2_OR_HIGHER
    #[serde(rename = "FIPS_140_2_LEVEL_2_OR_HIGHER")]
    Fips1402level2orhigher,

    /// FIPS_140_2_LEVEL_3_OR_HIGHER
    #[serde(rename = "FIPS_140_2_LEVEL_3_OR_HIGHER")]
    Fips1402level3orhigher,

}

impl Default for CertificateAuthorityKeyStorageSecurityStandardEnum {
    fn default() -> Self {
        CertificateAuthorityKeyStorageSecurityStandardEnum::Fips1402level2orhigher
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateAuthoritySigningAlgorithmEnum {

    /// SHA256WITHECDSA
    #[serde(rename = "SHA256WITHECDSA")]
    Sha256withecdsa,

    /// SHA256WITHRSA
    #[serde(rename = "SHA256WITHRSA")]
    Sha256withrsa,

    /// SHA384WITHECDSA
    #[serde(rename = "SHA384WITHECDSA")]
    Sha384withecdsa,

    /// SHA384WITHRSA
    #[serde(rename = "SHA384WITHRSA")]
    Sha384withrsa,

    /// SHA512WITHECDSA
    #[serde(rename = "SHA512WITHECDSA")]
    Sha512withecdsa,

    /// SHA512WITHRSA
    #[serde(rename = "SHA512WITHRSA")]
    Sha512withrsa,

}

impl Default for CertificateAuthoritySigningAlgorithmEnum {
    fn default() -> Self {
        CertificateAuthoritySigningAlgorithmEnum::Sha256withecdsa
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateAuthorityTypeEnum {

    /// ROOT
    #[serde(rename = "ROOT")]
    Root,

    /// SUBORDINATE
    #[serde(rename = "SUBORDINATE")]
    Subordinate,

}

impl Default for CertificateAuthorityTypeEnum {
    fn default() -> Self {
        CertificateAuthorityTypeEnum::Root
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateAuthorityUsageModeEnum {

    /// GENERAL_PURPOSE
    #[serde(rename = "GENERAL_PURPOSE")]
    Generalpurpose,

    /// SHORT_LIVED_CERTIFICATE
    #[serde(rename = "SHORT_LIVED_CERTIFICATE")]
    Shortlivedcertificate,

}

impl Default for CertificateAuthorityUsageModeEnum {
    fn default() -> Self {
        CertificateAuthorityUsageModeEnum::Generalpurpose
    }
}


impl cfn_resources::CfnResource for CfnCertificateAuthority {
    fn type_string() -> &'static str {
        "AWS::ACMPCA::CertificateAuthority"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Provides access information used by the authorityInfoAccess and 				subjectInfoAccess extensions described in RFC 5280.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessDescription {


    /// 
    /// The location of AccessDescription information.
    /// 
    /// Required: Yes
    ///
    /// Type: GeneralName
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessLocation")]
    pub access_location: GeneralName,


    /// 
    /// The type and format of AccessDescription information.
    /// 
    /// Required: Yes
    ///
    /// Type: AccessMethod
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessMethod")]
    pub access_method: AccessMethod,

}




/// Describes the type and format of extension access. Only one of 				CustomObjectIdentifier or AccessMethodType may be 			provided. Providing both results in InvalidArgsException.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessMethod {


    /// 
    /// Specifies the AccessMethod.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CA_REPOSITORY | RESOURCE_PKI_MANIFEST | RESOURCE_PKI_NOTIFY
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessMethodType")]
    pub access_method_type: Option<AccessMethodAccessMethodTypeEnum>,


    /// 
    /// An object identifier (OID) specifying the AccessMethod. The OID must 			satisfy the regular expression shown below. For more information, see NIST's definition 			of Object Identifier 				(OID).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^([0-2])\.([0-9]|([0-3][0-9]))((\.([0-9]+)){0,126})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomObjectIdentifier")]
    pub custom_object_identifier: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AccessMethodAccessMethodTypeEnum {

    /// CA_REPOSITORY
    #[serde(rename = "CA_REPOSITORY")]
    Carepository,

    /// RESOURCE_PKI_MANIFEST
    #[serde(rename = "RESOURCE_PKI_MANIFEST")]
    Resourcepkimanifest,

    /// RESOURCE_PKI_NOTIFY
    #[serde(rename = "RESOURCE_PKI_NOTIFY")]
    Resourcepkinotify,

}

impl Default for AccessMethodAccessMethodTypeEnum {
    fn default() -> Self {
        AccessMethodAccessMethodTypeEnum::Carepository
    }
}



/// Contains configuration information for a certificate revocation list (CRL). Your 			private certificate authority (CA) creates base CRLs. Delta CRLs are not supported. You 			can enable CRLs for your new or an existing private CA by setting the Enabled parameter to true. Your private CA 			writes CRLs to an S3 bucket that you specify in the S3BucketName parameter. You can hide the name of your bucket by 			specifying a value for the CustomCname parameter. Your 			private CA copies the CNAME or the S3 bucket name to the CRL 				Distribution Points extension of each certificate it issues. Your S3 			bucket policy must give write permission to AWS Private CA.
///
/// AWS Private CA assets that are stored in Amazon S3 can be protected with encryption.  For more information, see Encrypting Your 			CRLs.
///
/// Your private CA uses the value in the ExpirationInDays parameter to calculate the nextUpdate field in the CRL. The CRL is refreshed prior to a 			certificate's expiration date or when a certificate is revoked. When a certificate is 			revoked, it appears in the CRL until the certificate expires, and then in one additional 			CRL after expiration, and it always appears in the audit report.
///
/// A CRL is typically updated approximately 30 minutes after a certificate 	is revoked. If for any reason a CRL update fails, AWS Private CA makes further attempts 	every 15 minutes.
///
/// CRLs contain the following fields:
///
/// Certificate revocation lists created by AWS Private CA are DER-encoded. You can use the 			following OpenSSL command to list a CRL.
///
/// openssl crl -inform DER -text -in crl_path 			-noout
///
/// For more information, see Planning a certificate revocation list 				(CRL) in the         AWS Private Certificate Authority User Guide
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CrlConfiguration {


    /// 
    /// Name inserted into the certificate CRL Distribution         Points extension that enables the use of an alias for the CRL       distribution point. Use this value if you don't want the name of your S3 bucket to be       public.
    /// 
    /// NoteThe content of a Canonical Name (CNAME) record must conform to RFC2396 restrictions on the         use of special characters in URIs. Additionally, the value of the CNAME must not         include a protocol prefix such as "http://" or "https://".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[-a-zA-Z0-9;/?:@&=+$,%_.!~*()']*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomCname")]
    pub custom_cname: Option<String>,


    /// 
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled.       You can use this value to enable certificate revocation for a new CA when you call the         CreateCertificateAuthority operation or for an existing CA when you       call the UpdateCertificateAuthority operation.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// Validity period of the CRL in days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 5000
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExpirationInDays")]
    pub expiration_in_days: Option<i64>,


    /// 
    /// Name of the S3 bucket that contains the CRL. If you do not provide a value for the 				CustomCname argument, the name of your S3 bucket 			is placed into the CRL Distribution Points extension of 			the issued certificate. You can change the name of your bucket by calling the UpdateCertificateAuthority operation. You must specify a bucket 				policy that allows AWS Private CA to write the CRL to your bucket.
    /// 
    /// NoteThe S3BucketName parameter must conform to the S3 					bucket naming rules.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[-a-zA-Z0-9._/]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: Option<String>,


    /// 
    /// Determines whether the CRL will be publicly readable or privately held in the CRL       Amazon S3 bucket. If you choose PUBLIC_READ, the CRL will be accessible over the public       internet. If you choose BUCKET_OWNER_FULL_CONTROL, only the owner of the CRL S3 bucket       can access the CRL, and your PKI clients may need an alternative method of       access.
    /// 
    /// If no value is specified, the default is PUBLIC_READ.
    /// 
    /// Note: This default can cause CA creation to fail in some       circumstances. If you have have enabled the Block Public Access (BPA) feature in your S3       account, then you must specify the value of this parameter as         BUCKET_OWNER_FULL_CONTROL, and not doing so results in an error. If you       have disabled BPA in S3, then you can specify either         BUCKET_OWNER_FULL_CONTROL or PUBLIC_READ as the       value.
    /// 
    /// For more information, see Blocking public access to         the S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3ObjectAcl")]
    pub s3_object_acl: Option<String>,

}




/// Describes the certificate extensions to be added to the certificate signing request 			(CSR).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CsrExtensions {


    /// 
    /// Indicates the purpose of the certificate and of the key contained in the 			certificate.
    /// 
    /// Required: No
    ///
    /// Type: KeyUsage
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<KeyUsage>,


    /// 
    /// For CA certificates, provides a path to additional information pertaining to the CA, 			such as revocation and policy. For more information, see Subject 				Information Access in RFC 5280.
    /// 
    /// Required: No
    ///
    /// Type: List of AccessDescription
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubjectInformationAccess")]
    pub subject_information_access: Option<Vec<AccessDescription>>,

}




/// Defines the X.500 relative distinguished name (RDN).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomAttribute {


    /// 
    /// Specifies the object identifier (OID) of the attribute type of the relative 			distinguished name (RDN).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^([0-2])\.([0-9]|([0-3][0-9]))((\.([0-9]+)){0,126})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ObjectIdentifier")]
    pub object_identifier: String,


    /// 
    /// 
    /// 
    /// Specifies the attribute value of relative distinguished name (RDN).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,

}




/// Describes an Electronic Data Interchange (EDI) entity as described in as defined in 				Subject Alternative 				Name in RFC 5280.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EdiPartyName {


    /// 
    /// Specifies the name assigner.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "NameAssigner")]
    pub name_assigner: String,


    /// 
    /// Specifies the party name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "PartyName")]
    pub party_name: String,

}




/// Describes an ASN.1 X.400 GeneralName as defined in RFC 5280. Only one of 			the following naming options should be provided. Providing more than one option results 			in an InvalidArgsException error.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GeneralName {


    /// 
    /// Contains information about the certificate subject. The certificate can be one issued       by your private certificate authority (CA) or it can be your private CA certificate. The       Subject field in the certificate identifies the entity that owns or controls the public       key in the certificate. The entity can be a user, computer, device, or service. The       Subject must contain an X.500 distinguished name (DN). A DN is a sequence of relative       distinguished names (RDNs). The RDNs are separated by commas in the certificate. The DN       must be unique for each entity, but your private CA can issue more than one certificate       with the same DN to the same entity.
    /// 
    /// Required: No
    ///
    /// Type: Subject
    ///
    /// Update requires: Replacement
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<Subject>,


    /// 
    /// Represents GeneralName as a DNS name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 253
    ///
    /// Update requires: Replacement
    #[serde(rename = "DnsName")]
    pub dns_name: Option<String>,


    /// 
    /// Represents GeneralName as an EdiPartyName object.
    /// 
    /// Required: No
    ///
    /// Type: EdiPartyName
    ///
    /// Update requires: Replacement
    #[serde(rename = "EdiPartyName")]
    pub edi_party_name: Option<EdiPartyName>,


    /// 
    /// Represents GeneralName as an IPv4 or IPv6 address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 39
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,


    /// 
    /// Represents GeneralName using an OtherName object.
    /// 
    /// Required: No
    ///
    /// Type: OtherName
    ///
    /// Update requires: Replacement
    #[serde(rename = "OtherName")]
    pub other_name: Option<OtherName>,


    /// 
    /// Represents GeneralName as an object identifier (OID).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^([0-2])\.([0-9]|([0-3][0-9]))((\.([0-9]+)){0,126})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "RegisteredId")]
    pub registered_id: Option<String>,


    /// 
    /// Represents GeneralName as an RFC 822 email 			address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Rfc822Name")]
    pub rfc822_name: Option<String>,


    /// 
    /// Represents GeneralName as a URI.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 253
    ///
    /// Update requires: Replacement
    #[serde(rename = "UniformResourceIdentifier")]
    pub uniform_resource_identifier: Option<String>,

}




/// Defines one or more purposes for which the key contained in the certificate can be 			used. Default value for each option is false.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KeyUsage {


    /// 
    /// Key can be used to sign CRLs.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "CRLSign")]
    pub crlsign: Option<bool>,


    /// 
    /// Key can be used to decipher data.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataEncipherment")]
    pub data_encipherment: Option<bool>,


    /// 
    /// Key can be used only to decipher data.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DecipherOnly")]
    pub decipher_only: Option<bool>,


    /// 
    /// Key can be used for digital signing.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DigitalSignature")]
    pub digital_signature: Option<bool>,


    /// 
    /// Key can be used only to encipher data.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncipherOnly")]
    pub encipher_only: Option<bool>,


    /// 
    /// Key can be used in a key-agreement protocol.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyAgreement")]
    pub key_agreement: Option<bool>,


    /// 
    /// Key can be used to sign certificates.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyCertSign")]
    pub key_cert_sign: Option<bool>,


    /// 
    /// Key can be used to encipher data.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyEncipherment")]
    pub key_encipherment: Option<bool>,


    /// 
    /// Key can be used for non-repudiation.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "NonRepudiation")]
    pub non_repudiation: Option<bool>,

}




/// Contains information to enable and configure Online Certificate Status Protocol (OCSP)       for validating certificate revocation status.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OcspConfiguration {


    /// 
    /// Flag enabling use of the Online Certificate Status Protocol (OCSP) for validating       certificate revocation status.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// By default, AWS Private CA injects an Amazon domain into certificates being       validated by the Online Certificate Status Protocol (OCSP). A customer can alternatively       use this object to define a CNAME specifying a customized OCSP domain.
    /// 
    /// NoteThe content of a Canonical Name (CNAME) record must conform to RFC2396 restrictions on the         use of special characters in URIs. Additionally, the value of the CNAME must not         include a protocol prefix such as "http://" or "https://".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[-a-zA-Z0-9;/?:@&=+$,%_.!~*()']*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "OcspCustomCname")]
    pub ocsp_custom_cname: Option<String>,

}




/// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID) 			and value. The OID must satisfy the regular expression shown below. For more 			information, see NIST's definition of Object Identifier 				(OID).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OtherName {


    /// 
    /// Specifies an OID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^([0-2])\.([0-9]|([0-3][0-9]))((\.([0-9]+)){0,126})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TypeId")]
    pub type_id: String,


    /// 
    /// Specifies an OID value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,

}




/// Certificate revocation information used by the CreateCertificateAuthority and UpdateCertificateAuthority actions. Your private certificate authority (CA)       can configure Online Certificate Status Protocol (OCSP) support and/or maintain a       certificate revocation list (CRL). OCSP returns validation information about       certificates as requested by clients, and a CRL contains an updated list of certificates       revoked by your CA. For more information, see RevokeCertificate in the AWS Private CA API         Reference and Setting up a certificate         revocation method in the AWS Private CA User         Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RevocationConfiguration {


    /// 
    /// Configuration of the certificate revocation list (CRL), if any, maintained by your       private CA.
    /// 
    /// Required: No
    ///
    /// Type: CrlConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrlConfiguration")]
    pub crl_configuration: Option<CrlConfiguration>,


    /// 
    /// Configuration of Online Certificate Status Protocol (OCSP) support, if any, maintained       by your private CA.
    /// 
    /// Required: No
    ///
    /// Type: OcspConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "OcspConfiguration")]
    pub ocsp_configuration: Option<OcspConfiguration>,

}




/// ASN1 subject for the certificate authority.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Subject {


    /// 
    /// Fully qualified domain name (FQDN) associated with the certificate subject.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CommonName")]
    pub common_name: Option<String>,


    /// 
    /// Two-digit code that specifies the country in which the certificate subject       located.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Country")]
    pub country: Option<String>,


    /// 
    /// 
    /// 
    /// Contains a sequence of one or more X.500 relative distinguished names (RDNs), each of 			which consists of an object identifier (OID) and a value. For more information, see 			NIST’s definition of Object Identifier (OID).
    /// 
    /// NoteCustom attributes cannot be used in combination with standard attributes.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomAttribute
    ///
    /// Maximum: 30
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomAttributes")]
    pub custom_attributes: Option<Vec<CustomAttribute>>,


    /// 
    /// Disambiguating information for the certificate subject.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DistinguishedNameQualifier")]
    pub distinguished_name_qualifier: Option<String>,


    /// 
    /// Typically a qualifier appended to the name of an individual. Examples include Jr. for       junior, Sr. for senior, and III for third.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GenerationQualifier")]
    pub generation_qualifier: Option<String>,


    /// 
    /// First name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GivenName")]
    pub given_name: Option<String>,


    /// 
    /// Concatenation that typically contains the first letter of the GivenName, the first       letter of the middle name if one exists, and the first letter of the SurName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Initials")]
    pub initials: Option<String>,


    /// 
    /// The locality (such as a city or town) in which the certificate subject is       located.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Locality")]
    pub locality: Option<String>,


    /// 
    /// Legal name of the organization with which the certificate subject is       affiliated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Organization")]
    pub organization: Option<String>,


    /// 
    /// A subdivision or unit of the organization (such as sales or finance) with which the       certificate subject is affiliated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationalUnit")]
    pub organizational_unit: Option<String>,


    /// 
    /// Typically a shortened version of a longer GivenName. For example, Jonathan is often       shortened to John. Elizabeth is often shortened to Beth, Liz, or Eliza.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Pseudonym")]
    pub pseudonym: Option<String>,


    /// 
    /// The certificate serial number.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,


    /// 
    /// State in which the subject of the certificate is located.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "State")]
    pub state: Option<String>,


    /// 
    /// Family name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Surname")]
    pub surname: Option<String>,


    /// 
    /// A personal title such as Mr.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Title")]
    pub title: Option<String>,

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


