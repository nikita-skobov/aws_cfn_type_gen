
pub mod cfn_broker {

#[derive(serde::Serialize, Default)]
pub struct CfnBroker {
    /// List of User
    #[serde(rename = "Users")]
    pub users: Vec<User>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<ConfigurationId>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Logs")]
    pub logs: Option<LogList>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineType")]
    pub engine_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationStrategy")]
    pub authentication_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StorageType")]
    pub storage_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostInstanceType")]
    pub host_instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentMode")]
    pub deployment_mode: String,
    /// No documentation provided by AWS
    #[serde(rename = "LdapServerMetadata")]
    pub ldap_server_metadata: Option<LdapServerMetadata>,
    /// No documentation provided by AWS
    #[serde(rename = "PubliclyAccessible")]
    pub publicly_accessible: bool,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionOptions")]
    pub encryption_options: Option<EncryptionOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "MaintenanceWindowStartTime")]
    pub maintenance_window_start_time: Option<MaintenanceWindow>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// No documentation provided by AWS
    #[serde(rename = "BrokerName")]
    pub broker_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindow {
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: String,
    #[serde(rename = "TimeZone")]
    pub time_zone: String,
    #[serde(rename = "TimeOfDay")]
    pub time_of_day: String,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionOptions {
    #[serde(rename = "UseAwsOwnedKey")]
    pub use_aws_owned_key: bool,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConfigurationId {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Revision")]
    pub revision: usize,

}

#[derive(serde::Serialize, Default)]
pub struct User {
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "ConsoleAccess")]
    pub console_access: Option<bool>,
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct LogList {
    #[serde(rename = "General")]
    pub general: Option<bool>,
    #[serde(rename = "Audit")]
    pub audit: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct LdapServerMetadata {
    #[serde(rename = "Hosts")]
    pub hosts: Vec<String>,
    #[serde(rename = "RoleSearchSubtree")]
    pub role_search_subtree: Option<bool>,
    #[serde(rename = "UserRoleName")]
    pub user_role_name: Option<String>,
    #[serde(rename = "UserSearchMatching")]
    pub user_search_matching: String,
    #[serde(rename = "UserSearchSubtree")]
    pub user_search_subtree: Option<bool>,
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,
    #[serde(rename = "RoleBase")]
    pub role_base: String,
    #[serde(rename = "ServiceAccountPassword")]
    pub service_account_password: String,
    #[serde(rename = "UserBase")]
    pub user_base: String,
    #[serde(rename = "ServiceAccountUsername")]
    pub service_account_username: String,
    #[serde(rename = "RoleSearchMatching")]
    pub role_search_matching: String,

}


}

pub mod cfn_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "Data")]
    pub data: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationStrategy")]
    pub authentication_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of TagsEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagsEntry>>,
    /// No documentation provided by AWS
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// No documentation provided by AWS
    #[serde(rename = "EngineType")]
    pub engine_type: String,

}


#[derive(serde::Serialize, Default)]
pub struct TagsEntry {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_configuration_association {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "Broker")]
    pub broker: String,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: ConfigurationId,

}


#[derive(serde::Serialize, Default)]
pub struct ConfigurationId {
    #[serde(rename = "Revision")]
    pub revision: usize,
    #[serde(rename = "Id")]
    pub id: String,

}


}
