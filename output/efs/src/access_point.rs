/// The AWS::EFS::AccessPoint resource creates an EFS access point.     An access point is an application-specific view into an EFS file system that applies an operating system user and    group, and a file system path, to any file system request made through the access point. The operating system    user and group override any identity information provided by the NFS client. The file system path is exposed as    the access point's root directory. Applications using the access point can only access data in its own directory and below. To learn more, see    Mounting a file system using EFS access points.
///
/// This operation requires permissions for the elasticfilesystem:CreateAccessPoint action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAccessPoint {
    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of AccessPointTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPointTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_tags: Option<Vec<AccessPointTag>>,

    ///
    /// The opaque string specified in the request to ensure idempotent creation.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: .+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the EFS file system that the access point applies to. Accepts only the ID format for input when specifying a file system, for example fs-0123456789abcedf2.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^(arn:aws[-a-z]*:elasticfilesystem:[0-9a-z-:]+:file-system/fs-[0-9a-f]{8,40}|fs-[0-9a-f]{8,40})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileSystemId")]
    pub file_system_id: cfn_resources::StrVal,

    ///
    /// The full POSIX identity, including the user ID, group ID, and secondary group IDs on the access point that is used for all file operations by    NFS clients using the access point.
    ///
    /// Required: No
    ///
    /// Type: PosixUser
    ///
    /// Update requires: Replacement
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,

    ///
    /// The directory on the Amazon EFS file system that the access point exposes as the root directory to NFS clients using the access point.
    ///
    /// Required: No
    ///
    /// Type: RootDirectory
    ///
    /// Update requires: Replacement
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,

    #[serde(skip_serializing)]
    pub att_access_point_id: CfnAccessPointaccesspointid,

    #[serde(skip_serializing)]
    pub att_arn: CfnAccessPointarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointaccesspointid;
impl CfnAccessPointaccesspointid {
    pub fn att_name(&self) -> &'static str {
        r#"AccessPointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAccessPointarn;
impl CfnAccessPointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string(&self) -> &'static str {
        "AWS::EFS::AccessPoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.client_token {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'client_token'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.client_token {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'client_token'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.file_system_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'file_system_id'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        self.posix_user
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.root_directory
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A tag is a key-value pair attached to a file system. Allowed characters in the Key and Value properties       are letters, white space, and numbers that       can be represented in UTF-8, and the following characters: + - = . _ : /
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccessPointTag {
    ///
    /// The tag key (String). The key can't start with aws:.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^(?![aA]{1}[wW]{1}[sS]{1}:)([\p{L}\p{Z}\p{N}_.:/=+\-@]+)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The value of the tag key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^([\p{L}\p{Z}\p{N}_.:/=+\-@]*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AccessPointTag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'value'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Required if the RootDirectory > Path specified does not exist.    Specifies the POSIX IDs and permissions to apply to the access point's RootDirectory > Path.    If the access point root directory does not exist, EFS creates it with these settings when a client connects to the access point.    When specifying CreationInfo, you must include values for all properties.
///
/// Amazon EFS creates a root directory only if you have provided the CreationInfo: OwnUid, OwnGID, and permissions for the directory.    If you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount    using the access point will fail.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CreationInfo {
    ///
    /// Specifies the POSIX group ID to apply to the RootDirectory. Accepts values from 0 to 2^32 (4294967295).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerGid")]
    pub owner_gid: cfn_resources::StrVal,

    ///
    /// Specifies the POSIX user ID to apply to the RootDirectory. Accepts values from 0 to 2^32 (4294967295).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerUid")]
    pub owner_uid: cfn_resources::StrVal,

    ///
    /// Specifies the POSIX permissions to apply to the RootDirectory, in the format of an octal number representing the file's mode bits.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 4
    ///
    /// Pattern: ^[0-7]{3,4}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permissions")]
    pub permissions: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CreationInfo {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.permissions;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'permissions'. {} is greater than 4",
                    s.len()
                ));
            }
        }

        let the_val = &self.permissions;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'permissions'. {} is less than 3",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The full POSIX identity, including the user ID, group ID, and any secondary group IDs, on the access point that is used for all file system operations performed by    NFS clients using the access point.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PosixUser {
    ///
    /// The POSIX group ID used for all file system operations using this access point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Gid")]
    pub gid: cfn_resources::StrVal,

    ///
    /// Secondary POSIX group IDs used for all file system operations using this access point.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecondaryGids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<String>>,

    ///
    /// The POSIX user ID used for all file system operations using this access point.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Uid")]
    pub uid: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PosixUser {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.secondary_gids {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'secondary_gids'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the directory on the Amazon EFS file system that the access point provides access to.    The access point exposes the specified file system path as    the root directory of your file system to applications using the access point.    NFS clients using the access point can only access data in the access point's RootDirectory and it's subdirectories.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RootDirectory {
    ///
    /// (Optional) Specifies the POSIX IDs and permissions to apply to the access point's RootDirectory.    If the RootDirectory > Path specified does not exist,    EFS creates the root directory using the CreationInfo settings when a client connects to an access point.    When specifying the CreationInfo, you must provide values for all properties.
    ///
    /// ImportantIf you do not provide CreationInfo and the specified RootDirectory > Path does not exist,    attempts to mount the file system using the access point will fail.
    ///
    /// Required: No
    ///
    /// Type: CreationInfo
    ///
    /// Update requires: Replacement
    #[serde(rename = "CreationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_info: Option<CreationInfo>,

    ///
    /// Specifies the path on the EFS file system to expose as the root directory to NFS clients using the access point to access the EFS file system.    A path can have up to four subdirectories.    If the specified path does not exist, you are required to provide the CreationInfo.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^(\/|(\/(?!\.)+[^$#<>;`|&?{}^*/\n]+){1,4})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RootDirectory {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.creation_info
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'path'. {} is greater than 100",
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

        Ok(())
    }
}
