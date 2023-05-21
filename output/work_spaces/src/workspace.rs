

/// The AWS::WorkSpaces::Workspace resource specifies a WorkSpace.
///
/// Updates are not supported for the BundleId, RootVolumeEncryptionEnabled,      UserVolumeEncryptionEnabled, or VolumeEncryptionKey properties. To update      these properties, you must also update a property that triggers a replacement, such as the      UserName property.
#[derive(Default, serde::Serialize)]
pub struct CfnWorkspace {


    /// 
    /// The identifier of the bundle for the WorkSpace.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^wsb-[0-9a-z]{8,63}$
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "BundleId")]
    pub bundle_id: String,


    /// 
    /// Indicates whether the data stored on the user volume is encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    pub user_volume_encryption_enabled: Option<bool>,


    /// 
    /// The tags for the WorkSpace.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The identifier of the AWS Directory Service directory for the WorkSpace.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 10
    ///
    /// Maximum: 65
    ///
    /// Pattern: ^d-[0-9a-f]{8,63}$
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,


    /// 
    /// The WorkSpace properties.
    /// 
    /// Required: No
    ///
    /// Type: WorkspaceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: Option<WorkspaceProperties>,


    /// 
    /// The symmetric AWS KMS key used to encrypt data stored on your WorkSpace.     Amazon WorkSpaces does not support asymmetric KMS keys.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "VolumeEncryptionKey")]
    pub volume_encryption_key: Option<String>,


    /// 
    /// Indicates whether the data stored on the root volume is encrypted.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    pub root_volume_encryption_enabled: Option<bool>,


    /// 
    /// The user name of the user for the WorkSpace. This user name must exist in the AWS Directory Service directory for the WorkSpace.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserName")]
    pub user_name: String,

}


/// Information about a WorkSpace.
#[derive(Default, serde::Serialize)]
pub struct WorkspaceProperties {


    /// 
    /// The size of the root volume. For important information about how to modify the size of     the root and user volumes, see Modify a     WorkSpace.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RootVolumeSizeGib")]
    pub root_volume_size_gib: Option<i64>,


    /// 
    /// The time after a user logs off when WorkSpaces are automatically stopped. Configured in     60-minute intervals.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,


    /// 
    /// The size of the user storage. For important information about how to modify the size of     the root and user volumes, see Modify a     WorkSpace.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserVolumeSizeGib")]
    pub user_volume_size_gib: Option<i64>,


    /// 
    /// The running mode. For more information, see Manage the WorkSpace Running     Mode.
    /// 
    /// NoteThe MANUAL value is only supported by Amazon WorkSpaces Core. Contact       your account team to be allow-listed to use this value. For more information, see       Amazon WorkSpaces Core.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ALWAYS_ON | AUTO_STOP | MANUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunningMode")]
    pub running_mode: Option<String>,


    /// 
    /// The compute type. For more information, see Amazon WorkSpaces     Bundles.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GRAPHICS | GRAPHICS_G4DN | GRAPHICSPRO | GRAPHICSPRO_G4DN | PERFORMANCE | POWER | POWERPRO | STANDARD | VALUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComputeTypeName")]
    pub compute_type_name: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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