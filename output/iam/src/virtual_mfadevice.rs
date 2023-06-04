/// Creates a new virtual MFA device for the AWS account. After creating     the virtual MFA, use EnableMFADevice to attach     the MFA device to an IAM user. For more information about creating and     working with virtual MFA devices, see Using a virtual MFA device in     the IAM User Guide.
///
/// For information about the maximum number of MFA devices you can create, see IAM and AWS STS quotas in the IAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVirtualMFADevice {
    ///
    /// The path for the virtual MFA device. For more information about paths, see IAM         identifiers in the IAM User Guide.
    ///
    /// This parameter is optional. If it is not included, it defaults to a slash (/).
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting   of either a forward slash (/) by itself or a string that must begin and end with forward slashes.   In addition, it can contain any ASCII character from the ! (\u0021) through the DEL character (\u007F), including   most punctuation characters, digits, and upper and lowercased letters.
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
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub path: Option<cfn_resources::StrVal>,

    ///
    /// A list of tags that you want to attach to the new IAM virtual MFA device.    Each tag consists of a key name and an associated value. For more information about tagging, see Tagging IAM resources in the    IAM User Guide.
    ///
    /// NoteIf any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request   fails and the resource is not created.
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
    /// The IAM user associated with this virtual MFA device.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Users")]
    pub users: Vec<String>,

    ///
    /// The name of the virtual MFA device, which must be unique. Use with path to uniquely       identify a virtual MFA device.
    ///
    /// This parameter allows (through its regex pattern) a string of characters consisting of upper and lowercase alphanumeric   characters with no spaces. You can also include any of the following characters: _+=,.@-
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Pattern: [\w+=,.@-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualMfaDeviceName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub virtual_mfa_device_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_serial_number: CfnVirtualMFADeviceserialnumber,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualMFADeviceserialnumber;
impl CfnVirtualMFADeviceserialnumber {
    pub fn att_name(&self) -> &'static str {
        r#"SerialNumber"#
    }
}

impl cfn_resources::CfnResource for CfnVirtualMFADevice {
    fn type_string(&self) -> &'static str {
        "AWS::IAM::VirtualMFADevice"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'path'. {} is less than 1",
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

        if let Some(the_val) = &self.virtual_mfa_device_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'virtual_mfa_device_name'. {} is less than 1", s.len()));
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
