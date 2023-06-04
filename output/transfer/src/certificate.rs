/// Imports the signing and encryption certificates that you need to create local (AS2)    profiles and partner    profiles.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnCertificate {
    ///
    /// An optional date that specifies when the certificate becomes active.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveDate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub active_date: Option<cfn_resources::StrVal>,

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
    pub certificate: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub certificate_chain: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// An optional date that specifies when the certificate becomes inactive.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InactiveDate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub inactive_date: Option<cfn_resources::StrVal>,

    ///
    /// The file that contains the private key for the certificate that's being imported.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub private_key: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

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

    #[serde(skip_serializing)]
    pub att_arn: CfnCertificatearn,

    #[serde(skip_serializing)]
    pub att_certificate_id: CfnCertificatecertificateid,

    #[serde(skip_serializing)]
    pub att_not_after_date: CfnCertificatenotafterdate,

    #[serde(skip_serializing)]
    pub att_not_before_date: CfnCertificatenotbeforedate,

    #[serde(skip_serializing)]
    pub att_serial: CfnCertificateserial,

    #[serde(skip_serializing)]
    pub att_status: CfnCertificatestatus,

    #[serde(skip_serializing)]
    pub att_cfn_type: CfnCertificatecfntype,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatearn;
impl CfnCertificatearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatecertificateid;
impl CfnCertificatecertificateid {
    pub fn att_name(&self) -> &'static str {
        r#"CertificateId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatenotafterdate;
impl CfnCertificatenotafterdate {
    pub fn att_name(&self) -> &'static str {
        r#"NotAfterDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatenotbeforedate;
impl CfnCertificatenotbeforedate {
    pub fn att_name(&self) -> &'static str {
        r#"NotBeforeDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificateserial;
impl CfnCertificateserial {
    pub fn att_name(&self) -> &'static str {
        r#"Serial"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatestatus;
impl CfnCertificatestatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCertificatecfntype;
impl CfnCertificatecfntype {
    pub fn att_name(&self) -> &'static str {
        r#"Type"#
    }
}

impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::Transfer::Certificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.certificate;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 16384 as _ {
                return Err(format!(
                    "Max validation failed on field 'certificate'. {} is greater than 16384",
                    s.len()
                ));
            }
        }

        let the_val = &self.certificate;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'certificate'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.certificate_chain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2097152 as _ {
                    return Err(format!("Max validation failed on field 'certificate_chain'. {} is greater than 2097152", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.certificate_chain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'certificate_chain'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 200 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 200",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
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
