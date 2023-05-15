
pub mod cfn_schedule {

#[derive(serde::Serialize, Default)]
pub struct CfnSchedule {
    /// The timezone in which the scheduling expression is evaluated.
    #[serde(rename = "ScheduleExpressionTimezone")]
    pub schedule_expression_timezone: Option<String>,
    /// The ARN for a KMS Key that will be used to encrypt customer data.
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,
    /// The schedule target.
    #[serde(rename = "Target")]
    pub target: Target,
    /// The name of the schedule group to associate with this schedule. If you omit this, the default schedule group is used.
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    /// Flexible time window allows configuration of a window within which a schedule can be invoked
    #[serde(rename = "FlexibleTimeWindow")]
    pub flexible_time_window: FlexibleTimeWindow,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The description of the schedule.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The scheduling expression.
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
    /// The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the EndDate you specify.
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    /// The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the StartDate you specify.
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "State")]
    pub state: Option<ScheduleState>,

}


#[derive(serde::Serialize, Default)]
pub struct PlacementConstraint {
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<PlacementConstraintType>,

}

#[derive(serde::Serialize, Default)]
pub struct EcsParameters {
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,
    #[serde(rename = "EnableECSManagedTags")]
    pub enable_ecsmanaged_tags: Option<bool>,
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<LaunchType>,
    #[serde(rename = "Group")]
    pub group: Option<String>,
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "TaskCount")]
    pub task_count: Option<f64>,
    #[serde(rename = "ReferenceId")]
    pub reference_id: Option<String>,
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "CapacityProviderStrategy")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<PropagateTags>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagMap>>,
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: String,
    #[serde(rename = "EnableExecuteCommand")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "PlacementStrategy")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,

}
pub type FlexibleTimeWindowMode = String;
#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AwsvpcConfiguration")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct EventBridgeParameters {
    #[serde(rename = "DetailType")]
    pub detail_type: String,
    #[serde(rename = "Source")]
    pub source: String,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementStrategy {
    #[serde(rename = "Type")]
    pub ty: Option<PlacementStrategyType>,
    #[serde(rename = "Field")]
    pub field: Option<String>,

}
pub type PlacementStrategyType = String;pub type PlacementConstraintType = String;
#[derive(serde::Serialize, Default)]
pub struct SqsParameters {
    #[serde(rename = "MessageGroupId")]
    pub message_group_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "SageMakerPipelineParameters")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,
    #[serde(rename = "EventBridgeParameters")]
    pub event_bridge_parameters: Option<EventBridgeParameters>,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "EcsParameters")]
    pub ecs_parameters: Option<EcsParameters>,
    #[serde(rename = "SqsParameters")]
    pub sqs_parameters: Option<SqsParameters>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "KinesisParameters")]
    pub kinesis_parameters: Option<KinesisParameters>,
    #[serde(rename = "Input")]
    pub input: Option<String>,
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<RetryPolicy>,

}
pub type AssignPublicIp = String;
#[derive(serde::Serialize, Default)]
pub struct TagMap {

}

#[derive(serde::Serialize, Default)]
pub struct KinesisParameters {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct FlexibleTimeWindow {
    #[serde(rename = "Mode")]
    pub mode: FlexibleTimeWindowMode,
    #[serde(rename = "MaximumWindowInMinutes")]
    pub maximum_window_in_minutes: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}
pub type PropagateTags = String;
#[derive(serde::Serialize, Default)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<AssignPublicIp>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type ScheduleState = String;pub type LaunchType = String;
#[derive(serde::Serialize, Default)]
pub struct RetryPolicy {
    #[serde(rename = "MaximumEventAgeInSeconds")]
    pub maximum_event_age_in_seconds: Option<f64>,
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategyItem {
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: String,
    #[serde(rename = "Weight")]
    pub weight: Option<f64>,
    #[serde(rename = "Base")]
    pub base: Option<f64>,

}


}

pub mod cfn_schedule_group {

#[derive(serde::Serialize, Default)]
pub struct CfnScheduleGroup {
    /// The list of tags to associate with the schedule group.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

pub type ScheduleGroupState = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
