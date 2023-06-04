/// Creates an agreement. An agreement is a bilateral trading partner agreement, or partnership,    between an AWS Transfer Family server and an AS2 process. The agreement defines the file and message    transfer relationship between the server and the AS2 process. To define an agreement, Transfer Family    combines a server, local profile, partner profile, certificate, and other    attributes.
///
/// The partner is identified with the PartnerProfileId, and the AS2 process is identified with the LocalProfileId.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAgreement {
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
    /// The landing directory (folder) for files that are transferred by using the AS2    protocol.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^$|/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseDirectory")]
    pub base_directory: cfn_resources::StrVal,

    ///
    /// The name or short description that's used to identify the agreement.
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
    /// A unique identifier for the AS2 local profile.
    ///
    /// Required: Yes
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
    pub local_profile_id: cfn_resources::StrVal,

    ///
    /// A unique identifier for the partner profile used in the agreement.
    ///
    /// Required: Yes
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
    pub partner_profile_id: cfn_resources::StrVal,

    ///
    /// A system-assigned unique identifier for a server instance. This identifier indicates the    specific server that the agreement uses.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^s-([0-9a-f]{17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerId")]
    pub server_id: cfn_resources::StrVal,

    ///
    /// The current status of the agreement, either ACTIVE or    INACTIVE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTIVE | INACTIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status: Option<AgreementStatusEnum>,

    ///
    /// Key-value pairs that can be used to group and search for agreements.
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

    #[serde(skip_serializing)]
    pub att_agreement_id: CfnAgreementagreementid,

    #[serde(skip_serializing)]
    pub att_arn: CfnAgreementarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AgreementStatusEnum {
    /// ACTIVE
    #[serde(rename = "ACTIVE")]
    Active,

    /// INACTIVE
    #[serde(rename = "INACTIVE")]
    Inactive,
}

impl Default for AgreementStatusEnum {
    fn default() -> Self {
        AgreementStatusEnum::Active
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAgreementagreementid;
impl CfnAgreementagreementid {
    pub fn att_name(&self) -> &'static str {
        r#"AgreementId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAgreementarn;
impl CfnAgreementarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAgreement {
    fn type_string(&self) -> &'static str {
        "AWS::Transfer::Agreement"
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

        let the_val = &self.base_directory;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'base_directory'. {} is greater than 1024",
                    s.len()
                ));
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

        let the_val = &self.local_profile_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 19 as _ {
                return Err(format!(
                    "Max validation failed on field 'local_profile_id'. {} is greater than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.local_profile_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 19 as _ {
                return Err(format!(
                    "Min validation failed on field 'local_profile_id'. {} is less than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.partner_profile_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 19 as _ {
                return Err(format!(
                    "Max validation failed on field 'partner_profile_id'. {} is greater than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.partner_profile_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 19 as _ {
                return Err(format!(
                    "Min validation failed on field 'partner_profile_id'. {} is less than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.server_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 19 as _ {
                return Err(format!(
                    "Max validation failed on field 'server_id'. {} is greater than 19",
                    s.len()
                ));
            }
        }

        let the_val = &self.server_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 19 as _ {
                return Err(format!(
                    "Min validation failed on field 'server_id'. {} is less than 19",
                    s.len()
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
