

/// The AWS::CloudWatch::Alarm type specifies an alarm and associates it with the specified metric or metric math expression.
///
/// When this operation creates an alarm, the alarm state is immediately set to 			INSUFFICIENT_DATA. The alarm is then evaluated and its state is set 			appropriately. Any actions associated with the new state are then executed.
///
/// When you update an existing alarm, its state is left unchanged, but the 			update completely overwrites the previous configuration of the alarm.
/// 
#[derive(Default, serde::Serialize)]
pub struct CfnAlarm {


    /// 
    /// Indicates whether actions should be executed during any changes to the alarm state. The default is TRUE.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionsEnabled")]
    pub actions_enabled: Option<bool>,


    /// 
    /// In an alarm based on an anomaly detection model, this is the ID of the 			ANOMALY_DETECTION_BAND function 			used as the threshold for the alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ThresholdMetricId")]
    pub threshold_metric_id: Option<String>,


    /// 
    /// The statistic for the metric associated with the alarm, other than percentile. 		  For percentile statistics, use ExtendedStatistic.
    /// 
    /// For an alarm based on a metric, you must specify either Statistic    or ExtendedStatistic but not both.
    /// 
    /// For an alarm based on a math expression, you can't       specify Statistic. Instead, you use Metrics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Average | Maximum | Minimum | SampleCount | Sum
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,


    /// 
    /// The unit of the metric associated with the alarm. Specify this only if you are creating an alarm 		      based on a single metric. Do not specify this if you are specifying a Metrics array.
    /// 
    /// You can specify the following values: Seconds, Microseconds, Milliseconds, Bytes, Kilobytes, 		      Megabytes, Gigabytes, Terabytes, Bits, Kilobits, Megabits, Gigabits, Terabits, Percent, Count, 		      Bytes/Second, Kilobytes/Second, Megabytes/Second, Gigabytes/Second, Terabytes/Second, Bits/Second, 		      Kilobits/Second, Megabits/Second, Gigabits/Second, Terabits/Second, Count/Second, or None.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Bits | Bits/Second | Bytes | Bytes/Second | Count | Count/Second | Gigabits | Gigabits/Second | Gigabytes | Gigabytes/Second | Kilobits | Kilobits/Second | Kilobytes | Kilobytes/Second | Megabits | Megabits/Second | Megabytes | Megabytes/Second | Microseconds | Milliseconds | None | Percent | Seconds | Terabits | Terabits/Second | Terabytes | Terabytes/Second
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,


    /// 
    /// Sets how this alarm is to handle missing data points. Valid values are breaching, notBreaching, ignore,      and missing. For more information, see         Configuring How CloudWatchAlarms Treat Missing Data in the       Amazon CloudWatchUser Guide.
    /// 
    /// If you omit this parameter, the default behavior of missing is used.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatMissingData")]
    pub treat_missing_data: Option<String>,


    /// 
    /// The number of periods over which data is compared to the specified threshold. 		      If you are setting an alarm that requires that a number of consecutive data points be 		      breaching to trigger the alarm, this value specifies that number. If you 		      are setting an "M out of N" alarm, this value is the N, and DatapointsToAlarm 		      is the M.
    /// 
    /// For more information, see Evaluating      an Alarm in the Amazon CloudWatch User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: i64,


    /// 
    /// The actions to execute when this alarm transitions to the OK state 			from any other state. Each action is specified as an Amazon Resource Name (ARN).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "OKActions")]
    pub okactions: Option<Vec<String>>,


    /// 
    /// An array that enables you to create an alarm based on the result of a metric math expression. Each     item in the array either retrieves a metric or performs a math expression.
    /// 
    /// If you specify the Metrics parameter, you cannot specify MetricName, Dimensions,       Period, Namespace, Statistic, ExtendedStatistic, or Unit.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDataQuery
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metrics")]
    pub metrics: Option<Vec<MetricDataQuery>>,


    /// 
    /// The description of the alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmDescription")]
    pub alarm_description: Option<String>,


    /// 
    /// Used only for alarms based on percentiles. If ignore, the alarm state does not change 			during periods with too few data points to be statistically significant. If evaluate or this 			parameter is not used, the alarm is always evaluated and possibly changes state no matter 			how many data points are available.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    pub evaluate_low_sample_count_percentile: Option<String>,


    /// 
    /// The namespace of the metric associated with the alarm. This is required for an alarm based on a       metric. For an alarm based on a math expression, you can't       specify Namespace and you use Metrics instead.
    /// 
    /// For a list of namespaces for metrics from AWS services, see       AWS Services That Publish CloudWatchMetrics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [^:].*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,


    /// 
    /// The name of the metric associated with the alarm. This is required for an alarm based on a 		    metric. For an alarm based on a math expression, you use Metrics instead and you can't 		    specify MetricName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,


    /// 
    /// The name of the alarm. If you don't specify a name, AWS CloudFormation generates a unique physical ID and uses that ID for the alarm name.
    /// 
    /// ImportantIf you specify a name, you cannot perform updates that require replacement of this resource. You can       perform updates that require no or some interruption. If you must replace the resource, specify a new name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlarmName")]
    pub alarm_name: Option<String>,


    /// 
    /// The arithmetic operation to use when comparing the specified 			statistic and threshold. The specified statistic value is used as the first operand.
    /// 
    /// You can specify the following values: GreaterThanThreshold, GreaterThanOrEqualToThreshold,       LessThanThreshold, or LessThanOrEqualToThreshold.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GreaterThanOrEqualToThreshold | GreaterThanThreshold | GreaterThanUpperThreshold | LessThanLowerOrGreaterThanUpperThreshold | LessThanLowerThreshold | LessThanOrEqualToThreshold | LessThanThreshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,


    /// 
    /// The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state 			from any other state. Each action is specified as an Amazon Resource Name (ARN).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsufficientDataActions")]
    pub insufficient_data_actions: Option<Vec<String>>,


    /// 
    /// The list of actions to execute when this alarm transitions into an ALARM state from any other state.  			Specify each action as an Amazon Resource Name (ARN). For more information about creating alarms and the actions 			that you can specify, see PutMetricAlarm in the 		      Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<Vec<String>>,


    /// 
    /// The value to compare with the specified statistic.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: Option<f64>,


    /// 
    /// The percentile statistic for the metric associated with the alarm. Specify a value between 			p0.0 and p100.
    /// 
    /// For an alarm based on a metric, you must specify either Statistic      or ExtendedStatistic but not both.
    /// 
    /// For an alarm based on a math expression, you can't       specify ExtendedStatistic. Instead, you use Metrics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExtendedStatistic")]
    pub extended_statistic: Option<String>,


    /// 
    /// The dimensions for the metric associated with the alarm. For an alarm based on a math expression, you can't       specify Dimensions. Instead, you use Metrics.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,


    /// 
    /// The number of datapoints that must be breaching to trigger the alarm. 		      This is used only if you are setting an "M out of N" alarm. In that case, this value is the M, 		      and the value that you set for EvaluationPeriods is the N value. 		      For more information, see Evaluating 		      an Alarm in the Amazon CloudWatch User Guide.
    /// 
    /// If you omit this parameter, CloudWatch uses the same value here that you set    for EvaluationPeriods, and the alarm goes to alarm state if that many consecutive     periods are breaching.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatapointsToAlarm")]
    pub datapoints_to_alarm: Option<i64>,


    /// 
    /// The period, in seconds, over which the statistic is applied. This is required for an alarm based on a       metric. Valid values are 10, 30, 60, and any multiple of 60.
    /// 
    /// For an alarm based on a math expression, you can't       specify Period, and instead you use the Metrics parameter.
    /// 
    /// Minimum: 10
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: Option<i64>,

}


/// The MetricDataQuery property type specifies the metric data to return, and whether this call is       just retrieving a batch set of data for one metric, or is performing a math expression on metric data.
///
/// Any expression used must return a single time series. For more information, see Metric Math Syntax and Functions in the Amazon CloudWatch User 				Guide.
#[derive(Default, serde::Serialize)]
pub struct MetricDataQuery {


    /// 
    /// A short name used to tie this object to the results in the response. This name must be 			unique within a single call to GetMetricData. If you are performing math 			expressions on this set of data, this name represents that data and can serve as a 			variable in the mathematical expression. The valid characters are letters, numbers, and 			underscore. The first character must be a lowercase letter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know 			what the value represents. If the metric or expression is shown in a CloudWatch dashboard widget, the label is shown. If Label is omitted, CloudWatch 			generates a default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,


    /// 
    /// The metric to be returned, along with statistics, period, and units. Use this parameter only if this object is retrieving a metric 			and not performing a math expression on returned data.
    /// 
    /// Within one MetricDataQuery object, you must specify either 			Expression or MetricStat but not both.
    /// 
    /// Required: No
    ///
    /// Type: MetricStat
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,


    /// 
    /// The math expression to be performed on the returned data, if this object is performing a math expression. This expression 			can use the Id of the other metrics to refer to those metrics, and can also use the Id of other 			expressions to use the result of those expressions. For more information about metric math expressions, see 			Metric Math Syntax and Functions in the 			Amazon CloudWatch User Guide.
    /// 
    /// Within each MetricDataQuery object, you must specify either 			Expression or MetricStat but not both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    pub expression: Option<String>,


    /// 
    /// The ID of the account where the metrics are located, if this is a cross-account alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,


    /// 
    /// The granularity, in seconds, of the returned data points. For metrics with regular resolution, a 			period can be as short as one minute (60 seconds) and must be a multiple of 60. 			For high-resolution metrics that are collected at intervals of less than one minute, 			the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics are those metrics 			stored by a PutMetricData operation that includes a StorageResolution of 1 second.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: Option<i64>,


    /// 
    /// This option indicates whether to return the 			timestamps and raw data values of this metric.
    /// 
    /// When you create an alarm based on a metric math expression, specify True for    this value for only the one math expression that the alarm is based on. You must specify     False for ReturnData for all the other metrics and expressions    used in the alarm.
    /// 
    /// This field is required.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,

}


/// This structure defines the metric to be returned, along with the statistics, period, and units.
///
/// MetricStat is a property of the       MetricDataQuery property type.
#[derive(Default, serde::Serialize)]
pub struct MetricStat {


    /// 
    /// The unit to use for the returned data points.
    /// 
    /// Valid values are: Seconds, Microseconds, Milliseconds, Bytes, Kilobytes,       Megabytes, Gigabytes, Terabytes, Bits, Kilobits, Megabits, Gigabits, Terabits, Percent, Count,       Bytes/Second, Kilobytes/Second, Megabytes/Second, Gigabytes/Second, Terabytes/Second, Bits/Second,       Kilobits/Second, Megabits/Second, Gigabits/Second, Terabits/Second, Count/Second, or None.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Bits | Bits/Second | Bytes | Bytes/Second | Count | Count/Second | Gigabits | Gigabits/Second | Gigabytes | Gigabytes/Second | Kilobits | Kilobits/Second | Kilobytes | Kilobytes/Second | Megabits | Megabits/Second | Megabytes | Megabytes/Second | Microseconds | Milliseconds | None | Percent | Seconds | Terabits | Terabits/Second | Terabytes | Terabytes/Second
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,


    /// 
    /// The statistic to return. It can include any CloudWatch statistic or extended statistic. 		      For a list of valid values, see the table in 		        Statistics in the Amazon CloudWatch User Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stat")]
    pub stat: String,


    /// 
    /// The metric to return, including the metric name, namespace, and dimensions.
    /// 
    /// Required: Yes
    ///
    /// Type: Metric
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    pub metric: Metric,


    /// 
    /// The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can 			be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected 			at intervals of less than one minute, the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics 			are those metrics stored by a PutMetricData call that includes a StorageResolution of 1 second.
    /// 
    /// If the StartTime parameter specifies a time stamp that is greater than 				3 hours ago, you must specify the period as follows or no data points in that time range is returned:
    /// 
    /// Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).               Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).               Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: i64,

}


/// Dimension is an embedded property of the AWS::CloudWatch::Alarm type. Dimensions       are name/value pairs that can be associated with a CloudWatch metric. You can       specify a maximum of 10 dimensions for a given metric.
#[derive(Default, serde::Serialize)]
pub struct Dimension {


    /// 
    /// The name of the dimension, from 1–255 characters in length. This dimension      name must have been included when      the metric was published.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value for the dimension, from 1–255 characters in length.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// The Metric property type represents a specific metric. Metric is a property of the 		      MetricStat property type.
#[derive(Default, serde::Serialize)]
pub struct Metric {


    /// 
    /// The namespace of the metric that the alarm will watch.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [^:].*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,


    /// 
    /// The metric dimensions that you want to be used for the metric that 		      the alarm will watch.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,


    /// 
    /// The name of the metric that you want the alarm to watch. This is a required field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,

}
