
pub mod cfn_access_point {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPoint {
    /// If you include this field, Amazon S3 restricts access to this Access Point to requests from the specified Virtual Private Cloud (VPC).
    #[serde(rename = "VpcConfiguration")]
    pub vpc_configuration: Option<VpcConfiguration>,
    /// The Access Point Policy you want to apply to this access point.
    #[serde(rename = "Policy")]
    pub policy: Option<()>,
    /// The name of the bucket that you want to associate this Access Point with.
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// The AWS account ID associated with the S3 bucket associated with this access point.
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<String>,
    /// The name you want to assign to this Access Point. If you don't specify a name, AWS CloudFormation generates a unique ID and uses that ID for the access point name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The PublicAccessBlock configuration that you want to apply to this Access Point. You can enable the configuration options in any combination. For more information about when Amazon S3 considers a bucket or object public, see https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status 'The Meaning of Public' in the Amazon Simple Storage Service Developer Guide.
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct VpcConfiguration {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}


}

pub mod cfn_bucket {

#[derive(serde::Serialize, Default)]
pub struct CfnBucket {
    /// Configuration for the transfer acceleration state.
    #[serde(rename = "AccelerateConfiguration")]
    pub accelerate_configuration: Option<AccelerateConfiguration>,
    /// Indicates whether this bucket has an Object Lock configuration enabled.
    #[serde(rename = "ObjectLockEnabled")]
    pub object_lock_enabled: Option<bool>,
    /// Configuration that defines how Amazon S3 handles public access.
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    /// Specifies website configuration parameters for an Amazon S3 bucket.
    #[serde(rename = "WebsiteConfiguration")]
    pub website_configuration: Option<WebsiteConfiguration>,
    /// An arbitrary set of tags (key-value pairs) for this S3 bucket.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Configuration that defines how Amazon S3 handles bucket notifications.
    #[serde(rename = "NotificationConfiguration")]
    pub notification_configuration: Option<NotificationConfiguration>,
    /// Specifies the container element for object ownership rules.
    #[serde(rename = "OwnershipControls")]
    pub ownership_controls: Option<OwnershipControls>,
    /// Rules that define how Amazon S3 manages objects during their lifetime.
    #[serde(rename = "LifecycleConfiguration")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
    /// A name for the bucket. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the bucket name.
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    /// Configuration for replicating objects in an S3 bucket.
    #[serde(rename = "ReplicationConfiguration")]
    pub replication_configuration: Option<ReplicationConfiguration>,
    /// Specifies the S3 Intelligent-Tiering configuration for an Amazon S3 bucket.
    #[serde(rename = "IntelligentTieringConfigurations")]
    pub intelligent_tiering_configurations: Option<Vec<IntelligentTieringConfiguration>>,
    /// Rules that define cross-origin resource sharing of objects in this bucket.
    #[serde(rename = "CorsConfiguration")]
    pub cors_configuration: Option<CorsConfiguration>,
    /// A canned access control list (ACL) that grants predefined permissions to the bucket.
    #[serde(rename = "AccessControl")]
    pub access_control: Option<String>,
    /// Places an Object Lock configuration on the specified bucket.
    #[serde(rename = "ObjectLockConfiguration")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    /// Settings that define a metrics configuration for the CloudWatch request metrics from the bucket.
    #[serde(rename = "MetricsConfigurations")]
    pub metrics_configurations: Option<Vec<MetricsConfiguration>>,
    /// Settings that define where logs are stored.
    #[serde(rename = "LoggingConfiguration")]
    pub logging_configuration: Option<LoggingConfiguration>,
    /// The configuration and any analyses for the analytics filter of an Amazon S3 bucket.
    #[serde(rename = "AnalyticsConfigurations")]
    pub analytics_configurations: Option<Vec<AnalyticsConfiguration>>,
    /// Describes the versioning state of an Amazon S3 bucket.
    #[serde(rename = "VersioningConfiguration")]
    pub versioning_configuration: Option<VersioningConfiguration>,
    /// Specifies default encryption for a bucket using server-side encryption with either Amazon S3-managed keys (SSE-S3) or AWS KMS-managed keys (SSE-KMS).
    #[serde(rename = "BucketEncryption")]
    pub bucket_encryption: Option<BucketEncryption>,
    /// The inventory configuration for an Amazon S3 bucket.
    #[serde(rename = "InventoryConfigurations")]
    pub inventory_configurations: Option<Vec<InventoryConfiguration>>,

}


#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "ObjectSizeGreaterThan")]
    pub object_size_greater_than: Option<String>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    pub expired_object_delete_marker: Option<bool>,
    #[serde(rename = "ObjectSizeLessThan")]
    pub object_size_less_than: Option<String>,
    #[serde(rename = "NoncurrentVersionExpirationInDays")]
    pub noncurrent_version_expiration_in_days: Option<usize>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: Option<iso8601UTC>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "NoncurrentVersionExpiration")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    #[serde(rename = "NoncurrentVersionTransitions")]
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,
    #[serde(rename = "Transition")]
    pub transition: Option<Transition>,
    #[serde(rename = "Transitions")]
    pub transitions: Option<Vec<Transition>>,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "ExpirationInDays")]
    pub expiration_in_days: Option<usize>,
    #[serde(rename = "NoncurrentVersionTransition")]
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,

}

#[derive(serde::Serialize, Default)]
pub struct BucketEncryption {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,

}

#[derive(serde::Serialize, Default)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,

}

#[derive(serde::Serialize, Default)]
pub struct EventBridgeConfiguration {
    #[serde(rename = "EventBridgeEnabled")]
    pub event_bridge_enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct AccelerateConfiguration {
    #[serde(rename = "AccelerationStatus")]
    pub acceleration_status: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaConfiguration {
    #[serde(rename = "Function")]
    pub function: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessControlTranslation {
    #[serde(rename = "Owner")]
    pub owner: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationRuleAndOperator {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationConfiguration {
    #[serde(rename = "Role")]
    pub role: String,
    #[serde(rename = "Rules")]
    pub rules: Vec<ReplicationRule>,

}

#[derive(serde::Serialize, Default)]
pub struct RedirectRule {
    #[serde(rename = "ReplaceKeyPrefixWith")]
    pub replace_key_prefix_with: Option<String>,
    #[serde(rename = "ReplaceKeyWith")]
    pub replace_key_with: Option<String>,
    #[serde(rename = "HttpRedirectCode")]
    pub http_redirect_code: Option<String>,
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RoutingRuleCondition {
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: Option<String>,
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StorageClassAnalysis {
    #[serde(rename = "DataExport")]
    pub data_export: Option<DataExport>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationDestination {
    #[serde(rename = "ReplicationTime")]
    pub replication_time: Option<ReplicationTime>,
    #[serde(rename = "Account")]
    pub account: Option<String>,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "AccessControlTranslation")]
    pub access_control_translation: Option<AccessControlTranslation>,
    #[serde(rename = "Metrics")]
    pub metrics: Option<Metrics>,
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: Option<EncryptionConfiguration>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct DataExport {
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: String,
    #[serde(rename = "Destination")]
    pub destination: Destination,

}

#[derive(serde::Serialize, Default)]
pub struct MetricsConfiguration {
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct S3KeyFilter {
    #[serde(rename = "Rules")]
    pub rules: Vec<FilterRule>,

}

#[derive(serde::Serialize, Default)]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    pub days_after_initiation: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ObjectLockConfiguration {
    #[serde(rename = "ObjectLockEnabled")]
    pub object_lock_enabled: Option<String>,
    #[serde(rename = "Rule")]
    pub rule: Option<ObjectLockRule>,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingConfiguration {
    #[serde(rename = "LogFilePrefix")]
    pub log_file_prefix: Option<String>,
    #[serde(rename = "DestinationBucketName")]
    pub destination_bucket_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CorsRule {
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: Vec<String>,
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: Vec<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge")]
    pub max_age: Option<usize>,
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalyticsConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "StorageClassAnalysis")]
    pub storage_class_analysis: StorageClassAnalysis,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct OwnershipControlsRule {
    #[serde(rename = "ObjectOwnership")]
    pub object_ownership: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct QueueConfiguration {
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationFilter>,
    #[serde(rename = "Queue")]
    pub queue: String,

}

#[derive(serde::Serialize, Default)]
pub struct Transition {
    #[serde(rename = "TransitionInDays")]
    pub transition_in_days: Option<usize>,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
    #[serde(rename = "TransitionDate")]
    pub transition_date: Option<iso8601UTC>,

}

#[derive(serde::Serialize, Default)]
pub struct DefaultRetention {
    #[serde(rename = "Days")]
    pub days: Option<usize>,
    #[serde(rename = "Years")]
    pub years: Option<usize>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TagFilter {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct SseKmsEncryptedObjects {
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct WebsiteConfiguration {
    #[serde(rename = "RedirectAllRequestsTo")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    #[serde(rename = "IndexDocument")]
    pub index_document: Option<String>,
    #[serde(rename = "ErrorDocument")]
    pub error_document: Option<String>,
    #[serde(rename = "RoutingRules")]
    pub routing_rules: Option<Vec<RoutingRule>>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationTimeValue {
    #[serde(rename = "Minutes")]
    pub minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Destination {
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "BucketArn")]
    pub bucket_arn: String,
    #[serde(rename = "Format")]
    pub format: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationRuleFilter {
    #[serde(rename = "And")]
    pub and: Option<ReplicationRuleAndOperator>,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "TagFilter")]
    pub tag_filter: Option<TagFilter>,

}

#[derive(serde::Serialize, Default)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ServerSideEncryptionByDefault")]
    pub server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    #[serde(rename = "BucketKeyEnabled")]
    pub bucket_key_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterRule {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "HostName")]
    pub host_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationRule {
    #[serde(rename = "DeleteMarkerReplication")]
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    #[serde(rename = "Filter")]
    pub filter: Option<ReplicationRuleFilter>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: ReplicationDestination,
    #[serde(rename = "SourceSelectionCriteria")]
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Metrics {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "EventThreshold")]
    pub event_threshold: Option<ReplicationTimeValue>,

}

#[derive(serde::Serialize, Default)]
pub struct Tiering {
    #[serde(rename = "AccessTier")]
    pub access_tier: String,
    #[serde(rename = "Days")]
    pub days: usize,

}

#[derive(serde::Serialize, Default)]
pub struct RoutingRule {
    #[serde(rename = "RedirectRule")]
    pub redirect_rule: RedirectRule,
    #[serde(rename = "RoutingRuleCondition")]
    pub routing_rule_condition: Option<RoutingRuleCondition>,

}

#[derive(serde::Serialize, Default)]
pub struct VersioningConfiguration {
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
    #[serde(rename = "TransitionInDays")]
    pub transition_in_days: usize,
    #[serde(rename = "NewerNoncurrentVersions")]
    pub newer_noncurrent_versions: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceSelectionCriteria {
    #[serde(rename = "ReplicaModifications")]
    pub replica_modifications: Option<ReplicaModifications>,
    #[serde(rename = "SseKmsEncryptedObjects")]
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,

}

#[derive(serde::Serialize, Default)]
pub struct IntelligentTieringConfiguration {
    #[serde(rename = "Tierings")]
    pub tierings: Vec<Tiering>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TopicConfiguration {
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Filter")]
    pub filter: Option<NotificationFilter>,
    #[serde(rename = "Topic")]
    pub topic: String,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: String,

}
pub type iso8601UTC = String;
#[derive(serde::Serialize, Default)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CorsConfiguration {
    #[serde(rename = "CorsRules")]
    pub cors_rules: Vec<CorsRule>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicaModifications {
    #[serde(rename = "Status")]
    pub status: String,

}

#[derive(serde::Serialize, Default)]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "KMSMasterKeyID")]
    pub kmsmaster_key_id: Option<String>,
    #[serde(rename = "SSEAlgorithm")]
    pub ssealgorithm: String,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationFilter {
    #[serde(rename = "S3Key")]
    pub s3_key: S3KeyFilter,

}

#[derive(serde::Serialize, Default)]
pub struct InventoryConfiguration {
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "ScheduleFrequency")]
    pub schedule_frequency: String,
    #[serde(rename = "IncludedObjectVersions")]
    pub included_object_versions: String,
    #[serde(rename = "Destination")]
    pub destination: Destination,
    #[serde(rename = "OptionalFields")]
    pub optional_fields: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationConfiguration {
    #[serde(rename = "EventBridgeConfiguration")]
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    #[serde(rename = "TopicConfigurations")]
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
    #[serde(rename = "LambdaConfigurations")]
    pub lambda_configurations: Option<Vec<LambdaConfiguration>>,
    #[serde(rename = "QueueConfigurations")]
    pub queue_configurations: Option<Vec<QueueConfiguration>>,

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationTime {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Time")]
    pub time: ReplicationTimeValue,

}

#[derive(serde::Serialize, Default)]
pub struct DeleteMarkerReplication {
    #[serde(rename = "Status")]
    pub status: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NewerNoncurrentVersions")]
    pub newer_noncurrent_versions: Option<usize>,
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: usize,

}

#[derive(serde::Serialize, Default)]
pub struct ObjectLockRule {
    #[serde(rename = "DefaultRetention")]
    pub default_retention: Option<DefaultRetention>,

}

#[derive(serde::Serialize, Default)]
pub struct OwnershipControls {
    #[serde(rename = "Rules")]
    pub rules: Vec<OwnershipControlsRule>,

}


}

pub mod cfn_bucket_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnBucketPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    /// No documentation provided by AWS
    #[serde(rename = "Bucket")]
    pub bucket: String,

}



}

pub mod cfn_multi_region_access_point {

#[derive(serde::Serialize, Default)]
pub struct CfnMultiRegionAccessPoint {
    /// The list of buckets that you want to associate this Multi Region Access Point with.
    #[serde(rename = "Regions")]
    pub regions: Vec<Region>,
    /// The name you want to assign to this Multi Region Access Point.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The PublicAccessBlock configuration that you want to apply to this Multi Region Access Point. You can enable the configuration options in any combination. For more information about when Amazon S3 considers a bucket or object public, see https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status 'The Meaning of Public' in the Amazon Simple Storage Service Developer Guide.
    #[serde(rename = "PublicAccessBlockConfiguration")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct Region {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "BucketAccountId")]
    pub bucket_account_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "IgnorePublicAcls")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "BlockPublicAcls")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    pub restrict_public_buckets: Option<bool>,

}


}

pub mod cfn_multi_region_access_point_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnMultiRegionAccessPointPolicy {
    /// The name of the Multi Region Access Point to apply policy
    #[serde(rename = "MrapName")]
    pub mrap_name: String,
    /// Policy document to apply to a Multi Region Access Point
    #[serde(rename = "Policy")]
    pub policy: (),

}



}

pub mod cfn_storage_lens {

#[derive(serde::Serialize, Default)]
pub struct CfnStorageLens {
    /// A set of tags (key-value pairs) for this Amazon S3 Storage Lens configuration.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the details of Amazon S3 Storage Lens configuration.
    #[serde(rename = "StorageLensConfiguration")]
    pub storage_lens_configuration: StorageLensConfiguration,

}


#[derive(serde::Serialize, Default)]
pub struct DataExport {
    #[serde(rename = "S3BucketDestination")]
    pub s3_bucket_destination: Option<S3BucketDestination>,
    #[serde(rename = "CloudWatchMetrics")]
    pub cloud_watch_metrics: Option<CloudWatchMetrics>,

}

#[derive(serde::Serialize, Default)]
pub struct AccountLevel {
    #[serde(rename = "DetailedStatusCodesMetrics")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,
    #[serde(rename = "ActivityMetrics")]
    pub activity_metrics: Option<ActivityMetrics>,
    #[serde(rename = "BucketLevel")]
    pub bucket_level: BucketLevel,

}

#[derive(serde::Serialize, Default)]
pub struct DetailedStatusCodesMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ActivityMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PrefixLevelStorageMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "SelectionCriteria")]
    pub selection_criteria: Option<SelectionCriteria>,

}

#[derive(serde::Serialize, Default)]
pub struct Encryption {

}

#[derive(serde::Serialize, Default)]
pub struct S3BucketDestination {
    #[serde(rename = "OutputSchemaVersion")]
    pub output_schema_version: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Encryption")]
    pub encryption: Option<Encryption>,

}

#[derive(serde::Serialize, Default)]
pub struct SelectionCriteria {
    #[serde(rename = "MinStorageBytesPercentage")]
    pub min_storage_bytes_percentage: Option<f64>,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "MaxDepth")]
    pub max_depth: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PrefixLevel {
    #[serde(rename = "StorageMetrics")]
    pub storage_metrics: PrefixLevelStorageMetrics,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,

}
pub type Id = String;pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct StorageLensConfiguration {
    #[serde(rename = "AccountLevel")]
    pub account_level: AccountLevel,
    #[serde(rename = "AwsOrg")]
    pub aws_org: Option<AwsOrg>,
    #[serde(rename = "Id")]
    pub id: Id,
    #[serde(rename = "StorageLensArn")]
    pub storage_lens_arn: Option<String>,
    #[serde(rename = "Include")]
    pub include: Option<BucketsAndRegions>,
    #[serde(rename = "Exclude")]
    pub exclude: Option<BucketsAndRegions>,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "DataExport")]
    pub data_export: Option<DataExport>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedCostOptimizationMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct BucketLevel {
    #[serde(rename = "ActivityMetrics")]
    pub activity_metrics: Option<ActivityMetrics>,
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,
    #[serde(rename = "PrefixLevel")]
    pub prefix_level: Option<PrefixLevel>,
    #[serde(rename = "DetailedStatusCodesMetrics")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,

}

#[derive(serde::Serialize, Default)]
pub struct SSEKMS {
    #[serde(rename = "KeyId")]
    pub key_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct AwsOrg {
    #[serde(rename = "Arn")]
    pub arn: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct BucketsAndRegions {
    #[serde(rename = "Buckets")]
    pub buckets: Option<Vec<Arn>>,
    #[serde(rename = "Regions")]
    pub regions: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedDataProtectionMetrics {
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

}


}
