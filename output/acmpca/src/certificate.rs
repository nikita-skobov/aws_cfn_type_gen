/// The AWS::ACMPCA::Certificate resource is used to issue a certificate       using your private certificate authority. For more information, see the IssueCertificate action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificate {
    ///
    /// Specifies X.509 certificate information to be included in the issued certificate. An         APIPassthrough or APICSRPassthrough template variant must       be selected, or else this parameter is ignored.
    ///
    /// Required: No
    ///
    /// Type: ApiPassthrough
    ///
    /// Update requires: Replacement
    #[serde(rename = "ApiPassthrough")]
    pub api_passthrough: Option<ApiPassthrough>,

    ///
    /// The Amazon Resource Name (ARN) for the private CA issues the certificate.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: String,

    ///
    /// The certificate signing request (CSR) for the certificate.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateSigningRequest")]
    pub certificate_signing_request: String,

    ///
    /// The name of the algorithm that will be used to sign the certificate to be issued.
    ///
    /// This parameter should not be confused with the SigningAlgorithm parameter       used to sign a CSR in the CreateCertificateAuthority action.
    ///
    /// NoteThe specified signing algorithm family (RSA or ECDSA) must match the algorithm         family of the CA's secret key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SHA256WITHECDSA | SHA256WITHRSA | SHA384WITHECDSA | SHA384WITHRSA | SHA512WITHECDSA | SHA512WITHRSA
    ///
    /// Update requires: Replacement
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: CertificateSigningAlgorithmEnum,

    ///
    /// Specifies a custom configuration template to use when issuing a certificate. If this       parameter is not provided, AWS Private CA defaults to the         EndEntityCertificate/V1 template. For more information about AWS Private CA templates, see Using Templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TemplateArn")]
    pub template_arn: Option<String>,

    ///
    /// The period of time during which the certificate will be valid.
    ///
    /// Required: Yes
    ///
    /// Type: Validity
    ///
    /// Update requires: Replacement
    #[serde(rename = "Validity")]
    pub validity: Validity,

    ///
    /// Information describing the start of the validity period of the certificate. This       parameter sets the “Not Before" date for the certificate.
    ///
    /// By default, when issuing a certificate, AWS Private CA sets the "Not       Before" date to the issuance time minus 60 minutes. This compensates for clock       inconsistencies across computer systems. The ValidityNotBefore parameter       can be used to customize the “Not Before” value.
    ///
    /// Unlike the Validity parameter, the ValidityNotBefore       parameter is optional.
    ///
    /// The ValidityNotBefore value is expressed as an explicit date and time,       using the Validity type value ABSOLUTE.
    ///
    /// Required: No
    ///
    /// Type: Validity
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidityNotBefore")]
    pub validity_not_before: Option<Validity>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateSigningAlgorithmEnum {
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

impl Default for CertificateSigningAlgorithmEnum {
    fn default() -> Self {
        CertificateSigningAlgorithmEnum::Sha256withecdsa
    }
}

impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::ACMPCA::Certificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.api_passthrough
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.validity.validate()?;

        self.validity_not_before
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains X.509 certificate information to be placed in an issued certificate. An 				APIPassthrough or APICSRPassthrough template variant must 			be selected, or else this parameter is ignored.
///
/// If conflicting or duplicate certificate information is supplied from other sources, 			AWS Private CA applies order of 				operation rules to determine what information is used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApiPassthrough {
    ///
    /// Specifies X.509 extension information for a certificate.
    ///
    /// Required: No
    ///
    /// Type: Extensions
    ///
    /// Update requires: Replacement
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,

    ///
    /// Contains information about the certificate subject. The Subject field in the       certificate identifies the entity that owns or controls the public key in the       certificate. The entity can be a user, computer, device, or service. The Subject must       contain an X.500 distinguished name (DN). A DN is a sequence of relative distinguished       names (RDNs). The RDNs are separated by commas in the certificate.
    ///
    /// Required: No
    ///
    /// Type: Subject
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subject")]
    pub subject: Option<Subject>,
}

impl cfn_resources::CfnResource for ApiPassthrough {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.extensions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.subject.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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

impl cfn_resources::CfnResource for CustomAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.object_identifier;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'object_identifier'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.object_identifier;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'object_identifier'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

///
/// Specifies the X.509 extension information for a certificate.
///
/// Extensions present in CustomExtensions follow the 				ApiPassthrough       template 				rules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomExtension {
    ///
    ///
    ///
    /// Specifies the critical flag of the X.509 extension.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Critical")]
    pub critical: Option<bool>,

    ///
    ///
    ///
    /// Specifies the object identifier (OID) of the X.509 extension. For more information, 			see the Global OID reference database.
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
    /// Specifies the base64-encoded value of the X.509 extension.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for CustomExtension {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.object_identifier;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'object_identifier'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.object_identifier;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'object_identifier'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 4096 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 4096",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for EdiPartyName {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name_assigner;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'name_assigner'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.name_assigner;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'name_assigner'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.party_name;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'party_name'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.party_name;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'party_name'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Specifies additional purposes for which the certified public key may be used other 			than basic purposes indicated in the KeyUsage extension.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExtendedKeyUsage {
    ///
    /// Specifies a custom ExtendedKeyUsage with an object identifier 			(OID).
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
    #[serde(rename = "ExtendedKeyUsageObjectIdentifier")]
    pub extended_key_usage_object_identifier: Option<String>,

    ///
    /// Specifies a standard ExtendedKeyUsage as defined as in RFC 				5280.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CERTIFICATE_TRANSPARENCY | CLIENT_AUTH | CODE_SIGNING | DOCUMENT_SIGNING | EMAIL_PROTECTION | OCSP_SIGNING | SERVER_AUTH | SMART_CARD_LOGIN | TIME_STAMPING
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExtendedKeyUsageType")]
    pub extended_key_usage_type: Option<ExtendedKeyUsageExtendedKeyUsageTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ExtendedKeyUsageExtendedKeyUsageTypeEnum {
    /// CERTIFICATE_TRANSPARENCY
    #[serde(rename = "CERTIFICATE_TRANSPARENCY")]
    Certificatetransparency,

    /// CLIENT_AUTH
    #[serde(rename = "CLIENT_AUTH")]
    Clientauth,

    /// CODE_SIGNING
    #[serde(rename = "CODE_SIGNING")]
    Codesigning,

    /// DOCUMENT_SIGNING
    #[serde(rename = "DOCUMENT_SIGNING")]
    Documentsigning,

    /// EMAIL_PROTECTION
    #[serde(rename = "EMAIL_PROTECTION")]
    Emailprotection,

    /// OCSP_SIGNING
    #[serde(rename = "OCSP_SIGNING")]
    Ocspsigning,

    /// SERVER_AUTH
    #[serde(rename = "SERVER_AUTH")]
    Serverauth,

    /// SMART_CARD_LOGIN
    #[serde(rename = "SMART_CARD_LOGIN")]
    Smartcardlogin,

    /// TIME_STAMPING
    #[serde(rename = "TIME_STAMPING")]
    Timestamping,
}

impl Default for ExtendedKeyUsageExtendedKeyUsageTypeEnum {
    fn default() -> Self {
        ExtendedKeyUsageExtendedKeyUsageTypeEnum::Certificatetransparency
    }
}

impl cfn_resources::CfnResource for ExtendedKeyUsage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.extended_key_usage_object_identifier {
            if the_val.len() > 64 as _ {
                return Err(format!("Max validation failed on field 'extended_key_usage_object_identifier'. {} is greater than 64", the_val.len()));
            }
        }

        if let Some(the_val) = &self.extended_key_usage_object_identifier {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'extended_key_usage_object_identifier'. {} is less than 0", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Contains X.509 extension information for a certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Extensions {
    ///
    /// Contains a sequence of one or more policy information terms, each of which consists of 			an object identifier (OID) and optional qualifiers. For more information, see NIST's 			definition of Object 				Identifier (OID).
    ///
    /// In an end-entity certificate, these terms indicate the policy under which the 			certificate was issued and the purposes for which it may be used. In a CA certificate, 			these terms limit the set of policies for certification paths that include this 			certificate.
    ///
    /// Required: No
    ///
    /// Type: List of PolicyInformation
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificatePolicies")]
    pub certificate_policies: Option<Vec<PolicyInformation>>,

    ///
    ///
    ///
    /// Contains a sequence of one or more X.509 extensions, each of which consists of an 			object identifier (OID), a base64-encoded value, and the critical flag. For more 			information, see the Global OID reference 				database.
    ///
    /// Required: No
    ///
    /// Type: List of CustomExtension
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomExtensions")]
    pub custom_extensions: Option<Vec<CustomExtension>>,

    ///
    /// Specifies additional purposes for which the certified public key may be used other 			than basic purposes indicated in the KeyUsage extension.
    ///
    /// Required: No
    ///
    /// Type: List of ExtendedKeyUsage
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExtendedKeyUsage")]
    pub extended_key_usage: Option<Vec<ExtendedKeyUsage>>,

    ///
    /// Defines one or more purposes for which the key contained in the certificate can be       used. Default value for each option is false.
    ///
    /// Required: No
    ///
    /// Type: KeyUsage
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<KeyUsage>,

    ///
    /// The subject alternative name extension allows identities to be bound to the subject of 			the certificate. These identities may be included in addition to or in place of the 			identity in the subject field of the certificate.
    ///
    /// Required: No
    ///
    /// Type: List of GeneralName
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<Vec<GeneralName>>,
}

impl cfn_resources::CfnResource for Extensions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.certificate_policies {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'certificate_policies'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_extensions {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_extensions'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.extended_key_usage {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'extended_key_usage'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        self.key_usage
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.subject_alternative_names {
            if the_val.len() > 20 as _ {
                return Err(format!("Max validation failed on field 'subject_alternative_names'. {} is greater than 20", the_val.len()));
            }
        }

        Ok(())
    }
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
            if the_val.len() > 253 as _ {
                return Err(format!(
                    "Max validation failed on field 'dns_name'. {} is greater than 253",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.dns_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'dns_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.edi_party_name
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.ip_address {
            if the_val.len() > 39 as _ {
                return Err(format!(
                    "Max validation failed on field 'ip_address'. {} is greater than 39",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.ip_address {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'ip_address'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.other_name
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.registered_id {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'registered_id'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.registered_id {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'registered_id'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rfc822_name {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'rfc822_name'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.rfc822_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'rfc822_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.uniform_resource_identifier {
            if the_val.len() > 253 as _ {
                return Err(format!("Max validation failed on field 'uniform_resource_identifier'. {} is greater than 253", the_val.len()));
            }
        }

        if let Some(the_val) = &self.uniform_resource_identifier {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'uniform_resource_identifier'. {} is less than 0", the_val.len()));
            }
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for OtherName {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.type_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'type_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.type_id;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'type_id'. {} is less than 0",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Defines the X.509 CertificatePolicies extension.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyInformation {
    ///
    /// Specifies the object identifier (OID) of the certificate policy under which the 			certificate was issued. For more information, see NIST's definition of Object Identifier 				(OID).
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
    #[serde(rename = "CertPolicyId")]
    pub cert_policy_id: String,

    ///
    /// Modifies the given CertPolicyId with a qualifier. AWS Private CA supports the 			certification practice statement (CPS) qualifier.
    ///
    /// Required: No
    ///
    /// Type: List of PolicyQualifierInfo
    ///
    /// Maximum: 20
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyQualifiers")]
    pub policy_qualifiers: Option<Vec<PolicyQualifierInfo>>,
}

impl cfn_resources::CfnResource for PolicyInformation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cert_policy_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'cert_policy_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.cert_policy_id;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'cert_policy_id'. {} is less than 0",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.policy_qualifiers {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'policy_qualifiers'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Modifies the CertPolicyId of a PolicyInformation object with 			a qualifier. AWS Private CA supports the certification practice statement (CPS) 			qualifier.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyQualifierInfo {
    ///
    /// Identifies the qualifier modifying a CertPolicyId.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CPS
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyQualifierId")]
    pub policy_qualifier_id: PolicyQualifierInfoPolicyQualifierIdEnum,

    ///
    /// Defines the qualifier type. AWS Private CA supports the use of a URI for a CPS qualifier 			in this field.
    ///
    /// Required: Yes
    ///
    /// Type: Qualifier
    ///
    /// Update requires: Replacement
    #[serde(rename = "Qualifier")]
    pub qualifier: Qualifier,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PolicyQualifierInfoPolicyQualifierIdEnum {
    /// CPS
    #[serde(rename = "CPS")]
    Cps,
}

impl Default for PolicyQualifierInfoPolicyQualifierIdEnum {
    fn default() -> Self {
        PolicyQualifierInfoPolicyQualifierIdEnum::Cps
    }
}

impl cfn_resources::CfnResource for PolicyQualifierInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.qualifier.validate()?;

        Ok(())
    }
}

/// Defines a PolicyInformation qualifier. AWS Private CA supports the certification 				practice statement (CPS) qualifier defined in RFC 5280.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Qualifier {
    ///
    /// Contains a pointer to a certification practice statement (CPS) published by the 			CA.
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
    #[serde(rename = "CpsUri")]
    pub cps_uri: String,
}

impl cfn_resources::CfnResource for Qualifier {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cps_uri;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'cps_uri'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.cps_uri;

        if the_val.len() < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'cps_uri'. {} is less than 0",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Contains information about the certificate subject. The Subject field in       the certificate identifies the entity that owns or controls the public key in the       certificate. The entity can be a user, computer, device, or service. The Subject       must contain an X.500 distinguished name (DN). A DN is a sequence of relative       distinguished names (RDNs). The RDNs are separated by commas in the certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Subject {
    ///
    /// For CA and end-entity certificates in a private PKI, the common name (CN) can be any       string within the length limit.
    ///
    /// Note: In publicly trusted certificates, the common name must be a fully qualified       domain name (FQDN) associated with the certificate subject.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
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
    /// Minimum: 2
    ///
    /// Maximum: 2
    ///
    /// Pattern: [A-Za-z]{2}
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
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: [a-zA-Z0-9'()+-.?:/= ]*
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
    /// Minimum: 0
    ///
    /// Maximum: 3
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
    /// Minimum: 0
    ///
    /// Maximum: 16
    ///
    /// Update requires: Replacement
    #[serde(rename = "GivenName")]
    pub given_name: Option<String>,

    ///
    /// Concatenation that typically contains the first letter of the GivenName, the first letter of the middle name if one exists, and the       first letter of the Surname.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 5
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
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "Locality")]
    pub locality: Option<String>,

    ///
    /// Legal name of the organization with which the certificate subject is affiliated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
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
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "OrganizationalUnit")]
    pub organizational_unit: Option<String>,

    ///
    /// Typically a shortened version of a longer GivenName.       For example, Jonathan is often shortened to John. Elizabeth is often shortened to Beth,       Liz, or Eliza.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 128
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
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Pattern: [a-zA-Z0-9'()+-.?:/= ]*
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
    /// Minimum: 0
    ///
    /// Maximum: 128
    ///
    /// Update requires: Replacement
    #[serde(rename = "State")]
    pub state: Option<String>,

    ///
    /// Family name. In the US and the UK, for example, the surname of an individual is       ordered last. In Asian cultures the surname is typically ordered first.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 40
    ///
    /// Update requires: Replacement
    #[serde(rename = "Surname")]
    pub surname: Option<String>,

    ///
    /// A title such as Mr. or Ms., which is pre-pended to the name to refer formally to the       certificate subject.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "Title")]
    pub title: Option<String>,
}

impl cfn_resources::CfnResource for Subject {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.common_name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'common_name'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.common_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'common_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.country {
            if the_val.len() > 2 as _ {
                return Err(format!(
                    "Max validation failed on field 'country'. {} is greater than 2",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.country {
            if the_val.len() < 2 as _ {
                return Err(format!(
                    "Min validation failed on field 'country'. {} is less than 2",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_attributes {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_attributes'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.distinguished_name_qualifier {
            if the_val.len() > 64 as _ {
                return Err(format!("Max validation failed on field 'distinguished_name_qualifier'. {} is greater than 64", the_val.len()));
            }
        }

        if let Some(the_val) = &self.distinguished_name_qualifier {
            if the_val.len() < 0 as _ {
                return Err(format!("Min validation failed on field 'distinguished_name_qualifier'. {} is less than 0", the_val.len()));
            }
        }

        if let Some(the_val) = &self.generation_qualifier {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'generation_qualifier'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.generation_qualifier {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'generation_qualifier'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.given_name {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'given_name'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.given_name {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'given_name'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.initials {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'initials'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.initials {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'initials'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.locality {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'locality'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.locality {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'locality'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.organization {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'organization'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.organization {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'organization'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.organizational_unit {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'organizational_unit'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.organizational_unit {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'organizational_unit'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.pseudonym {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'pseudonym'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.pseudonym {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'pseudonym'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.serial_number {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'serial_number'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.serial_number {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'serial_number'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.state {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'state'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.state {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'state'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.surname {
            if the_val.len() > 40 as _ {
                return Err(format!(
                    "Max validation failed on field 'surname'. {} is greater than 40",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.surname {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'surname'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.title {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'title'. {} is greater than 64",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.title {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'title'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Length of time for which the certificate issued by your private certificate authority       (CA), or by the private CA itself, is valid in days, months, or years. You can issue a       certificate by calling the IssueCertificate operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Validity {
    ///
    /// Specifies whether the Value parameter represents days, months, or       years.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ABSOLUTE | DAYS | END_DATE | MONTHS | YEARS
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: ValidityTypeEnum,

    ///
    /// A long integer interpreted according to the value of Type, below.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: f64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ValidityTypeEnum {
    /// ABSOLUTE
    #[serde(rename = "ABSOLUTE")]
    Absolute,

    /// DAYS
    #[serde(rename = "DAYS")]
    Days,

    /// END_DATE
    #[serde(rename = "END_DATE")]
    Enddate,

    /// MONTHS
    #[serde(rename = "MONTHS")]
    Months,

    /// YEARS
    #[serde(rename = "YEARS")]
    Years,
}

impl Default for ValidityTypeEnum {
    fn default() -> Self {
        ValidityTypeEnum::Absolute
    }
}

impl cfn_resources::CfnResource for Validity {
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
