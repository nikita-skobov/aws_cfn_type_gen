

/// The AWS::Glue::Trigger resource specifies triggers that run AWS Glue       jobs. For more information, see Triggering Jobs in AWS Glue and Trigger Structure in the AWS Glue Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnTrigger {


    /// Set to true to start SCHEDULED and CONDITIONAL triggers when created. True is not supported for ON_DEMAND triggers.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartOnCreation")]
    pub start_on_creation: Option<bool>,


    /// 
    /// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires.
    ///
    /// Required: No
    ///
    /// Type: EventBatchingCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBatchingCondition")]
    pub event_batching_condition: Option<EventBatchingCondition>,


    /// 
    /// A cron expression used to specify the schedule. For more information, see         Time-Based Schedules for         Jobs and Crawlers in the AWS Glue Developer Guide. For       example, to run something every day at 12:15 UTC, specify cron(15 12 * * ?         *).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    pub schedule: Option<String>,


    /// 
    /// The tags to use with this trigger.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<serde_json::Value>,


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
    pub cfn_type: String,


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
    pub description: Option<String>,


    /// 
    /// The predicate of this trigger, which defines when it will fire.
    /// 
    /// Required: No
    ///
    /// Type: Predicate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Predicate")]
    pub predicate: Option<Predicate>,


    /// The name of the workflow associated with the trigger.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkflowName")]
    pub workflow_name: Option<String>,


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
    pub name: Option<String>,

}


/// Defines an action to be initiated by a trigger.
#[derive(Default, serde::Serialize)]
pub struct Action {


    /// The name of the crawler to be used with this action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlerName")]
    pub crawler_name: Option<String>,


    /// Specifies configuration properties of a job run notification.
    ///
    /// Required: No
    ///
    /// Type: NotificationProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationProperty")]
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
    pub security_configuration: Option<String>,


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
    pub arguments: Option<serde_json::Value>,


    /// The JobRun timeout in minutes. This is the maximum time that a job run can consume resources before it is terminated and enters TIMEOUT status. The default is 2,880 minutes (48 hours). This overrides the timeout value set in the parent job.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,


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
    pub job_name: Option<String>,

}


/// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires.
#[derive(Default, serde::Serialize)]
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
    pub batch_window: Option<i64>,

}


/// Specifies configuration properties of a job run notification.
#[derive(Default, serde::Serialize)]
pub struct NotificationProperty {


    /// After a job run starts, the number of minutes to wait before sending a job run delay notification
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotifyDelayAfter")]
    pub notify_delay_after: Option<i64>,

}


/// Defines the predicate of the trigger, which determines when it fires.
#[derive(Default, serde::Serialize)]
pub struct Predicate {


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
    pub logical: Option<String>,


    /// 
    /// A list of the conditions that determine when the trigger will fire.
    /// 
    /// Required: No
    ///
    /// Type: List of Condition
    ///
    /// Update requires: No interruption
    #[serde(rename = "Conditions")]
    pub conditions: Option<Vec<Condition>>,

}


/// Defines a condition under which a trigger fires.
#[derive(Default, serde::Serialize)]
pub struct Condition {


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
    pub state: Option<String>,


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
    pub job_name: Option<String>,


    /// The name of the crawler to which this condition applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlerName")]
    pub crawler_name: Option<String>,


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
    pub logical_operator: Option<String>,


    /// The state of the crawler to which this condition applies.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrawlState")]
    pub crawl_state: Option<String>,

}
