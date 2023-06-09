/// Use the AWS::ACMPCA::CertificateAuthority resource to create a private       CA. Once the CA exists, you can use the AWS::ACMPCA::Certificate resource       to issue a new CA certificate. Alternatively, you can issue a CA certificate using an       on-premises CA, and then use the         AWS::ACMPCA::CertificateAuthorityActivation resource to import the new       CA certificate and activate the CA.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_mode: Option<CertificateAuthorityUsageModeEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnCertificateAuthorityarn,

    #[serde(skip_serializing)]
    pub att_certificate_signing_request: CfnCertificateAuthoritycertificatesigningrequest,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificateAuthorityarn;
impl CfnCertificateAuthorityarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificateAuthoritycertificatesigningrequest;
impl CfnCertificateAuthoritycertificatesigningrequest {
    pub fn att_name(&self) -> &'static str {
        r#"CertificateSigningRequest"#
    }
}

impl cfn_resources::CfnResource for CfnCertificateAuthority {
    fn type_string(&self) -> &'static str {
        "AWS::ACMPCA::CertificateAuthority"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csr_extensions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.revocation_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.subject.validate()?;

        Ok(())
    }
}

/// Provides access information used by the authorityInfoAccess and 				subjectInfoAccess extensions described in RFC 5280.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for AccessDescription {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_location.validate()?;

        self.access_method.validate()?;

        Ok(())
    }
}

/// Describes the type and format of extension access. Only one of 				CustomObjectIdentifier or AccessMethodType may be 			provided. Providing both results in InvalidArgsException.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_identifier: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for AccessMethod {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_object_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!("Max validation failed on field 'custom_object_identifier'. {} is greater than 64", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.custom_object_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!("Min validation failed on field 'custom_object_identifier'. {} is less than 0", s.len()));
                }
            }
        }

        Ok(())
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cname: Option<cfn_resources::StrVal>,

    ///
    /// Boolean value that specifies whether certificate revocation lists (CRLs) are enabled.       You can use this value to enable certificate revocation for a new CA when you call the         CreateCertificateAuthority operation or for an existing CA when you       call the UpdateCertificateAuthority operation.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_acl: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CrlConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_cname {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!(
                        "Max validation failed on field 'custom_cname'. {} is greater than 253",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_cname {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_cname'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.expiration_in_days {
            if *the_val > 5000 as _ {
                return Err(format!(
                    "Max validation failed on field 'expiration_in_days'. {} is greater than 5000",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.expiration_in_days {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'expiration_in_days'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.s3_bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_bucket_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_bucket_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 3 as _ {
                    return Err(format!(
                        "Min validation failed on field 's3_bucket_name'. {} is less than 3",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Describes the certificate extensions to be added to the certificate signing request 			(CSR).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_information_access: Option<Vec<AccessDescription>>,
}

impl cfn_resources::CfnResource for CsrExtensions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.key_usage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Defines the X.500 relative distinguished name (RDN).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub object_identifier: cfn_resources::StrVal,

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
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CustomAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.object_identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'object_identifier'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.object_identifier;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'object_identifier'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes an Electronic Data Interchange (EDI) entity as described in as defined in 				Subject Alternative 				Name in RFC 5280.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub name_assigner: cfn_resources::StrVal,

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
    pub party_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EdiPartyName {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name_assigner;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'name_assigner'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.name_assigner;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'name_assigner'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.party_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'party_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.party_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'party_name'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes an ASN.1 X.400 GeneralName as defined in RFC 5280. Only one of 			the following naming options should be provided. Providing more than one option results 			in an InvalidArgsException error.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<cfn_resources::StrVal>,

    ///
    /// Represents GeneralName as an EdiPartyName object.
    ///
    /// Required: No
    ///
    /// Type: EdiPartyName
    ///
    /// Update requires: Replacement
    #[serde(rename = "EdiPartyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<cfn_resources::StrVal>,

    ///
    /// Represents GeneralName using an OtherName object.
    ///
    /// Required: No
    ///
    /// Type: OtherName
    ///
    /// Update requires: Replacement
    #[serde(rename = "OtherName")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc822_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_resource_identifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for GeneralName {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.directory_name
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.dns_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!(
                        "Max validation failed on field 'dns_name'. {} is greater than 253",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.dns_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'dns_name'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.edi_party_name
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.ip_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 39 as _ {
                    return Err(format!(
                        "Max validation failed on field 'ip_address'. {} is greater than 39",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.ip_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ip_address'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.other_name
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.registered_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'registered_id'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.registered_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'registered_id'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.rfc822_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'rfc822_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.rfc822_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'rfc822_name'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.uniform_resource_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!("Max validation failed on field 'uniform_resource_identifier'. {} is greater than 253", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.uniform_resource_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!("Min validation failed on field 'uniform_resource_identifier'. {} is less than 0", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Defines one or more purposes for which the key contained in the certificate can be 			used. Default value for each option is false.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_repudiation: Option<bool>,
}

impl cfn_resources::CfnResource for KeyUsage {
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

/// Contains information to enable and configure Online Certificate Status Protocol (OCSP)       for validating certificate revocation status.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_custom_cname: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for OcspConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ocsp_custom_cname {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!("Max validation failed on field 'ocsp_custom_cname'. {} is greater than 253", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.ocsp_custom_cname {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ocsp_custom_cname'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Defines a custom ASN.1 X.400 GeneralName using an object identifier (OID) 			and value. The OID must satisfy the regular expression shown below. For more 			information, see NIST's definition of Object Identifier 				(OID).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub type_id: cfn_resources::StrVal,

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
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OtherName {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.type_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'type_id'. {} is greater than 64",
                    s.len()
                ));
            }
        }

        let the_val = &self.type_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'type_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Certificate revocation information used by the CreateCertificateAuthority and UpdateCertificateAuthority actions. Your private certificate authority (CA)       can configure Online Certificate Status Protocol (OCSP) support and/or maintain a       certificate revocation list (CRL). OCSP returns validation information about       certificates as requested by clients, and a CRL contains an updated list of certificates       revoked by your CA. For more information, see RevokeCertificate in the AWS Private CA API         Reference and Setting up a certificate         revocation method in the AWS Private CA User         Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_configuration: Option<OcspConfiguration>,
}

impl cfn_resources::CfnResource for RevocationConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.crl_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ocsp_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// ASN1 subject for the certificate authority.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<cfn_resources::StrVal>,

    ///
    /// Two-digit code that specifies the country in which the certificate subject       located.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguished_name_qualifier: Option<cfn_resources::StrVal>,

    ///
    /// Typically a qualifier appended to the name of an individual. Examples include Jr. for       junior, Sr. for senior, and III for third.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GenerationQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_qualifier: Option<cfn_resources::StrVal>,

    ///
    /// First name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GivenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<cfn_resources::StrVal>,

    ///
    /// Concatenation that typically contains the first letter of the GivenName, the first       letter of the middle name if one exists, and the first letter of the SurName.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Initials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<cfn_resources::StrVal>,

    ///
    /// The locality (such as a city or town) in which the certificate subject is       located.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Locality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<cfn_resources::StrVal>,

    ///
    /// Legal name of the organization with which the certificate subject is       affiliated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<cfn_resources::StrVal>,

    ///
    /// A subdivision or unit of the organization (such as sales or finance) with which the       certificate subject is affiliated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<cfn_resources::StrVal>,

    ///
    /// Typically a shortened version of a longer GivenName. For example, Jonathan is often       shortened to John. Elizabeth is often shortened to Beth, Liz, or Eliza.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Pseudonym")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<cfn_resources::StrVal>,

    ///
    /// The certificate serial number.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SerialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<cfn_resources::StrVal>,

    ///
    /// State in which the subject of the certificate is located.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<cfn_resources::StrVal>,

    ///
    /// Family name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Surname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<cfn_resources::StrVal>,

    ///
    /// A personal title such as Mr.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Subject {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_attributes {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_attributes'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

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
