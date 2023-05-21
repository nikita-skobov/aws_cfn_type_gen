/// Contains an optional backup plan display name and an array of BackupRule     objects, each of which specifies a backup rule. Each rule in a backup plan is a separate     scheduled task and can back up a different selection of AWS     resources.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnBackupPlan {
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

    ///
    /// To help organize your resources, you can assign your own metadata to the resources that     you create. Each tag is a key-value pair. The specified tags are assigned to all backups     created with this plan.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupPlanTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_tags: Option<std::collections::HashMap<String, String>>,
}

impl cfn_resources::CfnResource for CfnBackupPlan {
    fn type_string(&self) -> &'static str {
        "AWS::Backup::BackupPlan"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.backup_plan.validate()?;

        Ok(())
    }
}

/// Specifies an object containing resource type and backup options. This is only supported     for Windows VSS backups.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for AdvancedBackupSettingResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies an object containing properties used to create a backup plan.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BackupPlanResourceType {
    ///
    /// A list of backup options for each resource type.
    ///
    /// Required: No
    ///
    /// Type: List of AdvancedBackupSettingResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
}

impl cfn_resources::CfnResource for BackupPlanResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies an object containing properties used to schedule a task to back up a selection     of resources.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BackupRuleResourceType {
    ///
    /// A value in minutes after a backup job is successfully started before it must be     completed or it is canceled by AWS Backup.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<f64>,

    ///
    /// An array of CopyAction objects, which contains the details of the copy operation.
    ///
    /// Required: No
    ///
    /// Type: List of CopyActionResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<CopyActionResourceType>>,

    ///
    /// Enables continuous backup and point-in-time restores (PITR).
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableContinuousBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_continuous_backup: Option<bool>,

    ///
    /// The lifecycle defines when a protected resource is transitioned to cold storage and when     it expires. AWS Backup transitions and expires backups automatically according to     the lifecycle that you define.
    ///
    /// Required: No
    ///
    /// Type: LifecycleResourceType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lifecycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<LifecycleResourceType>,

    ///
    /// To help organize your resources, you can assign your own metadata to the resources that     you create. Each tag is a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecoveryPointTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_tags: Option<std::collections::HashMap<String, String>>,

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
    /// A CRON expression specifying when AWS Backup initiates a backup job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<f64>,

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
}

impl cfn_resources::CfnResource for BackupRuleResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lifecycle
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Copies backups created by a backup rule to another vault.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<LifecycleResourceType>,
}

impl cfn_resources::CfnResource for CopyActionResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lifecycle
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies an object containing an array of Transition objects that     determine how long in days before a recovery point transitions to cold storage or is     deleted.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_after_days: Option<f64>,
}

impl cfn_resources::CfnResource for LifecycleResourceType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
