
pub mod cfn_account_audit_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnAccountAuditConfiguration {
    /// The ARN of the role that grants permission to AWS IoT to access information about your devices, policies, certificates and other items as required when performing an audit.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Information about the targets to which audit notifications are sent.
    #[serde(rename = "AuditNotificationTargetConfigurations")]
    pub audit_notification_target_configurations: Option<AuditNotificationTargetConfigurations>,
    /// Your 12-digit account ID (used as the primary identifier for the CloudFormation resource).
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// Specifies which audit checks are enabled and disabled for this account.
    #[serde(rename = "AuditCheckConfigurations")]
    pub audit_check_configurations: AuditCheckConfigurations,

}


#[derive(serde::Serialize, Default)]
pub struct AuditCheckConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct AuditNotificationTargetConfigurations {
    #[serde(rename = "Sns")]
    pub sns: Option<AuditNotificationTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct AuditCheckConfigurations {
    #[serde(rename = "RevokedCaCertificateStillActiveCheck")]
    pub revoked_ca_certificate_still_active_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "ConflictingClientIdsCheck")]
    pub conflicting_client_ids_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "IntermediateCaRevokedForActiveDeviceCertificatesCheck")]
    pub intermediate_ca_revoked_for_active_device_certificates_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "RevokedDeviceCertificateStillActiveCheck")]
    pub revoked_device_certificate_still_active_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "CaCertificateKeyQualityCheck")]
    pub ca_certificate_key_quality_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "IotPolicyOverlyPermissiveCheck")]
    pub iot_policy_overly_permissive_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "DeviceCertificateSharedCheck")]
    pub device_certificate_shared_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "IotRoleAliasOverlyPermissiveCheck")]
    pub iot_role_alias_overly_permissive_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "LoggingDisabledCheck")]
    pub logging_disabled_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "DeviceCertificateKeyQualityCheck")]
    pub device_certificate_key_quality_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "AuthenticatedCognitoRoleOverlyPermissiveCheck")]
    pub authenticated_cognito_role_overly_permissive_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "UnauthenticatedCognitoRoleOverlyPermissiveCheck")]
    pub unauthenticated_cognito_role_overly_permissive_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "DeviceCertificateExpiringCheck")]
    pub device_certificate_expiring_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "IotRoleAliasAllowsAccessToUnusedServicesCheck")]
    pub iot_role_alias_allows_access_to_unused_services_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "CaCertificateExpiringCheck")]
    pub ca_certificate_expiring_check: Option<AuditCheckConfiguration>,
    #[serde(rename = "IoTPolicyPotentialMisConfigurationCheck")]
    pub io_tpolicy_potential_mis_configuration_check: Option<AuditCheckConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct AuditNotificationTarget {
    #[serde(rename = "TargetArn")]
    pub target_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}


}

pub mod cfn_authorizer {

#[derive(serde::Serialize, Default)]
pub struct CfnAuthorizer {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "SigningDisabled")]
    pub signing_disabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TokenKeyName")]
    pub token_key_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableCachingForHttp")]
    pub enable_caching_for_http: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TokenSigningPublicKeys")]
    pub token_signing_public_keys: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerFunctionArn")]
    pub authorizer_function_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerName")]
    pub authorizer_name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_billing_group {

#[derive(serde::Serialize, Default)]
pub struct CfnBillingGroup {
    /// No documentation provided by AWS
    #[serde(rename = "BillingGroupName")]
    pub billing_group_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "BillingGroupProperties")]
    pub billing_group_properties: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_cacertificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCACertificate {
    /// No documentation provided by AWS
    #[serde(rename = "CertificateMode")]
    pub certificate_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RemoveAutoRegistration")]
    pub remove_auto_registration: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "CACertificatePem")]
    pub cacertificate_pem: String,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: String,
    /// The private key verification certificate.
    #[serde(rename = "VerificationCertificatePem")]
    pub verification_certificate_pem: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoRegistrationStatus")]
    pub auto_registration_status: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RegistrationConfig")]
    pub registration_config: Option<RegistrationConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct RegistrationConfig {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateBody")]
    pub template_body: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_certificate {

#[derive(serde::Serialize, Default)]
pub struct CfnCertificate {
    /// No documentation provided by AWS
    #[serde(rename = "CertificatePem")]
    pub certificate_pem: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateMode")]
    pub certificate_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CACertificatePem")]
    pub cacertificate_pem: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CertificateSigningRequest")]
    pub certificate_signing_request: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Status")]
    pub status: String,

}



}

pub mod cfn_custom_metric {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomMetric {
    /// The type of the custom metric. Types include string-list, ip-address-list, number-list, and number.
    #[serde(rename = "MetricType")]
    pub metric_type: String,
    /// The name of the custom metric. This will be used in the metric report submitted from the device/thing. Shouldn't begin with aws: . Cannot be updated once defined.
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    /// Field represents a friendly name in the console for the custom metric; it doesn't have to be unique. Don't use this name as the metric identifier in the device metric report. Can be updated once defined.
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
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

pub mod cfn_dimension {

#[derive(serde::Serialize, Default)]
pub struct CfnDimension {
    /// A unique identifier for the dimension.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Metadata that can be used to manage the dimension.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the type of the dimension.
    #[serde(rename = "Type")]
    pub ty: String,
    /// Specifies the value or list of values for the dimension.
    #[serde(rename = "StringValues")]
    pub string_values: Vec<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_domain_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnDomainConfiguration {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainConfigurationName")]
    pub domain_configuration_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizerConfig")]
    pub authorizer_config: Option<AuthorizerConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ValidationCertificateArn")]
    pub validation_certificate_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DomainConfigurationStatus")]
    pub domain_configuration_status: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerCertificateArns")]
    pub server_certificate_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceType")]
    pub service_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TlsConfig")]
    pub tls_config: Option<TlsConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct ServerCertificateSummary {
    #[serde(rename = "ServerCertificateStatusDetail")]
    pub server_certificate_status_detail: Option<String>,
    #[serde(rename = "ServerCertificateStatus")]
    pub server_certificate_status: Option<String>,
    #[serde(rename = "ServerCertificateArn")]
    pub server_certificate_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AuthorizerConfig {
    #[serde(rename = "DefaultAuthorizerName")]
    pub default_authorizer_name: Option<String>,
    #[serde(rename = "AllowAuthorizerOverride")]
    pub allow_authorizer_override: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TlsConfig {
    #[serde(rename = "SecurityPolicy")]
    pub security_policy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_fleet_metric {

#[derive(serde::Serialize, Default)]
pub struct CfnFleetMetric {
    /// The index name of a fleet metric
    #[serde(rename = "IndexName")]
    pub index_name: Option<String>,
    /// The aggregation field to perform aggregation and metric emission
    #[serde(rename = "AggregationField")]
    pub aggregation_field: Option<String>,
    /// The version of a Fleet Indexing query used by a fleet metric
    #[serde(rename = "QueryVersion")]
    pub query_version: Option<String>,
    /// The period of metric emission in seconds
    #[serde(rename = "Period")]
    pub period: Option<usize>,
    /// The Fleet Indexing query used by a fleet metric
    #[serde(rename = "QueryString")]
    pub query_string: Option<String>,
    /// The description of a fleet metric
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// An array of key-value pairs to apply to this resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the fleet metric
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// The unit of data points emitted by a fleet metric
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    /// Aggregation types supported by Fleet Indexing
    #[serde(rename = "AggregationType")]
    pub aggregation_type: Option<AggregationType>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct AggregationType {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Name")]
    pub name: String,

}


}

pub mod cfn_job_template {

#[derive(serde::Serialize, Default)]
pub struct CfnJobTemplate {
    /// Allows you to create a staged rollout of a job.
    #[serde(rename = "JobExecutionsRolloutConfig")]
    pub job_executions_rollout_config: Option<()>,
    /// Optional for copying a JobTemplate from a pre-existing Job configuration.
    #[serde(rename = "JobArn")]
    pub job_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "JobTemplateId")]
    pub job_template_id: String,
    /// Configuration for pre-signed S3 URLs.
    #[serde(rename = "PresignedUrlConfig")]
    pub presigned_url_config: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "JobExecutionsRetryConfig")]
    pub job_executions_retry_config: Option<()>,
    /// List of MaintenanceWindow
    #[serde(rename = "MaintenanceWindows")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,
    /// Metadata that can be used to manage the JobTemplate.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The criteria that determine when and how a job abort takes place.
    #[serde(rename = "AbortConfig")]
    pub abort_config: Option<()>,
    /// A description of the Job Template.
    #[serde(rename = "Description")]
    pub description: String,
    /// Specifies the amount of time each device has to finish its execution of the job.
    #[serde(rename = "TimeoutConfig")]
    pub timeout_config: Option<()>,
    /// The job document. Required if you don't specify a value for documentSource.
    #[serde(rename = "Document")]
    pub document: Option<String>,
    /// An S3 link to the job document to use in the template. Required if you don't specify a value for document.
    #[serde(rename = "DocumentSource")]
    pub document_source: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ExponentialRolloutRate {
    #[serde(rename = "BaseRatePerMinute")]
    pub base_rate_per_minute: BaseRatePerMinute,
    #[serde(rename = "RateIncreaseCriteria")]
    pub rate_increase_criteria: RateIncreaseCriteria,
    #[serde(rename = "IncrementFactor")]
    pub increment_factor: IncrementFactor,

}
pub type MaximumPerMinute = usize;pub type InProgressTimeoutInMinutes = usize;pub type Action = String;
#[derive(serde::Serialize, Default)]
pub struct RateIncreaseCriteria {
    #[serde(rename = "NumberOfSucceededThings")]
    pub number_of_succeeded_things: Option<NumberOfSucceededThings>,
    #[serde(rename = "NumberOfNotifiedThings")]
    pub number_of_notified_things: Option<NumberOfNotifiedThings>,

}

#[derive(serde::Serialize, Default)]
pub struct AbortCriteria {
    #[serde(rename = "ThresholdPercentage")]
    pub threshold_percentage: ThresholdPercentage,
    #[serde(rename = "MinNumberOfExecutedThings")]
    pub min_number_of_executed_things: MinNumberOfExecutedThings,
    #[serde(rename = "Action")]
    pub action: Action,
    #[serde(rename = "FailureType")]
    pub failure_type: FailureType,

}
pub type ExpiresInSec = usize;pub type MinNumberOfExecutedThings = usize;pub type IncrementFactor = f64;pub type NumberOfSucceededThings = usize;
#[derive(serde::Serialize, Default)]
pub struct MaintenanceWindow {
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: Option<usize>,

}
pub type BaseRatePerMinute = usize;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type NumberOfNotifiedThings = usize;
#[derive(serde::Serialize, Default)]
pub struct RetryCriteria {
    #[serde(rename = "NumberOfRetries")]
    pub number_of_retries: Option<usize>,
    #[serde(rename = "FailureType")]
    pub failure_type: Option<JobRetryFailureType>,

}
pub type ThresholdPercentage = f64;pub type RoleArn = String;pub type JobRetryFailureType = String;pub type FailureType = String;

}

pub mod cfn_logging {

#[derive(serde::Serialize, Default)]
pub struct CfnLogging {
    /// The log level to use. Valid values are: ERROR, WARN, INFO, DEBUG, or DISABLED.
    #[serde(rename = "DefaultLogLevel")]
    pub default_log_level: String,
    /// Your 12-digit account ID (used as the primary identifier for the CloudFormation resource).
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// The ARN of the role that allows IoT to write to Cloudwatch logs.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



}

pub mod cfn_mitigation_action {

#[derive(serde::Serialize, Default)]
pub struct CfnMitigationAction {
    /// A unique identifier for the mitigation action.
    #[serde(rename = "ActionName")]
    pub action_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// The set of parameters for this mitigation action. You can specify only one type of parameter (in other words, you can apply only one action for each defined mitigation action).
    #[serde(rename = "ActionParams")]
    pub action_params: ActionParams,

}


#[derive(serde::Serialize, Default)]
pub struct EnableIoTLoggingParams {
    #[serde(rename = "LogLevel")]
    pub log_level: String,
    #[serde(rename = "RoleArnForLogging")]
    pub role_arn_for_logging: String,

}

#[derive(serde::Serialize, Default)]
pub struct UpdateDeviceCertificateParams {
    #[serde(rename = "Action")]
    pub action: String,

}

#[derive(serde::Serialize, Default)]
pub struct UpdateCACertificateParams {
    #[serde(rename = "Action")]
    pub action: String,

}

#[derive(serde::Serialize, Default)]
pub struct ActionParams {
    #[serde(rename = "AddThingsToThingGroupParams")]
    pub add_things_to_thing_group_params: Option<AddThingsToThingGroupParams>,
    #[serde(rename = "EnableIoTLoggingParams")]
    pub enable_io_tlogging_params: Option<EnableIoTLoggingParams>,
    #[serde(rename = "UpdateCACertificateParams")]
    pub update_cacertificate_params: Option<UpdateCACertificateParams>,
    #[serde(rename = "UpdateDeviceCertificateParams")]
    pub update_device_certificate_params: Option<UpdateDeviceCertificateParams>,
    #[serde(rename = "PublishFindingToSnsParams")]
    pub publish_finding_to_sns_params: Option<PublishFindingToSnsParams>,
    #[serde(rename = "ReplaceDefaultPolicyVersionParams")]
    pub replace_default_policy_version_params: Option<ReplaceDefaultPolicyVersionParams>,

}

#[derive(serde::Serialize, Default)]
pub struct AddThingsToThingGroupParams {
    #[serde(rename = "OverrideDynamicGroups")]
    pub override_dynamic_groups: Option<bool>,
    #[serde(rename = "ThingGroupNames")]
    pub thing_group_names: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ReplaceDefaultPolicyVersionParams {
    #[serde(rename = "TemplateName")]
    pub template_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct PublishFindingToSnsParams {
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,

}


}

pub mod cfn_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDocument")]
    pub policy_document: (),
    /// No documentation provided by AWS
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,

}



}

pub mod cfn_policy_principal_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnPolicyPrincipalAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: String,

}



}

pub mod cfn_provisioning_template {

#[derive(serde::Serialize, Default)]
pub struct CfnProvisioningTemplate {
    /// No documentation provided by AWS
    #[serde(rename = "ProvisioningRoleArn")]
    pub provisioning_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "PreProvisioningHook")]
    pub pre_provisioning_hook: Option<ProvisioningHook>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateBody")]
    pub template_body: String,
    /// No documentation provided by AWS
    #[serde(rename = "TemplateType")]
    pub template_type: Option<String>,
    /// No documentation provided by AWS
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
pub struct ProvisioningHook {
    #[serde(rename = "PayloadVersion")]
    pub payload_version: Option<String>,
    #[serde(rename = "TargetArn")]
    pub target_arn: Option<String>,

}


}

pub mod cfn_resource_specific_logging {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceSpecificLogging {
    /// The target name.
    #[serde(rename = "TargetName")]
    pub target_name: String,
    /// The log level for a specific target. Valid values are: ERROR, WARN, INFO, DEBUG, or DISABLED.
    #[serde(rename = "LogLevel")]
    pub log_level: String,
    /// The target type. Value must be THING_GROUP, CLIENT_ID, SOURCE_IP, PRINCIPAL_ID, or EVENT_TYPE.
    #[serde(rename = "TargetType")]
    pub target_type: String,

}



}

pub mod cfn_role_alias {

#[derive(serde::Serialize, Default)]
pub struct CfnRoleAlias {
    /// No documentation provided by AWS
    #[serde(rename = "CredentialDurationSeconds")]
    pub credential_duration_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "RoleAlias")]
    pub role_alias: Option<String>,
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


}

pub mod cfn_scheduled_audit {

#[derive(serde::Serialize, Default)]
pub struct CfnScheduledAudit {
    /// The name you want to give to the scheduled audit.
    #[serde(rename = "ScheduledAuditName")]
    pub scheduled_audit_name: Option<String>,
    /// The day of the month on which the scheduled audit takes place. Can be 1 through 31 or LAST. This field is required if the frequency parameter is set to MONTHLY.
    #[serde(rename = "DayOfMonth")]
    pub day_of_month: Option<String>,
    /// Which checks are performed during the scheduled audit. Checks must be enabled for your account.
    #[serde(rename = "TargetCheckNames")]
    pub target_check_names: Vec<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The day of the week on which the scheduled audit takes place. Can be one of SUN, MON, TUE,WED, THU, FRI, or SAT. This field is required if the frequency parameter is set to WEEKLY or BIWEEKLY.
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: Option<String>,
    /// How often the scheduled audit takes place. Can be one of DAILY, WEEKLY, BIWEEKLY, or MONTHLY.
    #[serde(rename = "Frequency")]
    pub frequency: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_security_profile {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityProfile {
    /// A description of the security profile.
    #[serde(rename = "SecurityProfileDescription")]
    pub security_profile_description: Option<String>,
    /// A list of metrics whose data is retained (stored). By default, data is retained for any metric used in the profile's behaviors, but it is also retained for any metric specified here.
    #[serde(rename = "AdditionalMetricsToRetainV2")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,
    /// A set of target ARNs that the security profile is attached to.
    #[serde(rename = "TargetArns")]
    pub target_arns: Option<Vec<String>>,
    /// Specifies the destinations to which alerts are sent.
    #[serde(rename = "AlertTargets")]
    pub alert_targets: Option<()>,
    /// A unique identifier for the security profile.
    #[serde(rename = "SecurityProfileName")]
    pub security_profile_name: Option<String>,
    /// Metadata that can be used to manage the security profile.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Specifies the behaviors that, when violated by a device (thing), cause an alert.
    #[serde(rename = "Behaviors")]
    pub behaviors: Option<Vec<Behavior>>,

}


#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[serde(rename = "Operator")]
    pub operator: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BehaviorCriteria {
    #[serde(rename = "MlDetectionConfig")]
    pub ml_detection_config: Option<MachineLearningDetectionConfig>,
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<usize>,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<MetricValue>,
    #[serde(rename = "ConsecutiveDatapointsToAlarm")]
    pub consecutive_datapoints_to_alarm: Option<usize>,
    #[serde(rename = "StatisticalThreshold")]
    pub statistical_threshold: Option<StatisticalThreshold>,
    #[serde(rename = "ConsecutiveDatapointsToClear")]
    pub consecutive_datapoints_to_clear: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricValue {
    #[serde(rename = "Number")]
    pub number: Option<f64>,
    #[serde(rename = "Numbers")]
    pub numbers: Option<Vec<f64>>,
    #[serde(rename = "Cidrs")]
    pub cidrs: Option<Vec<String>>,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<usize>>,
    #[serde(rename = "Strings")]
    pub strings: Option<Vec<String>>,
    #[serde(rename = "Count")]
    pub count: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Behavior {
    #[serde(rename = "Criteria")]
    pub criteria: Option<BehaviorCriteria>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Metric")]
    pub metric: Option<String>,
    #[serde(rename = "SuppressAlerts")]
    pub suppress_alerts: Option<bool>,
    #[serde(rename = "MetricDimension")]
    pub metric_dimension: Option<MetricDimension>,

}

#[derive(serde::Serialize, Default)]
pub struct MachineLearningDetectionConfig {
    #[serde(rename = "ConfidenceLevel")]
    pub confidence_level: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricToRetain {
    #[serde(rename = "Metric")]
    pub metric: String,
    #[serde(rename = "MetricDimension")]
    pub metric_dimension: Option<MetricDimension>,

}

#[derive(serde::Serialize, Default)]
pub struct StatisticalThreshold {
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AlertTarget {
    #[serde(rename = "AlertTargetArn")]
    pub alert_target_arn: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_thing {

#[derive(serde::Serialize, Default)]
pub struct CfnThing {
    /// No documentation provided by AWS
    #[serde(rename = "ThingName")]
    pub thing_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AttributePayload")]
    pub attribute_payload: Option<AttributePayload>,

}


#[derive(serde::Serialize, Default)]
pub struct AttributePayload {
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,

}


}

pub mod cfn_thing_group {

#[derive(serde::Serialize, Default)]
pub struct CfnThingGroup {
    /// No documentation provided by AWS
    #[serde(rename = "ThingGroupName")]
    pub thing_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ParentGroupName")]
    pub parent_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ThingGroupProperties")]
    pub thing_group_properties: Option<()>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "QueryString")]
    pub query_string: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct AttributePayload {
    #[serde(rename = "Attributes")]
    pub attributes: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_thing_principal_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnThingPrincipalAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "ThingName")]
    pub thing_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: String,

}



}

pub mod cfn_thing_type {

#[derive(serde::Serialize, Default)]
pub struct CfnThingType {
    /// No documentation provided by AWS
    #[serde(rename = "ThingTypeName")]
    pub thing_type_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ThingTypeProperties")]
    pub thing_type_properties: Option<()>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DeprecateThingType")]
    pub deprecate_thing_type: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_topic_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnTopicRule {
    /// No documentation provided by AWS
    #[serde(rename = "TopicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,
    /// No documentation provided by AWS
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct IotSiteWiseAction {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "PutAssetPropertyValueEntries")]
    pub put_asset_property_value_entries: Vec<PutAssetPropertyValueEntry>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyVariant {
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<String>,
    #[serde(rename = "IntegerValue")]
    pub integer_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RepublishActionHeaders {
    #[serde(rename = "CorrelationData")]
    pub correlation_data: Option<String>,
    #[serde(rename = "ResponseTopic")]
    pub response_topic: Option<String>,
    #[serde(rename = "PayloadFormatIndicator")]
    pub payload_format_indicator: Option<String>,
    #[serde(rename = "MessageExpiry")]
    pub message_expiry: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    #[serde(rename = "UserProperties")]
    pub user_properties: Option<UserProperties>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpActionHeader {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyTimestamp {
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: String,
    #[serde(rename = "OffsetInNanos")]
    pub offset_in_nanos: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticsearchAction {
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Index")]
    pub index: String,

}

#[derive(serde::Serialize, Default)]
pub struct PutItemInput {
    #[serde(rename = "TableName")]
    pub table_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TimestreamDimensionsList {

}

#[derive(serde::Serialize, Default)]
pub struct TimestreamTimestamp {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Unit")]
    pub unit: String,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBAction {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: String,
    #[serde(rename = "RangeKeyType")]
    pub range_key_type: Option<String>,
    #[serde(rename = "HashKeyType")]
    pub hash_key_type: Option<String>,
    #[serde(rename = "RangeKeyField")]
    pub range_key_field: Option<String>,
    #[serde(rename = "PayloadField")]
    pub payload_field: Option<String>,
    #[serde(rename = "RangeKeyValue")]
    pub range_key_value: Option<String>,
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: String,

}

#[derive(serde::Serialize, Default)]
pub struct LambdaAction {
    #[serde(rename = "FunctionArn")]
    pub function_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct S3Action {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "CannedAcl")]
    pub canned_acl: Option<CannedAccessControlList>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyValue {
    #[serde(rename = "Quality")]
    pub quality: Option<String>,
    #[serde(rename = "Value")]
    pub value: AssetPropertyVariant,
    #[serde(rename = "Timestamp")]
    pub timestamp: AssetPropertyTimestamp,

}

#[derive(serde::Serialize, Default)]
pub struct TopicRulePayload {
    #[serde(rename = "RuleDisabled")]
    pub rule_disabled: Option<bool>,
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "AwsIotSqlVersion")]
    pub aws_iot_sql_version: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Sql")]
    pub sql: String,
    #[serde(rename = "ErrorAction")]
    pub error_action: Option<Action>,

}

#[derive(serde::Serialize, Default)]
pub struct CloudwatchAlarmAction {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "StateReason")]
    pub state_reason: String,
    #[serde(rename = "StateValue")]
    pub state_value: String,
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SqsAction {
    #[serde(rename = "QueueUrl")]
    pub queue_url: String,
    #[serde(rename = "UseBase64")]
    pub use_base64: Option<bool>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct CloudwatchLogsAction {
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "BatchMode")]
    pub batch_mode: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct KafkaAction {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,
    #[serde(rename = "Partition")]
    pub partition: Option<String>,
    #[serde(rename = "ClientProperties")]
    pub client_properties: (),
    #[serde(rename = "Topic")]
    pub topic: String,

}

#[derive(serde::Serialize, Default)]
pub struct CloudwatchMetricAction {
    #[serde(rename = "MetricValue")]
    pub metric_value: String,
    #[serde(rename = "MetricNamespace")]
    pub metric_namespace: String,
    #[serde(rename = "MetricTimestamp")]
    pub metric_timestamp: Option<String>,
    #[serde(rename = "MetricUnit")]
    pub metric_unit: String,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisAction {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct OpenSearchAction {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: String,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Index")]
    pub index: String,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Timestamp {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SigV4Authorization {
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "SigningRegion")]
    pub signing_region: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct FirehoseAction {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Separator")]
    pub separator: Option<String>,
    #[serde(rename = "BatchMode")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TimestreamDimension {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct HttpAction {
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpActionHeader>>,
    #[serde(rename = "ConfirmationUrl")]
    pub confirmation_url: Option<String>,
    #[serde(rename = "Auth")]
    pub auth: Option<HttpAuthorization>,

}

#[derive(serde::Serialize, Default)]
pub struct IotEventsAction {
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "InputName")]
    pub input_name: String,
    #[serde(rename = "MessageId")]
    pub message_id: Option<String>,
    #[serde(rename = "BatchMode")]
    pub batch_mode: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PutAssetPropertyValueEntry {
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,
    #[serde(rename = "EntryId")]
    pub entry_id: Option<String>,
    #[serde(rename = "PropertyValues")]
    pub property_values: Vec<AssetPropertyValue>,
    #[serde(rename = "AssetId")]
    pub asset_id: Option<String>,
    #[serde(rename = "PropertyId")]
    pub property_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct RepublishAction {
    #[serde(rename = "Topic")]
    pub topic: String,
    #[serde(rename = "Qos")]
    pub qos: Option<usize>,
    #[serde(rename = "Headers")]
    pub headers: Option<RepublishActionHeaders>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct SnsAction {
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
    #[serde(rename = "MessageFormat")]
    pub message_format: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBv2Action {
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "PutItem")]
    pub put_item: Option<PutItemInput>,

}

#[derive(serde::Serialize, Default)]
pub struct UserProperties {

}
pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "Location")]
    pub location: Option<LocationAction>,
    #[serde(rename = "Kinesis")]
    pub kinesis: Option<KinesisAction>,
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDBAction>,
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEventsAction>,
    #[serde(rename = "IotAnalytics")]
    pub iot_analytics: Option<IotAnalyticsAction>,
    #[serde(rename = "Http")]
    pub http: Option<HttpAction>,
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWiseAction>,
    #[serde(rename = "CloudwatchMetric")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,
    #[serde(rename = "OpenSearch")]
    pub open_search: Option<OpenSearchAction>,
    #[serde(rename = "Kafka")]
    pub kafka: Option<KafkaAction>,
    #[serde(rename = "Republish")]
    pub republish: Option<RepublishAction>,
    #[serde(rename = "StepFunctions")]
    pub step_functions: Option<StepFunctionsAction>,
    #[serde(rename = "S3")]
    pub s3: Option<S3Action>,
    #[serde(rename = "CloudwatchLogs")]
    pub cloudwatch_logs: Option<CloudwatchLogsAction>,
    #[serde(rename = "Firehose")]
    pub firehose: Option<FirehoseAction>,
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2Action>,
    #[serde(rename = "Sns")]
    pub sns: Option<SnsAction>,
    #[serde(rename = "Elasticsearch")]
    pub elasticsearch: Option<ElasticsearchAction>,
    #[serde(rename = "CloudwatchAlarm")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,
    #[serde(rename = "Sqs")]
    pub sqs: Option<SqsAction>,
    #[serde(rename = "Lambda")]
    pub lambda: Option<LambdaAction>,
    #[serde(rename = "Timestream")]
    pub timestream: Option<TimestreamAction>,

}

#[derive(serde::Serialize, Default)]
pub struct IotAnalyticsAction {
    #[serde(rename = "BatchMode")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "ChannelName")]
    pub channel_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct UserProperty {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct StepFunctionsAction {
    #[serde(rename = "ExecutionNamePrefix")]
    pub execution_name_prefix: Option<String>,
    #[serde(rename = "StateMachineName")]
    pub state_machine_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}
pub type CannedAccessControlList = String;
#[derive(serde::Serialize, Default)]
pub struct HttpAuthorization {
    #[serde(rename = "Sigv4")]
    pub sigv4: Option<SigV4Authorization>,

}

#[derive(serde::Serialize, Default)]
pub struct TimestreamAction {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: TimestreamDimensionsList,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<TimestreamTimestamp>,

}

#[derive(serde::Serialize, Default)]
pub struct LocationAction {
    #[serde(rename = "TrackerName")]
    pub tracker_name: String,
    #[serde(rename = "Longitude")]
    pub longitude: String,
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<Timestamp>,
    #[serde(rename = "Latitude")]
    pub latitude: String,

}


}

pub mod cfn_topic_rule_destination {

#[derive(serde::Serialize, Default)]
pub struct CfnTopicRuleDestination {
    /// The status of the TopicRuleDestination.
    #[serde(rename = "Status")]
    pub status: Option<TopicRuleDestinationStatus>,
    /// HTTP URL destination properties.
    #[serde(rename = "HttpUrlProperties")]
    pub http_url_properties: Option<HttpUrlDestinationSummary>,
    /// VPC destination properties.
    #[serde(rename = "VpcProperties")]
    pub vpc_properties: Option<VpcDestinationProperties>,

}

pub type TopicRuleDestinationStatus = String;
#[derive(serde::Serialize, Default)]
pub struct VpcDestinationProperties {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpUrlDestinationSummary {
    #[serde(rename = "ConfirmationUrl")]
    pub confirmation_url: Option<String>,

}


}
