
pub mod cfn_aggregation_authorization {

#[derive(serde::Serialize, Default)]
pub struct CfnAggregationAuthorization {
    /// The tags for the AggregationAuthorization.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The 12-digit account ID of the account authorized to aggregate data.
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// The region authorized to collect aggregated data.
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_config_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigRule {
    /// No documentation provided by AWS
    #[serde(rename = "Source")]
    pub source: Source,
    /// No documentation provided by AWS
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Scope")]
    pub scope: Option<Scope>,
    /// No documentation provided by AWS
    #[serde(rename = "ComplianceType")]
    pub compliance_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Scope {
    #[serde(rename = "ComplianceResourceTypes")]
    pub compliance_resource_types: Option<Vec<String>>,
    #[serde(rename = "TagValue")]
    pub tag_value: Option<String>,
    #[serde(rename = "ComplianceResourceId")]
    pub compliance_resource_id: Option<String>,
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Source {
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "CustomPolicyDetails")]
    pub custom_policy_details: Option<CustomPolicyDetails>,
    #[serde(rename = "SourceDetails")]
    pub source_details: Option<Vec<SourceDetail>>,
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomPolicyDetails {
    #[serde(rename = "PolicyRuntime")]
    pub policy_runtime: Option<String>,
    #[serde(rename = "EnableDebugLogDelivery")]
    pub enable_debug_log_delivery: Option<bool>,
    #[serde(rename = "PolicyText")]
    pub policy_text: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SourceDetail {
    #[serde(rename = "EventSource")]
    pub event_source: String,
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,

}


}

pub mod cfn_configuration_aggregator {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationAggregator {
    /// The tags for the configuration aggregator.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the aggregator.
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: Option<String>,
    /// List of AccountAggregationSource
    #[serde(rename = "AccountAggregationSources")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationAggregationSource")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,

}


#[derive(serde::Serialize, Default)]
pub struct OrganizationAggregationSource {
    #[serde(rename = "AwsRegions")]
    pub aws_regions: Option<Vec<String>>,
    #[serde(rename = "AllAwsRegions")]
    pub all_aws_regions: Option<bool>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccountAggregationSource {
    #[serde(rename = "AllAwsRegions")]
    pub all_aws_regions: Option<bool>,
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    #[serde(rename = "AwsRegions")]
    pub aws_regions: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_configuration_recorder {

#[derive(serde::Serialize, Default)]
pub struct CfnConfigurationRecorder {
    /// No documentation provided by AWS
    #[serde(rename = "RecordingGroup")]
    pub recording_group: Option<RecordingGroup>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleARN")]
    pub role_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct RecordingGroup {
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,
    #[serde(rename = "IncludeGlobalResourceTypes")]
    pub include_global_resource_types: Option<bool>,
    #[serde(rename = "AllSupported")]
    pub all_supported: Option<bool>,

}


}

pub mod cfn_conformance_pack {

#[derive(serde::Serialize, Default)]
pub struct CfnConformancePack {
    /// The prefix for delivery S3 bucket.
    #[serde(rename = "DeliveryS3KeyPrefix")]
    pub delivery_s3_key_prefix: Option<String>,
    /// AWS Config stores intermediate files while processing conformance pack template.
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: Option<String>,
    /// A string containing full conformance pack template body. You can only specify one of the template body or template S3Uri fields.
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,
    /// Location of file containing the template body which points to the conformance pack template that is located in an Amazon S3 bucket. You can only specify one of the template body or template S3Uri fields.
    #[serde(rename = "TemplateS3Uri")]
    pub template_s3_uri: Option<String>,
    /// The TemplateSSMDocumentDetails object contains the name of the SSM document and the version of the SSM document.
    #[serde(rename = "TemplateSSMDocumentDetails")]
    pub template_ssmdocument_details: Option<()>,
    /// A list of ConformancePackInputParameter objects.
    #[serde(rename = "ConformancePackInputParameters")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// Name of the conformance pack which will be assigned as the unique identifier.
    #[serde(rename = "ConformancePackName")]
    pub conformance_pack_name: String,

}

pub type ParameterValue = String;
#[derive(serde::Serialize, Default)]
pub struct ConformancePackInputParameter {
    #[serde(rename = "ParameterValue")]
    pub parameter_value: ParameterValue,
    #[serde(rename = "ParameterName")]
    pub parameter_name: ParameterName,

}
pub type ParameterName = String;

}

pub mod cfn_delivery_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnDeliveryChannel {
    /// No documentation provided by AWS
    #[serde(rename = "S3KmsKeyArn")]
    pub s3_kms_key_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigSnapshotDeliveryProperties")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "SnsTopicARN")]
    pub sns_topic_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ConfigSnapshotDeliveryProperties {
    #[serde(rename = "DeliveryFrequency")]
    pub delivery_frequency: Option<String>,

}


}

pub mod cfn_organization_config_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnOrganizationConfigRule {
    /// No documentation provided by AWS
    #[serde(rename = "ExcludedAccounts")]
    pub excluded_accounts: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationConfigRuleName")]
    pub organization_config_rule_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationCustomPolicyRuleMetadata")]
    pub organization_custom_policy_rule_metadata: Option<OrganizationCustomPolicyRuleMetadata>,
    /// No documentation provided by AWS
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,

}


#[derive(serde::Serialize, Default)]
pub struct OrganizationCustomRuleMetadata {
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    pub organization_config_rule_trigger_types: Vec<String>,
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct OrganizationCustomPolicyRuleMetadata {
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "PolicyText")]
    pub policy_text: String,
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    pub organization_config_rule_trigger_types: Option<Vec<String>>,
    #[serde(rename = "DebugLogDeliveryAccounts")]
    pub debug_log_delivery_accounts: Option<Vec<String>>,
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct OrganizationManagedRuleMetadata {
    #[serde(rename = "TagValueScope")]
    pub tag_value_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "MaximumExecutionFrequency")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "TagKeyScope")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "RuleIdentifier")]
    pub rule_identifier: String,
    #[serde(rename = "ResourceIdScope")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "InputParameters")]
    pub input_parameters: Option<String>,

}


}

pub mod cfn_organization_conformance_pack {

#[derive(serde::Serialize, Default)]
pub struct CfnOrganizationConformancePack {
    /// The prefix for the delivery S3 bucket.
    #[serde(rename = "DeliveryS3KeyPrefix")]
    pub delivery_s3_key_prefix: Option<String>,
    /// A list of ConformancePackInputParameter objects.
    #[serde(rename = "ConformancePackInputParameters")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    /// Location of file containing the template body.
    #[serde(rename = "TemplateS3Uri")]
    pub template_s3_uri: Option<String>,
    /// The name of the organization conformance pack.
    #[serde(rename = "OrganizationConformancePackName")]
    pub organization_conformance_pack_name: String,
    /// A string containing full conformance pack template body.
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,
    /// AWS Config stores intermediate files while processing conformance pack template.
    #[serde(rename = "DeliveryS3Bucket")]
    pub delivery_s3_bucket: Option<String>,
    /// A list of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack.
    #[serde(rename = "ExcludedAccounts")]
    pub excluded_accounts: Option<Vec<AccountId>>,

}

pub type AccountId = String;pub type ParameterName = String;
#[derive(serde::Serialize, Default)]
pub struct ConformancePackInputParameter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: ParameterName,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: ParameterValue,

}
pub type ParameterValue = String;

}

pub mod cfn_remediation_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnRemediationConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionControls")]
    pub execution_controls: Option<ExecutionControls>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetId")]
    pub target_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetVersion")]
    pub target_version: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaximumAutomaticAttempts")]
    pub maximum_automatic_attempts: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "RetryAttemptSeconds")]
    pub retry_attempt_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetType")]
    pub target_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "Automatic")]
    pub automatic: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct ExecutionControls {
    #[serde(rename = "SsmControls")]
    pub ssm_controls: Option<SsmControls>,

}

#[derive(serde::Serialize, Default)]
pub struct SsmControls {
    #[serde(rename = "ErrorPercentage")]
    pub error_percentage: Option<usize>,
    #[serde(rename = "ConcurrentExecutionRatePercentage")]
    pub concurrent_execution_rate_percentage: Option<usize>,

}


}

pub mod cfn_stored_query {

#[derive(serde::Serialize, Default)]
pub struct CfnStoredQuery {
    /// No documentation provided by AWS
    #[serde(rename = "QueryExpression")]
    pub query_expression: String,
    /// No documentation provided by AWS
    #[serde(rename = "QueryName")]
    pub query_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "QueryDescription")]
    pub query_description: Option<String>,
    /// The tags for the stored query.
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


}
