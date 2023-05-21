/// Uploads a server certificate entity for the AWS account. The server certificate       entity includes a public key certificate, a private key, and an optional certificate       chain, which should all be PEM-encoded.
///
/// We recommend that you use AWS Certificate Manager to       provision, manage, and deploy your server certificates. With ACM you can request a       certificate, deploy it to AWS resources, and let ACM handle certificate renewals for       you. Certificates provided by ACM are free. For more information about using ACM,       see the AWS Certificate Manager User         Guide.
///
/// For more information about working with server certificates, see Working         with server certificates in the IAM User Guide. This       topic includes a list of AWS services that can use the server certificates that you       manage with IAM.
///
/// For information about the number of server certificates you can upload, see IAM and AWS STS         quotas in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServerCertificate {
    ///
    /// The contents of the public key certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16384
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u00FF]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateBody")]
    pub certificate_body: Option<String>,

    ///
    /// The contents of the public key certificate chain.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2097152
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u00FF]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: Option<String>,

    ///
    /// The path for the server certificate. For more information about paths, see IAM         identifiers in the IAM User Guide.
    ///
    /// This parameter is optional. If it is not included, it defaults to a slash (/).       This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
    ///
    /// Note If you are uploading a server certificate specifically for use with Amazon         CloudFront distributions, you must specify a path using the path         parameter. The path must begin with /cloudfront and must include a         trailing slash (for example, /cloudfront/test/).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: (\u002F)|(\u002F[\u0021-\u007E]+\u002F)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,

    ///
    /// The contents of the private key in PEM-encoded format.
    ///
    /// The regex pattern   used to validate this parameter is a string of characters consisting of the following:
    ///
    /// Any printable ASCII   character ranging from the space character (\u0020) through the end of the ASCII character range               The printable characters in the Basic Latin and Latin-1 Supplement character set   (through \u00FF)               The special characters tab (\u0009), line feed (\u000A), and   carriage return (\u000D)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16384
    ///
    /// Pattern: [\u0009\u000A\u000D\u0020-\u00FF]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "PrivateKey")]
    pub private_key: Option<String>,

    ///
    /// The name for the server certificate. Do not include the path in this value. The name       of the certificate cannot contain any spaces.
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerCertificateName")]
    pub server_certificate_name: Option<String>,

    ///
    /// A list of tags that are attached to the server certificate. For more information about tagging, see Tagging IAM resources in the    IAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnServerCertificate {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::ServerCertificate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.certificate_body {
            if the_val.len() > 16384 as _ {
                return Err(format!(
                    "Max validation failed on field 'certificate_body'. {} is greater than 16384",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.certificate_body {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'certificate_body'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.certificate_chain {
            if the_val.len() > 2097152 as _ {
                return Err(format!("Max validation failed on field 'certificate_chain'. {} is greater than 2097152", the_val.len()));
            }
        }

        if let Some(the_val) = &self.certificate_chain {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'certificate_chain'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.path {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'path'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.path {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'path'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.private_key {
            if the_val.len() > 16384 as _ {
                return Err(format!(
                    "Max validation failed on field 'private_key'. {} is greater than 16384",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.private_key {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'private_key'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.server_certificate_name {
            if the_val.len() > 128 as _ {
                return Err(format!("Max validation failed on field 'server_certificate_name'. {} is greater than 128", the_val.len()));
            }
        }

        if let Some(the_val) = &self.server_certificate_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'server_certificate_name'. {} is less than 1",
                    the_val.len()
                ));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
