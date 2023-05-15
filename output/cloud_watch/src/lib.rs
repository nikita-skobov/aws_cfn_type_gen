
pub mod cfn_alarm {

#[derive(serde::Serialize, Default)]
pub struct CfnAlarm {
    /// No documentation provided by AWS
    #[serde(rename = "Period")]
    pub period: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ActionsEnabled")]
    pub actions_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AlarmDescription")]
    pub alarm_description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AlarmName")]
    pub alarm_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ThresholdMetricId")]
    pub threshold_metric_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InsufficientDataActions")]
    pub insufficient_data_actions: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    /// No documentation provided by AWS
    #[serde(rename = "TreatMissingData")]
    pub treat_missing_data: Option<String>,
    /// List of MetricDataQuery
    #[serde(rename = "Metrics")]
    pub metrics: Option<Vec<MetricDataQuery>>,
    /// No documentation provided by AWS
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DatapointsToAlarm")]
    pub datapoints_to_alarm: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ExtendedStatistic")]
    pub extended_statistic: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OKActions")]
    pub okactions: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Threshold")]
    pub threshold: Option<f64>,
    /// List of Dimension
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,
    /// No documentation provided by AWS
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: usize,

}


#[derive(serde::Serialize, Default)]
pub struct MetricDataQuery {
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,
    #[serde(rename = "Period")]
    pub period: Option<usize>,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricStat {
    #[serde(rename = "Period")]
    pub period: usize,
    #[serde(rename = "Stat")]
    pub stat: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Metric")]
    pub metric: Metric,

}

#[derive(serde::Serialize, Default)]
pub struct Metric {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,

}

#[derive(serde::Serialize, Default)]
pub struct Dimension {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_anomaly_detector {

#[derive(serde::Serialize, Default)]
pub struct CfnAnomalyDetector {
    /// No documentation provided by AWS
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MetricMathAnomalyDetector")]
    pub metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,
    /// List of Dimension
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,
    /// No documentation provided by AWS
    #[serde(rename = "Configuration")]
    pub configuration: Option<Configuration>,
    /// No documentation provided by AWS
    #[serde(rename = "Stat")]
    pub stat: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SingleMetricAnomalyDetector")]
    pub single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,
    /// No documentation provided by AWS
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct MetricMathAnomalyDetector {
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,

}

#[derive(serde::Serialize, Default)]
pub struct SingleMetricAnomalyDetector {
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Stat")]
    pub stat: Option<String>,
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricStat {
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Stat")]
    pub stat: String,
    #[serde(rename = "Metric")]
    pub metric: Metric,
    #[serde(rename = "Period")]
    pub period: usize,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDataQuery {
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Period")]
    pub period: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Metric {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,

}

#[derive(serde::Serialize, Default)]
pub struct Configuration {
    #[serde(rename = "ExcludedTimeRanges")]
    pub excluded_time_ranges: Option<Vec<Range>>,
    #[serde(rename = "MetricTimeZone")]
    pub metric_time_zone: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Dimension {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Range {
    #[serde(rename = "EndTime")]
    pub end_time: String,
    #[serde(rename = "StartTime")]
    pub start_time: String,

}


}

pub mod cfn_composite_alarm {

#[derive(serde::Serialize, Default)]
pub struct CfnCompositeAlarm {
    /// Expression which aggregates the state of other Alarms (Metric or Composite Alarms)
    #[serde(rename = "AlarmRule")]
    pub alarm_rule: String,
    /// Actions will be suppressed if WaitPeriod is active. The length of time that actions are suppressed is in seconds.
    #[serde(rename = "ActionsSuppressorExtensionPeriod")]
    pub actions_suppressor_extension_period: Option<usize>,
    /// Actions will be suppressed if ExtensionPeriod is active. The length of time that actions are suppressed is in seconds.
    #[serde(rename = "ActionsSuppressorWaitPeriod")]
    pub actions_suppressor_wait_period: Option<usize>,
    /// Indicates whether actions should be executed during any changes to the alarm state. The default is TRUE.
    #[serde(rename = "ActionsEnabled")]
    pub actions_enabled: Option<bool>,
    /// Actions will be suppressed if the suppressor alarm is in the ALARM state. ActionsSuppressor can be an AlarmName or an Amazon Resource Name (ARN) from an existing alarm.
    #[serde(rename = "ActionsSuppressor")]
    pub actions_suppressor: Option<String>,
    /// The actions to execute when this alarm transitions to the OK state from any other state. Each action is specified as an Amazon Resource Name (ARN).
    #[serde(rename = "OKActions")]
    pub okactions: Option<Vec<String>>,
    /// The name of the Composite Alarm
    #[serde(rename = "AlarmName")]
    pub alarm_name: Option<String>,
    /// The list of actions to execute when this alarm transitions into an ALARM state from any other state. Specify each action as an Amazon Resource Name (ARN).
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<Vec<String>>,
    /// The description of the alarm
    #[serde(rename = "AlarmDescription")]
    pub alarm_description: Option<String>,
    /// The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state from any other state. Each action is specified as an Amazon Resource Name (ARN).
    #[serde(rename = "InsufficientDataActions")]
    pub insufficient_data_actions: Option<Vec<String>>,

}



}

pub mod cfn_dashboard {

#[derive(serde::Serialize, Default)]
pub struct CfnDashboard {
    /// No documentation provided by AWS
    #[serde(rename = "DashboardBody")]
    pub dashboard_body: String,
    /// No documentation provided by AWS
    #[serde(rename = "DashboardName")]
    pub dashboard_name: Option<String>,

}



}

pub mod cfn_insight_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnInsightRule {
    /// No documentation provided by AWS
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RuleState")]
    pub rule_state: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// No documentation provided by AWS
    #[serde(rename = "RuleBody")]
    pub rule_body: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_metric_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnMetricStream {
    /// Define which metrics will be streamed. Metrics matched by multiple instances of MetricStreamFilter are joined with an OR operation by default. If both IncludeFilters and ExcludeFilters are omitted, all metrics in the account will be streamed. IncludeFilters and ExcludeFilters are mutually exclusive. Default to null.
    #[serde(rename = "IncludeFilters")]
    pub include_filters: Option<Vec<MetricStreamFilter>>,
    /// The ARN of the role that provides access to the Kinesis Firehose.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Name of the metric stream.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The ARN of the Kinesis Firehose where to stream the data.
    #[serde(rename = "FirehoseArn")]
    pub firehose_arn: String,
    /// By default, a metric stream always sends the MAX, MIN, SUM, and SAMPLECOUNT statistics for each metric that is streamed. You can use this parameter to have the metric stream also send additional statistics in the stream. This array can have up to 100 members.
    #[serde(rename = "StatisticsConfigurations")]
    pub statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>,
    /// If you are creating a metric stream in a monitoring account, specify true to include metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false.
    #[serde(rename = "IncludeLinkedAccountsMetrics")]
    pub include_linked_accounts_metrics: Option<bool>,
    /// Define which metrics will be not streamed. Metrics matched by multiple instances of MetricStreamFilter are joined with an OR operation by default. If both IncludeFilters and ExcludeFilters are omitted, all metrics in the account will be streamed. IncludeFilters and ExcludeFilters are mutually exclusive. Default to null.
    #[serde(rename = "ExcludeFilters")]
    pub exclude_filters: Option<Vec<MetricStreamFilter>>,
    /// The output format of the data streamed to the Kinesis Firehose.
    #[serde(rename = "OutputFormat")]
    pub output_format: String,
    /// A set of tags to assign to the delivery stream.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct MetricStreamStatisticsConfiguration {
    #[serde(rename = "AdditionalStatistics")]
    pub additional_statistics: Vec<String>,
    #[serde(rename = "IncludeMetrics")]
    pub include_metrics: Vec<MetricStreamStatisticsMetric>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricStreamStatisticsMetric {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricStreamFilter {
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "MetricNames")]
    pub metric_names: Option<Vec<String>>,

}


}
