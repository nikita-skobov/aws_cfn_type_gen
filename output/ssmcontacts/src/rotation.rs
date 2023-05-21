/// Specifies a rotation in an on-call schedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRotation {
    ///
    /// The Amazon Resource Names (ARNs) of the contacts to add to the rotation.
    ///
    /// The order in which you list the contacts is their shift order in the rotation       schedule.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactIds")]
    pub contact_ids: Vec<String>,

    ///
    /// The name for the rotation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9_\-\s\.]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Information about the rule that specifies when shift team members rotate.
    ///
    /// Required: Yes
    ///
    /// Type: RecurrenceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "Recurrence")]
    pub recurrence: RecurrenceSettings,

    ///
    /// The date and time the rotation goes into effect.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: String,

    ///
    /// Optional metadata to assign to the rotation. Tags enable you to categorize a resource       in different ways, such as by purpose, owner, or environment. For more information, see         Tagging Incident Manager resources in the Incident Manager User         Guide.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The time zone to base the rotation’s activity on, in Internet Assigned Numbers       Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For       more information, see the Time Zone         Database on the IANA website.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[:a-zA-Z0-9_\-\s\.\\/]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZoneId")]
    pub time_zone_id: String,
}

impl cfn_resources::CfnResource for CfnRotation {
    fn type_string(&self) -> &'static str {
        "AWS::SSMContacts::Rotation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.contact_ids;

        if the_val.len() > 25 as _ {
            return Err(format!(
                "Max validation failed on field 'contact_ids'. {} is greater than 25",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        self.recurrence.validate()?;

        let the_val = &self.time_zone_id;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'time_zone_id'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.time_zone_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'time_zone_id'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information about when an on-call shift begins and ends.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CoverageTime {
    ///
    /// Information about when an on-call rotation shift ends.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    pub end_time: String,

    ///
    /// Information about when an on-call rotation shift begins.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: String,
}

impl cfn_resources::CfnResource for CoverageTime {
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

/// Information about on-call rotations that recur monthly.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonthlySetting {
    ///
    /// The day of the month when monthly recurring on-call rotations begin.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 31
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfMonth")]
    pub day_of_month: i64,

    ///
    /// The time of day when a monthly recurring on-call shift rotation begins.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HandOffTime")]
    pub hand_off_time: String,
}

impl cfn_resources::CfnResource for MonthlySetting {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.day_of_month;

        if *the_val > 31 as _ {
            return Err(format!(
                "Max validation failed on field 'day_of_month'. {} is greater than 31",
                the_val
            ));
        }

        let the_val = &self.day_of_month;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'day_of_month'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Information about when an on-call rotation is in effect and how long the rotation       period lasts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RecurrenceSettings {
    ///
    /// Information about on-call rotations that recur daily.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DailySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_settings: Option<Vec<String>>,

    ///
    /// Information about on-call rotations that recur monthly.
    ///
    /// Required: No
    ///
    /// Type: List of MonthlySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonthlySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_settings: Option<Vec<MonthlySetting>>,

    ///
    /// The number of contacts, or shift team members designated to be on call concurrently       during a shift. For example, in an on-call schedule that contains ten contacts, a value of         2 designates that two of them are on call at any given time.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "NumberOfOnCalls")]
    pub number_of_on_calls: i64,

    ///
    /// The number of days, weeks, or months a single rotation lasts.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "RecurrenceMultiplier")]
    pub recurrence_multiplier: i64,

    ///
    /// Information about the days of the week included in on-call rotation coverage.
    ///
    /// Required: No
    ///
    /// Type: List of ShiftCoverage
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShiftCoverages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_coverages: Option<Vec<ShiftCoverage>>,

    ///
    /// Information about on-call rotations that recur weekly.
    ///
    /// Required: No
    ///
    /// Type: List of WeeklySetting
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeeklySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_settings: Option<Vec<WeeklySetting>>,
}

impl cfn_resources::CfnResource for RecurrenceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.number_of_on_calls;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'number_of_on_calls'. {} is less than 1",
                the_val
            ));
        }

        let the_val = &self.recurrence_multiplier;

        if *the_val > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'recurrence_multiplier'. {} is greater than 100",
                the_val
            ));
        }

        let the_val = &self.recurrence_multiplier;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'recurrence_multiplier'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Information about the days of the week that the on-call rotation coverage includes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShiftCoverage {
    ///
    /// The start and end times of the shift.
    ///
    /// Required: Yes
    ///
    /// Type: List of CoverageTime
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoverageTimes")]
    pub coverage_times: Vec<CoverageTime>,

    ///
    /// A list of days on which the schedule is active.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: String,
}

impl cfn_resources::CfnResource for ShiftCoverage {
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
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for Tag {
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

/// Information about rotations that recur weekly.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WeeklySetting {
    ///
    /// The day of the week when weekly recurring on-call shift rotations begins.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FRI | MON | SAT | SUN | THU | TUE | WED
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: WeeklySettingDayOfWeekEnum,

    ///
    /// The time of day when a weekly recurring on-call shift rotation begins.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HandOffTime")]
    pub hand_off_time: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum WeeklySettingDayOfWeekEnum {
    /// FRI
    #[serde(rename = "FRI")]
    Fri,

    /// MON
    #[serde(rename = "MON")]
    Mon,

    /// SAT
    #[serde(rename = "SAT")]
    Sat,

    /// SUN
    #[serde(rename = "SUN")]
    Sun,

    /// THU
    #[serde(rename = "THU")]
    Thu,

    /// TUE
    #[serde(rename = "TUE")]
    Tue,

    /// WED
    #[serde(rename = "WED")]
    Wed,
}

impl Default for WeeklySettingDayOfWeekEnum {
    fn default() -> Self {
        WeeklySettingDayOfWeekEnum::Fri
    }
}

impl cfn_resources::CfnResource for WeeklySetting {
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
