
pub mod cfn_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnChannel {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<ChannelName>,
    /// One or more resources to which events arriving through a channel are logged and stored.
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Destination>>,
    /// The ARN of an on-premises storage solution or application, or a partner event source.
    #[serde(rename = "Source")]
    pub source: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Destination {
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Type")]
    pub ty: String,

}
pub type ChannelName = String;pub type Timestamp = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type UUID = String;pub type ChannelArn = String;

}

pub mod cfn_event_data_store {

#[derive(serde::Serialize, Default)]
pub struct CfnEventDataStore {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Indicates whether the event data store includes events from all regions, or only from the region in which it was created.
    #[serde(rename = "MultiRegionEnabled")]
    pub multi_region_enabled: Option<bool>,
    /// Indicates whether the event data store is protected from termination.
    #[serde(rename = "TerminationProtectionEnabled")]
    pub termination_protection_enabled: Option<bool>,
    /// The name of the event data store.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Indicates that an event data store is collecting logged events for an organization.
    #[serde(rename = "OrganizationEnabled")]
    pub organization_enabled: Option<bool>,
    /// The advanced event selectors that were used to select events for the data store.
    #[serde(rename = "AdvancedEventSelectors")]
    pub advanced_event_selectors: Option<Vec<AdvancedEventSelector>>,
    /// The retention period, in days.
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: Option<usize>,
    /// Specifies the KMS key ID to use to encrypt the events delivered by CloudTrail. The value can be an alias name prefixed by 'alias/', a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AdvancedFieldSelector {
    #[serde(rename = "Field")]
    pub field: String,
    #[serde(rename = "StartsWith")]
    pub starts_with: Option<Vec<String>>,
    #[serde(rename = "Equals")]
    pub equals: Option<Vec<String>>,
    #[serde(rename = "NotEquals")]
    pub not_equals: Option<Vec<String>>,
    #[serde(rename = "NotEndsWith")]
    pub not_ends_with: Option<Vec<String>>,
    #[serde(rename = "EndsWith")]
    pub ends_with: Option<Vec<String>>,
    #[serde(rename = "NotStartsWith")]
    pub not_starts_with: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedEventSelector {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "FieldSelectors")]
    pub field_selectors: Vec<AdvancedFieldSelector>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Timestamp = String;

}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// A policy document containing permissions to add to the specified resource. In IAM, you must provide policy documents in JSON format. However, in CloudFormation you can provide the policy in JSON or YAML format because CloudFormation converts YAML to JSON before submitting it to IAM.
    #[serde(rename = "ResourcePolicy")]
    pub resource_policy: (),
    /// The ARN of the AWS CloudTrail resource to which the policy applies.
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}



}

pub mod cfn_trail {

#[derive(serde::Serialize, Default)]
pub struct CfnTrail {
    /// No documentation provided by AWS
    #[serde(rename = "TrailName")]
    pub trail_name: Option<String>,
    /// Specifies whether the trail is created for all accounts in an organization in AWS Organizations, or only for the current AWS account. The default is false, and cannot be true unless the call is made on behalf of an AWS account that is the master account for an organization in AWS Organizations.
    #[serde(rename = "IsOrganizationTrail")]
    pub is_organization_trail: Option<bool>,
    /// Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.
    #[serde(rename = "CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: Option<String>,
    /// Use event selectors to further specify the management and data event settings for your trail. By default, trails created without specific event selectors will be configured to log all read and write management events, and no data events. When an event occurs in your account, CloudTrail evaluates the event selector for all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event. You can configure up to five event selectors for a trail.
    #[serde(rename = "EventSelectors")]
    pub event_selectors: Option<Vec<EventSelector>>,
    /// Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be an alias name prefixed by 'alias/', a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.
    #[serde(rename = "KMSKeyId")]
    pub kmskey_id: Option<String>,
    /// Specifies whether the trail is publishing events from global services such as IAM to the log files.
    #[serde(rename = "IncludeGlobalServiceEvents")]
    pub include_global_service_events: Option<bool>,
    /// Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered. Not required unless you specify CloudWatchLogsRoleArn.
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    /// Specifies the name of the Amazon SNS topic defined for notification of log file delivery. The maximum length is 256 characters.
    #[serde(rename = "SnsTopicName")]
    pub sns_topic_name: Option<String>,
    /// Specifies whether log file validation is enabled. The default is false.
    #[serde(rename = "EnableLogFileValidation")]
    pub enable_log_file_validation: Option<bool>,
    /// Specifies whether the trail applies only to the current region or to all regions. The default is false. If the trail exists only in the current region and this value is set to true, shadow trails (replications of the trail) will be created in the other regions. If the trail exists in all regions and this value is set to false, the trail will remain in the region where it was created, and its shadow trails in other regions will be deleted. As a best practice, consider using trails that log events in all regions.
    #[serde(rename = "IsMultiRegionTrail")]
    pub is_multi_region_trail: Option<bool>,
    /// Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see Finding Your CloudTrail Log Files. The maximum length is 200 characters.
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,
    /// Lets you enable Insights event logging by specifying the Insights selectors that you want to enable on an existing trail.
    #[serde(rename = "InsightSelectors")]
    pub insight_selectors: Option<Vec<InsightSelector>>,
    /// Specifies the name of the Amazon S3 bucket designated for publishing log files. See Amazon S3 Bucket Naming Requirements.
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Whether the CloudTrail is currently logging AWS API calls.
    #[serde(rename = "IsLogging")]
    pub is_logging: bool,

}


#[derive(serde::Serialize, Default)]
pub struct EventSelector {
    #[serde(rename = "ReadWriteType")]
    pub read_write_type: Option<String>,
    #[serde(rename = "ExcludeManagementEventSources")]
    pub exclude_management_event_sources: Option<Vec<String>>,
    #[serde(rename = "IncludeManagementEvents")]
    pub include_management_events: Option<bool>,
    #[serde(rename = "DataResources")]
    pub data_resources: Option<Vec<DataResource>>,

}

#[derive(serde::Serialize, Default)]
pub struct InsightSelector {
    #[serde(rename = "InsightType")]
    pub insight_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DataResource {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
