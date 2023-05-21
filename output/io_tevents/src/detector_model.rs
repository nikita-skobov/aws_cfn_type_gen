/// The AWS::IoTEvents::DetectorModel resource creates a detector model. You create a detector    model (a model of your equipment or process) using states. For each    state, you define conditional (Boolean) logic that evaluates the incoming inputs to detect significant    events. When an event is detected, it can change the state or trigger custom-built or predefined    actions using other AWS services. You can define additional events that trigger actions when entering    or exiting a state and, optionally, when a condition is met. For more information, see        How to Use AWS IoT Events in the AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDetectorModel {
    ///
    /// Information that defines how a detector operates.
    ///
    /// Required: Yes
    ///
    /// Type: DetectorModelDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetectorModelDefinition")]
    pub detector_model_definition: DetectorModelDefinition,

    ///
    /// A brief description of the detector model.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetectorModelDescription")]
    pub detector_model_description: Option<String>,

    ///
    /// The name of the detector model.
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
    #[serde(rename = "DetectorModelName")]
    pub detector_model_name: Option<String>,

    ///
    /// Information about the order in which events are evaluated and how actions are executed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BATCH | SERIAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationMethod")]
    pub evaluation_method: Option<DetectorModelEvaluationMethodEnum>,

    ///
    /// The value used to identify a detector instance. When a device or system sends input, a new    detector instance with a unique key value is created. AWS IoT Events can continue to route input to its    corresponding detector instance based on this identifying information.
    ///
    /// This parameter uses a JSON-path expression to select the attribute-value pair in the    message payload that is used for identification. To route the message to the correct detector    instance, the device must send a message payload that contains the same    attribute-value.
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
    /// The ARN of the role that grants permission to AWS IoT Events to perform its operations.
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
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DetectorModelEvaluationMethodEnum {
    /// BATCH
    #[serde(rename = "BATCH")]
    Batch,

    /// SERIAL
    #[serde(rename = "SERIAL")]
    Serial,
}

impl Default for DetectorModelEvaluationMethodEnum {
    fn default() -> Self {
        DetectorModelEvaluationMethodEnum::Batch
    }
}

impl cfn_resources::CfnResource for CfnDetectorModel {
    fn type_string(&self) -> &'static str {
        "AWS::IoTEvents::DetectorModel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.detector_model_definition.validate()?;

        if let Some(the_val) = &self.detector_model_description {
            if the_val.len() > 128 as _ {
                return Err(format!("Max validation failed on field 'detector_model_description'. {} is greater than 128", the_val.len()));
            }
        }

        if let Some(the_val) = &self.detector_model_name {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'detector_model_name'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.detector_model_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'detector_model_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.key {
            if the_val.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 128",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.key {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An action to be performed when the condition is TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Action {
    ///
    /// Information needed to clear the timer.
    ///
    /// Required: No
    ///
    /// Type: ClearTimer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClearTimer")]
    pub clear_timer: Option<ClearTimer>,

    ///
    /// Writes to the DynamoDB table that you created. The default action payload contains all    attribute-value pairs that have the information about the detector model instance and the    event that triggered the action. You can customize the payload. One column of the    DynamoDB table receives all attribute-value pairs in the payload that you specify. For more    information, see Actions in             AWS IoT Events Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: DynamoDB
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDB")]
    pub dynamo_db: Option<DynamoDB>,

    ///
    /// Writes to the DynamoDB table that you created. The default action payload contains all    attribute-value pairs that have the information about the detector model instance and the    event that triggered the action. You can customize the payload. A separate column of    the DynamoDB table receives one attribute-value pair in the payload that you specify. For more    information, see Actions in             AWS IoT Events Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: DynamoDBv2
    ///
    /// Update requires: No interruption
    #[serde(rename = "DynamoDBv2")]
    pub dynamo_dbv2: Option<DynamoDBv2>,

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
    /// Sends AWS IoT Events input, which passes information about the detector model instance and the    event that triggered the action.
    ///
    /// Required: No
    ///
    /// Type: IotEvents
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEvents")]
    pub iot_events: Option<IotEvents>,

    ///
    /// Sends information about the detector model instance and the event that triggered the    action to an asset property in AWS IoT SiteWise .
    ///
    /// Required: No
    ///
    /// Type: IotSiteWise
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotSiteWise")]
    pub iot_site_wise: Option<IotSiteWise>,

    ///
    /// Publishes an MQTT message with the given topic to the AWS IoT message broker.
    ///
    /// Required: No
    ///
    /// Type: IotTopicPublish
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotTopicPublish")]
    pub iot_topic_publish: Option<IotTopicPublish>,

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
    /// Information needed to reset the timer.
    ///
    /// Required: No
    ///
    /// Type: ResetTimer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResetTimer")]
    pub reset_timer: Option<ResetTimer>,

    ///
    /// Information needed to set the timer.
    ///
    /// Required: No
    ///
    /// Type: SetTimer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetTimer")]
    pub set_timer: Option<SetTimer>,

    ///
    /// Sets a variable to a specified value.
    ///
    /// Required: No
    ///
    /// Type: SetVariable
    ///
    /// Update requires: No interruption
    #[serde(rename = "SetVariable")]
    pub set_variable: Option<SetVariable>,

    ///
    /// Sends an Amazon SNS message.
    ///
    /// Required: No
    ///
    /// Type: Sns
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    pub sns: Option<Sns>,

    ///
    /// Sends an Amazon SNS message.
    ///
    /// Required: No
    ///
    /// Type: Sqs
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqs")]
    pub sqs: Option<Sqs>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.clear_timer
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dynamo_db
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dynamo_dbv2
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.firehose
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_events
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_site_wise
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.iot_topic_publish
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.lambda.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.reset_timer
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.set_timer
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.set_variable
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sns.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sqs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that contains timestamp information. For more information, see TimeInNanos in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyTimestamp. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AssetPropertyTimestamp {
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
}

impl cfn_resources::CfnResource for AssetPropertyTimestamp {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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
    /// The timestamp associated with the asset property value. The default is the current event    time.
    ///
    /// Required: No
    ///
    /// Type: AssetPropertyTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<AssetPropertyTimestamp>,

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
}

impl cfn_resources::CfnResource for AssetPropertyValue {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.timestamp
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.value.validate()?;

        Ok(())
    }
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
    /// The asset property value is a Boolean value that must be 'TRUE' or     'FALSE'. You must use an expression, and the evaluated result should be a    Boolean value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<String>,

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
    /// The asset property value is a string. You must use an expression, and the evaluated result    should be a string.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,
}

impl cfn_resources::CfnResource for AssetPropertyVariant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information needed to clear the timer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClearTimer {
    ///
    /// The name of the timer to clear.
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
    #[serde(rename = "TimerName")]
    pub timer_name: String,
}

impl cfn_resources::CfnResource for ClearTimer {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.timer_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'timer_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.timer_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'timer_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information that defines how a detector operates.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DetectorModelDefinition {
    ///
    /// The state that is entered at the creation of each detector (instance).
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
    #[serde(rename = "InitialStateName")]
    pub initial_state_name: String,

    ///
    /// Information about the states of the detector.
    ///
    /// Required: Yes
    ///
    /// Type: List of State
    ///
    /// Update requires: No interruption
    #[serde(rename = "States")]
    pub states: Vec<State>,
}

impl cfn_resources::CfnResource for DetectorModelDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.initial_state_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'initial_state_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.initial_state_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'initial_state_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
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
    /// The name of the DynamoDB table. The tableName value must match the table name of    the target DynamoDB table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,
}

impl cfn_resources::CfnResource for DynamoDB {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    /// The name of the DynamoDB table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: String,
}

impl cfn_resources::CfnResource for DynamoDBv2 {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the actions to be performed when the condition    evaluates to TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Event {
    ///
    /// The actions to be performed.
    ///
    /// Required: No
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,

    ///
    /// Optional. The Boolean expression that, when TRUE, causes the actions to be    performed. If not present, the actions are performed (=TRUE). If the expression result is not    a Boolean value, the actions are not performed (=FALSE).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: Option<String>,

    ///
    /// The name of the event.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventName")]
    pub event_name: String,
}

impl cfn_resources::CfnResource for Event {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.condition {
            if the_val.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'condition'. {} is greater than 512",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.event_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'event_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Sends information about the detector model instance and the event that triggered the    action to an Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Firehose {
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

impl cfn_resources::CfnResource for Firehose {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Sends an AWS IoT Events input, passing in information about the detector model instance and the    event that triggered the action.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IotEvents {
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
}

impl cfn_resources::CfnResource for IotEvents {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.input_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'input_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.input_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'input_name'. {} is less than 1",
                the_val.len()
            ));
        }

        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
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
    /// The alias of the asset property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyAlias")]
    pub property_alias: Option<String>,

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
    /// The value to send to the asset property. This value contains timestamp, quality, and value    (TQV) information.
    ///
    /// Required: Yes
    ///
    /// Type: AssetPropertyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyValue")]
    pub property_value: AssetPropertyValue,
}

impl cfn_resources::CfnResource for IotSiteWise {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.property_value.validate()?;

        Ok(())
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

impl cfn_resources::CfnResource for IotTopicPublish {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.mqtt_topic;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'mqtt_topic'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.mqtt_topic;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'mqtt_topic'. {} is less than 1",
                the_val.len()
            ));
        }

        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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

impl cfn_resources::CfnResource for Lambda {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.function_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'function_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.function_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'function_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// When entering this state, perform these actions if the condition    is TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnEnter {
    ///
    /// Specifies the actions that are performed when the state is entered and the     condition is TRUE.
    ///
    /// Required: No
    ///
    /// Type: List of Event
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,
}

impl cfn_resources::CfnResource for OnEnter {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// When exiting this state, perform these actions if the specified     condition is TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnExit {
    ///
    /// Specifies the actions that are performed when the state is exited and the     condition is TRUE.
    ///
    /// Required: No
    ///
    /// Type: List of Event
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,
}

impl cfn_resources::CfnResource for OnExit {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the actions performed when the condition evaluates to TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnInput {
    ///
    /// Specifies the actions performed when the condition evaluates to TRUE.
    ///
    /// Required: No
    ///
    /// Type: List of Event
    ///
    /// Update requires: No interruption
    #[serde(rename = "Events")]
    pub events: Option<Vec<Event>>,

    ///
    /// Specifies the actions performed, and the next state entered, when a condition    evaluates to TRUE.
    ///
    /// Required: No
    ///
    /// Type: List of TransitionEvent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitionEvents")]
    pub transition_events: Option<Vec<TransitionEvent>>,
}

impl cfn_resources::CfnResource for OnInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl cfn_resources::CfnResource for Payload {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.content_expression;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'content_expression'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information required to reset the timer. The timer is reset to the previously evaluated    result of the duration. The duration expression isn't reevaluated when you reset the    timer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResetTimer {
    ///
    /// The name of the timer to reset.
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
    #[serde(rename = "TimerName")]
    pub timer_name: String,
}

impl cfn_resources::CfnResource for ResetTimer {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.timer_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'timer_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.timer_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'timer_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information needed to set the timer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SetTimer {
    ///
    /// The duration of the timer, in seconds. You can use a string expression that includes    numbers, variables ($variable.<variable-name>), and input values     ($input.<input-name>.<path-to-datum>) as the duration. The range of    the duration is 1-31622400 seconds. To ensure accuracy, the minimum duration is 60 seconds.    The evaluated result of the duration is rounded down to the nearest whole number.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "DurationExpression")]
    pub duration_expression: Option<String>,

    ///
    /// The number of seconds until the timer expires. The minimum value is 60 seconds to ensure    accuracy. The maximum value is 31622400 seconds.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 31622400
    ///
    /// Update requires: No interruption
    #[serde(rename = "Seconds")]
    pub seconds: Option<i64>,

    ///
    /// The name of the timer.
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
    #[serde(rename = "TimerName")]
    pub timer_name: String,
}

impl cfn_resources::CfnResource for SetTimer {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.duration_expression {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'duration_expression'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.duration_expression {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'duration_expression'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.seconds {
            if *the_val > 31622400 as _ {
                return Err(format!(
                    "Max validation failed on field 'seconds'. {} is greater than 31622400",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.seconds {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'seconds'. {} is less than 1",
                    the_val
                ));
            }
        }

        let the_val = &self.timer_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'timer_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.timer_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'timer_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Information about the variable and its new value.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SetVariable {
    ///
    /// The new value of the variable.
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
    pub value: String,

    ///
    /// The name of the variable.
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
    #[serde(rename = "VariableName")]
    pub variable_name: String,
}

impl cfn_resources::CfnResource for SetVariable {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.value;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.variable_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'variable_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.variable_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'variable_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for Sns {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.target_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'target_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.target_arn;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'target_arn'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Sends information about the detector model instance and the event that triggered the    action to an Amazon SQS queue.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sqs {
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
}

impl cfn_resources::CfnResource for Sqs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information that defines a state of a detector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct State {
    ///
    /// When entering this state, perform these actions if the condition    is TRUE.
    ///
    /// Required: No
    ///
    /// Type: OnEnter
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnEnter")]
    pub on_enter: Option<OnEnter>,

    ///
    /// When exiting this state, perform these actions if the specified     condition is TRUE.
    ///
    /// Required: No
    ///
    /// Type: OnExit
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnExit")]
    pub on_exit: Option<OnExit>,

    ///
    /// When an input is received and the condition is TRUE, perform the specified     actions.
    ///
    /// Required: No
    ///
    /// Type: OnInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnInput")]
    pub on_input: Option<OnInput>,

    ///
    /// The name of the state.
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
    #[serde(rename = "StateName")]
    pub state_name: String,
}

impl cfn_resources::CfnResource for State {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.on_enter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.on_exit.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.on_input
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.state_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'state_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.state_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'state_name'. {} is less than 1",
                the_val.len()
            ));
        }

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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the actions performed and the next state entered when a condition    evaluates to TRUE.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransitionEvent {
    ///
    /// The actions to be performed.
    ///
    /// Required: No
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,

    ///
    /// Required. A Boolean expression that when TRUE causes the actions to be performed and the     nextState to be entered.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Update requires: No interruption
    #[serde(rename = "Condition")]
    pub condition: String,

    ///
    /// The name of the transition event.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventName")]
    pub event_name: String,

    ///
    /// The next state to enter.
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
    #[serde(rename = "NextState")]
    pub next_state: String,
}

impl cfn_resources::CfnResource for TransitionEvent {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.condition;

        if the_val.len() > 512 as _ {
            return Err(format!(
                "Max validation failed on field 'condition'. {} is greater than 512",
                the_val.len()
            ));
        }

        let the_val = &self.event_name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'event_name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.next_state;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'next_state'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.next_state;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'next_state'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}
