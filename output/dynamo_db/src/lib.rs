
pub mod cfn_global_table {

#[derive(serde::Serialize, Default)]
pub struct CfnGlobalTable {
    /// List of GlobalSecondaryIndex
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,
    /// List of LocalSecondaryIndex
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,
    /// No documentation provided by AWS
    #[serde(rename = "WriteProvisionedThroughputSettings")]
    pub write_provisioned_throughput_settings: Option<WriteProvisionedThroughputSettings>,
    /// List of ReplicaSpecification
    #[serde(rename = "Replicas")]
    pub replicas: Vec<ReplicaSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<SSESpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    /// List of KeySchema
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    /// List of AttributeDefinition
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,
    /// No documentation provided by AWS
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,

}


#[derive(serde::Serialize, Default)]
pub struct TimeToLiveSpecification {
    #[serde(rename = "AttributeName")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct StreamSpecification {
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicaSpecification {
    #[serde(rename = "KinesisStreamSpecification")]
    pub kinesis_stream_specification: Option<KinesisStreamSpecification>,
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ReadProvisionedThroughputSettings")]
    pub read_provisioned_throughput_settings: Option<ReadProvisionedThroughputSettings>,
    #[serde(rename = "DeletionProtectionEnabled")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndexSpecification>>,
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<ReplicaSSESpecification>,
    #[serde(rename = "PointInTimeRecoverySpecification")]
    pub point_in_time_recovery_specification: Option<PointInTimeRecoverySpecification>,
    #[serde(rename = "TableClass")]
    pub table_class: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Projection {
    #[serde(rename = "ProjectionType")]
    pub projection_type: Option<String>,
    #[serde(rename = "NonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ContributorInsightsSpecification {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityAutoScalingSettings {
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: TargetTrackingScalingPolicyConfiguration,
    #[serde(rename = "SeedCapacity")]
    pub seed_capacity: Option<usize>,
    #[serde(rename = "MinCapacity")]
    pub min_capacity: usize,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct LocalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Projection,

}

#[derive(serde::Serialize, Default)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct PointInTimeRecoverySpecification {
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct KeySchema {
    #[serde(rename = "KeyType")]
    pub key_type: String,
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SSESpecification {
    #[serde(rename = "SSEEnabled")]
    pub sseenabled: bool,
    #[serde(rename = "SSEType")]
    pub ssetype: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicaSSESpecification {
    #[serde(rename = "KMSMasterKeyId")]
    pub kmsmaster_key_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReadProvisionedThroughputSettings {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: Option<usize>,
    #[serde(rename = "ReadCapacityAutoScalingSettings")]
    pub read_capacity_auto_scaling_settings: Option<CapacityAutoScalingSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct WriteProvisionedThroughputSettings {
    #[serde(rename = "WriteCapacityAutoScalingSettings")]
    pub write_capacity_auto_scaling_settings: Option<CapacityAutoScalingSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "WriteProvisionedThroughputSettings")]
    pub write_provisioned_throughput_settings: Option<WriteProvisionedThroughputSettings>,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "Projection")]
    pub projection: Projection,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamSpecification {
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicaGlobalSecondaryIndexSpecification {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,
    #[serde(rename = "ReadProvisionedThroughputSettings")]
    pub read_provisioned_throughput_settings: Option<ReadProvisionedThroughputSettings>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetTrackingScalingPolicyConfiguration {
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<usize>,
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<usize>,

}


}

pub mod cfn_table {

#[derive(serde::Serialize, Default)]
pub struct CfnTable {
    /// No documentation provided by AWS
    #[serde(rename = "PointInTimeRecoverySpecification")]
    pub point_in_time_recovery_specification: Option<PointInTimeRecoverySpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "KinesisStreamSpecification")]
    pub kinesis_stream_specification: Option<KinesisStreamSpecification>,
    /// List of GlobalSecondaryIndex
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<GlobalSecondaryIndex>>,
    /// No documentation provided by AWS
    #[serde(rename = "ImportSourceSpecification")]
    pub import_source_specification: Option<ImportSourceSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "StreamSpecification")]
    pub stream_specification: Option<StreamSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "DeletionProtectionEnabled")]
    pub deletion_protection_enabled: Option<bool>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "KeySchema")]
    pub key_schema: (),
    /// No documentation provided by AWS
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeToLiveSpecification")]
    pub time_to_live_specification: Option<TimeToLiveSpecification>,
    /// List of AttributeDefinition
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    /// List of LocalSecondaryIndex
    #[serde(rename = "LocalSecondaryIndexes")]
    pub local_secondary_indexes: Option<Vec<LocalSecondaryIndex>>,
    /// No documentation provided by AWS
    #[serde(rename = "BillingMode")]
    pub billing_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "TableClass")]
    pub table_class: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    /// No documentation provided by AWS
    #[serde(rename = "SSESpecification")]
    pub ssespecification: Option<SSESpecification>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct PointInTimeRecoverySpecification {
    #[serde(rename = "PointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "ContributorInsightsSpecification")]
    pub contributor_insights_specification: Option<ContributorInsightsSpecification>,

}

#[derive(serde::Serialize, Default)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "KeyType")]
    pub key_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProvisionedThroughput {
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: usize,
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: usize,

}

#[derive(serde::Serialize, Default)]
pub struct DeprecatedKeySchema {
    #[serde(rename = "HashKeyElement")]
    pub hash_key_element: DeprecatedHashKeyElement,

}

#[derive(serde::Serialize, Default)]
pub struct StreamSpecification {
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct LocalSecondaryIndex {
    #[serde(rename = "Projection")]
    pub projection: Projection,
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,

}

#[derive(serde::Serialize, Default)]
pub struct ContributorInsightsSpecification {
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct S3BucketSource {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "S3BucketOwner")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InputFormatOptions {
    #[serde(rename = "Csv")]
    pub csv: Option<Csv>,

}

#[derive(serde::Serialize, Default)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ImportSourceSpecification {
    #[serde(rename = "InputFormatOptions")]
    pub input_format_options: Option<InputFormatOptions>,
    #[serde(rename = "InputCompressionType")]
    pub input_compression_type: Option<String>,
    #[serde(rename = "S3BucketSource")]
    pub s3_bucket_source: S3BucketSource,
    #[serde(rename = "InputFormat")]
    pub input_format: String,

}

#[derive(serde::Serialize, Default)]
pub struct Projection {
    #[serde(rename = "ProjectionType")]
    pub projection_type: Option<String>,
    #[serde(rename = "NonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DeprecatedHashKeyElement {
    #[serde(rename = "AttributeType")]
    pub attribute_type: String,
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TimeToLiveSpecification {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Csv {
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "HeaderList")]
    pub header_list: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct SSESpecification {
    #[serde(rename = "KMSMasterKeyId")]
    pub kmsmaster_key_id: Option<String>,
    #[serde(rename = "SSEType")]
    pub ssetype: Option<String>,
    #[serde(rename = "SSEEnabled")]
    pub sseenabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisStreamSpecification {
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,

}


}
