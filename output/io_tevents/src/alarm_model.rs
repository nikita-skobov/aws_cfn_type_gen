

/// Represents an alarm model to monitor an AWS IoT Events input attribute. You can use the alarm to get    notified when the value is outside a specified range. For more information, see Create an     alarm model in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAlarmModel {


    /// 
    /// Defines when your alarm is invoked.
    /// 
    /// Required: Yes
    ///
    /// Type: AlarmRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmRule")]
    pub alarm_rule: AlarmRule,


    /// 
    /// An input attribute used as a key to create an alarm. AWS IoT Events routes inputs associated with this key to the alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^((`[\w\- ]+`)|([\w\-]+))(\.((`[\w- ]+`)|([\w\-]+)))*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Key")]
    pub key: Option<String>,


    /// 
    /// The ARN of the IAM role that allows the alarm to perform actions and access AWS resources. For more information, see Amazon Resource Names (ARNs) in the         AWS General Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// Contains information about one or more alarm actions.
    /// 
    /// Required: No
    ///
    /// Type: AlarmEventActions
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmEventActions")]
    pub alarm_event_actions: Option<AlarmEventActions>,


    /// 
    /// The description of the alarm model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmModelDescription")]
    pub alarm_model_description: Option<String>,


    /// 
    /// The name of the alarm model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z0-9_-]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlarmModelName")]
    pub alarm_model_name: Option<String>,


    /// 
    /// Contains the configuration information of alarm state changes.
    /// 
    /// Required: No
    ///
    /// Type: AlarmCapabilities
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmCapabilities")]
    pub alarm_capabilities: Option<AlarmCapabilities>,


    /// 
    /// A non-negative integer that reflects the severity level of the alarm.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    pub severity: Option<i64>,


    /// 
    /// A list of key-value pairs that contain metadata for the alarm model. The tags help you    manage the alarm model. For more information, see Tagging your AWS IoT Events     resources in the         AWS IoT Events Developer Guide.
    /// 
    /// You can create up to 50 tags for one alarm model.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnAlarmModel {
    fn type_string() -> &'static str {
        "AWS::IoTEvents::AlarmModel"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Sends information about the detector model instance and the event that triggered the    action to a specified asset property in AWS IoT SiteWise.
///
/// You must use expressions for all parameters in IotSiteWiseAction. The    expressions accept literals, operators, functions, references, and substitutions    templates.
///
/// You must specify either propertyAlias or both assetId and     propertyId to identify the target asset property in AWS IoT SiteWise.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotSiteWise {


    /// 
    /// The value to send to the asset property. This value contains timestamp, quality, and value    (TQV) information.
    /// 
    /// Required: No
    ///
    /// Type: AssetPropertyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyValue")]
    pub property_value: Option<AssetPropertyValue>,


    /// 
    /// A unique identifier for this entry. You can use the entry ID to track which data entry    causes an error in case of failure. The default is a new unique identifier.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntryId")]
    pub entry_id: Option<String>,


    /// 
    /// The ID of the asset that has the specified property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssetId")]
    pub asset_id: Option<String>,


    /// 
    /// The ID of the asset property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyId")]
    pub property_id: Option<String>,


    /// 
    /// The alias of the asset property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,

}




/// Information needed to configure the payload.
///
/// By default, AWS IoT Events generates a standard payload in JSON for any action. This action payload    contains all attribute-value pairs that have the information about the detector model instance    and the event triggered the action. To configure the action payload, you can use     contentExpression.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Payload {


    /// 
    /// The content of the payload. You can use a string expression that includes quoted strings     ('<string>'), variables ($variable.<variable-name>),    input values ($input.<input-name>.<path-to-datum>), string    concatenations, and quoted strings that contain ${} as the content. The    recommended maximum size of a content expression is 1 KB.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentExpression")]
    pub content_expression: String,


    /// 
    /// The value of the payload type can be either STRING or    JSON.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: JSON | STRING
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: PayloadTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PayloadTypeEnum {

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// STRING
    #[serde(rename = "STRING")]
    String,

}

impl Default for PayloadTypeEnum {
    fn default() -> Self {
        PayloadTypeEnum::Json
    }
}



/// Sends an AWS IoT Events input, passing in information about the detector model instance and the    event that triggered the action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotEvents {


    /// 
    /// You can configure the action payload when you send a message to an AWS IoT Events input.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,


    /// 
    /// The name of the AWS IoT Events input where the data is sent.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[a-zA-Z][a-zA-Z0-9_]*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputName")]
    pub input_name: String,

}




/// Defines an action to write to the Amazon DynamoDB table that you created. The default action    payload contains all the information about the detector model instance and the event that    triggered the action. You can customize the payload. A separate column of    the DynamoDB table receives one attribute-value pair in the payload that you specify.
///
/// You must use expressions for all parameters in DynamoDBv2Action. The expressions    accept literals, operators, functions, references, and substitution templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
///
/// The value for the type parameter in Payload must be     JSON.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamoDBv2 {


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
    /// Information needed to configure the payload.
    /// 
    /// By default, AWS IoT Events generates a standard payload in JSON for any action. This action payload    contains all attribute-value pairs that have the information about the detector model instance    and the event triggered the action. To configure the action payload, you can use     contentExpression.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}




/// Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InitializationConfiguration {


    /// 
    /// The value must be TRUE or FALSE. If FALSE, all    alarm instances created based on the alarm model are activated. The default value is     TRUE.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisabledOnInitialization")]
    pub disabled_on_initialization: bool,

}




/// A structure that contains value information. For more information, see AssetPropertyValue in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyValue. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyValue {


    /// 
    /// The quality of the asset property value. The value must be 'GOOD',     'BAD', or 'UNCERTAIN'.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Quality")]
    pub quality: Option<String>,


    /// 
    /// The value to send to an asset property.
    /// 
    /// Required: Yes
    ///
    /// Type: AssetPropertyVariant
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: AssetPropertyVariant,


    /// 
    /// The timestamp associated with the asset property value. The default is the current event    time.
    /// 
    /// Required: No
    ///
    /// Type: AssetPropertyTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<AssetPropertyTimestamp>,

}




/// Sends information about the detector model instance and the event that triggered the    action to an Amazon SQS queue.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sqs {


    /// 
    /// The URL of the SQS queue where the data is written.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueueUrl")]
    pub queue_url: String,


    /// 
    /// Set this to TRUE if you want the data to be base-64 encoded before it is written to the    queue. Otherwise, set this to FALSE.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseBase64")]
    pub use_base64: Option<bool>,


    /// 
    /// You can configure the action payload when you send a message to an Amazon SQS    queue.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}




/// Contains information about one or more alarm actions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlarmEventActions {


    /// 
    /// Specifies one or more supported actions to receive notifications when the alarm state    changes.
    /// 
    /// Required: No
    ///
    /// Type: List of AlarmAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<Vec<AlarmAction>>,

}




/// Sends information about the detector model instance and the event that triggered the    action to an Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Firehose {


    /// 
    /// You can configure the action payload when you send a message to an Amazon Kinesis Data Firehose delivery    stream.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,


    /// 
    /// The name of the Kinesis Data Firehose delivery stream where the data is written.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,


    /// 
    /// A character separator that is used to separate records written to the Kinesis Data    Firehose delivery stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows    newline), ',' (comma).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ([\n\t])|(\r\n)|(,)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Separator")]
    pub separator: Option<String>,

}




/// Specifies whether to get notified for alarm state changes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AcknowledgeFlow {


    /// 
    /// The value must be TRUE or FALSE. If TRUE, you    receive a notification when the alarm state changes. You must choose to acknowledge the    notification before the alarm state can return to NORMAL. If FALSE,    you won't receive notifications. The alarm automatically changes to the NORMAL    state when the input property value returns to the specified range.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}




/// Calls a Lambda function, passing in information about the detector model instance and the    event that triggered the action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Lambda {


    /// 
    /// The ARN of the Lambda function that is executed.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,


    /// 
    /// You can configure the action payload when you send a message to a Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}




/// A structure that contains timestamp information. For more information, see TimeInNanos in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyTimestamp. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyTimestamp {


    /// 
    /// The timestamp, in seconds, in the Unix epoch format. The valid range is between    1-31556889864403199.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeInSeconds")]
    pub time_in_seconds: String,


    /// 
    /// The nanosecond offset converted from timeInSeconds. The valid range is    between 0-999999999.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OffsetInNanos")]
    pub offset_in_nanos: Option<String>,

}




/// Defines when your alarm is invoked.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlarmRule {


    /// 
    /// A rule that compares an input property value to a threshold value with a comparison operator.
    /// 
    /// Required: No
    ///
    /// Type: SimpleRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleRule")]
    pub simple_rule: Option<SimpleRule>,

}




/// A rule that compares an input property value to a threshold value with a comparison operator.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SimpleRule {


    /// 
    /// The value on the left side of the comparison operator. You can specify an AWS IoT Events input    attribute as an input property.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputProperty")]
    pub input_property: String,


    /// 
    /// The comparison operator.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: EQUAL | GREATER | GREATER_OR_EQUAL | LESS | LESS_OR_EQUAL | NOT_EQUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: SimpleRuleComparisonOperatorEnum,


    /// 
    /// The value on the right side of the comparison operator. You can enter a number or specify    an AWS IoT Events input attribute.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SimpleRuleComparisonOperatorEnum {

    /// EQUAL
    #[serde(rename = "EQUAL")]
    Equal,

    /// GREATER
    #[serde(rename = "GREATER")]
    Greater,

    /// GREATER_OR_EQUAL
    #[serde(rename = "GREATER_OR_EQUAL")]
    Greaterorequal,

    /// LESS
    #[serde(rename = "LESS")]
    Less,

    /// LESS_OR_EQUAL
    #[serde(rename = "LESS_OR_EQUAL")]
    Lessorequal,

    /// NOT_EQUAL
    #[serde(rename = "NOT_EQUAL")]
    Notequal,

}

impl Default for SimpleRuleComparisonOperatorEnum {
    fn default() -> Self {
        SimpleRuleComparisonOperatorEnum::Equal
    }
}



/// Information required to publish the MQTT message through the AWS IoT message broker.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotTopicPublish {


    /// 
    /// The MQTT topic of the message. You can use a string expression that includes variables     ($variable.<variable-name>) and input values     ($input.<input-name>.<path-to-datum>) as the topic string.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "MqttTopic")]
    pub mqtt_topic: String,


    /// 
    /// You can configure the action payload when you publish a message to an AWS IoT Core    topic.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,

}




/// A structure that contains an asset property value. For more information, see Variant    in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyVariant. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
///
/// You must specify one of the following value types, depending on the dataType    of the specified asset property. For more information, see AssetProperty in the             AWS IoT SiteWise API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyVariant {


    /// 
    /// The asset property value is a string. You must use an expression, and the evaluated result    should be a string.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,


    /// 
    /// The asset property value is an integer. You must use an expression, and the evaluated    result should be an integer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntegerValue")]
    pub integer_value: Option<String>,


    /// 
    /// The asset property value is a double. You must use an expression, and the evaluated result    should be a double.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<String>,


    /// 
    /// The asset property value is a Boolean value that must be 'TRUE' or     'FALSE'. You must use an expression, and the evaluated result should be a    Boolean value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,

}




/// Specifies one of the following actions to receive notifications when the alarm state    changes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlarmAction {


    /// 
    /// Sends information about the detector model instance and the event that triggered the    action to a specified asset property in AWS IoT SiteWise.
    /// 
    /// You must use expressions for all parameters in IotSiteWiseAction. The    expressions accept literals, operators, functions, references, and substitutions    templates.
    /// 
    /// Examples                                  For literal values, the expressions must contain single quotes. For example, the value      for the propertyAlias parameter can be       '/company/windfarm/3/turbine/7/temperature'.               For references, you must specify either variables or input values. For example, the      value for the assetId parameter can be       $input.TurbineInput.assetId1.               For a substitution template, you must use ${}, and the template must be      in single quotes. A substitution template can also contain a combination of literals,      operators, functions, references, and substitution templates.        In the following example, the value for the propertyAlias parameter uses      a substitution template.                  'company/windfarm/${$input.TemperatureInput.sensorData.windfarmID}/turbine/       ${$input.TemperatureInput.sensorData.turbineID}/temperature'
    /// 
    /// You must specify either propertyAlias or both assetId and     propertyId to identify the target asset property in AWS IoT SiteWise.
    /// 
    /// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: IotSiteWise
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWise>,


    /// 
    /// Defines an action to write to the Amazon DynamoDB table that you created. The default action    payload contains all the information about the detector model instance and the event that    triggered the action. You can customize the payload. A separate column of    the DynamoDB table receives one attribute-value pair in the payload that you specify.
    /// 
    /// You must use expressions for all parameters in DynamoDBv2Action. The expressions    accept literals, operators, functions, references, and substitution templates.
    /// 
    /// Examples                                         For literal values, the expressions must contain single quotes. For example, the value      for the tableName parameter can be      'GreenhouseTemperatureTable'.               For references, you must specify either variables or input values. For example, the      value for the tableName parameter can be      $variable.ddbtableName.               For a substitution template, you must use ${}, and the template must be      in single quotes. A substitution template can also contain a combination of literals,      operators, functions, references, and substitution templates.        In the following example, the value for the contentExpression parameter      in Payload uses a substitution template.                  '{\"sensorID\": \"${$input.GreenhouseInput.sensor_id}\", \"temperature\":       \"${$input.GreenhouseInput.temperature * 9 / 5 + 32}\"}'                       For a string concatenation, you must use +. A string concatenation can      also contain a combination of literals, operators, functions, references, and substitution      templates.        In the following example, the value for the tableName parameter uses a      string concatenation.                  'GreenhouseTemperatureTable ' + $input.GreenhouseInput.date
    /// 
    /// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
    /// 
    /// The value for the type parameter in Payload must be     JSON.
    /// 
    /// Required: No
    ///
    /// Type: DynamoDBv2
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2>,


    /// 
    /// Defines an action to write to the Amazon DynamoDB table that you created. The standard action    payload contains all the information about the detector model instance and the event that    triggered the action. You can customize the payload. One column of the    DynamoDB table receives all attribute-value pairs in the payload that you specify.
    /// 
    /// You must use expressions for all parameters in DynamoDBAction. The expressions    accept literals, operators, functions, references, and substitution templates.
    /// 
    /// Examples                                         For literal values, the expressions must contain single quotes. For example, the value      for the hashKeyType parameter can be 'STRING'.               For references, you must specify either variables or input values. For example, the      value for the hashKeyField parameter can be       $input.GreenhouseInput.name.               For a substitution template, you must use ${}, and the template must be      in single quotes. A substitution template can also contain a combination of literals,      operators, functions, references, and substitution templates.        In the following example, the value for the hashKeyValue parameter uses a      substitution template.                  '${$input.GreenhouseInput.temperature * 6 / 5 + 32} in Fahrenheit'                       For a string concatenation, you must use +. A string concatenation can      also contain a combination of literals, operators, functions, references, and substitution      templates.        In the following example, the value for the tableName parameter uses a      string concatenation.                  'GreenhouseTemperatureTable ' + $input.GreenhouseInput.date
    /// 
    /// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
    /// 
    /// If the defined payload type is a string, DynamoDBAction writes non-JSON data to    the DynamoDB table as binary data. The DynamoDB console displays the data as Base64-encoded text.    The value for the payloadField parameter is     <payload-field>_raw.
    /// 
    /// Required: No
    ///
    /// Type: DynamoDB
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDB>,


    /// 
    /// Calls a Lambda function, passing in information about the detector model instance and the    event that triggered the action.
    /// 
    /// Required: No
    ///
    /// Type: Lambda
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lambda")]
    pub lambda: Option<Lambda>,


    /// 
    /// Sends information about the detector model instance and the event that triggered the    action to an Amazon Kinesis Data Firehose delivery stream.
    /// 
    /// Required: No
    ///
    /// Type: Firehose
    ///
    /// Update requires: No interruption
    #[serde(rename = "Firehose")]
    pub firehose: Option<Firehose>,


    /// 
    /// Information required to publish the MQTT message through the AWS IoT message broker.
    /// 
    /// Required: No
    ///
    /// Type: IotTopicPublish
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotTopicPublish")]
    pub iot_topic_publish: Option<IotTopicPublish>,


    /// 
    /// Information required to publish the Amazon SNS message.
    /// 
    /// Required: No
    ///
    /// Type: Sns
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    pub sns: Option<Sns>,


    /// 
    /// Sends an AWS IoT Events input, passing in information about the detector model instance and the    event that triggered the action.
    /// 
    /// Required: No
    ///
    /// Type: IotEvents
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEvents>,


    /// 
    /// Sends information about the detector model instance and the event that triggered the    action to an Amazon SQS queue.
    /// 
    /// Required: No
    ///
    /// Type: Sqs
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqs")]
    pub sqs: Option<Sqs>,

}




/// Information required to publish the Amazon SNS message.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sns {


    /// 
    /// You can configure the action payload when you send a message as an Amazon SNS push    notification.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,


    /// 
    /// The ARN of the Amazon SNS target where the message is sent.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetArn")]
    pub target_arn: String,

}




/// Contains the configuration information of alarm state changes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlarmCapabilities {


    /// 
    /// Specifies whether to get notified for alarm state changes.
    /// 
    /// Required: No
    ///
    /// Type: AcknowledgeFlow
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcknowledgeFlow")]
    pub acknowledge_flow: Option<AcknowledgeFlow>,


    /// 
    /// Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.
    /// 
    /// Required: No
    ///
    /// Type: InitializationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InitializationConfiguration")]
    pub initialization_configuration: Option<InitializationConfiguration>,

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




/// Defines an action to write to the Amazon DynamoDB table that you created. The standard action    payload contains all the information about the detector model instance and the event that    triggered the action. You can customize the payload. One column of the    DynamoDB table receives all attribute-value pairs in the payload that you specify.
///
/// You must use expressions for all parameters in DynamoDBAction. The expressions    accept literals, operators, functions, references, and substitution templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
///
/// If the defined payload type is a string, DynamoDBAction writes non-JSON data to    the DynamoDB table as binary data. The DynamoDB console displays the data as Base64-encoded text.    The value for the payloadField parameter is     <payload-field>_raw.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynamoDB {


    /// 
    /// The value of the hash key (also called the partition key).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: String,


    /// 
    /// The value of the range key (also called the sort key).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyValue")]
    pub range_key_value: Option<String>,


    /// 
    /// The name of the hash key (also called the partition key). The hashKeyField    value must match the partition key of the target DynamoDB table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyField")]
    pub hash_key_field: String,


    /// 
    /// The name of the range key (also called the sort key). The rangeKeyField value    must match the sort key of the target DynamoDB table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyField")]
    pub range_key_field: Option<String>,


    /// 
    /// Information needed to configure the payload.
    /// 
    /// By default, AWS IoT Events generates a standard payload in JSON for any action. This action payload    contains all attribute-value pairs that have the information about the detector model instance    and the event triggered the action. To configure the action payload, you can use     contentExpression.
    /// 
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    pub payload: Option<Payload>,


    /// 
    /// The type of operation to perform. You can specify the following values:
    /// 
    /// 'INSERT' - Insert data as a new item into the DynamoDB table. This item uses      the specified hash key as a partition key. If you specified a range key, the item uses the      range key as a sort key.                        'UPDATE' - Update an existing item of the DynamoDB table with new data. This      item's partition key must match the specified hash key. If you specified a range key, the      range key must match the item's sort key.                        'DELETE' - Delete an existing item of the DynamoDB table. This item's      partition key must match the specified hash key. If you specified a range key, the range      key must match the item's sort key.
    /// 
    /// If you don't specify this parameter, AWS IoT Events triggers the 'INSERT'    operation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Operation")]
    pub operation: Option<String>,


    /// 
    /// The name of the DynamoDB column that receives the action payload.
    /// 
    /// If you don't specify this parameter, the name of the DynamoDB column is    payload.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PayloadField")]
    pub payload_field: Option<String>,


    /// 
    /// The data type for the hash key (also called the partition key). You can specify the    following values:
    /// 
    /// 'STRING' - The hash key is a string.                        'NUMBER' - The hash key is a number.
    /// 
    /// If you don't specify hashKeyType, the default value is    'STRING'.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyType")]
    pub hash_key_type: Option<String>,


    /// 
    /// The name of the DynamoDB table. The tableName value must match the table name of    the target DynamoDB table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The data type for the range key (also called the sort key), You can specify the following    values:
    /// 
    /// 'STRING' - The range key is a string.                        'NUMBER' - The range key is number.
    /// 
    /// If you don't specify rangeKeyField, the default value is     'STRING'.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RangeKeyType")]
    pub range_key_type: Option<String>,

}


