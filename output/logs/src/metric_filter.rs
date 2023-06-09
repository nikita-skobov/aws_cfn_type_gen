/// The AWS::Logs::MetricFilter resource specifies a metric filter that describes how      CloudWatch Logs extracts information from logs and transforms it into Amazon CloudWatch metrics.     If you have multiple metric filters that are associated with a log group, all the filters are applied to the log streams in that group.
///
/// The maximum number of metric filters that can be associated with a log group is    100.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMetricFilter {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<cfn_resources::StrVal>,

    ///
    /// A filter pattern for extracting metric data out of ingested log events. For more information, see       Filter and Pattern Syntax.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: cfn_resources::StrVal,

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
    pub log_group_name: cfn_resources::StrVal,

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
}

impl cfn_resources::CfnResource for CfnMetricFilter {
    fn type_string(&self) -> &'static str {
        "AWS::Logs::MetricFilter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.filter_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'filter_name'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.filter_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'filter_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.log_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'log_group_name'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.log_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'log_group_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.metric_transformations;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'metric_transformations'. {} is greater than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Specifies the CloudWatch metric dimensions to publish with this metric.
///
/// Because dimensions are part of the unique identifier for a metric, whenever a unique dimension name/value      pair is extracted from your logs, you are creating a new variation of that metric.
///
/// For more information about publishing dimensions with metrics created by metric filters,      see       Publishing dimensions with metrics from values in JSON or space-delimited log events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub key: cfn_resources::StrVal,

    ///
    /// The log event field that will contain the value for this dimension. This dimension will only be      published for a metric if the value is found in the log event. For example, $.eventType for      JSON log events, or $server for space-delimited log events.
    ///
    /// Required: Yes
    ///
    /// Type: String
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
        Ok(())
    }
}

/// MetricTransformation is a property of the AWS::Logs::MetricFilter resource that describes      how to transform log streams into a CloudWatch metric.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricTransformation {
    ///
    /// (Optional) The value to emit when a filter pattern does not match a log event.    This value can be null.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,

    ///
    /// The name of the CloudWatch metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

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
    pub metric_namespace: cfn_resources::StrVal,

    ///
    /// The value that is published to the CloudWatch metric. For example, if you're counting the      occurrences of a particular term like Error, specify 1 for the metric value. If you're counting the      number of bytes transferred, reference the value that is in the log event by using $. followed by the name of the      field that you specified in the filter pattern, such as $.size.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricValue")]
    pub metric_value: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<MetricTransformationUnitEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MetricTransformationUnitEnum {
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

impl Default for MetricTransformationUnitEnum {
    fn default() -> Self {
        MetricTransformationUnitEnum::Bits
    }
}

impl cfn_resources::CfnResource for MetricTransformation {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.metric_namespace;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'metric_namespace'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
