/// Use the AWS::IoT::TopicRule resource to declare an AWS IoT rule. For     information about working with AWS IoT rules, see Rules for AWS IoT in the       AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopicRule {
    ///
    /// The name of the rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<cfn_resources::StrVal>,

    ///
    /// Metadata which can be used to manage the topic rule.
    ///
    /// NoteFor URI Request parameters use format: ...key1=value1&key2=value2...For the CLI command-line parameter use format: --tags       "key1=value1&key2=value2..."For the cli-input-json file use format: "tags":       "key1=value1&key2=value2..."
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
    /// The rule payload.
    ///
    /// Required: Yes
    ///
    /// Type: TopicRulePayload
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,

    #[serde(skip_serializing)]
    pub att_arn: CfnTopicRulearn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTopicRulearn;
impl CfnTopicRulearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnTopicRule {
    fn type_string(&self) -> &'static str {
        "AWS::IoT::TopicRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.topic_rule_payload.validate()?;

        Ok(())
    }
}

/// Describes the actions associated with a rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Action {
    ///
    /// Change the state of a CloudWatch alarm.
    ///
    /// Required: No
    ///
    /// Type: CloudwatchAlarmAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchAlarm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,

    ///
    /// Sends data to CloudWatch.
    ///
    /// Required: No
    ///
    /// Type: CloudwatchLogsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs: Option<CloudwatchLogsAction>,

    ///
    /// Capture a CloudWatch metric.
    ///
    /// Required: No
    ///
    /// Type: CloudwatchMetricAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,

    ///
    /// Write to a DynamoDB table.
    ///
    /// Required: No
    ///
    /// Type: DynamoDBAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<DynamoDBAction>,

    ///
    /// Write to a DynamoDB table. This is a new version of the DynamoDB action. It allows     you to write each attribute in an MQTT message payload into a separate DynamoDB     column.
    ///
    /// Required: No
    ///
    /// Type: DynamoDBv2Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBv2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_dbv2: Option<DynamoDBv2Action>,

    ///
    /// Write data to an Amazon OpenSearch Service domain.
    ///
    /// NoteThe Elasticsearch action can only be used by existing rule actions.       To create a new rule action or to update an existing rule action, use the       OpenSearch rule action instead. For more information, see       OpenSearchAction.
    ///
    /// Required: No
    ///
    /// Type: ElasticsearchAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Elasticsearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<ElasticsearchAction>,

    ///
    /// Write to an Amazon Kinesis Firehose stream.
    ///
    /// Required: No
    ///
    /// Type: FirehoseAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,

    ///
    /// Send data to an HTTPS endpoint.
    ///
    /// Required: No
    ///
    /// Type: HttpAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Http")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpAction>,

    ///
    /// Sends message data to an AWS IoT Analytics channel.
    ///
    /// Required: No
    ///
    /// Type: IotAnalyticsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_analytics: Option<IotAnalyticsAction>,

    ///
    /// Sends an input to an AWS IoT Events detector.
    ///
    /// Required: No
    ///
    /// Type: IotEventsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEventsAction>,

    ///
    /// Sends data from the MQTT message that triggered the rule to AWS IoT SiteWise asset    properties.
    ///
    /// Required: No
    ///
    /// Type: IotSiteWiseAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotSiteWise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise: Option<IotSiteWiseAction>,

    ///
    /// Send messages to an Amazon Managed Streaming for Apache Kafka (Amazon MSK) or self-managed Apache Kafka cluster.
    ///
    /// Required: No
    ///
    /// Type: KafkaAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kafka")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaAction>,

    ///
    /// Write data to an Amazon Kinesis stream.
    ///
    /// Required: No
    ///
    /// Type: KinesisAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kinesis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis: Option<KinesisAction>,

    ///
    /// Invoke a Lambda function.
    ///
    /// Required: No
    ///
    /// Type: LambdaAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaAction>,

    ///
    /// Sends device location data to Amazon Location Service.
    ///
    /// Required: No
    ///
    /// Type: LocationAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationAction>,

    ///
    /// Write data to an Amazon OpenSearch Service domain.
    ///
    /// Required: No
    ///
    /// Type: OpenSearchAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search: Option<OpenSearchAction>,

    ///
    /// Publish to another MQTT topic.
    ///
    /// Required: No
    ///
    /// Type: RepublishAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Republish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub republish: Option<RepublishAction>,

    ///
    /// Write to an Amazon S3 bucket.
    ///
    /// Required: No
    ///
    /// Type: S3Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Action>,

    ///
    /// Publish to an Amazon SNS topic.
    ///
    /// Required: No
    ///
    /// Type: SnsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SnsAction>,

    ///
    /// Publish to an Amazon SQS queue.
    ///
    /// Required: No
    ///
    /// Type: SqsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SqsAction>,

    ///
    /// Starts execution of a Step Functions state machine.
    ///
    /// Required: No
    ///
    /// Type: StepFunctionsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepFunctions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_functions: Option<StepFunctionsAction>,

    ///
    /// Writes attributes from an MQTT message.
    ///
    /// Required: No
    ///
    /// Type: TimestreamAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream: Option<TimestreamAction>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cloudwatch_alarm
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cloudwatch_logs
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.cloudwatch_metric
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dynamo_db
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dynamo_dbv2
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.elasticsearch
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.firehose
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.http.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.iot_analytics
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_events
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_site_wise
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kafka.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.kinesis.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.lambda.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.location
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.open_search
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.republish
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sns.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sqs.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.step_functions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.timestream
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An asset property timestamp entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AssetPropertyTimestamp {
    ///
    /// Optional. A string that contains the nanosecond time offset. Accepts substitution    templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffsetInNanos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_in_nanos: Option<cfn_resources::StrVal>,

    ///
    /// A string that contains the time in seconds since epoch. Accepts substitution    templates.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AssetPropertyTimestamp {
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

/// An asset property value entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AssetPropertyValue {
    ///
    /// Optional. A string that describes the quality of the value. Accepts substitution    templates. Must be GOOD, BAD, or UNCERTAIN.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<cfn_resources::StrVal>,

    ///
    /// The asset property value timestamp.
    ///
    /// Required: Yes
    ///
    /// Type: AssetPropertyTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: AssetPropertyTimestamp,

    ///
    /// The value of the asset property.
    ///
    /// Required: Yes
    ///
    /// Type: AssetPropertyVariant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: AssetPropertyVariant,
}

impl cfn_resources::CfnResource for AssetPropertyValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.timestamp.validate()?;

        self.value.validate()?;

        Ok(())
    }
}

/// Contains an asset property value (of a single type).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AssetPropertyVariant {
    ///
    /// Optional. A string that contains the boolean value (true or     false) of the value entry. Accepts substitution templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BooleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<cfn_resources::StrVal>,

    ///
    /// Optional. A string that contains the double value of the value entry. Accepts substitution    templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DoubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<cfn_resources::StrVal>,

    ///
    /// Optional. A string that contains the integer value of the value entry. Accepts    substitution templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<cfn_resources::StrVal>,

    ///
    /// Optional. The string value of the value entry. Accepts substitution templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for AssetPropertyVariant {
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

/// Describes an action that updates a CloudWatch alarm.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CloudwatchAlarmAction {
    ///
    /// The CloudWatch alarm name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmName")]
    pub alarm_name: cfn_resources::StrVal,

    ///
    /// The IAM role that allows access to the CloudWatch alarm.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The reason for the alarm change.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateReason")]
    pub state_reason: cfn_resources::StrVal,

    ///
    /// The value of the alarm state. Acceptable values are: OK, ALARM,     INSUFFICIENT_DATA.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateValue")]
    pub state_value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudwatchAlarmAction {
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

/// Describes an action that updates a CloudWatch log.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CloudwatchLogsAction {
    ///
    /// Indicates whether batches of log records will be extracted and uploaded into CloudWatch.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,

    ///
    /// The CloudWatch log name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: cfn_resources::StrVal,

    ///
    /// The IAM role that allows access to the CloudWatch log.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudwatchLogsAction {
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

/// Describes an action that captures a CloudWatch metric.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CloudwatchMetricAction {
    ///
    /// The CloudWatch metric name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

    ///
    /// The CloudWatch metric namespace name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricNamespace")]
    pub metric_namespace: cfn_resources::StrVal,

    ///
    /// An optional Unix timestamp.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_timestamp: Option<cfn_resources::StrVal>,

    ///
    /// The metric       unit supported by CloudWatch.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricUnit")]
    pub metric_unit: cfn_resources::StrVal,

    ///
    /// The CloudWatch metric value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricValue")]
    pub metric_value: cfn_resources::StrVal,

    ///
    /// The IAM role that allows access to the CloudWatch metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CloudwatchMetricAction {
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

/// Describes an action to write to a DynamoDB table.
///
/// The tableName, hashKeyField, and rangeKeyField     values must match the values used when you created the table.
///
/// The hashKeyValue and rangeKeyvalue fields use a     substitution template syntax. These templates provide data at runtime. The syntax is as     follows: ${sql-expression}.
///
/// You can specify any valid expression in a WHERE or SELECT clause, including JSON     properties, comparisons, calculations, and functions. For example, the following field uses     the third level of the topic:
///
/// "hashKeyValue": "${topic(3)}"
///
/// The following field uses the timestamp:
///
/// "rangeKeyValue": "${timestamp()}"
///
/// For more information, see DynamoDBv2 Action in the       AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DynamoDBAction {
    ///
    /// The hash key name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: cfn_resources::StrVal,

    ///
    /// The hash key type. Valid values are "STRING" or "NUMBER"
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_type: Option<cfn_resources::StrVal>,

    ///
    /// The hash key value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: cfn_resources::StrVal,

    ///
    /// The action payload. This name can be customized.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_field: Option<cfn_resources::StrVal>,

    ///
    /// The range key name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_field: Option<cfn_resources::StrVal>,

    ///
    /// The range key type. Valid values are "STRING" or "NUMBER"
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_type: Option<cfn_resources::StrVal>,

    ///
    /// The range key value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_value: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The name of the DynamoDB table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DynamoDBAction {
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

/// Describes an action to write to a DynamoDB table.
///
/// This DynamoDB action writes each attribute in the message payload into it's own     column in the DynamoDB table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DynamoDBv2Action {
    ///
    /// Specifies the DynamoDB table to which the message data will be written. For     example:
    ///
    /// { "dynamoDBv2": { "roleArn": "aws:iam:12341251:my-role" "putItem": { "tableName":       "my-table" } } }
    ///
    /// Each attribute in the message payload will be written to a separate column in the     DynamoDB database.
    ///
    /// Required: No
    ///
    /// Type: PutItemInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "PutItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_item: Option<PutItemInput>,

    ///
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DynamoDBv2Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.put_item
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an action that writes data to an Amazon OpenSearch Service     domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ElasticsearchAction {
    ///
    /// The endpoint of your OpenSearch domain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: cfn_resources::StrVal,

    ///
    /// The unique identifier for the document you are storing.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The index where you want to store your data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Index")]
    pub index: cfn_resources::StrVal,

    ///
    /// The IAM role ARN that has access to OpenSearch.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The type of document you are storing.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ElasticsearchAction {
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

/// Describes an action that writes data to an Amazon Kinesis Firehose stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FirehoseAction {
    ///
    /// Whether to deliver the Kinesis Data Firehose stream as a batch by using PutRecordBatch. The default value is     false.
    ///
    /// When batchMode is true and the rule's SQL statement       evaluates to an Array, each Array element forms one record in the PutRecordBatch request. The resulting array can't have more     than 500 records.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,

    ///
    /// The delivery stream name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: cfn_resources::StrVal,

    ///
    /// The IAM role that grants access to the Amazon Kinesis Firehose stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// A character separator that will be used to separate records written to the Firehose     stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ','     (comma).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Separator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FirehoseAction {
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

/// Send data to an HTTPS endpoint.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HttpAction {
    ///
    /// The authentication method to use when sending data to an HTTPS endpoint.
    ///
    /// Required: No
    ///
    /// Type: HttpAuthorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Auth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HttpAuthorization>,

    ///
    /// The URL to which AWS IoT sends a confirmation message. The value of the confirmation URL     must be a prefix of the endpoint URL. If you do not specify a confirmation URL AWS IoT uses     the endpoint URL as the confirmation URL. If you use substitution templates in the     confirmationUrl, you must create and enable topic rule destinations that match each     possible value of the substitution template before traffic is allowed to your endpoint     URL.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfirmationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<cfn_resources::StrVal>,

    ///
    /// The HTTP headers to send with the message data.
    ///
    /// Required: No
    ///
    /// Type: List of HttpActionHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpActionHeader>>,

    ///
    /// The endpoint URL. If substitution templates are used in the URL, you must also specify a       confirmationUrl. If this is a new destination, a new       TopicRuleDestination is created if possible.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for HttpAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auth.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The HTTP action header.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HttpActionHeader {
    ///
    /// The HTTP header key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The HTTP header value. Substitution templates are supported.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for HttpActionHeader {
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

/// The authorization method used to send messages.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HttpAuthorization {
    ///
    /// Use Sig V4 authorization. For more information, see Signature       Version 4 Signing Process.
    ///
    /// Required: No
    ///
    /// Type: SigV4Authorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sigv4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigv4: Option<SigV4Authorization>,
}

impl cfn_resources::CfnResource for HttpAuthorization {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sigv4.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Sends message data to an AWS IoT Analytics channel.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IotAnalyticsAction {
    ///
    /// Whether to process the action as a batch. The default value is     false.
    ///
    /// When batchMode is true and the rule SQL statement evaluates     to an Array, each Array element is delivered as a separate message when passed by BatchPutMessage The resulting array can't have more     than 100 messages.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,

    ///
    /// The name of the IoT Analytics channel to which message data will be sent.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ChannelName")]
    pub channel_name: cfn_resources::StrVal,

    ///
    /// The ARN of the role which has a policy that grants IoT Analytics permission to send     message data via IoT Analytics (iotanalytics:BatchPutMessage).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IotAnalyticsAction {
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

/// Sends an input to an AWS IoT Events detector.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IotEventsAction {
    ///
    /// Whether to process the event actions as a batch. The default value is       false.
    ///
    /// When batchMode is true, you can't specify a       messageId.
    ///
    /// When batchMode is true and the rule SQL statement evaluates     to an Array, each Array element is treated as a separate message when     Events by calling BatchPutMessage. The resulting array can't have more     than 10 messages.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,

    ///
    /// The name of the AWS IoT Events input.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputName")]
    pub input_name: cfn_resources::StrVal,

    ///
    /// The ID of the message. The default messageId is a new UUID value.
    ///
    /// When batchMode is true, you can't specify a     messageId--a new UUID value will be assigned.
    ///
    /// Assign a value to this property to ensure that only one input (message) with a given       messageId will be processed by an AWS IoT Events detector.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the role that grants AWS IoT permission to send an input to an AWS IoT    Events detector. ("Action":"iotevents:BatchPutMessage").
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IotEventsAction {
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

/// Describes an action to send data from an MQTT message that triggered the rule to AWS IoT    SiteWise asset properties.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IotSiteWiseAction {
    ///
    /// A list of asset property value entries.
    ///
    /// Required: Yes
    ///
    /// Type: List of PutAssetPropertyValueEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "PutAssetPropertyValueEntries")]
    pub put_asset_property_value_entries: Vec<PutAssetPropertyValueEntry>,

    ///
    /// The ARN of the role that grants AWS IoT permission to send an asset property value to AWS IoT SiteWise. ("Action": "iotsitewise:BatchPutAssetPropertyValue"). The trust    policy can restrict access to specific asset hierarchy paths.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IotSiteWiseAction {
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

/// Send messages to an Amazon Managed Streaming for Apache Kafka (Amazon MSK) or self-managed Apache Kafka cluster.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct KafkaAction {
    ///
    /// Properties of the Apache Kafka producer client.
    ///
    /// Required: Yes
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientProperties")]
    pub client_properties: std::collections::HashMap<String, String>,

    ///
    /// The ARN of Kafka action's VPC TopicRuleDestination.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: cfn_resources::StrVal,

    ///
    /// The Kafka message key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The Kafka message partition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Partition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<cfn_resources::StrVal>,

    ///
    /// The Kafka topic for messages to be sent to the Kafka broker.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    pub topic: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KafkaAction {
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

/// Describes an action to write data to an Amazon Kinesis stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct KinesisAction {
    ///
    /// The partition key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM role that grants access to the Amazon Kinesis stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The name of the Amazon Kinesis stream.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    pub stream_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KinesisAction {
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

/// Describes an action to invoke a Lambda function.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LambdaAction {
    ///
    /// The ARN of the Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LambdaAction {
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

/// Describes an action to send device location updates from an MQTT message to an Amazon Location tracker resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LocationAction {
    ///
    /// The unique ID of the device providing the location data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceId")]
    pub device_id: cfn_resources::StrVal,

    ///
    /// A string that evaluates to a double value that represents the latitude of the device's location.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Latitude")]
    pub latitude: cfn_resources::StrVal,

    ///
    /// A string that evaluates to a double value that represents the longitude of the device's location.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Longitude")]
    pub longitude: cfn_resources::StrVal,

    ///
    /// The IAM role that grants permission to write to the Amazon Location resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The time that the location data was sampled. The default value is the time the MQTT message was processed.
    ///
    /// Required: No
    ///
    /// Type: Timestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,

    ///
    /// The name of the tracker resource in Amazon Location in which the location is updated.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackerName")]
    pub tracker_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LocationAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.timestamp
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes an action that writes data to an Amazon OpenSearch Service domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OpenSearchAction {
    ///
    /// The endpoint of your OpenSearch domain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: cfn_resources::StrVal,

    ///
    /// The unique identifier for the document you are storing.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The OpenSearch index where you want to store your data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Index")]
    pub index: cfn_resources::StrVal,

    ///
    /// The IAM role ARN that has access to OpenSearch.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The type of document you are storing.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OpenSearchAction {
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

/// An asset property value entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PutAssetPropertyValueEntry {
    ///
    /// The ID of the AWS IoT SiteWise asset. You must specify either a propertyAlias    or both an aliasId and a propertyId. Accepts substitution    templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<cfn_resources::StrVal>,

    ///
    /// Optional. A unique identifier for this entry that you can define to better track which    message caused an error in case of failure. Accepts substitution templates. Defaults to a new    UUID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the property alias associated with your asset property. You must specify    either a propertyAlias or both an aliasId and a     propertyId. Accepts substitution templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_alias: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the asset's property. You must specify either a propertyAlias or    both an aliasId and a propertyId. Accepts substitution    templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of property values to insert that each contain timestamp, quality, and value (TQV)    information.
    ///
    /// Required: Yes
    ///
    /// Type: List of AssetPropertyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyValues")]
    pub property_values: Vec<AssetPropertyValue>,
}

impl cfn_resources::CfnResource for PutAssetPropertyValueEntry {
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

/// The input for the DynamoActionVS action that specifies the DynamoDB table to which     the message data will be written.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PutItemInput {
    ///
    /// The table where the message data will be written.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for PutItemInput {
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

/// Describes an action to republish to another topic.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RepublishAction {
    ///
    /// MQTT Version 5.0 headers information. For more information, see MQTT in the IoT Core Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: RepublishActionHeaders
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<RepublishActionHeaders>,

    ///
    /// The Quality of Service (QoS) level to use when republishing messages. The default value     is 0.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Qos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i64>,

    ///
    /// The ARN of the IAM role that grants access.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The name of the MQTT topic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    pub topic: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for RepublishAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.headers.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies MQTT Version 5.0 headers information. For more information, see MQTT in the IoT Core Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RepublishActionHeaders {
    ///
    /// A UTF-8 encoded string that describes the content of the publishing message.
    ///
    /// For more information, see       Content Type in the MQTT Version 5.0 specification.
    ///
    /// Supports substitution     templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<cfn_resources::StrVal>,

    ///
    /// The base64-encoded binary data used by the sender of the request message to identify     which request the response message is for.
    ///
    /// For more information, see       Correlation Data in the MQTT Version 5.0 specification.
    ///
    /// Supports substitution     templates.
    ///
    /// Note This binary data must be base64-encoded.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CorrelationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_data: Option<cfn_resources::StrVal>,

    ///
    /// A user-defined integer value that represents the message expiry interval at the broker.     If the messages haven't been sent to the subscribers within that interval, the message     expires and is removed. The value of messageExpiry represents the number of     seconds before it expires. For more information about the limits of       messageExpiry, see Message broker and protocol limits and       quotas in the IoT Core Reference Guide.
    ///
    /// Supports substitution     templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expiry: Option<cfn_resources::StrVal>,

    ///
    /// An Enum string value that indicates whether the payload is formatted as     UTF-8.
    ///
    /// Valid values are UNSPECIFIED_BYTES and UTF8_DATA.
    ///
    /// For more information, see      Payload Format Indicator from the MQTT Version 5.0 specification.
    ///
    /// Supports substitution     templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadFormatIndicator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_indicator: Option<cfn_resources::StrVal>,

    ///
    /// A UTF-8 encoded string that's used as the topic name for a response message. The     response topic is used to describe the topic to which the receiver should publish as part     of the request-response flow. The topic must not contain wildcard characters.
    ///
    /// For more information, see       Response Topic in the MQTT Version 5.0 specification.
    ///
    /// Supports substitution     templates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_topic: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs that you define in the MQTT5 header.
    ///
    /// Required: No
    ///
    /// Type: List of UserProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<Vec<UserProperty>>,
}

impl cfn_resources::CfnResource for RepublishActionHeaders {
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

/// Describes an action to write data to an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct S3Action {
    ///
    /// The Amazon S3 bucket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: cfn_resources::StrVal,

    ///
    /// The Amazon S3 canned ACL that controls access to the object identified by the object     key. For more information, see S3 canned ACLs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<cfn_resources::StrVal>,

    ///
    /// The object key. For more information, see Actions, resources, and condition keys for Amazon S3.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The ARN of the IAM role that grants access.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for S3Action {
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

/// For more information, see Signature Version 4 signing process.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SigV4Authorization {
    ///
    /// The ARN of the signing role.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The service name to use while signing with Sig V4.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: cfn_resources::StrVal,

    ///
    /// The signing region.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningRegion")]
    pub signing_region: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SigV4Authorization {
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

/// Describes an action to publish to an Amazon SNS topic.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SnsAction {
    ///
    /// (Optional) The message format of the message to publish. Accepted values are "JSON"     and "RAW". The default value of the attribute is "RAW". SNS uses this setting to determine     if the payload should be parsed and relevant platform-specific bits of the payload should     be extracted. For more information, see Amazon SNS Message and JSON Formats in the       Amazon Simple Notification Service Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM role that grants access.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The ARN of the SNS topic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    pub target_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SnsAction {
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

/// Describes an action to publish data to an Amazon SQS queue.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SqsAction {
    ///
    /// The URL of the Amazon SQS queue.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueueUrl")]
    pub queue_url: cfn_resources::StrVal,

    ///
    /// The ARN of the IAM role that grants access.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// Specifies whether to use Base64 encoding.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_base64: Option<bool>,
}

impl cfn_resources::CfnResource for SqsAction {
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

/// Starts execution of a Step Functions state machine.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StepFunctionsAction {
    ///
    /// (Optional) A name will be given to the state machine execution consisting of this    prefix followed by a UUID. Step Functions automatically creates a unique name for each state    machine execution if one is not provided.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_name_prefix: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the role that grants IoT permission to start execution of a state machine    ("Action":"states:StartExecution").
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The name of the Step Functions state machine whose execution will be started.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateMachineName")]
    pub state_machine_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StepFunctionsAction {
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

/// Describes how to interpret an application-defined timestamp value from an MQTT message payload and the precision of that value.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Timestamp {
    ///
    /// The precision of the timestamp value that results from the expression described in value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<cfn_resources::StrVal>,

    ///
    /// An expression that returns a long epoch time value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Timestamp {
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

/// Describes an action that writes records into an Amazon Timestream table.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimestreamAction {
    ///
    /// The name of an Amazon Timestream database that has the table to write records into.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: cfn_resources::StrVal,

    ///
    /// Metadata attributes of the time series that are written in each measure record.
    ///
    /// Required: Yes
    ///
    /// Type: List of TimestreamDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Vec<TimestreamDimension>,

    ///
    /// The Amazon Resource Name (ARN) of the role that grants AWS IoT permission to write to the Timestream database table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The table where the message data will be written.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,

    ///
    /// The value to use for the entry's timestamp. If blank, the time that the entry was processed is used.
    ///
    /// Required: No
    ///
    /// Type: TimestreamTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestreamTimestamp>,
}

impl cfn_resources::CfnResource for TimestreamAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.timestamp
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Metadata attributes of the time series that are written in each measure record.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimestreamDimension {
    ///
    /// The metadata dimension name. This is the name of the column in the Amazon Timestream database table record.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The value to write in this column of the database record.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TimestreamDimension {
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

/// The value to use for the entry's timestamp. If blank, the time that the entry was processed is used.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TimestreamTimestamp {
    ///
    /// The precision of the timestamp value that results from the expression described in value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: cfn_resources::StrVal,

    ///
    /// An expression that returns a long epoch time value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TimestreamTimestamp {
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

/// Describes a rule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TopicRulePayload {
    ///
    /// The actions associated with the rule.
    ///
    /// Required: Yes
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,

    ///
    /// The version of the SQL rules engine to use when evaluating the rule.
    ///
    /// The default value is 2015-10-08.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsIotSqlVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_sql_version: Option<cfn_resources::StrVal>,

    ///
    /// The description of the rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The action to take when an error occurs.
    ///
    /// Required: No
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_action: Option<Action>,

    ///
    /// Specifies whether the rule is disabled.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,

    ///
    /// The SQL statement used to query the topic. For more information, see AWS IoT SQL       Reference in the         AWS IoT Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sql")]
    pub sql: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TopicRulePayload {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.error_action
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A key-value pair that you define in the header.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct UserProperty {
    ///
    /// A key to be specified in UserProperty.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// A value to be specified in UserProperty.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for UserProperty {
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
