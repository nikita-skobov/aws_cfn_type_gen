/// Creates a logical container where backups are stored. A CreateBackupVault     request includes a name, optionally one or more resource tags, an encryption key, and a     request ID.
///
/// Do not include sensitive data, such as passport numbers, in the name of a backup     vault.
///
/// For a sample AWS CloudFormation template, see the AWS Backup Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnBackupVault {
    ///
    /// A resource-based policy that is used to manage access permissions on the target backup     vault.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<serde_json::Value>,

    ///
    /// The name of a logical container where backups are stored. Backup vaults are identified     by names that are unique to the account used to create them and the AWS     Region where they are created. They consist of lowercase letters, numbers, and     hyphens.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[a-zA-Z0-9\-\_]{2,50}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupVaultName")]
    pub backup_vault_name: cfn_resources::StrVal,

    ///
    /// Metadata that you can assign to help organize the resources that you create. Each tag is     a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupVaultTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_tags: Option<std::collections::HashMap<String, String>>,

    ///
    /// A server-side encryption key you can specify to encrypt your backups from services    that support full AWS Backup management; for example,    arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab.    If you specify a key, you must specify its ARN, not its alias. If you do not specify a key,    AWS Backup creates a KMS key for you by default.
    ///
    /// To learn which AWS Backup services support full AWS Backup management     and how AWS Backup handles encryption for backups from services that do not yet support     full AWS Backup, see       Encryption for backups in AWS Backup
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// Configuration for AWS Backup Vault     Lock.
    ///
    /// Required: No
    ///
    /// Type: LockConfigurationType
    ///
    /// Update requires: No interruption
    #[serde(rename = "LockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_configuration: Option<LockConfigurationType>,

    ///
    /// The SNS event notifications for the specified backup vault.
    ///
    /// Required: No
    ///
    /// Type: NotificationObjectType
    ///
    /// Update requires: No interruption
    #[serde(rename = "Notifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<NotificationObjectType>,

    #[serde(skip_serializing)]
    pub att_backup_vault_arn: CfnBackupVaultbackupvaultarn,

    #[serde(skip_serializing)]
    pub att_backup_vault_name: CfnBackupVaultbackupvaultname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBackupVaultbackupvaultarn;
impl CfnBackupVaultbackupvaultarn {
    pub fn att_name(&self) -> &'static str {
        r#"BackupVaultArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnBackupVaultbackupvaultname;
impl CfnBackupVaultbackupvaultname {
    pub fn att_name(&self) -> &'static str {
        r#"BackupVaultName"#
    }
}

impl cfn_resources::CfnResource for CfnBackupVault {
    fn type_string(&self) -> &'static str {
        "AWS::Backup::BackupVault"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lock_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.notifications
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The LockConfigurationType property type specifies configuration for AWS Backup Vault Lock.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LockConfigurationType {
    ///
    /// The AWS Backup Vault Lock configuration that specifies the number of days     before the lock date. For example, setting ChangeableForDays to 30 on Jan. 1,     2022 at 8pm UTC will set the lock date to Jan. 31, 2022 at 8pm UTC.
    ///
    /// AWS Backup enforces a 72-hour cooling-off period before Vault Lock takes     effect and becomes immutable. Therefore, you must set ChangeableForDays to 3     or greater.
    ///
    /// Before the lock date, you can delete Vault Lock from the vault using       DeleteBackupVaultLockConfiguration or change the Vault Lock configuration     using PutBackupVaultLockConfiguration. On and after the lock date, the Vault     Lock becomes immutable and cannot be changed or deleted.
    ///
    /// If this parameter is not specified, you can delete Vault Lock from the vault using       DeleteBackupVaultLockConfiguration or change the Vault Lock configuration     using PutBackupVaultLockConfiguration at any time.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChangeableForDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeable_for_days: Option<i64>,

    ///
    /// The AWS Backup Vault Lock configuration that specifies the maximum retention     period that the vault retains its recovery points. This setting can be useful if, for     example, your organization's policies require you to destroy certain data after retaining     it for four years (1460 days).
    ///
    /// If this parameter is not included, Vault Lock does not enforce a maximum retention     period on the recovery points in the vault. If this parameter is included without a value,     Vault Lock will not enforce a maximum retention period.
    ///
    /// If this parameter is specified, any backup or copy job to the vault must have a     lifecycle policy with a retention period equal to or shorter than the maximum retention     period. If the job's retention period is longer than that maximum retention period, then     the vault fails the backup or copy job, and you should either modify your lifecycle     settings or use a different vault. Recovery points already saved in the vault prior to     Vault Lock are not affected.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retention_days: Option<i64>,

    ///
    /// The AWS Backup Vault Lock configuration that specifies the minimum retention     period that the vault retains its recovery points. This setting can be useful if, for     example, your organization's policies require you to retain certain data for at least seven     years (2555 days).
    ///
    /// If this parameter is not specified, Vault Lock will not enforce a minimum retention     period.
    ///
    /// If this parameter is specified, any backup or copy job to the vault must have a     lifecycle policy with a retention period equal to or longer than the minimum retention     period. If the job's retention period is shorter than that minimum retention period, then     the vault fails that backup or copy job, and you should either modify your lifecycle     settings or use a different vault. Recovery points already saved in the vault prior to     Vault Lock are not affected.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinRetentionDays")]
    pub min_retention_days: i64,
}

impl cfn_resources::CfnResource for LockConfigurationType {
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

/// Specifies an object containing SNS event notification properties for the target backup     vault.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationObjectType {
    ///
    /// An array of events that indicate the status of jobs to back up resources to the backup     vault. For valid events, see BackupVaultEvents in the AWS Backup API     Guide.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupVaultEvents")]
    pub backup_vault_events: Vec<String>,

    ///
    /// An ARN that uniquely identifies an Amazon Simple Notification Service (Amazon SNS)     topic; for example, arn:aws:sns:us-west-2:111122223333:MyTopic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SNSTopicArn")]
    pub snstopic_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NotificationObjectType {
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
