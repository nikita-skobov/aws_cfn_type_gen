

/// Use the AWS::IoT::SecurityProfile resource to create a Device Defender     security profile. For API reference, see CreateSecurityProfile and for general information, see Detect.
#[derive(Default, serde::Serialize)]
pub struct CfnSecurityProfile {


    /// 
    /// Specifies the behaviors that, when violated by a device (thing), cause an alert.
    /// 
    /// Required: No
    ///
    /// Type: List of Behavior
    ///
    /// Update requires: No interruption
    #[serde(rename = "Behaviors")]
    pub behaviors: Option<Vec<Behavior>>,


    /// 
    /// The ARN of the target (thing group) to which the security profile is attached.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArns")]
    pub target_arns: Option<Vec<String>>,


    /// 
    /// The name you gave to the security profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityProfileName")]
    pub security_profile_name: Option<String>,


    /// 
    /// A description of the security profile.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityProfileDescription")]
    pub security_profile_description: Option<String>,


    /// 
    /// Specifies the destinations to which alerts are sent. (Alerts are always sent to the      console.) Alerts are generated when a device (thing) violates a behavior.
    /// 
    /// Required: No
    ///
    /// Type: Map of AlertTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlertTargets")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,


    /// 
    /// A list of metrics whose data is retained (stored). By default, data is retained for any     metric used in the profile's behaviors, but it's also retained for any metric     specified here. Can be used with custom metrics; can't be used with dimensions.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricToRetain
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalMetricsToRetainV2")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,


    /// 
    /// Metadata that can be used to manage the security profile.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// A statistical ranking (percentile) that    indicates a threshold value by which a behavior is determined to be in compliance or in    violation of the behavior.
#[derive(Default, serde::Serialize)]
pub struct StatisticalThreshold {


    /// 
    /// The percentile that    resolves to a threshold value by which compliance with a behavior is determined. Metrics are    collected over the specified period (durationSeconds) from all reporting devices    in your account and statistical ranks are calculated. Then, the measurements from a device are    collected over the same period. If the accumulated measurements from the device fall above or    below (comparisonOperator) the value associated with the percentile specified,    then the device is considered to be in compliance with the behavior, otherwise a violation    occurs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,

}


/// A Device Defender security profile behavior.
#[derive(Default, serde::Serialize)]
pub struct Behavior {


    /// 
    /// The criteria that determine if a device is behaving normally in regard to      the metric.
    /// 
    /// Required: No
    ///
    /// Type: BehaviorCriteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "Criteria")]
    pub criteria: Option<BehaviorCriteria>,


    /// 
    /// The alert status. If you set the value to true, alerts will be     suppressed.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuppressAlerts")]
    pub suppress_alerts: Option<bool>,


    /// 
    /// The name    you've given to the behavior.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The dimension of the metric.
    /// 
    /// Required: No
    ///
    /// Type: MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDimension")]
    pub metric_dimension: Option<MetricDimension>,


    /// 
    /// What is measured by the behavior.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    pub metric: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// The dimension of the metric.
#[derive(Default, serde::Serialize)]
pub struct MetricDimension {


    /// 
    /// Operators are constructs that perform logical operations. Valid values are IN and NOT_IN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    pub operator: Option<String>,


    /// 
    /// The name of the dimension.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,

}


/// The criteria by which the behavior is determined to be normal.
#[derive(Default, serde::Serialize)]
pub struct BehaviorCriteria {


    /// 
    /// If an alarm has occurred and the offending device is no longer in violation of the behavior      for the specified number of consecutive datapoints, the alarm is cleared. If not specified,      the default is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsecutiveDatapointsToClear")]
    pub consecutive_datapoints_to_clear: Option<i64>,


    /// 
    /// Use this to specify the time duration over which the behavior is evaluated, for those criteria that    have a time dimension (for example, NUM_MESSAGES_SENT). For a     statisticalThreshhold metric comparison, measurements from all devices are    accumulated over this time duration before being used to calculate percentiles, and later,    measurements from an individual device are also accumulated over this time duration before    being given a percentile rank. Cannot be used with list-based metric datatypes.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationSeconds")]
    pub duration_seconds: Option<i64>,


    /// 
    /// The confidence level of the detection model.
    /// 
    /// Required: No
    ///
    /// Type: MachineLearningDetectionConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "MlDetectionConfig")]
    pub ml_detection_config: Option<MachineLearningDetectionConfig>,


    /// 
    /// The value to be compared with the metric.
    /// 
    /// Required: No
    ///
    /// Type: MetricValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<MetricValue>,


    /// 
    /// A statistical ranking (percentile)that    indicates a threshold value by which a behavior is determined to be in compliance or in    violation of the behavior.
    /// 
    /// Required: No
    ///
    /// Type: StatisticalThreshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatisticalThreshold")]
    pub statistical_threshold: Option<StatisticalThreshold>,


    /// 
    /// If a device is in violation of the behavior for the specified number of consecutive      datapoints, an alarm occurs. If not specified, the default is 1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsecutiveDatapointsToAlarm")]
    pub consecutive_datapoints_to_alarm: Option<i64>,


    /// 
    /// The operator that relates the thing measured (metric) to the criteria      (containing a value or statisticalThreshold). Valid operators include:
    /// 
    /// string-list: in-set and not-in-set                                number-list: in-set and not-in-set                                ip-address-list: in-cidr-set and not-in-cidr-set                                number: less-than, less-than-equals, greater-than, and greater-than-equals
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: Option<String>,

}


/// The MachineLearningDetectionConfig property type controls confidence of the machine learning model.
#[derive(Default, serde::Serialize)]
pub struct MachineLearningDetectionConfig {


    /// 
    /// The model confidence level.
    /// 
    /// There are three levels of confidence, "high", "medium", and "low".
    /// 
    /// The higher the confidence level, the lower the sensitivity, and the lower the alarm     frequency will be.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfidenceLevel")]
    pub confidence_level: Option<String>,

}


/// A structure containing the alert target ARN and the role ARN.
#[derive(Default, serde::Serialize)]
pub struct AlertTarget {


    /// 
    /// The ARN of the role that grants permission to send alerts to the     notification target.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon Resource Name (ARN) of the notification target to which alerts are sent.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlertTargetArn")]
    pub alert_target_arn: String,

}


/// The value to be compared with the metric.
#[derive(Default, serde::Serialize)]
pub struct MetricValue {


    /// 
    /// The string values of a metric.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Strings")]
    pub strings: Option<Vec<String>>,


    /// 
    /// The numeric value of a metric.
    /// 
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Numbers")]
    pub numbers: Option<Vec<f64>>,


    /// 
    /// The numeric values of a metric.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Number")]
    pub number: Option<f64>,


    /// 
    /// If the comparisonOperator calls for a set of ports, use this      to specify that set to be compared with the metric.
    /// 
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<i64>>,


    /// 
    /// If the comparisonOperator calls for a numeric value, use this      to specify that numeric value to be compared with the metric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    pub count: Option<String>,


    /// 
    /// If the comparisonOperator calls for a set of CIDRs, use this      to specify that set to be compared with the metric.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidrs")]
    pub cidrs: Option<Vec<String>>,

}


/// The metric you want to retain. Dimensions are optional.
#[derive(Default, serde::Serialize)]
pub struct MetricToRetain {


    /// 
    /// The dimension of the metric.
    /// 
    /// Required: No
    ///
    /// Type: MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDimension")]
    pub metric_dimension: Option<MetricDimension>,


    /// 
    /// A standard of measurement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    pub metric: String,

}