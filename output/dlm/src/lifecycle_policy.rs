

/// Specifies a lifecycle policy, which is used to automate operations on Amazon EBS resources.
///
/// The properties are required when you add a lifecycle policy and optional when you update a lifecycle policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLifecyclePolicy {


    /// 
    /// A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are 			supported.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Pattern: [0-9A-Za-z _-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The tags to apply to the lifecycle policy during creation.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The configuration details of the lifecycle policy.
    /// 
    /// Required: Conditional
    ///
    /// Type: PolicyDetails
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDetails")]
    pub policy_details: Option<PolicyDetails>,


    /// 
    /// The activation state of the lifecycle policy.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED | ERROR
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<LifecyclePolicyStateEnum>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by 			the lifecycle policy.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws(-[a-z]{1,3}){0,2}:iam::\d+:role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LifecyclePolicyStateEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

}

impl Default for LifecyclePolicyStateEnum {
    fn default() -> Self {
        LifecyclePolicyStateEnum::Disabled
    }
}


impl cfn_resources::CfnResource for CfnLifecyclePolicy {
    fn type_string() -> &'static str {
        "AWS::DLM::LifecyclePolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}




/// [Event-based policies only] Specifies an event that activates an event-based policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventSource {


    /// 
    /// The source of the event. Currently only managed CloudWatch Events rules are supported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MANAGED_CWE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: EventSourceTypeEnum,


    /// 
    /// Information about the event.
    /// 
    /// Required: No
    ///
    /// Type: EventParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<EventParameters>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EventSourceTypeEnum {

    /// MANAGED_CWE
    #[serde(rename = "MANAGED_CWE")]
    Managedcwe,

}

impl Default for EventSourceTypeEnum {
    fn default() -> Self {
        EventSourceTypeEnum::Managedcwe
    }
}



/// [Event-based policies only] Specifies a cross-Region copy action for event-based policies.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CrossRegionCopyAction {


    /// 
    /// The target Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[\w:\-\/\*]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: String,


    /// 
    /// The encryption settings for the copied snapshot.
    /// 
    /// Required: Yes
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,


    /// 
    /// Specifies a retention rule for cross-Region snapshot copies created by snapshot or 			event-based policies, or cross-Region AMI copies created by AMI policies. After the 			retention period expires, the cross-Region copy is deleted.
    /// 
    /// Required: No
    ///
    /// Type: CrossRegionCopyRetainRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,

}




/// [Snapshot and AMI policies only] Specifies when the policy should create snapshots or AMIs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateRule {


    /// 
    /// [Snapshot policies only] Specifies the destination for snapshots created by the policy. To create 			snapshots in the same Region as the source resource, specify CLOUD. To create 			snapshots on the same Outpost as the source resource, specify OUTPOST_LOCAL. 			If you omit this parameter, CLOUD is used by default.
    /// 
    /// If the policy targets resources in an AWS Region, then you must create 			snapshots in the same Region as the source resource. If the policy targets resources on an 			Outpost, then you can create snapshots on the same Outpost as the source resource, or in 			the Region of that Outpost.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CLOUD | OUTPOST_LOCAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<CreateRuleLocationEnum>,


    /// 
    /// The schedule, as a Cron expression. The schedule interval must be between 1 hour and 1 			year. For more information, see Cron 				expressions in the Amazon CloudWatch User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 17
    ///
    /// Maximum: 106
    ///
    /// Pattern: cron\([^\n]{11,100}\)
    ///
    /// Update requires: No interruption
    #[serde(rename = "CronExpression")]
    pub cron_expression: Option<String>,


    /// 
    /// The time, in UTC, to start the operation. The supported format is hh:mm.
    /// 
    /// The operation occurs within a one-hour window following the specified time. If you do 			not specify a time, Amazon Data Lifecycle Manager selects a time within the next 24 hours.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Times")]
    pub times: Option<Vec<String>>,


    /// 
    /// The interval unit.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HOURS
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<CreateRuleIntervalUnitEnum>,


    /// 
    /// The interval between snapshots. The supported values are 1, 2, 3, 4, 6, 8, 12, and 24.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CreateRuleIntervalUnitEnum {

    /// HOURS
    #[serde(rename = "HOURS")]
    Hours,

}

impl Default for CreateRuleIntervalUnitEnum {
    fn default() -> Self {
        CreateRuleIntervalUnitEnum::Hours
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum CreateRuleLocationEnum {

    /// CLOUD
    #[serde(rename = "CLOUD")]
    Cloud,

    /// OUTPOST_LOCAL
    #[serde(rename = "OUTPOST_LOCAL")]
    Outpostlocal,

}

impl Default for CreateRuleLocationEnum {
    fn default() -> Self {
        CreateRuleLocationEnum::Cloud
    }
}



/// [Snapshot and AMI policies only] Specifies optional parameters for snapshot and AMI policies. The 			set of valid parameters depends on the combination of policy type and target resource 			type.
///
/// If you choose to exclude boot volumes and you specify tags that consequently exclude 			all of the additional data volumes attached to an instance, then Amazon Data Lifecycle Manager will not create 			any snapshots for the affected instance, and it will emit a SnapshotsCreateFailed 			Amazon CloudWatch metric. For more information, see Monitor your policies 				using Amazon CloudWatch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Parameters {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeDataVolumeTags")]
    pub exclude_data_volume_tags: Option<Vec<Tag>>,


    /// 
    /// [AMI policies only] Indicates whether targeted instances are rebooted when the lifecycle policy 			runs. true indicates that targeted instances are not rebooted when the policy 			runs. false indicates that target instances are rebooted when the policy runs. 			The default is true (instances are not rebooted).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NoReboot")]
    pub no_reboot: Option<bool>,


    /// 
    /// [Snapshot policies that target instances only] Indicates whether to exclude the root volume from multi-volume 			snapshot sets. The default is false. If you specify true, 			then the root volumes attached to targeted instances will be excluded from the multi-volume 			snapshot sets created by the policy.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeBootVolume")]
    pub exclude_boot_volume: Option<bool>,

}




/// [Snapshot policies only] Specifies a rule for sharing snapshots across AWS accounts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShareRule {


    /// 
    /// The period after which snapshots that are shared with other AWS accounts are automatically unshared.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnshareInterval")]
    pub unshare_interval: Option<i64>,


    /// 
    /// The IDs of the AWS accounts with which to share the snapshots.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetAccounts")]
    pub target_accounts: Option<Vec<String>>,


    /// 
    /// The unit of time for the automatic unsharing interval.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAYS | MONTHS | WEEKS | YEARS
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnshareIntervalUnit")]
    pub unshare_interval_unit: Option<ShareRuleUnshareIntervalUnitEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ShareRuleUnshareIntervalUnitEnum {

    /// DAYS
    #[serde(rename = "DAYS")]
    Days,

    /// MONTHS
    #[serde(rename = "MONTHS")]
    Months,

    /// WEEKS
    #[serde(rename = "WEEKS")]
    Weeks,

    /// YEARS
    #[serde(rename = "YEARS")]
    Years,

}

impl Default for ShareRuleUnshareIntervalUnitEnum {
    fn default() -> Self {
        ShareRuleUnshareIntervalUnitEnum::Days
    }
}



/// The DeprecateRule property type specifies Property description not available. for an AWS::DLM::LifecyclePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DeprecateRule {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,

}




/// [Snapshot and AMI policies only] Specifies a cross-Region copy rule for snapshot and AMI policies.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CrossRegionCopyRule {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: CrossRegionCopyDeprecateRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeprecateRule")]
    pub deprecate_rule: Option<CrossRegionCopyDeprecateRule>,


    /// 
    /// NoteAvoid using this parameter when creating new policies. Instead, use 				Target to specify a target Region or a target 				Outpost for snapshot copies.For policies created before the Target parameter 				was introduced, this parameter indicates the target Region for snapshot copies.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 16
    ///
    /// Pattern: ([a-z]+-){2,3}\d
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetRegion")]
    pub target_region: Option<String>,


    /// 
    /// To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, 			enable encryption using this parameter. Copies of encrypted snapshots are encrypted, 			even if this parameter is false or if encryption by default is not enabled.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,


    /// 
    /// The target Region or the Amazon Resource Name (ARN) of the target Outpost for the 			snapshot copies.
    /// 
    /// Use this parameter instead of TargetRegion. Do not 			specify both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^[\w:\-\/\*]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Option<String>,


    /// 
    /// Indicates whether to copy all user-defined tags from the source snapshot or AMI to the 			cross-Region copy.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTags")]
    pub copy_tags: Option<bool>,


    /// 
    /// The retention rule that indicates how long the cross-Region snapshot or AMI copies are 			to be retained in the destination Region.
    /// 
    /// Required: No
    ///
    /// Type: CrossRegionCopyRetainRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS KMS key to use for EBS encryption. If this 			parameter is not specified, the default KMS key for the account is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws(-[a-z]{1,3}){0,2}:kms:([a-z]+-){2,3}\d:\d+:key/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CmkArn")]
    pub cmk_arn: Option<String>,

}




/// [Event-based policies only] Specifies an action for an event-based policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {


    /// 
    /// A descriptive name for the action.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 120
    ///
    /// Pattern: [0-9A-Za-z _-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The rule for copying shared snapshots across Regions.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CrossRegionCopyAction
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossRegionCopy")]
    pub cross_region_copy: Vec<CrossRegionCopyAction>,

}




/// [Snapshot policies only] Specifies a rule for enabling fast snapshot restore for snapshots created by 			snapshot policies. You can enable fast snapshot restore based on either a count or a 			time interval.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FastRestoreRule {


    /// 
    /// The unit of time for enabling fast snapshot restore.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAYS | MONTHS | WEEKS | YEARS
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<FastRestoreRuleIntervalUnitEnum>,


    /// 
    /// The number of snapshots to be enabled with fast snapshot restore.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// 
    /// The Availability Zones in which to enable fast snapshot restore.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,


    /// 
    /// The amount of time to enable fast snapshot restore. The maximum is 100 years. This is 			equivalent to 1200 months, 5200 weeks, or 36500 days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FastRestoreRuleIntervalUnitEnum {

    /// DAYS
    #[serde(rename = "DAYS")]
    Days,

    /// MONTHS
    #[serde(rename = "MONTHS")]
    Months,

    /// WEEKS
    #[serde(rename = "WEEKS")]
    Weeks,

    /// YEARS
    #[serde(rename = "YEARS")]
    Years,

}

impl Default for FastRestoreRuleIntervalUnitEnum {
    fn default() -> Self {
        FastRestoreRuleIntervalUnitEnum::Days
    }
}



/// [Snapshot and AMI policies only] Specifies a retention rule for snapshots created by snapshot policies, or for AMIs 			created by AMI policies.
///
/// You can retain snapshots based on either a count or a time interval.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetainRule {


    /// 
    /// The unit of time for time-based retention. For example, to retain snapshots for 3 months, specify 			Interval=3 and IntervalUnit=MONTHS. Once the snapshot has been retained for 			3 months, it is deleted, or it is moved to the archive tier if you have specified an 			ArchiveRule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAYS | MONTHS | WEEKS | YEARS
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<RetainRuleIntervalUnitEnum>,


    /// 
    /// The number of snapshots to retain for each volume, up to a maximum of 1000. For example if you want to 			retain a maximum of three snapshots, specify 3. When the fourth snapshot is created, the 			oldest retained snapshot is deleted, or it is moved to the archive tier if you have specified an 			ArchiveRule.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// 
    /// The amount of time to retain each snapshot. The maximum is 100 years. This is 			equivalent to 1200 months, 5200 weeks, or 36500 days.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum RetainRuleIntervalUnitEnum {

    /// DAYS
    #[serde(rename = "DAYS")]
    Days,

    /// MONTHS
    #[serde(rename = "MONTHS")]
    Months,

    /// WEEKS
    #[serde(rename = "WEEKS")]
    Weeks,

    /// YEARS
    #[serde(rename = "YEARS")]
    Years,

}

impl Default for RetainRuleIntervalUnitEnum {
    fn default() -> Self {
        RetainRuleIntervalUnitEnum::Days
    }
}



/// The ArchiveRetainRule property type specifies Property description not available. for an AWS::DLM::LifecyclePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveRetainRule {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: RetentionArchiveTier
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetentionArchiveTier")]
    pub retention_archive_tier: RetentionArchiveTier,

}




/// [Event-based policies only] Specifies the encryption settings for cross-Region snapshot copies created by 			event-based policies.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EncryptionConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the AWS KMS key to use for EBS encryption. If 			this parameter is not specified, the default KMS key for the account is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws(-[a-z]{1,3}){0,2}:kms:([a-z]+-){2,3}\d:\d+:key/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CmkArn")]
    pub cmk_arn: Option<String>,


    /// 
    /// To encrypt a copy of an unencrypted snapshot when encryption by default is not enabled, enable 			encryption using this parameter. Copies of encrypted snapshots are encrypted, even if this 			parameter is false or when encryption by default is not enabled.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,

}




/// [All policy types] Specifies the configuration of a lifecycle policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PolicyDetails {


    /// 
    /// [Snapshot and AMI policies only] The location of the resources to backup. If the source resources are located in an 			AWS Region, specify CLOUD. If the source resources are located on an Outpost 			in your account, specify OUTPOST.
    /// 
    /// If you specify OUTPOST, Amazon Data Lifecycle Manager backs up all resources 				of the specified type with matching target tags across all of the Outposts in your account.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLocations")]
    pub resource_locations: Option<Vec<String>>,


    /// 
    /// [Event-based policies only] The event that activates the event-based policy.
    /// 
    /// Required: No
    ///
    /// Type: EventSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventSource")]
    pub event_source: Option<EventSource>,


    /// 
    /// [Snapshot and AMI policies only] The schedules of policy-defined actions for snapshot and AMI lifecycle policies. A policy 			can have up to four schedules—one mandatory schedule and up to three optional schedules.
    /// 
    /// Required: No
    ///
    /// Type: List of Schedule
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedules")]
    pub schedules: Option<Vec<Schedule>>,


    /// 
    /// [Event-based policies only] The actions to be performed when the event-based policy is activated. You can specify 			only one action per policy.
    /// 
    /// Required: No
    ///
    /// Type: List of Action
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,


    /// 
    /// [Snapshot and AMI policies only] A set of optional parameters for snapshot and AMI lifecycle policies.
    /// 
    /// NoteIf you are modifying a policy that was created or previously modified using the Amazon 				Data Lifecycle Manager console, then you must include this parameter and specify either 				the default values or the new values that you require. You can't omit this parameter or 				set its values to null.
    /// 
    /// Required: No
    ///
    /// Type: Parameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<Parameters>,


    /// 
    /// [Snapshot policies only] The target resource type for snapshot and AMI lifecycle policies. Use VOLUME to 			create snapshots of individual volumes or use INSTANCE to create multi-volume 			snapshots from the volumes for an instance.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,


    /// 
    /// [Snapshot and AMI policies only] The single tag that identifies targeted resources for this policy.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTags")]
    pub target_tags: Option<Vec<Tag>>,


    /// 
    /// [All policy types] The valid target resource types and actions a policy can manage. Specify EBS_SNAPSHOT_MANAGEMENT 			to create a lifecycle policy that manages the lifecycle of Amazon EBS snapshots. Specify IMAGE_MANAGEMENT 			to create a lifecycle policy that manages the lifecycle of EBS-backed AMIs. Specify EVENT_BASED_POLICY  			to create an event-based policy that performs specific actions when a defined event occurs in your AWS account.
    /// 
    /// The default is EBS_SNAPSHOT_MANAGEMENT.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EBS_SNAPSHOT_MANAGEMENT | EVENT_BASED_POLICY | IMAGE_MANAGEMENT
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<PolicyDetailsPolicyTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PolicyDetailsPolicyTypeEnum {

    /// EBS_SNAPSHOT_MANAGEMENT
    #[serde(rename = "EBS_SNAPSHOT_MANAGEMENT")]
    Ebssnapshotmanagement,

    /// EVENT_BASED_POLICY
    #[serde(rename = "EVENT_BASED_POLICY")]
    Eventbasedpolicy,

    /// IMAGE_MANAGEMENT
    #[serde(rename = "IMAGE_MANAGEMENT")]
    Imagemanagement,

}

impl Default for PolicyDetailsPolicyTypeEnum {
    fn default() -> Self {
        PolicyDetailsPolicyTypeEnum::Ebssnapshotmanagement
    }
}



/// [Event-based policies only] Specifies an event that activates an event-based policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventParameters {


    /// 
    /// The snapshot description that can trigger the policy. The description pattern is specified using 			a regular expression. The policy runs only if a snapshot with a description that matches the 			specified pattern is shared with your account.
    /// 
    /// For example, specifying ^.*Created for policy: policy-1234567890abcdef0.*$  			configures the policy to run only if snapshots created by policy policy-1234567890abcdef0 			are shared with your account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Pattern: [\p{all}]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DescriptionRegex")]
    pub description_regex: Option<String>,


    /// 
    /// The type of event. Currently, only snapshot sharing events are supported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: shareSnapshot
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventType")]
    pub event_type: EventParametersEventTypeEnum,


    /// 
    /// The IDs of the AWS accounts that can trigger policy by sharing snapshots with your account. 			The policy only runs if one of the specified AWS accounts shares a snapshot with your account.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnapshotOwner")]
    pub snapshot_owner: Vec<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EventParametersEventTypeEnum {

    /// shareSnapshot
    #[serde(rename = "shareSnapshot")]
    Sharesnapshot,

}

impl Default for EventParametersEventTypeEnum {
    fn default() -> Self {
        EventParametersEventTypeEnum::Sharesnapshot
    }
}



/// The RetentionArchiveTier property type specifies Property description not available. for an AWS::DLM::LifecyclePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RetentionArchiveTier {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: Option<String>,

}




/// The ArchiveRule property type specifies Property description not available. for an AWS::DLM::LifecyclePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ArchiveRule {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ArchiveRetainRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainRule")]
    pub retain_rule: ArchiveRetainRule,

}




/// [Snapshot and AMI policies only] Specifies a schedule for a snapshot or AMI lifecycle policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Schedule {


    /// 
    /// Copy all user-defined tags on a source volume to snapshots of the volume created by 			this policy.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CopyTags")]
    pub copy_tags: Option<bool>,


    /// 
    /// [AMI policies and snapshot policies that target instances only] 			A collection of key/value pairs with values determined dynamically when the policy is 			executed. Keys may be any valid Amazon EC2 tag key. Values must be in one of the two 			following formats: $(instance-id) or $(timestamp). Variable 			tags are only valid for EBS Snapshot Management – Instance policies.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 45
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariableTags")]
    pub variable_tags: Option<Vec<Tag>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ArchiveRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArchiveRule")]
    pub archive_rule: Option<ArchiveRule>,


    /// 
    /// The creation rule.
    /// 
    /// Required: No
    ///
    /// Type: CreateRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateRule")]
    pub create_rule: Option<CreateRule>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DeprecateRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeprecateRule")]
    pub deprecate_rule: Option<DeprecateRule>,


    /// 
    /// Specifies a rule for copying snapshots or AMIs across regions.
    /// 
    /// NoteYou can't specify cross-Region copy rules for policies that create snapshots on an Outpost. 			If the policy creates snapshots in a Region, then snapshots can be copied to up to three 			Regions or Outposts.
    /// 
    /// Required: No
    ///
    /// Type: List of CrossRegionCopyRule
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossRegionCopyRules")]
    pub cross_region_copy_rules: Option<Vec<CrossRegionCopyRule>>,


    /// 
    /// The retention rule for snapshots or AMIs created by the policy.
    /// 
    /// Required: No
    ///
    /// Type: RetainRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetainRule")]
    pub retain_rule: Option<RetainRule>,


    /// 
    /// [Snapshot policies only] The rule for enabling fast snapshot restore.
    /// 
    /// Required: No
    ///
    /// Type: FastRestoreRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "FastRestoreRule")]
    pub fast_restore_rule: Option<FastRestoreRule>,


    /// 
    /// The name of the schedule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 120
    ///
    /// Pattern: [0-9A-Za-z _-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The tags to apply to policy-created resources. These user-defined tags are in addition 			to the AWS-added lifecycle tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 45
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagsToAdd")]
    pub tags_to_add: Option<Vec<Tag>>,


    /// 
    /// [Snapshot policies only] The rule for sharing snapshots with other AWS accounts.
    /// 
    /// Required: No
    ///
    /// Type: List of ShareRule
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShareRules")]
    pub share_rules: Option<Vec<ShareRule>>,

}




/// The CrossRegionCopyDeprecateRule property type specifies Property description not available. for an AWS::DLM::LifecyclePolicy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CrossRegionCopyDeprecateRule {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: i64,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: String,

}




/// Specifies a retention rule for cross-Region snapshot copies created by snapshot or 			event-based policies, or cross-Region AMI copies created by AMI policies. After the 			retention period expires, the cross-Region copy is deleted.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CrossRegionCopyRetainRule {


    /// 
    /// The unit of time for time-based retention. For example, to retain a cross-Region copy for 			3 months, specify Interval=3 and IntervalUnit=MONTHS.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DAYS | MONTHS | WEEKS | YEARS
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalUnit")]
    pub interval_unit: CrossRegionCopyRetainRuleIntervalUnitEnum,


    /// 
    /// The amount of time to retain a cross-Region snapshot or AMI copy. The maximum is 100 years. 			This is equivalent to 1200 months, 5200 weeks, or 36500 days.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: i64,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CrossRegionCopyRetainRuleIntervalUnitEnum {

    /// DAYS
    #[serde(rename = "DAYS")]
    Days,

    /// MONTHS
    #[serde(rename = "MONTHS")]
    Months,

    /// WEEKS
    #[serde(rename = "WEEKS")]
    Weeks,

    /// YEARS
    #[serde(rename = "YEARS")]
    Years,

}

impl Default for CrossRegionCopyRetainRuleIntervalUnitEnum {
    fn default() -> Self {
        CrossRegionCopyRetainRuleIntervalUnitEnum::Days
    }
}

