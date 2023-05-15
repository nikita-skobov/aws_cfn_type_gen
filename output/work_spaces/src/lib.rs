
pub mod cfn_workspace {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkspace {
    /// No documentation provided by AWS
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    pub root_volume_encryption_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "BundleId")]
    pub bundle_id: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "VolumeEncryptionKey")]
    pub volume_encryption_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    pub user_volume_encryption_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkspaceProperties")]
    pub workspace_properties: Option<WorkspaceProperties>,

}


#[derive(serde::Serialize, Default)]
pub struct WorkspaceProperties {
    #[serde(rename = "ComputeTypeName")]
    pub compute_type_name: Option<String>,
    #[serde(rename = "RootVolumeSizeGib")]
    pub root_volume_size_gib: Option<usize>,
    #[serde(rename = "RunningModeAutoStopTimeoutInMinutes")]
    pub running_mode_auto_stop_timeout_in_minutes: Option<usize>,
    #[serde(rename = "UserVolumeSizeGib")]
    pub user_volume_size_gib: Option<usize>,
    #[serde(rename = "RunningMode")]
    pub running_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
