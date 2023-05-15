
pub mod cfn_association {

#[derive(serde::Serialize, Default)]
pub struct CfnAssociation {
    /// The version of the SSM document to associate with the target.
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncCompliance")]
    pub sync_compliance: Option<String>,
    /// Parameter values that the SSM document uses at runtime.
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// A Cron or Rate expression that specifies when the association is applied to the target.
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxErrors")]
    pub max_errors: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "WaitForSuccessTimeoutSeconds")]
    pub wait_for_success_timeout_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "CalendarNames")]
    pub calendar_names: Option<Vec<String>>,
    /// The name of the association.
    #[serde(rename = "AssociationName")]
    pub association_name: Option<String>,
    /// The ID of the instance that the SSM document is associated with.
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    pub apply_only_at_cron_interval: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "OutputLocation")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    /// The name of the SSM document.
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ComplianceSeverity")]
    pub compliance_severity: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<usize>,
    /// The targets that the SSM document sends commands to.
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,
    /// No documentation provided by AWS
    #[serde(rename = "AutomationTargetParameterName")]
    pub automation_target_parameter_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct S3OutputLocation {
    #[serde(rename = "OutputS3Region")]
    pub output_s3_region: Option<S3Region>,
    #[serde(rename = "OutputS3BucketName")]
    pub output_s3_bucket_name: Option<S3BucketName>,
    #[serde(rename = "OutputS3KeyPrefix")]
    pub output_s3_key_prefix: Option<S3KeyPrefix>,

}
pub type S3BucketName = String;
#[derive(serde::Serialize, Default)]
pub struct InstanceAssociationOutputLocation {
    #[serde(rename = "S3Location")]
    pub s3_location: Option<S3OutputLocation>,

}

#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}
pub type ParameterValues = Vec<String>;pub type S3Region = String;pub type S3KeyPrefix = String;

}

pub mod cfn_document {

#[derive(serde::Serialize, Default)]
pub struct CfnDocument {
    /// Specify a target type to define the kinds of resources the document can run on.
    #[serde(rename = "TargetType")]
    pub target_type: Option<String>,
    /// A name for the Systems Manager document.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// A list of SSM documents required by a document. For example, an ApplicationConfiguration document requires an ApplicationConfigurationSchema document.
    #[serde(rename = "Requires")]
    pub requires: Option<Vec<DocumentRequires>>,
    /// Update method - when set to 'Replace', the update will replace the existing document; when set to 'NewVersion', the update will create a new version.
    #[serde(rename = "UpdateMethod")]
    pub update_method: Option<String>,
    /// The content for the Systems Manager document in JSON, YAML or String format.
    #[serde(rename = "Content")]
    pub content: (),
    /// Specify the document format for the request. The document format can be either JSON or YAML. JSON is the default format.
    #[serde(rename = "DocumentFormat")]
    pub document_format: Option<String>,
    /// Optional metadata that you assign to a resource. Tags enable you to categorize a resource in different ways, such as by purpose, owner, or environment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The type of document to create.
    #[serde(rename = "DocumentType")]
    pub document_type: Option<String>,
    /// A list of key and value pairs that describe attachments to a version of a document.
    #[serde(rename = "Attachments")]
    pub attachments: Option<Vec<AttachmentsSource>>,
    /// An optional field specifying the version of the artifact you are creating with the document. This value is unique across all versions of a document, and cannot be changed.
    #[serde(rename = "VersionName")]
    pub version_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AttachmentsSource {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DocumentRequires {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,

}


}

pub mod cfn_maintenance_window {

#[derive(serde::Serialize, Default)]
pub struct CfnMaintenanceWindow {
    /// No documentation provided by AWS
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowUnassociatedTargets")]
    pub allow_unassociated_targets: bool,
    /// No documentation provided by AWS
    #[serde(rename = "Duration")]
    pub duration: usize,
    /// No documentation provided by AWS
    #[serde(rename = "Cutoff")]
    pub cutoff: usize,
    /// No documentation provided by AWS
    #[serde(rename = "Schedule")]
    pub schedule: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ScheduleTimezone")]
    pub schedule_timezone: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_maintenance_window_target {

#[derive(serde::Serialize, Default)]
pub struct CfnMaintenanceWindowTarget {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OwnerInformation")]
    pub owner_information: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// List of Targets
    #[serde(rename = "Targets")]
    pub targets: Vec<Targets>,
    /// No documentation provided by AWS
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Targets {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Values")]
    pub values: Vec<String>,

}


}

pub mod cfn_maintenance_window_task {

#[derive(serde::Serialize, Default)]
pub struct CfnMaintenanceWindowTask {
    /// No documentation provided by AWS
    #[serde(rename = "Priority")]
    pub priority: usize,
    /// No documentation provided by AWS
    #[serde(rename = "TaskInvocationParameters")]
    pub task_invocation_parameters: Option<TaskInvocationParameters>,
    /// List of Target
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,
    /// No documentation provided by AWS
    #[serde(rename = "WindowId")]
    pub window_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "CutoffBehavior")]
    pub cutoff_behavior: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "LoggingInfo")]
    pub logging_info: Option<LoggingInfo>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxErrors")]
    pub max_errors: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TaskParameters")]
    pub task_parameters: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindowRunCommandParameters {
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "NotificationConfig")]
    pub notification_config: Option<NotificationConfig>,
    #[serde(rename = "CloudWatchOutputConfig")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "DocumentHashType")]
    pub document_hash_type: Option<String>,
    #[serde(rename = "OutputS3BucketName")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "TimeoutSeconds")]
    pub timeout_seconds: Option<usize>,
    #[serde(rename = "DocumentHash")]
    pub document_hash: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    pub output_s3_key_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindowAutomationParameters {
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TaskInvocationParameters {
    #[serde(rename = "MaintenanceWindowLambdaParameters")]
    pub maintenance_window_lambda_parameters: Option<MaintenanceWindowLambdaParameters>,
    #[serde(rename = "MaintenanceWindowStepFunctionsParameters")]
    pub maintenance_window_step_functions_parameters: Option<MaintenanceWindowStepFunctionsParameters>,
    #[serde(rename = "MaintenanceWindowAutomationParameters")]
    pub maintenance_window_automation_parameters: Option<MaintenanceWindowAutomationParameters>,
    #[serde(rename = "MaintenanceWindowRunCommandParameters")]
    pub maintenance_window_run_command_parameters: Option<MaintenanceWindowRunCommandParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationConfig {
    #[serde(rename = "NotificationEvents")]
    pub notification_events: Option<Vec<String>>,
    #[serde(rename = "NotificationArn")]
    pub notification_arn: String,
    #[serde(rename = "NotificationType")]
    pub notification_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LoggingInfo {
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "S3Prefix")]
    pub s3_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindowStepFunctionsParameters {
    #[serde(rename = "Input")]
    pub input: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindowLambdaParameters {
    #[serde(rename = "Qualifier")]
    pub qualifier: Option<String>,
    #[serde(rename = "Payload")]
    pub payload: Option<String>,
    #[serde(rename = "ClientContext")]
    pub client_context: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudWatchOutputConfig {
    #[serde(rename = "CloudWatchLogGroupName")]
    pub cloud_watch_log_group_name: Option<String>,
    #[serde(rename = "CloudWatchOutputEnabled")]
    pub cloud_watch_output_enabled: Option<bool>,

}


}

pub mod cfn_parameter {

#[derive(serde::Serialize, Default)]
pub struct CfnParameter {
    /// No documentation provided by AWS
    #[serde(rename = "AllowedPattern")]
    pub allowed_pattern: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Value")]
    pub value: String,
    /// No documentation provided by AWS
    #[serde(rename = "Policies")]
    pub policies: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tier")]
    pub tier: Option<String>,

}



}

pub mod cfn_patch_baseline {

#[derive(serde::Serialize, Default)]
pub struct CfnPatchBaseline {
    /// No documentation provided by AWS
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    pub approved_patches_compliance_level: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "ApprovalRules")]
    pub approval_rules: Option<RuleGroup>,
    /// No documentation provided by AWS
    #[serde(rename = "PatchGroups")]
    pub patch_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ApprovedPatches")]
    pub approved_patches: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    pub approved_patches_enable_non_security: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "RejectedPatches")]
    pub rejected_patches: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GlobalFilters")]
    pub global_filters: Option<PatchFilterGroup>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of PatchSource
    #[serde(rename = "Sources")]
    pub sources: Option<Vec<PatchSource>>,
    /// No documentation provided by AWS
    #[serde(rename = "OperatingSystem")]
    pub operating_system: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RejectedPatchesAction")]
    pub rejected_patches_action: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct PatchFilter {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "ApproveUntilDate")]
    pub approve_until_date: Option<PatchStringDate>,
    #[serde(rename = "ComplianceLevel")]
    pub compliance_level: Option<String>,
    #[serde(rename = "ApproveAfterDays")]
    pub approve_after_days: Option<usize>,
    #[serde(rename = "EnableNonSecurity")]
    pub enable_non_security: Option<bool>,
    #[serde(rename = "PatchFilterGroup")]
    pub patch_filter_group: Option<PatchFilterGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct PatchStringDate {

}

#[derive(serde::Serialize, Default)]
pub struct PatchSource {
    #[serde(rename = "Configuration")]
    pub configuration: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Products")]
    pub products: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct RuleGroup {
    #[serde(rename = "PatchRules")]
    pub patch_rules: Option<Vec<Rule>>,

}

#[derive(serde::Serialize, Default)]
pub struct PatchFilterGroup {
    #[serde(rename = "PatchFilters")]
    pub patch_filters: Option<Vec<PatchFilter>>,

}


}

pub mod cfn_resource_data_sync {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceDataSync {
    /// No documentation provided by AWS
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncSource")]
    pub sync_source: Option<SyncSource>,
    /// No documentation provided by AWS
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncFormat")]
    pub sync_format: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BucketRegion")]
    pub bucket_region: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "KMSKeyArn")]
    pub kmskey_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SyncType")]
    pub sync_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "S3Destination")]
    pub s3_destination: Option<S3Destination>,

}


#[derive(serde::Serialize, Default)]
pub struct AwsOrganizationsSource {
    #[serde(rename = "OrganizationalUnits")]
    pub organizational_units: Option<Vec<String>>,
    #[serde(rename = "OrganizationSourceType")]
    pub organization_source_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Destination {
    #[serde(rename = "SyncFormat")]
    pub sync_format: String,
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "KMSKeyArn")]
    pub kmskey_arn: Option<String>,
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "BucketRegion")]
    pub bucket_region: String,

}

#[derive(serde::Serialize, Default)]
pub struct SyncSource {
    #[serde(rename = "SourceType")]
    pub source_type: String,
    #[serde(rename = "IncludeFutureRegions")]
    pub include_future_regions: Option<bool>,
    #[serde(rename = "SourceRegions")]
    pub source_regions: Vec<String>,
    #[serde(rename = "AwsOrganizationsSource")]
    pub aws_organizations_source: Option<AwsOrganizationsSource>,

}


}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// Actual policy statement.
    #[serde(rename = "Policy")]
    pub policy: (),
    /// Arn of OpsItemGroup etc.
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}



}
