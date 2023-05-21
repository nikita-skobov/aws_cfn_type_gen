

/// The AWS::CloudWatch::AnomalyDetector type specifies an anomaly detection band for a certain metric and statistic. The band     represents the expected "normal" range for the metric values. Anomaly detection bands can be used for visualization of a metric's expected values,     and for alarms.
#[derive(Default, serde::Serialize)]
pub struct CfnAnomalyDetector {


    /// 
    /// The CloudWatch metric and statistic for this anomaly detector.
    /// 
    /// Required: No
    ///
    /// Type: SingleMetricAnomalyDetector
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleMetricAnomalyDetector")]
    pub single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,


    /// 
    /// The name of the metric associated with the anomaly detection band.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,


    /// 
    /// The statistic of the metric associated with the anomaly detection band.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Stat")]
    pub stat: Option<String>,


    /// 
    /// The dimensions of the metric associated with the anomaly detection band.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Update requires: Replacement
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,


    /// 
    /// The CloudWatch metric math expression for this anomaly detector.
    /// 
    /// Required: No
    ///
    /// Type: MetricMathAnomalyDetector
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricMathAnomalyDetector")]
    pub metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,


    /// 
    /// Specifies details about how the anomaly detection model is to be trained, including time ranges to exclude     when training and updating the model. The configuration can also include the time zone to use for the metric.
    /// 
    /// Required: No
    ///
    /// Type: Configuration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: Option<Configuration>,


    /// 
    /// The namespace of the metric associated with the anomaly detection band.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

}


/// Indicates the CloudWatch math expression that provides the time series the anomaly detector 			uses as input. 			The designated math expression must return a single time series.
#[derive(Default, serde::Serialize)]
pub struct MetricMathAnomalyDetector {


    /// 
    /// An array of metric data query structures 			that enables you to create an anomaly detector 			based on the result of a metric math expression. 			Each item in MetricDataQueries gets a metric or performs a math expression. 			One item in MetricDataQueries is the expression 			that provides the time series 			that the anomaly detector uses as input. 			Designate the expression by setting ReturnData to true 			for this object in the array. 			For all other expressions and metrics, set ReturnData to false. 			The designated expression must return 			a single time series.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDataQuery
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,

}


/// A dimension is a name/value pair that is part of the identity of a metric. Because dimensions are part of the unique 			identifier for a metric, whenever you add a unique name/value pair to one of 			your metrics, you are creating a new variation of that metric. For example, many Amazon EC2 metrics publish 		InstanceId as a dimension name, and the actual instance ID as the value for that dimension.
///
/// You 		can assign up to 30 dimensions to a metric.
#[derive(Default, serde::Serialize)]
pub struct Dimension {


    /// 
    /// The value of the dimension. Dimension values must contain only ASCII characters and must include 			at least one non-whitespace character. ASCII 			control characters are not supported as part of dimension values.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The name of the dimension.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// Represents a specific metric.
#[derive(Default, serde::Serialize)]
pub struct Metric {


    /// 
    /// The dimensions for the metric.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 30
    ///
    /// Update requires: Replacement
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,


    /// 
    /// The name of the metric. This is a required field.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The namespace of the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [^:].*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    pub namespace: String,

}


/// This structure defines the metric to be returned, along with the statistics, period, and units.
#[derive(Default, serde::Serialize)]
pub struct MetricStat {


    /// 
    /// The metric to return, including the metric name, namespace, and dimensions.
    /// 
    /// Required: Yes
    ///
    /// Type: Metric
    ///
    /// Update requires: Replacement
    #[serde(rename = "Metric")]
    pub metric: Metric,


    /// 
    /// The statistic to return. It can include any CloudWatch statistic or extended statistic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Stat")]
    pub stat: String,


    /// 
    /// When you are using a Put operation, this defines what unit you want to use when storing the metric.
    /// 
    /// In a Get operation, if you omit Unit then all data that was collected with any unit is returned, along with the corresponding units that were specified 			when the data was reported to CloudWatch. If you specify a unit, the operation returns only data that was collected with that unit specified. 			If you specify a unit that does not match the data collected, the results of the operation are null. CloudWatch does not perform unit conversions.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Bits | Bits/Second | Bytes | Bytes/Second | Count | Count/Second | Gigabits | Gigabits/Second | Gigabytes | Gigabytes/Second | Kilobits | Kilobits/Second | Kilobytes | Kilobytes/Second | Megabits | Megabits/Second | Megabytes | Megabytes/Second | Microseconds | Milliseconds | None | Percent | Seconds | Terabits | Terabits/Second | Terabytes | Terabytes/Second
    ///
    /// Update requires: Replacement
    #[serde(rename = "Unit")]
    pub unit: Option<String>,


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
    /// Update requires: Replacement
    #[serde(rename = "Period")]
    pub period: i64,

}


/// Specifies details about how the anomaly detection model is to be trained, including time ranges to exclude         when training and updating the model. The configuration can also include the time zone to use for the metric.
#[derive(Default, serde::Serialize)]
pub struct Configuration {


    /// 
    /// The time zone to use for the metric. This is useful to enable the model to automatically account for daylight savings       time changes if the metric is sensitive to such time changes.
    /// 
    /// To specify a time zone, use the name of the time zone as specified in the standard tz database. For more information,         see tz database.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricTimeZone")]
    pub metric_time_zone: Option<String>,


    /// 
    /// Specifies an array of time ranges to exclude from use when the anomaly detection model is trained and updated.       Use this to make sure that events that could cause unusual values for the metric, such as deployments, aren't used when       CloudWatch creates or updates the model.
    /// 
    /// Required: No
    ///
    /// Type: List of Range
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedTimeRanges")]
    pub excluded_time_ranges: Option<Vec<Range>>,

}


/// Each Range specifies one range of days or times to exclude from use for training or updating an     anomaly detection model.
#[derive(Default, serde::Serialize)]
pub struct Range {


    /// 
    /// The start time of the range to exclude. The format is yyyy-MM-dd'T'HH:mm:ss. For example,         2019-07-01T23:59:59.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: String,


    /// 
    /// The end time of the range to exclude. The format is yyyy-MM-dd'T'HH:mm:ss. For example,         2019-07-01T23:59:59.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    pub end_time: String,

}


/// An array of metric data query structures 			that enables you to create an anomaly detector 			based on the result of a metric math expression. 			Each item in MetricDataQueries gets a metric or performs a math expression. 			One item in MetricDataQueries is the expression 			that provides the time series 			that the anomaly detector uses as input. 			Designate the expression by setting ReturnData to true 			for this object in the array. 			For all other expressions and metrics, set ReturnData to false. 			The designated expression must return 			a single time series.
#[derive(Default, serde::Serialize)]
pub struct MetricDataQueries {

}


/// This structure is used in both GetMetricData and PutMetricAlarm. The supported 			use of this structure is different for those two operations.
///
/// When used in GetMetricData, it indicates the metric data to return, and whether this call is just retrieving 			a batch set of data for one metric, or is performing a Metrics Insights query or a math expression. A 			single GetMetricData call can include up to 500 MetricDataQuery 			structures.
///
/// When used in PutMetricAlarm, it enables you to create an alarm based on a 			metric math expression. Each MetricDataQuery in the array specifies either 			a metric to retrieve, or a math expression to be performed on retrieved metrics. A 			single PutMetricAlarm call can include up to 20 				MetricDataQuery structures in the array. The 20 structures can include 			as many as 10 structures that contain a MetricStat parameter to retrieve a 			metric, and as many as 10 structures that contain the Expression parameter 			to perform a math expression. Of those Expression structures, one must have true 		as the value for ReturnData. The result of this expression is the value the alarm watches.
///
/// Any expression used in a PutMetricAlarm 			operation must return a single time series. For more information, see Metric Math Syntax and Functions in the Amazon CloudWatch User 				Guide.
///
/// Some of the parameters of this structure also have different uses whether you are using this structure in a GetMetricData 			operation or a PutMetricAlarm operation. These differences are explained in the following parameter list.
#[derive(Default, serde::Serialize)]
pub struct MetricDataQuery {


    /// 
    /// A human-readable label for this metric or expression. This is especially useful 			if this is an expression, so that you know 			what the value represents. If the metric or expression is shown in a 			CloudWatch dashboard widget, the label is shown. If Label is omitted, CloudWatch 			generates a default.
    /// 
    /// You can put dynamic expressions into a label, so that it is more descriptive. 			For more information, see Using Dynamic Labels.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,


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
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// When used in GetMetricData, this option indicates whether to return the 			timestamps and raw data values of this metric. If you are performing this call just to 			do math expressions and do not also need the raw data returned, you can specify 				false. If you omit this, the default of true is 			used.
    /// 
    /// When used in PutMetricAlarm, specify true for the one expression result to use as the alarm. For all 		other metrics and expressions in the same PutMetricAlarm operation, specify ReturnData as False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,


    /// 
    /// This field can contain either a Metrics Insights query, or a metric math expression to be performed on the 			returned data. For more information about Metrics Insights queries, see 			Metrics Insights query components and syntax in the 			Amazon CloudWatch User Guide.
    /// 
    /// A math expression 			can use the Id of the other metrics or queries to refer to those metrics, and can also use 			the Id of other 			expressions to use the result of those expressions. For more information about metric math expressions, see 			Metric Math Syntax and Functions in the 			Amazon CloudWatch User Guide.
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
    /// Update requires: Replacement
    #[serde(rename = "Expression")]
    pub expression: Option<String>,


    /// 
    /// The metric to be returned, along with statistics, period, and units. Use this parameter only if this object is retrieving a metric 			and not performing a math expression on returned data.
    /// 
    /// Within one MetricDataQuery object, you must specify either 			Expression or MetricStat but not both.
    /// 
    /// Required: No
    ///
    /// Type: MetricStat
    ///
    /// Update requires: Replacement
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,


    /// 
    /// The granularity, in seconds, of the returned data points. For metrics with regular resolution, a 			period can be as short as one minute (60 seconds) and must be a multiple of 60. 			For high-resolution metrics that are collected at intervals of less than one minute, 			the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics are those metrics 			stored by a PutMetricData operation that includes a StorageResolution of 1 second.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Period")]
    pub period: Option<i64>,


    /// 
    /// The ID of the account where the metrics are located.
    /// 
    /// If you are performing a GetMetricData operation in a monitoring account, use this to specify 			which account to retrieve this metric from.
    /// 
    /// If you are performing a PutMetricAlarm operation, use this to specify 			which account contains the metric that the alarm is watching.
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
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,

}


/// Designates the CloudWatch metric and statistic that provides the time series the anomaly detector 			uses as input.
#[derive(Default, serde::Serialize)]
pub struct SingleMetricAnomalyDetector {


    /// 
    /// The namespace of the metric to create the anomaly detection model for.
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
    /// Update requires: Replacement
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,


    /// 
    /// The metric dimensions to create the anomaly detection model for.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Maximum: 30
    ///
    /// Update requires: Replacement
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,


    /// 
    /// The name of the metric to create the anomaly detection model for.
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
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,


    /// 
    /// The statistic to use for the metric and anomaly detection model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 50
    ///
    /// Pattern: (SampleCount|Average|Sum|Minimum|Maximum|IQM|(p|tc|tm|ts|wm)(\d{1,2}(\.\d{0,10})?|100)|[ou]\d+(\.\d*)?)(_E|_L|_H)?|(TM|TC|TS|WM)\(((((\d{1,2})(\.\d{0,10})?|100(\.0{0,10})?)%)?:((\d{1,2})(\.\d{0,10})?|100(\.0{0,10})?)%|((\d{1,2})(\.\d{0,10})?|100(\.0{0,10})?)%:(((\d{1,2})(\.\d{0,10})?|100(\.0{0,10})?)%)?)\)|(TM|TC|TS|WM|PR)\(((\d+(\.\d{0,10})?|(\d+(\.\d{0,10})?[Ee][+-]?\d+)):((\d+(\.\d{0,10})?|(\d+(\.\d{0,10})?[Ee][+-]?\d+)))?|((\d+(\.\d{0,10})?|(\d+(\.\d{0,10})?[Ee][+-]?\d+)))?:(\d+(\.\d{0,10})?|(\d+(\.\d{0,10})?[Ee][+-]?\d+)))\)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Stat")]
    pub stat: Option<String>,

}