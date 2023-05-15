
pub mod cfn_alias {

#[derive(serde::Serialize, Default)]
pub struct CfnAlias {
    /// A descriptive label that is associated with an alias. Alias names do not need to be unique.
    #[serde(rename = "Name")]
    pub name: String,
    /// A routing configuration that specifies where traffic is directed for this alias, such as to a fleet or to a message.
    #[serde(rename = "RoutingStrategy")]
    pub routing_strategy: RoutingStrategy,
    /// A human-readable description of the alias.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct RoutingStrategy {
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "FleetId")]
    pub fleet_id: Option<String>,

}


}

pub mod cfn_build {

#[derive(serde::Serialize, Default)]
pub struct CfnBuild {
    /// A server SDK version you used when integrating your game server build with Amazon GameLift. By default Amazon GameLift sets this value to 4.0.2.
    #[serde(rename = "ServerSdkVersion")]
    pub server_sdk_version: Option<String>,
    /// Version information that is associated with this build. Version strings do not need to be unique.
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// The operating system that the game server binaries are built to run on. This value determines the type of fleet resources that you can use for this build. If your game build contains multiple executables, they all must run on the same operating system. If an operating system is not specified when creating a build, Amazon GameLift uses the default value (WINDOWS_2012). This value cannot be changed later.
    #[serde(rename = "OperatingSystem")]
    pub operating_system: Option<String>,
    /// A descriptive label that is associated with a build. Build names do not need to be unique.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Information indicating where your game build files are stored. Use this parameter only when creating a build with files stored in an Amazon S3 bucket that you own. The storage location must specify an Amazon S3 bucket name and key. The location must also specify a role ARN that you set up to allow Amazon GameLift to access your Amazon S3 bucket. The S3 bucket and your new build must be in the same Region.
    #[serde(rename = "StorageLocation")]
    pub storage_location: Option<StorageLocation>,

}


#[derive(serde::Serialize, Default)]
pub struct StorageLocation {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


}

pub mod cfn_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnFleet {
    /// A game session protection policy to apply to all game sessions hosted on instances in this fleet. When protected, active game sessions cannot be terminated during a scale-down event. If this parameter is not set, instances in this fleet default to no protection. You can change a fleet's protection policy to affect future game sessions on the fleet. You can also set protection for individual game sessions.
    #[serde(rename = "NewGameSessionProtectionPolicy")]
    pub new_game_session_protection_policy: Option<String>,
    /// A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the VPC Dashboard in the AWS Management Console.
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: Option<String>,
    /// Indicates whether to generate a TLS/SSL certificate for the new fleet. TLS certificates are used for encrypting traffic between game clients and game servers running on GameLift. If this parameter is not set, certificate generation is disabled. This fleet setting cannot be changed once the fleet is created.
    #[serde(rename = "CertificateConfiguration")]
    pub certificate_configuration: Option<CertificateConfiguration>,
    /// Instructions for launching server processes on each instance in the fleet. Server processes run either a custom game build executable or a Realtime script. The runtime configuration defines the server executables or launch script file, launch parameters, and the number of processes to run concurrently on each instance. When creating a fleet, the runtime configuration must have at least one server process configuration; otherwise the request fails with an invalid request exception.
    /// 
    /// This parameter is required unless the parameters ServerLaunchPath and ServerLaunchParameters are defined. Runtime configuration has replaced these parameters, but fleets that use them will continue to work.
    #[serde(rename = "RuntimeConfiguration")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
    /// The name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See Amazon EC2 Instance Types for detailed descriptions.
    #[serde(rename = "EC2InstanceType")]
    pub ec2_instance_type: Option<String>,
    /// A unique identifier for a build to be deployed on the new fleet. If you are deploying the fleet with a custom game build, you must specify this property. The build must have been successfully uploaded to Amazon GameLift and be in a READY status. This fleet setting cannot be changed once the fleet is created.
    #[serde(rename = "BuildId")]
    pub build_id: Option<String>,
    /// A unique identifier for a Realtime script to be deployed on a new Realtime Servers fleet. The script must have been successfully uploaded to Amazon GameLift. This fleet setting cannot be changed once the fleet is created.
    /// 
    /// Note: It is not currently possible to use the !Ref command to reference a script created with a CloudFormation template for the fleet property ScriptId. Instead, use Fn::GetAtt Script.Arn or Fn::GetAtt Script.Id to retrieve either of these properties as input for ScriptId. Alternatively, enter a ScriptId string manually.
    #[serde(rename = "ScriptId")]
    pub script_id: Option<String>,
    /// ComputeType to differentiate EC2 hardware managed by GameLift and Anywhere hardware managed by the customer.
    #[serde(rename = "ComputeType")]
    pub compute_type: Option<String>,
    /// A policy that limits the number of game sessions an individual player can create over a span of time for this fleet.
    #[serde(rename = "ResourceCreationLimitPolicy")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
    /// A unique identifier for the AWS account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your account ID in the AWS Management Console under account settings.
    #[serde(rename = "PeerVpcAwsAccountId")]
    pub peer_vpc_aws_account_id: Option<String>,
    /// [DEPRECATED] The number of EC2 instances that you want this fleet to host. When creating a new fleet, GameLift automatically sets this value to "1" and initiates a single instance. Once the fleet is active, update this value to trigger GameLift to add or remove instances from the fleet.
    #[serde(rename = "DesiredEC2Instances")]
    pub desired_ec2_instances: Option<usize>,
    /// Indicates whether to use On-Demand instances or Spot instances for this fleet. If empty, the default is ON_DEMAND. Both categories of instances use identical hardware and configurations based on the instance type selected for this fleet.
    #[serde(rename = "FleetType")]
    pub fleet_type: Option<String>,
    /// The name of an Amazon CloudWatch metric group. A metric group aggregates the metrics for all fleets in the group. Specify a string containing the metric group name. You can use an existing name or use a new name to create a new metric group. Currently, this parameter can have only one string.
    #[serde(rename = "MetricGroups")]
    pub metric_groups: Option<Vec<String>>,
    /// [DEPRECATED] The maximum value that is allowed for the fleet's instance count. When creating a new fleet, GameLift automatically sets this value to "1". Once the fleet is active, you can change this value.
    #[serde(rename = "MaxSize")]
    pub max_size: Option<usize>,
    /// [DEPRECATED] The minimum value allowed for the fleet's instance count. When creating a new fleet, GameLift automatically sets this value to "0". After the fleet is active, you can change this value.
    #[serde(rename = "MinSize")]
    pub min_size: Option<usize>,
    /// A descriptive label that is associated with a fleet. Fleet names do not need to be unique.
    #[serde(rename = "Name")]
    pub name: String,
    /// This parameter is no longer used but is retained for backward compatibility. Instead, specify server launch parameters in the RuntimeConfiguration parameter. A request must specify either a runtime configuration or values for both ServerLaunchParameters and ServerLaunchPath.
    #[serde(rename = "ServerLaunchParameters")]
    pub server_launch_parameters: Option<String>,
    /// This parameter is no longer used. Instead, specify a server launch path using the RuntimeConfiguration parameter. Requests that specify a server launch path and launch parameters instead of a runtime configuration will continue to work.
    #[serde(rename = "ServerLaunchPath")]
    pub server_launch_path: Option<String>,
    /// A human-readable description of a fleet.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// This parameter is no longer used. When hosting a custom game build, specify where Amazon GameLift should store log files using the Amazon GameLift server API call ProcessReady()
    #[serde(rename = "LogPaths")]
    pub log_paths: Option<Vec<String>>,
    /// A range of IP addresses and port settings that allow inbound traffic to connect to server processes on an Amazon GameLift server.
    #[serde(rename = "EC2InboundPermissions")]
    pub ec2_inbound_permissions: Option<Vec<IpPermission>>,
    /// List of LocationConfiguration
    #[serde(rename = "Locations")]
    pub locations: Option<Vec<LocationConfiguration>>,
    /// A unique identifier for an AWS IAM role that manages access to your AWS services. With an instance role ARN set, any application that runs on an instance in this fleet can assume the role, including install scripts, server processes, and daemons (background processes). Create a role or look up a role's ARN from the IAM dashboard in the AWS Management Console.
    #[serde(rename = "InstanceRoleARN")]
    pub instance_role_arn: Option<String>,
    /// Configuration for Anywhere fleet.
    #[serde(rename = "AnywhereConfiguration")]
    pub anywhere_configuration: Option<AnywhereConfiguration>,

}


#[derive(serde::Serialize, Default)]
pub struct IpPermission {
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "IpRange")]
    pub ip_range: String,
    #[serde(rename = "FromPort")]
    pub from_port: usize,
    #[serde(rename = "ToPort")]
    pub to_port: usize,

}

#[derive(serde::Serialize, Default)]
pub struct CertificateConfiguration {
    #[serde(rename = "CertificateType")]
    pub certificate_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct LocationConfiguration {
    #[serde(rename = "Location")]
    pub location: Location,
    #[serde(rename = "LocationCapacity")]
    pub location_capacity: Option<LocationCapacity>,

}

#[derive(serde::Serialize, Default)]
pub struct ResourceCreationLimitPolicy {
    #[serde(rename = "NewGameSessionsPerCreator")]
    pub new_game_sessions_per_creator: Option<usize>,
    #[serde(rename = "PolicyPeriodInMinutes")]
    pub policy_period_in_minutes: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct RuntimeConfiguration {
    #[serde(rename = "ServerProcesses")]
    pub server_processes: Option<Vec<ServerProcess>>,
    #[serde(rename = "GameSessionActivationTimeoutSeconds")]
    pub game_session_activation_timeout_seconds: Option<usize>,
    #[serde(rename = "MaxConcurrentGameSessionActivations")]
    pub max_concurrent_game_session_activations: Option<usize>,

}
pub type Location = String;
#[derive(serde::Serialize, Default)]
pub struct LocationCapacity {
    #[serde(rename = "MaxSize")]
    pub max_size: usize,
    #[serde(rename = "DesiredEC2Instances")]
    pub desired_ec2_instances: usize,
    #[serde(rename = "MinSize")]
    pub min_size: usize,

}

#[derive(serde::Serialize, Default)]
pub struct AnywhereConfiguration {
    #[serde(rename = "Cost")]
    pub cost: String,

}

#[derive(serde::Serialize, Default)]
pub struct ServerProcess {
    #[serde(rename = "LaunchPath")]
    pub launch_path: String,
    #[serde(rename = "Parameters")]
    pub parameters: Option<String>,
    #[serde(rename = "ConcurrentExecutions")]
    pub concurrent_executions: usize,

}


}

pub mod cfn_game_server_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGameServerGroup {
    /// The type of delete to perform.
    #[serde(rename = "DeleteOption")]
    pub delete_option: Option<DeleteOption>,
    /// A list of virtual private cloud (VPC) subnets to use with instances in the game server group.
    #[serde(rename = "VpcSubnets")]
    pub vpc_subnets: Option<VpcSubnets>,
    /// A flag that indicates whether instances in the game server group are protected from early termination.
    #[serde(rename = "GameServerProtectionPolicy")]
    pub game_server_protection_policy: Option<GameServerProtectionPolicy>,
    /// Configuration settings to define a scaling policy for the Auto Scaling group that is optimized for game hosting
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,
    /// A list of labels to assign to the new game server group resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The minimum number of instances allowed in the EC2 Auto Scaling group.
    #[serde(rename = "MinSize")]
    pub min_size: Option<MinSize>,
    /// The maximum number of instances allowed in the EC2 Auto Scaling group.
    #[serde(rename = "MaxSize")]
    pub max_size: Option<MaxSize>,
    /// The fallback balancing method to use for the game server group when Spot Instances in a Region become unavailable or are not viable for game hosting.
    #[serde(rename = "BalancingStrategy")]
    pub balancing_strategy: Option<BalancingStrategy>,
    /// A set of EC2 instance types to use when creating instances in the group.
    #[serde(rename = "InstanceDefinitions")]
    pub instance_definitions: InstanceDefinitions,
    /// An identifier for the new game server group.
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: GameServerGroupName,
    /// The EC2 launch template that contains configuration settings and game server code to be deployed to all instances in the game server group.
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplate>,
    /// The Amazon Resource Name (ARN) for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,

}

pub type GameServerGroupArn = String;pub type InstanceType = String;
#[derive(serde::Serialize, Default)]
pub struct InstanceDefinition {
    #[serde(rename = "InstanceType")]
    pub instance_type: InstanceType,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<WeightedCapacity>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplate {
    #[serde(rename = "Version")]
    pub version: Option<Version>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<LaunchTemplateId>,
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<LaunchTemplateName>,

}
pub type LaunchTemplateId = String;pub type MinSize = f64;pub type DeleteOption = String;pub type GameServerProtectionPolicy = String;
#[derive(serde::Serialize, Default)]
pub struct AutoScalingPolicy {
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<EstimatedInstanceWarmup>,
    #[serde(rename = "TargetTrackingConfiguration")]
    pub target_tracking_configuration: TargetTrackingConfiguration,

}
pub type WeightedCapacity = String;pub type Status = String;pub type BalancingStrategy = String;pub type SuspendedActions = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}
pub type CreationTime = String;pub type EstimatedInstanceWarmup = f64;pub type GameServerGroupName = String;pub type RoleArn = String;pub type Version = String;pub type StatusReason = String;pub type LastUpdatedTime = String;
#[derive(serde::Serialize, Default)]
pub struct TargetTrackingConfiguration {
    #[serde(rename = "TargetValue")]
    pub target_value: TargetValue,

}
pub type AutoScalingGroupArn = String;pub type VpcSubnets = Vec<String>;
#[derive(serde::Serialize, Default)]
pub struct GameServerGroup {
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<LastUpdatedTime>,
    #[serde(rename = "AutoScalingGroupArn")]
    pub auto_scaling_group_arn: Option<AutoScalingGroupArn>,
    #[serde(rename = "GameServerProtectionPolicy")]
    pub game_server_protection_policy: Option<GameServerProtectionPolicy>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<RoleArn>,
    #[serde(rename = "Status")]
    pub status: Option<Status>,
    #[serde(rename = "InstanceDefinitions")]
    pub instance_definitions: Option<InstanceDefinitions>,
    #[serde(rename = "StatusReason")]
    pub status_reason: Option<StatusReason>,
    #[serde(rename = "SuspendedActions")]
    pub suspended_actions: Option<SuspendedActions>,
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<CreationTime>,
    #[serde(rename = "GameServerGroupArn")]
    pub game_server_group_arn: Option<GameServerGroupArn>,
    #[serde(rename = "BalancingStrategy")]
    pub balancing_strategy: Option<BalancingStrategy>,
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: Option<GameServerGroupName>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}
pub type LaunchTemplateName = String;pub type TargetValue = f64;
#[derive(serde::Serialize, Default)]
pub struct InstanceDefinitions {

}
pub type MaxSize = f64;

}

pub mod cfn_game_session_queue {

#[derive(serde::Serialize, Default)]
pub struct CfnGameSessionQueue {
    /// No documentation provided by AWS
    #[serde(rename = "FilterConfiguration")]
    pub filter_configuration: Option<FilterConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "CustomEventData")]
    pub custom_event_data: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of PlayerLatencyPolicy
    #[serde(rename = "PlayerLatencyPolicies")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    /// No documentation provided by AWS
    #[serde(rename = "PriorityConfiguration")]
    pub priority_configuration: Option<PriorityConfiguration>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<usize>,
    /// List of Destination
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Destination>>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationTarget")]
    pub notification_target: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PriorityConfiguration {
    #[serde(rename = "LocationOrder")]
    pub location_order: Option<Vec<String>>,
    #[serde(rename = "PriorityOrder")]
    pub priority_order: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Destination {
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PlayerLatencyPolicy {
    #[serde(rename = "PolicyDurationSeconds")]
    pub policy_duration_seconds: Option<usize>,
    #[serde(rename = "MaximumIndividualPlayerLatencyMilliseconds")]
    pub maximum_individual_player_latency_milliseconds: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct FilterConfiguration {
    #[serde(rename = "AllowedLocations")]
    pub allowed_locations: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_location {

#[derive(serde::Serialize, Default)]
pub struct CfnLocation {
    /// No documentation provided by AWS
    #[serde(rename = "LocationName")]
    pub location_name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_matchmaking_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnMatchmakingConfiguration {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptanceRequired")]
    pub acceptance_required: bool,
    /// No documentation provided by AWS
    #[serde(rename = "FlexMatchMode")]
    pub flex_match_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "CustomEventData")]
    pub custom_event_data: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "BackfillMode")]
    pub backfill_mode: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GameSessionQueueArns")]
    pub game_session_queue_arns: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RequestTimeoutSeconds")]
    pub request_timeout_seconds: usize,
    /// List of GameProperty
    #[serde(rename = "GameProperties")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// No documentation provided by AWS
    #[serde(rename = "GameSessionData")]
    pub game_session_data: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NotificationTarget")]
    pub notification_target: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptanceTimeoutSeconds")]
    pub acceptance_timeout_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "RuleSetName")]
    pub rule_set_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalPlayerCount")]
    pub additional_player_count: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct GameProperty {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_matchmaking_rule_set {

#[derive(serde::Serialize, Default)]
pub struct CfnMatchmakingRuleSet {
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "RuleSetBody")]
    pub rule_set_body: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_script {

#[derive(serde::Serialize, Default)]
pub struct CfnScript {
    /// No documentation provided by AWS
    #[serde(rename = "Version")]
    pub version: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "StorageLocation")]
    pub storage_location: S3Location,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct S3Location {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "ObjectVersion")]
    pub object_version: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
