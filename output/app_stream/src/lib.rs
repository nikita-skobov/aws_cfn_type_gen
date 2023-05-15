
pub mod cfn_app_block {

#[derive(serde::Serialize, Default)]
pub struct CfnAppBlock {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SourceS3Location")]
    pub source_s3_location: S3Location,
    /// No documentation provided by AWS
    #[serde(rename = "SetupScriptDetails")]
    pub setup_script_details: ScriptDetails,

}


#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "TagValue")]
    pub tag_value: String,
    #[serde(rename = "TagKey")]
    pub tag_key: String,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct ScriptDetails {
    #[serde(rename = "ScriptS3Location")]
    pub script_s3_location: S3Location,
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: usize,
    #[serde(rename = "ExecutableParameters")]
    pub executable_parameters: Option<String>,
    #[serde(rename = "ExecutablePath")]
    pub executable_path: String,

}


}

pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchPath")]
    pub launch_path: String,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchParameters")]
    pub launch_parameters: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceFamilies")]
    pub instance_families: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IconS3Location")]
    pub icon_s3_location: S3Location,
    /// No documentation provided by AWS
    #[serde(rename = "AppBlockArn")]
    pub app_block_arn: Arn,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
    /// List of PlatformType
    #[serde(rename = "Platforms")]
    pub platforms: Vec<PlatformType>,
    /// List of ApplicationAttribute
    #[serde(rename = "AttributesToDelete")]
    pub attributes_to_delete: Option<Vec<ApplicationAttribute>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}

pub type PlatformType = String;
#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3Key")]
    pub s3_key: String,

}
pub type Arn = String;pub type ApplicationAttribute = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    #[serde(rename = "TagValue")]
    pub tag_value: String,

}


}

pub mod cfn_application_entitlement_association {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationEntitlementAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationIdentifier")]
    pub application_identifier: String,
    /// No documentation provided by AWS
    #[serde(rename = "EntitlementName")]
    pub entitlement_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "StackName")]
    pub stack_name: String,

}



}

pub mod cfn_application_fleet_association {

#[derive(serde::Serialize, Default)]
pub struct CfnApplicationFleetAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationArn")]
    pub application_arn: Arn,

}

pub type Arn = String;

}

pub mod cfn_directory_config {

#[derive(serde::Serialize, Default)]
pub struct CfnDirectoryConfig {
    /// No documentation provided by AWS
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationalUnitDistinguishedNames")]
    pub organizational_unit_distinguished_names: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateBasedAuthProperties")]
    pub certificate_based_auth_properties: Option<CertificateBasedAuthProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceAccountCredentials")]
    pub service_account_credentials: ServiceAccountCredentials,

}


#[derive(serde::Serialize, Default)]
pub struct CertificateBasedAuthProperties {
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "CertificateAuthorityArn")]
    pub certificate_authority_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceAccountCredentials {
    #[serde(rename = "AccountName")]
    pub account_name: String,
    #[serde(rename = "AccountPassword")]
    pub account_password: String,

}


}

pub mod cfn_entitlement {

#[derive(serde::Serialize, Default)]
pub struct CfnEntitlement {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AppVisibility")]
    pub app_visibility: String,
    /// List of Attribute
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,

}


#[derive(serde::Serialize, Default)]
pub struct Attribute {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnFleet {
    /// No documentation provided by AWS
    #[serde(rename = "FleetType")]
    pub fleet_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SessionScriptS3Location")]
    pub session_script_s3_location: Option<S3Location>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxConcurrentSessions")]
    pub max_concurrent_sessions: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Platform")]
    pub platform: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StreamView")]
    pub stream_view: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "UsbDeviceFilterStrings")]
    pub usb_device_filter_strings: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxUserDurationInSeconds")]
    pub max_user_duration_in_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "IdleDisconnectTimeoutInSeconds")]
    pub idle_disconnect_timeout_in_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "DisconnectTimeoutInSeconds")]
    pub disconnect_timeout_in_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainJoinInfo")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ImageArn")]
    pub image_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableDefaultInternetAccess")]
    pub enable_default_internet_access: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ComputeCapacity")]
    pub compute_capacity: Option<ComputeCapacity>,

}


#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ComputeCapacity {
    #[serde(rename = "DesiredInstances")]
    pub desired_instances: usize,

}

#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "S3Key")]
    pub s3_key: String,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,

}

#[derive(serde::Serialize, Default)]
pub struct DomainJoinInfo {
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,

}


}

pub mod cfn_image_builder {

#[derive(serde::Serialize, Default)]
pub struct CfnImageBuilder {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of AccessEndpoint
    #[serde(rename = "AccessEndpoints")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainJoinInfo")]
    pub domain_join_info: Option<DomainJoinInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "AppstreamAgentVersion")]
    pub appstream_agent_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableDefaultInternetAccess")]
    pub enable_default_internet_access: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ImageArn")]
    pub image_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AccessEndpoint {
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    #[serde(rename = "VpceId")]
    pub vpce_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DomainJoinInfo {
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_stack {

#[derive(serde::Serialize, Default)]
pub struct CfnStack {
    /// No documentation provided by AWS
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationSettings")]
    pub application_settings: Option<ApplicationSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "StreamingExperienceSettings")]
    pub streaming_experience_settings: Option<StreamingExperienceSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "DeleteStorageConnectors")]
    pub delete_storage_connectors: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EmbedHostDomains")]
    pub embed_host_domains: Option<Vec<String>>,
    /// List of UserSetting
    #[serde(rename = "UserSettings")]
    pub user_settings: Option<Vec<UserSetting>>,
    /// No documentation provided by AWS
    #[serde(rename = "FeedbackURL")]
    pub feedback_url: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RedirectURL")]
    pub redirect_url: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AttributesToDelete")]
    pub attributes_to_delete: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of AccessEndpoint
    #[serde(rename = "AccessEndpoints")]
    pub access_endpoints: Option<Vec<AccessEndpoint>>,
    /// List of StorageConnector
    #[serde(rename = "StorageConnectors")]
    pub storage_connectors: Option<Vec<StorageConnector>>,

}


#[derive(serde::Serialize, Default)]
pub struct ApplicationSettings {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "SettingsGroup")]
    pub settings_group: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StorageConnector {
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "Domains")]
    pub domains: Option<Vec<String>>,
    #[serde(rename = "ConnectorType")]
    pub connector_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct StreamingExperienceSettings {
    #[serde(rename = "PreferredProtocol")]
    pub preferred_protocol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccessEndpoint {
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    #[serde(rename = "VpceId")]
    pub vpce_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct UserSetting {
    #[serde(rename = "Permission")]
    pub permission: String,
    #[serde(rename = "Action")]
    pub action: String,

}


}

pub mod cfn_stack_fleet_association {

#[derive(serde::Serialize, Default)]
pub struct CfnStackFleetAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "StackName")]
    pub stack_name: String,

}



}

pub mod cfn_stack_user_association {

#[derive(serde::Serialize, Default)]
pub struct CfnStackUserAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "StackName")]
    pub stack_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SendEmailNotification")]
    pub send_email_notification: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: String,

}



}

pub mod cfn_user {

#[derive(serde::Serialize, Default)]
pub struct CfnUser {
    /// No documentation provided by AWS
    #[serde(rename = "MessageAction")]
    pub message_action: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "UserName")]
    pub user_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "FirstName")]
    pub first_name: Option<String>,

}



}
