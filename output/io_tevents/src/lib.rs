
pub mod cfn_alarm_model {

#[derive(serde::Serialize, Default)]
pub struct CfnAlarmModel {
    /// A brief description of the alarm model.
    #[serde(rename = "AlarmModelDescription")]
    pub alarm_model_description: Option<String>,
    /// A non-negative integer that reflects the severity level of the alarm.
    /// 
    #[serde(rename = "Severity")]
    pub severity: Option<usize>,
    /// Defines when your alarm is invoked.
    #[serde(rename = "AlarmRule")]
    pub alarm_rule: AlarmRule,
    /// The name of the alarm model.
    #[serde(rename = "AlarmModelName")]
    pub alarm_model_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see [Tag](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html).
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The value used to identify a alarm instance. When a device or system sends input, a new alarm instance with a unique key value is created. AWS IoT Events can continue to route input to its corresponding alarm instance based on this identifying information.
    /// 
    /// This parameter uses a JSON-path expression to select the attribute-value pair in the message payload that is used for identification. To route the message to the correct alarm instance, the device must send a message payload that contains the same attribute-value.
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Contains information about one or more alarm actions.
    #[serde(rename = "AlarmEventActions")]
    pub alarm_event_actions: Option<AlarmEventActions>,
    /// The ARN of the role that grants permission to AWS IoT Events to perform its operations.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// Contains the configuration information of alarm state changes
    #[serde(rename = "AlarmCapabilities")]
    pub alarm_capabilities: Option<AlarmCapabilities>,

}


#[derive(serde::Serialize, Default)]
pub struct AlarmActions {

}

#[derive(serde::Serialize, Default)]
pub struct Payload {
    #[serde(rename = "ContentExpression")]
    pub content_expression: String,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct Lambda {
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmAction {
    #[serde(rename = "Lambda")]
    pub lambda: Option<Lambda>,
    #[serde(rename = "Sqs")]
    pub sqs: Option<Sqs>,
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2>,
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDB>,
    #[serde(rename = "IotTopicPublish")]
    pub iot_topic_publish: Option<IotTopicPublish>,
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEvents>,
    #[serde(rename = "Firehose")]
    pub firehose: Option<Firehose>,
    #[serde(rename = "Sns")]
    pub sns: Option<Sns>,
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWise>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyValue {
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<AssetPropertyTimestamp>,
    #[serde(rename = "Value")]
    pub value: AssetPropertyVariant,
    #[serde(rename = "Quality")]
    pub quality: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyVariant {
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<String>,
    #[serde(rename = "IntegerValue")]
    pub integer_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmEventActions {
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<AlarmActions>,

}

#[derive(serde::Serialize, Default)]
pub struct Sns {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "TargetArn")]
    pub target_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmRule {
    #[serde(rename = "SimpleRule")]
    pub simple_rule: Option<SimpleRule>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDB {
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: String,
    #[serde(rename = "RangeKeyType")]
    pub range_key_type: Option<String>,
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: String,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "RangeKeyValue")]
    pub range_key_value: Option<String>,
    #[serde(rename = "Operation")]
    pub operation: Option<String>,
    #[serde(rename = "PayloadField")]
    pub payload_field: Option<String>,
    #[serde(rename = "HashKeyType")]
    pub hash_key_type: Option<String>,
    #[serde(rename = "RangeKeyField")]
    pub range_key_field: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Firehose {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "Separator")]
    pub separator: Option<String>,
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct IotEvents {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "InputName")]
    pub input_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct IotTopicPublish {
    #[serde(rename = "MqttTopic")]
    pub mqtt_topic: String,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}

#[derive(serde::Serialize, Default)]
pub struct IotSiteWise {
    #[serde(rename = "PropertyId")]
    pub property_id: Option<String>,
    #[serde(rename = "AssetId")]
    pub asset_id: Option<String>,
    #[serde(rename = "EntryId")]
    pub entry_id: Option<String>,
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,
    #[serde(rename = "PropertyValue")]
    pub property_value: Option<AssetPropertyValue>,

}

#[derive(serde::Serialize, Default)]
pub struct Sqs {
    #[serde(rename = "QueueUrl")]
    pub queue_url: String,
    #[serde(rename = "UseBase64")]
    pub use_base64: Option<bool>,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyTimestamp {
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: String,
    #[serde(rename = "OffsetInNanos")]
    pub offset_in_nanos: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SimpleRule {
    #[serde(rename = "Threshold")]
    pub threshold: String,
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,
    #[serde(rename = "InputProperty")]
    pub input_property: String,

}

#[derive(serde::Serialize, Default)]
pub struct AcknowledgeFlow {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct InitializationConfiguration {
    #[serde(rename = "DisabledOnInitialization")]
    pub disabled_on_initialization: bool,

}

#[derive(serde::Serialize, Default)]
pub struct AlarmCapabilities {
    #[serde(rename = "InitializationConfiguration")]
    pub initialization_configuration: Option<InitializationConfiguration>,
    #[serde(rename = "AcknowledgeFlow")]
    pub acknowledge_flow: Option<AcknowledgeFlow>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBv2 {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "TableName")]
    pub table_name: String,

}


}

pub mod cfn_detector_model {

#[derive(serde::Serialize, Default)]
pub struct CfnDetectorModel {
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see [Tag](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html).
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Information about the order in which events are evaluated and how actions are executed.
    #[serde(rename = "EvaluationMethod")]
    pub evaluation_method: Option<String>,
    /// The value used to identify a detector instance. When a device or system sends input, a new detector instance with a unique key value is created. AWS IoT Events can continue to route input to its corresponding detector instance based on this identifying information.
    /// 
    /// This parameter uses a JSON-path expression to select the attribute-value pair in the message payload that is used for identification. To route the message to the correct detector instance, the device must send a message payload that contains the same attribute-value.
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// The ARN of the role that grants permission to AWS IoT Events to perform its operations.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// A brief description of the detector model.
    #[serde(rename = "DetectorModelDescription")]
    pub detector_model_description: Option<String>,
    /// The name of the detector model.
    #[serde(rename = "DetectorModelName")]
    pub detector_model_name: Option<String>,
    /// Information that defines how a detector operates.
    #[serde(rename = "DetectorModelDefinition")]
    pub detector_model_definition: DetectorModelDefinition,

}


#[derive(serde::Serialize, Default)]
pub struct TransitionEvent {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "Condition")]
    pub condition: String,
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "NextState")]
    pub next_state: String,

}

#[derive(serde::Serialize, Default)]
pub struct DetectorModelDefinition {
    #[serde(rename = "InitialStateName")]
    pub initial_state_name: String,
    #[serde(rename = "States")]
    pub states: Vec<State>,

}

#[derive(serde::Serialize, Default)]
pub struct ResetTimer {
    #[serde(rename = "TimerName")]
    pub timer_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDBv2 {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "TableName")]
    pub table_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SetTimer {
    #[serde(rename = "DurationExpression")]
    pub duration_expression: Option<String>,
    #[serde(rename = "Seconds")]
    pub seconds: Option<usize>,
    #[serde(rename = "TimerName")]
    pub timer_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Sqs {
    #[serde(rename = "UseBase64")]
    pub use_base64: Option<bool>,
    #[serde(rename = "QueueUrl")]
    pub queue_url: String,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyVariant {
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<String>,
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "IntegerValue")]
    pub integer_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Sns {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "TargetArn")]
    pub target_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct State {
    #[serde(rename = "StateName")]
    pub state_name: String,
    #[serde(rename = "OnEnter")]
    pub on_enter: Option<OnEnter>,
    #[serde(rename = "OnInput")]
    pub on_input: Option<OnInput>,
    #[serde(rename = "OnExit")]
    pub on_exit: Option<OnExit>,

}

#[derive(serde::Serialize, Default)]
pub struct Event {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ClearTimer {
    #[serde(rename = "TimerName")]
    pub timer_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Payload {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "ContentExpression")]
    pub content_expression: String,

}

#[derive(serde::Serialize, Default)]
pub struct OnInput {
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,
    #[serde(rename = "TransitionEvents")]
    pub transition_events: Option<Vec<TransitionEvent>>,

}

#[derive(serde::Serialize, Default)]
pub struct IotEvents {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "InputName")]
    pub input_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Lambda {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct OnExit {
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "ClearTimer")]
    pub clear_timer: Option<ClearTimer>,
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDB>,
    #[serde(rename = "ResetTimer")]
    pub reset_timer: Option<ResetTimer>,
    #[serde(rename = "Lambda")]
    pub lambda: Option<Lambda>,
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2>,
    #[serde(rename = "SetVariable")]
    pub set_variable: Option<SetVariable>,
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEvents>,
    #[serde(rename = "Sns")]
    pub sns: Option<Sns>,
    #[serde(rename = "IotTopicPublish")]
    pub iot_topic_publish: Option<IotTopicPublish>,
    #[serde(rename = "SetTimer")]
    pub set_timer: Option<SetTimer>,
    #[serde(rename = "Sqs")]
    pub sqs: Option<Sqs>,
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWise>,
    #[serde(rename = "Firehose")]
    pub firehose: Option<Firehose>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct IotSiteWise {
    #[serde(rename = "AssetId")]
    pub asset_id: Option<String>,
    #[serde(rename = "EntryId")]
    pub entry_id: Option<String>,
    #[serde(rename = "PropertyId")]
    pub property_id: Option<String>,
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,
    #[serde(rename = "PropertyValue")]
    pub property_value: AssetPropertyValue,

}

#[derive(serde::Serialize, Default)]
pub struct IotTopicPublish {
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "MqttTopic")]
    pub mqtt_topic: String,

}

#[derive(serde::Serialize, Default)]
pub struct SetVariable {
    #[serde(rename = "VariableName")]
    pub variable_name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct OnEnter {
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamoDB {
    #[serde(rename = "HashKeyType")]
    pub hash_key_type: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: String,
    #[serde(rename = "RangeKeyType")]
    pub range_key_type: Option<String>,
    #[serde(rename = "RangeKeyValue")]
    pub range_key_value: Option<String>,
    #[serde(rename = "PayloadField")]
    pub payload_field: Option<String>,
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: String,
    #[serde(rename = "RangeKeyField")]
    pub range_key_field: Option<String>,
    #[serde(rename = "Operation")]
    pub operation: Option<String>,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyValue {
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<AssetPropertyTimestamp>,
    #[serde(rename = "Value")]
    pub value: AssetPropertyVariant,
    #[serde(rename = "Quality")]
    pub quality: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AssetPropertyTimestamp {
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: String,
    #[serde(rename = "OffsetInNanos")]
    pub offset_in_nanos: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Firehose {
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,
    #[serde(rename = "Separator")]
    pub separator: Option<String>,

}


}

pub mod cfn_input {

#[derive(serde::Serialize, Default)]
pub struct CfnInput {
    /// A brief description of the input.
    #[serde(rename = "InputDescription")]
    pub input_description: Option<String>,
    /// The definition of the input.
    #[serde(rename = "InputDefinition")]
    pub input_definition: InputDefinition,
    /// The name of the input.
    #[serde(rename = "InputName")]
    pub input_name: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see [Tag](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resource-tags.html).
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct InputDefinition {
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,

}

#[derive(serde::Serialize, Default)]
pub struct Attribute {
    #[serde(rename = "JsonPath")]
    pub json_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
