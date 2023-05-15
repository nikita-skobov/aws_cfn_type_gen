
pub mod cfn_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnDomain {
    /// No documentation provided by AWS
    #[serde(rename = "SoftwareUpdateOptions")]
    pub software_update_options: Option<SoftwareUpdateOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: Option<ClusterConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotOptions")]
    pub snapshot_options: Option<SnapshotOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "OffPeakWindowOptions")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "VPCOptions")]
    pub vpcoptions: Option<VPCOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AdvancedSecurityOptions")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionAtRestOptions")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessPolicies")]
    pub access_policies: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "AdvancedOptions")]
    pub advanced_options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainEndpointOptions")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    /// An arbitrary set of tags (key-value pairs) for this Domain.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EBSOptions")]
    pub ebsoptions: Option<EBSOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "CognitoOptions")]
    pub cognito_options: Option<CognitoOptions>,

}


#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "ZoneAwarenessEnabled")]
    pub zone_awareness_enabled: Option<bool>,
    #[serde(rename = "ZoneAwarenessConfig")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,
    #[serde(rename = "DedicatedMasterEnabled")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "WarmEnabled")]
    pub warm_enabled: Option<bool>,
    #[serde(rename = "WarmType")]
    pub warm_type: Option<String>,
    #[serde(rename = "DedicatedMasterType")]
    pub dedicated_master_type: Option<String>,
    #[serde(rename = "InstanceCount")]
    pub instance_count: Option<usize>,
    #[serde(rename = "DedicatedMasterCount")]
    pub dedicated_master_count: Option<usize>,
    #[serde(rename = "WarmCount")]
    pub warm_count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct OffPeakWindow {
    #[serde(rename = "WindowStartTime")]
    pub window_start_time: Option<WindowStartTime>,

}

#[derive(serde::Serialize, Default)]
pub struct SoftwareUpdateOptions {
    #[serde(rename = "AutoSoftwareUpdateEnabled")]
    pub auto_software_update_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct OffPeakWindowOptions {
    #[serde(rename = "OffPeakWindow")]
    pub off_peak_window: Option<OffPeakWindow>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ZoneAwarenessConfig {
    #[serde(rename = "AvailabilityZoneCount")]
    pub availability_zone_count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SnapshotOptions {
    #[serde(rename = "AutomatedSnapshotStartHour")]
    pub automated_snapshot_start_hour: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedSecurityOptionsInput {
    #[serde(rename = "AnonymousAuthEnabled")]
    pub anonymous_auth_enabled: Option<bool>,
    #[serde(rename = "InternalUserDatabaseEnabled")]
    pub internal_user_database_enabled: Option<bool>,
    #[serde(rename = "MasterUserOptions")]
    pub master_user_options: Option<MasterUserOptions>,
    #[serde(rename = "SAMLOptions")]
    pub samloptions: Option<SAMLOptions>,
    #[serde(rename = "AnonymousAuthDisableDate")]
    pub anonymous_auth_disable_date: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct VPCOptions {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct CognitoOptions {
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NodeToNodeEncryptionOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct EBSOptions {
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "EBSEnabled")]
    pub ebsenabled: Option<bool>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionAtRestOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SAMLOptions {
    #[serde(rename = "SessionTimeoutMinutes")]
    pub session_timeout_minutes: Option<usize>,
    #[serde(rename = "MasterBackendRole")]
    pub master_backend_role: Option<String>,
    #[serde(rename = "SubjectKey")]
    pub subject_key: Option<String>,
    #[serde(rename = "RolesKey")]
    pub roles_key: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Idp")]
    pub idp: Option<Idp>,
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DomainEndpointOptions {
    #[serde(rename = "EnforceHTTPS")]
    pub enforce_https: Option<bool>,
    #[serde(rename = "TLSSecurityPolicy")]
    pub tlssecurity_policy: Option<String>,
    #[serde(rename = "CustomEndpointEnabled")]
    pub custom_endpoint_enabled: Option<bool>,
    #[serde(rename = "CustomEndpointCertificateArn")]
    pub custom_endpoint_certificate_arn: Option<String>,
    #[serde(rename = "CustomEndpoint")]
    pub custom_endpoint: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MasterUserOptions {
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,
    #[serde(rename = "MasterUserARN")]
    pub master_user_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Idp {
    #[serde(rename = "MetadataContent")]
    pub metadata_content: String,
    #[serde(rename = "EntityId")]
    pub entity_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct WindowStartTime {
    #[serde(rename = "Minutes")]
    pub minutes: usize,
    #[serde(rename = "Hours")]
    pub hours: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceSoftwareOptions {
    #[serde(rename = "Cancellable")]
    pub cancellable: Option<bool>,
    #[serde(rename = "UpdateStatus")]
    pub update_status: Option<String>,
    #[serde(rename = "NewVersion")]
    pub new_version: Option<String>,
    #[serde(rename = "UpdateAvailable")]
    pub update_available: Option<bool>,
    #[serde(rename = "CurrentVersion")]
    pub current_version: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "AutomatedUpdateDate")]
    pub automated_update_date: Option<String>,
    #[serde(rename = "OptionalDeployment")]
    pub optional_deployment: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct LogPublishingOption {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


}
