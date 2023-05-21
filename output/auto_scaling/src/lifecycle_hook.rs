

/// The AWS::AutoScaling::LifecycleHook resource specifies lifecycle hooks for an    Auto Scaling group. These hooks let you create solutions that are aware of events in the Auto    Scaling instance lifecycle, and then perform a custom action on instances when the    corresponding lifecycle event occurs. A lifecycle hook provides a specified amount of time    (one hour by default) to wait for the action to complete before the instance transitions to    the next state.
///
/// Use lifecycle hooks to prepare new instances for use or to delay them from being    registered behind a load balancer before their configuration has been applied completely. You    can also use lifecycle hooks to prepare running instances to be terminated by, for example,    downloading logs or other data.
///
/// For more information, see Amazon EC2 Auto Scaling lifecycle     hooks in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLifecycleHook {


    /// 
    /// The name of the Auto Scaling group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,


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
    pub default_result: Option<String>,


    /// 
    /// The maximum time, in seconds, that can elapse before the lifecycle hook times out. The       range is from 30 to 7200 seconds. The default value is         3600 seconds (1 hour).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HeartbeatTimeout")]
    pub heartbeat_timeout: Option<i64>,


    /// 
    /// The name of the lifecycle hook.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [A-Za-z0-9\-_\/]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "LifecycleHookName")]
    pub lifecycle_hook_name: Option<String>,


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
    pub lifecycle_transition: String,


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
    pub notification_metadata: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the notification target that Amazon EC2 Auto Scaling sends       notifications to when an instance is in a wait state for the lifecycle hook. You can       specify an Amazon SNS topic or an Amazon SQS queue.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTargetARN")]
    pub notification_target_arn: Option<String>,


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
    pub role_arn: Option<String>,

}



impl cfn_resources::CfnResource for CfnLifecycleHook {
    fn type_string() -> &'static str {
        "AWS::AutoScaling::LifecycleHook"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
