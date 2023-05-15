
pub mod cfn_connector_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnConnectorDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<ConnectorDefinitionVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectorDefinitionVersion {
    #[serde(rename = "Connectors")]
    pub connectors: Vec<Connector>,

}

#[derive(serde::Serialize, Default)]
pub struct Connector {
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "ConnectorArn")]
    pub connector_arn: String,
    #[serde(rename = "Id")]
    pub id: String,

}


}

pub mod cfn_connector_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnConnectorDefinitionVersion {
    /// List of Connector
    #[serde(rename = "Connectors")]
    pub connectors: Vec<Connector>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Connector {
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ConnectorArn")]
    pub connector_arn: String,

}


}

pub mod cfn_core_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnCoreDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<CoreDefinitionVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct CoreDefinitionVersion {
    #[serde(rename = "Cores")]
    pub cores: Vec<Core>,

}

#[derive(serde::Serialize, Default)]
pub struct Core {
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,

}


}

pub mod cfn_core_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnCoreDefinitionVersion {
    /// No documentation provided by AWS
    #[serde(rename = "CoreDefinitionId")]
    pub core_definition_id: String,
    /// List of Core
    #[serde(rename = "Cores")]
    pub cores: Vec<Core>,

}


#[derive(serde::Serialize, Default)]
pub struct Core {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,

}


}

pub mod cfn_device_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnDeviceDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<DeviceDefinitionVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Device {
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeviceDefinitionVersion {
    #[serde(rename = "Devices")]
    pub devices: Vec<Device>,

}


}

pub mod cfn_device_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnDeviceDefinitionVersion {
    /// No documentation provided by AWS
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,
    /// List of Device
    #[serde(rename = "Devices")]
    pub devices: Vec<Device>,

}


#[derive(serde::Serialize, Default)]
pub struct Device {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,

}


}

pub mod cfn_function_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnFunctionDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<FunctionDefinitionVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Execution {
    #[serde(rename = "RunAs")]
    pub run_as: Option<RunAs>,
    #[serde(rename = "IsolationMode")]
    pub isolation_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RunAs {
    #[serde(rename = "Uid")]
    pub uid: Option<usize>,
    #[serde(rename = "Gid")]
    pub gid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Function {
    #[serde(rename = "FunctionConfiguration")]
    pub function_configuration: FunctionConfiguration,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct FunctionConfiguration {
    #[serde(rename = "Executable")]
    pub executable: Option<String>,
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<usize>,
    #[serde(rename = "Pinned")]
    pub pinned: Option<bool>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
    #[serde(rename = "ExecArgs")]
    pub exec_args: Option<String>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,

}

#[derive(serde::Serialize, Default)]
pub struct Environment {
    #[serde(rename = "Variables")]
    pub variables: Option<()>,
    #[serde(rename = "AccessSysfs")]
    pub access_sysfs: Option<bool>,
    #[serde(rename = "Execution")]
    pub execution: Option<Execution>,
    #[serde(rename = "ResourceAccessPolicies")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultConfig {
    #[serde(rename = "Execution")]
    pub execution: Execution,

}

#[derive(serde::Serialize, Default)]
pub struct FunctionDefinitionVersion {
    #[serde(rename = "Functions")]
    pub functions: Vec<Function>,
    #[serde(rename = "DefaultConfig")]
    pub default_config: Option<DefaultConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceAccessPolicy {
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "Permission")]
    pub permission: Option<String>,

}


}

pub mod cfn_function_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnFunctionDefinitionVersion {
    /// No documentation provided by AWS
    #[serde(rename = "DefaultConfig")]
    pub default_config: Option<DefaultConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "FunctionDefinitionId")]
    pub function_definition_id: String,
    /// List of Function
    #[serde(rename = "Functions")]
    pub functions: Vec<Function>,

}


#[derive(serde::Serialize, Default)]
pub struct DefaultConfig {
    #[serde(rename = "Execution")]
    pub execution: Execution,

}

#[derive(serde::Serialize, Default)]
pub struct FunctionConfiguration {
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<usize>,
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,
    #[serde(rename = "Pinned")]
    pub pinned: Option<bool>,
    #[serde(rename = "Executable")]
    pub executable: Option<String>,
    #[serde(rename = "ExecArgs")]
    pub exec_args: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RunAs {
    #[serde(rename = "Gid")]
    pub gid: Option<usize>,
    #[serde(rename = "Uid")]
    pub uid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceAccessPolicy {
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "Permission")]
    pub permission: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Environment {
    #[serde(rename = "AccessSysfs")]
    pub access_sysfs: Option<bool>,
    #[serde(rename = "Execution")]
    pub execution: Option<Execution>,
    #[serde(rename = "ResourceAccessPolicies")]
    pub resource_access_policies: Option<Vec<ResourceAccessPolicy>>,
    #[serde(rename = "Variables")]
    pub variables: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Function {
    #[serde(rename = "FunctionConfiguration")]
    pub function_configuration: FunctionConfiguration,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Execution {
    #[serde(rename = "IsolationMode")]
    pub isolation_mode: Option<String>,
    #[serde(rename = "RunAs")]
    pub run_as: Option<RunAs>,

}


}

pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<GroupVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct GroupVersion {
    #[serde(rename = "FunctionDefinitionVersionArn")]
    pub function_definition_version_arn: Option<String>,
    #[serde(rename = "LoggerDefinitionVersionArn")]
    pub logger_definition_version_arn: Option<String>,
    #[serde(rename = "ResourceDefinitionVersionArn")]
    pub resource_definition_version_arn: Option<String>,
    #[serde(rename = "DeviceDefinitionVersionArn")]
    pub device_definition_version_arn: Option<String>,
    #[serde(rename = "CoreDefinitionVersionArn")]
    pub core_definition_version_arn: Option<String>,
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    pub connector_definition_version_arn: Option<String>,
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    pub subscription_definition_version_arn: Option<String>,

}


}

pub mod cfn_group_version {

#[derive(serde::Serialize, Default)]
pub struct CfnGroupVersion {
    /// No documentation provided by AWS
    #[serde(rename = "FunctionDefinitionVersionArn")]
    pub function_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectorDefinitionVersionArn")]
    pub connector_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DeviceDefinitionVersionArn")]
    pub device_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LoggerDefinitionVersionArn")]
    pub logger_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CoreDefinitionVersionArn")]
    pub core_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceDefinitionVersionArn")]
    pub resource_definition_version_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubscriptionDefinitionVersionArn")]
    pub subscription_definition_version_arn: Option<String>,

}



}

pub mod cfn_logger_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnLoggerDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<LoggerDefinitionVersion>,

}


#[derive(serde::Serialize, Default)]
pub struct LoggerDefinitionVersion {
    #[serde(rename = "Loggers")]
    pub loggers: Vec<Logger>,

}

#[derive(serde::Serialize, Default)]
pub struct Logger {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Component")]
    pub component: String,
    #[serde(rename = "Space")]
    pub space: Option<usize>,
    #[serde(rename = "Level")]
    pub level: String,
    #[serde(rename = "Type")]
    pub ty: String,

}


}

pub mod cfn_logger_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnLoggerDefinitionVersion {
    /// No documentation provided by AWS
    #[serde(rename = "LoggerDefinitionId")]
    pub logger_definition_id: String,
    /// List of Logger
    #[serde(rename = "Loggers")]
    pub loggers: Vec<Logger>,

}


#[derive(serde::Serialize, Default)]
pub struct Logger {
    #[serde(rename = "Level")]
    pub level: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Component")]
    pub component: String,
    #[serde(rename = "Space")]
    pub space: Option<usize>,

}


}

pub mod cfn_resource_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<ResourceDefinitionVersion>,

}


#[derive(serde::Serialize, Default)]
pub struct ResourceInstance {
    #[serde(rename = "ResourceDataContainer")]
    pub resource_data_container: ResourceDataContainer,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct LocalDeviceResourceData {
    #[serde(rename = "SourcePath")]
    pub source_path: String,
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,

}

#[derive(serde::Serialize, Default)]
pub struct SecretsManagerSecretResourceData {
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    pub additional_staging_labels_to_download: Option<Vec<String>>,
    #[serde(rename = "ARN")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct LocalVolumeResourceData {
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,
    #[serde(rename = "SourcePath")]
    pub source_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerMachineLearningModelResourceData {
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,
    #[serde(rename = "SageMakerJobArn")]
    pub sage_maker_job_arn: String,
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceDataContainer {
    #[serde(rename = "LocalDeviceResourceData")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,
    #[serde(rename = "SecretsManagerSecretResourceData")]
    pub secrets_manager_secret_resource_data: Option<SecretsManagerSecretResourceData>,
    #[serde(rename = "S3MachineLearningModelResourceData")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,
    #[serde(rename = "LocalVolumeResourceData")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    pub sage_maker_machine_learning_model_resource_data: Option<SageMakerMachineLearningModelResourceData>,

}

#[derive(serde::Serialize, Default)]
pub struct S3MachineLearningModelResourceData {
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceDefinitionVersion {
    #[serde(rename = "Resources")]
    pub resources: Vec<ResourceInstance>,

}

#[derive(serde::Serialize, Default)]
pub struct GroupOwnerSetting {
    #[serde(rename = "GroupOwner")]
    pub group_owner: Option<String>,
    #[serde(rename = "AutoAddGroupOwner")]
    pub auto_add_group_owner: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceDownloadOwnerSetting {
    #[serde(rename = "GroupPermission")]
    pub group_permission: String,
    #[serde(rename = "GroupOwner")]
    pub group_owner: String,

}


}

pub mod cfn_resource_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceDefinitionVersion {
    /// List of ResourceInstance
    #[serde(rename = "Resources")]
    pub resources: Vec<ResourceInstance>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct SageMakerMachineLearningModelResourceData {
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,
    #[serde(rename = "SageMakerJobArn")]
    pub sage_maker_job_arn: String,
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceDownloadOwnerSetting {
    #[serde(rename = "GroupPermission")]
    pub group_permission: String,
    #[serde(rename = "GroupOwner")]
    pub group_owner: String,

}

#[derive(serde::Serialize, Default)]
pub struct GroupOwnerSetting {
    #[serde(rename = "GroupOwner")]
    pub group_owner: Option<String>,
    #[serde(rename = "AutoAddGroupOwner")]
    pub auto_add_group_owner: bool,

}

#[derive(serde::Serialize, Default)]
pub struct LocalVolumeResourceData {
    #[serde(rename = "SourcePath")]
    pub source_path: String,
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceInstance {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ResourceDataContainer")]
    pub resource_data_container: ResourceDataContainer,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct LocalDeviceResourceData {
    #[serde(rename = "SourcePath")]
    pub source_path: String,
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,

}

#[derive(serde::Serialize, Default)]
pub struct SecretsManagerSecretResourceData {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    pub additional_staging_labels_to_download: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct S3MachineLearningModelResourceData {
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceDataContainer {
    #[serde(rename = "SecretsManagerSecretResourceData")]
    pub secrets_manager_secret_resource_data: Option<SecretsManagerSecretResourceData>,
    #[serde(rename = "S3MachineLearningModelResourceData")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,
    #[serde(rename = "LocalVolumeResourceData")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    pub sage_maker_machine_learning_model_resource_data: Option<SageMakerMachineLearningModelResourceData>,
    #[serde(rename = "LocalDeviceResourceData")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,

}


}

pub mod cfn_subscription_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnSubscriptionDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<SubscriptionDefinitionVersion>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct SubscriptionDefinitionVersion {
    #[serde(rename = "Subscriptions")]
    pub subscriptions: Vec<Subscription>,

}

#[derive(serde::Serialize, Default)]
pub struct Subscription {
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Source")]
    pub source: String,

}


}

pub mod cfn_subscription_definition_version {

#[derive(serde::Serialize, Default)]
pub struct CfnSubscriptionDefinitionVersion {
    /// No documentation provided by AWS
    #[serde(rename = "SubscriptionDefinitionId")]
    pub subscription_definition_id: String,
    /// List of Subscription
    #[serde(rename = "Subscriptions")]
    pub subscriptions: Vec<Subscription>,

}


#[derive(serde::Serialize, Default)]
pub struct Subscription {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "Target")]
    pub target: String,

}


}
