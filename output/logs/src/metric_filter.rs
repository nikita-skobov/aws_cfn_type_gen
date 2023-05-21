

/// The AWS::Logs::MetricFilter resource specifies a metric filter that describes how      CloudWatch Logs extracts information from logs and transforms it into Amazon CloudWatch metrics.     If you have multiple metric filters that are associated with a log group, all the filters are applied to the log streams in that group.
///
/// The maximum number of metric filters that can be associated with a log group is    100.
#[derive(Default, serde::Serialize)]
pub struct CfnMetricFilter {


    /// 
    /// The metric transformations.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricTransformation
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricTransformations")]
    pub metric_transformations: Vec<MetricTransformation>,


    /// 
    /// The name of the metric filter.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [^:*]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterName")]
    pub filter_name: Option<String>,


    /// 
    /// A filter pattern for extracting metric data out of ingested log events. For more information, see       Filter and Pattern Syntax.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: String,


    /// 
    /// The name of an existing log group that you want to associate with this metric filter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\.\-_/#A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,

}


/// MetricTransformation is a property of the AWS::Logs::MetricFilter resource that describes      how to transform log streams into a CloudWatch metric.
#[derive(Default, serde::Serialize)]
pub struct MetricTransformation {


    /// 
    /// The value that is published to the CloudWatch metric. For example, if you're counting the      occurrences of a particular term like Error, specify 1 for the metric value. If you're counting the      number of bytes transferred, reference the value that is in the log event by using $. followed by the name of the      field that you specified in the filter pattern, such as $.size.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricValue")]
    pub metric_value: String,


    /// 
    /// A custom namespace to contain your metric in CloudWatch. Use namespaces to group together metrics       that are similar. For more information, see Namespaces.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: [^:*$]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricNamespace")]
    pub metric_namespace: String,


    /// 
    /// The unit to assign to the metric. If you omit this, the unit is set as None.
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
    /// (Optional) The value to emit when a filter pattern does not match a log event.    This value can be null.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<f64>,


    /// 
    /// The name of the CloudWatch metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The fields to use as dimensions for the metric. One metric filter can include   as many as three dimensions.
    /// 
    /// ImportantMetrics extracted from log events are charged as custom metrics.    To prevent unexpected high charges, do not specify high-cardinality fields such as    IPAddress or requestID as dimensions. Each different value    found for    a dimension is treated as a separate metric and accrues charges as a separate custom metric.   CloudWatch Logs disables a metric filter if it generates 1000 different name/value pairs for your     specified dimensions within a certain amount of time. This helps to prevent accidental high     charges.You can also set up a billing alarm to alert you if your charges are higher than     expected. For more information,     see      Creating a Billing Alarm to Monitor Your Estimated AWS Charges.
    /// 
    /// Required: No
    ///
    /// Type: List of Dimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,

}


/// Specifies the CloudWatch metric dimensions to publish with this metric.
///
/// Because dimensions are part of the unique identifier for a metric, whenever a unique dimension name/value      pair is extracted from your logs, you are creating a new variation of that metric.
///
/// For more information about publishing dimensions with metrics created by metric filters,      see       Publishing dimensions with metrics from values in JSON or space-delimited log events.
#[derive(Default, serde::Serialize)]
pub struct Dimension {


    /// 
    /// The name for the CloudWatch metric dimension that the metric filter creates.
    /// 
    /// Dimension names must contain only ASCII characters, must include at least one non-whitespace character,      and cannot start with a colon (:).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The log event field that will contain the value for this dimension. This dimension will only be      published for a metric if the value is found in the log event. For example, $.eventType for      JSON log events, or $server for space-delimited log events.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}