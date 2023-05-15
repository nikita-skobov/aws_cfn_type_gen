
pub mod cfn_scaling_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnScalingPlan {
    /// List of ScalingInstruction
    #[serde(rename = "ScalingInstructions")]
    pub scaling_instructions: Vec<ScalingInstruction>,
    /// No documentation provided by AWS
    #[serde(rename = "ApplicationSource")]
    pub application_source: ApplicationSource,

}


#[derive(serde::Serialize, Default)]
pub struct PredefinedScalingMetricSpecification {
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,
    #[serde(rename = "PredefinedScalingMetricType")]
    pub predefined_scaling_metric_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct TagFilter {
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ScalingInstruction {
    #[serde(rename = "PredictiveScalingMode")]
    pub predictive_scaling_mode: Option<String>,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "MinCapacity")]
    pub min_capacity: usize,
    #[serde(rename = "TargetTrackingConfigurations")]
    pub target_tracking_configurations: Vec<TargetTrackingConfiguration>,
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    pub customized_load_metric_specification: Option<CustomizedLoadMetricSpecification>,
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: String,
    #[serde(rename = "PredictiveScalingMaxCapacityBehavior")]
    pub predictive_scaling_max_capacity_behavior: Option<String>,
    #[serde(rename = "ScheduledActionBufferTime")]
    pub scheduled_action_buffer_time: Option<usize>,
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: String,
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    pub predefined_load_metric_specification: Option<PredefinedLoadMetricSpecification>,
    #[serde(rename = "ScalingPolicyUpdateBehavior")]
    pub scaling_policy_update_behavior: Option<String>,
    #[serde(rename = "DisableDynamicScaling")]
    pub disable_dynamic_scaling: Option<bool>,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: usize,
    #[serde(rename = "PredictiveScalingMaxCapacityBuffer")]
    pub predictive_scaling_max_capacity_buffer: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetTrackingConfiguration {
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    pub predefined_scaling_metric_specification: Option<PredefinedScalingMetricSpecification>,
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<usize>,
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<usize>,
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    pub customized_scaling_metric_specification: Option<CustomizedScalingMetricSpecification>,
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomizedScalingMetricSpecification {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "Statistic")]
    pub statistic: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: String,

}

#[derive(serde::Serialize, Default)]
pub struct CustomizedLoadMetricSpecification {
    #[serde(rename = "Statistic")]
    pub statistic: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "MetricName")]
    pub metric_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ApplicationSource {
    #[serde(rename = "CloudFormationStackARN")]
    pub cloud_formation_stack_arn: Option<String>,
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,

}

#[derive(serde::Serialize, Default)]
pub struct PredefinedLoadMetricSpecification {
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,
    #[serde(rename = "PredefinedLoadMetricType")]
    pub predefined_load_metric_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
