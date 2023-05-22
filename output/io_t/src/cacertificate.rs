/// Specifies a CA certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCACertificate {
    ///
    /// Whether the CA certificate is configured for auto registration of device certificates.     Valid values are "ENABLE" and "DISABLE".
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoRegistrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_registration_status: Option<cfn_resources::StrVal>,

    ///
    /// The certificate data in PEM format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CACertificatePem")]
    pub cacertificate_pem: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<cfn_resources::StrVal>,

    ///
    /// Information about the registration configuration.
    ///
    /// Required: No
    ///
    /// Type: RegistrationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,

    ///
    /// If true, removes auto registration.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveAutoRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_auto_registration: Option<bool>,

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
    pub status: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_certificate_pem: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnCACertificatearn,

    #[serde(skip_serializing)]
    pub att_id: CfnCACertificateid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCACertificatearn;
impl CfnCACertificatearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCACertificateid;
impl CfnCACertificateid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnCACertificate {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::CACertificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.registration_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The registration configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RegistrationConfig {
    ///
    /// The ARN of the role.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The template body.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<cfn_resources::StrVal>,

    ///
    /// The name of the provisioning template.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RegistrationConfig {
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
