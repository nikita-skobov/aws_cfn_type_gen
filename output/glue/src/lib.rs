
pub mod cfn_classifier {

#[derive(serde::Serialize, Default)]
pub struct CfnClassifier {
    /// No documentation provided by AWS
    #[serde(rename = "XMLClassifier")]
    pub xmlclassifier: Option<XMLClassifier>,
    /// No documentation provided by AWS
    #[serde(rename = "CsvClassifier")]
    pub csv_classifier: Option<CsvClassifier>,
    /// No documentation provided by AWS
    #[serde(rename = "JsonClassifier")]
    pub json_classifier: Option<JsonClassifier>,
    /// No documentation provided by AWS
    #[serde(rename = "GrokClassifier")]
    pub grok_classifier: Option<GrokClassifier>,

}


#[derive(serde::Serialize, Default)]
pub struct CsvClassifier {
    #[serde(rename = "DisableValueTrimming")]
    pub disable_value_trimming: Option<bool>,
    #[serde(rename = "Header")]
    pub header: Option<Vec<String>>,
    #[serde(rename = "AllowSingleColumn")]
    pub allow_single_column: Option<bool>,
    #[serde(rename = "QuoteSymbol")]
    pub quote_symbol: Option<String>,
    #[serde(rename = "ContainsHeader")]
    pub contains_header: Option<String>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct XMLClassifier {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "RowTag")]
    pub row_tag: String,
    #[serde(rename = "Classification")]
    pub classification: String,

}

#[derive(serde::Serialize, Default)]
pub struct JsonClassifier {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "JsonPath")]
    pub json_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct GrokClassifier {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Classification")]
    pub classification: String,
    #[serde(rename = "CustomPatterns")]
    pub custom_patterns: Option<String>,
    #[serde(rename = "GrokPattern")]
    pub grok_pattern: String,

}


}

pub mod cfn_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnConnection {
    /// No documentation provided by AWS
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,

}


#[derive(serde::Serialize, Default)]
pub struct PhysicalConnectionRequirements {
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "SecurityGroupIdList")]
    pub security_group_id_list: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionInput {
    #[serde(rename = "PhysicalConnectionRequirements")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
    #[serde(rename = "ConnectionProperties")]
    pub connection_properties: Option<()>,
    #[serde(rename = "MatchCriteria")]
    pub match_criteria: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


}

pub mod cfn_crawler {

#[derive(serde::Serialize, Default)]
pub struct CfnCrawler {
    /// No documentation provided by AWS
    #[serde(rename = "SchemaChangePolicy")]
    pub schema_change_policy: Option<SchemaChangePolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "RecrawlPolicy")]
    pub recrawl_policy: Option<RecrawlPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "CrawlerSecurityConfiguration")]
    pub crawler_security_configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Role")]
    pub role: String,
    /// No documentation provided by AWS
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Targets")]
    pub targets: Targets,
    /// No documentation provided by AWS
    #[serde(rename = "Classifiers")]
    pub classifiers: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: Option<Schedule>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TablePrefix")]
    pub table_prefix: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Targets {
    #[serde(rename = "DeltaTargets")]
    pub delta_targets: Option<Vec<DeltaTarget>>,
    #[serde(rename = "CatalogTargets")]
    pub catalog_targets: Option<Vec<CatalogTarget>>,
    #[serde(rename = "S3Targets")]
    pub s3_targets: Option<Vec<S3Target>>,
    #[serde(rename = "MongoDBTargets")]
    pub mongo_dbtargets: Option<Vec<MongoDBTarget>>,
    #[serde(rename = "JdbcTargets")]
    pub jdbc_targets: Option<Vec<JdbcTarget>>,
    #[serde(rename = "DynamoDBTargets")]
    pub dynamo_dbtargets: Option<Vec<DynamoDBTarget>>,

}

#[derive(serde::Serialize, Default)]
pub struct RecrawlPolicy {
    #[serde(rename = "RecrawlBehavior")]
    pub recrawl_behavior: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Target {
    #[serde(rename = "Exclusions")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "SampleSize")]
    pub sample_size: Option<usize>,
    #[serde(rename = "DlqEventQueueArn")]
    pub dlq_event_queue_arn: Option<String>,
    #[serde(rename = "EventQueueArn")]
    pub event_queue_arn: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Schedule {
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MongoDBTarget {
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CatalogTarget {
    #[serde(rename = "Tables")]
    pub tables: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBTarget {
    #[serde(rename = "Path")]
    pub path: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JdbcTarget {
    #[serde(rename = "Exclusions")]
    pub exclusions: Option<Vec<String>>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DeltaTarget {
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "CreateNativeDeltaTable")]
    pub create_native_delta_table: Option<bool>,
    #[serde(rename = "DeltaTables")]
    pub delta_tables: Option<Vec<String>>,
    #[serde(rename = "WriteManifest")]
    pub write_manifest: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaChangePolicy {
    #[serde(rename = "UpdateBehavior")]
    pub update_behavior: Option<String>,
    #[serde(rename = "DeleteBehavior")]
    pub delete_behavior: Option<String>,

}


}

pub mod cfn_database {

#[derive(serde::Serialize, Default)]
pub struct CfnDatabase {
    /// No documentation provided by AWS
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,
    /// No documentation provided by AWS
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct DatabaseInput {
    #[serde(rename = "CreateTableDefaultPermissions")]
    pub create_table_default_permissions: Option<Vec<PrincipalPrivileges>>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "TargetDatabase")]
    pub target_database: Option<DatabaseIdentifier>,
    #[serde(rename = "FederatedDatabase")]
    pub federated_database: Option<FederatedDatabase>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "LocationUri")]
    pub location_uri: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DatabaseIdentifier {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PrincipalPrivileges {
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    pub principal: Option<DataLakePrincipal>,

}

#[derive(serde::Serialize, Default)]
pub struct FederatedDatabase {
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<String>,

}


}

pub mod cfn_data_catalog_encryption_settings {

#[derive(serde::Serialize, Default)]
pub struct CfnDataCatalogEncryptionSettings {
    /// No documentation provided by AWS
    #[serde(rename = "DataCatalogEncryptionSettings")]
    pub data_catalog_encryption_settings: DataCatalogEncryptionSettings,
    /// No documentation provided by AWS
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct DataCatalogEncryptionSettings {
    #[serde(rename = "EncryptionAtRest")]
    pub encryption_at_rest: Option<EncryptionAtRest>,
    #[serde(rename = "ConnectionPasswordEncryption")]
    pub connection_password_encryption: Option<ConnectionPasswordEncryption>,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionAtRest {
    #[serde(rename = "CatalogEncryptionMode")]
    pub catalog_encryption_mode: Option<String>,
    #[serde(rename = "SseAwsKmsKeyId")]
    pub sse_aws_kms_key_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionPasswordEncryption {
    #[serde(rename = "ReturnConnectionPasswordEncrypted")]
    pub return_connection_password_encrypted: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


}

pub mod cfn_dev_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnDevEndpoint {
    /// No documentation provided by AWS
    #[serde(rename = "ExtraJarsS3Path")]
    pub extra_jars_s3_path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PublicKeys")]
    pub public_keys: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ExtraPythonLibsS3Path")]
    pub extra_python_libs_s3_path: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EndpointName")]
    pub endpoint_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PublicKey")]
    pub public_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Arguments")]
    pub arguments: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "NumberOfWorkers")]
    pub number_of_workers: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "NumberOfNodes")]
    pub number_of_nodes: Option<usize>,

}



}

pub mod cfn_job {

#[derive(serde::Serialize, Default)]
pub struct CfnJob {
    /// No documentation provided by AWS
    #[serde(rename = "Connections")]
    pub connections: Option<ConnectionsList>,
    /// No documentation provided by AWS
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "NumberOfWorkers")]
    pub number_of_workers: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionProperty")]
    pub execution_property: Option<ExecutionProperty>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllocatedCapacity")]
    pub allocated_capacity: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultArguments")]
    pub default_arguments: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionClass")]
    pub execution_class: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LogUri")]
    pub log_uri: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Role")]
    pub role: String,
    /// No documentation provided by AWS
    #[serde(rename = "Command")]
    pub command: JobCommand,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "NonOverridableArguments")]
    pub non_overridable_arguments: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationProperty")]
    pub notification_property: Option<NotificationProperty>,

}


#[derive(serde::Serialize, Default)]
pub struct ExecutionProperty {
    #[serde(rename = "MaxConcurrentRuns")]
    pub max_concurrent_runs: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct JobCommand {
    #[serde(rename = "ScriptLocation")]
    pub script_location: Option<String>,
    #[serde(rename = "PythonVersion")]
    pub python_version: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionsList {
    #[serde(rename = "Connections")]
    pub connections: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationProperty {
    #[serde(rename = "NotifyDelayAfter")]
    pub notify_delay_after: Option<usize>,

}


}

pub mod cfn_mltransform {

#[derive(serde::Serialize, Default)]
pub struct CfnMLTransform {
    /// No documentation provided by AWS
    #[serde(rename = "WorkerType")]
    pub worker_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InputRecordTables")]
    pub input_record_tables: InputRecordTables,
    /// No documentation provided by AWS
    #[serde(rename = "TransformParameters")]
    pub transform_parameters: TransformParameters,
    /// No documentation provided by AWS
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<f64>,
    /// No documentation provided by AWS
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TransformEncryption")]
    pub transform_encryption: Option<TransformEncryption>,
    /// No documentation provided by AWS
    #[serde(rename = "GlueVersion")]
    pub glue_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Role")]
    pub role: String,
    /// No documentation provided by AWS
    #[serde(rename = "NumberOfWorkers")]
    pub number_of_workers: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxRetries")]
    pub max_retries: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct FindMatchesParameters {
    #[serde(rename = "EnforceProvidedLabels")]
    pub enforce_provided_labels: Option<bool>,
    #[serde(rename = "AccuracyCostTradeoff")]
    pub accuracy_cost_tradeoff: Option<f64>,
    #[serde(rename = "PrecisionRecallTradeoff")]
    pub precision_recall_tradeoff: Option<f64>,
    #[serde(rename = "PrimaryKeyColumnName")]
    pub primary_key_column_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct GlueTables {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputRecordTables {
    #[serde(rename = "GlueTables")]
    pub glue_tables: Option<Vec<GlueTables>>,

}

#[derive(serde::Serialize, Default)]
pub struct TransformEncryption {
    #[serde(rename = "MLUserDataEncryption")]
    pub mluser_data_encryption: Option<MLUserDataEncryption>,
    #[serde(rename = "TaskRunSecurityConfigurationName")]
    pub task_run_security_configuration_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MLUserDataEncryption {
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MLUserDataEncryptionMode")]
    pub mluser_data_encryption_mode: String,

}

#[derive(serde::Serialize, Default)]
pub struct TransformParameters {
    #[serde(rename = "FindMatchesParameters")]
    pub find_matches_parameters: Option<FindMatchesParameters>,
    #[serde(rename = "TransformType")]
    pub transform_type: String,

}


}

pub mod cfn_partition {

#[derive(serde::Serialize, Default)]
pub struct CfnPartition {
    /// No documentation provided by AWS
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TableName")]
    pub table_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "PartitionInput")]
    pub partition_input: PartitionInput,

}


#[derive(serde::Serialize, Default)]
pub struct Order {
    #[serde(rename = "Column")]
    pub column: String,
    #[serde(rename = "SortOrder")]
    pub sort_order: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct StorageDescriptor {
    #[serde(rename = "SortColumns")]
    pub sort_columns: Option<Vec<Order>>,
    #[serde(rename = "SerdeInfo")]
    pub serde_info: Option<SerdeInfo>,
    #[serde(rename = "SchemaReference")]
    pub schema_reference: Option<SchemaReference>,
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,
    #[serde(rename = "SkewedInfo")]
    pub skewed_info: Option<SkewedInfo>,
    #[serde(rename = "OutputFormat")]
    pub output_format: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "InputFormat")]
    pub input_format: Option<String>,
    #[serde(rename = "Location")]
    pub location: Option<String>,
    #[serde(rename = "BucketColumns")]
    pub bucket_columns: Option<Vec<String>>,
    #[serde(rename = "NumberOfBuckets")]
    pub number_of_buckets: Option<usize>,
    #[serde(rename = "StoredAsSubDirectories")]
    pub stored_as_sub_directories: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaReference {
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: Option<String>,
    #[serde(rename = "SchemaVersionNumber")]
    pub schema_version_number: Option<usize>,
    #[serde(rename = "SchemaId")]
    pub schema_id: Option<SchemaId>,

}

#[derive(serde::Serialize, Default)]
pub struct SerdeInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "SerializationLibrary")]
    pub serialization_library: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SkewedInfo {
    #[serde(rename = "SkewedColumnValues")]
    pub skewed_column_values: Option<Vec<String>>,
    #[serde(rename = "SkewedColumnNames")]
    pub skewed_column_names: Option<Vec<String>>,
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    pub skewed_column_value_location_maps: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Column {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaId {
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PartitionInput {
    #[serde(rename = "StorageDescriptor")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}


}

pub mod cfn_registry {

#[derive(serde::Serialize, Default)]
pub struct CfnRegistry {
    /// List of tags to tag the Registry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Name of the registry to be created of max length of 255, and may only contain letters, numbers, hyphen, underscore, dollar sign, or hash mark.  No whitespace.
    #[serde(rename = "Name")]
    pub name: String,
    /// A description of the registry. If description is not provided, there will not be any default value for this.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_schema {

#[derive(serde::Serialize, Default)]
pub struct CfnSchema {
    /// Definition for the initial schema version in plain-text.
    #[serde(rename = "SchemaDefinition")]
    pub schema_definition: String,
    /// No documentation provided by AWS
    #[serde(rename = "CheckpointVersion")]
    pub checkpoint_version: Option<SchemaVersion>,
    /// Name of the schema.
    #[serde(rename = "Name")]
    pub name: String,
    /// List of tags to tag the schema
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// A description of the schema. If description is not provided, there will not be any default value for this.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Identifier for the registry which the schema is part of.
    #[serde(rename = "Registry")]
    pub registry: Option<Registry>,
    /// Data format name to use for the schema. Accepted values: 'AVRO', 'JSON', 'PROTOBUF'
    #[serde(rename = "DataFormat")]
    pub data_format: String,
    /// Compatibility setting for the schema.
    #[serde(rename = "Compatibility")]
    pub compatibility: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Registry {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaVersion {
    #[serde(rename = "IsLatest")]
    pub is_latest: Option<bool>,
    #[serde(rename = "VersionNumber")]
    pub version_number: Option<usize>,

}


}

pub mod cfn_schema_version {

#[derive(serde::Serialize, Default)]
pub struct CfnSchemaVersion {
    /// Identifier for the schema where the schema version will be created.
    #[serde(rename = "Schema")]
    pub schema: Schema,
    /// Complete definition of the schema in plain-text.
    #[serde(rename = "SchemaDefinition")]
    pub schema_definition: String,

}


#[derive(serde::Serialize, Default)]
pub struct Schema {
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,

}


}

pub mod cfn_schema_version_metadata {

#[derive(serde::Serialize, Default)]
pub struct CfnSchemaVersionMetadata {
    /// Metadata key
    #[serde(rename = "Key")]
    pub key: String,
    /// Represents the version ID associated with the schema version.
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: String,
    /// Metadata value
    #[serde(rename = "Value")]
    pub value: String,

}



}

pub mod cfn_security_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "S3Encryptions")]
    pub s3_encryptions: Option<S3Encryptions>,
    #[serde(rename = "JobBookmarksEncryption")]
    pub job_bookmarks_encryption: Option<JobBookmarksEncryption>,
    #[serde(rename = "CloudWatchEncryption")]
    pub cloud_watch_encryption: Option<CloudWatchEncryption>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Encryptions {

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchEncryption {
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "CloudWatchEncryptionMode")]
    pub cloud_watch_encryption_mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct JobBookmarksEncryption {
    #[serde(rename = "JobBookmarksEncryptionMode")]
    pub job_bookmarks_encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,

}


}

pub mod cfn_table {

#[derive(serde::Serialize, Default)]
pub struct CfnTable {
    /// No documentation provided by AWS
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TableInput")]
    pub table_input: TableInput,
    /// No documentation provided by AWS
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct SchemaReference {
    #[serde(rename = "SchemaVersionNumber")]
    pub schema_version_number: Option<usize>,
    #[serde(rename = "SchemaId")]
    pub schema_id: Option<SchemaId>,
    #[serde(rename = "SchemaVersionId")]
    pub schema_version_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SkewedInfo {
    #[serde(rename = "SkewedColumnNames")]
    pub skewed_column_names: Option<Vec<String>>,
    #[serde(rename = "SkewedColumnValues")]
    pub skewed_column_values: Option<Vec<String>>,
    #[serde(rename = "SkewedColumnValueLocationMaps")]
    pub skewed_column_value_location_maps: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct SchemaId {
    #[serde(rename = "SchemaName")]
    pub schema_name: Option<String>,
    #[serde(rename = "RegistryName")]
    pub registry_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    pub schema_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SerdeInfo {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SerializationLibrary")]
    pub serialization_library: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct TableInput {
    #[serde(rename = "PartitionKeys")]
    pub partition_keys: Option<Vec<Column>>,
    #[serde(rename = "TargetTable")]
    pub target_table: Option<TableIdentifier>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    #[serde(rename = "TableType")]
    pub table_type: Option<String>,
    #[serde(rename = "ViewOriginalText")]
    pub view_original_text: Option<String>,
    #[serde(rename = "StorageDescriptor")]
    pub storage_descriptor: Option<StorageDescriptor>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Retention")]
    pub retention: Option<usize>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ViewExpandedText")]
    pub view_expanded_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Column {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Order {
    #[serde(rename = "Column")]
    pub column: String,
    #[serde(rename = "SortOrder")]
    pub sort_order: usize,

}

#[derive(serde::Serialize, Default)]
pub struct TableIdentifier {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StorageDescriptor {
    #[serde(rename = "Columns")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "SortColumns")]
    pub sort_columns: Option<Vec<Order>>,
    #[serde(rename = "BucketColumns")]
    pub bucket_columns: Option<Vec<String>>,
    #[serde(rename = "SerdeInfo")]
    pub serde_info: Option<SerdeInfo>,
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "StoredAsSubDirectories")]
    pub stored_as_sub_directories: Option<bool>,
    #[serde(rename = "NumberOfBuckets")]
    pub number_of_buckets: Option<usize>,
    #[serde(rename = "SkewedInfo")]
    pub skewed_info: Option<SkewedInfo>,
    #[serde(rename = "OutputFormat")]
    pub output_format: Option<String>,
    #[serde(rename = "InputFormat")]
    pub input_format: Option<String>,
    #[serde(rename = "Location")]
    pub location: Option<String>,
    #[serde(rename = "SchemaReference")]
    pub schema_reference: Option<SchemaReference>,

}


}

pub mod cfn_trigger {

#[derive(serde::Serialize, Default)]
pub struct CfnTrigger {
    /// No documentation provided by AWS
    #[serde(rename = "StartOnCreation")]
    pub start_on_creation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Action
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Predicate")]
    pub predicate: Option<Predicate>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WorkflowName")]
    pub workflow_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EventBatchingCondition")]
    pub event_batching_condition: Option<EventBatchingCondition>,

}


#[derive(serde::Serialize, Default)]
pub struct EventBatchingCondition {
    #[serde(rename = "BatchWindow")]
    pub batch_window: Option<usize>,
    #[serde(rename = "BatchSize")]
    pub batch_size: usize,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationProperty {
    #[serde(rename = "NotifyDelayAfter")]
    pub notify_delay_after: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Predicate {
    #[serde(rename = "Conditions")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "Logical")]
    pub logical: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Condition {
    #[serde(rename = "CrawlerName")]
    pub crawler_name: Option<String>,
    #[serde(rename = "CrawlState")]
    pub crawl_state: Option<String>,
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,
    #[serde(rename = "LogicalOperator")]
    pub logical_operator: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "NotificationProperty")]
    pub notification_property: Option<NotificationProperty>,
    #[serde(rename = "CrawlerName")]
    pub crawler_name: Option<String>,
    #[serde(rename = "Arguments")]
    pub arguments: Option<()>,
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,
    #[serde(rename = "SecurityConfiguration")]
    pub security_configuration: Option<String>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,

}


}

pub mod cfn_workflow {

#[derive(serde::Serialize, Default)]
pub struct CfnWorkflow {
    /// No documentation provided by AWS
    #[serde(rename = "MaxConcurrentRuns")]
    pub max_concurrent_runs: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRunProperties")]
    pub default_run_properties: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}



}
