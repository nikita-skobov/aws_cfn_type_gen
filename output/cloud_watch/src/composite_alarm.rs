

/// The AWS::CloudWatch::CompositeAlarm type creates or updates a composite alarm. When you create       a composite alarm, you specify a rule expression for the alarm that takes into       account the alarm states of other alarms that you have created. The composite alarm goes into ALARM state       only if all conditions of the rule are met.
///
/// The alarms specified in a composite alarm's rule expression can include metric alarms and other composite alarms.
///
/// Using composite alarms can reduce alarm noise. You can create multiple metric alarms, and also create a composite alarm and set       up alerts only for the composite alarm. For example, you could create a composite alarm that goes into ALARM state       only when more than one of the underlying metric alarms are in ALARM state.
///
/// Currently, the only alarm actions that can be taken by composite alarms are notifying SNS topics.
///
/// When this operation creates an alarm, the alarm state is immediately set to INSUFFICIENT_DATA. The alarm is then evaluated and       its state is set appropriately. Any actions associated with the new state are then executed. For a composite alarm, this initial       time after creation is the only time that the alarm can be in INSUFFICIENT_DATA state.
///
/// When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCompositeAlarm {


    /// 
    /// The actions to execute when this alarm transitions to the INSUFFICIENT_DATA state from any other state. Each action is specified as an Amazon Resource Name (ARN).       For more information about creating alarms and the actions       that you can specify, see PutCompositeAlarm in the       Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsufficientDataActions")]
    pub insufficient_data_actions: Option<Vec<String>>,


    /// 
    /// Actions will be suppressed       if the suppressor alarm is       in the ALARM state.       ActionsSuppressor can be an AlarmName or an Amazon Resource Name (ARN)       from an existing alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionsSuppressor")]
    pub actions_suppressor: Option<String>,


    /// 
    /// The actions to execute when this alarm transitions to the ALARM state from any other state. Each action is specified as an Amazon Resource Name (ARN).       For more information about creating alarms and the actions     that you can specify, see PutCompositeAlarm in the     Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmActions")]
    pub alarm_actions: Option<Vec<String>>,


    /// 
    /// Indicates whether actions should be executed during any changes to the alarm state of the composite alarm. The default is TRUE.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionsEnabled")]
    pub actions_enabled: Option<bool>,


    /// 
    /// The description for the composite alarm.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmDescription")]
    pub alarm_description: Option<String>,


    /// The maximum time     in seconds     that the composite alarm waits     for the suppressor alarm     to go     into the ALARM state.     After this time,     the composite alarm performs its actions.
    /// 
    /// Important         WaitPeriod         is required only         when ActionsSuppressor is specified.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionsSuppressorWaitPeriod")]
    pub actions_suppressor_wait_period: Option<i64>,


    /// 
    /// An expression that specifies which other alarms are to be evaluated to determine this composite alarm's state. For each       alarm that you reference, you designate a function that specifies whether that alarm needs to be in ALARM state, OK state,       or INSUFFICIENT_DATA state. You can use operators (AND, OR and NOT) to combine multiple functions in a       single expression. You can use parenthesis to logically group the functions in your expression.
    /// 
    /// You can use either alarm names or ARNs to reference the other alarms that are to be evaluated.
    /// 
    /// Functions can include the following:
    /// 
    /// ALARM("alarm-name or alarm-ARN") is TRUE if the named alarm is in ALARM state.                                  OK("alarm-name or alarm-ARN") is TRUE if the named alarm is in OK state.                                  INSUFFICIENT_DATA("alarm-name or alarm-ARN") is TRUE if the named alarm is in INSUFFICIENT_DATA state.                        TRUE always evaluates to TRUE.               FALSE always evaluates to FALSE.
    /// 
    /// TRUE and FALSE are useful for testing a complex AlarmRule structure, and for testing your alarm actions.
    /// 
    /// For more information about AlarmRule syntax, see PutCompositeAlarm in the     Amazon CloudWatch API Reference.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AlarmRule")]
    pub alarm_rule: String,


    /// 
    /// The actions to execute when this alarm transitions to the OK state from any other state. Each action is specified as an Amazon Resource Name (ARN).       For more information about creating alarms and the actions       that you can specify, see PutCompositeAlarm in the       Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OKActions")]
    pub okactions: Option<Vec<String>>,


    /// 
    /// The name for the composite alarm. This name must be unique within your AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AlarmName")]
    pub alarm_name: Option<String>,


    /// 
    /// The maximum time     in seconds     that the composite alarm waits     after suppressor alarm goes out     of the ALARM state.     After this time,     the composite alarm performs its actions.
    /// 
    /// Important         ExtensionPeriod         is required only         when ActionsSuppressor is specified.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionsSuppressorExtensionPeriod")]
    pub actions_suppressor_extension_period: Option<i64>,

}

impl cfn_resources::CfnResource for CfnCompositeAlarm {
    fn type_string() -> &'static str {
        "AWS::CloudWatch::CompositeAlarm"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
