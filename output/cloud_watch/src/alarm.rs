/// The AWS::CloudWatch::Alarm type specifies an alarm and associates it with the specified metric or metric math expression.
///
/// When this operation creates an alarm, the alarm state is immediately set to 			INSUFFICIENT_DATA. The alarm is then evaluated and its state is set 			appropriately. Any actions associated with the new state are then executed.
///
/// When you update an existing alarm, its state is left unchanged, but the 			update completely overwrites the previous configuration of the alarm.
///
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<cfn_resources::StrVal>,

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
    pub comparison_operator: AlarmComparisonOperatorEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricDataQuery>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okactions: Option<Vec<String>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<AlarmStatisticEnum>,

    ///
    /// The value to compare with the specified statistic.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_metric_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<AlarmUnitEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnAlarmarn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AlarmComparisonOperatorEnum {
    /// GreaterThanOrEqualToThreshold
    #[serde(rename = "GreaterThanOrEqualToThreshold")]
    Greaterthanorequaltothreshold,

    /// GreaterThanThreshold
    #[serde(rename = "GreaterThanThreshold")]
    Greaterthanthreshold,

    /// GreaterThanUpperThreshold
    #[serde(rename = "GreaterThanUpperThreshold")]
    Greaterthanupperthreshold,

    /// LessThanLowerOrGreaterThanUpperThreshold
    #[serde(rename = "LessThanLowerOrGreaterThanUpperThreshold")]
    Lessthanlowerorgreaterthanupperthreshold,

    /// LessThanLowerThreshold
    #[serde(rename = "LessThanLowerThreshold")]
    Lessthanlowerthreshold,

    /// LessThanOrEqualToThreshold
    #[serde(rename = "LessThanOrEqualToThreshold")]
    Lessthanorequaltothreshold,

    /// LessThanThreshold
    #[serde(rename = "LessThanThreshold")]
    Lessthanthreshold,
}

impl Default for AlarmComparisonOperatorEnum {
    fn default() -> Self {
        AlarmComparisonOperatorEnum::Greaterthanorequaltothreshold
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AlarmStatisticEnum {
    /// Average
    #[serde(rename = "Average")]
    Average,

    /// Maximum
    #[serde(rename = "Maximum")]
    Maximum,

    /// Minimum
    #[serde(rename = "Minimum")]
    Minimum,

    /// SampleCount
    #[serde(rename = "SampleCount")]
    Samplecount,

    /// Sum
    #[serde(rename = "Sum")]
    Sum,
}

impl Default for AlarmStatisticEnum {
    fn default() -> Self {
        AlarmStatisticEnum::Average
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum AlarmUnitEnum {
    /// Bits
    #[serde(rename = "Bits")]
    Bits,

    /// Bits/Second
    #[serde(rename = "Bits/Second")]
    Bitssecond,

    /// Bytes
    #[serde(rename = "Bytes")]
    Bytes,

    /// Bytes/Second
    #[serde(rename = "Bytes/Second")]
    Bytessecond,

    /// Count
    #[serde(rename = "Count")]
    Count,

    /// Count/Second
    #[serde(rename = "Count/Second")]
    Countsecond,

    /// Gigabits
    #[serde(rename = "Gigabits")]
    Gigabits,

    /// Gigabits/Second
    #[serde(rename = "Gigabits/Second")]
    Gigabitssecond,

    /// Gigabytes
    #[serde(rename = "Gigabytes")]
    Gigabytes,

    /// Gigabytes/Second
    #[serde(rename = "Gigabytes/Second")]
    Gigabytessecond,

    /// Kilobits
    #[serde(rename = "Kilobits")]
    Kilobits,

    /// Kilobits/Second
    #[serde(rename = "Kilobits/Second")]
    Kilobitssecond,

    /// Kilobytes
    #[serde(rename = "Kilobytes")]
    Kilobytes,

    /// Kilobytes/Second
    #[serde(rename = "Kilobytes/Second")]
    Kilobytessecond,

    /// Megabits
    #[serde(rename = "Megabits")]
    Megabits,

    /// Megabits/Second
    #[serde(rename = "Megabits/Second")]
    Megabitssecond,

    /// Megabytes
    #[serde(rename = "Megabytes")]
    Megabytes,

    /// Megabytes/Second
    #[serde(rename = "Megabytes/Second")]
    Megabytessecond,

    /// Microseconds
    #[serde(rename = "Microseconds")]
    Microseconds,

    /// Milliseconds
    #[serde(rename = "Milliseconds")]
    Milliseconds,

    /// None
    #[serde(rename = "None")]
    None,

    /// Percent
    #[serde(rename = "Percent")]
    Percent,

    /// Seconds
    #[serde(rename = "Seconds")]
    Seconds,

    /// Terabits
    #[serde(rename = "Terabits")]
    Terabits,

    /// Terabits/Second
    #[serde(rename = "Terabits/Second")]
    Terabitssecond,

    /// Terabytes
    #[serde(rename = "Terabytes")]
    Terabytes,

    /// Terabytes/Second
    #[serde(rename = "Terabytes/Second")]
    Terabytessecond,
}

impl Default for AlarmUnitEnum {
    fn default() -> Self {
        AlarmUnitEnum::Bits
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAlarmarn;
impl CfnAlarmarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnAlarm {
    fn type_string(&self) -> &'static str {
        "AWS::CloudWatch::Alarm"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.alarm_actions {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'alarm_actions'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.alarm_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'alarm_description'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.alarm_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'alarm_description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'alarm_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'alarm_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.datapoints_to_alarm {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'datapoints_to_alarm'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.dimensions {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'dimensions'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.evaluate_low_sample_count_percentile {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'evaluate_low_sample_count_percentile'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.evaluate_low_sample_count_percentile {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'evaluate_low_sample_count_percentile'. {} is less than 1", s.len()));
                }
            }
        }

        let the_val = &self.evaluation_periods;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'evaluation_periods'. {} is less than 1",
                the_val
            ));
        }

        if let Some(the_val) = &self.insufficient_data_actions {
            if the_val.len() > 5 as _ {
                return Err(format!("Max validation failed on field 'insufficient_data_actions'. {} is greater than 5", the_val.len()));
            }
        }

        if let Some(the_val) = &self.metric_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'metric_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.metric_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'metric_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'namespace'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'namespace'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.okactions {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'okactions'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.period {
            if *the_val < 10 as _ {
                return Err(format!(
                    "Min validation failed on field 'period'. {} is less than 10",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.threshold_metric_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'threshold_metric_id'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.threshold_metric_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'threshold_metric_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.treat_missing_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'treat_missing_data'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.treat_missing_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'treat_missing_data'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Dimension is an embedded property of the AWS::CloudWatch::Alarm type. Dimensions       are name/value pairs that can be associated with a CloudWatch metric. You can       specify a maximum of 10 dimensions for a given metric.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub name: cfn_resources::StrVal,

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
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Dimension {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The Metric property type represents a specific metric. Metric is a property of the 		      MetricStat property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Metric {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Metric {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.dimensions {
            if the_val.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'dimensions'. {} is greater than 30",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.metric_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'metric_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.metric_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'metric_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'namespace'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'namespace'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The MetricDataQuery property type specifies the metric data to return, and whether this call is       just retrieving a batch set of data for one metric, or is performing a math expression on metric data.
///
/// Any expression used must return a single time series. For more information, see Metric Math Syntax and Functions in the Amazon CloudWatch User 				Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricDataQuery {
    ///
    /// The ID of the account where the metrics are located, if this is a cross-account alarm.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<cfn_resources::StrVal>,

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
    pub id: cfn_resources::StrVal,

    ///
    /// A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know 			what the value represents. If the metric or expression is shown in a CloudWatch dashboard widget, the label is shown. If Label is omitted, CloudWatch 			generates a default.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

impl cfn_resources::CfnResource for MetricDataQuery {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.expression {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'expression'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.expression {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'expression'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'id'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.metric_stat
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.period {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'period'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// This structure defines the metric to be returned, along with the statistics, period, and units.
///
/// MetricStat is a property of the       MetricDataQuery property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricStat {
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

    ///
    /// The statistic to return. It can include any CloudWatch statistic or extended statistic. 		      For a list of valid values, see the table in 		        Statistics in the Amazon CloudWatch User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stat")]
    pub stat: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<MetricStatUnitEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MetricStatUnitEnum {
    /// Bits
    #[serde(rename = "Bits")]
    Bits,

    /// Bits/Second
    #[serde(rename = "Bits/Second")]
    Bitssecond,

    /// Bytes
    #[serde(rename = "Bytes")]
    Bytes,

    /// Bytes/Second
    #[serde(rename = "Bytes/Second")]
    Bytessecond,

    /// Count
    #[serde(rename = "Count")]
    Count,

    /// Count/Second
    #[serde(rename = "Count/Second")]
    Countsecond,

    /// Gigabits
    #[serde(rename = "Gigabits")]
    Gigabits,

    /// Gigabits/Second
    #[serde(rename = "Gigabits/Second")]
    Gigabitssecond,

    /// Gigabytes
    #[serde(rename = "Gigabytes")]
    Gigabytes,

    /// Gigabytes/Second
    #[serde(rename = "Gigabytes/Second")]
    Gigabytessecond,

    /// Kilobits
    #[serde(rename = "Kilobits")]
    Kilobits,

    /// Kilobits/Second
    #[serde(rename = "Kilobits/Second")]
    Kilobitssecond,

    /// Kilobytes
    #[serde(rename = "Kilobytes")]
    Kilobytes,

    /// Kilobytes/Second
    #[serde(rename = "Kilobytes/Second")]
    Kilobytessecond,

    /// Megabits
    #[serde(rename = "Megabits")]
    Megabits,

    /// Megabits/Second
    #[serde(rename = "Megabits/Second")]
    Megabitssecond,

    /// Megabytes
    #[serde(rename = "Megabytes")]
    Megabytes,

    /// Megabytes/Second
    #[serde(rename = "Megabytes/Second")]
    Megabytessecond,

    /// Microseconds
    #[serde(rename = "Microseconds")]
    Microseconds,

    /// Milliseconds
    #[serde(rename = "Milliseconds")]
    Milliseconds,

    /// None
    #[serde(rename = "None")]
    None,

    /// Percent
    #[serde(rename = "Percent")]
    Percent,

    /// Seconds
    #[serde(rename = "Seconds")]
    Seconds,

    /// Terabits
    #[serde(rename = "Terabits")]
    Terabits,

    /// Terabits/Second
    #[serde(rename = "Terabits/Second")]
    Terabitssecond,

    /// Terabytes
    #[serde(rename = "Terabytes")]
    Terabytes,

    /// Terabytes/Second
    #[serde(rename = "Terabytes/Second")]
    Terabytessecond,
}

impl Default for MetricStatUnitEnum {
    fn default() -> Self {
        MetricStatUnitEnum::Bits
    }
}

impl cfn_resources::CfnResource for MetricStat {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.metric.validate()?;

        let the_val = &self.period;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'period'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}
