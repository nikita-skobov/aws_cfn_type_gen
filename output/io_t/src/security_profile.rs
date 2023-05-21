/// Use the AWS::IoT::SecurityProfile resource to create a Device Defender     security profile. For API reference, see CreateSecurityProfile and for general information, see Detect.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecurityProfile {
    ///
    /// A list of metrics whose data is retained (stored). By default, data is retained for any     metric used in the profile's behaviors, but it's also retained for any metric     specified here. Can be used with custom metrics; can't be used with dimensions.
    ///
    /// Required: No
    ///
    /// Type: List of MetricToRetain
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalMetricsToRetainV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,

    ///
    /// Specifies the destinations to which alerts are sent. (Alerts are always sent to the      console.) Alerts are generated when a device (thing) violates a behavior.
    ///
    /// Required: No
    ///
    /// Type: Map of AlertTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlertTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,

    ///
    /// Specifies the behaviors that, when violated by a device (thing), cause an alert.
    ///
    /// Required: No
    ///
    /// Type: List of Behavior
    ///
    /// Update requires: No interruption
    #[serde(rename = "Behaviors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,

    ///
    /// A description of the security profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityProfileDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_description: Option<cfn_resources::StrVal>,

    ///
    /// The name you gave to the security profile.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<cfn_resources::StrVal>,

    ///
    /// Metadata that can be used to manage the security profile.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ARN of the target (thing group) to which the security profile is attached.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arns: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CfnSecurityProfile {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::SecurityProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A structure containing the alert target ARN and the role ARN.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlertTarget {
    ///
    /// The Amazon Resource Name (ARN) of the notification target to which alerts are sent.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlertTargetArn")]
    pub alert_target_arn: cfn_resources::StrVal,

    ///
    /// The ARN of the role that grants permission to send alerts to the     notification target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AlertTarget {
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

/// A Device Defender security profile behavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<BehaviorCriteria>,

    ///
    /// What is measured by the behavior.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<cfn_resources::StrVal>,

    ///
    /// The dimension of the metric.
    ///
    /// Required: No
    ///
    /// Type: MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimension: Option<MetricDimension>,

    ///
    /// The name    you've given to the behavior.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The alert status. If you set the value to true, alerts will be     suppressed.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuppressAlerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_alerts: Option<bool>,
}

impl cfn_resources::CfnResource for Behavior {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.criteria
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.metric_dimension
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The criteria by which the behavior is determined to be normal.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BehaviorCriteria {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<cfn_resources::StrVal>,

    ///
    /// If a device is in violation of the behavior for the specified number of consecutive      datapoints, an alarm occurs. If not specified, the default is 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsecutiveDatapointsToAlarm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_datapoints_to_alarm: Option<i64>,

    ///
    /// If an alarm has occurred and the offending device is no longer in violation of the behavior      for the specified number of consecutive datapoints, the alarm is cleared. If not specified,      the default is 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConsecutiveDatapointsToClear")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_detection_config: Option<MachineLearningDetectionConfig>,

    ///
    /// A statistical ranking (percentile)that    indicates a threshold value by which a behavior is determined to be in compliance or in    violation of the behavior.
    ///
    /// Required: No
    ///
    /// Type: StatisticalThreshold
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatisticalThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistical_threshold: Option<StatisticalThreshold>,

    ///
    /// The value to be compared with the metric.
    ///
    /// Required: No
    ///
    /// Type: MetricValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MetricValue>,
}

impl cfn_resources::CfnResource for BehaviorCriteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.ml_detection_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.statistical_threshold
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.value.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The MachineLearningDetectionConfig property type controls confidence of the machine learning model.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MachineLearningDetectionConfig {
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

/// The dimension of the metric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDimension {
    ///
    /// The name of the dimension.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DimensionName")]
    pub dimension_name: cfn_resources::StrVal,

    ///
    /// Operators are constructs that perform logical operations. Valid values are IN and NOT_IN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for MetricDimension {
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

/// The metric you want to retain. Dimensions are optional.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricToRetain {
    ///
    /// A standard of measurement.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    pub metric: cfn_resources::StrVal,

    ///
    /// The dimension of the metric.
    ///
    /// Required: No
    ///
    /// Type: MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimension: Option<MetricDimension>,
}

impl cfn_resources::CfnResource for MetricToRetain {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.metric_dimension
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The value to be compared with the metric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricValue {
    ///
    /// If the comparisonOperator calls for a set of CIDRs, use this      to specify that set to be compared with the metric.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,

    ///
    /// If the comparisonOperator calls for a numeric value, use this      to specify that numeric value to be compared with the metric.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<cfn_resources::StrVal>,

    ///
    /// The numeric values of a metric.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,

    ///
    /// The numeric value of a metric.
    ///
    /// Required: No
    ///
    /// Type: List of Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Numbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbers: Option<Vec<f64>>,

    ///
    /// If the comparisonOperator calls for a set of ports, use this      to specify that set to be compared with the metric.
    ///
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,

    ///
    /// The string values of a metric.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Strings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for MetricValue {
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

/// A statistical ranking (percentile) that    indicates a threshold value by which a behavior is determined to be in compliance or in    violation of the behavior.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StatisticalThreshold {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
