/// Creates the connector, which captures the parameters for an outbound connection for the    AS2 protocol. The connector is required for sending files to an externally hosted AS2 server.    For more details about connectors, see Create AS2 connectors.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnector {
    ///
    /// With AS2, you can send files by calling StartFileTransfer and specifying the    file paths in the request parameter, SendFilePaths. We use the file’s parent    directory (for example, for --send-file-paths /bucket/dir/file.txt, parent    directory is /bucket/dir/) to temporarily store a processed AS2 message file,    store the MDN when we receive them from the partner, and write a final JSON file containing    relevant metadata of the transmission. So, the AccessRole needs to provide read    and write access to the parent directory of the file location used in the     StartFileTransfer request. Additionally, you need to provide read and write    access to the parent directory of the files that you intend to send with     StartFileTransfer.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessRole")]
    pub access_role: cfn_resources::StrVal,

    ///
    /// A structure that contains the parameters for a connector object.
    ///
    /// Required: Yes
    ///
    /// Type: As2Config
    ///
    /// Update requires: No interruption
    #[serde(rename = "As2Config")]
    pub as2_config: As2Config,

    ///
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role that allows a connector to turn    on CloudWatch logging for Amazon S3 events. When set, you can view connector    activity in your CloudWatch logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoggingRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_role: Option<cfn_resources::StrVal>,

    ///
    /// Key-value pairs that can be used to group and search for connectors.
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
    /// The URL of the partner's AS2 endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnConnectorarn,

    #[serde(skip_serializing)]
    pub att_connector_id: CfnConnectorconnectorid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectorarn;
impl CfnConnectorarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectorconnectorid;
impl CfnConnectorconnectorid {
    pub fn att_name(&self) -> &'static str {
        r#"ConnectorId"#
    }
}

impl cfn_resources::CfnResource for CfnConnector {
    fn type_string(&self) -> &'static str {
        "AWS::Transfer::Connector"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.access_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'access_role'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.access_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'access_role'. {} is less than 20",
                    s.len()
                ));
            }
        }

        self.as2_config.validate()?;

        if let Some(the_val) = &self.logging_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'logging_role'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.logging_role {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 20 as _ {
                    return Err(format!(
                        "Min validation failed on field 'logging_role'. {} is less than 20",
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

        let the_val = &self.url;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'url'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A structure that contains the parameters for a connector object.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct As2Config {
    ///
    /// Specifies whether the AS2 file is compressed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ZLIB
    ///
    /// Update requires: No interruption
    #[serde(rename = "Compression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<As2ConfigCompressionEnum>,

    ///
    /// The algorithm that is used to encrypt the file.
    ///
    /// NoteYou can only specify NONE if the URL for your connector uses HTTPS. This ensures that     no traffic is sent in clear text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AES128_CBC | AES192_CBC | AES256_CBC | NONE
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<As2ConfigEncryptionAlgorithmEnum>,

    ///
    /// A unique identifier for the AS2 local profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_profile_id: Option<cfn_resources::StrVal>,

    ///
    /// Used for outbound requests (from an AWS Transfer Family server to a partner AS2 server) to determine whether    the partner response for transfers is synchronous or asynchronous. Specify either of the following values:
    ///
    /// SYNC: The system expects a synchronous MDN response, confirming that the file was transferred successfully (or not).                        NONE: Specifies that no MDN response is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "MdnResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdn_response: Option<As2ConfigMdnResponseEnum>,

    ///
    /// The signing algorithm for the MDN response.
    ///
    /// NoteIf set to DEFAULT (or not set at all), the value for SigningAlgorithm is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DEFAULT | NONE | SHA1 | SHA256 | SHA384 | SHA512
    ///
    /// Update requires: No interruption
    #[serde(rename = "MdnSigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdn_signing_algorithm: Option<As2ConfigMdnSigningAlgorithmEnum>,

    ///
    /// Used as the Subject HTTP header attribute in AS2 messages that are being sent with the connector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^[\p{Print}\p{Blank}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_subject: Option<cfn_resources::StrVal>,

    ///
    /// A unique identifier for the partner profile for the connector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartnerProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_profile_id: Option<cfn_resources::StrVal>,

    ///
    /// The algorithm that is used to sign the AS2 messages sent with the connector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NONE | SHA1 | SHA256 | SHA384 | SHA512
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<As2ConfigSigningAlgorithmEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum As2ConfigCompressionEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ZLIB
    #[serde(rename = "ZLIB")]
    Zlib,
}

impl Default for As2ConfigCompressionEnum {
    fn default() -> Self {
        As2ConfigCompressionEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum As2ConfigEncryptionAlgorithmEnum {
    /// AES128_CBC
    #[serde(rename = "AES128_CBC")]
    Aes128cbc,

    /// AES192_CBC
    #[serde(rename = "AES192_CBC")]
    Aes192cbc,

    /// AES256_CBC
    #[serde(rename = "AES256_CBC")]
    Aes256cbc,

    /// NONE
    #[serde(rename = "NONE")]
    None,
}

impl Default for As2ConfigEncryptionAlgorithmEnum {
    fn default() -> Self {
        As2ConfigEncryptionAlgorithmEnum::Aes128cbc
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum As2ConfigMdnResponseEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SYNC
    #[serde(rename = "SYNC")]
    Sync,
}

impl Default for As2ConfigMdnResponseEnum {
    fn default() -> Self {
        As2ConfigMdnResponseEnum::None
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum As2ConfigMdnSigningAlgorithmEnum {
    /// DEFAULT
    #[serde(rename = "DEFAULT")]
    Default,

    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SHA1
    #[serde(rename = "SHA1")]
    Sha1,

    /// SHA256
    #[serde(rename = "SHA256")]
    Sha256,

    /// SHA384
    #[serde(rename = "SHA384")]
    Sha384,

    /// SHA512
    #[serde(rename = "SHA512")]
    Sha512,
}

impl Default for As2ConfigMdnSigningAlgorithmEnum {
    fn default() -> Self {
        As2ConfigMdnSigningAlgorithmEnum::Default
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum As2ConfigSigningAlgorithmEnum {
    /// NONE
    #[serde(rename = "NONE")]
    None,

    /// SHA1
    #[serde(rename = "SHA1")]
    Sha1,

    /// SHA256
    #[serde(rename = "SHA256")]
    Sha256,

    /// SHA384
    #[serde(rename = "SHA384")]
    Sha384,

    /// SHA512
    #[serde(rename = "SHA512")]
    Sha512,
}

impl Default for As2ConfigSigningAlgorithmEnum {
    fn default() -> Self {
        As2ConfigSigningAlgorithmEnum::None
    }
}

impl cfn_resources::CfnResource for As2Config {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.local_profile_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 19 as _ {
                    return Err(format!(
                        "Max validation failed on field 'local_profile_id'. {} is greater than 19",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.local_profile_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 19 as _ {
                    return Err(format!(
                        "Min validation failed on field 'local_profile_id'. {} is less than 19",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.message_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'message_subject'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.message_subject {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'message_subject'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.partner_profile_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 19 as _ {
                    return Err(format!("Max validation failed on field 'partner_profile_id'. {} is greater than 19", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.partner_profile_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 19 as _ {
                    return Err(format!(
                        "Min validation failed on field 'partner_profile_id'. {} is less than 19",
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
