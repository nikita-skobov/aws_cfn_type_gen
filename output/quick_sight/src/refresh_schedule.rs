

/// Creates a refresh schedule for a dataset in Amazon QuickSight.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRefreshSchedule {


    /// 
    /// The AWS account ID of the account that you are creating a schedule in.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: Option<String>,


    /// 
    /// The ID of the dataset that you are creating a refresh schedule for.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSetId")]
    pub data_set_id: Option<String>,


    /// 
    /// The refresh schedule of a dataset.
    /// 
    /// Required: No
    ///
    /// Type: RefreshScheduleMap
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<RefreshScheduleMap>,

}



impl cfn_resources::CfnResource for CfnRefreshSchedule {
    fn type_string() -> &'static str {
        "AWS::QuickSight::RefreshSchedule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The day that you want yout dataset to refresh.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RefreshOnDay {


    /// 
    /// The day of the month that you want your dataset to refresh. This value is required for monthly refresh  intervals.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfMonth")]
    pub day_of_month: Option<String>,


    /// 
    /// The day of the week that you want to schedule the refresh on. This value is required for weekly and monthly  refresh intervals.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: Option<String>,

}




/// A summary of a configured refresh schedule for a dataset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RefreshScheduleMap {


    /// 
    /// The type of refresh that a dataset undergoes. Valid values are as follows:
    /// 
    /// FULL_REFRESH: A complete refresh of a dataset.     INCREMENTAL_REFRESH: A partial refresh of some rows of a dataset, based on the time window   specified.
    /// 
    /// For more information on full and incremental refreshes, see Refreshing SPICE data in the Amazon QuickSight User Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshType")]
    pub refresh_type: Option<String>,


    /// 
    /// The frequency for the refresh schedule.
    /// 
    /// Required: No
    ///
    /// Type: ScheduleFrequency
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleFrequency")]
    pub schedule_frequency: Option<ScheduleFrequency>,


    /// 
    /// An identifier for the refresh schedule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScheduleId")]
    pub schedule_id: Option<String>,


    /// 
    /// Time after which the refresh schedule can be started, expressed in YYYY-MM-DDTHH:MM:SS  format.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartAfterDateTime")]
    pub start_after_date_time: Option<String>,

}




/// The frequency for the refresh schedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScheduleFrequency {


    /// 
    /// The interval between scheduled refreshes. Valid values are as follows:
    /// 
    /// MINUTE15: The dataset refreshes every 15 minutes. This value is only supported for incremental   refreshes. This interval can only be used for one schedule per dataset.     MINUTE30: The dataset refreshes every 30 minutes. This value is only supported for incremental   refreshes. This interval can only be used for one schedule per dataset.     HOURLY: The dataset refreshes every hour. This interval can only be used for one schedule per   dataset.     DAILY: The dataset refreshes every day.     WEEKLY: The dataset refreshes every week.     MONTHLY: The dataset refreshes every month.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Option<String>,


    /// 
    /// The day of the week that you want to schedule the refresh on. This value is required for weekly and monthly  refresh intervals.
    /// 
    /// Required: No
    ///
    /// Type: RefreshOnDay
    ///
    /// Update requires: No interruption
    #[serde(rename = "RefreshOnDay")]
    pub refresh_on_day: Option<RefreshOnDay>,


    /// 
    /// The time of day that you want the dataset to refresh. This value is expressed in HH:MM format. This field is not  required for schedules that refresh hourly.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeOfTheDay")]
    pub time_of_the_day: Option<String>,


    /// 
    /// The timezone that you want the refresh schedule to use. The timezone ID must match a corresponding ID found on  java.util.time.getAvailableIDs().
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,

}


