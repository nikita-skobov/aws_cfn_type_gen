

/// The AWS::EFS::AccessPoint resource creates an EFS access point.     An access point is an application-specific view into an EFS file system that applies an operating system user and    group, and a file system path, to any file system request made through the access point. The operating system    user and group override any identity information provided by the NFS client. The file system path is exposed as    the access point's root directory. Applications using the access point can only access data in its own directory and below. To learn more, see    Mounting a file system using EFS access points.
///
/// This operation requires permissions for the elasticfilesystem:CreateAccessPoint action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAccessPoint {


    /// 
    /// The directory on the Amazon EFS file system that the access point exposes as the root directory to NFS clients using the access point.
    /// 
    /// Required: No
    ///
    /// Type: RootDirectory
    ///
    /// Update requires: Replacement
    #[serde(rename = "RootDirectory")]
    pub root_directory: Option<RootDirectory>,


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
    pub client_token: Option<String>,


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
    pub access_point_tags: Option<Vec<AccessPointTag>>,


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
    pub file_system_id: String,


    /// 
    /// The full POSIX identity, including the user ID, group ID, and secondary group IDs on the access point that is used for all file operations by    NFS clients using the access point.
    /// 
    /// Required: No
    ///
    /// Type: PosixUser
    ///
    /// Update requires: Replacement
    #[serde(rename = "PosixUser")]
    pub posix_user: Option<PosixUser>,

}

impl cfn_resources::CfnResource for CfnAccessPoint {
    fn type_string() -> &'static str {
        "AWS::EFS::AccessPoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Required if the RootDirectory > Path specified does not exist.    Specifies the POSIX IDs and permissions to apply to the access point's RootDirectory > Path.    If the access point root directory does not exist, EFS creates it with these settings when a client connects to the access point.    When specifying CreationInfo, you must include values for all properties.
///
/// Amazon EFS creates a root directory only if you have provided the CreationInfo: OwnUid, OwnGID, and permissions for the directory.    If you do not provide this information, Amazon EFS does not create the root directory. If the root directory does not exist, attempts to mount    using the access point will fail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreationInfo {


    /// 
    /// Specifies the POSIX user ID to apply to the RootDirectory. Accepts values from 0 to 2^32 (4294967295).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerUid")]
    pub owner_uid: String,


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
    pub permissions: String,


    /// 
    /// Specifies the POSIX group ID to apply to the RootDirectory. Accepts values from 0 to 2^32 (4294967295).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerGid")]
    pub owner_gid: String,

}


/// The full POSIX identity, including the user ID, group ID, and any secondary group IDs, on the access point that is used for all file system operations performed by    NFS clients using the access point.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub gid: String,


    /// 
    /// The POSIX user ID used for all file system operations using this access point.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Uid")]
    pub uid: String,


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
    pub secondary_gids: Option<Vec<String>>,

}


/// Specifies the directory on the Amazon EFS file system that the access point provides access to.    The access point exposes the specified file system path as    the root directory of your file system to applications using the access point.    NFS clients using the access point can only access data in the access point's RootDirectory and it's subdirectories.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RootDirectory {


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
    pub path: Option<String>,


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
    pub creation_info: Option<CreationInfo>,

}


/// A tag is a key-value pair attached to a file system. Allowed characters in the Key and Value properties       are letters, white space, and numbers that       can be represented in UTF-8, and the following characters: + - = . _ : /
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub key: Option<String>,


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
    pub value: Option<String>,

}
