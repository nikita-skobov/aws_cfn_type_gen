/// Creates or updates a metric stream. Metrics streams can automatically stream CloudWatch metrics to     AWS destinations including Amazon S3 and to many third-party solutions. For more information, see          Metric streams.
///
/// To create a metric stream, you must be logged on to an account that has the iam:PassRole permission and     either the CloudWatchFullAccess policy or the cloudwatch:PutMetricStream permission.
///
/// When you create or update a metric stream, you choose one of the following:
///
/// When you create a metric stream, the stream is created in the running state. If you update an existing metric stream,     the state does not change.
///
/// If you create a metric stream in an account that has been set up as a monitoring account in CloudWatch cross-account observability,   you can choose whether to include metrics from linked source accounts in the metric stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMetricStream {
    ///
    /// If you specify this parameter, the stream sends metrics from all metric namespaces except       for the namespaces that you specify here. You cannot specify both IncludeFilters       and ExcludeFilters in the same metric stream.
    ///
    /// When you modify the IncludeFilters or ExcludeFilters of an existing metric stream       in any way, the metric stream is effectively restarted, so after such a change you will get       only the datapoints that have a timestamp after the time of the update.
    ///
    /// Required: No
    ///
    /// Type: List of MetricStreamFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_filters: Option<Vec<MetricStreamFilter>>,

    ///
    /// The ARN of the Amazon Kinesis Firehose delivery stream to use for this metric stream. This       Amazon Kinesis Firehose delivery stream must already exist and must be in the same account as the metric stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirehoseArn")]
    pub firehose_arn: cfn_resources::StrVal,

    ///
    /// If you specify this parameter, the stream sends only the metrics from the metric namespaces that you specify here.       You cannot specify both IncludeFilters and ExcludeFilters in the same metric stream.
    ///
    /// When you modify the IncludeFilters or ExcludeFilters of an existing metric stream       in any way, the metric stream is effectively restarted, so after such a change you will get       only the datapoints that have a timestamp after the time of the update.
    ///
    /// Required: No
    ///
    /// Type: List of MetricStreamFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_filters: Option<Vec<MetricStreamFilter>>,

    ///
    /// If you are creating a metric stream in a monitoring account, specify true to include       metrics from source accounts that are linked to this monitoring account, in the metric stream. The default is false.
    ///
    /// For more information about linking accounts, see     CloudWatch cross-account observability
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeLinkedAccountsMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts_metrics: Option<bool>,

    ///
    /// If you are creating a new metric stream, this is the name for the new stream.       The name must be different than the names of other metric streams in this account and Region.
    ///
    /// If you are updating a metric stream, specify the name of that stream here.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The output format for the stream. Valid values are json and       opentelemetry0.7 For more information about metric stream output formats, see                Metric streams output formats.
    ///
    /// This parameter is required.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputFormat")]
    pub output_format: cfn_resources::StrVal,

    ///
    /// The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Firehose       resources. This IAM role must already exist and must be in the same account as the metric stream.       This IAM role must include the firehose:PutRecord and firehose:PutRecordBatch     permissions.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    /// By default, a     metric stream always sends the MAX, MIN, SUM, and SAMPLECOUNT statistics for each metric that is streamed.     You can use this parameter to have the metric stream also send additional statistics in the stream. This     array can have up to 100 members.
    ///
    /// For each entry in this array, you specify one or more metrics and the list of additional statistics to       stream for those metrics. The additional statistics that you can stream depend on the stream's OutputFormat.       If the OutputFormat is json, you can stream any additional statistic that is supported by       CloudWatch, listed in       CloudWatch statistics definitions. If the OutputFormat is       opentelemetry0.7, you can stream percentile statistics (p??).
    ///
    /// Required: No
    ///
    /// Type: List of MetricStreamStatisticsConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatisticsConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>,

    ///
    /// An array of key-value pairs to apply to the metric stream.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnMetricStreamarn,

    #[serde(skip_serializing)]
    pub att_creation_date: CfnMetricStreamcreationdate,

    #[serde(skip_serializing)]
    pub att_last_update_date: CfnMetricStreamlastupdatedate,

    #[serde(skip_serializing)]
    pub att_state: CfnMetricStreamstate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMetricStreamarn;
impl CfnMetricStreamarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMetricStreamcreationdate;
impl CfnMetricStreamcreationdate {
    pub fn att_name(&self) -> &'static str {
        r#"CreationDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMetricStreamlastupdatedate;
impl CfnMetricStreamlastupdatedate {
    pub fn att_name(&self) -> &'static str {
        r#"LastUpdateDate"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMetricStreamstate;
impl CfnMetricStreamstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnMetricStream {
    fn type_string(&self) -> &'static str {
        "AWS::CloudWatch::MetricStream"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// This structure contains a metric namespace and optionally, a list of metric names, to either include in a metric '       stream or exclude from a metric stream.
///
/// A metric stream's filters can include up to 1000 total names. This limit applies to the sum of namespace names       and metric names in the filters. For example, this could include 10 metric namespace filters with       99 metrics each, or 20 namespace filters with 49 metrics specified in each filter.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricStreamFilter {
    ///
    /// The names of the metrics to either include or exclude from the metric stream.
    ///
    /// If you omit this parameter, all metrics in the namespace are included or excluded, depending on whether this         filter is specified as an exclude filter or an include filter.
    ///
    /// Each metric name can contain only ASCII printable characters (ASCII range 32 through 126). Each metric name         must contain at least one non-whitespace character.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_names: Option<Vec<String>>,

    ///
    /// The name of the metric namespace in the filter.
    ///
    /// The namespace can contain only ASCII printable characters (ASCII range 32 through 126). It must       contain at least one non-whitespace character.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MetricStreamFilter {
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

/// This structure specifies a list of additional statistics to stream, and the metrics to stream those     additional statistics for.
///
/// All metrics that match the combination of metric name and namespace will be streamed       with the additional statistics, no matter their dimensions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricStreamStatisticsConfiguration {
    /// The     additional statistics to stream for the metrics listed in IncludeMetrics.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalStatistics")]
    pub additional_statistics: Vec<String>,

    /// An array that   defines the metrics that are to have additional statistics streamed.
    ///
    /// Required: Yes
    ///
    /// Type: List of MetricStreamStatisticsMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeMetrics")]
    pub include_metrics: Vec<MetricStreamStatisticsMetric>,
}

impl cfn_resources::CfnResource for MetricStreamStatisticsConfiguration {
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

/// A structure that specifies the   metric name and namespace for one metric that is going to have additional statistics included in the stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MetricStreamStatisticsMetric {
    /// The name of the metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

    /// The namespace of the metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MetricStreamStatisticsMetric {
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
