/// Creates a new capacity provider. Capacity providers are associated with an Amazon ECS 			cluster and are used in capacity provider strategies to facilitate cluster auto 			scaling.
///
/// Only capacity providers that use an Auto Scaling group can be created. Amazon ECS tasks on 			AWS Fargate use the FARGATE and FARGATE_SPOT capacity providers. 			These providers are available to all accounts in the AWS Regions that AWS Fargate 			supports.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnCapacityProvider {
    ///
    /// The Auto Scaling group settings for the capacity provider.
    ///
    /// Required: Yes
    ///
    /// Type: AutoScalingGroupProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingGroupProvider")]
    pub auto_scaling_group_provider: AutoScalingGroupProvider,

    ///
    /// The name of the capacity provider. If a name is specified, it cannot start with aws,   ecs, or fargate. If no name is specified, a default name in the   CFNStackName-CFNResourceName-RandomString format is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// The metadata that you apply to the capacity provider to help you categorize and 			organize it. Each tag consists of a key and an optional value. You define both.
    ///
    /// The following basic restrictions apply to tags:
    ///
    /// Maximum number of tags per resource - 50               For each resource, each tag key must be unique, and each tag key can have only           one value.               Maximum key length - 128 Unicode characters in UTF-8               Maximum value length - 256 Unicode characters in UTF-8               If your tagging schema is used across multiple services and resources,           remember that other services may have restrictions on allowed characters.           Generally allowed characters are: letters, numbers, and spaces representable in           UTF-8, and the following characters: + - = . _ : / @.               Tag keys and values are case-sensitive.               Do not use aws:, AWS:, or any upper or lowercase           combination of such as a prefix for either keys or values as it is reserved for           AWS use. You cannot edit or delete tag keys or values with this prefix. Tags with           this prefix do not count against your tags per resource limit.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnCapacityProvider {
    fn type_string(&self) -> &'static str {
        "AWS::ECS::CapacityProvider"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.auto_scaling_group_provider.validate()?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The details of the Auto Scaling group for the capacity provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AutoScalingGroupProvider {
    ///
    /// The Amazon Resource Name (ARN) that identifies the Auto Scaling group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingGroupArn")]
    pub auto_scaling_group_arn: String,

    ///
    /// The managed scaling settings for the Auto Scaling group capacity provider.
    ///
    /// Required: No
    ///
    /// Type: ManagedScaling
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedScaling")]
    pub managed_scaling: Option<ManagedScaling>,

    ///
    /// The managed termination protection setting to use for the Auto Scaling group capacity 			provider. This determines whether the Auto Scaling group has managed termination 			protection. The default is off.
    ///
    /// ImportantWhen using managed termination protection, managed scaling must also be used 				otherwise managed termination protection doesn't work.
    ///
    /// When managed termination protection is on, Amazon ECS prevents the Amazon EC2 instances in an Auto 			Scaling group that contain tasks from being terminated during a scale-in action. The 			Auto Scaling group and each instance in the Auto Scaling group must have instance 			protection from scale-in actions on as well. For more information, see Instance Protection in the         AWS Auto Scaling User Guide.
    ///
    /// When managed termination protection is off, your Amazon EC2 instances aren't protected from 			termination when the Auto Scaling group scales in.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "ManagedTerminationProtection")]
    pub managed_termination_protection:
        Option<AutoScalingGroupProviderManagedTerminationProtectionEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AutoScalingGroupProviderManagedTerminationProtectionEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for AutoScalingGroupProviderManagedTerminationProtectionEnum {
    fn default() -> Self {
        AutoScalingGroupProviderManagedTerminationProtectionEnum::Disabled
    }
}

impl cfn_resources::CfnResource for AutoScalingGroupProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.managed_scaling
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The managed scaling settings for the Auto Scaling group capacity provider.
///
/// When managed scaling is turned on, Amazon ECS manages the scale-in and scale-out actions of 			the Auto Scaling group. Amazon ECS manages a target tracking scaling policy using an Amazon ECS 			managed CloudWatch metric with the specified targetCapacity value as the target 			value for the metric. For more information, see Using managed scaling in the Amazon Elastic Container Service Developer Guide.
///
/// If managed scaling is off, the user must manage the scaling of the Auto Scaling 			group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ManagedScaling {
    ///
    /// The period of time, in seconds, after a newly launched Amazon EC2 instance can contribute 			to CloudWatch metrics for Auto Scaling group. If this parameter is omitted, the default value 			of 300 seconds is used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceWarmupPeriod")]
    pub instance_warmup_period: Option<i64>,

    ///
    /// The maximum number of Amazon EC2 instances that Amazon ECS will scale out at one time. The scale 			in process is not affected by this parameter. If this parameter is omitted, the default 			value of 1 is used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumScalingStepSize")]
    pub maximum_scaling_step_size: Option<i64>,

    ///
    /// The minimum number of Amazon EC2 instances that Amazon ECS will scale out at one time. The scale 			in process is not affected by this parameter If this parameter is omitted, the default 			value of 1 is used.
    ///
    /// When additional capacity is required, Amazon ECS will scale up the minimum scaling step 			size even if the actual demand is less than the minimum scaling step size.
    ///
    /// If you use a capacity provider with an Auto Scaling group configured with more than 			one Amazon EC2 instance type or Availability Zone, Amazon ECS will scale up by the exact minimum 			scaling step size value and will ignore both the maximum scaling step size as well as 			the capacity demand.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10000
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumScalingStepSize")]
    pub minimum_scaling_step_size: Option<i64>,

    ///
    /// Determines whether to use managed scaling for the capacity provider.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<ManagedScalingStatusEnum>,

    ///
    /// The target capacity utilization as a percentage for the capacity provider. The 			specified value must be greater than 0 and less than or equal to 				100. For example, if you want the capacity provider to maintain 10% 			spare capacity, then that means the utilization is 90%, so use a 				targetCapacity of 90. The default value of 				100 percent results in the Amazon EC2 instances in your Auto Scaling group being 			completely used.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetCapacity")]
    pub target_capacity: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ManagedScalingStatusEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for ManagedScalingStatusEnum {
    fn default() -> Self {
        ManagedScalingStatusEnum::Disabled
    }
}

impl cfn_resources::CfnResource for ManagedScaling {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.instance_warmup_period {
            if *the_val > 10000 as _ {
                return Err(format!("Max validation failed on field 'instance_warmup_period'. {} is greater than 10000", the_val));
            }
        }

        if let Some(the_val) = &self.instance_warmup_period {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'instance_warmup_period'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.maximum_scaling_step_size {
            if *the_val > 10000 as _ {
                return Err(format!("Max validation failed on field 'maximum_scaling_step_size'. {} is greater than 10000", the_val));
            }
        }

        if let Some(the_val) = &self.maximum_scaling_step_size {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'maximum_scaling_step_size'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.minimum_scaling_step_size {
            if *the_val > 10000 as _ {
                return Err(format!("Max validation failed on field 'minimum_scaling_step_size'. {} is greater than 10000", the_val));
            }
        }

        if let Some(the_val) = &self.minimum_scaling_step_size {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'minimum_scaling_step_size'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.target_capacity {
            if *the_val > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'target_capacity'. {} is greater than 100",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.target_capacity {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'target_capacity'. {} is less than 1",
                    the_val
                ));
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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
