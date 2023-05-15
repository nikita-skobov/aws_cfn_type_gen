
pub mod cfn_scalable_target {

#[derive(serde::Serialize, Default)]
pub struct CfnScalableTarget {
    /// No documentation provided by AWS
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    /// No documentation provided by AWS
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: usize,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SuspendedState")]
    pub suspended_state: Option<SuspendedState>,
    /// List of ScheduledAction
    #[serde(rename = "ScheduledActions")]
    pub scheduled_actions: Option<Vec<ScheduledAction>>,
    /// No documentation provided by AWS
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    /// No documentation provided by AWS
    #[serde(rename = "MinCapacity")]
    pub min_capacity: usize,

}


#[derive(serde::Serialize, Default)]
pub struct ScheduledAction {
    #[serde(rename = "ScalableTargetAction")]
    pub scalable_target_action: Option<ScalableTargetAction>,
    #[serde(rename = "ScheduledActionName")]
    pub scheduled_action_name: String,
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    #[serde(rename = "Schedule")]
    pub schedule: String,
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SuspendedState {
    #[serde(rename = "ScheduledScalingSuspended")]
    pub scheduled_scaling_suspended: Option<bool>,
    #[serde(rename = "DynamicScalingOutSuspended")]
    pub dynamic_scaling_out_suspended: Option<bool>,
    #[serde(rename = "DynamicScalingInSuspended")]
    pub dynamic_scaling_in_suspended: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ScalableTargetAction {
    #[serde(rename = "MinCapacity")]
    pub min_capacity: Option<usize>,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: Option<usize>,

}


}

pub mod cfn_scaling_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnScalingPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ScalingTargetId")]
    pub scaling_target_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "StepScalingPolicyConfiguration")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: Option<TargetTrackingScalingPolicyConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct StepScalingPolicyConfiguration {
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,
    #[serde(rename = "Cooldown")]
    pub cooldown: Option<usize>,
    #[serde(rename = "MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: Option<usize>,
    #[serde(rename = "MetricAggregationType")]
    pub metric_aggregation_type: Option<String>,
    #[serde(rename = "StepAdjustments")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetTrackingScalingPolicyConfiguration {
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<usize>,
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "CustomizedMetricSpecification")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    #[serde(rename = "PredefinedMetricSpecification")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PredefinedMetricSpecification {
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomizedMetricSpecification {
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Statistic")]
    pub statistic: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct StepAdjustment {
    #[serde(rename = "MetricIntervalUpperBound")]
    pub metric_interval_upper_bound: Option<f64>,
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: usize,
    #[serde(rename = "MetricIntervalLowerBound")]
    pub metric_interval_lower_bound: Option<f64>,

}


}
