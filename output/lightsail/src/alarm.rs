

/// The AWS::Lightsail::Alarm resource specifies an alarm that can be used to     monitor a single metric for one of your Lightsail resources.
#[derive(Default, serde::Serialize)]
pub struct CfnAlarm {


    /// 
    /// A Boolean value indicating whether the alarm is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationEnabled")]
    pub notification_enabled: Option<bool>,


    /// 
    /// The name of the metric associated with the alarm.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: BurstCapacityPercentage | BurstCapacityTime | ClientTLSNegotiationErrorCount | CPUUtilization | DatabaseConnections | DiskQueueDepth | FreeStorageSpace | HealthyHostCount | HTTPCode_Instance_2XX_Count | HTTPCode_Instance_3XX_Count | HTTPCode_Instance_4XX_Count | HTTPCode_Instance_5XX_Count | HTTPCode_LB_4XX_Count | HTTPCode_LB_5XX_Count | InstanceResponseTime | NetworkIn | NetworkOut | NetworkReceiveThroughput | NetworkTransmitThroughput | RejectedConnectionCount | RequestCount | StatusCheckFailed | StatusCheckFailed_Instance | StatusCheckFailed_System | UnhealthyHostCount
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The arithmetic operation to use when comparing the specified statistic and     threshold.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GreaterThanOrEqualToThreshold | GreaterThanThreshold | LessThanOrEqualToThreshold | LessThanThreshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,


    /// 
    /// The number of data points within the evaluation periods that must be breaching to cause     the alarm to go to the ALARM state.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatapointsToAlarm")]
    pub datapoints_to_alarm: Option<i64>,


    /// 
    /// Specifies how the alarm handles missing data points.
    /// 
    /// An alarm can treat missing data in the following ways:
    /// 
    /// breaching - Assumes the missing data is not within the threshold. Missing        data counts towards the number of times that the metric is not within the         threshold.                    notBreaching - Assumes the missing data is within the threshold. Missing        data does not count towards the number of times that the metric is not within the         threshold.                    ignore - Ignores the missing data. Maintains the current alarm        state.                    missing - Missing data is treated as missing.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: breaching | ignore | missing | notBreaching
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatMissingData")]
    pub treat_missing_data: Option<String>,


    /// 
    /// The value against which the specified statistic is compared.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: f64,


    /// 
    /// The alarm states that trigger a notification.
    /// 
    /// NoteTo specify the OK and INSUFFICIENT_DATA values, you must also       specify ContactProtocols values. Otherwise, the OK       and INSUFFICIENT_DATA values will not take effect and the stack will       drift.
    /// 
    /// Allowed Values: OK | ALARM |       INSUFFICIENT_DATA
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTriggers")]
    pub notification_triggers: Option<Vec<String>>,


    /// 
    /// The number of periods over which data is compared to the specified threshold.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: i64,


    /// 
    /// The contact protocols for the alarm, such as Email, SMS (text     messaging), or both.
    /// 
    /// Allowed Values: Email | SMS
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContactProtocols")]
    pub contact_protocols: Option<Vec<String>>,


    /// 
    /// The name of the alarm.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlarmName")]
    pub alarm_name: String,


    /// 
    /// The name of the Lightsail resource that the alarm monitors.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "MonitoredResourceName")]
    pub monitored_resource_name: String,

}
