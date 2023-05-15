
pub mod cfn_contact {

#[derive(serde::Serialize, Default)]
pub struct CfnContact {
    /// Name of the contact. String value with 3 to 256 characters. Only alphabetical, space, numeric characters, dash, or underscore allowed.
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    /// Contact type, which specify type of contact. Currently supported values: “PERSONAL”, “SHARED”, “OTHER“.
    #[serde(rename = "Type")]
    pub ty: String,
    /// The stages that an escalation plan or engagement plan engages contacts and contact methods in.
    #[serde(rename = "Plan")]
    pub plan: Option<Vec<Stage>>,
    /// Alias of the contact. String value with 20 to 256 characters. Only alphabetical, numeric characters, dash, or underscore allowed.
    #[serde(rename = "Alias")]
    pub alias: String,

}


#[derive(serde::Serialize, Default)]
pub struct Targets {
    #[serde(rename = "ChannelTargetInfo")]
    pub channel_target_info: Option<ChannelTargetInfo>,
    #[serde(rename = "ContactTargetInfo")]
    pub contact_target_info: Option<ContactTargetInfo>,

}

#[derive(serde::Serialize, Default)]
pub struct ContactTargetInfo {
    #[serde(rename = "IsEssential")]
    pub is_essential: bool,
    #[serde(rename = "ContactId")]
    pub contact_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ChannelTargetInfo {
    #[serde(rename = "RetryIntervalInMinutes")]
    pub retry_interval_in_minutes: usize,
    #[serde(rename = "ChannelId")]
    pub channel_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Stage {
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Targets>>,
    #[serde(rename = "RotationIds")]
    pub rotation_ids: Option<Vec<String>>,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: Option<usize>,

}


}

pub mod cfn_contact_channel {

#[derive(serde::Serialize, Default)]
pub struct CfnContactChannel {
    /// The device name. String of 6 to 50 alphabetical, numeric, dash, and underscore characters.
    #[serde(rename = "ChannelName")]
    pub channel_name: Option<String>,
    /// ARN of the contact resource
    #[serde(rename = "ContactId")]
    pub contact_id: Option<String>,
    /// If you want to activate the channel at a later time, you can choose to defer activation. SSM Incident Manager can't engage your contact channel until it has been activated.
    #[serde(rename = "DeferActivation")]
    pub defer_activation: Option<bool>,
    /// The details that SSM Incident Manager uses when trying to engage the contact channel.
    #[serde(rename = "ChannelAddress")]
    pub channel_address: Option<String>,
    /// Device type, which specify notification channel. Currently supported values: “SMS”, “VOICE”, “EMAIL”, “CHATBOT.
    #[serde(rename = "ChannelType")]
    pub channel_type: Option<String>,

}



}

pub mod cfn_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnPlan {
    /// The stages that an escalation plan or engagement plan engages contacts and contact methods in.
    #[serde(rename = "Stages")]
    pub stages: Option<Vec<Stage>>,
    /// Rotation Ids to associate with Oncall Contact for engagement.
    #[serde(rename = "RotationIds")]
    pub rotation_ids: Option<Vec<String>>,
    /// Contact ID for the AWS SSM Incident Manager Contact to associate the plan.
    #[serde(rename = "ContactId")]
    pub contact_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ContactTargetInfo {
    #[serde(rename = "ContactId")]
    pub contact_id: String,
    #[serde(rename = "IsEssential")]
    pub is_essential: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ChannelTargetInfo {
    #[serde(rename = "ChannelId")]
    pub channel_id: String,
    #[serde(rename = "RetryIntervalInMinutes")]
    pub retry_interval_in_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Stage {
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Targets>>,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Targets {
    #[serde(rename = "ChannelTargetInfo")]
    pub channel_target_info: Option<ChannelTargetInfo>,
    #[serde(rename = "ContactTargetInfo")]
    pub contact_target_info: Option<ContactTargetInfo>,

}


}

pub mod cfn_rotation {

#[derive(serde::Serialize, Default)]
pub struct CfnRotation {
    /// Members of the rotation
    #[serde(rename = "ContactIds")]
    pub contact_ids: Vec<String>,
    /// Start time of the first shift of Oncall Schedule
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// Name of the Rotation
    #[serde(rename = "Name")]
    pub name: String,
    /// TimeZone Identifier for the Oncall Schedule
    #[serde(rename = "TimeZoneId")]
    pub time_zone_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Recurrence")]
    pub recurrence: RecurrenceSettings,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct DailySetting {
    #[serde(rename = "HandOffTime")]
    pub hand_off_time: HandOffTime,

}

#[derive(serde::Serialize, Default)]
pub struct WeeklySetting {
    #[serde(rename = "HandOffTime")]
    pub hand_off_time: HandOffTime,
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: DayOfWeek,

}

#[derive(serde::Serialize, Default)]
pub struct MonthlySetting {
    #[serde(rename = "HandOffTime")]
    pub hand_off_time: HandOffTime,
    #[serde(rename = "DayOfMonth")]
    pub day_of_month: usize,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ShiftCoverage {
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: DayOfWeek,
    #[serde(rename = "CoverageTimes")]
    pub coverage_times: Vec<CoverageTime>,

}

#[derive(serde::Serialize, Default)]
pub struct CoverageTime {
    #[serde(rename = "EndTime")]
    pub end_time: HandOffTime,
    #[serde(rename = "StartTime")]
    pub start_time: HandOffTime,

}
pub type HandOffTime = String;pub type DayOfWeek = String;
#[derive(serde::Serialize, Default)]
pub struct RecurrenceSettings {
    #[serde(rename = "MonthlySettings")]
    pub monthly_settings: Option<Vec<MonthlySetting>>,
    #[serde(rename = "NumberOfOnCalls")]
    pub number_of_on_calls: Option<usize>,
    #[serde(rename = "WeeklySettings")]
    pub weekly_settings: Option<Vec<WeeklySetting>>,
    #[serde(rename = "RecurrenceMultiplier")]
    pub recurrence_multiplier: Option<usize>,
    #[serde(rename = "DailySettings")]
    pub daily_settings: Option<Vec<HandOffTime>>,
    #[serde(rename = "ShiftCoverages")]
    pub shift_coverages: Option<Vec<ShiftCoverage>>,

}


}
