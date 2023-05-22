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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGameServerGroup {
    ///
    /// Configuration settings to define a scaling policy for the Auto Scaling group that is       optimized for game hosting. The scaling policy uses the metric         "PercentUtilizedGameServers" to maintain a buffer of idle game servers       that can immediately accommodate new games and players. After the Auto Scaling group is       created, update this value directly in the Auto Scaling group using the AWS console or       APIs.
    ///
    /// Required: No
    ///
    /// Type: AutoScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_strategy: Option<GameServerGroupBalancingStrategyEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<GameServerGroupDeleteOptionEnum>,

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
    pub game_server_group_name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_protection_policy: Option<GameServerGroupGameServerProtectionPolicyEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplate>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<f64>,

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
    pub role_arn: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnets: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_auto_scaling_group_arn: CfnGameServerGroupautoscalinggrouparn,

    #[serde(skip_serializing)]
    pub att_game_server_group_arn: CfnGameServerGroupgameservergrouparn,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum GameServerGroupBalancingStrategyEnum {
    /// ON_DEMAND_ONLY
    #[serde(rename = "ON_DEMAND_ONLY")]
    Ondemandonly,

    /// SPOT_ONLY
    #[serde(rename = "SPOT_ONLY")]
    Spotonly,

    /// SPOT_PREFERRED
    #[serde(rename = "SPOT_PREFERRED")]
    Spotpreferred,
}

impl Default for GameServerGroupBalancingStrategyEnum {
    fn default() -> Self {
        GameServerGroupBalancingStrategyEnum::Ondemandonly
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum GameServerGroupDeleteOptionEnum {
    /// FORCE_DELETE
    #[serde(rename = "FORCE_DELETE")]
    Forcedelete,

    /// RETAIN
    #[serde(rename = "RETAIN")]
    Retain,

    /// SAFE_DELETE
    #[serde(rename = "SAFE_DELETE")]
    Safedelete,
}

impl Default for GameServerGroupDeleteOptionEnum {
    fn default() -> Self {
        GameServerGroupDeleteOptionEnum::Forcedelete
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum GameServerGroupGameServerProtectionPolicyEnum {
    /// FULL_PROTECTION
    #[serde(rename = "FULL_PROTECTION")]
    Fullprotection,

    /// NO_PROTECTION
    #[serde(rename = "NO_PROTECTION")]
    Noprotection,
}

impl Default for GameServerGroupGameServerProtectionPolicyEnum {
    fn default() -> Self {
        GameServerGroupGameServerProtectionPolicyEnum::Fullprotection
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGameServerGroupautoscalinggrouparn;
impl CfnGameServerGroupautoscalinggrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"AutoScalingGroupArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGameServerGroupgameservergrouparn;
impl CfnGameServerGroupgameservergrouparn {
    pub fn att_name(&self) -> &'static str {
        r#"GameServerGroupArn"#
    }
}

impl cfn_resources::CfnResource for CfnGameServerGroup {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::GameServerGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_scaling_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.game_server_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!("Max validation failed on field 'game_server_group_name'. {} is greater than 128", s.len()));
            }
        }

        let the_val = &self.game_server_group_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'game_server_group_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.instance_definitions;

        if the_val.len() > 20 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_definitions'. {} is greater than 20",
                the_val.len()
            ));
        }

        self.launch_template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.max_size {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'max_size'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min_size {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min_size'. {} is less than 0",
                    the_val
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 256",
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

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.vpc_subnets {
            if the_val.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpc_subnets'. {} is greater than 20",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// This data type is used with the GameLift FleetIQ and game server groups.
///
/// Configuration settings for intelligent automatic scaling that uses target tracking.    After the Auto Scaling group is created, all updates to Auto Scaling policies, including    changing this policy and adding or removing other policies, is done directly on the Auto    Scaling group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoScalingPolicy {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<f64>,

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
}

impl cfn_resources::CfnResource for AutoScalingPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.estimated_instance_warmup {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'estimated_instance_warmup'. {} is less than 1",
                    the_val
                ));
            }
        }

        self.target_tracking_configuration.validate()?;

        Ok(())
    }
}

/// This data type is used with the Amazon GameLift FleetIQ and game server groups.
///
/// An allowed instance type for a GameServerGroup. All game server groups must have at least two    instance types defined for it. GameLift FleetIQ periodically evaluates each defined instance type    for viability. It then updates the Auto Scaling group with the list of viable instance    types.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub instance_type: InstanceDefinitionInstanceTypeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InstanceDefinitionInstanceTypeEnum {
    /// c4.2xlarge
    #[serde(rename = "c4.2xlarge")]
    C42xlarge,

    /// c4.4xlarge
    #[serde(rename = "c4.4xlarge")]
    C44xlarge,

    /// c4.8xlarge
    #[serde(rename = "c4.8xlarge")]
    C48xlarge,

    /// c4.large
    #[serde(rename = "c4.large")]
    C4large,

    /// c4.xlarge
    #[serde(rename = "c4.xlarge")]
    C4xlarge,

    /// c5.12xlarge
    #[serde(rename = "c5.12xlarge")]
    C512xlarge,

    /// c5.18xlarge
    #[serde(rename = "c5.18xlarge")]
    C518xlarge,

    /// c5.24xlarge
    #[serde(rename = "c5.24xlarge")]
    C524xlarge,

    /// c5.2xlarge
    #[serde(rename = "c5.2xlarge")]
    C52xlarge,

    /// c5.4xlarge
    #[serde(rename = "c5.4xlarge")]
    C54xlarge,

    /// c5.9xlarge
    #[serde(rename = "c5.9xlarge")]
    C59xlarge,

    /// c5.large
    #[serde(rename = "c5.large")]
    C5large,

    /// c5.xlarge
    #[serde(rename = "c5.xlarge")]
    C5xlarge,

    /// c5a.12xlarge
    #[serde(rename = "c5a.12xlarge")]
    C5a12xlarge,

    /// c5a.16xlarge
    #[serde(rename = "c5a.16xlarge")]
    C5a16xlarge,

    /// c5a.24xlarge
    #[serde(rename = "c5a.24xlarge")]
    C5a24xlarge,

    /// c5a.2xlarge
    #[serde(rename = "c5a.2xlarge")]
    C5a2xlarge,

    /// c5a.4xlarge
    #[serde(rename = "c5a.4xlarge")]
    C5a4xlarge,

    /// c5a.8xlarge
    #[serde(rename = "c5a.8xlarge")]
    C5a8xlarge,

    /// c5a.large
    #[serde(rename = "c5a.large")]
    C5alarge,

    /// c5a.xlarge
    #[serde(rename = "c5a.xlarge")]
    C5axlarge,

    /// c6g.12xlarge
    #[serde(rename = "c6g.12xlarge")]
    C6g12xlarge,

    /// c6g.16xlarge
    #[serde(rename = "c6g.16xlarge")]
    C6g16xlarge,

    /// c6g.2xlarge
    #[serde(rename = "c6g.2xlarge")]
    C6g2xlarge,

    /// c6g.4xlarge
    #[serde(rename = "c6g.4xlarge")]
    C6g4xlarge,

    /// c6g.8xlarge
    #[serde(rename = "c6g.8xlarge")]
    C6g8xlarge,

    /// c6g.large
    #[serde(rename = "c6g.large")]
    C6glarge,

    /// c6g.medium
    #[serde(rename = "c6g.medium")]
    C6gmedium,

    /// c6g.xlarge
    #[serde(rename = "c6g.xlarge")]
    C6gxlarge,

    /// m4.10xlarge
    #[serde(rename = "m4.10xlarge")]
    M410xlarge,

    /// m4.2xlarge
    #[serde(rename = "m4.2xlarge")]
    M42xlarge,

    /// m4.4xlarge
    #[serde(rename = "m4.4xlarge")]
    M44xlarge,

    /// m4.large
    #[serde(rename = "m4.large")]
    M4large,

    /// m4.xlarge
    #[serde(rename = "m4.xlarge")]
    M4xlarge,

    /// m5.12xlarge
    #[serde(rename = "m5.12xlarge")]
    M512xlarge,

    /// m5.16xlarge
    #[serde(rename = "m5.16xlarge")]
    M516xlarge,

    /// m5.24xlarge
    #[serde(rename = "m5.24xlarge")]
    M524xlarge,

    /// m5.2xlarge
    #[serde(rename = "m5.2xlarge")]
    M52xlarge,

    /// m5.4xlarge
    #[serde(rename = "m5.4xlarge")]
    M54xlarge,

    /// m5.8xlarge
    #[serde(rename = "m5.8xlarge")]
    M58xlarge,

    /// m5.large
    #[serde(rename = "m5.large")]
    M5large,

    /// m5.xlarge
    #[serde(rename = "m5.xlarge")]
    M5xlarge,

    /// m5a.12xlarge
    #[serde(rename = "m5a.12xlarge")]
    M5a12xlarge,

    /// m5a.16xlarge
    #[serde(rename = "m5a.16xlarge")]
    M5a16xlarge,

    /// m5a.24xlarge
    #[serde(rename = "m5a.24xlarge")]
    M5a24xlarge,

    /// m5a.2xlarge
    #[serde(rename = "m5a.2xlarge")]
    M5a2xlarge,

    /// m5a.4xlarge
    #[serde(rename = "m5a.4xlarge")]
    M5a4xlarge,

    /// m5a.8xlarge
    #[serde(rename = "m5a.8xlarge")]
    M5a8xlarge,

    /// m5a.large
    #[serde(rename = "m5a.large")]
    M5alarge,

    /// m5a.xlarge
    #[serde(rename = "m5a.xlarge")]
    M5axlarge,

    /// m6g.12xlarge
    #[serde(rename = "m6g.12xlarge")]
    M6g12xlarge,

    /// m6g.16xlarge
    #[serde(rename = "m6g.16xlarge")]
    M6g16xlarge,

    /// m6g.2xlarge
    #[serde(rename = "m6g.2xlarge")]
    M6g2xlarge,

    /// m6g.4xlarge
    #[serde(rename = "m6g.4xlarge")]
    M6g4xlarge,

    /// m6g.8xlarge
    #[serde(rename = "m6g.8xlarge")]
    M6g8xlarge,

    /// m6g.large
    #[serde(rename = "m6g.large")]
    M6glarge,

    /// m6g.medium
    #[serde(rename = "m6g.medium")]
    M6gmedium,

    /// m6g.xlarge
    #[serde(rename = "m6g.xlarge")]
    M6gxlarge,

    /// r4.16xlarge
    #[serde(rename = "r4.16xlarge")]
    R416xlarge,

    /// r4.2xlarge
    #[serde(rename = "r4.2xlarge")]
    R42xlarge,

    /// r4.4xlarge
    #[serde(rename = "r4.4xlarge")]
    R44xlarge,

    /// r4.8xlarge
    #[serde(rename = "r4.8xlarge")]
    R48xlarge,

    /// r4.large
    #[serde(rename = "r4.large")]
    R4large,

    /// r4.xlarge
    #[serde(rename = "r4.xlarge")]
    R4xlarge,

    /// r5.12xlarge
    #[serde(rename = "r5.12xlarge")]
    R512xlarge,

    /// r5.16xlarge
    #[serde(rename = "r5.16xlarge")]
    R516xlarge,

    /// r5.24xlarge
    #[serde(rename = "r5.24xlarge")]
    R524xlarge,

    /// r5.2xlarge
    #[serde(rename = "r5.2xlarge")]
    R52xlarge,

    /// r5.4xlarge
    #[serde(rename = "r5.4xlarge")]
    R54xlarge,

    /// r5.8xlarge
    #[serde(rename = "r5.8xlarge")]
    R58xlarge,

    /// r5.large
    #[serde(rename = "r5.large")]
    R5large,

    /// r5.xlarge
    #[serde(rename = "r5.xlarge")]
    R5xlarge,

    /// r5a.12xlarge
    #[serde(rename = "r5a.12xlarge")]
    R5a12xlarge,

    /// r5a.16xlarge
    #[serde(rename = "r5a.16xlarge")]
    R5a16xlarge,

    /// r5a.24xlarge
    #[serde(rename = "r5a.24xlarge")]
    R5a24xlarge,

    /// r5a.2xlarge
    #[serde(rename = "r5a.2xlarge")]
    R5a2xlarge,

    /// r5a.4xlarge
    #[serde(rename = "r5a.4xlarge")]
    R5a4xlarge,

    /// r5a.8xlarge
    #[serde(rename = "r5a.8xlarge")]
    R5a8xlarge,

    /// r5a.large
    #[serde(rename = "r5a.large")]
    R5alarge,

    /// r5a.xlarge
    #[serde(rename = "r5a.xlarge")]
    R5axlarge,

    /// r6g.12xlarge
    #[serde(rename = "r6g.12xlarge")]
    R6g12xlarge,

    /// r6g.16xlarge
    #[serde(rename = "r6g.16xlarge")]
    R6g16xlarge,

    /// r6g.2xlarge
    #[serde(rename = "r6g.2xlarge")]
    R6g2xlarge,

    /// r6g.4xlarge
    #[serde(rename = "r6g.4xlarge")]
    R6g4xlarge,

    /// r6g.8xlarge
    #[serde(rename = "r6g.8xlarge")]
    R6g8xlarge,

    /// r6g.large
    #[serde(rename = "r6g.large")]
    R6glarge,

    /// r6g.medium
    #[serde(rename = "r6g.medium")]
    R6gmedium,

    /// r6g.xlarge
    #[serde(rename = "r6g.xlarge")]
    R6gxlarge,
}

impl Default for InstanceDefinitionInstanceTypeEnum {
    fn default() -> Self {
        InstanceDefinitionInstanceTypeEnum::C42xlarge
    }
}

impl cfn_resources::CfnResource for InstanceDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.weighted_capacity {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 3 as _ {
                    return Err(format!(
                        "Max validation failed on field 'weighted_capacity'. {} is greater than 3",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.weighted_capacity {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'weighted_capacity'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// This data type is used with the GameLift FleetIQ and game server groups.
///
/// An Amazon EC2 launch template that contains configuration settings and game server code to    be deployed to all instances in a game server group. The launch template is specified    when creating a new game server group with GameServerGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchTemplate {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LaunchTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.launch_template_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'launch_template_id'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.launch_template_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'launch_template_id'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.launch_template_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!("Max validation failed on field 'launch_template_name'. {} is greater than 128", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.launch_template_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 3 as _ {
                    return Err(format!(
                        "Min validation failed on field 'launch_template_name'. {} is less than 3",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 128 as _ {
                    return Err(format!(
                        "Max validation failed on field 'version'. {} is greater than 128",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'version'. {} is less than 1",
                        s.len()
                    ));
                }
            }
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

/// This data type is used with the Amazon GameLift FleetIQ and game server groups.
///
/// Settings for a target-based scaling policy as part of a GameServerGroupAutoScalingPolicy.    These settings are used to    create a target-based policy that tracks the GameLift FleetIQ metric    "PercentUtilizedGameServers" and specifies a target value for the    metric. As player usage changes, the policy triggers to adjust the game server group    capacity so that the metric returns to the target value.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for TargetTrackingConfiguration {
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
