

/// The AWS::AutoScaling::ScalingPolicy resource specifies an Amazon EC2 Auto    Scaling scaling policy so that the Auto Scaling group can scale the number of instances    available for your application.
///
/// For more information about using scaling policies to scale your Auto Scaling group    automatically, see Dynamic scaling and     Predictive     scaling in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnScalingPolicy {


    /// 
    /// Specifies how the scaling adjustment is interpreted (for example, an absolute number       or a percentage). The valid values are ChangeInCapacity,         ExactCapacity, and PercentChangeInCapacity.
    /// 
    /// Required if the policy type is StepScaling or SimpleScaling.       For more information, see Scaling adjustment types in the Amazon EC2 Auto Scaling User Guide.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<String>,


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
    /// A cooldown period, in seconds, that applies to a specific simple scaling policy. When       a cooldown period is specified here, it overrides the default cooldown.
    /// 
    /// Valid only if the policy type is SimpleScaling. For more information, see         Scaling         cooldowns for Amazon EC2 Auto Scaling in the Amazon EC2 Auto Scaling User Guide.
    /// 
    /// Default: None
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cooldown")]
    pub cooldown: Option<String>,


    /// 
    /// Not needed if the default instance warmup is defined for the         group.
    /// 
    /// The estimated time, in seconds, until a newly launched instance can contribute to the       CloudWatch metrics. This warm-up period applies to instances launched due to a specific target       tracking or step scaling policy. When a warm-up period is specified here, it overrides       the default instance warmup.
    /// 
    /// Valid only if the policy type is TargetTrackingScaling or         StepScaling.
    /// 
    /// NoteThe default is to use the value for the default instance warmup defined for the         group. If default instance warmup is null, then EstimatedInstanceWarmup         falls back to the value of default cooldown.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<i64>,


    /// 
    /// The aggregation type for the CloudWatch metrics. The valid values are Minimum,         Maximum, and Average. If the aggregation type is null, the       value is treated as Average.
    /// 
    /// Valid only if the policy type is StepScaling.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricAggregationType")]
    pub metric_aggregation_type: Option<String>,


    /// 
    /// The minimum value to scale by when the adjustment type is         PercentChangeInCapacity. For example, suppose that you create a step       scaling policy to scale out an Auto Scaling group by 25 percent and you specify a         MinAdjustmentMagnitude of 2. If the group has 4 instances and the       scaling policy is performed, 25 percent of 4 is 1. However, because you specified a         MinAdjustmentMagnitude of 2, Amazon EC2 Auto Scaling scales out the group by 2       instances.
    /// 
    /// Valid only if the policy type is StepScaling or         SimpleScaling. For more information, see Scaling adjustment types in the Amazon EC2 Auto Scaling User       Guide.
    /// 
    /// NoteSome Auto Scaling groups use instance weights. In this case, set the           MinAdjustmentMagnitude to a value that is at least as large as your         largest instance weight.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: Option<i64>,


    /// 
    /// One of the following policy types:
    /// 
    /// TargetTrackingScaling                                StepScaling                                SimpleScaling (default)                        PredictiveScaling
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<String>,


    /// 
    /// A predictive scaling policy. Provides support for predefined and custom       metrics.
    /// 
    /// Predefined metrics include CPU utilization, network in/out, and the Application Load       Balancer request count.
    /// 
    /// Required if the policy type is PredictiveScaling.
    /// 
    /// Required: Conditional
    ///
    /// Type: PredictiveScalingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictiveScalingConfiguration")]
    pub predictive_scaling_configuration: Option<PredictiveScalingConfiguration>,


    /// 
    /// The amount by which to scale, based on the specified adjustment type. A positive value       adds to the current capacity while a negative number removes from the current capacity.       For exact capacity, you must specify a non-negative value.
    /// 
    /// Required if the policy type is SimpleScaling. (Not used with any other       policy type.)
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: Option<i64>,


    /// 
    /// A set of adjustments that enable you to scale based on the size of the alarm       breach.
    /// 
    /// Required if the policy type is StepScaling. (Not used with any other       policy type.)
    /// 
    /// Required: Conditional
    ///
    /// Type: List of StepAdjustment
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepAdjustments")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,


    /// 
    /// A target tracking scaling policy. Provides support for predefined or custom       metrics.
    /// 
    /// The following predefined metrics are available:
    /// 
    /// ASGAverageCPUUtilization                                ASGAverageNetworkIn                                ASGAverageNetworkOut                                ALBRequestCountPerTarget
    /// 
    /// If you specify ALBRequestCountPerTarget for the metric, you must specify       the ResourceLabel property with the         PredefinedMetricSpecification.
    /// 
    /// Required if the policy type is TargetTrackingScaling.
    /// 
    /// Required: Conditional
    ///
    /// Type: TargetTrackingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTrackingConfiguration")]
    pub target_tracking_configuration: Option<TargetTrackingConfiguration>,

}



impl cfn_resources::CfnResource for CfnScalingPolicy {
    fn type_string() -> &'static str {
        "AWS::AutoScaling::ScalingPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains customized metric specification information for a target tracking scaling policy    for Amazon EC2 Auto Scaling.
///
/// To create your customized metric specification:
///
/// For more information about CloudWatch, see Amazon CloudWatch     Concepts.
///
/// CustomizedMetricSpecification is a property of the AWS::AutoScaling::ScalingPolicy TargetTrackingConfiguration property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomizedMetricSpecification {


    /// 
    /// The dimensions of the metric.
    /// 
    /// Conditional: If you published your metric with dimensions, you must specify the same       dimensions in your scaling policy.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,


    /// 
    /// The name of the metric. To get the exact metric name, namespace, and dimensions,       inspect the Metric object       that is returned by a call to ListMetrics.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The namespace of the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: String,


    /// 
    /// The statistic of the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Average | Maximum | Minimum | SampleCount | Sum
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: CustomizedMetricSpecificationStatisticEnum,


    /// 
    /// The unit of the metric. For a complete list of the units that CloudWatch supports, see the         MetricDatum       data type in the Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomizedMetricSpecificationStatisticEnum {

    /// Average
    #[serde(rename = "Average")]
    Average,

    /// Maximum
    #[serde(rename = "Maximum")]
    Maximum,

    /// Minimum
    #[serde(rename = "Minimum")]
    Minimum,

    /// SampleCount
    #[serde(rename = "SampleCount")]
    Samplecount,

    /// Sum
    #[serde(rename = "Sum")]
    Sum,

}

impl Default for CustomizedMetricSpecificationStatisticEnum {
    fn default() -> Self {
        CustomizedMetricSpecificationStatisticEnum::Average
    }
}



/// Represents a specific metric.
///
/// Metric is a property of the AWS::AutoScaling::ScalingPolicy MetricStat property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Metric {


    /// 
    /// The dimensions for the metric. For the list of available dimensions, see the AWS       documentation available from the table in AWS         services that publish CloudWatch metrics in the Amazon CloudWatch User         Guide.
    /// 
    /// Conditional: If you published your metric with dimensions, you must specify the same       dimensions in your scaling policy.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,


    /// 
    /// The name of the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,


    /// 
    /// The namespace of the metric. For more information, see the table in AWS         services that publish CloudWatch metrics in the Amazon CloudWatch User         Guide.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    pub namespace: String,

}




/// The metric data to return. Also defines whether this call is returning data for one metric    only, or whether it is performing a math expression on the values of returned metric    statistics to create a new time series. A time series is a series of data points, each of    which is associated with a timestamp.
///
/// MetricDataQuery is a property of the following property types:
///
/// Predictive scaling uses the time series data received from CloudWatch to understand how to    schedule capacity based on your historical workload patterns.
///
/// You can call for a single metric or perform math expressions on multiple metrics. Any    expressions used in a metric specification must eventually return a single time series.
///
/// For more information and examples, see Advanced predictive scaling policy configurations using custom metrics in the     Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDataQuery {


    /// 
    /// The math expression to perform on the returned data, if this object is performing a       math expression. This expression can use the Id of the other metrics to       refer to those metrics, and can also use the Id of other expressions to use       the result of those expressions.
    /// 
    /// Conditional: Within each MetricDataQuery object, you must specify either         Expression or MetricStat, but not both.
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
    #[serde(rename = "Expression")]
    pub expression: Option<String>,


    /// 
    /// A short name that identifies the object's results in the response. This name must be       unique among all MetricDataQuery objects specified for a single scaling       policy. If you are performing math expressions on this set of data, this name represents       that data and can serve as a variable in the mathematical expression. The valid       characters are letters, numbers, and underscores. The first character must be a       lowercase letter.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// A human-readable label for this metric or expression. This is especially useful if       this is a math expression, so that you know what the value represents.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2047
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Label")]
    pub label: Option<String>,


    /// 
    /// Information about the metric data to return.
    /// 
    /// Conditional: Within each MetricDataQuery object, you must specify either         Expression or MetricStat, but not both.
    /// 
    /// Required: No
    ///
    /// Type: MetricStat
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<MetricStat>,


    /// 
    /// Indicates whether to return the timestamps and raw data values of this metric.
    /// 
    /// If you use any math expressions, specify true for this value for only the       final math expression that the metric specification is based on. You must specify         false for ReturnData for all the other metrics and       expressions used in the metric specification.
    /// 
    /// If you are only retrieving metrics and not performing any math expressions, do not       specify anything for ReturnData. This sets it to its default         (true).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,

}




/// MetricDimension specifies a name/value pair that is part of the identity of a    CloudWatch metric for the Dimensions property of the AWS::AutoScaling::ScalingPolicy CustomizedMetricSpecification property type.    Duplicate dimensions are not allowed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDimension {


    /// 
    /// The name of the dimension.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The value of the dimension.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}




/// MetricStat is a property of the AWS::AutoScaling::ScalingPolicy MetricDataQuery property type.
///
/// This structure defines the CloudWatch metric to return, along with the statistic, period,    and unit.
///
/// For more information about the CloudWatch terminology below, see Amazon CloudWatch concepts in the Amazon CloudWatch User    Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricStat {


    /// 
    /// The CloudWatch metric to return, including the metric name, namespace, and dimensions. To       get the exact metric name, namespace, and dimensions, inspect the Metric object that is returned by a call to ListMetrics.
    /// 
    /// Required: Yes
    ///
    /// Type: Metric
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metric")]
    pub metric: Metric,


    /// 
    /// The statistic to return. It can include any CloudWatch statistic or extended statistic. For       a list of valid values, see the table in Statistics in the Amazon CloudWatch User Guide.
    /// 
    /// The most commonly used metrics for predictive scaling are Average and         Sum.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stat")]
    pub stat: String,


    /// 
    /// The unit to use for the returned data points. For a complete list of the units that       CloudWatch supports, see the MetricDatum       data type in the Amazon CloudWatch API Reference.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: Option<String>,

}




/// Contains predefined metric specification information for a target tracking scaling policy    for Amazon EC2 Auto Scaling.
///
/// PredefinedMetricSpecification is a property of the AWS::AutoScaling::ScalingPolicy TargetTrackingConfiguration property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredefinedMetricSpecification {


    /// 
    /// The metric type. The following predefined metrics are available:
    /// 
    /// ASGAverageCPUUtilization - Average CPU utilization of the Auto Scaling           group.                        ASGAverageNetworkIn - Average number of bytes received on all           network interfaces by the Auto Scaling group.                        ASGAverageNetworkOut - Average number of bytes sent out on all           network interfaces by the Auto Scaling group.                        ALBRequestCountPerTarget - Average Application Load Balancer request count per target           for your Auto Scaling group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBRequestCountPerTarget | ASGAverageCPUUtilization | ASGAverageNetworkIn | ASGAverageNetworkOut
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: PredefinedMetricSpecificationPredefinedMetricTypeEnum,


    /// 
    /// A label that uniquely identifies a specific Application Load Balancer target group       from which to determine the average request count served by your Auto Scaling group. You can't       specify a resource label unless the target group is attached to the Auto Scaling group.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN       and the final portion of the target group ARN into a single value, separated by a forward       slash (/). The format of the resource label is:
    /// 
    /// app/my-alb/778d41231b141a0f/targetgroup/my-alb-target-group/943f017f100becff.
    /// 
    /// Where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of           the load balancer ARN               targetgroup/<target-group-name>/<target-group-id> is the final portion           of the target group ARN.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use       the DescribeTargetGroups API operation.
    /// 
    /// Required: Conditional
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
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredefinedMetricSpecificationPredefinedMetricTypeEnum {

    /// ALBRequestCountPerTarget
    #[serde(rename = "ALBRequestCountPerTarget")]
    Albrequestcountpertarget,

    /// ASGAverageCPUUtilization
    #[serde(rename = "ASGAverageCPUUtilization")]
    Asgaveragecpuutilization,

    /// ASGAverageNetworkIn
    #[serde(rename = "ASGAverageNetworkIn")]
    Asgaveragenetworkin,

    /// ASGAverageNetworkOut
    #[serde(rename = "ASGAverageNetworkOut")]
    Asgaveragenetworkout,

}

impl Default for PredefinedMetricSpecificationPredefinedMetricTypeEnum {
    fn default() -> Self {
        PredefinedMetricSpecificationPredefinedMetricTypeEnum::Albrequestcountpertarget
    }
}



/// PredictiveScalingConfiguration is a property of the AWS::AutoScaling::ScalingPolicy resource that specifies a predictive scaling policy    for Amazon EC2 Auto Scaling.
///
/// For more information, see Predictive     scaling in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingConfiguration {


    /// 
    /// Defines the behavior that should be applied if the forecast capacity approaches or       exceeds the maximum capacity of the Auto Scaling group. Defaults to         HonorMaxCapacity if not specified.
    /// 
    /// The following are possible values:
    /// 
    /// HonorMaxCapacity - Amazon EC2 Auto Scaling cannot scale out capacity higher than           the maximum capacity. The maximum capacity is enforced as a hard limit.                         IncreaseMaxCapacity - Amazon EC2 Auto Scaling can scale out capacity higher than           the maximum capacity when the forecast capacity is close to or exceeds the           maximum capacity. The upper limit is determined by the forecasted capacity and           the value for MaxCapacityBuffer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HonorMaxCapacity | IncreaseMaxCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacityBreachBehavior")]
    pub max_capacity_breach_behavior: Option<PredictiveScalingConfigurationMaxCapacityBreachBehaviorEnum>,


    /// 
    /// The size of the capacity buffer to use when the forecast capacity is close to or       exceeds the maximum capacity. The value is specified as a percentage relative to the       forecast capacity. For example, if the buffer is 10, this means a 10 percent buffer,       such that if the forecast capacity is 50, and the maximum capacity is 40, then the       effective maximum capacity is 55.
    /// 
    /// If set to 0, Amazon EC2 Auto Scaling may scale capacity higher than the maximum capacity to equal but       not exceed forecast capacity.
    /// 
    /// Required if the MaxCapacityBreachBehavior property is set to         IncreaseMaxCapacity, and cannot be used otherwise.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacityBuffer")]
    pub max_capacity_buffer: Option<i64>,


    /// 
    /// This structure includes the metrics and target utilization to use for predictive       scaling.
    /// 
    /// This is an array, but we currently only support a single metric specification. That       is, you can specify a target value and a single metric pair, or a target value and one       scaling metric and one load metric.
    /// 
    /// Required: Yes
    ///
    /// Type: List of PredictiveScalingMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricSpecifications")]
    pub metric_specifications: Vec<PredictiveScalingMetricSpecification>,


    /// 
    /// The predictive scaling mode. Defaults to ForecastOnly if not       specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ForecastAndScale | ForecastOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: Option<PredictiveScalingConfigurationModeEnum>,


    /// 
    /// The amount of time, in seconds, by which the instance launch time can be advanced. For       example, the forecast says to add capacity at 10:00 AM, and you choose to pre-launch       instances by 5 minutes. In that case, the instances will be launched at 9:55 AM. The       intention is to give resources time to be provisioned. It can take a few minutes to       launch an EC2 instance. The actual amount of time required depends on several factors,       such as the size of the instance and whether there are startup scripts to complete.
    /// 
    /// The value must be less than the forecast interval duration of 3600 seconds (60       minutes). Defaults to 300 seconds if not specified.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "SchedulingBufferTime")]
    pub scheduling_buffer_time: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredictiveScalingConfigurationMaxCapacityBreachBehaviorEnum {

    /// HonorMaxCapacity
    #[serde(rename = "HonorMaxCapacity")]
    Honormaxcapacity,

    /// IncreaseMaxCapacity
    #[serde(rename = "IncreaseMaxCapacity")]
    Increasemaxcapacity,

}

impl Default for PredictiveScalingConfigurationMaxCapacityBreachBehaviorEnum {
    fn default() -> Self {
        PredictiveScalingConfigurationMaxCapacityBreachBehaviorEnum::Honormaxcapacity
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PredictiveScalingConfigurationModeEnum {

    /// ForecastAndScale
    #[serde(rename = "ForecastAndScale")]
    Forecastandscale,

    /// ForecastOnly
    #[serde(rename = "ForecastOnly")]
    Forecastonly,

}

impl Default for PredictiveScalingConfigurationModeEnum {
    fn default() -> Self {
        PredictiveScalingConfigurationModeEnum::Forecastandscale
    }
}



/// Contains capacity metric information for the     CustomizedCapacityMetricSpecification property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingCustomizedCapacityMetric {


    /// 
    /// One or more metric data queries to provide the data points for a capacity metric. Use       multiple metric data queries only if you are performing a math expression on returned       data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricDataQuery
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}




/// Contains load metric information for the CustomizedLoadMetricSpecification    property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingCustomizedLoadMetric {


    /// 
    /// One or more metric data queries to provide the data points for a load metric. Use       multiple metric data queries only if you are performing a math expression on returned       data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricDataQuery
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}




/// Contains scaling metric information for the     CustomizedScalingMetricSpecification property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingCustomizedScalingMetric {


    /// 
    /// One or more metric data queries to provide the data points for a scaling metric. Use       multiple metric data queries only if you are performing a math expression on returned       data.
    /// 
    /// Required: Yes
    ///
    /// Type: List of MetricDataQuery
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricDataQueries")]
    pub metric_data_queries: Vec<MetricDataQuery>,

}




/// A structure that specifies a metric specification for the     MetricSpecifications property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingConfiguration property    type.
///
/// You must specify either a metric pair, or a load metric and a scaling metric individually.    Specifying a metric pair instead of individual metrics provides a simpler way to configure    metrics for a scaling policy. You choose the metric pair, and the policy automatically knows    the correct sum and average statistics to use for the load metric and the scaling    metric.
///
/// Example
///
/// For information about using custom metrics with predictive scaling, see Advanced predictive scaling policy configurations using custom metrics in the     Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingMetricSpecification {


    /// 
    /// The customized capacity metric specification.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingCustomizedCapacityMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedCapacityMetricSpecification")]
    pub customized_capacity_metric_specification: Option<PredictiveScalingCustomizedCapacityMetric>,


    /// 
    /// The customized load metric specification.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingCustomizedLoadMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    pub customized_load_metric_specification: Option<PredictiveScalingCustomizedLoadMetric>,


    /// 
    /// The customized scaling metric specification.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingCustomizedScalingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    pub customized_scaling_metric_specification: Option<PredictiveScalingCustomizedScalingMetric>,


    /// 
    /// The predefined load metric specification.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingPredefinedLoadMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    pub predefined_load_metric_specification: Option<PredictiveScalingPredefinedLoadMetric>,


    /// 
    /// The predefined metric pair specification from which Amazon EC2 Auto Scaling determines the       appropriate scaling metric and load metric to use.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingPredefinedMetricPair
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricPairSpecification")]
    pub predefined_metric_pair_specification: Option<PredictiveScalingPredefinedMetricPair>,


    /// 
    /// The predefined scaling metric specification.
    /// 
    /// Required: No
    ///
    /// Type: PredictiveScalingPredefinedScalingMetric
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    pub predefined_scaling_metric_specification: Option<PredictiveScalingPredefinedScalingMetric>,


    /// 
    /// Specifies the target utilization.
    /// 
    /// NoteSome metrics are based on a count instead of a percentage, such as the request         count for an Application Load Balancer or the number of messages in an SQS queue. If the scaling policy         specifies one of these metrics, specify the target utilization as the optimal         average request or message count per instance during any one-minute interval.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}




/// Contains load metric information for the PredefinedLoadMetricSpecification    property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingPredefinedLoadMetric {


    /// 
    /// The metric type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBTargetGroupRequestCount | ASGTotalCPUUtilization | ASGTotalNetworkIn | ASGTotalNetworkOut
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: PredictiveScalingPredefinedLoadMetricPredefinedMetricTypeEnum,


    /// 
    /// A label that uniquely identifies a specific Application Load Balancer target group from which to determine       the request count served by your Auto Scaling group. You can't specify a resource label unless       the target group is attached to the Auto Scaling group.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN       and the final portion of the target group ARN into a single value, separated by a forward       slash (/). The format of the resource label is:
    /// 
    /// app/my-alb/778d41231b141a0f/targetgroup/my-alb-target-group/943f017f100becff.
    /// 
    /// Where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of           the load balancer ARN               targetgroup/<target-group-name>/<target-group-id> is the final portion           of the target group ARN.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use       the DescribeTargetGroups API operation.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredictiveScalingPredefinedLoadMetricPredefinedMetricTypeEnum {

    /// ALBTargetGroupRequestCount
    #[serde(rename = "ALBTargetGroupRequestCount")]
    Albtargetgrouprequestcount,

    /// ASGTotalCPUUtilization
    #[serde(rename = "ASGTotalCPUUtilization")]
    Asgtotalcpuutilization,

    /// ASGTotalNetworkIn
    #[serde(rename = "ASGTotalNetworkIn")]
    Asgtotalnetworkin,

    /// ASGTotalNetworkOut
    #[serde(rename = "ASGTotalNetworkOut")]
    Asgtotalnetworkout,

}

impl Default for PredictiveScalingPredefinedLoadMetricPredefinedMetricTypeEnum {
    fn default() -> Self {
        PredictiveScalingPredefinedLoadMetricPredefinedMetricTypeEnum::Albtargetgrouprequestcount
    }
}



/// Contains metric pair information for the PredefinedMetricPairSpecification    property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
///
/// For more information, see Predictive     scaling in the Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingPredefinedMetricPair {


    /// 
    /// Indicates which metrics to use. There are two different types of metrics for each       metric type: one is a load metric and one is a scaling metric. For example, if the       metric type is ASGCPUUtilization, the Auto Scaling group's total CPU metric is used       as the load metric, and the average CPU metric is used for the scaling metric.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBRequestCount | ASGCPUUtilization | ASGNetworkIn | ASGNetworkOut
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: PredictiveScalingPredefinedMetricPairPredefinedMetricTypeEnum,


    /// 
    /// A label that uniquely identifies a specific Application Load Balancer target group from    which to determine the total and average request count served by your Auto Scaling group. You    can't specify a resource label unless the target group is attached to the Auto Scaling    group.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN and    the final portion of the target group ARN into a single value, separated by a forward slash    (/). The format of the resource label is:
    /// 
    /// app/my-alb/778d41231b141a0f/targetgroup/my-alb-target-group/943f017f100becff.
    /// 
    /// Where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of the      load balancer ARN        targetgroup/<target-group-name>/<target-group-id> is the final portion of      the target group ARN.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use the     DescribeTargetGroups API operation.
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
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredictiveScalingPredefinedMetricPairPredefinedMetricTypeEnum {

    /// ALBRequestCount
    #[serde(rename = "ALBRequestCount")]
    Albrequestcount,

    /// ASGCPUUtilization
    #[serde(rename = "ASGCPUUtilization")]
    Asgcpuutilization,

    /// ASGNetworkIn
    #[serde(rename = "ASGNetworkIn")]
    Asgnetworkin,

    /// ASGNetworkOut
    #[serde(rename = "ASGNetworkOut")]
    Asgnetworkout,

}

impl Default for PredictiveScalingPredefinedMetricPairPredefinedMetricTypeEnum {
    fn default() -> Self {
        PredictiveScalingPredefinedMetricPairPredefinedMetricTypeEnum::Albrequestcount
    }
}



/// Contains scaling metric information for the     PredefinedScalingMetricSpecification property of the AWS::AutoScaling::ScalingPolicy PredictiveScalingMetricSpecification property    type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredictiveScalingPredefinedScalingMetric {


    /// 
    /// The metric type.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBRequestCountPerTarget | ASGAverageCPUUtilization | ASGAverageNetworkIn | ASGAverageNetworkOut
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: PredictiveScalingPredefinedScalingMetricPredefinedMetricTypeEnum,


    /// 
    /// A label that uniquely identifies a specific Application Load Balancer target group from which to determine       the average request count served by your Auto Scaling group. You can't specify a resource label       unless the target group is attached to the Auto Scaling group.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN       and the final portion of the target group ARN into a single value, separated by a forward       slash (/). The format of the resource label is:
    /// 
    /// app/my-alb/778d41231b141a0f/targetgroup/my-alb-target-group/943f017f100becff.
    /// 
    /// Where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of           the load balancer ARN               targetgroup/<target-group-name>/<target-group-id> is the final portion           of the target group ARN.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use       the DescribeTargetGroups API operation.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredictiveScalingPredefinedScalingMetricPredefinedMetricTypeEnum {

    /// ALBRequestCountPerTarget
    #[serde(rename = "ALBRequestCountPerTarget")]
    Albrequestcountpertarget,

    /// ASGAverageCPUUtilization
    #[serde(rename = "ASGAverageCPUUtilization")]
    Asgaveragecpuutilization,

    /// ASGAverageNetworkIn
    #[serde(rename = "ASGAverageNetworkIn")]
    Asgaveragenetworkin,

    /// ASGAverageNetworkOut
    #[serde(rename = "ASGAverageNetworkOut")]
    Asgaveragenetworkout,

}

impl Default for PredictiveScalingPredefinedScalingMetricPredefinedMetricTypeEnum {
    fn default() -> Self {
        PredictiveScalingPredefinedScalingMetricPredefinedMetricTypeEnum::Albrequestcountpertarget
    }
}



/// StepAdjustment specifies a step adjustment for the StepAdjustments    property of the AWS::AutoScaling::ScalingPolicy resource.
///
/// For the following examples, suppose that you have an alarm with a breach threshold of 50:
///
/// There are a few rules for the step adjustments for your step policy:
///
/// For more information, see Step     adjustments in the Amazon EC2 Auto Scaling User Guide.
///
/// You can find a sample template snippet in the Examples section of the AWS::AutoScaling::ScalingPolicy    resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepAdjustment {


    /// 
    /// The lower bound for the difference between the alarm threshold and the CloudWatch metric. If       the metric value is above the breach threshold, the lower bound is inclusive (the metric       must be greater than or equal to the threshold plus the lower bound). Otherwise, it is       exclusive (the metric must be greater than the threshold plus the lower bound). A null       value indicates negative infinity.
    /// 
    /// Required: Conditional
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricIntervalLowerBound")]
    pub metric_interval_lower_bound: Option<f64>,


    /// 
    /// The upper bound for the difference between the alarm threshold and the CloudWatch metric. If       the metric value is above the breach threshold, the upper bound is exclusive (the metric       must be less than the threshold plus the upper bound). Otherwise, it is inclusive (the       metric must be less than or equal to the threshold plus the upper bound). A null value       indicates positive infinity.
    /// 
    /// The upper bound must be greater than the lower bound.
    /// 
    /// Required: Conditional
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricIntervalUpperBound")]
    pub metric_interval_upper_bound: Option<f64>,


    /// 
    /// The amount by which to scale, based on the specified adjustment type. A positive value       adds to the current capacity while a negative number removes from the current       capacity. For exact capacity, you must specify a non-negative value.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,

}




/// TargetTrackingConfiguration is a property of the AWS::AutoScaling::ScalingPolicy resource that specifies a target tracking scaling    policy configuration for Amazon EC2 Auto Scaling.
///
/// For more information about scaling policies, see Dynamic scaling in the     Amazon EC2 Auto Scaling User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetTrackingConfiguration {


    /// 
    /// A customized metric. You must specify either a predefined metric or a customized       metric.
    /// 
    /// Required: Conditional
    ///
    /// Type: CustomizedMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedMetricSpecification")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,


    /// 
    /// Indicates whether scaling in by the target tracking scaling policy is disabled. If       scaling in is disabled, the target tracking scaling policy doesn't remove instances from       the Auto Scaling group. Otherwise, the target tracking scaling policy can remove instances from       the Auto Scaling group. The default is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,


    /// 
    /// A predefined metric. You must specify either a predefined metric or a customized       metric.
    /// 
    /// Required: Conditional
    ///
    /// Type: PredefinedMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricSpecification")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,


    /// 
    /// The target value for the metric.
    /// 
    /// NoteSome metrics are based on a count instead of a percentage, such as the request         count for an Application Load Balancer or the number of messages in an SQS queue. If the scaling policy         specifies one of these metrics, specify the target utilization as the optimal         average request or message count per instance during any one-minute interval.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}


