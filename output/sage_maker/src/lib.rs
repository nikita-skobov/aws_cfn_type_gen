
pub mod cfn_app {

#[derive(serde::Serialize, Default)]
pub struct CfnApp {
    /// The instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance.
    #[serde(rename = "ResourceSpec")]
    pub resource_spec: Option<ResourceSpec>,
    /// The type of app.
    #[serde(rename = "AppType")]
    pub app_type: String,
    /// A list of tags to apply to the app.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The user profile name.
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
    /// The name of the app.
    #[serde(rename = "AppName")]
    pub app_name: String,
    /// The domain ID.
    #[serde(rename = "DomainId")]
    pub domain_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceSpec {
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,

}


}

pub mod cfn_app_image_config {

#[derive(serde::Serialize, Default)]
pub struct CfnAppImageConfig {
    /// The Name of the AppImageConfig.
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,
    /// The KernelGatewayImageConfig.
    #[serde(rename = "KernelGatewayImageConfig")]
    pub kernel_gateway_image_config: Option<KernelGatewayImageConfig>,
    /// A list of tags to apply to the AppImageConfig.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct KernelGatewayImageConfig {
    #[serde(rename = "KernelSpecs")]
    pub kernel_specs: Vec<KernelSpec>,
    #[serde(rename = "FileSystemConfig")]
    pub file_system_config: Option<FileSystemConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct FileSystemConfig {
    #[serde(rename = "MountPath")]
    pub mount_path: Option<String>,
    #[serde(rename = "DefaultGid")]
    pub default_gid: Option<usize>,
    #[serde(rename = "DefaultUid")]
    pub default_uid: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct KernelSpec {
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_code_repository {

#[derive(serde::Serialize, Default)]
pub struct CfnCodeRepository {
    /// No documentation provided by AWS
    #[serde(rename = "GitConfig")]
    pub git_config: GitConfig,
    /// No documentation provided by AWS
    #[serde(rename = "CodeRepositoryName")]
    pub code_repository_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct GitConfig {
    #[serde(rename = "SecretArn")]
    pub secret_arn: Option<String>,
    #[serde(rename = "Branch")]
    pub branch: Option<String>,
    #[serde(rename = "RepositoryUrl")]
    pub repository_url: String,

}


}

pub mod cfn_data_quality_job_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnDataQualityJobDefinition {
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The name of the job definition.
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<JobDefinitionName>,
    /// Networking options for a job, such as network traffic encryption between containers, whether to allow inbound and outbound network calls to and from containers, and the VPC subnets and security groups to use for VPC-enabled jobs.
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,
    /// The name of the endpoint used to run the monitoring job.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,
    /// Specifies a time limit for how long the monitoring job is allowed to run.
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,
    /// No documentation provided by AWS
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,
    /// Baseline configuration used to validate that the data conforms to the specified constraints and statistics.
    #[serde(rename = "DataQualityBaselineConfig")]
    pub data_quality_baseline_config: Option<DataQualityBaselineConfig>,
    /// The inputs for a monitoring job.
    #[serde(rename = "DataQualityJobInput")]
    pub data_quality_job_input: DataQualityJobInput,
    /// No documentation provided by AWS
    #[serde(rename = "DataQualityJobOutputConfig")]
    pub data_quality_job_output_config: MonitoringOutputConfig,
    /// Container image configuration object for the monitoring job.
    #[serde(rename = "DataQualityAppSpecification")]
    pub data_quality_app_specification: DataQualityAppSpecification,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct DataQualityBaselineConfig {
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,
    #[serde(rename = "BaseliningJobName")]
    pub baselining_job_name: Option<ProcessingJobName>,
    #[serde(rename = "StatisticsResource")]
    pub statistics_resource: Option<StatisticsResource>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}
pub type Parquet = bool;pub type EndpointName = String;
#[derive(serde::Serialize, Default)]
pub struct MonitoringResources {
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}
pub type JobDefinitionName = String;
#[derive(serde::Serialize, Default)]
pub struct Json {
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct StatisticsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetFormat {
    #[serde(rename = "Parquet")]
    pub parquet: Option<Parquet>,
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,
    #[serde(rename = "Json")]
    pub json: Option<Json>,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointInput {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutput {
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfig {
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct StoppingCondition {
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,

}
pub type S3Uri = String;
#[derive(serde::Serialize, Default)]
pub struct DataQualityAppSpecification {
    #[serde(rename = "Environment")]
    pub environment: Option<()>,
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    pub post_analytics_processor_source_uri: Option<S3Uri>,
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
    #[serde(rename = "ContainerArguments")]
    pub container_arguments: Option<Vec<String>>,
    #[serde(rename = "ContainerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,
    #[serde(rename = "RecordPreprocessorSourceUri")]
    pub record_preprocessor_source_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct ConstraintsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchTransformInput {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataQualityJobInput {
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutputConfig {
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Output {
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,

}
pub type ProcessingJobName = String;

}

pub mod cfn_device {

#[derive(serde::Serialize, Default)]
pub struct CfnDevice {
    /// The name of the edge device fleet
    #[serde(rename = "DeviceFleetName")]
    pub device_fleet_name: String,
    /// Associate tags with the resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Edge Device you want to register against a device fleet
    #[serde(rename = "Device")]
    pub device: Option<Device>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Device {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "IotThingName")]
    pub iot_thing_name: Option<String>,
    #[serde(rename = "DeviceName")]
    pub device_name: String,

}


}

pub mod cfn_device_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnDeviceFleet {
    /// Associate tags with the resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the edge device fleet
    #[serde(rename = "DeviceFleetName")]
    pub device_fleet_name: String,
    /// Role associated with the device fleet
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// S3 bucket and an ecryption key id (if available) to store outputs for the fleet
    #[serde(rename = "OutputConfig")]
    pub output_config: EdgeOutputConfig,
    /// Description for the edge device fleet
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct EdgeOutputConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "S3OutputLocation")]
    pub s3_output_location: String,

}


}

pub mod cfn_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnDomain {
    /// The default space settings.
    #[serde(rename = "DefaultSpaceSettings")]
    pub default_space_settings: Option<DefaultSpaceSettings>,
    /// A name for the domain.
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// The entity that creates and manages the required security groups for inter-app communication in VPCOnly mode. Required when CreateDomain.AppNetworkAccessType is VPCOnly and DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn is provided.
    #[serde(rename = "AppSecurityGroupManagement")]
    pub app_security_group_management: Option<String>,
    /// A collection of Domain settings.
    #[serde(rename = "DomainSettings")]
    pub domain_settings: Option<DomainSettings>,
    /// The mode of authentication that members use to access the domain.
    #[serde(rename = "AuthMode")]
    pub auth_mode: String,
    /// The VPC subnets that Studio uses for communication.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// A list of tags to apply to the user profile.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the Amazon Virtual Private Cloud (VPC) that Studio uses for communication.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The default user settings.
    #[serde(rename = "DefaultUserSettings")]
    pub default_user_settings: UserSettings,
    /// SageMaker uses AWS KMS to encrypt the EFS volume attached to the domain with an AWS managed customer master key (CMK) by default.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// Specifies the VPC used for non-EFS traffic. The default value is PublicInternetOnly.
    #[serde(rename = "AppNetworkAccessType")]
    pub app_network_access_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct SharingSettings {
    #[serde(rename = "S3KmsKeyId")]
    pub s3_kms_key_id: Option<String>,
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,
    #[serde(rename = "NotebookOutputOption")]
    pub notebook_output_option: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultSpaceSettings {
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct JupyterServerAppSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct RStudioServerProAppSettings {
    #[serde(rename = "UserGroup")]
    pub user_group: Option<String>,
    #[serde(rename = "AccessStatus")]
    pub access_status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KernelGatewayAppSettings {
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}

#[derive(serde::Serialize, Default)]
pub struct DomainSettings {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "RStudioServerProDomainSettings")]
    pub rstudio_server_pro_domain_settings: Option<RStudioServerProDomainSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct RStudioServerProDomainSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,
    #[serde(rename = "RStudioConnectUrl")]
    pub rstudio_connect_url: Option<String>,
    #[serde(rename = "DomainExecutionRoleArn")]
    pub domain_execution_role_arn: String,
    #[serde(rename = "RStudioPackageManagerUrl")]
    pub rstudio_package_manager_url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RSessionAppSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomImage {
    #[serde(rename = "ImageVersionNumber")]
    pub image_version_number: Option<usize>,
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,
    #[serde(rename = "ImageName")]
    pub image_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct UserSettings {
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,
    #[serde(rename = "SharingSettings")]
    pub sharing_settings: Option<SharingSettings>,
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,
    #[serde(rename = "RStudioServerProAppSettings")]
    pub rstudio_server_pro_app_settings: Option<RStudioServerProAppSettings>,
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "RSessionAppSettings")]
    pub rsession_app_settings: Option<RSessionAppSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceSpec {
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(rename = "LifecycleConfigArn")]
    pub lifecycle_config_arn: Option<String>,

}


}

pub mod cfn_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpoint {
    /// List of VariantProperty
    #[serde(rename = "ExcludeRetainedVariantProperties")]
    pub exclude_retained_variant_properties: Option<Vec<VariantProperty>>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RetainAllVariantProperties")]
    pub retain_all_variant_properties: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RetainDeploymentConfig")]
    pub retain_deployment_config: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "DeploymentConfig")]
    pub deployment_config: Option<DeploymentConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct CapacitySize {
    #[serde(rename = "Value")]
    pub value: usize,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct AutoRollbackConfig {
    #[serde(rename = "Alarms")]
    pub alarms: Vec<Alarm>,

}

#[derive(serde::Serialize, Default)]
pub struct TrafficRoutingConfig {
    #[serde(rename = "WaitIntervalInSeconds")]
    pub wait_interval_in_seconds: Option<usize>,
    #[serde(rename = "LinearStepSize")]
    pub linear_step_size: Option<CapacitySize>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "CanarySize")]
    pub canary_size: Option<CapacitySize>,

}

#[derive(serde::Serialize, Default)]
pub struct VariantProperty {
    #[serde(rename = "VariantPropertyType")]
    pub variant_property_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct BlueGreenUpdatePolicy {
    #[serde(rename = "TrafficRoutingConfiguration")]
    pub traffic_routing_configuration: TrafficRoutingConfig,
    #[serde(rename = "TerminationWaitInSeconds")]
    pub termination_wait_in_seconds: Option<usize>,
    #[serde(rename = "MaximumExecutionTimeoutInSeconds")]
    pub maximum_execution_timeout_in_seconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct DeploymentConfig {
    #[serde(rename = "BlueGreenUpdatePolicy")]
    pub blue_green_update_policy: BlueGreenUpdatePolicy,
    #[serde(rename = "AutoRollbackConfiguration")]
    pub auto_rollback_configuration: Option<AutoRollbackConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct Alarm {
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,

}


}

pub mod cfn_endpoint_config {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpointConfig {
    /// No documentation provided by AWS
    #[serde(rename = "AsyncInferenceConfig")]
    pub async_inference_config: Option<AsyncInferenceConfig>,
    /// List of ProductionVariant
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,
    /// No documentation provided by AWS
    #[serde(rename = "ExplainerConfig")]
    pub explainer_config: Option<ExplainerConfig>,
    /// List of ProductionVariant
    #[serde(rename = "ShadowProductionVariants")]
    pub shadow_production_variants: Option<Vec<ProductionVariant>>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DataCaptureConfig")]
    pub data_capture_config: Option<DataCaptureConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct ClarifyExplainerConfig {
    #[serde(rename = "ShapConfig")]
    pub shap_config: ClarifyShapConfig,
    #[serde(rename = "InferenceConfig")]
    pub inference_config: Option<ClarifyInferenceConfig>,
    #[serde(rename = "EnableExplanations")]
    pub enable_explanations: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AsyncInferenceConfig {
    #[serde(rename = "OutputConfig")]
    pub output_config: AsyncInferenceOutputConfig,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<AsyncInferenceClientConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ServerlessConfig {
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: usize,
    #[serde(rename = "MemorySizeInMB")]
    pub memory_size_in_mb: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyInferenceConfig {
    #[serde(rename = "ContentTemplate")]
    pub content_template: Option<String>,
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,
    #[serde(rename = "MaxRecordCount")]
    pub max_record_count: Option<usize>,
    #[serde(rename = "FeatureTypes")]
    pub feature_types: Option<Vec<ClarifyFeatureType>>,
    #[serde(rename = "MaxPayloadInMB")]
    pub max_payload_in_mb: Option<usize>,
    #[serde(rename = "ProbabilityIndex")]
    pub probability_index: Option<usize>,
    #[serde(rename = "LabelIndex")]
    pub label_index: Option<usize>,
    #[serde(rename = "LabelHeaders")]
    pub label_headers: Option<Vec<ClarifyHeader>>,
    #[serde(rename = "FeatureHeaders")]
    pub feature_headers: Option<Vec<ClarifyHeader>>,
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "LabelAttribute")]
    pub label_attribute: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyHeader {

}

#[derive(serde::Serialize, Default)]
pub struct CaptureContentTypeHeader {
    #[serde(rename = "CsvContentTypes")]
    pub csv_content_types: Option<Vec<String>>,
    #[serde(rename = "JsonContentTypes")]
    pub json_content_types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ProductionVariant {
    #[serde(rename = "AcceleratorType")]
    pub accelerator_type: Option<String>,
    #[serde(rename = "ModelDataDownloadTimeoutInSeconds")]
    pub model_data_download_timeout_in_seconds: Option<usize>,
    #[serde(rename = "ModelName")]
    pub model_name: String,
    #[serde(rename = "InitialVariantWeight")]
    pub initial_variant_weight: f64,
    #[serde(rename = "EnableSSMAccess")]
    pub enable_ssmaccess: Option<bool>,
    #[serde(rename = "ContainerStartupHealthCheckTimeoutInSeconds")]
    pub container_startup_health_check_timeout_in_seconds: Option<usize>,
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: Option<usize>,
    #[serde(rename = "ServerlessConfig")]
    pub serverless_config: Option<ServerlessConfig>,
    #[serde(rename = "VariantName")]
    pub variant_name: String,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "InitialInstanceCount")]
    pub initial_instance_count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyTextConfig {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Granularity")]
    pub granularity: String,

}

#[derive(serde::Serialize, Default)]
pub struct CaptureOption {
    #[serde(rename = "CaptureMode")]
    pub capture_mode: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExplainerConfig {
    #[serde(rename = "ClarifyExplainerConfig")]
    pub clarify_explainer_config: Option<ClarifyExplainerConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyShapBaselineConfig {
    #[serde(rename = "ShapBaseline")]
    pub shap_baseline: Option<String>,
    #[serde(rename = "ShapBaselineUri")]
    pub shap_baseline_uri: Option<String>,
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AsyncInferenceOutputConfig {
    #[serde(rename = "NotificationConfig")]
    pub notification_config: Option<AsyncInferenceNotificationConfig>,
    #[serde(rename = "S3FailurePath")]
    pub s3_failure_path: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyFeatureType {

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ClarifyShapConfig {
    #[serde(rename = "NumberOfSamples")]
    pub number_of_samples: Option<usize>,
    #[serde(rename = "Seed")]
    pub seed: Option<usize>,
    #[serde(rename = "TextConfig")]
    pub text_config: Option<ClarifyTextConfig>,
    #[serde(rename = "UseLogit")]
    pub use_logit: Option<bool>,
    #[serde(rename = "ShapBaselineConfig")]
    pub shap_baseline_config: ClarifyShapBaselineConfig,

}

#[derive(serde::Serialize, Default)]
pub struct DataCaptureConfig {
    #[serde(rename = "DestinationS3Uri")]
    pub destination_s3_uri: String,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "CaptureContentTypeHeader")]
    pub capture_content_type_header: Option<CaptureContentTypeHeader>,
    #[serde(rename = "InitialSamplingPercentage")]
    pub initial_sampling_percentage: usize,
    #[serde(rename = "CaptureOptions")]
    pub capture_options: Vec<CaptureOption>,
    #[serde(rename = "EnableCapture")]
    pub enable_capture: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AsyncInferenceClientConfig {
    #[serde(rename = "MaxConcurrentInvocationsPerInstance")]
    pub max_concurrent_invocations_per_instance: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AsyncInferenceNotificationConfig {
    #[serde(rename = "IncludeInferenceResponseIn")]
    pub include_inference_response_in: Option<Vec<String>>,
    #[serde(rename = "SuccessTopic")]
    pub success_topic: Option<String>,
    #[serde(rename = "ErrorTopic")]
    pub error_topic: Option<String>,

}


}

pub mod cfn_feature_group {

#[derive(serde::Serialize, Default)]
pub struct CfnFeatureGroup {
    /// No documentation provided by AWS
    #[serde(rename = "OnlineStoreConfig")]
    pub online_store_config: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "OfflineStoreConfig")]
    pub offline_store_config: Option<()>,
    /// Role Arn
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// The Event Time Feature Name.
    #[serde(rename = "EventTimeFeatureName")]
    pub event_time_feature_name: String,
    /// The Name of the FeatureGroup.
    #[serde(rename = "FeatureGroupName")]
    pub feature_group_name: String,
    /// An Array of Feature Definition
    #[serde(rename = "FeatureDefinitions")]
    pub feature_definitions: Vec<FeatureDefinition>,
    /// Description about the FeatureGroup.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// An array of key-value pair to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Record Identifier Feature Name.
    #[serde(rename = "RecordIdentifierFeatureName")]
    pub record_identifier_feature_name: String,

}

pub type KmsKeyId = String;
#[derive(serde::Serialize, Default)]
pub struct OnlineStoreSecurityConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<KmsKeyId>,

}

#[derive(serde::Serialize, Default)]
pub struct FeatureDefinition {
    #[serde(rename = "FeatureName")]
    pub feature_name: String,
    #[serde(rename = "FeatureType")]
    pub feature_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3StorageConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<KmsKeyId>,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type TableFormat = String;
#[derive(serde::Serialize, Default)]
pub struct DataCatalogConfig {
    #[serde(rename = "Catalog")]
    pub catalog: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Database")]
    pub database: String,

}


}

pub mod cfn_image {

#[derive(serde::Serialize, Default)]
pub struct CfnImage {
    /// A description of the image.
    #[serde(rename = "ImageDescription")]
    pub image_description: Option<ImageDescription>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on behalf of the customer.
    #[serde(rename = "ImageRoleArn")]
    pub image_role_arn: ImageRoleArn,
    /// The name of the image.
    #[serde(rename = "ImageName")]
    pub image_name: ImageName,
    /// The display name of the image.
    #[serde(rename = "ImageDisplayName")]
    pub image_display_name: Option<ImageDisplayName>,

}

pub type ImageArn = String;pub type ImageRoleArn = String;pub type ImageDisplayName = String;pub type ImageDescription = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type ImageName = String;

}

pub mod cfn_image_version {

#[derive(serde::Serialize, Default)]
pub struct CfnImageVersion {
    /// The name of the image this version belongs to.
    #[serde(rename = "ImageName")]
    pub image_name: ImageName,
    /// The registry path of the container image on which this image version is based.
    #[serde(rename = "BaseImage")]
    pub base_image: BaseImage,

}

pub type ImageArn = String;pub type ImageName = String;pub type BaseImage = String;pub type Version = usize;pub type ImageVersionArn = String;pub type ContainerImage = String;

}

pub mod cfn_inference_experiment {

#[derive(serde::Serialize, Default)]
pub struct CfnInferenceExperiment {
    /// The Amazon S3 location and configuration for storing inference request and response data.
    #[serde(rename = "DataStorageConfig")]
    pub data_storage_config: Option<DataStorageConfig>,
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute instance that hosts the endpoint.
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,
    /// The desired state of the experiment after starting or stopping operation.
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage Amazon SageMaker Inference endpoints for model deployment.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The name of the endpoint used to run the inference experiment.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    /// The type of the inference experiment that you want to run.
    #[serde(rename = "Type")]
    pub ty: String,
    /// The description of the inference experiment.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The configuration of ShadowMode inference experiment type. Use this field to specify a production variant which takes all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant also specify the percentage of requests that Amazon SageMaker replicates.
    #[serde(rename = "ShadowModeConfig")]
    pub shadow_mode_config: Option<ShadowModeConfig>,
    /// The error message or client-specified reason from the StopInferenceExperiment API, that explains the status of the inference experiment.
    #[serde(rename = "StatusReason")]
    pub status_reason: Option<String>,
    /// An array of ModelVariantConfig objects. Each ModelVariantConfig object in the array describes the infrastructure configuration for the corresponding variant.
    #[serde(rename = "ModelVariants")]
    pub model_variants: Vec<ModelVariantConfig>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: Option<InferenceExperimentSchedule>,
    /// The name for the inference experiment.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct EndpointMetadata {
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: Option<String>,
    #[serde(rename = "EndpointStatus")]
    pub endpoint_status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelVariantConfig {
    #[serde(rename = "VariantName")]
    pub variant_name: String,
    #[serde(rename = "InfrastructureConfig")]
    pub infrastructure_config: ModelInfrastructureConfig,
    #[serde(rename = "ModelName")]
    pub model_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct InferenceExperimentSchedule {
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RealTimeInferenceConfig {
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ModelInfrastructureConfig {
    #[serde(rename = "InfrastructureType")]
    pub infrastructure_type: String,
    #[serde(rename = "RealTimeInferenceConfig")]
    pub real_time_inference_config: RealTimeInferenceConfig,

}

#[derive(serde::Serialize, Default)]
pub struct ShadowModeConfig {
    #[serde(rename = "ShadowModelVariants")]
    pub shadow_model_variants: Vec<ShadowModelVariantConfig>,
    #[serde(rename = "SourceModelVariantName")]
    pub source_model_variant_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct CaptureContentTypeHeader {
    #[serde(rename = "JsonContentTypes")]
    pub json_content_types: Option<Vec<String>>,
    #[serde(rename = "CsvContentTypes")]
    pub csv_content_types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DataStorageConfig {
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "KmsKey")]
    pub kms_key: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<CaptureContentTypeHeader>,

}

#[derive(serde::Serialize, Default)]
pub struct ShadowModelVariantConfig {
    #[serde(rename = "SamplingPercentage")]
    pub sampling_percentage: usize,
    #[serde(rename = "ShadowModelVariantName")]
    pub shadow_model_variant_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type EndpointName = String;

}

pub mod cfn_model {

#[derive(serde::Serialize, Default)]
pub struct CfnModel {
    /// List of ContainerDefinition
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<ContainerDefinition>>,
    /// No documentation provided by AWS
    #[serde(rename = "InferenceExecutionConfig")]
    pub inference_execution_config: Option<InferenceExecutionConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "PrimaryContainer")]
    pub primary_container: Option<ContainerDefinition>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct RepositoryAuthConfig {
    #[serde(rename = "RepositoryCredentialsProviderArn")]
    pub repository_credentials_provider_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerDefinition {
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,
    #[serde(rename = "ContainerHostname")]
    pub container_hostname: Option<String>,
    #[serde(rename = "ImageConfig")]
    pub image_config: Option<ImageConfig>,
    #[serde(rename = "Environment")]
    pub environment: Option<()>,
    #[serde(rename = "InferenceSpecificationName")]
    pub inference_specification_name: Option<String>,
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<String>,
    #[serde(rename = "MultiModelConfig")]
    pub multi_model_config: Option<MultiModelConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct InferenceExecutionConfig {
    #[serde(rename = "Mode")]
    pub mode: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ImageConfig {
    #[serde(rename = "RepositoryAccessMode")]
    pub repository_access_mode: String,
    #[serde(rename = "RepositoryAuthConfig")]
    pub repository_auth_config: Option<RepositoryAuthConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MultiModelConfig {
    #[serde(rename = "ModelCacheSetting")]
    pub model_cache_setting: Option<String>,

}


}

pub mod cfn_model_bias_job_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnModelBiasJobDefinition {
    /// Networking options for a job, such as network traffic encryption between containers, whether to allow inbound and outbound network calls to and from containers, and the VPC subnets and security groups to use for VPC-enabled jobs.
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelBiasJobOutputConfig")]
    pub model_bias_job_output_config: MonitoringOutputConfig,
    /// Specifies a time limit for how long the monitoring job is allowed to run.
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,
    /// Baseline configuration used to validate that the data conforms to the specified constraints and statistics.
    #[serde(rename = "ModelBiasBaselineConfig")]
    pub model_bias_baseline_config: Option<ModelBiasBaselineConfig>,
    /// The name of the job definition.
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<JobDefinitionName>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,
    /// The name of the endpoint used to run the monitoring job.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The inputs for a monitoring job.
    #[serde(rename = "ModelBiasJobInput")]
    pub model_bias_job_input: ModelBiasJobInput,
    /// Container image configuration object for the monitoring job.
    #[serde(rename = "ModelBiasAppSpecification")]
    pub model_bias_app_specification: ModelBiasAppSpecification,

}


#[derive(serde::Serialize, Default)]
pub struct MonitoringOutputConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringResources {
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}
pub type EndpointName = String;pub type MonitoringTimeOffsetString = String;
#[derive(serde::Serialize, Default)]
pub struct DatasetFormat {
    #[serde(rename = "Json")]
    pub json: Option<Json>,
    #[serde(rename = "Parquet")]
    pub parquet: Option<Parquet>,
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfig {
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelBiasBaselineConfig {
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,
    #[serde(rename = "BaseliningJobName")]
    pub baselining_job_name: Option<ProcessingJobName>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchTransformInput {
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,

}
pub type S3Uri = String;
#[derive(serde::Serialize, Default)]
pub struct S3Output {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelBiasJobInput {
    #[serde(rename = "GroundTruthS3Input")]
    pub ground_truth_s3_input: MonitoringGroundTruthS3Input,
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutput {
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}

#[derive(serde::Serialize, Default)]
pub struct StoppingCondition {
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ModelBiasAppSpecification {
    #[serde(rename = "Environment")]
    pub environment: Option<()>,
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
    #[serde(rename = "ConfigUri")]
    pub config_uri: S3Uri,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}
pub type JobDefinitionName = String;pub type ProcessingJobName = String;
#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Json {
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringGroundTruthS3Input {
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}
pub type Parquet = bool;
#[derive(serde::Serialize, Default)]
pub struct ConstraintsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointInput {
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,

}


}

pub mod cfn_model_card {

#[derive(serde::Serialize, Default)]
pub struct CfnModelCard {
    /// The unique name of the model card.
    #[serde(rename = "ModelCardName")]
    pub model_card_name: String,
    /// Information about the user who created or modified an experiment, trial, trial component, lineage group, project, or model card.
    #[serde(rename = "LastModifiedBy")]
    pub last_modified_by: Option<UserContext>,
    /// The approval status of the model card within your organization. Different organizations might have different criteria for model card review and approval.
    #[serde(rename = "ModelCardStatus")]
    pub model_card_status: String,
    /// Key-value pairs used to manage metadata for model cards.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Information about the user who created or modified an experiment, trial, trial component, lineage group, project, or model card.
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<UserContext>,
    /// An optional Key Management Service key to encrypt, decrypt, and re-encrypt model card content for regulated workloads with highly sensitive data.
    /// 
    #[serde(rename = "SecurityConfig")]
    pub security_config: Option<SecurityConfig>,
    /// The content of the model card.
    #[serde(rename = "Content")]
    pub content: Content,

}


#[derive(serde::Serialize, Default)]
pub struct InferenceSpecification {
    #[serde(rename = "Containers")]
    pub containers: Vec<Container>,

}

#[derive(serde::Serialize, Default)]
pub struct BarChartMetric {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "XAxisName")]
    pub xaxis_name: Option<AxisNameArray>,
    #[serde(rename = "YAxisName")]
    pub yaxis_name: Option<AxisNameString>,
    #[serde(rename = "Value")]
    pub value: (),
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct MatrixMetric {
    #[serde(rename = "XAxisName")]
    pub xaxis_name: Option<AxisNameArray>,
    #[serde(rename = "YAxisName")]
    pub yaxis_name: Option<AxisNameArray>,
    #[serde(rename = "Value")]
    pub value: (),
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

}
pub type RiskRating = String;
#[derive(serde::Serialize, Default)]
pub struct SimpleMetric {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "YAxisName")]
    pub yaxis_name: Option<AxisNameString>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Value")]
    pub value: (),
    #[serde(rename = "XAxisName")]
    pub xaxis_name: Option<AxisNameString>,

}

#[derive(serde::Serialize, Default)]
pub struct Container {
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,
    #[serde(rename = "NearestModelName")]
    pub nearest_model_name: Option<String>,

}
pub type AxisNameString = String;pub type AxisNameArray = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct SecurityConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct UserContext {
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,
    #[serde(rename = "UserProfileArn")]
    pub user_profile_arn: Option<String>,
    #[serde(rename = "DomainId")]
    pub domain_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Content {
    #[serde(rename = "ModelOverview")]
    pub model_overview: Option<ModelOverview>,
    #[serde(rename = "TrainingDetails")]
    pub training_details: Option<TrainingDetails>,
    #[serde(rename = "EvaluationDetails")]
    pub evaluation_details: Option<EvaluationDetails>,
    #[serde(rename = "AdditionalInformation")]
    pub additional_information: Option<AdditionalInformation>,
    #[serde(rename = "BusinessDetails")]
    pub business_details: Option<BusinessDetails>,
    #[serde(rename = "IntendedUses")]
    pub intended_uses: Option<IntendedUses>,
    #[serde(rename = "ModelPackageDetails")]
    pub model_package_details: Option<ModelPackageDetails>,

}

#[derive(serde::Serialize, Default)]
pub struct ObjectiveFunction {
    #[serde(rename = "Function")]
    pub function: Option<()>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelPackageDetails {
    #[serde(rename = "ModelPackageStatus")]
    pub model_package_status: Option<String>,
    #[serde(rename = "ModelPackageVersion")]
    pub model_package_version: Option<f64>,
    #[serde(rename = "ModelPackageArn")]
    pub model_package_arn: Option<String>,
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    #[serde(rename = "ModelPackageDescription")]
    pub model_package_description: Option<String>,
    #[serde(rename = "ApprovalDescription")]
    pub approval_description: Option<String>,
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<String>,
    #[serde(rename = "SourceAlgorithms")]
    pub source_algorithms: Option<SourceAlgorithms>,
    #[serde(rename = "Task")]
    pub task: Option<String>,
    #[serde(rename = "InferenceSpecification")]
    pub inference_specification: Option<InferenceSpecification>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<ModelPackageCreator>,
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: Option<String>,
    #[serde(rename = "ModelApprovalStatus")]
    pub model_approval_status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TrainingMetric {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct LinearGraphMetric {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Value")]
    pub value: (),
    #[serde(rename = "YAxisName")]
    pub yaxis_name: Option<AxisNameString>,
    #[serde(rename = "XAxisName")]
    pub xaxis_name: Option<AxisNameString>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct IntendedUses {
    #[serde(rename = "ExplanationsForRiskRating")]
    pub explanations_for_risk_rating: Option<String>,
    #[serde(rename = "FactorsAffectingModelEfficiency")]
    pub factors_affecting_model_efficiency: Option<String>,
    #[serde(rename = "PurposeOfModel")]
    pub purpose_of_model: Option<String>,
    #[serde(rename = "IntendedUses")]
    pub intended_uses: Option<String>,
    #[serde(rename = "RiskRating")]
    pub risk_rating: Option<RiskRating>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceAlgorithms {

}

#[derive(serde::Serialize, Default)]
pub struct TrainingDetails {
    #[serde(rename = "TrainingJobDetails")]
    pub training_job_details: Option<()>,
    #[serde(rename = "ObjectiveFunction")]
    pub objective_function: Option<ObjectiveFunction>,
    #[serde(rename = "TrainingObservations")]
    pub training_observations: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceAlgorithm {
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelOverview {
    #[serde(rename = "ModelName")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelOwner")]
    pub model_owner: Option<String>,
    #[serde(rename = "AlgorithmType")]
    pub algorithm_type: Option<String>,
    #[serde(rename = "ModelVersion")]
    pub model_version: Option<f64>,
    #[serde(rename = "ModelArtifact")]
    pub model_artifact: Option<Vec<String>>,
    #[serde(rename = "ProblemType")]
    pub problem_type: Option<String>,
    #[serde(rename = "InferenceEnvironment")]
    pub inference_environment: Option<()>,
    #[serde(rename = "ModelCreator")]
    pub model_creator: Option<String>,
    #[serde(rename = "ModelId")]
    pub model_id: Option<String>,
    #[serde(rename = "ModelDescription")]
    pub model_description: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TrainingHyperParameter {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct EvaluationDetails {

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalInformation {
    #[serde(rename = "EthicalConsiderations")]
    pub ethical_considerations: Option<String>,
    #[serde(rename = "CaveatsAndRecommendations")]
    pub caveats_and_recommendations: Option<String>,
    #[serde(rename = "CustomDetails")]
    pub custom_details: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelPackageCreator {
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BusinessDetails {
    #[serde(rename = "BusinessStakeholders")]
    pub business_stakeholders: Option<String>,
    #[serde(rename = "BusinessProblem")]
    pub business_problem: Option<String>,
    #[serde(rename = "LineOfBusiness")]
    pub line_of_business: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricGroup {
    #[serde(rename = "MetricData")]
    pub metric_data: (),
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct EvaluationDetail {
    #[serde(rename = "EvaluationObservation")]
    pub evaluation_observation: Option<String>,
    #[serde(rename = "Metadata")]
    pub metadata: Option<()>,
    #[serde(rename = "Datasets")]
    pub datasets: Option<Vec<String>>,
    #[serde(rename = "EvaluationJobArn")]
    pub evaluation_job_arn: Option<String>,
    #[serde(rename = "MetricGroups")]
    pub metric_groups: Option<Vec<MetricGroup>>,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_model_explainability_job_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnModelExplainabilityJobDefinition {
    /// No documentation provided by AWS
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,
    /// Specifies a time limit for how long the monitoring job is allowed to run.
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,
    /// Networking options for a job, such as network traffic encryption between containers, whether to allow inbound and outbound network calls to and from containers, and the VPC subnets and security groups to use for VPC-enabled jobs.
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,
    /// The inputs for a monitoring job.
    #[serde(rename = "ModelExplainabilityJobInput")]
    pub model_explainability_job_input: ModelExplainabilityJobInput,
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelExplainabilityJobOutputConfig")]
    pub model_explainability_job_output_config: MonitoringOutputConfig,
    /// Baseline configuration used to validate that the data conforms to the specified constraints and statistics.
    #[serde(rename = "ModelExplainabilityBaselineConfig")]
    pub model_explainability_baseline_config: Option<ModelExplainabilityBaselineConfig>,
    /// Container image configuration object for the monitoring job.
    #[serde(rename = "ModelExplainabilityAppSpecification")]
    pub model_explainability_app_specification: ModelExplainabilityAppSpecification,
    /// The name of the job definition.
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<JobDefinitionName>,
    /// The name of the endpoint used to run the monitoring job.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,

}

pub type EndpointName = String;
#[derive(serde::Serialize, Default)]
pub struct DatasetFormat {
    #[serde(rename = "Parquet")]
    pub parquet: Option<Parquet>,
    #[serde(rename = "Json")]
    pub json: Option<Json>,
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,

}
pub type Parquet = bool;
#[derive(serde::Serialize, Default)]
pub struct S3Output {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointInput {
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfig {
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelExplainabilityAppSpecification {
    #[serde(rename = "ConfigUri")]
    pub config_uri: S3Uri,
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
    #[serde(rename = "Environment")]
    pub environment: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutput {
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Json {
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}
pub type MonitoringTimeOffsetString = String;
#[derive(serde::Serialize, Default)]
pub struct ModelExplainabilityJobInput {
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,

}

#[derive(serde::Serialize, Default)]
pub struct ConstraintsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchTransformInput {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutputConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,

}
pub type S3Uri = String;
#[derive(serde::Serialize, Default)]
pub struct MonitoringResources {
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ModelExplainabilityBaselineConfig {
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,
    #[serde(rename = "BaseliningJobName")]
    pub baselining_job_name: Option<ProcessingJobName>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StoppingCondition {
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: usize,

}
pub type JobDefinitionName = String;pub type ProcessingJobName = String;

}

pub mod cfn_model_package {

#[derive(serde::Serialize, Default)]
pub struct CfnModelPackage {
    /// The Amazon Simple Storage Service (Amazon S3) path where the sample payload are stored pointing to single gzip compressed tar archive.
    #[serde(rename = "SamplePayloadUrl")]
    pub sample_payload_url: Option<SamplePayloadUrl>,
    /// The machine learning domain of the model package you specified.
    #[serde(rename = "Domain")]
    pub domain: Option<Domain>,
    /// An array of additional Inference Specification objects.
    #[serde(rename = "AdditionalInferenceSpecifications")]
    pub additional_inference_specifications: Option<AdditionalInferenceSpecifications>,
    /// Details about inference jobs that can be run with models based on this model package.
    #[serde(rename = "InferenceSpecification")]
    pub inference_specification: Option<InferenceSpecification>,
    /// Details about the current status of the model package.
    #[serde(rename = "ModelPackageStatusDetails")]
    pub model_package_status_details: Option<ModelPackageStatusDetails>,
    /// No documentation provided by AWS
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<CreatedBy>,
    /// Metadata properties of the tracking entity, trial, or trial component.
    #[serde(rename = "MetadataProperties")]
    pub metadata_properties: Option<MetadataProperties>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The metadata properties associated with the model package versions.
    #[serde(rename = "CustomerMetadataProperties")]
    pub customer_metadata_properties: Option<CustomerMetadataProperties>,
    /// The name of the model package group.
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: Option<ModelPackageGroupName>,
    /// Represents the overall status of a model package.
    #[serde(rename = "ModelPackageStatusItem")]
    pub model_package_status_item: Option<ModelPackageStatusItem>,
    /// A unique token that guarantees that the call to this API is idempotent.
    #[serde(rename = "ClientToken")]
    pub client_token: Option<ClientToken>,
    /// Additional Inference Specification specifies details about inference jobs that can be run with models based on this model package.AdditionalInferenceSpecifications can be added to existing model packages using AdditionalInferenceSpecificationsToAdd.
    #[serde(rename = "AdditionalInferenceSpecificationDefinition")]
    pub additional_inference_specification_definition: Option<AdditionalInferenceSpecificationDefinition>,
    /// The version of the model package.
    #[serde(rename = "ModelPackageVersion")]
    pub model_package_version: Option<ModelPackageVersion>,
    /// Details about the algorithm that was used to create the model package.
    #[serde(rename = "SourceAlgorithmSpecification")]
    pub source_algorithm_specification: Option<SourceAlgorithmSpecification>,
    /// The approval status of the model package.
    #[serde(rename = "ModelApprovalStatus")]
    pub model_approval_status: Option<ModelApprovalStatus>,
    /// The machine learning task your model package accomplishes.
    #[serde(rename = "Task")]
    pub task: Option<Task>,
    /// No documentation provided by AWS
    #[serde(rename = "LastModifiedBy")]
    pub last_modified_by: Option<LastModifiedBy>,
    /// Whether to certify the model package for listing on AWS Marketplace.
    #[serde(rename = "CertifyForMarketplace")]
    pub certify_for_marketplace: Option<CertifyForMarketplace>,
    /// A description provided for the model approval.
    #[serde(rename = "ApprovalDescription")]
    pub approval_description: Option<ApprovalDescription>,
    /// A structure that contains model metrics reports.
    #[serde(rename = "ModelMetrics")]
    pub model_metrics: Option<ModelMetrics>,
    /// The description of the model package.
    #[serde(rename = "ModelPackageDescription")]
    pub model_package_description: Option<ModelPackageDescription>,
    /// Specifies configurations for one or more transform jobs that Amazon SageMaker runs to test the model package.
    #[serde(rename = "ValidationSpecification")]
    pub validation_specification: Option<ValidationSpecification>,
    /// The name or arn of the model package.
    #[serde(rename = "ModelPackageName")]
    pub model_package_name: Option<ModelPackageName>,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalInferenceSpecificationsToAdd")]
    pub additional_inference_specifications_to_add: Option<AdditionalInferenceSpecifications>,
    /// Represents the drift check baselines that can be used when the model monitor is set using the model package.
    #[serde(rename = "DriftCheckBaselines")]
    pub drift_check_baselines: Option<DriftCheckBaselines>,
    /// The time at which the model package was last modified.
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: Option<LastModifiedTime>,
    /// Sets the environment variables in the Docker container
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,

}


#[derive(serde::Serialize, Default)]
pub struct CustomerMetadataProperties {

}

#[derive(serde::Serialize, Default)]
pub struct Bias {
    #[serde(rename = "PreTrainingReport")]
    pub pre_training_report: Option<MetricsSource>,
    #[serde(rename = "Report")]
    pub report: Option<MetricsSource>,
    #[serde(rename = "PostTrainingReport")]
    pub post_training_report: Option<MetricsSource>,

}
pub type ModelPackageDescription = String;
#[derive(serde::Serialize, Default)]
pub struct SourceAlgorithm {
    #[serde(rename = "AlgorithmName")]
    pub algorithm_name: String,
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,

}
pub type ResponseMIMEType = String;
#[derive(serde::Serialize, Default)]
pub struct FileSource {
    #[serde(rename = "ContentDigest")]
    pub content_digest: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct TransformInput {
    #[serde(rename = "SplitType")]
    pub split_type: Option<String>,
    #[serde(rename = "DataSource")]
    pub data_source: DataSource,
    #[serde(rename = "CompressionType")]
    pub compression_type: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TransformResources {
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ModelQuality {
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}
pub type ModelPackageName = String;
#[derive(serde::Serialize, Default)]
pub struct LastModifiedBy {

}
pub type LastModifiedTime = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Environment {

}
pub type ModelApprovalStatus = String;
#[derive(serde::Serialize, Default)]
pub struct ValidationSpecification {
    #[serde(rename = "ValidationProfiles")]
    pub validation_profiles: Vec<ValidationProfile>,
    #[serde(rename = "ValidationRole")]
    pub validation_role: String,

}

#[derive(serde::Serialize, Default)]
pub struct DriftCheckBias {
    #[serde(rename = "PreTrainingConstraints")]
    pub pre_training_constraints: Option<MetricsSource>,
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<FileSource>,
    #[serde(rename = "PostTrainingConstraints")]
    pub post_training_constraints: Option<MetricsSource>,

}

#[derive(serde::Serialize, Default)]
pub struct DriftCheckModelQuality {
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelPackageContainerDefinition {
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: Option<String>,
    #[serde(rename = "ModelInput")]
    pub model_input: Option<()>,
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ContainerHostname")]
    pub container_hostname: Option<String>,
    #[serde(rename = "ModelDataUrl")]
    pub model_data_url: Option<String>,
    #[serde(rename = "ImageDigest")]
    pub image_digest: Option<String>,
    #[serde(rename = "Framework")]
    pub framework: Option<String>,
    #[serde(rename = "NearestModelName")]
    pub nearest_model_name: Option<String>,

}
pub type Domain = String;
#[derive(serde::Serialize, Default)]
pub struct ModelMetrics {
    #[serde(rename = "ModelQuality")]
    pub model_quality: Option<ModelQuality>,
    #[serde(rename = "Bias")]
    pub bias: Option<Bias>,
    #[serde(rename = "ModelDataQuality")]
    pub model_data_quality: Option<ModelDataQuality>,
    #[serde(rename = "Explainability")]
    pub explainability: Option<Explainability>,

}

#[derive(serde::Serialize, Default)]
pub struct S3DataSource {
    #[serde(rename = "S3DataType")]
    pub s3_data_type: String,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct Explainability {
    #[serde(rename = "Report")]
    pub report: Option<MetricsSource>,

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalInferenceSpecifications {

}

#[derive(serde::Serialize, Default)]
pub struct DriftCheckBaselines {
    #[serde(rename = "Bias")]
    pub bias: Option<DriftCheckBias>,
    #[serde(rename = "ModelDataQuality")]
    pub model_data_quality: Option<DriftCheckModelDataQuality>,
    #[serde(rename = "ModelQuality")]
    pub model_quality: Option<DriftCheckModelQuality>,
    #[serde(rename = "Explainability")]
    pub explainability: Option<DriftCheckExplainability>,

}
pub type InferenceInstanceType = String;pub type SamplePayloadUrl = String;
#[derive(serde::Serialize, Default)]
pub struct TransformJobDefinition {
    #[serde(rename = "Environment")]
    pub environment: Option<Environment>,
    #[serde(rename = "BatchStrategy")]
    pub batch_strategy: Option<String>,
    #[serde(rename = "TransformInput")]
    pub transform_input: TransformInput,
    #[serde(rename = "TransformResources")]
    pub transform_resources: TransformResources,
    #[serde(rename = "MaxConcurrentTransforms")]
    pub max_concurrent_transforms: Option<usize>,
    #[serde(rename = "MaxPayloadInMB")]
    pub max_payload_in_mb: Option<usize>,
    #[serde(rename = "TransformOutput")]
    pub transform_output: TransformOutput,

}
pub type CertifyForMarketplace = bool;pub type ModelPackageGroupName = String;pub type ModelPackageArn = String;
#[derive(serde::Serialize, Default)]
pub struct ValidationProfile {
    #[serde(rename = "ProfileName")]
    pub profile_name: String,
    #[serde(rename = "TransformJobDefinition")]
    pub transform_job_definition: TransformJobDefinition,

}
pub type ApprovalDescription = String;pub type TransformInstanceType = String;pub type ClientToken = String;
#[derive(serde::Serialize, Default)]
pub struct SourceAlgorithmSpecification {
    #[serde(rename = "SourceAlgorithms")]
    pub source_algorithms: Vec<SourceAlgorithm>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelPackageStatusDetails {
    #[serde(rename = "ImageScanStatuses")]
    pub image_scan_statuses: Option<Vec<ModelPackageStatusItem>>,
    #[serde(rename = "ValidationStatuses")]
    pub validation_statuses: Vec<ModelPackageStatusItem>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelPackageStatusItem {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ModelDataQuality {
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}

#[derive(serde::Serialize, Default)]
pub struct DriftCheckModelDataQuality {
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,
    #[serde(rename = "Statistics")]
    pub statistics: Option<MetricsSource>,

}
pub type Task = String;
#[derive(serde::Serialize, Default)]
pub struct MetricsSource {
    #[serde(rename = "ContentDigest")]
    pub content_digest: Option<String>,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    #[serde(rename = "ContentType")]
    pub content_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataSource {
    #[serde(rename = "S3DataSource")]
    pub s3_data_source: S3DataSource,

}
pub type CreationTime = String;
#[derive(serde::Serialize, Default)]
pub struct InferenceSpecification {
    #[serde(rename = "Containers")]
    pub containers: Vec<ModelPackageContainerDefinition>,
    #[serde(rename = "SupportedRealtimeInferenceInstanceTypes")]
    pub supported_realtime_inference_instance_types: Option<Vec<InferenceInstanceType>>,
    #[serde(rename = "SupportedResponseMIMETypes")]
    pub supported_response_mimetypes: Vec<ResponseMIMEType>,
    #[serde(rename = "SupportedTransformInstanceTypes")]
    pub supported_transform_instance_types: Option<Vec<TransformInstanceType>>,
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Vec<ContentType>,

}

#[derive(serde::Serialize, Default)]
pub struct DriftCheckExplainability {
    #[serde(rename = "ConfigFile")]
    pub config_file: Option<FileSource>,
    #[serde(rename = "Constraints")]
    pub constraints: Option<MetricsSource>,

}
pub type ModelPackageStatus = String;
#[derive(serde::Serialize, Default)]
pub struct TransformOutput {
    #[serde(rename = "AssembleWith")]
    pub assemble_with: Option<String>,
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: String,
    #[serde(rename = "Accept")]
    pub accept: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CreatedBy {

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalInferenceSpecificationDefinition {
    #[serde(rename = "SupportedContentTypes")]
    pub supported_content_types: Option<Vec<ContentType>>,
    #[serde(rename = "SupportedRealtimeInferenceInstanceTypes")]
    pub supported_realtime_inference_instance_types: Option<Vec<InferenceInstanceType>>,
    #[serde(rename = "SupportedTransformInstanceTypes")]
    pub supported_transform_instance_types: Option<Vec<TransformInstanceType>>,
    #[serde(rename = "SupportedResponseMIMETypes")]
    pub supported_response_mimetypes: Option<Vec<ResponseMIMEType>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Vec<ModelPackageContainerDefinition>,

}

#[derive(serde::Serialize, Default)]
pub struct UserContext {
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: Option<String>,
    #[serde(rename = "UserProfileArn")]
    pub user_profile_arn: Option<String>,
    #[serde(rename = "DomainId")]
    pub domain_id: Option<String>,

}
pub type ModelPackageVersion = usize;pub type ContentType = String;
#[derive(serde::Serialize, Default)]
pub struct MetadataProperties {
    #[serde(rename = "Repository")]
    pub repository: Option<String>,
    #[serde(rename = "ProjectId")]
    pub project_id: Option<String>,
    #[serde(rename = "CommitId")]
    pub commit_id: Option<String>,
    #[serde(rename = "GeneratedBy")]
    pub generated_by: Option<String>,

}


}

pub mod cfn_model_package_group {

#[derive(serde::Serialize, Default)]
pub struct CfnModelPackageGroup {
    /// The description of the model package group.
    #[serde(rename = "ModelPackageGroupDescription")]
    pub model_package_group_description: Option<ModelPackageGroupDescription>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelPackageGroupPolicy")]
    pub model_package_group_policy: Option<()>,
    /// The name of the model package group.
    #[serde(rename = "ModelPackageGroupName")]
    pub model_package_group_name: ModelPackageGroupName,

}

pub type ModelPackageGroupName = String;pub type ModelPackageGroupDescription = String;pub type ModelPackageGroupArn = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_model_quality_job_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnModelQualityJobDefinition {
    /// The name of the job definition.
    #[serde(rename = "JobDefinitionName")]
    pub job_definition_name: Option<JobDefinitionName>,
    /// Baseline configuration used to validate that the data conforms to the specified constraints and statistics.
    #[serde(rename = "ModelQualityBaselineConfig")]
    pub model_quality_baseline_config: Option<ModelQualityBaselineConfig>,
    /// Container image configuration object for the monitoring job.
    #[serde(rename = "ModelQualityAppSpecification")]
    pub model_quality_app_specification: ModelQualityAppSpecification,
    /// The name of the endpoint used to run the monitoring job.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies a time limit for how long the monitoring job is allowed to run.
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelQualityJobOutputConfig")]
    pub model_quality_job_output_config: MonitoringOutputConfig,
    /// The inputs for a monitoring job.
    #[serde(rename = "ModelQualityJobInput")]
    pub model_quality_job_input: ModelQualityJobInput,
    /// Networking options for a job, such as network traffic encryption between containers, whether to allow inbound and outbound network calls to and from containers, and the VPC subnets and security groups to use for VPC-enabled jobs.
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct StoppingCondition {
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: usize,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringResources {
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}

#[derive(serde::Serialize, Default)]
pub struct ModelQualityJobInput {
    #[serde(rename = "GroundTruthS3Input")]
    pub ground_truth_s3_input: MonitoringGroundTruthS3Input,
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,

}

#[derive(serde::Serialize, Default)]
pub struct Json {
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}
pub type EndpointName = String;
#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}
pub type Parquet = bool;pub type S3Uri = String;pub type MonitoringTimeOffsetString = String;
#[derive(serde::Serialize, Default)]
pub struct MonitoringOutputConfig {
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelQualityBaselineConfig {
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,
    #[serde(rename = "BaseliningJobName")]
    pub baselining_job_name: Option<ProcessingJobName>,

}
pub type ProcessingJobName = String;
#[derive(serde::Serialize, Default)]
pub struct MonitoringGroundTruthS3Input {
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Output {
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ModelQualityAppSpecification {
    #[serde(rename = "ContainerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,
    #[serde(rename = "ContainerArguments")]
    pub container_arguments: Option<Vec<String>>,
    #[serde(rename = "ImageUri")]
    pub image_uri: String,
    #[serde(rename = "RecordPreprocessorSourceUri")]
    pub record_preprocessor_source_uri: Option<S3Uri>,
    #[serde(rename = "ProblemType")]
    pub problem_type: ProblemType,
    #[serde(rename = "Environment")]
    pub environment: Option<()>,
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    pub post_analytics_processor_source_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchTransformInput {
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,

}
pub type JobDefinitionName = String;pub type ProblemType = String;
#[derive(serde::Serialize, Default)]
pub struct NetworkConfig {
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointInput {
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "EndTimeOffset")]
    pub end_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "ProbabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "StartTimeOffset")]
    pub start_time_offset: Option<MonitoringTimeOffsetString>,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "InferenceAttribute")]
    pub inference_attribute: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutput {
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConstraintsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetFormat {
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,
    #[serde(rename = "Json")]
    pub json: Option<Json>,
    #[serde(rename = "Parquet")]
    pub parquet: Option<Parquet>,

}


}

pub mod cfn_monitoring_schedule {

#[derive(serde::Serialize, Default)]
pub struct CfnMonitoringSchedule {
    /// The configuration object that specifies the monitoring schedule and defines the monitoring job.
    #[serde(rename = "MonitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleConfig,
    /// The name of the endpoint used to run the monitoring job.
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,
    /// Contains the reason a monitoring job failed, if it failed.
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<String>,
    /// The status of a schedule job.
    #[serde(rename = "MonitoringScheduleStatus")]
    pub monitoring_schedule_status: Option<String>,
    /// The name of the monitoring schedule.
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: MonitoringScheduleName,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Describes metadata on the last execution to run, if there was one.
    #[serde(rename = "LastMonitoringExecutionSummary")]
    pub last_monitoring_execution_summary: Option<MonitoringExecutionSummary>,

}


#[derive(serde::Serialize, Default)]
pub struct ConstraintsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct ScheduleConfig {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringAppSpecification {
    #[serde(rename = "ContainerArguments")]
    pub container_arguments: Option<Vec<ContainerArgument>>,
    #[serde(rename = "ContainerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    pub post_analytics_processor_source_uri: Option<S3Uri>,
    #[serde(rename = "RecordPreprocessorSourceUri")]
    pub record_preprocessor_source_uri: Option<S3Uri>,
    #[serde(rename = "ImageUri")]
    pub image_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringResources {
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringExecutionSummary {
    #[serde(rename = "MonitoringExecutionStatus")]
    pub monitoring_execution_status: String,
    #[serde(rename = "ProcessingJobArn")]
    pub processing_job_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
    #[serde(rename = "ScheduledTime")]
    pub scheduled_time: String,
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: String,
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<EndpointName>,
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: MonitoringScheduleName,

}
pub type Parquet = bool;pub type S3Uri = String;
#[derive(serde::Serialize, Default)]
pub struct StoppingCondition {
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: usize,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringJobDefinition {
    #[serde(rename = "NetworkConfig")]
    pub network_config: Option<NetworkConfig>,
    #[serde(rename = "BaselineConfig")]
    pub baseline_config: Option<BaselineConfig>,
    #[serde(rename = "MonitoringInputs")]
    pub monitoring_inputs: MonitoringInputs,
    #[serde(rename = "MonitoringAppSpecification")]
    pub monitoring_app_specification: MonitoringAppSpecification,
    #[serde(rename = "MonitoringOutputConfig")]
    pub monitoring_output_config: MonitoringOutputConfig,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "StoppingCondition")]
    pub stopping_condition: Option<StoppingCondition>,
    #[serde(rename = "Environment")]
    pub environment: Option<()>,
    #[serde(rename = "MonitoringResources")]
    pub monitoring_resources: MonitoringResources,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointInput {
    #[serde(rename = "EndpointName")]
    pub endpoint_name: EndpointName,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Json {
    #[serde(rename = "Line")]
    pub line: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringInput {
    #[serde(rename = "EndpointInput")]
    pub endpoint_input: Option<EndpointInput>,
    #[serde(rename = "BatchTransformInput")]
    pub batch_transform_input: Option<BatchTransformInput>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfig {
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(rename = "EnableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetFormat {
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,
    #[serde(rename = "Json")]
    pub json: Option<Json>,
    #[serde(rename = "Parquet")]
    pub parquet: Option<Parquet>,

}

#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Header")]
    pub header: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Output {
    #[serde(rename = "S3UploadMode")]
    pub s3_upload_mode: Option<String>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutput {
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,

}

#[derive(serde::Serialize, Default)]
pub struct BaselineConfig {
    #[serde(rename = "StatisticsResource")]
    pub statistics_resource: Option<StatisticsResource>,
    #[serde(rename = "ConstraintsResource")]
    pub constraints_resource: Option<ConstraintsResource>,

}

#[derive(serde::Serialize, Default)]
pub struct StatisticsResource {
    #[serde(rename = "S3Uri")]
    pub s3_uri: Option<S3Uri>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchTransformInput {
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,
    #[serde(rename = "S3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(rename = "S3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(rename = "LocalPath")]
    pub local_path: String,

}
pub type ContainerArgument = String;
#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringOutputConfig {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,

}

#[derive(serde::Serialize, Default)]
pub struct ClusterConfig {
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "InstanceCount")]
    pub instance_count: usize,
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringInputs {

}

#[derive(serde::Serialize, Default)]
pub struct MonitoringScheduleConfig {
    #[serde(rename = "MonitoringJobDefinitionName")]
    pub monitoring_job_definition_name: Option<String>,
    #[serde(rename = "ScheduleConfig")]
    pub schedule_config: Option<ScheduleConfig>,
    #[serde(rename = "MonitoringJobDefinition")]
    pub monitoring_job_definition: Option<MonitoringJobDefinition>,
    #[serde(rename = "MonitoringType")]
    pub monitoring_type: Option<MonitoringType>,

}
pub type EndpointName = String;pub type MonitoringType = String;pub type MonitoringScheduleName = String;

}

pub mod cfn_notebook_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnNotebookInstance {
    /// No documentation provided by AWS
    #[serde(rename = "LifecycleConfigName")]
    pub lifecycle_config_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DirectInternetAccess")]
    pub direct_internet_access: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PlatformIdentifier")]
    pub platform_identifier: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "RootAccess")]
    pub root_access: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceMetadataServiceConfiguration")]
    pub instance_metadata_service_configuration: Option<InstanceMetadataServiceConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalCodeRepositories")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultCodeRepository")]
    pub default_code_repository: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct InstanceMetadataServiceConfiguration {
    #[serde(rename = "MinimumInstanceMetadataServiceVersion")]
    pub minimum_instance_metadata_service_version: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_notebook_instance_lifecycle_config {

#[derive(serde::Serialize, Default)]
pub struct CfnNotebookInstanceLifecycleConfig {
    /// List of NotebookInstanceLifecycleHook
    #[serde(rename = "OnStart")]
    pub on_start: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// List of NotebookInstanceLifecycleHook
    #[serde(rename = "OnCreate")]
    pub on_create: Option<Vec<NotebookInstanceLifecycleHook>>,
    /// No documentation provided by AWS
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    pub notebook_instance_lifecycle_config_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NotebookInstanceLifecycleHook {
    #[serde(rename = "Content")]
    pub content: Option<String>,

}


}

pub mod cfn_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnPipeline {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Role Arn
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ParallelismConfiguration")]
    pub parallelism_configuration: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "PipelineDefinition")]
    pub pipeline_definition: (),
    /// The display name of the Pipeline.
    #[serde(rename = "PipelineDisplayName")]
    pub pipeline_display_name: Option<String>,
    /// The description of the Pipeline.
    #[serde(rename = "PipelineDescription")]
    pub pipeline_description: Option<String>,
    /// The name of the Pipeline.
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,

}


#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "ETag")]
    pub etag: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// Input ServiceCatalog Provisioning Details
    #[serde(rename = "ServiceCatalogProvisioningDetails")]
    pub service_catalog_provisioning_details: (),
    /// The name of the project.
    #[serde(rename = "ProjectName")]
    pub project_name: ProjectName,
    /// The description of the project.
    #[serde(rename = "ProjectDescription")]
    pub project_description: Option<ProjectDescription>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type ProvisionedProductStatusMessage = String;pub type ProjectName = String;pub type ProjectId = String;pub type ProjectArn = String;pub type PathId = String;
#[derive(serde::Serialize, Default)]
pub struct ProvisioningParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type ProjectDescription = String;pub type ProductId = String;pub type ProvisioningArtifactId = String;

}

pub mod cfn_space {

#[derive(serde::Serialize, Default)]
pub struct CfnSpace {
    /// A collection of settings.
    #[serde(rename = "SpaceSettings")]
    pub space_settings: Option<SpaceSettings>,
    /// The ID of the associated Domain.
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// A name for the Space.
    #[serde(rename = "SpaceName")]
    pub space_name: String,
    /// A list of tags to apply to the space.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct CustomImage {
    #[serde(rename = "ImageVersionNumber")]
    pub image_version_number: Option<usize>,
    #[serde(rename = "ImageName")]
    pub image_name: String,
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct KernelGatewayAppSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct SpaceSettings {
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct JupyterServerAppSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceSpec {
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,

}


}

pub mod cfn_user_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnUserProfile {
    /// A specifier for the type of value specified in SingleSignOnUserValue. Currently, the only supported value is "UserName". If the Domain's AuthMode is SSO, this field is required. If the Domain's AuthMode is not SSO, this field cannot be specified.
    #[serde(rename = "SingleSignOnUserIdentifier")]
    pub single_sign_on_user_identifier: Option<String>,
    /// The username of the associated AWS Single Sign-On User for this UserProfile. If the Domain's AuthMode is SSO, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not SSO, this field cannot be specified.
    #[serde(rename = "SingleSignOnUserValue")]
    pub single_sign_on_user_value: Option<String>,
    /// The ID of the associated Domain.
    #[serde(rename = "DomainId")]
    pub domain_id: String,
    /// A name for the UserProfile.
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,
    /// A collection of settings.
    #[serde(rename = "UserSettings")]
    pub user_settings: Option<UserSettings>,
    /// A list of tags to apply to the user profile.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct UserSettings {
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SharingSettings")]
    pub sharing_settings: Option<SharingSettings>,
    #[serde(rename = "ExecutionRole")]
    pub execution_role: Option<String>,
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,
    #[serde(rename = "RStudioServerProAppSettings")]
    pub rstudio_server_pro_app_settings: Option<RStudioServerProAppSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceSpec {
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct KernelGatewayAppSettings {
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}

#[derive(serde::Serialize, Default)]
pub struct JupyterServerAppSettings {
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomImage {
    #[serde(rename = "ImageName")]
    pub image_name: String,
    #[serde(rename = "ImageVersionNumber")]
    pub image_version_number: Option<usize>,
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SharingSettings {
    #[serde(rename = "NotebookOutputOption")]
    pub notebook_output_option: Option<String>,
    #[serde(rename = "S3KmsKeyId")]
    pub s3_kms_key_id: Option<String>,
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RStudioServerProAppSettings {
    #[serde(rename = "AccessStatus")]
    pub access_status: Option<String>,
    #[serde(rename = "UserGroup")]
    pub user_group: Option<String>,

}


}

pub mod cfn_workteam {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkteam {
    /// No documentation provided by AWS
    #[serde(rename = "WorkforceName")]
    pub workforce_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkteamName")]
    pub workteam_name: Option<String>,
    /// List of MemberDefinition
    #[serde(rename = "MemberDefinitions")]
    pub member_definitions: Option<Vec<MemberDefinition>>,

}


#[derive(serde::Serialize, Default)]
pub struct MemberDefinition {
    #[serde(rename = "OidcMemberDefinition")]
    pub oidc_member_definition: Option<OidcMemberDefinition>,
    #[serde(rename = "CognitoMemberDefinition")]
    pub cognito_member_definition: Option<CognitoMemberDefinition>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct OidcMemberDefinition {
    #[serde(rename = "OidcGroups")]
    pub oidc_groups: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CognitoMemberDefinition {
    #[serde(rename = "CognitoUserPool")]
    pub cognito_user_pool: String,
    #[serde(rename = "CognitoClientId")]
    pub cognito_client_id: String,
    #[serde(rename = "CognitoUserGroup")]
    pub cognito_user_group: String,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationConfiguration {
    #[serde(rename = "NotificationTopicArn")]
    pub notification_topic_arn: String,

}


}
