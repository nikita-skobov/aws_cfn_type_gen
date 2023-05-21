

/// Contains an optional backup plan display name and an array of BackupRule     objects, each of which specifies a backup rule. Each rule in a backup plan is a separate     scheduled task and can back up a different selection of AWS     resources.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnBackupPlan {


    /// 
    /// To help organize your resources, you can assign your own metadata to the resources that     you create. Each tag is a key-value pair. The specified tags are assigned to all backups     created with this plan.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPlanTags")]
    pub backup_plan_tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// Uniquely identifies the backup plan to be associated with the selection of     resources.
    /// 
    /// Required: Yes
    ///
    /// Type: BackupPlanResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPlan")]
    pub backup_plan: BackupPlanResourceType,

}


/// Specifies an object containing resource type and backup options. This is only supported     for Windows VSS backups.
#[derive(Default, serde::Serialize)]
pub struct AdvancedBackupSettingResourceType {


    /// 
    /// The backup option for the resource. Each option is a key-value pair. This option is only     available for Windows VSS backup jobs.
    /// 
    /// Valid values:
    /// 
    /// Set to "WindowsVSS":"enabled" to enable the WindowsVSS backup     option and create a Windows VSS backup.
    /// 
    /// Set to "WindowsVSS":"disabled" to create a regular backup. The       WindowsVSS option is not enabled by default.
    /// 
    /// If you specify an invalid option, you get an InvalidParameterValueException     exception.
    /// 
    /// For more information about Windows VSS backups, see Creating a VSS-Enabled Windows       Backup.
    /// 
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupOptions")]
    pub backup_options: serde_json::Value,


    /// 
    /// The name of a resource type. The only supported resource type is EC2.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

}


/// Specifies an object containing an array of Transition objects that     determine how long in days before a recovery point transitions to cold storage or is     deleted.
#[derive(Default, serde::Serialize)]
pub struct LifecycleResourceType {


    /// 
    /// Specifies the number of days after creation that a recovery point is deleted. Must be     greater than MoveToColdStorageAfterDays.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteAfterDays")]
    pub delete_after_days: Option<f64>,


    /// 
    /// Specifies the number of days after creation that a recovery point is moved to cold     storage.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MoveToColdStorageAfterDays")]
    pub move_to_cold_storage_after_days: Option<f64>,

}


/// Copies backups created by a backup rule to another vault.
#[derive(Default, serde::Serialize)]
pub struct CopyActionResourceType {


    /// 
    /// An Amazon Resource Name (ARN) that uniquely identifies the destination backup vault for     the copied backup. For example,       arn:aws:backup:us-east-1:123456789012:vault:aBackupVault.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationBackupVaultArn")]
    pub destination_backup_vault_arn: String,


    /// 
    /// Defines when a protected resource is transitioned to cold storage and when it expires.       AWS Backup transitions and expires backups automatically according to the     lifecycle that you define. If you do not specify a lifecycle, AWS Backup applies     the lifecycle policy of the source backup to the destination backup.
    /// 
    /// Backups transitioned to cold storage must be stored in cold storage for a minimum of 90     days.
    /// 
    /// Required: No
    ///
    /// Type: LifecycleResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lifecycle")]
    pub lifecycle: Option<LifecycleResourceType>,

}


/// Specifies an object containing properties used to schedule a task to back up a selection     of resources.
#[derive(Default, serde::Serialize)]
pub struct BackupRuleResourceType {


    /// 
    /// The lifecycle defines when a protected resource is transitioned to cold storage and when     it expires. AWS Backup transitions and expires backups automatically according to     the lifecycle that you define.
    /// 
    /// Required: No
    ///
    /// Type: LifecycleResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lifecycle")]
    pub lifecycle: Option<LifecycleResourceType>,


    /// 
    /// Enables continuous backup and point-in-time restores (PITR).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableContinuousBackup")]
    pub enable_continuous_backup: Option<bool>,


    /// 
    /// The name of a logical container where backups are stored. Backup vaults are identified     by names that are unique to the account used to create them and the AWS Region where they are created. They consist of letters, numbers, and     hyphens.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetBackupVault")]
    pub target_backup_vault: String,


    /// 
    /// A value in minutes after a backup job is successfully started before it must be     completed or it is canceled by AWS Backup.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompletionWindowMinutes")]
    pub completion_window_minutes: Option<f64>,


    /// 
    /// A CRON expression specifying when AWS Backup initiates a backup job.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,


    /// 
    /// An array of CopyAction objects, which contains the details of the copy operation.
    /// 
    /// Required: No
    ///
    /// Type: List of CopyActionResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyActions")]
    pub copy_actions: Option<Vec<CopyActionResourceType>>,


    /// 
    /// A display name for a backup rule.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleName")]
    pub rule_name: String,


    /// 
    /// To help organize your resources, you can assign your own metadata to the resources that     you create. Each tag is a key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecoveryPointTags")]
    pub recovery_point_tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// An optional value that specifies a period of time in minutes after a backup is scheduled     before a job is canceled if it doesn't start successfully.
    /// 
    /// If this value is included, it must be at least 60 minutes to avoid errors.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartWindowMinutes")]
    pub start_window_minutes: Option<f64>,

}


/// Specifies an object containing properties used to create a backup plan.
#[derive(Default, serde::Serialize)]
pub struct BackupPlanResourceType {


    /// 
    /// An array of BackupRule objects, each of which specifies a scheduled task     that is used to back up a selection of resources.
    /// 
    /// Required: Yes
    ///
    /// Type: List of BackupRuleResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPlanRule")]
    pub backup_plan_rule: Vec<BackupRuleResourceType>,


    /// 
    /// A list of backup options for each resource type.
    /// 
    /// Required: No
    ///
    /// Type: List of AdvancedBackupSettingResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedBackupSettings")]
    pub advanced_backup_settings: Option<Vec<AdvancedBackupSettingResourceType>>,


    /// 
    /// The display name of a backup plan.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPlanName")]
    pub backup_plan_name: String,

}