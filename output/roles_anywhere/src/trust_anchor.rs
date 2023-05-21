/// Creates a trust anchor to establish trust between IAM Roles Anywhere and     your certificate authority (CA). You can define a trust anchor as a reference to an AWS Private Certificate Authority (AWS Private CA) or      by uploading a CA certificate. Your AWS workloads can authenticate with the trust anchor using certificates issued by     the CA in exchange for temporary AWS credentials.
///
/// Required permissions: rolesanywhere:CreateTrustAnchor.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrustAnchor {
    ///
    /// Indicates whether the trust anchor is enabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

    ///
    /// The name of the trust anchor.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[ a-zA-Z0-9-_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// The trust anchor type and its related certificate data.
    ///
    /// Required: Yes
    ///
    /// Type: Source
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: Source,

    ///
    /// The tags to attach to the trust anchor.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnTrustAnchor {
    fn type_string(&self) -> &'static str {
        "AWS::RolesAnywhere::TrustAnchor"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        self.source.validate()?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The trust anchor type and its related certificate data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Source {
    ///
    /// The data field of the trust anchor depending on its type.
    ///
    /// Required: No
    ///
    /// Type: SourceData
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceData")]
    pub source_data: Option<SourceData>,

    ///
    /// The type of the TrustAnchor.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AWS_ACM_PCA | CERTIFICATE_BUNDLE | SELF_SIGNED_REPOSITORY
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceType")]
    pub source_type: Option<SourceSourceTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceSourceTypeEnum {
    /// AWS_ACM_PCA
    #[serde(rename = "AWS_ACM_PCA")]
    Awsacmpca,

    /// CERTIFICATE_BUNDLE
    #[serde(rename = "CERTIFICATE_BUNDLE")]
    Certificatebundle,

    /// SELF_SIGNED_REPOSITORY
    #[serde(rename = "SELF_SIGNED_REPOSITORY")]
    Selfsignedrepository,
}

impl Default for SourceSourceTypeEnum {
    fn default() -> Self {
        SourceSourceTypeEnum::Awsacmpca
    }
}

impl cfn_resources::CfnResource for Source {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.source_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The data field of the trust anchor depending on its type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceData {
    ///
    /// The root certificate of the AWS Private Certificate Authority specified by this ARN is used in trust     validation for temporary credential requests. Included for trust anchors of type AWS_ACM_PCA.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcmPcaArn")]
    pub acm_pca_arn: Option<String>,

    ///
    /// The PEM-encoded data for the certificate anchor. Included for trust anchors of type CERTIFICATE_BUNDLE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 8000
    ///
    /// Update requires: No interruption
    #[serde(rename = "X509CertificateData")]
    pub x509_certificate_data: Option<String>,
}

impl cfn_resources::CfnResource for SourceData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.x509_certificate_data {
            if the_val.len() > 8000 as _ {
                return Err(format!("Max validation failed on field 'x509_certificate_data'. {} is greater than 8000", the_val.len()));
            }
        }

        if let Some(the_val) = &self.x509_certificate_data {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'x509_certificate_data'. {} is less than 1",
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
