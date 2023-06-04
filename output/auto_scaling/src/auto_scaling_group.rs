/// The AWS::AutoScaling::AutoScalingGroup resource defines an Amazon EC2 Auto    Scaling group, which is a collection of Amazon EC2 instances that are treated as a logical    grouping for the purposes of automatic scaling and management.
///
/// For more information about Amazon EC2 Auto Scaling, see the Amazon EC2 Auto Scaling     User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnAutoScalingGroup {
    ///
    /// The name of the Auto Scaling group. This name must be unique per Region per account.
    ///
    /// The name can contain any ASCII character 33 to 126 including most punctuation       characters, digits, and upper and lowercased letters.
    ///
    /// NoteYou cannot use a colon (:) in the name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingGroupName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub auto_scaling_group_name: Option<cfn_resources::StrVal>,

    ///
    /// A list of Availability Zones where instances in the Auto Scaling group can be created. Used       for launching into the default VPC subnet in each Availability Zone when not using the         VPCZoneIdentifier property, or for attaching a network interface when       an existing network interface ID is specified in a launch template.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// Indicates whether Capacity Rebalancing is enabled. Otherwise, Capacity Rebalancing is       disabled. When you turn on Capacity Rebalancing, Amazon EC2 Auto Scaling attempts to launch a Spot       Instance whenever Amazon EC2 notifies that a Spot Instance is at an elevated risk of       interruption. After launching a new instance, it then terminates an old instance. For       more information, see Use Capacity         Rebalancing to handle Amazon EC2 Spot Interruptions in the in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CapacityRebalance")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub capacity_rebalance: Option<bool>,

    ///
    /// Reserved.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Context")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub context: Option<cfn_resources::StrVal>,

    ///
    /// Only needed if you use simple scaling policies.
    ///
    /// The amount of time, in seconds, between one scaling activity ending and another one       starting due to simple scaling policies. For more information, see Scaling cooldowns         for Amazon EC2 Auto Scaling in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Default: 300 seconds
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cooldown")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cooldown: Option<cfn_resources::StrVal>,

    ///
    /// The amount of time, in seconds, until a new instance is considered to have finished       initializing and resource consumption to become stable after it enters the         InService state.
    ///
    /// During an instance refresh, Amazon EC2 Auto Scaling waits for the warm-up period after it replaces an       instance before it moves on to replacing the next instance. Amazon EC2 Auto Scaling also waits for the       warm-up period before aggregating the metrics for new instances with existing instances       in the Amazon CloudWatch metrics that are used for scaling, resulting in more reliable usage       data. For more information, see Set         the default instance warmup for an Auto Scaling group in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// ImportantTo manage various warm-up settings at the group level, we recommend that you set         the default instance warmup, even if it is set to 0 seconds. To         remove a value that you previously set, include the property but specify           -1 for the value. However, we strongly recommend keeping the         default instance warmup enabled by specifying a value of 0 or other         nominal value.
    ///
    /// Default: None
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultInstanceWarmup")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub default_instance_warmup: Option<i64>,

    ///
    /// The desired capacity is the initial capacity of the Auto Scaling group at the time of its    creation and the capacity it attempts to maintain. It can scale beyond this capacity if you    configure automatic scaling.
    ///
    /// The number must be greater than or equal to the minimum size of the group and less than or    equal to the maximum size of the group. If you do not specify a desired capacity when creating    the stack, the default is the minimum size of the group.
    ///
    /// CloudFormation marks the Auto Scaling group as successful (by setting its status to    CREATE_COMPLETE) when the desired capacity is reached. However, if a maximum Spot price is set    in the launch template or launch configuration that you specified, then desired capacity is    not used as a criteria for success. Whether your request is fulfilled depends on Spot Instance    capacity and your maximum price.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredCapacity")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub desired_capacity: Option<cfn_resources::StrVal>,

    ///
    /// The unit of measurement for the value specified for desired capacity. Amazon EC2 Auto Scaling       supports DesiredCapacityType for attribute-based instance type selection       only. For more information, see Creating         an Auto Scaling group using attribute-based instance type selection in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// By default, Amazon EC2 Auto Scaling specifies units, which translates into number of       instances.
    ///
    /// Valid values: units | vcpu | memory-mib
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredCapacityType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub desired_capacity_type: Option<cfn_resources::StrVal>,

    ///
    /// The amount of time, in seconds, that Amazon EC2 Auto Scaling waits before checking the health status       of an EC2 instance that has come into service and marking it unhealthy due to a failed       health check. This is useful if your instances do not immediately pass their health       checks after they enter the InService state. For more information, see         Set the health check         grace period for an Auto Scaling group in the       Amazon EC2 Auto Scaling User Guide.
    ///
    /// Default: 0 seconds
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub health_check_grace_period: Option<i64>,

    ///
    /// A comma-separated value string of one or more health check types.
    ///
    /// The valid values are EC2, ELB, and VPC_LATTICE.         EC2 is the default health check and cannot be disabled. For more       information, see Health checks for Auto Scaling         instances in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Only specify EC2 if you must clear a value that was previously       set.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub health_check_type: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the instance used to base the launch configuration on. For more information, see     Create an Auto Scaling group using an EC2 instance in the Amazon EC2 Auto     Scaling User Guide.
    ///
    /// If you specify LaunchTemplate, MixedInstancesPolicy, or     LaunchConfigurationName, don't specify InstanceId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the launch configuration to use to launch instances.
    ///
    /// Required only if you don't specify LaunchTemplate,     MixedInstancesPolicy, or InstanceId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub launch_configuration_name: Option<cfn_resources::StrVal>,

    ///
    /// Information used to specify the launch template and version to use to launch instances.    You can alternatively associate a launch template to the Auto Scaling group by specifying a     MixedInstancesPolicy. For more information about creating launch templates, see     Create a launch template for an Auto Scaling group in the Amazon EC2 Auto     Scaling User Guide.
    ///
    /// If you omit this property, you must specify MixedInstancesPolicy,     LaunchConfigurationName, or InstanceId.
    ///
    /// Required: No
    ///
    /// Type: LaunchTemplateSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub launch_template: Option<LaunchTemplateSpecification>,

    ///
    /// One or more lifecycle hooks to add to the Auto Scaling group before instances are       launched.
    ///
    /// Required: No
    ///
    /// Type: List of LifecycleHookSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleHookSpecificationList")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lifecycle_hook_specification_list: Option<Vec<LifecycleHookSpecification>>,

    ///
    /// A list of Classic Load Balancers associated with this Auto Scaling group. For Application Load Balancers, Network Load Balancers, and Gateway Load Balancers,       specify the TargetGroupARNs property instead.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerNames")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub load_balancer_names: Option<Vec<String>>,

    ///
    /// The maximum amount of time, in seconds, that an instance can be in service. The       default is null. If specified, the value must be either 0 or a number equal to or       greater than 86,400 seconds (1 day). For more information, see Replacing Auto Scaling instances based on maximum instance lifetime in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxInstanceLifetime")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_instance_lifetime: Option<i64>,

    ///
    /// The maximum size of the group.
    ///
    /// NoteWith a mixed instances policy that uses instance weighting, Amazon EC2 Auto Scaling may need to         go above MaxSize to meet your capacity requirements. In this event,         Amazon EC2 Auto Scaling will never go above MaxSize by more than your largest instance         weight (weights that define how many units each instance contributes to the desired         capacity of the group).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxSize")]
    pub max_size: cfn_resources::StrVal,

    ///
    /// Enables the monitoring of group metrics of an Auto Scaling group. By default, these    metrics are disabled.
    ///
    /// Required: No
    ///
    /// Type: List of MetricsCollection
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricsCollection")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub metrics_collection: Option<Vec<MetricsCollection>>,

    ///
    /// The minimum size of the group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    pub min_size: cfn_resources::StrVal,

    ///
    /// An embedded object that specifies a mixed instances policy.
    ///
    /// The policy includes properties that not only define the distribution of On-Demand    Instances and Spot Instances, the maximum price to pay for Spot Instances (optional), and how    the Auto Scaling group allocates instance types to fulfill On-Demand and Spot capacities, but    also the properties that specify the instance configuration information—the launch template    and instance types. The policy can also include a weight for each instance type and different    launch templates for individual instance types.
    ///
    /// For more information, see Auto Scaling     groups with multiple instance types and purchase options in the Amazon EC2     Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: MixedInstancesPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,

    ///
    /// Indicates whether newly launched instances are protected from termination by Amazon EC2 Auto Scaling       when scaling in. For more information about preventing instances from terminating on       scale in, see Using         instance scale-in protection in the       Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "NewInstancesProtectedFromScaleIn")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub new_instances_protected_from_scale_in: Option<bool>,

    ///
    /// Configures an Auto Scaling group to send notifications when specified events take    place.
    ///
    /// Required: No
    ///
    /// Type: List of NotificationConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationConfigurations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notification_configurations: Option<Vec<NotificationConfiguration>>,

    ///
    /// The name of the placement group into which to launch your instances. For more       information, see Placement groups in the         Amazon EC2 User Guide for Linux Instances.
    ///
    /// NoteA cluster placement group is a logical grouping of instances         within a single Availability Zone. You cannot specify multiple Availability Zones         and a cluster placement group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlacementGroup")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub placement_group: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the service-linked role that the Auto Scaling group uses to       call other AWS service on your behalf. By default, Amazon EC2 Auto Scaling uses a service-linked role       named AWSServiceRoleForAutoScaling, which it creates if it does not exist.       For more information, see Service-linked         roles in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceLinkedRoleARN")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub service_linked_role_arn: Option<cfn_resources::StrVal>,

    ///
    /// One or more tags. You can tag your Auto Scaling group and propagate the tags to the Amazon EC2       instances it launches. Tags are not propagated to Amazon EBS volumes. To add tags to Amazon EBS       volumes, specify the tags in a launch template but use caution. If the launch template       specifies an instance tag with a key that is also specified for the Auto Scaling group, Amazon EC2 Auto Scaling       overrides the value of that instance tag with the value specified by the Auto Scaling group. For       more information, see Tag Auto Scaling groups and         instances in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of TagProperty
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<TagProperty>>,

    ///
    /// The Amazon Resource Names (ARN) of the Elastic Load Balancing target groups to associate with the Auto Scaling       group. Instances are registered as targets with the target groups. The target groups       receive incoming traffic and route requests to one or more registered targets. For more       information, see Use Elastic Load Balancing to         distribute traffic across the instances in your Auto Scaling group in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupARNs")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub target_group_arns: Option<Vec<String>>,

    ///
    /// A policy or a list of policies that are used to select the instance to terminate.       These policies are executed in the order that you list them. For more information, see         Work with         Amazon EC2 Auto Scaling termination policies in the       Amazon EC2 Auto Scaling User Guide.
    ///
    /// Valid values: Default | AllocationStrategy |         ClosestToNextInstanceHour | NewestInstance |         OldestInstance | OldestLaunchConfiguration |         OldestLaunchTemplate |         arn:aws:lambda:region:account-id:function:my-function:my-alias
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TerminationPolicies")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub termination_policies: Option<Vec<String>>,

    ///
    /// A list of subnet IDs for a virtual private cloud (VPC) where instances in the Auto Scaling    group can be created.
    ///
    /// If this resource specifies public subnets and is also in a VPC that is defined in the same    stack template, you must use the DependsOn     attribute to declare a dependency on the VPC-gateway attachment.
    ///
    /// NoteWhen you update VPCZoneIdentifier, this retains the same Auto Scaling group     and replaces old instances with new ones, according to the specified subnets. You can     optionally specify how CloudFormation handles these updates by using an UpdatePolicy      attribute.
    ///
    /// Required to launch instances into a nondefault VPC. If you specify     VPCZoneIdentifier with AvailabilityZones, the subnets that you    specify for this property must reside in those Availability Zones.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "VPCZoneIdentifier")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vpczone_identifier: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub att_launch_configuration_name: CfnAutoScalingGrouplaunchconfigurationname,

    #[serde(skip_serializing)]
    pub att_launch_template_specification: CfnAutoScalingGrouplaunchtemplatespecification,

    #[serde(skip_serializing)]
    pub att_mixed_instances_policy: CfnAutoScalingGroupmixedinstancespolicy,

    #[serde(skip_serializing)]
    pub att_placement_group: CfnAutoScalingGroupplacementgroup,

    #[serde(skip_serializing)]
    pub att_vpczone_identifier: CfnAutoScalingGroupvpczoneidentifier,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAutoScalingGrouplaunchconfigurationname;
impl CfnAutoScalingGrouplaunchconfigurationname {
    pub fn att_name(&self) -> &'static str {
        r#"LaunchConfigurationName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAutoScalingGrouplaunchtemplatespecification;
impl CfnAutoScalingGrouplaunchtemplatespecification {
    pub fn att_name(&self) -> &'static str {
        r#"LaunchTemplateSpecification"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAutoScalingGroupmixedinstancespolicy;
impl CfnAutoScalingGroupmixedinstancespolicy {
    pub fn att_name(&self) -> &'static str {
        r#"MixedInstancesPolicy"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAutoScalingGroupplacementgroup;
impl CfnAutoScalingGroupplacementgroup {
    pub fn att_name(&self) -> &'static str {
        r#"PlacementGroup"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnAutoScalingGroupvpczoneidentifier;
impl CfnAutoScalingGroupvpczoneidentifier {
    pub fn att_name(&self) -> &'static str {
        r#"VPCZoneIdentifier"#
    }
}

impl cfn_resources::CfnResource for CfnAutoScalingGroup {
    fn type_string(&self) -> &'static str {
        "AWS::AutoScaling::AutoScalingGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.launch_template
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.mixed_instances_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// AcceleratorCountRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum number of accelerators for an instance type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AcceleratorCountRequest {
    ///
    /// The maximum value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The minimum value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for AcceleratorCountRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// AcceleratorTotalMemoryMiBRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum total memory size for the accelerators for an instance type,    in MiB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AcceleratorTotalMemoryMiBRequest {
    ///
    /// The memory maximum in MiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The memory minimum in MiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for AcceleratorTotalMemoryMiBRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// BaselineEbsBandwidthMbpsRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum baseline bandwidth performance for an instance type, in    Mbps.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BaselineEbsBandwidthMbpsRequest {
    ///
    /// The maximum value in Mbps.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The minimum value in Mbps.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for BaselineEbsBandwidthMbpsRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// The attributes for the instance types for a mixed instances policy. Amazon EC2 Auto    Scaling uses your specified requirements to identify instance types. Then, it uses your    On-Demand and Spot allocation strategies to launch instances from these instance types.
///
/// When you specify multiple attributes, you get instance types that satisfy all of the    specified attributes. If you specify multiple values for an attribute, you get instance types    that satisfy any of the specified values.
///
/// To limit the list of instance types from which Amazon EC2 Auto Scaling can identify    matching instance types, you can use one of the following parameters, but not both in the same    request:
///
/// For an example template, see Auto scaling template     snippets.
///
/// For more information, see Creating an Auto     Scaling group using attribute-based instance type selection in the Amazon     EC2 Auto Scaling User Guide. For help determining which instance types match your    attributes before you apply them to your Auto Scaling group, see Preview instance types with specified attributes in the Amazon EC2 User     Guide for Linux Instances.
///
/// InstanceRequirements is a property of the     LaunchTemplateOverrides property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplate property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceRequirements {
    ///
    /// The minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia       chips) for an instance type.
    ///
    /// To exclude accelerator-enabled instance types, set Max to       0.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: AcceleratorCountRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_count: Option<AcceleratorCountRequest>,

    ///
    /// Indicates whether instance types must have accelerators by specific       manufacturers.
    ///
    /// For instance types with NVIDIA devices, specify nvidia.               For instance types with AMD devices, specify amd.               For instance types with AWS devices, specify             amazon-web-services.               For instance types with Xilinx devices, specify xilinx.
    ///
    /// Default: Any manufacturer
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorManufacturers")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_manufacturers: Option<Vec<String>>,

    ///
    /// Lists the accelerators that must be on an instance type.
    ///
    /// For instance types with NVIDIA A100 GPUs, specify a100.               For instance types with NVIDIA V100 GPUs, specify v100.               For instance types with NVIDIA K80 GPUs, specify k80.               For instance types with NVIDIA T4 GPUs, specify t4.               For instance types with NVIDIA M60 GPUs, specify m60.               For instance types with AMD Radeon Pro V520 GPUs, specify             radeon-pro-v520.               For instance types with Xilinx VU9P FPGAs, specify vu9p.
    ///
    /// Default: Any accelerator
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorNames")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_names: Option<Vec<String>>,

    ///
    /// The minimum and maximum total memory size for the accelerators on an instance type, in       MiB.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: AcceleratorTotalMemoryMiBRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,

    ///
    /// Lists the accelerator types that must be on an instance type.
    ///
    /// For instance types with GPU accelerators, specify gpu.               For instance types with FPGA accelerators, specify fpga.               For instance types with inference accelerators, specify           inference.
    ///
    /// Default: Any accelerator type
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_types: Option<Vec<String>>,

    ///
    /// The instance types to apply your specified attributes against. All other instance       types are ignored, even if they match your specified attributes.
    ///
    /// You can use strings with one or more wild cards, represented by an asterisk         (*), to allow an instance type, size, or generation. The following are       examples: m5.8xlarge, c5*.*, m5a.*,         r*, *3*.
    ///
    /// For example, if you specify c5*, Amazon EC2 Auto Scaling will allow the entire C5       instance family, which includes all C5a and C5n instance types. If you specify         m5a.*, Amazon EC2 Auto Scaling will allow all the M5a instance types, but not the M5n       instance types.
    ///
    /// NoteIf you specify AllowedInstanceTypes, you can't specify           ExcludedInstanceTypes.
    ///
    /// Default: All instance types
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 400
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedInstanceTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub allowed_instance_types: Option<Vec<String>>,

    ///
    /// Indicates whether bare metal instance types are included, excluded, or       required.
    ///
    /// Default: excluded
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "BareMetal")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bare_metal: Option<InstanceRequirementsBareMetalEnum>,

    ///
    /// The minimum and maximum baseline bandwidth performance for an instance type, in Mbps.       For more information, see Amazon EBS–optimized instances       in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: BaselineEbsBandwidthMbpsRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,

    ///
    /// Indicates whether burstable performance instance types are included, excluded, or       required. For more information, see Burstable         performance instances in the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Default: excluded
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "BurstablePerformance")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub burstable_performance: Option<InstanceRequirementsBurstablePerformanceEnum>,

    ///
    /// Lists which specific CPU manufacturers to include.
    ///
    /// For instance types with Intel CPUs, specify intel.               For instance types with AMD CPUs, specify amd.               For instance types with AWS CPUs, specify           amazon-web-services.
    ///
    /// NoteDon't confuse the CPU hardware manufacturer with the CPU hardware architecture.         Instances will be launched with a compatible CPU architecture based on the Amazon         Machine Image (AMI) that you specify in your launch template.
    ///
    /// Default: Any manufacturer
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CpuManufacturers")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub cpu_manufacturers: Option<Vec<String>>,

    ///
    /// The instance types to exclude. You can use strings with one or more wild cards,       represented by an asterisk (*), to exclude an instance family, type, size,       or generation. The following are examples: m5.8xlarge, c5*.*,         m5a.*, r*, *3*.
    ///
    /// For example, if you specify c5*, you are excluding the entire C5 instance       family, which includes all C5a and C5n instance types. If you specify       m5a.*, Amazon EC2 Auto Scaling will exclude all the M5a instance types, but not the M5n       instance types.
    ///
    /// NoteIf you specify ExcludedInstanceTypes, you can't specify           AllowedInstanceTypes.
    ///
    /// Default: No excluded instance types
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 400
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedInstanceTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub excluded_instance_types: Option<Vec<String>>,

    ///
    /// Indicates whether current or previous generation instance types are included.
    ///
    /// For current generation instance types, specify current. The           current generation includes EC2 instance types currently recommended for use.           This typically includes the latest two to three generations in each instance           family. For more information, see Instance types in           the Amazon EC2 User Guide for Linux Instances.               For previous generation instance types, specify previous.
    ///
    /// Default: Any current or previous generation
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceGenerations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_generations: Option<Vec<String>>,

    ///
    /// Indicates whether instance types with instance store volumes are included, excluded,       or required. For more information, see Amazon EC2 instance store in       the Amazon EC2 User Guide for Linux Instances.
    ///
    /// Default: included
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: excluded | included | required
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalStorage")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub local_storage: Option<InstanceRequirementsLocalStorageEnum>,

    ///
    /// Indicates the type of local storage that is required.
    ///
    /// For instance types with hard disk drive (HDD) storage, specify             hdd.               For instance types with solid state drive (SSD) storage, specify             ssd.
    ///
    /// Default: Any local storage type
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalStorageTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub local_storage_types: Option<Vec<String>>,

    ///
    /// The minimum and maximum amount of memory per vCPU for an instance type, in GiB.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: MemoryGiBPerVCpuRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryGiBPerVCpu")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpuRequest>,

    ///
    /// The minimum and maximum instance memory size for an instance type, in MiB.
    ///
    /// Required: No
    ///
    /// Type: MemoryMiBRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "MemoryMiB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub memory_mi_b: Option<MemoryMiBRequest>,

    ///
    /// The minimum and maximum amount of network bandwidth, in gigabits per second       (Gbps).
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: NetworkBandwidthGbpsRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkBandwidthGbps")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,

    ///
    /// The minimum and maximum number of network interfaces for an instance type.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: NetworkInterfaceCountRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,

    ///
    /// The price protection threshold for On-Demand Instances. This is the maximum you’ll pay       for an On-Demand Instance, expressed as a percentage higher than the least expensive       current generation M, C, or R instance type with your specified attributes. When       Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types       whose price is higher than your threshold. The parameter accepts an integer, which       Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value,       such as 999999.
    ///
    /// If you set DesiredCapacityType to vcpu or         memory-mib, the price protection threshold is applied based on the per       vCPU or per memory price instead of the per instance price.
    ///
    /// Default: 20
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i64>,

    ///
    /// Indicates whether instance types must provide On-Demand Instance hibernation       support.
    ///
    /// Default: false
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequireHibernateSupport")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub require_hibernate_support: Option<bool>,

    ///
    /// The price protection threshold for Spot Instances. This is the maximum you’ll pay for       a Spot Instance, expressed as a percentage higher than the least expensive current       generation M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling       selects instance types with your attributes, we will exclude instance types whose price       is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling       interprets as a percentage. To turn off price protection, specify a high value, such as         999999.
    ///
    /// If you set DesiredCapacityType to vcpu or         memory-mib, the price protection threshold is applied based on the per       vCPU or per memory price instead of the per instance price.
    ///
    /// Default: 100
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub spot_max_price_percentage_over_lowest_price: Option<i64>,

    ///
    /// The minimum and maximum total local storage size for an instance type, in GB.
    ///
    /// Default: No minimum or maximum limits
    ///
    /// Required: No
    ///
    /// Type: TotalLocalStorageGBRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "TotalLocalStorageGB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub total_local_storage_gb: Option<TotalLocalStorageGBRequest>,

    ///
    /// The minimum and maximum number of vCPUs for an instance type.
    ///
    /// Required: No
    ///
    /// Type: VCpuCountRequest
    ///
    /// Update requires: No interruption
    #[serde(rename = "VCpuCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub vcpu_count: Option<VCpuCountRequest>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceRequirementsBareMetalEnum {
    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,
}

impl Default for InstanceRequirementsBareMetalEnum {
    fn default() -> Self {
        InstanceRequirementsBareMetalEnum::Excluded
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceRequirementsBurstablePerformanceEnum {
    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,
}

impl Default for InstanceRequirementsBurstablePerformanceEnum {
    fn default() -> Self {
        InstanceRequirementsBurstablePerformanceEnum::Excluded
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum InstanceRequirementsLocalStorageEnum {
    /// excluded
    #[serde(rename = "excluded")]
    Excluded,

    /// included
    #[serde(rename = "included")]
    Included,

    /// required
    #[serde(rename = "required")]
    Required,
}

impl Default for InstanceRequirementsLocalStorageEnum {
    fn default() -> Self {
        InstanceRequirementsLocalStorageEnum::Excluded
    }
}

impl cfn_resources::CfnResource for InstanceRequirements {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.accelerator_count
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.accelerator_total_memory_mi_b
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.allowed_instance_types {
            if the_val.len() > 400 as _ {
                return Err(format!("Max validation failed on field 'allowed_instance_types'. {} is greater than 400", the_val.len()));
            }
        }

        self.baseline_ebs_bandwidth_mbps
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.excluded_instance_types {
            if the_val.len() > 400 as _ {
                return Err(format!("Max validation failed on field 'excluded_instance_types'. {} is greater than 400", the_val.len()));
            }
        }

        self.memory_gi_bper_vcpu
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.memory_mi_b
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_bandwidth_gbps
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.network_interface_count
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.on_demand_max_price_percentage_over_lowest_price {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'on_demand_max_price_percentage_over_lowest_price'. {} is less than 0", the_val));
            }
        }

        if let Some(the_val) = &self.spot_max_price_percentage_over_lowest_price {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'spot_max_price_percentage_over_lowest_price'. {} is less than 0", the_val));
            }
        }

        self.total_local_storage_gb
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.vcpu_count
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use this structure to specify the distribution of On-Demand Instances and Spot Instances    and the allocation strategies used to fulfill On-Demand and Spot capacities for a mixed    instances policy.
///
/// For more information, see Auto Scaling     groups with multiple instance types and purchase options in the Amazon EC2     Auto Scaling User Guide.
///
/// InstancesDistribution is a property of the AWS::AutoScaling::AutoScalingGroup MixedInstancesPolicy property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstancesDistribution {
    ///
    /// The allocation strategy to apply to your On-Demand Instances when they are launched.    Possible instance types are determined by the launch template overrides that you    specify.
    ///
    /// The following lists the valid values:
    ///
    /// lowest-price          Uses price to determine which instance types are the highest priority, launching the       lowest priced instance types within an Availability Zone first. This is the default       value for Auto Scaling groups that specify InstanceRequirements.              prioritized          You set the order of instance types for the launch template overrides from highest       to lowest priority (from first to last in the list). Amazon EC2 Auto Scaling launches       your highest priority instance types first. If all your On-Demand capacity cannot be       fulfilled using your highest priority instance type, then Amazon EC2 Auto Scaling       launches the remaining capacity using the second priority instance type, and so on. This       is the default value for Auto Scaling groups that don't specify        InstanceRequirements and cannot be used for groups that do.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnDemandAllocationStrategy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_demand_allocation_strategy: Option<cfn_resources::StrVal>,

    ///
    /// The minimum amount of the Auto Scaling group's capacity that must be fulfilled by On-Demand       Instances. This base portion is launched first as your group scales.
    ///
    /// This number has the same unit of measurement as the group's desired capacity. If you       change the default unit of measurement (number of instances) by specifying weighted       capacity values in your launch template overrides list, or by changing the default       desired capacity type setting of the group, you must specify this number using the same       unit of measurement.
    ///
    /// Default: 0
    ///
    /// NoteAn update to this setting means a gradual replacement of instances to adjust the         current On-Demand Instance levels. When replacing instances, Amazon EC2 Auto Scaling launches new         instances before terminating the previous ones.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "OnDemandBaseCapacity")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_demand_base_capacity: Option<i64>,

    ///
    /// Controls the percentages of On-Demand Instances and Spot Instances for your additional       capacity beyond OnDemandBaseCapacity. Expressed as a number (for example,       20 specifies 20% On-Demand Instances, 80% Spot Instances). If set to 100, only On-Demand       Instances are used.
    ///
    /// Default: 100
    ///
    /// NoteAn update to this setting means a gradual replacement of instances to adjust the         current On-Demand and Spot Instance levels for your additional capacity higher than         the base capacity. When replacing instances, Amazon EC2 Auto Scaling launches new instances before         terminating the previous ones.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "OnDemandPercentageAboveBaseCapacity")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub on_demand_percentage_above_base_capacity: Option<i64>,

    ///
    /// The allocation strategy to apply to your Spot Instances when they are launched. Possible    instance types are determined by the launch template overrides that you specify.
    ///
    /// The following lists the valid values:
    ///
    /// capacity-optimized          Requests Spot Instances using pools that are optimally chosen based on the available       Spot capacity. This strategy has the lowest risk of interruption. To give certain       instance types a higher chance of launching first, use        capacity-optimized-prioritized.             capacity-optimized-prioritized          You set the order of instance types for the launch template overrides from highest       to lowest priority (from first to last in the list). Amazon EC2 Auto Scaling honors the       instance type priorities on a best effort basis but optimizes for capacity first. Note       that if the On-Demand allocation strategy is set to prioritized, the same       priority is applied when fulfilling On-Demand capacity. This is not a valid value for       Auto Scaling groups that specify InstanceRequirements.             lowest-price          Requests Spot Instances using the lowest priced pools within an Availability Zone,       across the number of Spot pools that you specify for the SpotInstancePools       property. To ensure that your desired capacity is met, you might receive Spot Instances       from several pools. This is the default value, but it might lead to high interruption       rates because this strategy only considers instance price and not available       capacity.             price-capacity-optimized (recommended)          The price and capacity optimized allocation strategy looks at both price and       capacity to select the Spot Instance pools that are the least likely to be interrupted       and have the lowest possible price.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotAllocationStrategy")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub spot_allocation_strategy: Option<cfn_resources::StrVal>,

    ///
    /// The number of Spot Instance pools across which to allocate your Spot Instances. The       Spot pools are determined from the different instance types in the overrides. Valid only       when the SpotAllocationStrategy is lowest-price. Value must be       in the range of 1–20.
    ///
    /// Default: 2
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotInstancePools")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub spot_instance_pools: Option<i64>,

    ///
    /// The maximum price per unit hour that you are willing to pay for a Spot Instance. If       your maximum price is lower than the Spot price for the instance types that you       selected, your Spot Instances are not launched. We do not recommend specifying a maximum       price because it can lead to increased interruptions. When Spot Instances launch, you       pay the current Spot price. To remove a maximum price that you previously set, include       the property but specify an empty string ("") for the value.
    ///
    /// ImportantIf you specify a maximum price, your instances will be interrupted more frequently         than if you do not specify one.
    ///
    /// Valid Range: Minimum value of 0.001
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SpotMaxPrice")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub spot_max_price: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InstancesDistribution {
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

/// Use this structure to specify the launch templates and instance types (overrides) for a    mixed instances policy.
///
/// LaunchTemplate is a property of the AWS::AutoScaling::AutoScalingGroup MixedInstancesPolicy property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LaunchTemplate {
    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplate.html#cfn-as-group-launchtemplate
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: LaunchTemplateSpecification,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplate.html#cfn-as-mixedinstancespolicy-overrides
    #[serde(rename = "Overrides")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub overrides: Option<Vec<LaunchTemplateOverrides>>,
}

impl cfn_resources::CfnResource for LaunchTemplate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.launch_template_specification.validate()?;

        Ok(())
    }
}

/// Use this structure to let Amazon EC2 Auto Scaling do the following when the Auto Scaling    group has a mixed instances policy:
///
/// Specify the instance types that you want, or define your instance requirements instead and    let Amazon EC2 Auto Scaling provision the available instance types that meet your    requirements. This can provide Amazon EC2 Auto Scaling with a larger selection of instance    types to choose from when fulfilling Spot and On-Demand capacities. You can view which    instance types are matched before you apply the instance requirements to your Auto Scaling    group.
///
/// After you define your instance requirements, you don't have to keep updating these    settings to get new EC2 instance types automatically. Amazon EC2 Auto Scaling uses the    instance requirements of the Auto Scaling group to determine whether a new EC2 instance type    can be used.
///
/// LaunchTemplateOverrides is a property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplate property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LaunchTemplateOverrides {
    ///
    /// The instance requirements. Amazon EC2 Auto Scaling uses your specified requirements to identify       instance types. Then, it uses your On-Demand and Spot allocation strategies to launch       instances from these instance types.
    ///
    /// You can specify up to four separate sets of instance requirements per Auto Scaling group. This       is useful for provisioning instances from different Amazon Machine Images (AMIs) in the       same Auto Scaling group. To do this, create the AMIs and create a new launch template for each       AMI. Then, create a compatible set of instance requirements for each launch template.
    ///
    /// NoteIf you specify InstanceRequirements, you can't specify           InstanceType.
    ///
    /// Required: No
    ///
    /// Type: InstanceRequirements
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceRequirements")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_requirements: Option<InstanceRequirements>,

    ///
    /// The instance type, such as m3.xlarge. You must specify an instance type       that is supported in your requested Region and Availability Zones. For more information,       see Instance types in the Amazon Elastic Compute Cloud User         Guide.
    ///
    /// You can specify up to 40 instance types per Auto Scaling group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_type: Option<cfn_resources::StrVal>,

    ///
    /// Provides a launch template for the specified instance type or set of instance       requirements. For example, some instance types might require a launch template with a       different AMI. If not provided, Amazon EC2 Auto Scaling uses the launch template that's specified in       the LaunchTemplate definition. For more information, see Specifying a different launch template for an instance type in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// You can specify up to 20 launch templates per Auto Scaling group. The launch templates       specified in the overrides and in the LaunchTemplate definition count       towards this limit.
    ///
    /// Required: No
    ///
    /// Type: LaunchTemplateSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateSpecification")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub launch_template_specification: Option<LaunchTemplateSpecification>,

    ///
    /// If you provide a list of instance types to use, you can specify the number of capacity    units provided by each instance type in terms of virtual CPUs, memory, storage, throughput, or    other relative performance characteristic. When a Spot or On-Demand Instance is launched, the    capacity units count toward the desired capacity. Amazon EC2 Auto Scaling launches instances    until the desired capacity is totally fulfilled, even if this results in an overage. For    example, if there are two units remaining to fulfill capacity, and Amazon EC2 Auto Scaling can    only launch an instance with a WeightedCapacity of five units, the instance is    launched, and the desired capacity is exceeded by three units. For more information, see     Configure instance weighting for Amazon EC2 Auto Scaling in the Amazon     EC2 Auto Scaling User Guide. Value must be in the range of 1-999.
    ///
    /// If you specify a value for WeightedCapacity for one instance type, you must    specify a value for WeightedCapacity for all of them.
    ///
    /// ImportantEvery Auto Scaling group has three size parameters (DesiredCapacity,      MaxSize, and MinSize). Usually, you set these sizes based on a     specific number of instances. However, if you configure a mixed instances policy that     defines weights for the instance types, you must specify these sizes with the same units     that you use for weighting instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightedCapacity")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub weighted_capacity: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LaunchTemplateOverrides {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.instance_requirements
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.launch_template_specification
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies a launch template to use when provisioning EC2 instances for an Auto Scaling    group.
///
/// You must specify the following:
///
/// LaunchTemplateSpecification is property of the AWS::AutoScaling::AutoScalingGroup resource. It is also a property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplate and AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property types.
///
/// For information about creating a launch template, see AWS::EC2::LaunchTemplate and Create a launch template for an     Auto Scaling group in the Amazon EC2 Auto Scaling User    Guide.
///
/// For examples of launch templates, see Auto scaling template     snippets and the Examples section in the AWS::EC2::LaunchTemplate resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LaunchTemplateSpecification {
    ///
    /// The ID of the launch template.
    ///
    /// You must specify the LaunchTemplateID or the LaunchTemplateName,    but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub launch_template_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the launch template.
    ///
    /// You must specify the LaunchTemplateName or the LaunchTemplateID,    but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplateName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub launch_template_name: Option<cfn_resources::StrVal>,

    ///
    /// The version number of the launch template.
    ///
    /// Specifying $Latest or $Default for the template version number    is not supported. However, you can specify LatestVersionNumber or     DefaultVersionNumber using the Fn::GetAtt intrinsic function. For    more information, see Fn::GetAtt.
    ///
    /// NoteFor an example of using the Fn::GetAtt function, see the Examples section of the AWS::AutoScaling::AutoScalingGroup     resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LaunchTemplateSpecification {
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

/// LifecycleHookSpecification specifies a lifecycle hook for the     LifecycleHookSpecificationList property of the AWS::AutoScaling::AutoScalingGroup resource. A lifecycle hook specifies actions to    perform when Amazon EC2 Auto Scaling launches or terminates instances.
///
/// For more information, see Amazon EC2 Auto Scaling lifecycle     hooks in the Amazon EC2 Auto Scaling User Guide. You can find a    sample template snippet in the Examples section of the AWS::AutoScaling::LifecycleHook    resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecycleHookSpecification {
    ///
    /// The action the Auto Scaling group takes when the lifecycle hook timeout elapses or if an       unexpected failure occurs. The default value is ABANDON.
    ///
    /// Valid values: CONTINUE | ABANDON
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResult")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub default_result: Option<cfn_resources::StrVal>,

    ///
    /// The maximum time, in seconds, that can elapse before the lifecycle hook times out. The       range is from 30 to 7200 seconds. The default value is         3600 seconds (1 hour).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeartbeatTimeout")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub heartbeat_timeout: Option<i64>,

    ///
    /// The name of the lifecycle hook.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [A-Za-z0-9\-_\/]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleHookName")]
    pub lifecycle_hook_name: cfn_resources::StrVal,

    ///
    /// The lifecycle transition. For Auto Scaling groups, there are two major lifecycle       transitions.
    ///
    /// To create a lifecycle hook for scale-out events, specify             autoscaling:EC2_INSTANCE_LAUNCHING.               To create a lifecycle hook for scale-in events, specify             autoscaling:EC2_INSTANCE_TERMINATING.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleTransition")]
    pub lifecycle_transition: cfn_resources::StrVal,

    ///
    /// Additional information that you want to include any time Amazon EC2 Auto Scaling sends a message to       the notification target.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1023
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationMetadata")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notification_metadata: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) of the notification target that Amazon EC2 Auto Scaling sends       notifications to when an instance is in a wait state for the lifecycle hook. You can       specify an Amazon SNS topic or an Amazon SQS queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTargetARN")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notification_target_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM role that allows the Auto Scaling group to publish to the specified       notification target. For information about creating this role, see Configure a notification target for a lifecycle hook in the         Amazon EC2 Auto Scaling User Guide.
    ///
    /// Valid only if the notification target is an Amazon SNS topic or an Amazon SQS queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub role_arn: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LifecycleHookSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.lifecycle_hook_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'lifecycle_hook_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.lifecycle_hook_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'lifecycle_hook_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.notification_metadata {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1023 as _ {
                    return Err(format!("Max validation failed on field 'notification_metadata'. {} is greater than 1023", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.notification_metadata {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'notification_metadata'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// MemoryGiBPerVCpuRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum amount of memory per vCPU for an instance type, in    GiB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MemoryGiBPerVCpuRequest {
    ///
    /// The memory maximum in GiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The memory minimum in GiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for MemoryGiBPerVCpuRequest {
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

/// MemoryMiBRequest is a property of the InstanceRequirements    property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum instance memory size for an instance type, in MiB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MemoryMiBRequest {
    ///
    /// The memory maximum in MiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The memory minimum in MiB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for MemoryMiBRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// MetricsCollection is a property of the AWS::AutoScaling::AutoScalingGroup resource that describes the group metrics that    an Amazon EC2 Auto Scaling group sends to Amazon CloudWatch. These metrics describe the group    rather than any of its instances.
///
/// For more information, see Monitor CloudWatch metrics for     your Auto Scaling groups and instances in the Amazon EC2 Auto Scaling User     Guide. You can find a sample template snippet in the Examples section of the AWS::AutoScaling::AutoScalingGroup    resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MetricsCollection {
    ///
    /// The frequency at which Amazon EC2 Auto Scaling sends aggregated data to CloudWatch. The only valid value is         1Minute.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Granularity")]
    pub granularity: cfn_resources::StrVal,

    ///
    /// Identifies the metrics to enable.
    ///
    /// You can specify one or more of the following metrics:
    ///
    /// GroupMinSize                                GroupMaxSize                                GroupDesiredCapacity                                GroupInServiceInstances                                GroupPendingInstances                                GroupStandbyInstances                                GroupTerminatingInstances                                GroupTotalInstances                                GroupInServiceCapacity                                GroupPendingCapacity                                GroupStandbyCapacity                                GroupTerminatingCapacity                                GroupTotalCapacity                                WarmPoolDesiredCapacity                                WarmPoolWarmedCapacity                                WarmPoolPendingCapacity                                WarmPoolTerminatingCapacity                                WarmPoolTotalCapacity                                GroupAndWarmPoolDesiredCapacity                                GroupAndWarmPoolTotalCapacity
    ///
    /// If you specify Granularity and don't specify any metrics, all metrics are       enabled.
    ///
    /// For more information, see Auto Scaling group metrics in the Amazon EC2 Auto Scaling User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metrics")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub metrics: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for MetricsCollection {
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

/// Use this structure to launch multiple instance types and On-Demand Instances and Spot    Instances within a single Auto Scaling group.
///
/// A mixed instances policy contains information that Amazon EC2 Auto Scaling can use to    launch instances and help optimize your costs. For more information, see Auto Scaling     groups with multiple instance types and purchase options in the Amazon EC2     Auto Scaling User Guide.
///
/// You can create a mixed instances policy for new and existing Auto Scaling groups. You must    use a launch template to configure the policy. You cannot use a launch configuration.
///
/// There are key differences between Spot Instances and On-Demand Instances:
///
/// When a Spot Instance is terminated, Amazon EC2 Auto Scaling group attempts to launch a    replacement instance to maintain the desired capacity for the group.
///
/// MixedInstancesPolicy is a property of the AWS::AutoScaling::AutoScalingGroup resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MixedInstancesPolicy {
    ///
    /// The instances distribution.
    ///
    /// Required: No
    ///
    /// Type: InstancesDistribution
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstancesDistribution")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instances_distribution: Option<InstancesDistribution>,

    ///
    /// One or more launch templates and the instance types (overrides) that are used to       launch EC2 instances to fulfill On-Demand and Spot capacities.
    ///
    /// Required: Yes
    ///
    /// Type: LaunchTemplate
    ///
    /// Update requires: No interruption
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: LaunchTemplate,
}

impl cfn_resources::CfnResource for MixedInstancesPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.instances_distribution
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.launch_template.validate()?;

        Ok(())
    }
}

/// NetworkBandwidthGbpsRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum network bandwidth for an instance type, in Gbps.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkBandwidthGbpsRequest {
    ///
    /// The maximum amount of network bandwidth, in gigabits per second (Gbps).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<f64>,

    ///
    /// The minimum amount of network bandwidth, in gigabits per second (Gbps).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<f64>,
}

impl cfn_resources::CfnResource for NetworkBandwidthGbpsRequest {
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

/// NetworkInterfaceCountRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum number of network interfaces for an instance type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkInterfaceCountRequest {
    ///
    /// The maximum number of network interfaces.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The minimum number of network interfaces.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for NetworkInterfaceCountRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// A structure that specifies an Amazon SNS notification configuration for the     NotificationConfigurations property of the AWS::AutoScaling::AutoScalingGroup resource.
///
/// For an example template snippet, see Auto scaling template     snippets.
///
/// For more information, see Get Amazon SNS notifications     when your Auto Scaling group scales in the Amazon EC2 Auto Scaling User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NotificationConfiguration {
    ///
    /// A list of event types that send a notification. Event types can include any of the    following types.
    ///
    /// Allowed values:
    ///
    /// autoscaling:EC2_INSTANCE_LAUNCH                  autoscaling:EC2_INSTANCE_LAUNCH_ERROR                  autoscaling:EC2_INSTANCE_TERMINATE                  autoscaling:EC2_INSTANCE_TERMINATE_ERROR                  autoscaling:TEST_NOTIFICATION
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notification_types: Option<Vec<String>>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TopicARN")]
    pub topic_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for NotificationConfiguration {
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

/// A structure that specifies a tag for the Tags property of AWS::AutoScaling::AutoScalingGroup resource.
///
/// For more information, see Tag Auto Scaling groups and     instances in the Amazon EC2 Auto Scaling User Guide. You can    find a sample template snippet in the Examples section of the AWS::AutoScaling::AutoScalingGroup    resource.
///
/// CloudFormation adds the following tags to all Auto Scaling groups and associated    instances:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TagProperty {
    ///
    /// The tag key.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// Set to true if you want CloudFormation to copy the tag to EC2 instances that    are launched as part of the Auto Scaling group. Set to false if you want the tag    attached only to the Auto Scaling group and not copied to any instances launched as part of    the Auto Scaling group.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagateAtLaunch")]
    pub propagate_at_launch: bool,

    ///
    /// The tag value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for TagProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'key'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'value'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// TotalLocalStorageGBRequest is a property of the     InstanceRequirements property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum total local storage size for an instance type, in GB.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TotalLocalStorageGBRequest {
    ///
    /// The storage maximum in GB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The storage minimum in GB.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for TotalLocalStorageGBRequest {
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

/// VCpuCountRequest is a property of the InstanceRequirements    property of the AWS::AutoScaling::AutoScalingGroup LaunchTemplateOverrides property type that    describes the minimum and maximum number of vCPUs for an instance type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VCpuCountRequest {
    ///
    /// The maximum number of vCPUs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max: Option<i64>,

    ///
    /// The minimum number of vCPUs.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub min: Option<i64>,
}

impl cfn_resources::CfnResource for VCpuCountRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.max {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'max'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.min {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'min'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}
