/// Represents an alarm model to monitor an AWS IoT Events input attribute. You can use the alarm to get    notified when the value is outside a specified range. For more information, see Create an     alarm model in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAlarmModel {
    ///
    /// Contains the configuration information of alarm state changes.
    ///
    /// Required: No
    ///
    /// Type: AlarmCapabilities
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmCapabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_capabilities: Option<AlarmCapabilities>,

    ///
    /// Contains information about one or more alarm actions.
    ///
    /// Required: No
    ///
    /// Type: AlarmEventActions
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmEventActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_model_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

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
    pub role_arn: cfn_resources::StrVal,

    ///
    /// A non-negative integer that reflects the severity level of the alarm.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnAlarmModel {
    fn type_string(&self) -> &'static str {
        "AWS::IoTEvents::AlarmModel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.alarm_capabilities
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.alarm_event_actions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.alarm_model_description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'alarm_model_description'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.alarm_model_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'alarm_model_name'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.alarm_model_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'alarm_model_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.alarm_rule.validate()?;

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies whether to get notified for alarm state changes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl cfn_resources::CfnResource for AcknowledgeFlow {
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

/// Specifies one of the following actions to receive notifications when the alarm state    changes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AlarmAction {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<DynamoDB>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<Firehose>,

    ///
    /// Sends an AWS IoT Events input, passing in information about the detector model instance and the    event that triggered the action.
    ///
    /// Required: No
    ///
    /// Type: IotEvents
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEvents>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise: Option<IotSiteWise>,

    ///
    /// Information required to publish the MQTT message through the AWS IoT message broker.
    ///
    /// Required: No
    ///
    /// Type: IotTopicPublish
    ///
    /// Update requires: No interruption
    #[serde(rename = "IotTopicPublish")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<Lambda>,

    ///
    /// Information required to publish the Amazon SNS message.
    ///
    /// Required: No
    ///
    /// Type: Sns
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<Sns>,

    ///
    /// Sends information about the detector model instance and the event that triggered the    action to an Amazon SQS queue.
    ///
    /// Required: No
    ///
    /// Type: Sqs
    ///
    /// Update requires: No interruption
    #[serde(rename = "Sqs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<Sqs>,
}

impl cfn_resources::CfnResource for AlarmAction {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        self.sns.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sqs.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains the configuration information of alarm state changes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_configuration: Option<InitializationConfiguration>,
}

impl cfn_resources::CfnResource for AlarmCapabilities {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acknowledge_flow
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.initialization_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Contains information about one or more alarm actions.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<AlarmAction>>,
}

impl cfn_resources::CfnResource for AlarmEventActions {
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

/// Defines when your alarm is invoked.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_rule: Option<SimpleRule>,
}

impl cfn_resources::CfnResource for AlarmRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.simple_rule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A structure that contains timestamp information. For more information, see TimeInNanos in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyTimestamp. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_in_nanos: Option<cfn_resources::StrVal>,

    ///
    /// The timestamp, in seconds, in the Unix epoch format. The valid range is between    1-31556889864403199.
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

/// A structure that contains value information. For more information, see AssetPropertyValue in the         AWS IoT SiteWise API Reference.
///
/// You must use expressions for all parameters in AssetPropertyValue. The    expressions accept literals, operators, functions, references, and substitution    templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<cfn_resources::StrVal>,

    ///
    /// The timestamp associated with the asset property value. The default is the current event    time.
    ///
    /// Required: No
    ///
    /// Type: AssetPropertyTimestamp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<cfn_resources::StrVal>,

    ///
    /// The asset property value is a double. You must use an expression, and the evaluated result    should be a double.
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
    /// The asset property value is an integer. You must use an expression, and the evaluated    result should be an integer.
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
    /// The asset property value is a string. You must use an expression, and the evaluated result    should be a string.
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

/// Defines an action to write to the Amazon DynamoDB table that you created. The standard action    payload contains all the information about the detector model instance and the event that    triggered the action. You can customize the payload. One column of the    DynamoDB table receives all attribute-value pairs in the payload that you specify.
///
/// You must use expressions for all parameters in DynamoDBAction. The expressions    accept literals, operators, functions, references, and substitution templates.
///
/// For more information,     see Expressions     in the         AWS IoT Events Developer Guide.
///
/// If the defined payload type is a string, DynamoDBAction writes non-JSON data to    the DynamoDB table as binary data. The DynamoDB console displays the data as Base64-encoded text.    The value for the payloadField parameter is     <payload-field>_raw.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub hash_key_field: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_type: Option<cfn_resources::StrVal>,

    ///
    /// The value of the hash key (also called the partition key).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HashKeyValue")]
    pub hash_key_value: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_field: Option<cfn_resources::StrVal>,

    ///
    /// The name of the range key (also called the sort key). The rangeKeyField value    must match the sort key of the target DynamoDB table.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_type: Option<cfn_resources::StrVal>,

    ///
    /// The value of the range key (also called the sort key).
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
    /// The name of the DynamoDB table. The tableName value must match the table name of    the target DynamoDB table.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableName")]
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DynamoDB {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub table_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for DynamoDBv2 {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Sends information about the detector model instance and the event that triggered the    action to an Amazon Kinesis Data Firehose delivery stream.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub delivery_stream_name: cfn_resources::StrVal,

    ///
    /// You can configure the action payload when you send a message to an Amazon Kinesis Data Firehose delivery    stream.
    ///
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Firehose {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the default alarm state. The configuration applies to all alarms that were created based on this alarm model.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for InitializationConfiguration {
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

/// Sends an AWS IoT Events input, passing in information about the detector model instance and the    event that triggered the action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub input_name: cfn_resources::StrVal,

    ///
    /// You can configure the action payload when you send a message to an AWS IoT Events input.
    ///
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

impl cfn_resources::CfnResource for IotEvents {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.input_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'input_name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.input_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'input_name'. {} is less than 1",
                    s.len()
                ));
            }
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<cfn_resources::StrVal>,

    ///
    /// A unique identifier for this entry. You can use the entry ID to track which data entry    causes an error in case of failure. The default is a new unique identifier.
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
    /// The alias of the asset property.
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
    /// The ID of the asset property.
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
    /// The value to send to the asset property. This value contains timestamp, quality, and value    (TQV) information.
    ///
    /// Required: No
    ///
    /// Type: AssetPropertyValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<AssetPropertyValue>,
}

impl cfn_resources::CfnResource for IotSiteWise {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.property_value
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information required to publish the MQTT message through the AWS IoT message broker.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub mqtt_topic: cfn_resources::StrVal,

    ///
    /// You can configure the action payload when you publish a message to an AWS IoT Core    topic.
    ///
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

impl cfn_resources::CfnResource for IotTopicPublish {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.mqtt_topic;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'mqtt_topic'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.mqtt_topic;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'mqtt_topic'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Calls a Lambda function, passing in information about the detector model instance and the    event that triggered the action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub function_arn: cfn_resources::StrVal,

    ///
    /// You can configure the action payload when you send a message to a Lambda function.
    ///
    /// Required: No
    ///
    /// Type: Payload
    ///
    /// Update requires: No interruption
    #[serde(rename = "Payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

impl cfn_resources::CfnResource for Lambda {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.function_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'function_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.function_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'function_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Information needed to configure the payload.
///
/// By default, AWS IoT Events generates a standard payload in JSON for any action. This action payload    contains all attribute-value pairs that have the information about the detector model instance    and the event triggered the action. To configure the action payload, you can use     contentExpression.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub content_expression: cfn_resources::StrVal,

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.content_expression;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'content_expression'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A rule that compares an input property value to a threshold value with a comparison operator.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SimpleRule {
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
    pub input_property: cfn_resources::StrVal,

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
    pub threshold: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for SimpleRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.input_property;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'input_property'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.input_property;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'input_property'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.threshold;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'threshold'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.threshold;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'threshold'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Information required to publish the Amazon SNS message.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub target_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Sns {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.target_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.target_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Sends information about the detector model instance and the event that triggered the    action to an Amazon SQS queue.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub queue_url: cfn_resources::StrVal,

    ///
    /// Set this to TRUE if you want the data to be base-64 encoded before it is written to the    queue. Otherwise, set this to FALSE.
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

impl cfn_resources::CfnResource for Sqs {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.payload.as_ref().map_or(Ok(()), |val| val.validate())?;

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
#[serde(default)]
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
