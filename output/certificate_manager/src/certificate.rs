/// The AWS::CertificateManager::Certificate resource requests an AWS Certificate Manager (ACM) certificate that you can use to enable secure     connections. For example, you can deploy an ACM certificate to an Elastic Load Balancer to     enable HTTPS support. For more information, see RequestCertificate in     the AWS Certificate Manager API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCertificate {
    ///
    /// The Amazon Resource Name (ARN) of the private certificate authority (CA) that will be used    to issue the certificate. If you do not provide an ARN and you are trying to request a private    certificate, ACM will attempt to issue a public certificate. For more information about    private CAs, see the AWS Private Certificate Authority user guide. The ARN must have the following form:
    ///
    /// arn:aws:acm-pca:region:account:certificate-authority/12345678-1234-1234-1234-123456789012
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:[\w+=/,.@-]+:acm-pca:[\w+=/,.@-]*:[0-9]+:[\w+=,.@-]+(/[\w+=,.@-]+)*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<cfn_resources::StrVal>,

    ///
    /// You can opt out of certificate transparency logging by specifying the       DISABLED option. Opt in by specifying ENABLED.
    ///
    /// If you do not specify a certificate transparency logging preference on a new     CloudFormation template, or if you remove the logging preference from an existing template,     this is the same as explicitly enabling the preference.
    ///
    /// Changing the certificate transparency logging preference will update the existing     resource by calling UpdateCertificateOptions on the certificate. This action     will not create a new resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference:
        Option<CertificateCertificateTransparencyLoggingPreferenceEnum>,

    ///
    /// The fully qualified domain name (FQDN), such as www.example.com, with which you want to     secure an ACM certificate. Use an asterisk (*) to create a wildcard certificate that     protects several sites in the same domain. For example, *.example.com protects       www.example.com, site.example.com, and       images.example.com.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^(\*\.)?(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// Domain information that domain name registrars use to verify your identity.
    ///
    /// ImportantIn order for a AWS::CertificateManager::Certificate to be provisioned and validated       in CloudFormation automatically, the `DomainName` property needs to be identical to one       of the `DomainName` property supplied in DomainValidationOptions, if the       ValidationMethod is **DNS**. Failing to keep them like-for-like will result in failure       to create the domain validation records in Route53.
    ///
    /// Required: No
    ///
    /// Type: List of DomainValidationOption
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidationOption>>,

    ///
    /// Additional FQDNs to be included in the Subject Alternative Name extension of the ACM     certificate. For example, you can add www.example.net to a certificate for which the       DomainName field is www.example.com if users can reach your site by using     either name.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,

    ///
    /// Key-value pairs that can identify the certificate.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The method you want to use to validate that you own or control the domain associated     with a public certificate. You can validate with DNS or validate with       email. We recommend that you use DNS validation.
    ///
    /// If not specified, this property defaults to email validation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DNS | EMAIL
    ///
    /// Update requires: Replacement
    #[serde(rename = "ValidationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<CertificateValidationMethodEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateCertificateTransparencyLoggingPreferenceEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for CertificateCertificateTransparencyLoggingPreferenceEnum {
    fn default() -> Self {
        CertificateCertificateTransparencyLoggingPreferenceEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CertificateValidationMethodEnum {
    /// DNS
    #[serde(rename = "DNS")]
    Dns,

    /// EMAIL
    #[serde(rename = "EMAIL")]
    Email,
}

impl Default for CertificateValidationMethodEnum {
    fn default() -> Self {
        CertificateValidationMethodEnum::Dns
    }
}

impl cfn_resources::CfnResource for CfnCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::CertificateManager::Certificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.certificate_authority_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'certificate_authority_arn'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.certificate_authority_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!("Min validation failed on field 'certificate_authority_arn'. {} is less than 20", s.len()));
                }
            }
        }

        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 253 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain_name'. {} is greater than 253",
                    s.len()
                ));
            }
        }

        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'domain_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.domain_validation_options {
            if the_val.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'domain_validation_options'. {} is greater than 100", the_val.len()));
            }
        }

        if let Some(the_val) = &self.subject_alternative_names {
            if the_val.len() > 100 as _ {
                return Err(format!("Max validation failed on field 'subject_alternative_names'. {} is greater than 100", the_val.len()));
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

/// DomainValidationOption is a property of the AWS::CertificateManager::Certificate resource that specifies the AWS Certificate Manager (ACM) certificate domain to validate. Depending on the     chosen validation method, ACM checks the domain's DNS record for a validation CNAME, or it     attempts to send a validation email message to the domain owner.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DomainValidationOption {
    ///
    /// A fully qualified domain name (FQDN) in the certificate request.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^(\*\.)?(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// The HostedZoneId option, which is available if you are using Route 53 as     your domain registrar, causes ACM to add your CNAME to the domain record. Your list of       DomainValidationOptions must contain one and only one of the     domain-validation options, and the HostedZoneId can be used only when       DNS is specified as your validation method.
    ///
    /// Use the Route 53 ListHostedZones API to discover IDs for available hosted     zones.
    ///
    /// This option is required for publicly trusted certificates.
    ///
    /// NoteThe ListHostedZones API returns IDs in the format       "/hostedzone/Z111111QQQQQQQ", but CloudFormation requires the IDs to be in the format       "Z111111QQQQQQQ".
    ///
    /// When you change your DomainValidationOptions, a new resource is     created.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<cfn_resources::StrVal>,

    ///
    /// The domain name to which you want ACM to send validation emails. This domain name is the     suffix of the email addresses that you want ACM to use. This must be the same as the       DomainName value or a superdomain of the DomainName value. For     example, if you request a certificate for testing.example.com, you can specify       example.com as this value. In that case, ACM sends domain validation emails     to the following five addresses:
    ///
    /// admin@example.com            administrator@example.com            hostmaster@example.com            postmaster@example.com            webmaster@example.com
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^(\*\.)?(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValidationDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_domain: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DomainValidationOption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 253 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain_name'. {} is greater than 253",
                    s.len()
                ));
            }
        }

        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'domain_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.validation_domain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!("Max validation failed on field 'validation_domain'. {} is greater than 253", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.validation_domain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'validation_domain'. {} is less than 1",
                        s.len()
                    ));
                }
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
