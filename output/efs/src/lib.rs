
pub mod cfn_access_point {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPoint {
    /// The operating system user and group applied to all file system requests made using the access point.
    #[serde(rename = "PosixUser")]
    pub posix_user: Option<PosixUser>,
    /// (optional) A string of up to 64 ASCII characters that Amazon EFS uses to ensure idempotent creation.
    #[serde(rename = "ClientToken")]
    pub client_token: Option<String>,
    /// Specifies the directory on the Amazon EFS file system that the access point exposes as the root directory of your file system to NFS clients using the access point. The clients using the access point can only access the root directory and below. If the RootDirectory>Path specified does not exist, EFS creates it and applies the CreationInfo settings when a client connects to an access point. When specifying a RootDirectory, you need to provide the Path, and the CreationInfo is optional.
    #[serde(rename = "RootDirectory")]
    pub root_directory: Option<RootDirectory>,
    /// The ID of the EFS file system that the access point provides access to.
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// List of AccessPointTag
    #[serde(rename = "AccessPointTags")]
    pub access_point_tags: Option<Vec<AccessPointTag>>,

}


#[derive(serde::Serialize, Default)]
pub struct PosixUser {
    #[serde(rename = "SecondaryGids")]
    pub secondary_gids: Option<Vec<String>>,
    #[serde(rename = "Uid")]
    pub uid: String,
    #[serde(rename = "Gid")]
    pub gid: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccessPointTag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RootDirectory {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "CreationInfo")]
    pub creation_info: Option<CreationInfo>,

}

#[derive(serde::Serialize, Default)]
pub struct CreationInfo {
    #[serde(rename = "OwnerUid")]
    pub owner_uid: String,
    #[serde(rename = "Permissions")]
    pub permissions: String,
    #[serde(rename = "OwnerGid")]
    pub owner_gid: String,

}


}

pub mod cfn_file_system {

#[derive(serde::Serialize, Default)]
pub struct CfnFileSystem {
    /// No documentation provided by AWS
    #[serde(rename = "ProvisionedThroughputInMibps")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    /// List of LifecyclePolicy
    #[serde(rename = "LifecyclePolicies")]
    pub lifecycle_policies: Option<Vec<LifecyclePolicy>>,
    /// No documentation provided by AWS
    #[serde(rename = "FileSystemPolicy")]
    pub file_system_policy: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ThroughputMode")]
    pub throughput_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupPolicy")]
    pub backup_policy: Option<BackupPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZoneName")]
    pub availability_zone_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    /// Whether to bypass the FileSystemPolicy lockout safety check. The policy lockout safety check determines whether the policy in the request will prevent the principal making the request to be locked out from making future PutFileSystemPolicy requests on the file system. Set BypassPolicyLockoutSafetyCheck to True only when you intend to prevent the principal that is making the request from making a subsequent PutFileSystemPolicy request on the file system. Defaults to false
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// List of ElasticFileSystemTag
    #[serde(rename = "FileSystemTags")]
    pub file_system_tags: Option<Vec<ElasticFileSystemTag>>,

}


#[derive(serde::Serialize, Default)]
pub struct LifecyclePolicy {
    #[serde(rename = "TransitionToIA")]
    pub transition_to_ia: Option<String>,
    #[serde(rename = "TransitionToPrimaryStorageClass")]
    pub transition_to_primary_storage_class: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BackupPolicy {
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticFileSystemTag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_mount_target {

#[derive(serde::Serialize, Default)]
pub struct CfnMountTarget {
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,

}



}
