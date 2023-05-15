
pub mod cfn_lifecycle_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnLifecyclePolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyDetails")]
    pub policy_details: Option<PolicyDetails>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct EventParameters {
    #[serde(rename = "DescriptionRegex")]
    pub description_regex: Option<String>,
    #[serde(rename = "SnapshotOwner")]
    pub snapshot_owner: Vec<String>,
    #[serde(rename = "EventType")]
    pub event_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveRetainRule {
    #[serde(rename = "RetentionArchiveTier")]
    pub retention_archive_tier: RetentionArchiveTier,

}

#[derive(serde::Serialize, Default)]
pub struct Parameters {
    #[serde(rename = "ExcludeDataVolumeTags")]
    pub exclude_data_volume_tags: Option<Vec<Tag>>,
    #[serde(rename = "ExcludeBootVolume")]
    pub exclude_boot_volume: Option<bool>,
    #[serde(rename = "NoReboot")]
    pub no_reboot: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionConfiguration {
    #[serde(rename = "CmkArn")]
    pub cmk_arn: Option<String>,
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,

}

#[derive(serde::Serialize, Default)]
pub struct Schedule {
    #[serde(rename = "DeprecateRule")]
    pub deprecate_rule: Option<DeprecateRule>,
    #[serde(rename = "VariableTags")]
    pub variable_tags: Option<Vec<Tag>>,
    #[serde(rename = "CopyTags")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "FastRestoreRule")]
    pub fast_restore_rule: Option<FastRestoreRule>,
    #[serde(rename = "CreateRule")]
    pub create_rule: Option<CreateRule>,
    #[serde(rename = "ShareRules")]
    pub share_rules: Option<Vec<ShareRule>>,
    #[serde(rename = "CrossRegionCopyRules")]
    pub cross_region_copy_rules: Option<Vec<CrossRegionCopyRule>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ArchiveRule")]
    pub archive_rule: Option<ArchiveRule>,
    #[serde(rename = "TagsToAdd")]
    pub tags_to_add: Option<Vec<Tag>>,
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<RetainRule>,

}

#[derive(serde::Serialize, Default)]
pub struct CrossRegionCopyAction {
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,

}

#[derive(serde::Serialize, Default)]
pub struct PolicyDetails {
    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "Schedules")]
    pub schedules: Option<Vec<Schedule>>,
    #[serde(rename = "ResourceLocations")]
    pub resource_locations: Option<Vec<String>>,
    #[serde(rename = "EventSource")]
    pub event_source: Option<EventSource>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<String>,
    #[serde(rename = "TargetTags")]
    pub target_tags: Option<Vec<Tag>>,

}

#[derive(serde::Serialize, Default)]
pub struct CrossRegionCopyDeprecateRule {
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: String,
    #[serde(rename = "Interval")]
    pub interval: usize,

}

#[derive(serde::Serialize, Default)]
pub struct CrossRegionCopyRule {
    #[serde(rename = "CmkArn")]
    pub cmk_arn: Option<String>,
    #[serde(rename = "CopyTags")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,
    #[serde(rename = "TargetRegion")]
    pub target_region: Option<String>,
    #[serde(rename = "DeprecateRule")]
    pub deprecate_rule: Option<CrossRegionCopyDeprecateRule>,
    #[serde(rename = "Target")]
    pub target: Option<String>,
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,

}

#[derive(serde::Serialize, Default)]
pub struct CreateRule {
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,
    #[serde(rename = "CronExpression")]
    pub cron_expression: Option<String>,
    #[serde(rename = "Times")]
    pub times: Option<Vec<String>>,
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Location")]
    pub location: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FastRestoreRule {
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RetainRule {
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct EventSource {
    #[serde(rename = "Parameters")]
    pub parameters: Option<EventParameters>,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeprecateRule {
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RetentionArchiveTier {
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    #[serde(rename = "Interval")]
    pub interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct CrossRegionCopyRetainRule {
    #[serde(rename = "Interval")]
    pub interval: usize,
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: String,

}

#[derive(serde::Serialize, Default)]
pub struct ArchiveRule {
    #[serde(rename = "RetainRule")]
    pub retain_rule: ArchiveRetainRule,

}

#[derive(serde::Serialize, Default)]
pub struct ShareRule {
    #[serde(rename = "UnshareIntervalUnit")]
    pub unshare_interval_unit: Option<String>,
    #[serde(rename = "TargetAccounts")]
    pub target_accounts: Option<Vec<String>>,
    #[serde(rename = "UnshareInterval")]
    pub unshare_interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CrossRegionCopy")]
    pub cross_region_copy: Vec<CrossRegionCopyAction>,

}


}
