/// The AWS::Glue::Trigger resource specifies triggers that run AWS Glue       jobs. For more information, see Triggering Jobs in AWS Glue and Trigger Structure in the AWS Glue Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTrigger {
    ///
    /// The actions initiated by this trigger.
    ///
    /// Required: Yes
    ///
    /// Type: List of Action
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,

    ///
    /// A description of this trigger.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires.
    ///
    /// Required: No
    ///
    /// Type: EventBatchingCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBatchingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_batching_condition: Option<EventBatchingCondition>,

    ///
    /// The name of the trigger.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The predicate of this trigger, which defines when it will fire.
    ///
    /// Required: No
    ///
    /// Type: Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,

    ///
    /// A cron expression used to specify the schedule. For more information, see         Time-Based Schedules for         Jobs and Crawlers in the AWS Glue Developer Guide. For       example, to run something every day at 12:15 UTC, specify cron(15 12 * * ?         *).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<cfn_resources::StrVal>,

    /// Set to true to start SCHEDULED and CONDITIONAL triggers when created. True is not supported for ON_DEMAND triggers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartOnCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on_creation: Option<bool>,

    ///
    /// The tags to use with this trigger.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    ///
    /// The type of trigger that this is.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CONDITIONAL | EVENT | ON_DEMAND | SCHEDULED
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: TriggerTypeEnum,

    /// The name of the workflow associated with the trigger.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkflowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TriggerTypeEnum {
    /// CONDITIONAL
    #[serde(rename = "CONDITIONAL")]
    Conditional,

    /// EVENT
    #[serde(rename = "EVENT")]
    Event,

    /// ON_DEMAND
    #[serde(rename = "ON_DEMAND")]
    Ondemand,

    /// SCHEDULED
    #[serde(rename = "SCHEDULED")]
    Scheduled,
}

impl Default for TriggerTypeEnum {
    fn default() -> Self {
        TriggerTypeEnum::Conditional
    }
}

impl cfn_resources::CfnResource for CfnTrigger {
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Trigger"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.event_batching_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.predicate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Defines an action to be initiated by a trigger.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Action {
    ///
    /// The job arguments used when this trigger fires. For this job run, they replace the       default arguments set in the job definition itself.
    ///
    /// You can specify arguments here that your own job-execution script consumes, in       addition to arguments that AWS Glue itself consumes.
    ///
    /// For information about how to specify and consume your own job arguments, see Calling AWS Glue APIs in Python in the AWS Glue Developer         Guide.
    ///
    /// For information about the key-value pairs that AWS Glue consumes to set up your job,       see the Special Parameters         Used by AWS Glue topic in the developer guide.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,

    /// The name of the crawler to be used with this action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of a job to be executed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<cfn_resources::StrVal>,

    /// Specifies configuration properties of a job run notification.
    ///
    /// Required: No
    ///
    /// Type: NotificationProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_property: Option<NotificationProperty>,

    ///
    /// The name of the SecurityConfiguration structure to be used with this       action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_configuration: Option<cfn_resources::StrVal>,

    /// The JobRun timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters TIMEOUT status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl cfn_resources::CfnResource for Action {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'job_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'job_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.notification_property
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'security_configuration'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.security_configuration {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'security_configuration'. {} is less than 1", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Defines a condition under which a trigger fires.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Condition {
    /// The state of the crawler to which this condition applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_state: Option<cfn_resources::StrVal>,

    /// The name of the crawler to which this condition applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the job whose JobRuns this condition applies to, and on which       this trigger waits.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<cfn_resources::StrVal>,

    ///
    /// A logical operator.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: EQUALS
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<ConditionLogicalOperatorEnum>,

    ///
    /// The condition state. Currently, the values supported are SUCCEEDED,         STOPPED, TIMEOUT, and FAILED.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ERROR | FAILED | RUNNING | STARTING | STOPPED | STOPPING | SUCCEEDED | TIMEOUT | WAITING
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ConditionStateEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ConditionLogicalOperatorEnum {
    /// EQUALS
    #[serde(rename = "EQUALS")]
    Equals,
}

impl Default for ConditionLogicalOperatorEnum {
    fn default() -> Self {
        ConditionLogicalOperatorEnum::Equals
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ConditionStateEnum {
    /// ERROR
    #[serde(rename = "ERROR")]
    Error,

    /// FAILED
    #[serde(rename = "FAILED")]
    Failed,

    /// RUNNING
    #[serde(rename = "RUNNING")]
    Running,

    /// STARTING
    #[serde(rename = "STARTING")]
    Starting,

    /// STOPPED
    #[serde(rename = "STOPPED")]
    Stopped,

    /// STOPPING
    #[serde(rename = "STOPPING")]
    Stopping,

    /// SUCCEEDED
    #[serde(rename = "SUCCEEDED")]
    Succeeded,

    /// TIMEOUT
    #[serde(rename = "TIMEOUT")]
    Timeout,

    /// WAITING
    #[serde(rename = "WAITING")]
    Waiting,
}

impl Default for ConditionStateEnum {
    fn default() -> Self {
        ConditionStateEnum::Error
    }
}

impl cfn_resources::CfnResource for Condition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'job_name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'job_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventBatchingCondition {
    ///
    /// Number of events that must be received from Amazon EventBridge before EventBridge event trigger fires.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchSize")]
    pub batch_size: i64,

    ///
    /// Window of time in seconds after which EventBridge event trigger fires. Window starts when first event is received.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_window: Option<i64>,
}

impl cfn_resources::CfnResource for EventBatchingCondition {
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

/// Specifies configuration properties of a job run notification.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationProperty {
    /// After a job run starts, the number of minutes to wait before sending a job run delay notification
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyDelayAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_delay_after: Option<i64>,
}

impl cfn_resources::CfnResource for NotificationProperty {
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

/// Defines the predicate of the trigger, which determines when it fires.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Predicate {
    ///
    /// A list of the conditions that determine when the trigger will fire.
    ///
    /// Required: No
    ///
    /// Type: List of Condition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    ///
    /// An optional field if only one condition is listed. If multiple conditions are listed,       then this field is required.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AND | ANY
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logical")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical: Option<PredicateLogicalEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PredicateLogicalEnum {
    /// AND
    #[serde(rename = "AND")]
    And,

    /// ANY
    #[serde(rename = "ANY")]
    Any,
}

impl Default for PredicateLogicalEnum {
    fn default() -> Self {
        PredicateLogicalEnum::And
    }
}

impl cfn_resources::CfnResource for Predicate {
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
