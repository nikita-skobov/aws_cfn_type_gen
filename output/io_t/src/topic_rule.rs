

/// Use the AWS::IoT::TopicRule resource to declare an AWS IoT rule. For     information about working with AWS IoT rules, see Rules for AWS IoT in the       AWS IoT Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTopicRule {


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


    /// 
    /// The name of the rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleName")]
    pub rule_name: Option<String>,


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
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnTopicRule {
    fn type_string() -> &'static str {
        "AWS::IoT::TopicRule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes an action that updates a CloudWatch alarm.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub alarm_name: String,


    /// 
    /// The value of the alarm state. Acceptable values are: OK, ALARM,     INSUFFICIENT_DATA.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateValue")]
    pub state_value: String,


    /// 
    /// The IAM role that allows access to the CloudWatch alarm.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The reason for the alarm change.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateReason")]
    pub state_reason: String,

}




/// Describes an action that captures a CloudWatch metric.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudwatchMetricAction {


    /// 
    /// The CloudWatch metric value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricValue")]
    pub metric_value: String,


    /// 
    /// The metric       unit supported by CloudWatch.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricUnit")]
    pub metric_unit: String,


    /// 
    /// An optional Unix timestamp.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricTimestamp")]
    pub metric_timestamp: Option<String>,


    /// 
    /// The CloudWatch metric namespace name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricNamespace")]
    pub metric_namespace: String,


    /// 
    /// The CloudWatch metric name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The IAM role that allows access to the CloudWatch metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Describes an action to write data to an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Action {


    /// 
    /// The ARN of the IAM role that grants access.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The Amazon S3 canned ACL that controls access to the object identified by the object     key. For more information, see S3 canned ACLs.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CannedAcl")]
    pub canned_acl: Option<String>,


    /// 
    /// The object key. For more information, see Actions, resources, and condition keys for Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The Amazon S3 bucket.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}




/// Send data to an HTTPS endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpAction {


    /// 
    /// The URL to which AWS IoT sends a confirmation message. The value of the confirmation URL     must be a prefix of the endpoint URL. If you do not specify a confirmation URL AWS IoT uses     the endpoint URL as the confirmation URL. If you use substitution templates in the     confirmationUrl, you must create and enable topic rule destinations that match each     possible value of the substitution template before traffic is allowed to your endpoint     URL.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConfirmationUrl")]
    pub confirmation_url: Option<String>,


    /// 
    /// The endpoint URL. If substitution templates are used in the URL, you must also specify a       confirmationUrl. If this is a new destination, a new       TopicRuleDestination is created if possible.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: String,


    /// 
    /// The authentication method to use when sending data to an HTTPS endpoint.
    /// 
    /// Required: No
    ///
    /// Type: HttpAuthorization
    ///
    /// Update requires: No interruption
    #[serde(rename = "Auth")]
    pub auth: Option<HttpAuthorization>,


    /// 
    /// The HTTP headers to send with the message data.
    /// 
    /// Required: No
    ///
    /// Type: List of HttpActionHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpActionHeader>>,

}




/// An asset property value entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub quality: Option<String>,


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




/// An asset property timestamp entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyTimestamp {


    /// 
    /// A string that contains the time in seconds since epoch. Accepts substitution    templates.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: String,


    /// 
    /// Optional. A string that contains the nanosecond time offset. Accepts substitution    templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffsetInNanos")]
    pub offset_in_nanos: Option<String>,

}




/// Describes an action to send device location updates from an MQTT message to an Amazon Location tracker resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_id: String,


    /// 
    /// A string that evaluates to a double value that represents the longitude of the device's location.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Longitude")]
    pub longitude: String,


    /// 
    /// A string that evaluates to a double value that represents the latitude of the device's location.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Latitude")]
    pub latitude: String,


    /// 
    /// The time that the location data was sampled. The default value is the time the MQTT message was processed.
    /// 
    /// Required: No
    ///
    /// Type: Timestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<Timestamp>,


    /// 
    /// The IAM role that grants permission to write to the Amazon Location resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the tracker resource in Amazon Location in which the location is updated.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrackerName")]
    pub tracker_name: String,

}




/// The authorization method used to send messages.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub sigv4: Option<SigV4Authorization>,

}




/// Describes an action to send data from an MQTT message that triggered the rule to AWS IoT    SiteWise asset properties.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotSiteWiseAction {


    /// 
    /// The ARN of the role that grants AWS IoT permission to send an asset property value to AWS IoT SiteWise. ("Action": "iotsitewise:BatchPutAssetPropertyValue"). The trust    policy can restrict access to specific asset hierarchy paths.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


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

}




/// Describes an action that writes data to an Amazon OpenSearch Service     domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ElasticsearchAction {


    /// 
    /// The IAM role ARN that has access to OpenSearch.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The unique identifier for the document you are storing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The endpoint of your OpenSearch domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: String,


    /// 
    /// The index where you want to store your data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Index")]
    pub index: String,


    /// 
    /// The type of document you are storing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,

}




/// Describes an action to write data to an Amazon Kinesis stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KinesisAction {


    /// 
    /// The ARN of the IAM role that grants access to the Amazon Kinesis stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The partition key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionKey")]
    pub partition_key: Option<String>,


    /// 
    /// The name of the Amazon Kinesis stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StreamName")]
    pub stream_name: String,

}




/// Sends an input to an AWS IoT Events detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotEventsAction {


    /// 
    /// The name of the AWS IoT Events input.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputName")]
    pub input_name: String,


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
    pub message_id: Option<String>,


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
    pub batch_mode: Option<bool>,


    /// 
    /// The ARN of the role that grants AWS IoT permission to send an input to an AWS IoT    Events detector. ("Action":"iotevents:BatchPutMessage").
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Describes an action to publish data to an Amazon SQS queue.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SqsAction {


    /// 
    /// Specifies whether to use Base64 encoding.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBase64")]
    pub use_base64: Option<bool>,


    /// 
    /// The URL of the Amazon SQS queue.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueueUrl")]
    pub queue_url: String,


    /// 
    /// The ARN of the IAM role that grants access.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Metadata attributes of the time series that are written in each measure record.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub name: String,


    /// 
    /// The value to write in this column of the database record.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}




/// Specifies MQTT Version 5.0 headers information. For more information, see MQTT in the IoT Core Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RepublishActionHeaders {


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
    pub correlation_data: Option<String>,


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
    pub message_expiry: Option<String>,


    /// 
    /// An array of key-value pairs that you define in the MQTT5 header.
    /// 
    /// Required: No
    ///
    /// Type: List of UserProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserProperties")]
    pub user_properties: Option<Vec<UserProperty>>,


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
    pub response_topic: Option<String>,


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
    pub payload_format_indicator: Option<String>,


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
    pub content_type: Option<String>,

}




/// Describes the actions associated with a rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {


    /// 
    /// Writes attributes from an MQTT message.
    /// 
    /// Required: No
    ///
    /// Type: TimestreamAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestream")]
    pub timestream: Option<TimestreamAction>,


    /// 
    /// Publish to an Amazon SQS queue.
    /// 
    /// Required: No
    ///
    /// Type: SqsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqs")]
    pub sqs: Option<SqsAction>,


    /// 
    /// Capture a CloudWatch metric.
    /// 
    /// Required: No
    ///
    /// Type: CloudwatchMetricAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchMetric")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,


    /// 
    /// Publish to an Amazon SNS topic.
    /// 
    /// Required: No
    ///
    /// Type: SnsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    pub sns: Option<SnsAction>,


    /// 
    /// Invoke a Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: LambdaAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lambda")]
    pub lambda: Option<LambdaAction>,


    /// 
    /// Send data to an HTTPS endpoint.
    /// 
    /// Required: No
    ///
    /// Type: HttpAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Http")]
    pub http: Option<HttpAction>,


    /// 
    /// Sends data from the MQTT message that triggered the rule to AWS IoT SiteWise asset    properties.
    /// 
    /// Required: No
    ///
    /// Type: IotSiteWiseAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWiseAction>,


    /// 
    /// Sends message data to an AWS IoT Analytics channel.
    /// 
    /// Required: No
    ///
    /// Type: IotAnalyticsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotAnalytics")]
    pub iot_analytics: Option<IotAnalyticsAction>,


    /// 
    /// Write to a DynamoDB table. This is a new version of the DynamoDB action. It allows     you to write each attribute in an MQTT message payload into a separate DynamoDB     column.
    /// 
    /// Required: No
    ///
    /// Type: DynamoDBv2Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2Action>,


    /// 
    /// Write to an Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: S3Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3Action>,


    /// 
    /// Write to a DynamoDB table.
    /// 
    /// Required: No
    ///
    /// Type: DynamoDBAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDBAction>,


    /// 
    /// Publish to another MQTT topic.
    /// 
    /// Required: No
    ///
    /// Type: RepublishAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Republish")]
    pub republish: Option<RepublishAction>,


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
    pub elasticsearch: Option<ElasticsearchAction>,


    /// 
    /// Send messages to an Amazon Managed Streaming for Apache Kafka (Amazon MSK) or self-managed Apache Kafka cluster.
    /// 
    /// Required: No
    ///
    /// Type: KafkaAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kafka")]
    pub kafka: Option<KafkaAction>,


    /// 
    /// Starts execution of a Step Functions state machine.
    /// 
    /// Required: No
    ///
    /// Type: StepFunctionsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepFunctions")]
    pub step_functions: Option<StepFunctionsAction>,


    /// 
    /// Write to an Amazon Kinesis Firehose stream.
    /// 
    /// Required: No
    ///
    /// Type: FirehoseAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Firehose")]
    pub firehose: Option<FirehoseAction>,


    /// 
    /// Change the state of a CloudWatch alarm.
    /// 
    /// Required: No
    ///
    /// Type: CloudwatchAlarmAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchAlarm")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,


    /// 
    /// Write data to an Amazon OpenSearch Service domain.
    /// 
    /// Required: No
    ///
    /// Type: OpenSearchAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "OpenSearch")]
    pub open_search: Option<OpenSearchAction>,


    /// 
    /// Sends data to CloudWatch.
    /// 
    /// Required: No
    ///
    /// Type: CloudwatchLogsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudwatchLogs")]
    pub cloudwatch_logs: Option<CloudwatchLogsAction>,


    /// 
    /// Write data to an Amazon Kinesis stream.
    /// 
    /// Required: No
    ///
    /// Type: KinesisAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Kinesis")]
    pub kinesis: Option<KinesisAction>,


    /// 
    /// Sends device location data to Amazon Location Service.
    /// 
    /// Required: No
    ///
    /// Type: LocationAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: Option<LocationAction>,


    /// 
    /// Sends an input to an AWS IoT Events detector.
    /// 
    /// Required: No
    ///
    /// Type: IotEventsAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEventsAction>,

}




/// Send messages to an Amazon Managed Streaming for Apache Kafka (Amazon MSK) or self-managed Apache Kafka cluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KafkaAction {


    /// 
    /// The ARN of Kafka action's VPC TopicRuleDestination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,


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
    /// The Kafka topic for messages to be sent to the Kafka broker.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    pub topic: String,


    /// 
    /// The Kafka message key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The Kafka message partition.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Partition")]
    pub partition: Option<String>,

}




/// Describes an action to invoke a Lambda function.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub function_arn: Option<String>,

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




/// Describes how to interpret an application-defined timestamp value from an MQTT message payload and the precision of that value.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub unit: Option<String>,


    /// 
    /// An expression that returns a long epoch time value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}




/// Describes a rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TopicRulePayload {


    /// 
    /// Specifies whether the rule is disabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleDisabled")]
    pub rule_disabled: Option<bool>,


    /// 
    /// The action to take when an error occurs.
    /// 
    /// Required: No
    ///
    /// Type: Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorAction")]
    pub error_action: Option<Action>,


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
    pub aws_iot_sql_version: Option<String>,


    /// 
    /// The SQL statement used to query the topic. For more information, see AWS IoT SQL       Reference in the         AWS IoT Developer Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sql")]
    pub sql: String,


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
    /// The description of the rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}




/// The HTTP action header.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpActionHeader {


    /// 
    /// The HTTP header value. Substitution templates are supported.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The HTTP header key.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

}




/// For more information, see Signature Version 4 signing process.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SigV4Authorization {


    /// 
    /// The service name to use while signing with Sig V4.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: String,


    /// 
    /// The ARN of the signing role.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The signing region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SigningRegion")]
    pub signing_region: String,

}




/// Describes an action to write to a DynamoDB table.
///
/// This DynamoDB action writes each attribute in the message payload into it's own     column in the DynamoDB table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamoDBv2Action {


    /// 
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


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
    pub put_item: Option<PutItemInput>,

}




/// The input for the DynamoActionVS action that specifies the DynamoDB table to which     the message data will be written.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub table_name: String,

}




/// Contains an asset property value (of a single type).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyVariant {


    /// 
    /// Optional. The string value of the value entry. Accepts substitution templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,


    /// 
    /// Optional. A string that contains the integer value of the value entry. Accepts    substitution templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerValue")]
    pub integer_value: Option<String>,


    /// 
    /// Optional. A string that contains the boolean value (true or     false) of the value entry. Accepts substitution templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,


    /// 
    /// Optional. A string that contains the double value of the value entry. Accepts substitution    templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<String>,

}




/// A key-value pair that you define in the header.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub key: String,


    /// 
    /// A value to be specified in UserProperty.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}




/// Sends message data to an AWS IoT Analytics channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub channel_name: String,


    /// 
    /// The ARN of the role which has a policy that grants IoT Analytics permission to send     message data via IoT Analytics (iotanalytics:BatchPutMessage).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// The value to use for the entry's timestamp. If blank, the time that the entry was processed is used.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimestreamTimestamp {


    /// 
    /// An expression that returns a long epoch time value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The precision of the timestamp value that results from the expression described in value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: String,

}




/// Describes an action that writes data to an Amazon Kinesis Firehose stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub batch_mode: Option<bool>,


    /// 
    /// A character separator that will be used to separate records written to the Firehose     stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ','     (comma).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Separator")]
    pub separator: Option<String>,


    /// 
    /// The IAM role that grants access to the Amazon Kinesis Firehose stream.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The delivery stream name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,

}




/// Describes an action that updates a CloudWatch log.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CloudwatchLogsAction {


    /// 
    /// The CloudWatch log name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,


    /// 
    /// Indicates whether batches of log records will be extracted and uploaded into CloudWatch.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchMode")]
    pub batch_mode: Option<bool>,


    /// 
    /// The IAM role that allows access to the CloudWatch log.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// An asset property value entry containing the following information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PutAssetPropertyValueEntry {


    /// 
    /// Optional. A unique identifier for this entry that you can define to better track which    message caused an error in case of failure. Accepts substitution templates. Defaults to a new    UUID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntryId")]
    pub entry_id: Option<String>,


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


    /// 
    /// The ID of the asset's property. You must specify either a propertyAlias or    both an aliasId and a propertyId. Accepts substitution    templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyId")]
    pub property_id: Option<String>,


    /// 
    /// The ID of the AWS IoT SiteWise asset. You must specify either a propertyAlias    or both an aliasId and a propertyId. Accepts substitution    templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetId")]
    pub asset_id: Option<String>,


    /// 
    /// The name of the property alias associated with your asset property. You must specify    either a propertyAlias or both an aliasId and a     propertyId. Accepts substitution templates.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,

}




/// Describes an action to republish to another topic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RepublishAction {


    /// 
    /// The ARN of the IAM role that grants access.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the MQTT topic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Topic")]
    pub topic: String,


    /// 
    /// MQTT Version 5.0 headers information. For more information, see MQTT in the IoT Core Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: RepublishActionHeaders
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
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
    pub qos: Option<i64>,

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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamoDBAction {


    /// 
    /// The range key name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyField")]
    pub range_key_field: Option<String>,


    /// 
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The range key type. Valid values are "STRING" or "NUMBER"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyType")]
    pub range_key_type: Option<String>,


    /// 
    /// The hash key value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: String,


    /// 
    /// The name of the DynamoDB table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The hash key type. Valid values are "STRING" or "NUMBER"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyType")]
    pub hash_key_type: Option<String>,


    /// 
    /// The action payload. This name can be customized.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadField")]
    pub payload_field: Option<String>,


    /// 
    /// The range key value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyValue")]
    pub range_key_value: Option<String>,


    /// 
    /// The hash key name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: String,

}




/// Starts execution of a Step Functions state machine.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepFunctionsAction {


    /// 
    /// The ARN of the role that grants IoT permission to start execution of a state machine    ("Action":"states:StartExecution").
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The name of the Step Functions state machine whose execution will be started.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StateMachineName")]
    pub state_machine_name: String,


    /// 
    /// (Optional) A name will be given to the state machine execution consisting of this    prefix followed by a UUID. Step Functions automatically creates a unique name for each state    machine execution if one is not provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionNamePrefix")]
    pub execution_name_prefix: Option<String>,

}




/// Describes an action that writes records into an Amazon Timestream table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TimestreamAction {


    /// 
    /// The table where the message data will be written.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


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
    /// The value to use for the entry's timestamp. If blank, the time that the entry was processed is used.
    /// 
    /// Required: No
    ///
    /// Type: TimestreamTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<TimestreamTimestamp>,


    /// 
    /// The name of an Amazon Timestream database that has the table to write records into.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the role that grants AWS IoT permission to write to the Timestream database table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}




/// Describes an action to publish to an Amazon SNS topic.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnsAction {


    /// 
    /// The ARN of the IAM role that grants access.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// (Optional) The message format of the message to publish. Accepted values are "JSON"     and "RAW". The default value of the attribute is "RAW". SNS uses this setting to determine     if the payload should be parsed and relevant platform-specific bits of the payload should     be extracted. For more information, see Amazon SNS Message and JSON Formats in the       Amazon Simple Notification Service Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageFormat")]
    pub message_format: Option<String>,


    /// 
    /// The ARN of the SNS topic.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    pub target_arn: String,

}




/// Describes an action that writes data to an Amazon OpenSearch Service domain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OpenSearchAction {


    /// 
    /// The unique identifier for the document you are storing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The OpenSearch index where you want to store your data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Index")]
    pub index: String,


    /// 
    /// The type of document you are storing.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The endpoint of your OpenSearch domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Endpoint")]
    pub endpoint: String,


    /// 
    /// The IAM role ARN that has access to OpenSearch.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


