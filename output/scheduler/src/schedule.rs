/// A schedule is the main resource you create, configure, and manage using Amazon EventBridge Scheduler.
///
/// Every schedule has a schedule expression that determines when, and with what frequency, the schedule runs. EventBridge Scheduler     supports three types of schedules: rate, cron, and one-time schedules. For more information about different schedule types,     see Schedule types in the EventBridge Scheduler User Guide.
///
/// When you create a schedule, you configure a target for the schedule to invoke. A target is an API operation that EventBridge Scheduler calls on your behalf     every time your schedule runs. EventBridge Scheduler supports two types of targets: templated targets invoke common API operations across     a core groups of services, and customizeable universal targets that you can use to call more than 6,000 operations     across over 270 services. For more information about configuring targets, see     Managing targets in the EventBridge Scheduler User Guide.
///
/// For more information about managing schedules, changing the schedule state, setting up flexible time windows, and configuring a dead-letter queue for a schedule, see     Managing a schedule in the EventBridge Scheduler User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSchedule {
    ///
    /// The description you specify for the schedule.
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
    /// The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the EndDate you specify. EventBridge Scheduler ignores EndDate for one-time schedules.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<cfn_resources::StrVal>,

    ///
    /// Allows you to configure a time window during which EventBridge Scheduler invokes the schedule.
    ///
    /// Required: Yes
    ///
    /// Type: FlexibleTimeWindow
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlexibleTimeWindow")]
    pub flexible_time_window: FlexibleTimeWindow,

    ///
    /// The name of the schedule group associated with this schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) for the customer managed KMS key that EventBridge Scheduler will use to encrypt and decrypt your data.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the schedule.
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
    /// The expression that defines when the schedule runs. The following formats are supported.
    ///
    /// at expression - at(yyyy-mm-ddThh:mm:ss)          rate expression - rate(value unit)          cron expression - cron(fields)
    ///
    /// You can use at expressions to create one-time schedules that invoke a target once, at the time and in the time zone, that you specify.  You can use rate and cron expressions to create recurring schedules. Rate-based schedules are useful when you want to invoke a target  at regular intervals, such as every 15 minutes or every five days. Cron-based schedules are useful when you want to invoke a target periodically at a specific time,  such as at 8:00 am (UTC+0) every 1st day of the month.
    ///
    /// A cron expression consists of six fields separated by white spaces: (minutes hours day_of_month month day_of_week year).
    ///
    /// A rate expression consists of a value as a positive integer, and a unit with the following options:  minute | minutes | hour | hours | day | days
    ///
    /// For more information and examples, see Schedule types on EventBridge Scheduler in the EventBridge Scheduler User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: cfn_resources::StrVal,

    ///
    /// The timezone in which the scheduling expression is evaluated.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpressionTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_timezone: Option<cfn_resources::StrVal>,

    ///
    /// The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the StartDate you specify. EventBridge Scheduler ignores StartDate for one-time schedules.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether the schedule is enabled or disabled.
    ///
    /// Allowed Values: ENABLED | DISABLED
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ScheduleStateEnum>,

    ///
    /// The schedule's target details.
    ///
    /// Required: Yes
    ///
    /// Type: Target
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: Target,

    #[serde(skip_serializing)]
    pub att_arn: CfnSchedulearn,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ScheduleStateEnum {
    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,
}

impl Default for ScheduleStateEnum {
    fn default() -> Self {
        ScheduleStateEnum::Enabled
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSchedulearn;
impl CfnSchedulearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnSchedule {
    fn type_string(&self) -> &'static str {
        "AWS::Scheduler::Schedule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.flexible_time_window.validate()?;

        self.target.validate()?;

        Ok(())
    }
}

/// This structure specifies the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the awsvpc network mode.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AwsVpcConfiguration {
    ///
    /// Specifies whether the task's elastic network interface receives a public IP address. You can specify ENABLED only when LaunchType in EcsParameters is set to FARGATE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the security groups associated with the task. These security groups must all be in the same VPC. You can specify as many as five security groups.     If you do not specify a security group, the default security group for the VPC is used.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// Specifies the subnets associated with the task. These subnets must all be in the same VPC. You can specify as many as 16 subnets.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

impl cfn_resources::CfnResource for AwsVpcConfiguration {
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

/// The details of a capacity provider strategy.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CapacityProviderStrategyItem {
    ///
    /// The base value designates how many tasks, at a minimum, to run on the specified capacity provider. Only one capacity provider in a capacity provider strategy can have a base defined.     If no value is specified, the default value of 0 is used.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,

    ///
    /// The short name of the capacity provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: cfn_resources::StrVal,

    ///
    /// The weight value designates the relative percentage of the total number of tasks launched that should use the specified capacity provider. The weight value is taken into consideration after the     base value, if defined, is satisfied.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl cfn_resources::CfnResource for CapacityProviderStrategyItem {
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

/// An object that contains information about an Amazon SQS queue that EventBridge Scheduler uses as a dead-letter queue for your schedule. If specified, EventBridge Scheduler delivers failed events that could not be successfully delivered to a target to the queue.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DeadLetterConfig {
    ///
    /// The Amazon Resource Name (ARN) of the SQS queue specified as the destination for the dead-letter queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DeadLetterConfig {
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

/// The templated target type for the Amazon ECS RunTask API operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EcsParameters {
    ///
    /// The capacity provider strategy to use for the task.
    ///
    /// Required: No
    ///
    /// Type: List of CapacityProviderStrategyItem
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,

    ///
    /// Specifies whether to enable Amazon ECS managed tags for the task. For more information, see Tagging Your Amazon ECS Resources     in the Amazon ECS Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecsmanaged_tags: Option<bool>,

    ///
    /// Whether or not to enable the execute command functionality for the containers in this task. If true, this enables execute command functionality on all containers in the task.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,

    ///
    /// Specifies an Amazon ECS task group for the task. The maximum length is 255 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<cfn_resources::StrVal>,

    ///
    /// Specifies the launch type on which your task is running. The launch type that you specify here must match one of the launch type (compatibilities) of the target task.     The FARGATE value is supported only in the Regions where Fargate with Amazon ECS is supported.     For more information, see AWS Fargate on Amazon ECS in the Amazon ECS Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<cfn_resources::StrVal>,

    ///
    /// This structure specifies the network configuration for an ECS task.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,

    ///
    /// An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime).
    ///
    /// Required: No
    ///
    /// Type: List of PlacementConstraint
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,

    ///
    /// The task placement strategy for a task or service.
    ///
    /// Required: No
    ///
    /// Type: List of PlacementStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,

    ///
    /// Specifies the platform version for the task. Specify only the numeric portion of the platform version, such as 1.1.0.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<cfn_resources::StrVal>,

    ///
    /// Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated.     Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the Amazon ECS TagResource     API action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<cfn_resources::StrVal>,

    ///
    /// The reference ID to use for the task.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<cfn_resources::StrVal>,

    ///
    /// The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.     For more information, see RunTask in the Amazon ECS API Reference.
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
    /// The number of tasks to create based on TaskDefinition. The default is 1.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<f64>,

    ///
    /// The Amazon Resource Name (ARN) of the task definition to use if the event target is an Amazon ECS task.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EcsParameters {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.network_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The templated target type for the EventBridge PutEvents API operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EventBridgeParameters {
    ///
    /// A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DetailType")]
    pub detail_type: cfn_resources::StrVal,

    ///
    /// The source of the event.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Source")]
    pub source: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EventBridgeParameters {
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

/// Allows you to configure a time window during which EventBridge Scheduler invokes the schedule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FlexibleTimeWindow {
    ///
    /// The maximum time window during which a schedule can be invoked.
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1440
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumWindowInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_window_in_minutes: Option<f64>,

    ///
    /// Determines whether the schedule is invoked within a flexible time window.
    ///
    /// Allowed Values: OFF | FLEXIBLE
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: FlexibleTimeWindowModeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FlexibleTimeWindowModeEnum {
    /// OFF
    #[serde(rename = "OFF")]
    Off,

    /// FLEXIBLE
    #[serde(rename = "FLEXIBLE")]
    Flexible,
}

impl Default for FlexibleTimeWindowModeEnum {
    fn default() -> Self {
        FlexibleTimeWindowModeEnum::Off
    }
}

impl cfn_resources::CfnResource for FlexibleTimeWindow {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.maximum_window_in_minutes {
            if *the_val > 1440 as _ {
                return Err(format!("Max validation failed on field 'maximum_window_in_minutes'. {} is greater than 1440", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_window_in_minutes {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'maximum_window_in_minutes'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The templated target type for the Amazon Kinesis PutRecord API operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct KinesisParameters {
    ///
    /// Specifies the shard to which EventBridge Scheduler sends the event. For more information, see Amazon Kinesis Data Streams terminology and concepts in the     Amazon Kinesis Streams Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartitionKey")]
    pub partition_key: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KinesisParameters {
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

/// Specifies the network configuration for an ECS task.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkConfiguration {
    ///
    /// Specifies the Amazon VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the awsvpc network mode.
    ///
    /// Required: No
    ///
    /// Type: AwsVpcConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AwsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

impl cfn_resources::CfnResource for NetworkConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.awsvpc_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object representing a constraint on task placement.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PlacementConstraint {
    ///
    /// A cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is distinctInstance.     For more information, see Cluster query language in the Amazon ECS Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<cfn_resources::StrVal>,

    ///
    /// The type of constraint. Use distinctInstance to ensure that each task in a particular group is running on a different container instance. Use memberOf to restrict the selection to a group of valid candidates.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for PlacementConstraint {
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

/// The task placement strategy for a task or service.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PlacementStrategy {
    ///
    /// The field to apply the placement strategy against. For the spread placement strategy, valid values are instanceId (or instanceId, which has the same effect),     or any platform or custom attribute that is applied to a container instance, such as attribute:ecs.availability-zone. For the binpack placement strategy, valid values are     cpu and memory. For the random placement strategy, this field is not used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<cfn_resources::StrVal>,

    ///
    /// The type of placement strategy. The random placement strategy randomly places tasks on available candidates. The spread placement strategy spreads placement across available candidates     evenly based on the field parameter. The binpack strategy places tasks on available candidates that have the least available amount of the resource that is specified with the field parameter.     For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for PlacementStrategy {
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

/// A RetryPolicy object that includes information about the retry policy settings, including the maximum age of an event, and the maximum number of times EventBridge Scheduler will try to deliver the event to a target.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RetryPolicy {
    ///
    /// The maximum amount of time, in seconds, to continue to make retry attempts.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<f64>,

    ///
    /// The maximum number of retry attempts to make before the request fails. Retry attempts with exponential backoff continue until either the maximum number of attempts is made or     until the duration of the MaximumEventAgeInSeconds is reached.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<f64>,
}

impl cfn_resources::CfnResource for RetryPolicy {
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

/// The name and value pair of a parameter to use to start execution of a SageMaker Model Building Pipeline.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SageMakerPipelineParameter {
    ///
    /// Name of parameter to start execution of a SageMaker Model Building Pipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Value of parameter to start execution of a SageMaker Model Building Pipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SageMakerPipelineParameter {
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

/// The templated target type for the Amazon SageMaker StartPipelineExecution API operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SageMakerPipelineParameters {
    ///
    /// List of parameter names and values to use when executing the SageMaker Model Building Pipeline.
    ///
    /// Required: No
    ///
    /// Type: List of SageMakerPipelineParameter
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineParameterList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,
}

impl cfn_resources::CfnResource for SageMakerPipelineParameters {
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

/// The templated target type for the Amazon SQS SendMessage API operation.     Contains the message group ID to use when the target is a FIFO queue. If you specify an Amazon SQS FIFO queue as a target, the queue must have content-based deduplication enabled.     For more information, see Using the Amazon SQS message deduplication ID in the     Amazon SQS Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SqsParameters {
    ///
    /// The FIFO message group ID to use as the target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MessageGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for SqsParameters {
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

/// The schedule's target. EventBridge Scheduler supports templated target that invoke common API operations, as well as universal targets that you can customize to     invoke over 6,000 API operations across more than 270 services. You can only specify one templated or universal target for a schedule.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Target {
    ///
    /// The Amazon Resource Name (ARN) of the target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: cfn_resources::StrVal,

    ///
    /// An object that contains information about an Amazon SQS queue that EventBridge Scheduler uses as a dead-letter queue for your schedule. If specified, EventBridge Scheduler delivers failed events that could not be successfully delivered to a target to the queue.
    ///
    /// Required: No
    ///
    /// Type: DeadLetterConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,

    ///
    /// The templated target type for the Amazon ECS RunTask API operation.
    ///
    /// Required: No
    ///
    /// Type: EcsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "EcsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,

    ///
    /// The templated target type for the EventBridge PutEvents API operation.
    ///
    /// Required: No
    ///
    /// Type: EventBridgeParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridgeParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_parameters: Option<EventBridgeParameters>,

    ///
    /// The text, or well-formed JSON, passed to the target. If you are configuring a templated Lambda, AWS Step Functions, or Amazon EventBridge target,     the input must be a well-formed JSON. For all other target types, a JSON is not required. If you do not specify anything for this field, Amazon EventBridge Scheduler     delivers a default notification to the target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<cfn_resources::StrVal>,

    ///
    /// The templated target type for the Amazon Kinesis PutRecord API operation.
    ///
    /// Required: No
    ///
    /// Type: KinesisParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "KinesisParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,

    ///
    /// A RetryPolicy object that includes information about the retry policy settings, including the maximum age of an event, and the maximum number of times EventBridge Scheduler will try to deliver the event to a target.
    ///
    /// Required: No
    ///
    /// Type: RetryPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,

    ///
    /// The Amazon Resource Name (ARN) of the IAM role that EventBridge Scheduler will use for this target when the schedule is invoked.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The templated target type for the Amazon SageMaker StartPipelineExecution API operation.
    ///
    /// Required: No
    ///
    /// Type: SageMakerPipelineParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerPipelineParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,

    ///
    /// The templated target type for the Amazon SQS SendMessage API operation.     Contains the message group ID to use when the target is a FIFO queue. If you specify an Amazon SQS FIFO queue as a target, the queue must have content-based deduplication enabled.     For more information, see Using the Amazon SQS message deduplication ID in the     Amazon SQS Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: SqsParameters
    ///
    /// Update requires: No interruption
    #[serde(rename = "SqsParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

impl cfn_resources::CfnResource for Target {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dead_letter_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.ecs_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.event_bridge_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kinesis_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.retry_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sage_maker_pipeline_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sqs_parameters
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
