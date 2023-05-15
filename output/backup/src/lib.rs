
pub mod cfn_backup_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnBackupPlan {
    /// No documentation provided by AWS
    #[serde(rename = "BackupPlan")]
    pub backup_plan: BackupPlanResourceType,
    /// No documentation provided by AWS
    #[serde(rename = "BackupPlanTags")]
    pub backup_plan_tags: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct BackupPlanResourceType {
    #[serde(rename = "BackupPlanName")]
    pub backup_plan_name: String,
    #[serde(rename = "AdvancedBackupSettings")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSettingResourceType>>,
    #[serde(rename = "BackupPlanRule")]
    pub backup_plan_rule: Vec<BackupRuleResourceType>,

}

#[derive(serde::Serialize, Default)]
pub struct AdvancedBackupSettingResourceType {
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "BackupOptions")]
    pub backup_options: (),

}

#[derive(serde::Serialize, Default)]
pub struct CopyActionResourceType {
    #[serde(rename = "Lifecycle")]
    pub lifecycle: Option<LifecycleResourceType>,
    #[serde(rename = "DestinationBackupVaultArn")]
    pub destination_backup_vault_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct LifecycleResourceType {
    #[serde(rename = "MoveToColdStorageAfterDays")]
    pub move_to_cold_storage_after_days: Option<f64>,
    #[serde(rename = "DeleteAfterDays")]
    pub delete_after_days: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct BackupRuleResourceType {
    #[serde(rename = "EnableContinuousBackup")]
    pub enable_continuous_backup: Option<bool>,
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    #[serde(rename = "CopyActions")]
    pub copy_actions: Option<Vec<CopyActionResourceType>>,
    #[serde(rename = "Lifecycle")]
    pub lifecycle: Option<LifecycleResourceType>,
    #[serde(rename = "CompletionWindowMinutes")]
    pub completion_window_minutes: Option<f64>,
    #[serde(rename = "StartWindowMinutes")]
    pub start_window_minutes: Option<f64>,
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "RecoveryPointTags")]
    pub recovery_point_tags: Option<()>,
    #[serde(rename = "TargetBackupVault")]
    pub target_backup_vault: String,

}


}

pub mod cfn_backup_selection {

#[derive(serde::Serialize, Default)]
pub struct CfnBackupSelection {
    /// No documentation provided by AWS
    #[serde(rename = "BackupPlanId")]
    pub backup_plan_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "BackupSelection")]
    pub backup_selection: BackupSelectionResourceType,

}


#[derive(serde::Serialize, Default)]
pub struct ConditionParameter {
    #[serde(rename = "ConditionKey")]
    pub condition_key: Option<String>,
    #[serde(rename = "ConditionValue")]
    pub condition_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BackupSelectionResourceType {
    #[serde(rename = "Conditions")]
    pub conditions: Option<()>,
    #[serde(rename = "ListOfTags")]
    pub list_of_tags: Option<Vec<ConditionResourceType>>,
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "SelectionName")]
    pub selection_name: String,
    #[serde(rename = "NotResources")]
    pub not_resources: Option<Vec<String>>,
    #[serde(rename = "IamRoleArn")]
    pub iam_role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConditionResourceType {
    #[serde(rename = "ConditionValue")]
    pub condition_value: String,
    #[serde(rename = "ConditionType")]
    pub condition_type: String,
    #[serde(rename = "ConditionKey")]
    pub condition_key: String,

}


}

pub mod cfn_backup_vault {

#[derive(serde::Serialize, Default)]
pub struct CfnBackupVault {
    /// No documentation provided by AWS
    #[serde(rename = "Notifications")]
    pub notifications: Option<NotificationObjectType>,
    /// No documentation provided by AWS
    #[serde(rename = "LockConfiguration")]
    pub lock_configuration: Option<LockConfigurationType>,
    /// No documentation provided by AWS
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: BackupVaultNamePattern,
    /// No documentation provided by AWS
    #[serde(rename = "BackupVaultTags")]
    pub backup_vault_tags: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessPolicy")]
    pub access_policy: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "EncryptionKeyArn")]
    pub encryption_key_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NotificationObjectType {
    #[serde(rename = "SNSTopicArn")]
    pub snstopic_arn: String,
    #[serde(rename = "BackupVaultEvents")]
    pub backup_vault_events: Vec<String>,

}
pub type BackupVaultNamePattern = String;
#[derive(serde::Serialize, Default)]
pub struct LockConfigurationType {
    #[serde(rename = "ChangeableForDays")]
    pub changeable_for_days: Option<usize>,
    #[serde(rename = "MaxRetentionDays")]
    pub max_retention_days: Option<usize>,
    #[serde(rename = "MinRetentionDays")]
    pub min_retention_days: usize,

}


}

pub mod cfn_framework {

#[derive(serde::Serialize, Default)]
pub struct CfnFramework {
    /// The unique name of a framework. This name is between 1 and 256 characters, starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and underscores (_).
    #[serde(rename = "FrameworkName")]
    pub framework_name: Option<String>,
    /// Contains detailed information about all of the controls of a framework. Each framework must contain at least one control.
    #[serde(rename = "FrameworkControls")]
    pub framework_controls: Vec<FrameworkControl>,
    /// An optional description of the framework with a maximum 1,024 characters.
    #[serde(rename = "FrameworkDescription")]
    pub framework_description: Option<String>,
    /// Metadata that you can assign to help organize the frameworks that you create. Each tag is a key-value pair.
    #[serde(rename = "FrameworkTags")]
    pub framework_tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct ControlInputParameter {
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FrameworkControl {
    #[serde(rename = "ControlInputParameters")]
    pub control_input_parameters: Option<Vec<ControlInputParameter>>,
    #[serde(rename = "ControlScope")]
    pub control_scope: Option<()>,
    #[serde(rename = "ControlName")]
    pub control_name: String,

}


}

pub mod cfn_report_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnReportPlan {
    /// A structure that contains information about where and how to deliver your reports, specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your reports.
    #[serde(rename = "ReportDeliveryChannel")]
    pub report_delivery_channel: (),
    /// Identifies the report template for the report. Reports are built using a report template.
    #[serde(rename = "ReportSetting")]
    pub report_setting: (),
    /// The unique name of the report plan. The name must be between 1 and 256 characters, starting with a letter, and consisting of letters (a-z, A-Z), numbers (0-9), and underscores (_).
    #[serde(rename = "ReportPlanName")]
    pub report_plan_name: Option<String>,
    /// Metadata that you can assign to help organize the report plans that you create. Each tag is a key-value pair.
    #[serde(rename = "ReportPlanTags")]
    pub report_plan_tags: Option<Vec<Tag>>,
    /// An optional description of the report plan with a maximum of 1,024 characters.
    #[serde(rename = "ReportPlanDescription")]
    pub report_plan_description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}
