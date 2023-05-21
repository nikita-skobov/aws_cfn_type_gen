

/// The AWS::AutoScaling::ScheduledAction resource specifies an Amazon EC2 Auto    Scaling scheduled action so that the Auto Scaling group can change the number of instances    available for your application in response to predictable load changes.
///
/// When you update a stack with an Auto Scaling group and scheduled action, CloudFormation    always sets the min size, max size, and desired capacity properties of your group to the    values that are defined in the AWS::AutoScaling::AutoScalingGroup section of your    template. However, you might not want CloudFormation to do that when you have a scheduled    action in effect. You can use an UpdatePolicy     attribute to prevent CloudFormation from changing the min size, max size, or desired    capacity property values during a stack update unless you modified the individual values in    your template. If you have rolling updates enabled, before you can update the Auto Scaling    group, you must suspend scheduled actions by specifying an UpdatePolicy     attribute for the Auto Scaling group. You can find a sample update policy for    rolling updates in Auto scaling template     snippets.
///
/// For more information, see Scheduled scaling and Suspending and resuming scaling processes in the Amazon EC2 Auto Scaling     User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnScheduledAction {


    /// 
    /// Specifies the time zone for a cron expression. If a time zone is not provided, UTC is       used by default.
    /// 
    /// Valid values are the canonical names of the IANA time zones, derived from the IANA       Time Zone Database (such as Etc/GMT+9 or Pacific/Tahiti). For       more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,


    /// 
    /// The date and time for the recurring schedule to end, in UTC. For example,         "2021-06-01T00:00:00Z".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,


    /// 
    /// The desired capacity is the initial capacity of the Auto Scaling group after the scheduled       action runs and the capacity it attempts to maintain. It can scale beyond this capacity       if you add more scaling conditions.
    /// 
    /// NoteYou must specify at least one of the following properties: MaxSize,           MinSize, or DesiredCapacity.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredCapacity")]
    pub desired_capacity: Option<i64>,


    /// 
    /// The name of the Auto Scaling group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,


    /// 
    /// The minimum size of the Auto Scaling group.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: Option<i64>,


    /// 
    /// The recurring schedule for this action. This format consists of five fields separated       by white spaces: [Minute] [Hour] [Day_of_Month] [Month_of_Year] [Day_of_Week]. The value       must be in quotes (for example, "30 0 1 1,6,12 *"). For more information       about this format, see Crontab.
    /// 
    /// When StartTime and EndTime are specified with         Recurrence, they form the boundaries of when the recurring action       starts and stops.
    /// 
    /// Cron expressions use Universal Coordinated Time (UTC) by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Recurrence")]
    pub recurrence: Option<String>,


    /// 
    /// The date and time for this action to start, in YYYY-MM-DDThh:mm:ssZ format in UTC/GMT       only and in quotes (for example, "2021-06-01T00:00:00Z").
    /// 
    /// If you specify Recurrence and StartTime, Amazon EC2 Auto Scaling performs       the action at this time, and then performs the action based on the specified       recurrence.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,


    /// 
    /// The maximum size of the Auto Scaling group.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: Option<i64>,

}