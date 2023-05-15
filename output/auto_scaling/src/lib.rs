
pub mod cfn_auto_scaling_group {

#[derive(serde::Serialize, Default)]
pub struct CfnAutoScalingGroup {
    /// List of NotificationConfiguration
    #[serde(rename = "NotificationConfigurations")]
    pub notification_configurations: Option<Vec<NotificationConfiguration>>,
    /// No documentation provided by AWS
    #[serde(rename = "NewInstancesProtectedFromScaleIn")]
    pub new_instances_protected_from_scale_in: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetGroupARNs")]
    pub target_group_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxSize")]
    pub max_size: String,
    /// No documentation provided by AWS
    #[serde(rename = "MinSize")]
    pub min_size: String,
    /// No documentation provided by AWS
    #[serde(rename = "DesiredCapacity")]
    pub desired_capacity: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VPCZoneIdentifier")]
    pub vpczone_identifier: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultInstanceWarmup")]
    pub default_instance_warmup: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Context")]
    pub context: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceLinkedRoleARN")]
    pub service_linked_role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Cooldown")]
    pub cooldown: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckGracePeriod")]
    pub health_check_grace_period: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: Option<String>,
    /// List of MetricsCollection
    #[serde(rename = "MetricsCollection")]
    pub metrics_collection: Option<Vec<MetricsCollection>>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheckType")]
    pub health_check_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CapacityRebalance")]
    pub capacity_rebalance: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxInstanceLifetime")]
    pub max_instance_lifetime: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "TerminationPolicies")]
    pub termination_policies: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "LoadBalancerNames")]
    pub load_balancer_names: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchConfigurationName")]
    pub launch_configuration_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DesiredCapacityType")]
    pub desired_capacity_type: Option<String>,
    /// List of LifecycleHookSpecification
    #[serde(rename = "LifecycleHookSpecificationList")]
    pub lifecycle_hook_specification_list: Option<Vec<LifecycleHookSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "PlacementGroup")]
    pub placement_group: Option<String>,
    /// List of TagProperty
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<TagProperty>>,
    /// No documentation provided by AWS
    #[serde(rename = "MixedInstancesPolicy")]
    pub mixed_instances_policy: Option<MixedInstancesPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,

}


#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaceCountRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct VCpuCountRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationConfiguration {
    #[serde(rename = "NotificationTypes")]
    pub notification_types: Option<Vec<String>>,
    #[serde(rename = "TopicARN")]
    pub topic_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstancesDistribution {
    #[serde(rename = "OnDemandAllocationStrategy")]
    pub on_demand_allocation_strategy: Option<String>,
    #[serde(rename = "OnDemandBaseCapacity")]
    pub on_demand_base_capacity: Option<usize>,
    #[serde(rename = "SpotAllocationStrategy")]
    pub spot_allocation_strategy: Option<String>,
    #[serde(rename = "SpotMaxPrice")]
    pub spot_max_price: Option<String>,
    #[serde(rename = "OnDemandPercentageAboveBaseCapacity")]
    pub on_demand_percentage_above_base_capacity: Option<usize>,
    #[serde(rename = "SpotInstancePools")]
    pub spot_instance_pools: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkBandwidthGbpsRequest {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct LifecycleHookSpecification {
    #[serde(rename = "NotificationTargetARN")]
    pub notification_target_arn: Option<String>,
    #[serde(rename = "NotificationMetadata")]
    pub notification_metadata: Option<String>,
    #[serde(rename = "LifecycleTransition")]
    pub lifecycle_transition: String,
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,
    #[serde(rename = "LifecycleHookName")]
    pub lifecycle_hook_name: String,
    #[serde(rename = "HeartbeatTimeout")]
    pub heartbeat_timeout: Option<usize>,
    #[serde(rename = "DefaultResult")]
    pub default_result: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricsCollection {
    #[serde(rename = "Granularity")]
    pub granularity: String,
    #[serde(rename = "Metrics")]
    pub metrics: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryMiBRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "Version")]
    pub version: String,

}

#[derive(serde::Serialize, Default)]
pub struct MixedInstancesPolicy {
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: LaunchTemplate,
    #[serde(rename = "InstancesDistribution")]
    pub instances_distribution: Option<InstancesDistribution>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceRequirements {
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<String>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpuRequest>,
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCountRequest>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGBRequest>,
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<String>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCountRequest>,
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiBRequest>,
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct BaselineEbsBandwidthMbpsRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorTotalMemoryMiBRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorCountRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TagProperty {
    #[serde(rename = "PropagateAtLaunch")]
    pub propagate_at_launch: bool,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryGiBPerVCpuRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplate {
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<LaunchTemplateOverrides>>,
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: LaunchTemplateSpecification,

}

#[derive(serde::Serialize, Default)]
pub struct TotalLocalStorageGBRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateOverrides {
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirements>,
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: Option<LaunchTemplateSpecification>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,

}


}

pub mod cfn_launch_configuration {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunchConfiguration {
    /// The ID of the RAM disk to select.
    #[serde(rename = "RamDiskId")]
    pub ram_disk_id: Option<String>,
    /// Specifies whether the launch configuration is optimized for EBS I/O (true) or not (false).
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    /// Provides the unique ID of the Amazon Machine Image (AMI) that was assigned during registration.
    #[serde(rename = "ImageId")]
    pub image_id: String,
    /// Specifies the instance type of the EC2 instance.
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// The metadata options for the instances.
    #[serde(rename = "MetadataOptions")]
    pub metadata_options: Option<MetadataOptions>,
    /// The name of the launch configuration. This name must be unique per Region per account.
    #[serde(rename = "LaunchConfigurationName")]
    pub launch_configuration_name: Option<String>,
    /// The Base64-encoded user data to make available to the launched EC2 instances.
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,
    /// Provides the name of the EC2 key pair.
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,
    /// The ID of the Amazon EC2 instance you want to use to create the launch configuration.
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// Controls whether instances in this group are launched with detailed (true) or basic (false) monitoring.
    #[serde(rename = "InstanceMonitoring")]
    pub instance_monitoring: Option<bool>,
    /// A list that contains the security groups to assign to the instances in the Auto Scaling group.
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// The ID of a ClassicLink-enabled VPC to link your EC2-Classic instances to.
    #[serde(rename = "ClassicLinkVPCId")]
    pub classic_link_vpcid: Option<String>,
    /// The IDs of one or more security groups for the VPC that you specified in the ClassicLinkVPCId property.
    #[serde(rename = "ClassicLinkVPCSecurityGroups")]
    pub classic_link_vpcsecurity_groups: Option<Vec<String>>,
    /// Provides the name or the Amazon Resource Name (ARN) of the instance profile associated with the IAM role for the instance. The instance profile contains the IAM role.
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<String>,
    /// The tenancy of the instance, either default or dedicated.
    #[serde(rename = "PlacementTenancy")]
    pub placement_tenancy: Option<String>,
    /// The maximum hourly price you are willing to pay for any Spot Instances launched to fulfill the request.
    #[serde(rename = "SpotPrice")]
    pub spot_price: Option<String>,
    /// Specifies how block devices are exposed to the instance. You can specify virtual devices and EBS volumes.
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// For Auto Scaling groups that are running in a virtual private cloud (VPC), specifies whether to assign a public IP address to the group's instances.
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,
    /// Provides the ID of the kernel associated with the EC2 AMI.
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct MetadataOptions {
    #[serde(rename = "HttpEndpoint")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<usize>,
    #[serde(rename = "HttpTokens")]
    pub http_tokens: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BlockDevice {
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BlockDeviceMapping {
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<bool>,
    #[serde(rename = "Ebs")]
    pub ebs: Option<BlockDevice>,

}


}

pub mod cfn_lifecycle_hook {

#[derive(serde::Serialize, Default)]
pub struct CfnLifecycleHook {
    /// The action the Auto Scaling group takes when the lifecycle hook timeout elapses or if an unexpected failure occurs. The valid values are CONTINUE and ABANDON (default).
    #[serde(rename = "DefaultResult")]
    pub default_result: Option<String>,
    /// The name of the Auto Scaling group for the lifecycle hook.
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// The Amazon Resource Name (ARN) of the notification target that Amazon EC2 Auto Scaling uses to notify you when an instance is in the transition state for the lifecycle hook. You can specify an Amazon SQS queue or an Amazon SNS topic. The notification message includes the following information: lifecycle action token, user account ID, Auto Scaling group name, lifecycle hook name, instance ID, lifecycle transition, and notification metadata.
    #[serde(rename = "NotificationTargetARN")]
    pub notification_target_arn: Option<String>,
    /// The ARN of the IAM role that allows the Auto Scaling group to publish to the specified notification target, for example, an Amazon SNS topic or an Amazon SQS queue.
    #[serde(rename = "RoleARN")]
    pub role_arn: Option<String>,
    /// The maximum time, in seconds, that can elapse before the lifecycle hook times out. The range is from 30 to 7200 seconds. The default value is 3600 seconds (1 hour). If the lifecycle hook times out, Amazon EC2 Auto Scaling performs the action that you specified in the DefaultResult property.
    #[serde(rename = "HeartbeatTimeout")]
    pub heartbeat_timeout: Option<usize>,
    /// The instance state to which you want to attach the lifecycle hook.
    #[serde(rename = "LifecycleTransition")]
    pub lifecycle_transition: String,
    /// The name of the lifecycle hook.
    #[serde(rename = "LifecycleHookName")]
    pub lifecycle_hook_name: Option<String>,
    /// Additional information that is included any time Amazon EC2 Auto Scaling sends a message to the notification target.
    #[serde(rename = "NotificationMetadata")]
    pub notification_metadata: Option<String>,

}



}

pub mod cfn_scaling_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnScalingPolicy {
    /// The duration of the policy's cooldown period, in seconds. When a cooldown period is specified here, it overrides the default cooldown period defined for the Auto Scaling group.
    #[serde(rename = "Cooldown")]
    pub cooldown: Option<String>,
    /// A predictive scaling policy. Includes support for predefined metrics only.
    #[serde(rename = "PredictiveScalingConfiguration")]
    pub predictive_scaling_configuration: Option<PredictiveScalingConfiguration>,
    /// One of the following policy types: TargetTrackingScaling, StepScaling, SimpleScaling (default), PredictiveScaling
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<String>,
    /// The aggregation type for the CloudWatch metrics. The valid values are Minimum, Maximum, and Average. If the aggregation type is null, the value is treated as Average. Valid only if the policy type is StepScaling.
    #[serde(rename = "MetricAggregationType")]
    pub metric_aggregation_type: Option<String>,
    /// A set of adjustments that enable you to scale based on the size of the alarm breach. Required if the policy type is StepScaling. (Not used with any other policy type.)
    #[serde(rename = "StepAdjustments")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,
    /// The amount by which to scale, based on the specified adjustment type. A positive value adds to the current capacity while a negative number removes from the current capacity. For exact capacity, you must specify a positive value. Required if the policy type is SimpleScaling. (Not used with any other policy type.)
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: Option<usize>,
    /// The minimum value to scale by when the adjustment type is PercentChangeInCapacity. For example, suppose that you create a step scaling policy to scale out an Auto Scaling group by 25 percent and you specify a MinAdjustmentMagnitude of 2. If the group has 4 instances and the scaling policy is performed, 25 percent of 4 is 1. However, because you specified a MinAdjustmentMagnitude of 2, Amazon EC2 Auto Scaling scales out the group by 2 instances.
    #[serde(rename = "MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: Option<usize>,
    /// The name of the Auto Scaling group.
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// A target tracking scaling policy. Includes support for predefined or customized metrics.
    #[serde(rename = "TargetTrackingConfiguration")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,
    /// The estimated time, in seconds, until a newly launched instance can contribute to the CloudWatch metrics. If not provided, the default is to use the value from the default cooldown period for the Auto Scaling group. Valid only if the policy type is TargetTrackingScaling or StepScaling.
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<usize>,
    /// Specifies how the scaling adjustment is interpreted. The valid values are ChangeInCapacity, ExactCapacity, and PercentChangeInCapacity.
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingPredefinedLoadMetric {
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct TargetTrackingConfiguration {
    #[serde(rename = "CustomizedMetricSpecification")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,
    #[serde(rename = "PredefinedMetricSpecification")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingCustomizedCapacityMetric {
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingMetricSpecification {
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    pub predefined_scaling_metric_specification: Option<PredictiveScalingPredefinedScalingMetric>,
    #[serde(rename = "PredefinedMetricPairSpecification")]
    pub predefined_metric_pair_specification: Option<PredictiveScalingPredefinedMetricPair>,
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
    #[serde(rename = "CustomizedCapacityMetricSpecification")]
    pub customized_capacity_metric_specification: Option<PredictiveScalingCustomizedCapacityMetric>,
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    pub customized_scaling_metric_specification: Option<PredictiveScalingCustomizedScalingMetric>,
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    pub customized_load_metric_specification: Option<PredictiveScalingCustomizedLoadMetric>,
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    pub predefined_load_metric_specification: Option<PredictiveScalingPredefinedLoadMetric>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDimension {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricStat {
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Metric")]
    pub metric: Metric,
    #[serde(rename = "Stat")]
    pub stat: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDataQuery {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingPredefinedMetricPair {
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingCustomizedScalingMetric {
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}

#[derive(serde::Serialize, Default)]
pub struct CustomizedMetricSpecification {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Statistic")]
    pub statistic: String,

}

#[derive(serde::Serialize, Default)]
pub struct Metric {
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingCustomizedLoadMetric {
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}

#[derive(serde::Serialize, Default)]
pub struct StepAdjustment {
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: usize,
    #[serde(rename = "MetricIntervalUpperBound")]
    pub metric_interval_upper_bound: Option<f64>,
    #[serde(rename = "MetricIntervalLowerBound")]
    pub metric_interval_lower_bound: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingPredefinedScalingMetric {
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PredefinedMetricSpecification {
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct PredictiveScalingConfiguration {
    #[serde(rename = "MaxCapacityBreachBehavior")]
    pub max_capacity_breach_behavior: Option<String>,
    #[serde(rename = "MaxCapacityBuffer")]
    pub max_capacity_buffer: Option<usize>,
    #[serde(rename = "SchedulingBufferTime")]
    pub scheduling_buffer_time: Option<usize>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    #[serde(rename = "MetricSpecifications")]
    pub metric_specifications: Vec<PredictiveScalingMetricSpecification>,

}


}

pub mod cfn_scheduled_action {

#[derive(serde::Serialize, Default)]
pub struct CfnScheduledAction {
    /// The time zone for the cron expression.
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,
    /// The latest scheduled start time to return. If scheduled action names are provided, this parameter is ignored.
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,
    /// The desired capacity is the initial capacity of the Auto Scaling group after the scheduled action runs and the capacity it attempts to maintain.
    #[serde(rename = "DesiredCapacity")]
    pub desired_capacity: Option<usize>,
    /// The earliest scheduled start time to return. If scheduled action names are provided, this parameter is ignored.
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    /// The recurring schedule for the action, in Unix cron syntax format. When StartTime and EndTime are specified with Recurrence , they form the boundaries of when the recurring action starts and stops.
    #[serde(rename = "Recurrence")]
    pub recurrence: Option<String>,
    /// The minimum size of the Auto Scaling group.
    #[serde(rename = "MinSize")]
    pub min_size: Option<usize>,
    /// The name of the Auto Scaling group.
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// The minimum size of the Auto Scaling group.
    #[serde(rename = "MaxSize")]
    pub max_size: Option<usize>,

}



}

pub mod cfn_warm_pool {

#[derive(serde::Serialize, Default)]
pub struct CfnWarmPool {
    /// No documentation provided by AWS
    #[serde(rename = "AutoScalingGroupName")]
    pub auto_scaling_group_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "MaxGroupPreparedCapacity")]
    pub max_group_prepared_capacity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceReusePolicy")]
    pub instance_reuse_policy: Option<InstanceReusePolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "PoolState")]
    pub pool_state: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MinSize")]
    pub min_size: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct InstanceReusePolicy {
    #[serde(rename = "ReuseOnScaleIn")]
    pub reuse_on_scale_in: Option<bool>,

}


}
