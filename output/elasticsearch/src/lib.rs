
pub mod cfn_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnDomain {
    /// No documentation provided by AWS
    #[serde(rename = "SnapshotOptions")]
    pub snapshot_options: Option<SnapshotOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionAtRestOptions")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "AdvancedOptions")]
    pub advanced_options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "CognitoOptions")]
    pub cognito_options: Option<CognitoOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticsearchVersion")]
    pub elasticsearch_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: Option<ElasticsearchClusterConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "AdvancedSecurityOptions")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
    /// No documentation provided by AWS
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainEndpointOptions")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessPolicies")]
    pub access_policies: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EBSOptions")]
    pub ebsoptions: Option<EBSOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "VPCOptions")]
    pub vpcoptions: Option<VPCOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,

}


#[derive(serde::Serialize, Default)]
pub struct LogPublishingOption {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionAtRestOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct VPCOptions {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedSecurityOptionsInput {
    #[serde(rename = "MasterUserOptions")]
    pub master_user_options: Option<MasterUserOptions>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "InternalUserDatabaseEnabled")]
    pub internal_user_database_enabled: Option<bool>,
    #[serde(rename = "AnonymousAuthEnabled")]
    pub anonymous_auth_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MasterUserOptions {
    #[serde(rename = "MasterUserARN")]
    pub master_user_arn: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserName")]
    pub master_user_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchClusterConfig {
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "ZoneAwarenessEnabled")]
    pub zone_awareness_enabled: Option<bool>,
    #[serde(rename = "InstanceCount")]
    pub instance_count: Option<usize>,
    #[serde(rename = "DedicatedMasterCount")]
    pub dedicated_master_count: Option<usize>,
    #[serde(rename = "DedicatedMasterType")]
    pub dedicated_master_type: Option<String>,
    #[serde(rename = "ZoneAwarenessConfig")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,
    #[serde(rename = "WarmEnabled")]
    pub warm_enabled: Option<bool>,
    #[serde(rename = "DedicatedMasterEnabled")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(rename = "ColdStorageOptions")]
    pub cold_storage_options: Option<ColdStorageOptions>,
    #[serde(rename = "WarmCount")]
    pub warm_count: Option<usize>,
    #[serde(rename = "WarmType")]
    pub warm_type: Option<String>,

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
pub struct NodeToNodeEncryptionOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct EBSOptions {
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "EBSEnabled")]
    pub ebsenabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ColdStorageOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct DomainEndpointOptions {
    #[serde(rename = "CustomEndpointEnabled")]
    pub custom_endpoint_enabled: Option<bool>,
    #[serde(rename = "EnforceHTTPS")]
    pub enforce_https: Option<bool>,
    #[serde(rename = "TLSSecurityPolicy")]
    pub tlssecurity_policy: Option<String>,
    #[serde(rename = "CustomEndpoint")]
    pub custom_endpoint: Option<String>,
    #[serde(rename = "CustomEndpointCertificateArn")]
    pub custom_endpoint_certificate_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CognitoOptions {
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


}
