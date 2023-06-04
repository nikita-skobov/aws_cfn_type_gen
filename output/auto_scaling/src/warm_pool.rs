/// The AWS::AutoScaling::WarmPool resource creates a pool of pre-initialized EC2    instances that sits alongside the Auto Scaling group. Whenever your application needs to scale    out, the Auto Scaling group can draw on the warm pool to meet its new desired capacity.
///
/// When you create a warm pool, you can define a minimum size. When your Auto Scaling group    scales out and the size of the warm pool shrinks, Amazon EC2 Auto Scaling launches new    instances into the warm pool to maintain its minimum size.
///
/// For more information, see Warm pools for Amazon EC2     Auto Scaling in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnWarmPool {
    ///
    /// The name of the Auto Scaling group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: cfn_resources::StrVal,

    ///
    /// Indicates whether instances in the Auto Scaling group can be returned to the warm pool on       scale in. The default is to terminate instances in the Auto Scaling group when the group scales       in.
    ///
    /// Required: No
    ///
    /// Type: InstanceReusePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceReusePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_reuse_policy: Option<InstanceReusePolicy>,

    ///
    /// Specifies the maximum number of instances that are allowed to be in the warm pool or       in any state except Terminated for the Auto Scaling group. This is an optional       property. Specify it only if you do not want the warm pool size to be determined by the       difference between the group's maximum capacity and its desired capacity.
    ///
    /// ImportantIf a value for MaxGroupPreparedCapacity is not specified, Amazon EC2 Auto Scaling         launches and maintains the difference between the group's maximum capacity and its         desired capacity. If you specify a value for MaxGroupPreparedCapacity,         Amazon EC2 Auto Scaling uses the difference between the MaxGroupPreparedCapacity and         the desired capacity instead. The size of the warm pool is dynamic. Only when           MaxGroupPreparedCapacity and MinSize are set to the         same value does the warm pool have an absolute size.
    ///
    /// If the desired capacity of the Auto Scaling group is higher than the         MaxGroupPreparedCapacity, the capacity of the warm pool is 0, unless       you specify a value for MinSize. To remove a value that you previously set,       include the property but specify -1 for the value.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: -1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxGroupPreparedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_group_prepared_capacity: Option<i64>,

    ///
    /// Specifies the minimum number of instances to maintain in the warm pool. This helps you       to ensure that there is always a certain number of warmed instances available to handle       traffic spikes. Defaults to 0 if not specified.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,

    ///
    /// Sets the instance state to transition to after the lifecycle actions are complete.       Default is Stopped.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Hibernated | Running | Stopped
    ///
    /// Update requires: No interruption
    #[serde(rename = "PoolState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_state: Option<WarmPoolPoolStateEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum WarmPoolPoolStateEnum {
    /// Hibernated
    #[serde(rename = "Hibernated")]
    Hibernated,

    /// Running
    #[serde(rename = "Running")]
    Running,

    /// Stopped
    #[serde(rename = "Stopped")]
    Stopped,
}

impl Default for WarmPoolPoolStateEnum {
    fn default() -> Self {
        WarmPoolPoolStateEnum::Hibernated
    }
}

impl cfn_resources::CfnResource for CfnWarmPool {
    fn type_string(&self) -> &'static str {
        "AWS::AutoScaling::WarmPool"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.instance_reuse_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.max_group_prepared_capacity {
            if *the_val < -1 as _ {
                return Err(format!("Min validation failed on field 'max_group_prepared_capacity'. {} is less than -1", the_val));
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

        Ok(())
    }
}

/// A structure that specifies an instance reuse policy for the     InstanceReusePolicy property of the AWS::AutoScaling::WarmPool resource.
///
/// For more information, see Warm pools for Amazon EC2     Auto Scaling in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceReusePolicy {
    ///
    /// Specifies whether instances in the Auto Scaling group can be returned to the warm pool on       scale in.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReuseOnScaleIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_on_scale_in: Option<bool>,
}

impl cfn_resources::CfnResource for InstanceReusePolicy {
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
