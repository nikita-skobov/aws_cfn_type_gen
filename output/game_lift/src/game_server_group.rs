

/// This operation is used with the Amazon GameLift FleetIQ solution and game server groups.
///
/// Creates a GameLift FleetIQ game server group for managing game hosting on a collection of       Amazon EC2 instances for game hosting. This operation creates the game server group,       creates an Auto Scaling group in your AWS account, and establishes a link between the       two groups. You can view the status of your game server groups in the GameLift console.       Game server group metrics and events are emitted to Amazon CloudWatch.
///
/// Before creating a new game server group, you must have the following:
///
/// To create a new game server group, specify a unique group name, IAM role and Amazon EC2       launch template, and provide a list of instance types that can be used in the group. You       must also set initial maximum and minimum limits on the group's instance count. You can       optionally set an Auto Scaling policy with target tracking based on a GameLift FleetIQ       metric.
///
/// Once the game server group and corresponding Auto Scaling group are created, you have       full access to change the Auto Scaling group's configuration as needed. Several       properties that are set when creating a game server group, including maximum/minimum       size and auto-scaling policy settings, must be updated directly in the Auto Scaling       group. Keep in mind that some Auto Scaling group properties are periodically updated by       GameLift FleetIQ as part of its balancing activities to optimize for availability and       cost.
///
/// Learn more
///
/// GameLift FleetIQ Guide
#[derive(Default, serde::Serialize)]
pub struct CfnGameServerGroup {


    /// 
    /// The type of delete to perform. To delete a game server group, specify the     DeleteOption. Options include the following:
    /// 
    /// SAFE_DELETE – (default) Terminates the game server group and           Amazon EC2 Auto Scaling group only when it has no game servers that are in             UTILIZED status.                         FORCE_DELETE – Terminates the game server group, including all           active game servers regardless of their utilization status, and the Amazon EC2 Auto           Scaling group.                         RETAIN – Does a safe delete of the game server group but retains           the Amazon EC2 Auto Scaling group as is.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FORCE_DELETE | RETAIN | SAFE_DELETE
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOption")]
    pub delete_option: Option<String>,


    /// 
    /// A developer-defined identifier for the game server group. The name is unique for each       Region in each AWS account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9-\.]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,


    /// 
    /// A flag that indicates whether instances in the game server group are protected       from early termination. Unprotected instances that have active game servers running might       be terminated during a scale-down event, causing players to be dropped from the game.       Protected instances cannot be terminated while there are active game servers running except       in the event of a forced game server group deletion (see ). An exception to this is with Spot       Instances, which can be terminated by AWS regardless of protection status.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FULL_PROTECTION | NO_PROTECTION
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameServerProtectionPolicy")]
    pub game_server_protection_policy: Option<String>,


    /// 
    /// Indicates how Amazon GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances in the       game server group. Method options include the following:
    /// 
    /// SPOT_ONLY - Only Spot Instances are used in the game server group. If Spot           Instances are unavailable or not viable for game hosting, the game server group           provides no hosting capacity until Spot Instances can again be used. Until then,           no new instances are started, and the existing nonviable Spot Instances are           terminated (after current gameplay ends) and are not replaced.                        SPOT_PREFERRED - (default value) Spot Instances are used whenever available in           the game server group. If Spot Instances are unavailable, the game server group           continues to provide hosting capacity by falling back to On-Demand Instances.           Existing nonviable Spot Instances are terminated (after current gameplay ends)           and are replaced with new On-Demand Instances.                        ON_DEMAND_ONLY - Only On-Demand Instances are used in the game           server group. No Spot Instances are used, even when available, while this           balancing strategy is in force.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND_ONLY | SPOT_ONLY | SPOT_PREFERRED
    ///
    /// Update requires: No interruption
    #[serde(rename = "BalancingStrategy")]
    pub balancing_strategy: Option<String>,


    /// 
    /// A list of labels to assign to the new game server group resource. Tags are    developer-defined key-value pairs. Tagging AWS resources is useful for resource    management, access management, and cost allocation. For more information, see Tagging AWS     Resources in the      AWS General Reference. Once the    resource is created, you can use TagResource, UntagResource, and ListTagsForResource to add, remove,    and view tags, respectively. The maximum tag limit may be lower than stated. See the    AWS General Reference for actual tagging limits.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Configuration settings to define a scaling policy for the Auto Scaling group that is       optimized for game hosting. The scaling policy uses the metric         "PercentUtilizedGameServers" to maintain a buffer of idle game servers       that can immediately accommodate new games and players. After the Auto Scaling group is       created, update this value directly in the Auto Scaling group using the AWS console or       APIs.
    /// 
    /// Required: No
    ///
    /// Type: AutoScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,


    /// 
    /// The set of Amazon EC2 instance types that Amazon GameLift FleetIQ can use when balancing and automatically       scaling instances in the corresponding Auto Scaling group.
    /// 
    /// Required: Yes
    ///
    /// Type: List of InstanceDefinition
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceDefinitions")]
    pub instance_definitions: Vec<InstanceDefinition>,


    /// 
    /// A list of virtual private cloud (VPC) subnets to use with instances in the game server       group. By default, all Amazon GameLift FleetIQ-supported Availability Zones are used. You can use this       parameter to specify VPCs that you've set up. This property cannot be updated after the       game server group is created, and the corresponding Auto Scaling group will always use       the property value that is set with this request, even if the Auto Scaling group is       updated directly.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSubnets")]
    pub vpc_subnets: Option<Vec<String>>,


    /// 
    /// The Amazon EC2 launch template that contains configuration settings and game server code to       be deployed to all instances in the game server group. You can specify the template       using either the template name or ID. For help with creating a launch template, see         Creating a Launch         Template for an Auto Scaling Group in the Amazon Elastic Compute Cloud Auto Scaling         User Guide. After the Auto Scaling group is created, update this value       directly in the Auto Scaling group using the AWS console or APIs.
    /// 
    /// NoteIf you specify network interfaces in your launch template, you must explicitly set         the property AssociatePublicIpAddress to "true". If no network         interface is specified in the launch template, Amazon GameLift FleetIQ uses your account's default         VPC.
    /// 
    /// Required: No
    ///
    /// Type: LaunchTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplate>,


    /// 
    /// The minimum number of instances allowed in the Amazon EC2 Auto Scaling group. During       automatic scaling events, Amazon GameLift FleetIQ and Amazon EC2 do not scale down the group below this       minimum. In production, this value should be set to at least 1. After the Auto Scaling       group is created, update this value directly in the Auto Scaling group using the AWS       console or APIs.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: Option<f64>,


    /// 
    /// The maximum number of instances allowed in the Amazon EC2 Auto Scaling group. During       automatic scaling events, Amazon GameLift FleetIQ and EC2 do not scale up the group above this maximum.       After the Auto Scaling group is created, update this value directly in the Auto Scaling       group using the AWS console or APIs.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: Option<f64>,


    /// 
    /// The Amazon Resource Name (ARN) for an IAM role that       allows Amazon GameLift to access your Amazon EC2 Auto Scaling groups.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:.*:role\/[\w+=,.@-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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


/// This data type is used with the GameLift FleetIQ and game server groups.
///
/// Configuration settings for intelligent automatic scaling that uses target tracking.    After the Auto Scaling group is created, all updates to Auto Scaling policies, including    changing this policy and adding or removing other policies, is done directly on the Auto    Scaling group.
#[derive(Default, serde::Serialize)]
pub struct AutoScalingPolicy {


    /// 
    /// Settings for a target-based scaling policy applied to Auto Scaling group.    These settings are used to create a target-based policy that tracks the GameLift    FleetIQ metric PercentUtilizedGameServers and specifies a target value    for the metric. As player usage changes, the policy triggers to adjust the game server group    capacity so that the metric returns to the target value.
    /// 
    /// Required: Yes
    ///
    /// Type: TargetTrackingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTrackingConfiguration")]
    pub target_tracking_configuration: TargetTrackingConfiguration,


    /// 
    /// Length of time, in seconds, it takes for a new instance to start new game server       processes and register with Amazon GameLift FleetIQ. Specifying a warm-up time can be useful, particularly       with game servers that take a long time to start up, because it avoids prematurely       starting new instances.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<f64>,

}


/// This data type is used with the Amazon GameLift FleetIQ and game server groups.
///
/// Settings for a target-based scaling policy as part of a GameServerGroupAutoScalingPolicy.    These settings are used to    create a target-based policy that tracks the GameLift FleetIQ metric    "PercentUtilizedGameServers" and specifies a target value for the    metric. As player usage changes, the policy triggers to adjust the game server group    capacity so that the metric returns to the target value.
#[derive(Default, serde::Serialize)]
pub struct TargetTrackingConfiguration {


    /// 
    /// Desired value to use with a game server group target-based scaling policy.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}


/// This data type is used with the GameLift FleetIQ and game server groups.
///
/// An Amazon EC2 launch template that contains configuration settings and game server code to    be deployed to all instances in a game server group. The launch template is specified    when creating a new game server group with GameServerGroup.
#[derive(Default, serde::Serialize)]
pub struct LaunchTemplate {


    /// 
    /// The version of the Amazon EC2 launch template to use. If no version is specified, the       default version will be used. With Amazon EC2, you can specify a default version for a launch       template. If none is set, the default is the first version created.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,


    /// 
    /// A readable identifier for an existing Amazon EC2 launch template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9\(\)\.\-/_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,


    /// 
    /// A unique identifier for an existing Amazon EC2 launch template.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,

}


/// This data type is used with the Amazon GameLift FleetIQ and game server groups.
///
/// An allowed instance type for a GameServerGroup. All game server groups must have at least two    instance types defined for it. GameLift FleetIQ periodically evaluates each defined instance type    for viability. It then updates the Auto Scaling group with the list of viable instance    types.
#[derive(Default, serde::Serialize)]
pub struct InstanceDefinition {


    /// 
    /// An Amazon EC2 instance type designation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: c4.2xlarge | c4.4xlarge | c4.8xlarge | c4.large | c4.xlarge | c5.12xlarge | c5.18xlarge | c5.24xlarge | c5.2xlarge | c5.4xlarge | c5.9xlarge | c5.large | c5.xlarge | c5a.12xlarge | c5a.16xlarge | c5a.24xlarge | c5a.2xlarge | c5a.4xlarge | c5a.8xlarge | c5a.large | c5a.xlarge | c6g.12xlarge | c6g.16xlarge | c6g.2xlarge | c6g.4xlarge | c6g.8xlarge | c6g.large | c6g.medium | c6g.xlarge | m4.10xlarge | m4.2xlarge | m4.4xlarge | m4.large | m4.xlarge | m5.12xlarge | m5.16xlarge | m5.24xlarge | m5.2xlarge | m5.4xlarge | m5.8xlarge | m5.large | m5.xlarge | m5a.12xlarge | m5a.16xlarge | m5a.24xlarge | m5a.2xlarge | m5a.4xlarge | m5a.8xlarge | m5a.large | m5a.xlarge | m6g.12xlarge | m6g.16xlarge | m6g.2xlarge | m6g.4xlarge | m6g.8xlarge | m6g.large | m6g.medium | m6g.xlarge | r4.16xlarge | r4.2xlarge | r4.4xlarge | r4.8xlarge | r4.large | r4.xlarge | r5.12xlarge | r5.16xlarge | r5.24xlarge | r5.2xlarge | r5.4xlarge | r5.8xlarge | r5.large | r5.xlarge | r5a.12xlarge | r5a.16xlarge | r5a.24xlarge | r5a.2xlarge | r5a.4xlarge | r5a.8xlarge | r5a.large | r5a.xlarge | r6g.12xlarge | r6g.16xlarge | r6g.2xlarge | r6g.4xlarge | r6g.8xlarge | r6g.large | r6g.medium | r6g.xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,


    /// 
    /// Instance weighting that indicates how much this instance type contributes to the total       capacity of a game server group. Instance weights are used by Amazon GameLift FleetIQ to calculate the       instance type's cost per unit hour and better identify the most cost-effective options.       For detailed information on weighting instance capacity, see Instance         Weighting in the Amazon Elastic Compute Cloud Auto Scaling User Guide.       Default value is "1".
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 3
    ///
    /// Pattern: ^[\u0031-\u0039][\u0030-\u0039]{0,2}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<String>,

}