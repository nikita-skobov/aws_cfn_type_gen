/// The AWS::DataSync::LocationFSxWindows resource specifies an endpoint for an     Amazon FSx for Windows Server file system.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationFSxWindows {
    ///
    /// Specifies the name of the Windows domain that the FSx for Windows File Server belongs    to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^[A-Za-z0-9]((\.|-+)?[A-Za-z0-9]){0,252}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the Amazon Resource Name (ARN) for the FSx for Windows File Server file    system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):fsx:[a-z\-0-9]*:[0-9]{12}:file-system/fs-.*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FsxFilesystemArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_filesystem_arn: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the password of the user who has the permissions to access files and folders in    the file system.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^.{0,104}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Names (ARNs) of the security groups that are used to configure the     FSx for Windows File Server file system.
    ///
    /// Pattern:       ^arn:(aws|aws-cn|aws-us-gov|aws-iso|aws-iso-b):ec2:[a-z\-0-9]*:[0-9]{12}:security-group/.*$
    ///
    /// Length constraints: Maximum length of 128.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupArns")]
    pub security_group_arns: Vec<String>,

    ///
    /// Specifies a mount path for your file system using forward slashes. This is where DataSync reads or writes data (depending on if this is a source or destination    location).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\+\./\(\)\$\p{Zs}]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<cfn_resources::StrVal>,

    ///
    /// Specifies labels that help you categorize, filter, and search for your AWS    resources. We recommend creating at least a name tag for your location.
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
    /// The user who has the permissions to access files and folders in the FSx for Windows File    Server file system.
    ///
    /// For information about choosing a user name that ensures sufficient permissions to files,       folders, and metadata, see user.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 104
    ///
    /// Pattern: ^[^\x5B\x5D\\/:;|=,+*?]{1,104}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "User")]
    pub user: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_location_arn: CfnLocationFSxWindowslocationarn,

    #[serde(skip_serializing)]
    pub att_location_uri: CfnLocationFSxWindowslocationuri,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationFSxWindowslocationarn;
impl CfnLocationFSxWindowslocationarn {
    pub fn att_name(&self) -> &'static str {
        r#"LocationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLocationFSxWindowslocationuri;
impl CfnLocationFSxWindowslocationuri {
    pub fn att_name(&self) -> &'static str {
        r#"LocationUri"#
    }
}

impl cfn_resources::CfnResource for CfnLocationFSxWindows {
    fn type_string(&self) -> &'static str {
        "AWS::DataSync::LocationFSxWindows"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.domain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!(
                        "Max validation failed on field 'domain'. {} is greater than 253",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.fsx_filesystem_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'fsx_filesystem_arn'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.password {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 104 as _ {
                    return Err(format!(
                        "Max validation failed on field 'password'. {} is greater than 104",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.subdirectory {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 4096 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subdirectory'. {} is greater than 4096",
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

        let the_val = &self.user;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 104 as _ {
                return Err(format!(
                    "Max validation failed on field 'user'. {} is greater than 104",
                    s.len()
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
