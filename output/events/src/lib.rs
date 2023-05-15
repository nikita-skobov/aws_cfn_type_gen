
pub mod cfn_api_destination {

#[derive(serde::Serialize, Default)]
pub struct CfnApiDestination {
    /// No documentation provided by AWS
    #[serde(rename = "HttpMethod")]
    pub http_method: String,
    /// No documentation provided by AWS
    #[serde(rename = "InvocationRateLimitPerSecond")]
    pub invocation_rate_limit_per_second: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Name of the apiDestination.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// The arn of the connection.
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
    /// Url endpoint to invoke.
    #[serde(rename = "InvocationEndpoint")]
    pub invocation_endpoint: String,

}



}

pub mod cfn_archive {

#[derive(serde::Serialize, Default)]
pub struct CfnArchive {
    /// No documentation provided by AWS
    #[serde(rename = "SourceArn")]
    pub source_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "RetentionDays")]
    pub retention_days: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ArchiveName")]
    pub archive_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



}

pub mod cfn_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnConnection {
    /// Name of the connection.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthParameters")]
    pub auth_parameters: AuthParameters,
    /// Description of the connection.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizationType")]
    pub authorization_type: String,

}


#[derive(serde::Serialize, Default)]
pub struct AuthParameters {
    #[serde(rename = "OAuthParameters")]
    pub oauth_parameters: Option<OAuthParameters>,
    #[serde(rename = "BasicAuthParameters")]
    pub basic_auth_parameters: Option<BasicAuthParameters>,
    #[serde(rename = "ApiKeyAuthParameters")]
    pub api_key_auth_parameters: Option<ApiKeyAuthParameters>,
    #[serde(rename = "InvocationHttpParameters")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct ApiKeyAuthParameters {
    #[serde(rename = "ApiKeyValue")]
    pub api_key_value: String,
    #[serde(rename = "ApiKeyName")]
    pub api_key_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct BasicAuthParameters {
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,

}

#[derive(serde::Serialize, Default)]
pub struct OAuthParameters {
    #[serde(rename = "ClientParameters")]
    pub client_parameters: ClientParameters,
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "HttpMethod")]
    pub http_method: String,
    #[serde(rename = "OAuthHttpParameters")]
    pub oauth_http_parameters: Option<ConnectionHttpParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct ClientParameters {
    #[serde(rename = "ClientID")]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    pub client_secret: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionHttpParameters {
    #[serde(rename = "BodyParameters")]
    pub body_parameters: Option<Vec<Parameter>>,
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<Vec<Parameter>>,
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<Vec<Parameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct Parameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "IsValueSecret")]
    pub is_value_secret: Option<bool>,

}


}

pub mod cfn_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnEndpoint {
    /// No documentation provided by AWS
    #[serde(rename = "ReplicationConfig")]
    pub replication_config: Option<ReplicationConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoutingConfig")]
    pub routing_config: RoutingConfig,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EventBuses")]
    pub event_buses: EventBuses,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}

pub type HealthCheck = String;
#[derive(serde::Serialize, Default)]
pub struct FailoverConfig {
    #[serde(rename = "Secondary")]
    pub secondary: Secondary,
    #[serde(rename = "Primary")]
    pub primary: Primary,

}

#[derive(serde::Serialize, Default)]
pub struct Secondary {
    #[serde(rename = "Route")]
    pub route: Route,

}
pub type Route = String;
#[derive(serde::Serialize, Default)]
pub struct Primary {
    #[serde(rename = "HealthCheck")]
    pub health_check: HealthCheck,

}

#[derive(serde::Serialize, Default)]
pub struct EventBuses {

}

#[derive(serde::Serialize, Default)]
pub struct ReplicationConfig {
    #[serde(rename = "State")]
    pub state: ReplicationState,

}

#[derive(serde::Serialize, Default)]
pub struct EndpointEventBus {
    #[serde(rename = "EventBusArn")]
    pub event_bus_arn: EventBusArn,

}

#[derive(serde::Serialize, Default)]
pub struct RoutingConfig {
    #[serde(rename = "FailoverConfig")]
    pub failover_config: FailoverConfig,

}
pub type EventBusArn = String;pub type ReplicationState = String;

}

pub mod cfn_event_bus {

#[derive(serde::Serialize, Default)]
pub struct CfnEventBus {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "EventSourceName")]
    pub event_source_name: Option<String>,
    /// List of TagEntry
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagEntry>>,

}


#[derive(serde::Serialize, Default)]
pub struct TagEntry {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_event_bus_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnEventBusPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Condition")]
    pub condition: Option<Condition>,
    /// No documentation provided by AWS
    #[serde(rename = "StatementId")]
    pub statement_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Action")]
    pub action: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Statement")]
    pub statement: Option<()>,

}


#[derive(serde::Serialize, Default)]
pub struct Condition {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}


}

pub mod cfn_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnRule {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Target
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,
    /// No documentation provided by AWS
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct InputTransformer {
    #[serde(rename = "InputTemplate")]
    pub input_template: String,
    #[serde(rename = "InputPathsMap")]
    pub input_paths_map: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchRetryStrategy {
    #[serde(rename = "Attempts")]
    pub attempts: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct EcsParameters {
    #[serde(rename = "TagList")]
    pub tag_list: Option<Vec<Tag>>,
    #[serde(rename = "CapacityProviderStrategy")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "EnableECSManagedTags")]
    pub enable_ecsmanaged_tags: Option<bool>,
    #[serde(rename = "Group")]
    pub group: Option<String>,
    #[serde(rename = "LaunchType")]
    pub launch_type: Option<String>,
    #[serde(rename = "TaskCount")]
    pub task_count: Option<usize>,
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "PlatformVersion")]
    pub platform_version: Option<String>,
    #[serde(rename = "ReferenceId")]
    pub reference_id: Option<String>,
    #[serde(rename = "TaskDefinitionArn")]
    pub task_definition_arn: String,
    #[serde(rename = "PropagateTags")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "EnableExecuteCommand")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "PlacementConstraints")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "PlacementStrategies")]
    pub placement_strategies: Option<Vec<PlacementStrategy>>,

}

#[derive(serde::Serialize, Default)]
pub struct Target {
    #[serde(rename = "SageMakerPipelineParameters")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,
    #[serde(rename = "EcsParameters")]
    pub ecs_parameters: Option<EcsParameters>,
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "InputTransformer")]
    pub input_transformer: Option<InputTransformer>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "RunCommandParameters")]
    pub run_command_parameters: Option<RunCommandParameters>,
    #[serde(rename = "HttpParameters")]
    pub http_parameters: Option<HttpParameters>,
    #[serde(rename = "DeadLetterConfig")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "InputPath")]
    pub input_path: Option<String>,
    #[serde(rename = "Input")]
    pub input: Option<String>,
    #[serde(rename = "SqsParameters")]
    pub sqs_parameters: Option<SqsParameters>,
    #[serde(rename = "BatchParameters")]
    pub batch_parameters: Option<BatchParameters>,
    #[serde(rename = "KinesisParameters")]
    pub kinesis_parameters: Option<KinesisParameters>,
    #[serde(rename = "RedshiftDataParameters")]
    pub redshift_data_parameters: Option<RedshiftDataParameters>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementConstraint {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SqsParameters {
    #[serde(rename = "MessageGroupId")]
    pub message_group_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,

}

#[derive(serde::Serialize, Default)]
pub struct KinesisParameters {
    #[serde(rename = "PartitionKeyPath")]
    pub partition_key_path: String,

}

#[derive(serde::Serialize, Default)]
pub struct RunCommandParameters {
    #[serde(rename = "RunCommandTargets")]
    pub run_command_targets: Vec<RunCommandTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "AssignPublicIp")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct RetryPolicy {
    #[serde(rename = "MaximumEventAgeInSeconds")]
    pub maximum_event_age_in_seconds: Option<usize>,
    #[serde(rename = "MaximumRetryAttempts")]
    pub maximum_retry_attempts: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct HttpParameters {
    #[serde(rename = "PathParameterValues")]
    pub path_parameter_values: Option<Vec<String>>,
    #[serde(rename = "HeaderParameters")]
    pub header_parameters: Option<()>,
    #[serde(rename = "QueryStringParameters")]
    pub query_string_parameters: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityProviderStrategyItem {
    #[serde(rename = "Weight")]
    pub weight: Option<usize>,
    #[serde(rename = "CapacityProvider")]
    pub capacity_provider: String,
    #[serde(rename = "Base")]
    pub base: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BatchParameters {
    #[serde(rename = "JobDefinition")]
    pub job_definition: String,
    #[serde(rename = "ArrayProperties")]
    pub array_properties: Option<BatchArrayProperties>,
    #[serde(rename = "JobName")]
    pub job_name: String,
    #[serde(rename = "RetryStrategy")]
    pub retry_strategy: Option<BatchRetryStrategy>,

}

#[derive(serde::Serialize, Default)]
pub struct RedshiftDataParameters {
    #[serde(rename = "SecretManagerArn")]
    pub secret_manager_arn: Option<String>,
    #[serde(rename = "StatementName")]
    pub statement_name: Option<String>,
    #[serde(rename = "DbUser")]
    pub db_user: Option<String>,
    #[serde(rename = "WithEvent")]
    pub with_event: Option<bool>,
    #[serde(rename = "Sql")]
    pub sql: String,
    #[serde(rename = "Database")]
    pub database: String,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkConfiguration {
    #[serde(rename = "AwsVpcConfiguration")]
    pub aws_vpc_configuration: Option<AwsVpcConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct RunCommandTarget {
    #[serde(rename = "Values")]
    pub values: Vec<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PlacementStrategy {
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Field")]
    pub field: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct BatchArrayProperties {
    #[serde(rename = "Size")]
    pub size: Option<usize>,

}


}
