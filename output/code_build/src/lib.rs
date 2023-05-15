
pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// No documentation provided by AWS
    #[serde(rename = "ConcurrentBuildLimit")]
    pub concurrent_build_limit: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Visibility")]
    pub visibility: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceAccessRole")]
    pub resource_access_role: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Source")]
    pub source: Source,
    /// No documentation provided by AWS
    #[serde(rename = "Artifacts")]
    pub artifacts: Artifacts,
    /// No documentation provided by AWS
    #[serde(rename = "BuildBatchConfig")]
    pub build_batch_config: Option<ProjectBuildBatchConfig>,
    /// List of Artifacts
    #[serde(rename = "SecondaryArtifacts")]
    pub secondary_artifacts: Option<Vec<Artifacts>>,
    /// No documentation provided by AWS
    #[serde(rename = "Environment")]
    pub environment: Environment,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRole")]
    pub service_role: String,
    /// List of Source
    #[serde(rename = "SecondarySources")]
    pub secondary_sources: Option<Vec<Source>>,
    /// List of ProjectSourceVersion
    #[serde(rename = "SecondarySourceVersions")]
    pub secondary_source_versions: Option<Vec<ProjectSourceVersion>>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Cache")]
    pub cache: Option<ProjectCache>,
    /// No documentation provided by AWS
    #[serde(rename = "BadgeEnabled")]
    pub badge_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeoutInMinutes")]
    pub timeout_in_minutes: Option<usize>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Triggers")]
    pub triggers: Option<ProjectTriggers>,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "QueuedTimeoutInMinutes")]
    pub queued_timeout_in_minutes: Option<usize>,
    /// List of ProjectFileSystemLocation
    #[serde(rename = "FileSystemLocations")]
    pub file_system_locations: Option<Vec<ProjectFileSystemLocation>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LogsConfig")]
    pub logs_config: Option<LogsConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceVersion")]
    pub source_version: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct BatchRestrictions {
    #[serde(rename = "ComputeTypesAllowed")]
    pub compute_types_allowed: Option<Vec<String>>,
    #[serde(rename = "MaximumBuildsAllowed")]
    pub maximum_builds_allowed: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectCache {
    #[serde(rename = "Modes")]
    pub modes: Option<Vec<String>>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Location")]
    pub location: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchLogsConfig {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectTriggers {
    #[serde(rename = "FilterGroups")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "Webhook")]
    pub webhook: Option<bool>,
    #[serde(rename = "BuildType")]
    pub build_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EnvironmentVariable {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Artifacts {
    #[serde(rename = "NamespaceType")]
    pub namespace_type: Option<String>,
    #[serde(rename = "Location")]
    pub location: Option<String>,
    #[serde(rename = "OverrideArtifactName")]
    pub override_artifact_name: Option<bool>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "ArtifactIdentifier")]
    pub artifact_identifier: Option<String>,
    #[serde(rename = "Packaging")]
    pub packaging: Option<String>,
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Environment {
    #[serde(rename = "ComputeType")]
    pub compute_type: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "RegistryCredential")]
    pub registry_credential: Option<RegistryCredential>,
    #[serde(rename = "Certificate")]
    pub certificate: Option<String>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImagePullCredentialsType")]
    pub image_pull_credentials_type: Option<String>,
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    #[serde(rename = "PrivilegedMode")]
    pub privileged_mode: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterGroup {

}

#[derive(serde::Serialize, Default)]
pub struct S3LogsConfig {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "Location")]
    pub location: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectSourceVersion {
    #[serde(rename = "SourceVersion")]
    pub source_version: Option<String>,
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: String,

}

#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "GitSubmodulesConfig")]
    pub git_submodules_config: Option<GitSubmodulesConfig>,
    #[serde(rename = "Location")]
    pub location: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "BuildSpec")]
    pub build_spec: Option<String>,
    #[serde(rename = "InsecureSsl")]
    pub insecure_ssl: Option<bool>,
    #[serde(rename = "ReportBuildStatus")]
    pub report_build_status: Option<bool>,
    #[serde(rename = "Auth")]
    pub auth: Option<SourceAuth>,
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: Option<String>,
    #[serde(rename = "BuildStatusConfig")]
    pub build_status_config: Option<BuildStatusConfig>,
    #[serde(rename = "GitCloneDepth")]
    pub git_clone_depth: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceAuth {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Resource")]
    pub resource: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RegistryCredential {
    #[serde(rename = "Credential")]
    pub credential: String,
    #[serde(rename = "CredentialProvider")]
    pub credential_provider: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct GitSubmodulesConfig {
    #[serde(rename = "FetchSubmodules")]
    pub fetch_submodules: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectBuildBatchConfig {
    #[serde(rename = "Restrictions")]
    pub restrictions: Option<BatchRestrictions>,
    #[serde(rename = "CombineArtifacts")]
    pub combine_artifacts: Option<bool>,
    #[serde(rename = "BatchReportMode")]
    pub batch_report_mode: Option<String>,
    #[serde(rename = "ServiceRole")]
    pub service_role: Option<String>,
    #[serde(rename = "TimeoutInMins")]
    pub timeout_in_mins: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LogsConfig {
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<CloudWatchLogsConfig>,
    #[serde(rename = "S3Logs")]
    pub s3_logs: Option<S3LogsConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct BuildStatusConfig {
    #[serde(rename = "Context")]
    pub context: Option<String>,
    #[serde(rename = "TargetUrl")]
    pub target_url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProjectFileSystemLocation {
    #[serde(rename = "MountPoint")]
    pub mount_point: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Identifier")]
    pub identifier: String,
    #[serde(rename = "MountOptions")]
    pub mount_options: Option<String>,

}


}

pub mod cfn_report_group {

#[derive(serde::Serialize, Default)]
pub struct CfnReportGroup {
    /// No documentation provided by AWS
    #[serde(rename = "DeleteReports")]
    pub delete_reports: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ExportConfig")]
    pub export_config: ReportExportConfig,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct S3ReportExportConfig {
    #[serde(rename = "EncryptionDisabled")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,
    #[serde(rename = "Packaging")]
    pub packaging: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "BucketOwner")]
    pub bucket_owner: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ReportExportConfig {
    #[serde(rename = "S3Destination")]
    pub s3_destination: Option<S3ReportExportConfig>,
    #[serde(rename = "ExportConfigType")]
    pub export_config_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_source_credential {

#[derive(serde::Serialize, Default)]
pub struct CfnSourceCredential {
    /// No documentation provided by AWS
    #[serde(rename = "Username")]
    pub username: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerType")]
    pub server_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "Token")]
    pub token: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthType")]
    pub auth_type: String,

}



}
