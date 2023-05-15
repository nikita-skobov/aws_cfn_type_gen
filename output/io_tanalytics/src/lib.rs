
pub mod cfn_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnChannel {
    /// No documentation provided by AWS
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<RetentionPeriod>,
    /// No documentation provided by AWS
    #[serde(rename = "ChannelStorage")]
    pub channel_storage: Option<ChannelStorage>,

}


#[derive(serde::Serialize, Default)]
pub struct RetentionPeriod {
    #[serde(rename = "NumberOfDays")]
    pub number_of_days: Option<usize>,
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ChannelStorage {
    #[serde(rename = "CustomerManagedS3")]
    pub customer_managed_s3: Option<CustomerManagedS3>,
    #[serde(rename = "ServiceManagedS3")]
    pub service_managed_s3: Option<ServiceManagedS3>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomerManagedS3 {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "KeyPrefix")]
    pub key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceManagedS3 {

}


}

pub mod cfn_dataset {

#[derive(serde::Serialize, Default)]
pub struct CfnDataset {
    /// No documentation provided by AWS
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,
    /// List of Trigger
    #[serde(rename = "Triggers")]
    pub triggers: Option<Vec<Trigger>>,
    /// List of LateDataRule
    #[serde(rename = "LateDataRules")]
    pub late_data_rules: Option<Vec<LateDataRule>>,
    /// No documentation provided by AWS
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<RetentionPeriod>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: Option<VersioningConfiguration>,
    /// List of Action
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    /// List of DatasetContentDeliveryRule
    #[serde(rename = "ContentDeliveryRules")]
    pub content_delivery_rules: Option<Vec<DatasetContentDeliveryRule>>,

}


#[derive(serde::Serialize, Default)]
pub struct Variable {
    #[serde(rename = "DatasetContentVersionValue")]
    pub dataset_content_version_value: Option<DatasetContentVersionValue>,
    #[serde(rename = "VariableName")]
    pub variable_name: String,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<f64>,
    #[serde(rename = "OutputFileUriValue")]
    pub output_file_uri_value: Option<OutputFileUriValue>,

}

#[derive(serde::Serialize, Default)]
pub struct OutputFileUriValue {
    #[serde(rename = "FileName")]
    pub file_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct QueryAction {
    #[serde(rename = "SqlQuery")]
    pub sql_query: String,
    #[serde(rename = "Filters")]
    pub filters: Option<Vec<Filter>>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "ActionName")]
    pub action_name: String,
    #[serde(rename = "ContainerAction")]
    pub container_action: Option<ContainerAction>,
    #[serde(rename = "QueryAction")]
    pub query_action: Option<QueryAction>,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetContentDeliveryRuleDestination {
    #[serde(rename = "S3DestinationConfiguration")]
    pub s3_destination_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "IotEventsDestinationConfiguration")]
    pub iot_events_destination_configuration: Option<IotEventsDestinationConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct S3DestinationConfiguration {
    #[serde(rename = "GlueConfiguration")]
    pub glue_configuration: Option<GlueConfiguration>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct LateDataRuleConfiguration {
    #[serde(rename = "DeltaTimeSessionWindowConfiguration")]
    pub delta_time_session_window_configuration: Option<DeltaTimeSessionWindowConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct VersioningConfiguration {
    #[serde(rename = "MaxVersions")]
    pub max_versions: Option<usize>,
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "DeltaTime")]
    pub delta_time: Option<DeltaTime>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceConfiguration {
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: usize,
    #[serde(rename = "ComputeType")]
    pub compute_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct GlueConfiguration {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetContentVersionValue {
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct LateDataRule {
    #[serde(rename = "RuleConfiguration")]
    pub rule_configuration: LateDataRuleConfiguration,
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DeltaTime {
    #[serde(rename = "TimeExpression")]
    pub time_expression: String,
    #[serde(rename = "OffsetSeconds")]
    pub offset_seconds: usize,

}

#[derive(serde::Serialize, Default)]
pub struct IotEventsDestinationConfiguration {
    #[serde(rename = "InputName")]
    pub input_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DatasetContentDeliveryRule {
    #[serde(rename = "EntryName")]
    pub entry_name: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: DatasetContentDeliveryRuleDestination,

}

#[derive(serde::Serialize, Default)]
pub struct ContainerAction {
    #[serde(rename = "ResourceConfiguration")]
    pub resource_configuration: ResourceConfiguration,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Variables")]
    pub variables: Option<Vec<Variable>>,
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Schedule {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct TriggeringDataset {
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeltaTimeSessionWindowConfiguration {
    #[serde(rename = "TimeoutInMinutes")]
    pub timeout_in_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Trigger {
    #[serde(rename = "TriggeringDataset")]
    pub triggering_dataset: Option<TriggeringDataset>,
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct RetentionPeriod {
    #[serde(rename = "NumberOfDays")]
    pub number_of_days: Option<usize>,
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,

}


}

pub mod cfn_datastore {

#[derive(serde::Serialize, Default)]
pub struct CfnDatastore {
    /// No documentation provided by AWS
    #[serde(rename = "DatastoreStorage")]
    pub datastore_storage: Option<DatastoreStorage>,
    /// No documentation provided by AWS
    #[serde(rename = "DatastoreName")]
    pub datastore_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "FileFormatConfiguration")]
    pub file_format_configuration: Option<FileFormatConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "DatastorePartitions")]
    pub datastore_partitions: Option<DatastorePartitions>,
    /// No documentation provided by AWS
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<RetentionPeriod>,

}


#[derive(serde::Serialize, Default)]
pub struct DatastorePartitions {
    #[serde(rename = "Partitions")]
    pub partitions: Option<Vec<DatastorePartition>>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaDefinition {
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<Column>>,

}

#[derive(serde::Serialize, Default)]
pub struct ParquetConfiguration {
    #[serde(rename = "SchemaDefinition")]
    pub schema_definition: Option<SchemaDefinition>,

}

#[derive(serde::Serialize, Default)]
pub struct TimestampPartition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "TimestampFormat")]
    pub timestamp_format: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomerManagedS3 {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "KeyPrefix")]
    pub key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ServiceManagedS3 {

}

#[derive(serde::Serialize, Default)]
pub struct FileFormatConfiguration {
    #[serde(rename = "JsonConfiguration")]
    pub json_configuration: Option<JsonConfiguration>,
    #[serde(rename = "ParquetConfiguration")]
    pub parquet_configuration: Option<ParquetConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct DatastoreStorage {
    #[serde(rename = "IotSiteWiseMultiLayerStorage")]
    pub iot_site_wise_multi_layer_storage: Option<IotSiteWiseMultiLayerStorage>,
    #[serde(rename = "ServiceManagedS3")]
    pub service_managed_s3: Option<ServiceManagedS3>,
    #[serde(rename = "CustomerManagedS3")]
    pub customer_managed_s3: Option<CustomerManagedS3>,

}

#[derive(serde::Serialize, Default)]
pub struct RetentionPeriod {
    #[serde(rename = "Unlimited")]
    pub unlimited: Option<bool>,
    #[serde(rename = "NumberOfDays")]
    pub number_of_days: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Partition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Column {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct JsonConfiguration {

}

#[derive(serde::Serialize, Default)]
pub struct IotSiteWiseMultiLayerStorage {
    #[serde(rename = "CustomerManagedS3Storage")]
    pub customer_managed_s3_storage: Option<CustomerManagedS3Storage>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomerManagedS3Storage {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "KeyPrefix")]
    pub key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct DatastorePartition {
    #[serde(rename = "Partition")]
    pub partition: Option<Partition>,
    #[serde(rename = "TimestampPartition")]
    pub timestamp_partition: Option<TimestampPartition>,

}


}

pub mod cfn_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnPipeline {
    /// No documentation provided by AWS
    #[serde(rename = "PipelineName")]
    pub pipeline_name: Option<String>,
    /// List of Activity
    #[serde(rename = "PipelineActivities")]
    pub pipeline_activities: Vec<Activity>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Datastore {
    #[serde(rename = "DatastoreName")]
    pub datastore_name: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeviceRegistryEnrich {
    #[serde(rename = "ThingName")]
    pub thing_name: String,
    #[serde(rename = "Attribute")]
    pub attribute: String,
    #[serde(rename = "Next")]
    pub next: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Channel {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    #[serde(rename = "Next")]
    pub next: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Lambda {
    #[serde(rename = "BatchSize")]
    pub batch_size: usize,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Next")]
    pub next: Option<String>,
    #[serde(rename = "LambdaName")]
    pub lambda_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Activity {
    #[serde(rename = "DeviceRegistryEnrich")]
    pub device_registry_enrich: Option<DeviceRegistryEnrich>,
    #[serde(rename = "Math")]
    pub math: Option<Math>,
    #[serde(rename = "Channel")]
    pub channel: Option<Channel>,
    #[serde(rename = "Filter")]
    pub filter: Option<Filter>,
    #[serde(rename = "SelectAttributes")]
    pub select_attributes: Option<SelectAttributes>,
    #[serde(rename = "RemoveAttributes")]
    pub remove_attributes: Option<RemoveAttributes>,
    #[serde(rename = "Datastore")]
    pub datastore: Option<Datastore>,
    #[serde(rename = "AddAttributes")]
    pub add_attributes: Option<AddAttributes>,
    #[serde(rename = "DeviceShadowEnrich")]
    pub device_shadow_enrich: Option<DeviceShadowEnrich>,
    #[serde(rename = "Lambda")]
    pub lambda: Option<Lambda>,

}

#[derive(serde::Serialize, Default)]
pub struct DeviceShadowEnrich {
    #[serde(rename = "Next")]
    pub next: Option<String>,
    #[serde(rename = "ThingName")]
    pub thing_name: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Attribute")]
    pub attribute: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Filter {
    #[serde(rename = "Next")]
    pub next: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Filter")]
    pub filter: String,

}

#[derive(serde::Serialize, Default)]
pub struct SelectAttributes {
    #[serde(rename = "Next")]
    pub next: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Attributes")]
    pub attributes: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Math {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Math")]
    pub math: String,
    #[serde(rename = "Attribute")]
    pub attribute: String,
    #[serde(rename = "Next")]
    pub next: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RemoveAttributes {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Attributes")]
    pub attributes: Vec<String>,
    #[serde(rename = "Next")]
    pub next: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AddAttributes {
    #[serde(rename = "Attributes")]
    pub attributes: (),
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Next")]
    pub next: Option<String>,

}


}
