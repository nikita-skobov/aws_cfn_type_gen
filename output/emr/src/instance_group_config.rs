

/// Use InstanceGroupConfig to define instance groups for an EMR cluster. A cluster can not use both instance groups and instance fleets. For more information, see Create a Cluster with Instance Fleets or Uniform Instance Groups in the Amazon EMR Management Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnInstanceGroupConfig {


    /// 
    /// The role of the instance group in the cluster.
    /// 
    /// Allowed Values: TASK
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceRole")]
    pub instance_role: String,


    /// 
    /// The ID of an Amazon EMR cluster that you want to associate this instance group with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobFlowId")]
    pub job_flow_id: String,


    /// 
    /// NoteAmazon EMR releases 4.x or later.
    /// 
    /// The list of configurations supplied for an EMR cluster instance group. You can specify a     separate configuration for each instance group (master, core, and task).
    /// 
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,


    /// 
    /// Friendly name given to the instance group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The custom AMI ID to use for the provisioned instance group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomAmiId")]
    pub custom_ami_id: Option<String>,


    /// 
    /// AutoScalingPolicy is a subproperty of InstanceGroupConfig. AutoScalingPolicy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. For more information, see Using Automatic Scaling in Amazon EMR in the Amazon EMR Management Guide.
    /// 
    /// Required: No
    ///
    /// Type: AutoScalingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoScalingPolicy")]
    pub auto_scaling_policy: Option<AutoScalingPolicy>,


    /// 
    /// Market type of the EC2 instances used to create a cluster node.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: Replacement
    #[serde(rename = "Market")]
    pub market: Option<String>,


    /// 
    /// EbsConfiguration determines the EBS volumes to attach to EMR cluster instances.
    /// 
    /// Required: No
    ///
    /// Type: EbsConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EbsConfiguration")]
    pub ebs_configuration: Option<EbsConfiguration>,


    /// 
    /// Target number of instances for the instance group.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,


    /// 
    /// If specified, indicates that the instance group uses Spot Instances. This is the maximum price you are willing to pay for Spot Instances. Specify OnDemandPrice to set the amount equal to the On-Demand price, or specify an amount in USD.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "BidPrice")]
    pub bid_price: Option<String>,


    /// 
    /// The EC2 instance type for all instances in the instance group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

}


/// SimpleScalingPolicyConfiguration is a subproperty of the ScalingAction property type. SimpleScalingPolicyConfiguration determines how an automatic scaling action adds or removes instances, the cooldown period, and the number of EC2 instances that are added each time the CloudWatch metric alarm condition is satisfied.
#[derive(Default, serde::Serialize)]
pub struct SimpleScalingPolicyConfiguration {


    /// 
    /// The way in which EC2 instances are added (if ScalingAdjustment is a     positive number) or terminated (if ScalingAdjustment is a negative number)     each time the scaling activity is triggered. CHANGE_IN_CAPACITY is the     default. CHANGE_IN_CAPACITY indicates that the EC2 instance count increments     or decrements by ScalingAdjustment, which should be expressed as an integer.       PERCENT_CHANGE_IN_CAPACITY indicates the instance count increments or     decrements by the percentage specified by ScalingAdjustment, which should be     expressed as an integer. For example, 20 indicates an increase in 20% increments of cluster     capacity. EXACT_CAPACITY indicates the scaling activity results in an instance     group with the number of EC2 instances specified by ScalingAdjustment, which     should be expressed as a positive integer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CHANGE_IN_CAPACITY | EXACT_CAPACITY | PERCENT_CHANGE_IN_CAPACITY
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,


    /// 
    /// The amount by which to scale in or scale out, based on the specified       AdjustmentType. A positive value adds to the instance group's EC2 instance     count while a negative number removes instances. If AdjustmentType is set to       EXACT_CAPACITY, the number should only be a positive integer. If       AdjustmentType is set to PERCENT_CHANGE_IN_CAPACITY, the value     should express the percentage as an integer. For example, -20 indicates a decrease in 20%     increments of cluster capacity.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,


    /// 
    /// The amount of time, in seconds, after a scaling activity completes before any further     trigger-related scaling activities can start. The default value is 0.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoolDown")]
    pub cool_down: Option<i64>,

}


/// Configuration of requested EBS block device associated with the instance group with     count of volumes that are associated to every instance.
#[derive(Default, serde::Serialize)]
pub struct EbsBlockDeviceConfig {


    /// 
    /// Number of EBS volumes with a specific volume configuration that are associated with     every instance in the instance group
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumesPerInstance")]
    pub volumes_per_instance: Option<i64>,


    /// 
    /// EBS volume specifications such as volume type, IOPS, size (GiB) and throughput (MiB/s)     that are requested for the EBS volume attached to an EC2 instance in the cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: VolumeSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSpecification")]
    pub volume_specification: VolumeSpecification,

}


/// The Amazon EBS configuration of a cluster instance.
#[derive(Default, serde::Serialize)]
pub struct EbsConfiguration {


    /// 
    /// Indicates whether an Amazon EBS volume is EBS-optimized.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,


    /// 
    /// An array of Amazon EBS volume specifications attached to a cluster     instance.
    /// 
    /// Required: No
    ///
    /// Type: List of EbsBlockDeviceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "EbsBlockDeviceConfigs")]
    pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig>>,

}


/// VolumeSpecification is a subproperty of the EbsBlockDeviceConfig property type. VolumeSecification determines the volume type, IOPS, and size (GiB) for EBS volumes attached to EC2 instances.
#[derive(Default, serde::Serialize)]
pub struct VolumeSpecification {


    /// 
    /// The volume type. Volume types supported are gp3, gp2, io1, st1, sc1, and     standard.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeType")]
    pub volume_type: String,


    /// 
    /// The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume     type is EBS-optimized, the minimum value is 10.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SizeInGB")]
    pub size_in_gb: i64,


    /// 
    /// The number of I/O operations per second (IOPS) that the volume supports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,

}


/// ScalingRule is a subproperty of the AutoScalingPolicy property type. ScalingRule defines the scale-in or scale-out rules for scaling activity, including the CloudWatch metric alarm that triggers activity, how EC2 instances are added or removed, and the periodicity of adjustments. The automatic scaling policy for an instance group can comprise one or more automatic scaling rules.
#[derive(Default, serde::Serialize)]
pub struct ScalingRule {


    /// 
    /// The conditions that trigger an automatic scaling activity.
    /// 
    /// Required: Yes
    ///
    /// Type: ScalingAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: ScalingAction,


    /// 
    /// The name used to identify an automatic scaling rule. Rule names must be unique within a     scaling policy.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The CloudWatch alarm definition that determines when automatic scaling activity is     triggered.
    /// 
    /// Required: Yes
    ///
    /// Type: ScalingTrigger
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trigger")]
    pub trigger: ScalingTrigger,


    /// 
    /// A friendly, more verbose description of the automatic scaling rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


/// ScalingAction is a subproperty of the ScalingRule property type. ScalingAction determines the type of adjustment the automatic scaling activity makes when triggered, and the periodicity of the adjustment.
#[derive(Default, serde::Serialize)]
pub struct ScalingAction {


    /// 
    /// The type of adjustment the automatic scaling activity makes when triggered, and the     periodicity of the adjustment.
    /// 
    /// Required: Yes
    ///
    /// Type: SimpleScalingPolicyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "SimpleScalingPolicyConfiguration")]
    pub simple_scaling_policy_configuration: SimpleScalingPolicyConfiguration,


    /// 
    /// Not available for instance groups. Instance groups use the market type specified for the     group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ON_DEMAND | SPOT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Market")]
    pub market: Option<String>,

}


/// MetricDimension is a subproperty of the CloudWatchAlarmDefinition property type. MetricDimension specifies a CloudWatch dimension, which is specified with a Key Value pair. The key is known as a Name in CloudWatch. By default, Amazon EMR uses one dimension whose Key is JobFlowID and Value is a variable representing the cluster ID, which is ${emr.clusterId}. This enables the automatic scaling rule for EMR to bootstrap when the cluster ID becomes available during cluster creation.
#[derive(Default, serde::Serialize)]
pub struct MetricDimension {


    /// 
    /// The dimension name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The dimension value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


/// CloudWatchAlarmDefinition is a subproperty of the ScalingTrigger property, which determines when to trigger an automatic scaling activity. Scaling activity begins when you satisfy the defined alarm conditions.
#[derive(Default, serde::Serialize)]
pub struct CloudWatchAlarmDefinition {


    /// 
    /// Determines how the metric specified by MetricName is compared to the value     specified by Threshold.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: GREATER_THAN | GREATER_THAN_OR_EQUAL | LESS_THAN | LESS_THAN_OR_EQUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: String,


    /// 
    /// The statistic to apply to the metric associated with the alarm. The default is       AVERAGE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AVERAGE | MAXIMUM | MINIMUM | SAMPLE_COUNT | SUM
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,


    /// 
    /// The value against which the specified statistic is compared.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Threshold")]
    pub threshold: f64,


    /// 
    /// The number of periods, in five-minute increments, during which the alarm condition must     exist before the alarm triggers automatic scaling activity. The default value is       1.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: Option<i64>,


    /// 
    /// The period, in seconds, over which the statistic is applied. EMR CloudWatch metrics are     emitted every five minutes (300 seconds), so if an EMR CloudWatch metric is specified,     specify 300.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Period")]
    pub period: i64,


    /// 
    /// A CloudWatch metric dimension.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,


    /// 
    /// The namespace for the CloudWatch metric. The default is       AWS/ElasticMapReduce.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,


    /// 
    /// The name of the CloudWatch metric that is watched to determine an alarm     condition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The unit of measure associated with the CloudWatch metric being watched. The value     specified for Unit must correspond to the units specified in the CloudWatch     metric.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BITS | BITS_PER_SECOND | BYTES | BYTES_PER_SECOND | COUNT | COUNT_PER_SECOND | GIGA_BITS | GIGA_BITS_PER_SECOND | GIGA_BYTES | GIGA_BYTES_PER_SECOND | KILO_BITS | KILO_BITS_PER_SECOND | KILO_BYTES | KILO_BYTES_PER_SECOND | MEGA_BITS | MEGA_BITS_PER_SECOND | MEGA_BYTES | MEGA_BYTES_PER_SECOND | MICRO_SECONDS | MILLI_SECONDS | NONE | PERCENT | SECONDS | TERA_BITS | TERA_BITS_PER_SECOND | TERA_BYTES | TERA_BYTES_PER_SECOND
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,

}


/// ScalingConstraints is a subproperty of the AutoScalingPolicy property type. ScalingConstraints defines the upper and lower EC2 instance limits for an automatic scaling policy. Automatic scaling activities triggered by automatic scaling rules will not cause an instance group to grow above or shrink below these limits.
#[derive(Default, serde::Serialize)]
pub struct ScalingConstraints {


    /// 
    /// The lower boundary of EC2 instances in an instance group below which scaling activities     are not allowed to shrink. Scale-in activities will not terminate instances below this     boundary.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,


    /// 
    /// The upper boundary of EC2 instances in an instance group beyond which scaling activities     are not allowed to grow. Scale-out activities will not add instances beyond this     boundary.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,

}


/// Configurations is a property of the AWS::EMR::Cluster resource that specifies the configuration of applications on an Amazon EMR cluster.
///
/// Configurations are optional. You can use them to have EMR customize applications and software bundled with Amazon EMR when a cluster is created. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file. For more information, see Configuring Applications.
#[derive(Default, serde::Serialize)]
pub struct Configuration {


    /// 
    /// A list of additional configurations to apply within a configuration object.
    /// 
    /// Required: No
    ///
    /// Type: List of Configuration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Configurations")]
    pub configurations: Option<Vec<Configuration>>,


    /// 
    /// Within a configuration classification, a set of properties that represent the settings that you want to change in the configuration file. Duplicates not allowed.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The classification within a configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Classification")]
    pub classification: Option<String>,

}


/// ScalingTrigger is a subproperty of the ScalingRule property type. ScalingTrigger determines the conditions that trigger an automatic scaling activity.
#[derive(Default, serde::Serialize)]
pub struct ScalingTrigger {


    /// 
    /// The definition of a CloudWatch metric alarm. When the defined alarm conditions are met     along with other trigger parameters, scaling activity begins.
    /// 
    /// Required: Yes
    ///
    /// Type: CloudWatchAlarmDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchAlarmDefinition")]
    pub cloud_watch_alarm_definition: CloudWatchAlarmDefinition,

}


/// AutoScalingPolicy defines how an instance group dynamically adds and terminates EC2 instances in response to the value of a CloudWatch metric. For more information, see Using Automatic Scaling in Amazon EMR in the Amazon EMR Management Guide.
#[derive(Default, serde::Serialize)]
pub struct AutoScalingPolicy {


    /// 
    /// The scale-in and scale-out rules that comprise the automatic scaling policy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ScalingRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rules")]
    pub rules: Vec<ScalingRule>,


    /// 
    /// The upper and lower EC2 instance limits for an automatic scaling policy. Automatic     scaling activity will not cause an instance group to grow above or below these     limits.
    /// 
    /// Required: Yes
    ///
    /// Type: ScalingConstraints
    ///
    /// Update requires: No interruption
    #[serde(rename = "Constraints")]
    pub constraints: ScalingConstraints,

}
