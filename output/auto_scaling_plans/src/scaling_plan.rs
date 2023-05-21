

/// The AWS::AutoScalingPlans::ScalingPlan resource defines an AWS Auto Scaling scaling plan. A scaling plan is used to scale application resources to     size them appropriately to ensure that enough resource is available in the application at     peak times and to reduce allocated resource during periods of low utilization. The     following resources can be added to a scaling plan:
///
/// For more information, see the AWS Auto Scaling       User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnScalingPlan {


    /// 
    /// A CloudFormation stack or a set of tags. You can create one scaling plan per application     source. The ApplicationSource property must be present to ensure     interoperability with the AWS Auto Scaling console.
    /// 
    /// Required: Yes
    ///
    /// Type: ApplicationSource
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationSource")]
    pub application_source: ApplicationSource,


    /// 
    /// The scaling instructions.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ScalingInstruction
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingInstructions")]
    pub scaling_instructions: Vec<ScalingInstruction>,

}



impl cfn_resources::CfnResource for CfnScalingPlan {
    fn type_string() -> &'static str {
        "AWS::AutoScalingPlans::ScalingPlan"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// ApplicationSource is a property of ScalingPlan that specifies the application source to use with AWS Auto Scaling (Auto Scaling Plans). You can create one scaling plan per application source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplicationSource {


    /// 
    /// The Amazon Resource Name (ARN) of a CloudFormation stack.
    /// 
    /// You must specify either a CloudFormationStackARN or     TagFilters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudFormationStackARN")]
    pub cloud_formation_stack_arn: Option<String>,


    /// 
    /// A set of tag filters (keys and values). Each tag filter specified must contain a key     with values as optional. Each scaling plan can include up to 50 keys, and each key can     include up to 20 values.
    /// 
    /// Tags are part of the syntax that you use to specify the resources you want returned when     configuring a scaling plan from the AWS Auto Scaling console. You do not need to     specify valid tag filter values when you create a scaling plan with CloudFormation. The       Key and Values properties can accept any value as long as the     combination of values is unique across scaling plans. However, if you also want to use the       AWS Auto Scaling console to edit the scaling plan, then you must specify valid     values.
    /// 
    /// You must specify either a CloudFormationStackARN or     TagFilters.
    /// 
    /// Required: No
    ///
    /// Type: List of TagFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "TagFilters")]
    pub tag_filters: Option<Vec<TagFilter>>,

}




/// CustomizedLoadMetricSpecification is a subproperty of ScalingInstruction that specifies a customized load metric for predictive     scaling to use with AWS Auto Scaling (Auto Scaling Plans).
///
/// For predictive scaling to work with a customized load metric specification, AWS Auto Scaling needs access to the Sum and Average statistics     that CloudWatch computes from metric data.
///
/// When you choose a load metric, make sure that the required Sum and       Average statistics for your metric are available in CloudWatch and that     they provide relevant data for predictive scaling. The Sum statistic must     represent the total load on the resource, and the Average statistic must     represent the average load per capacity unit of the resource. For example, there is a     metric that counts the number of requests processed by your Auto Scaling group. If the       Sum statistic represents the total request count processed by the group,     then the Average statistic for the specified metric must represent the average     request count processed by each instance of the group.
///
/// If you publish your own metrics, you can aggregate the data points at a given interval     and then publish the aggregated data points to CloudWatch. Before AWS Auto Scaling     generates the forecast, it sums up all the metric data points that occurred within each     hour to match the granularity period that is used in the forecast (60 minutes).
///
/// For information about terminology, available metrics, or how to publish new metrics, see       Amazon CloudWatch       Concepts in the Amazon CloudWatch User Guide.
///
/// After creating your scaling plan, you can use the AWS Auto Scaling console to     visualize forecasts for the specified metric. For more information, see View       Scaling Information for a Resource in the AWS Auto Scaling User       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomizedLoadMetricSpecification {


    /// 
    /// The dimensions of the metric.
    /// 
    /// Conditional: If you published your metric with dimensions, you must specify the same     dimensions in your customized load metric specification.
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
    /// Allowed Values: Sum
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Statistic")]
    pub statistic: CustomizedLoadMetricSpecificationStatisticEnum,


    /// 
    /// The unit of the metric.
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
pub enum CustomizedLoadMetricSpecificationStatisticEnum {

    /// Sum
    #[serde(rename = "Sum")]
    Sum,

}

impl Default for CustomizedLoadMetricSpecificationStatisticEnum {
    fn default() -> Self {
        CustomizedLoadMetricSpecificationStatisticEnum::Sum
    }
}



/// CustomizedScalingMetricSpecification is a subproperty of TargetTrackingConfiguration that specifies a customized scaling metric for a     target tracking configuration to use with AWS Auto Scaling (Auto Scaling Plans).
///
/// To create your customized scaling metric specification:
///
/// For information about terminology, available metrics, or how to publish new metrics, see       Amazon CloudWatch       Concepts in the Amazon CloudWatch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomizedScalingMetricSpecification {


    /// 
    /// The dimensions of the metric.
    /// 
    /// Conditional: If you published your metric with dimensions, you must specify the same     dimensions in your scaling policy.
    /// 
    /// Required: No
    ///
    /// Type: List of MetricDimension
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<MetricDimension>>,


    /// 
    /// The name of the metric. To get the exact metric name, namespace, and dimensions, inspect     the Metrics object that is returned by a call to ListMetrics.
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
    pub statistic: CustomizedScalingMetricSpecificationStatisticEnum,


    /// 
    /// The unit of the metric. For a complete list of the units that CloudWatch supports, see the       MetricDatum data     type in the Amazon CloudWatch API Reference.
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
pub enum CustomizedScalingMetricSpecificationStatisticEnum {

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

impl Default for CustomizedScalingMetricSpecificationStatisticEnum {
    fn default() -> Self {
        CustomizedScalingMetricSpecificationStatisticEnum::Average
    }
}



/// MetricDimension is a subproperty of CustomizedScalingMetricSpecification that specifies a dimension for a     customized metric to use with AWS Auto Scaling (Auto Scaling Plans).     Dimensions are arbitrary name/value pairs that can be associated with a CloudWatch metric.     Duplicate dimensions are not allowed.
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




/// PredefinedLoadMetricSpecification is a subproperty of ScalingInstruction that specifies a predefined load metric for predictive     scaling to use with AWS Auto Scaling (Auto Scaling Plans).
///
/// After creating your scaling plan, you can use the AWS Auto Scaling console to     visualize forecasts for the specified metric. For more information, see View       Scaling Information for a Resource in the AWS Auto Scaling User       Guide.
/// 
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredefinedLoadMetricSpecification {


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
    #[serde(rename = "PredefinedLoadMetricType")]
    pub predefined_load_metric_type: PredefinedLoadMetricSpecificationPredefinedLoadMetricTypeEnum,


    /// 
    /// Identifies the resource associated with the metric type. You can't specify a resource     label unless the metric type is ALBTargetGroupRequestCount and there is a     target group for an Application Load Balancer attached to the Auto Scaling group.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN     and the final portion of the target group ARN into a single value, separated by a forward     slash (/). The format is     app/<load-balancer-name>/<load-balancer-id>/targetgroup/<target-group-name>/<target-group-id>,     where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of        the load balancer ARN               targetgroup/<target-group-name>/<target-group-id> is the final portion        of the target group ARN.
    /// 
    /// This is an example:     app/EC2Co-EcsEl-1TKLTMITMM0EO/f37c06a68c1748aa/targetgroup/EC2Co-Defau-LDNM7Q3ZH1ZN/6d4ea56ca2d6a18d.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use     the DescribeTargetGroups API operation.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1023
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredefinedLoadMetricSpecificationPredefinedLoadMetricTypeEnum {

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

impl Default for PredefinedLoadMetricSpecificationPredefinedLoadMetricTypeEnum {
    fn default() -> Self {
        PredefinedLoadMetricSpecificationPredefinedLoadMetricTypeEnum::Albtargetgrouprequestcount
    }
}



/// PredefinedScalingMetricSpecification is a subproperty of TargetTrackingConfiguration that specifies a customized scaling metric for a     target tracking configuration to use with AWS Auto Scaling (Auto Scaling Plans).
/// 
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredefinedScalingMetricSpecification {


    /// 
    /// The metric type. The ALBRequestCountPerTarget metric type applies only to     Auto Scaling groups, Spot Fleet requests, and ECS services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBRequestCountPerTarget | ASGAverageCPUUtilization | ASGAverageNetworkIn | ASGAverageNetworkOut | DynamoDBReadCapacityUtilization | DynamoDBWriteCapacityUtilization | EC2SpotFleetRequestAverageCPUUtilization | EC2SpotFleetRequestAverageNetworkIn | EC2SpotFleetRequestAverageNetworkOut | ECSServiceAverageCPUUtilization | ECSServiceAverageMemoryUtilization | RDSReaderAverageCPUUtilization | RDSReaderAverageDatabaseConnections
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedScalingMetricType")]
    pub predefined_scaling_metric_type: PredefinedScalingMetricSpecificationPredefinedScalingMetricTypeEnum,


    /// 
    /// Identifies the resource associated with the metric type. You can't specify a resource     label unless the metric type is ALBRequestCountPerTarget and there is a target     group for an Application Load Balancer attached to the Auto Scaling group, Spot Fleet request, or     ECS service.
    /// 
    /// You create the resource label by appending the final portion of the load balancer ARN     and the final portion of the target group ARN into a single value, separated by a forward     slash (/). The format is     app/<load-balancer-name>/<load-balancer-id>/targetgroup/<target-group-name>/<target-group-id>,     where:
    /// 
    /// app/<load-balancer-name>/<load-balancer-id> is the final portion of        the load balancer ARN               targetgroup/<target-group-name>/<target-group-id> is the final portion        of the target group ARN.
    /// 
    /// This is an example:     app/EC2Co-EcsEl-1TKLTMITMM0EO/f37c06a68c1748aa/targetgroup/EC2Co-Defau-LDNM7Q3ZH1ZN/6d4ea56ca2d6a18d.
    /// 
    /// To find the ARN for an Application Load Balancer, use the DescribeLoadBalancers API operation. To find the ARN for the target group, use     the DescribeTargetGroups API operation.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1023
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredefinedScalingMetricSpecificationPredefinedScalingMetricTypeEnum {

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

    /// DynamoDBReadCapacityUtilization
    #[serde(rename = "DynamoDBReadCapacityUtilization")]
    Dynamodbreadcapacityutilization,

    /// DynamoDBWriteCapacityUtilization
    #[serde(rename = "DynamoDBWriteCapacityUtilization")]
    Dynamodbwritecapacityutilization,

    /// EC2SpotFleetRequestAverageCPUUtilization
    #[serde(rename = "EC2SpotFleetRequestAverageCPUUtilization")]
    Ec2spotfleetrequestaveragecpuutilization,

    /// EC2SpotFleetRequestAverageNetworkIn
    #[serde(rename = "EC2SpotFleetRequestAverageNetworkIn")]
    Ec2spotfleetrequestaveragenetworkin,

    /// EC2SpotFleetRequestAverageNetworkOut
    #[serde(rename = "EC2SpotFleetRequestAverageNetworkOut")]
    Ec2spotfleetrequestaveragenetworkout,

    /// ECSServiceAverageCPUUtilization
    #[serde(rename = "ECSServiceAverageCPUUtilization")]
    Ecsserviceaveragecpuutilization,

    /// ECSServiceAverageMemoryUtilization
    #[serde(rename = "ECSServiceAverageMemoryUtilization")]
    Ecsserviceaveragememoryutilization,

    /// RDSReaderAverageCPUUtilization
    #[serde(rename = "RDSReaderAverageCPUUtilization")]
    Rdsreaderaveragecpuutilization,

    /// RDSReaderAverageDatabaseConnections
    #[serde(rename = "RDSReaderAverageDatabaseConnections")]
    Rdsreaderaveragedatabaseconnections,

}

impl Default for PredefinedScalingMetricSpecificationPredefinedScalingMetricTypeEnum {
    fn default() -> Self {
        PredefinedScalingMetricSpecificationPredefinedScalingMetricTypeEnum::Albrequestcountpertarget
    }
}



/// ScalingInstruction is a property of ScalingPlan that specifies the scaling instruction for a scalable resource in a     scaling plan. Each scaling instruction applies to one resource.
///
/// AWS Auto Scaling creates target tracking scaling policies based on the scaling     instructions. Target tracking scaling policies adjust the capacity of your scalable     resource as required to maintain resource utilization at the target value that you     specified.
///
/// AWS Auto Scaling also configures predictive scaling for your Amazon EC2 Auto     Scaling groups using a subset of properties, including the load metric, the scaling metric,     the target value for the scaling metric, the predictive scaling mode (forecast and scale or     forecast only), and the desired behavior when the forecast capacity exceeds the maximum     capacity of the resource. With predictive scaling, AWS Auto Scaling generates     forecasts with traffic predictions for the two days ahead and schedules scaling actions     that proactively add and remove resource capacity to match the forecast.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScalingInstruction {


    /// 
    /// The customized load metric to use for predictive scaling. This property or a PredefinedLoadMetricSpecification is required when configuring     predictive scaling, and cannot be used otherwise.
    /// 
    /// Required: Conditional
    ///
    /// Type: CustomizedLoadMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedLoadMetricSpecification")]
    pub customized_load_metric_specification: Option<CustomizedLoadMetricSpecification>,


    /// 
    /// Controls whether dynamic scaling by AWS Auto Scaling is disabled. When dynamic scaling is     enabled, AWS Auto Scaling creates target tracking scaling policies based on the specified target     tracking configurations.
    /// 
    /// The default is enabled (false).
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableDynamicScaling")]
    pub disable_dynamic_scaling: Option<bool>,


    /// 
    /// The maximum capacity of the resource. The exception to this upper limit is if you     specify a non-default setting for PredictiveScalingMaxCapacityBehavior.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,


    /// 
    /// The minimum capacity of the resource.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,


    /// 
    /// The predefined load metric to use for predictive scaling. This property or a CustomizedLoadMetricSpecification is required when configuring     predictive scaling, and cannot be used otherwise.
    /// 
    /// Required: Conditional
    ///
    /// Type: PredefinedLoadMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedLoadMetricSpecification")]
    pub predefined_load_metric_specification: Option<PredefinedLoadMetricSpecification>,


    /// 
    /// Defines the behavior that should be applied if the forecast capacity approaches or     exceeds the maximum capacity specified for the resource. The default value is       SetForecastCapacityToMaxCapacity.
    /// 
    /// The following are possible values:
    /// 
    /// SetForecastCapacityToMaxCapacity - AWS Auto Scaling cannot scale        resource capacity higher than the maximum capacity. The maximum capacity is enforced        as a hard limit.                    SetMaxCapacityToForecastCapacity - AWS Auto Scaling can scale        resource capacity higher than the maximum capacity to equal but not exceed forecast        capacity.                    SetMaxCapacityAboveForecastCapacity - AWS Auto Scaling can scale        resource capacity higher than the maximum capacity by a specified buffer value. The        intention is to give the target tracking scaling policy extra capacity if unexpected        traffic occurs.
    /// 
    /// Valid only when configuring predictive scaling.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SetForecastCapacityToMaxCapacity | SetMaxCapacityAboveForecastCapacity | SetMaxCapacityToForecastCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictiveScalingMaxCapacityBehavior")]
    pub predictive_scaling_max_capacity_behavior: Option<ScalingInstructionPredictiveScalingMaxCapacityBehaviorEnum>,


    /// 
    /// The size of the capacity buffer to use when the forecast capacity is close to or exceeds     the maximum capacity. The value is specified as a percentage relative to the forecast     capacity. For example, if the buffer is 10, this means a 10 percent buffer. With a 10     percent buffer, if the forecast capacity is 50, and the maximum capacity is 40, then the     effective maximum capacity is 55.
    /// 
    /// Valid only when configuring predictive scaling. Required if PredictiveScalingMaxCapacityBehavior is set to       SetMaxCapacityAboveForecastCapacity, and cannot be used otherwise.
    /// 
    /// The range is 1-100.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictiveScalingMaxCapacityBuffer")]
    pub predictive_scaling_max_capacity_buffer: Option<i64>,


    /// 
    /// The predictive scaling mode. The default value is ForecastAndScale.     Otherwise, AWS Auto Scaling forecasts capacity but does not apply any scheduled     scaling actions based on the capacity forecast.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ForecastAndScale | ForecastOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredictiveScalingMode")]
    pub predictive_scaling_mode: Option<ScalingInstructionPredictiveScalingModeEnum>,


    /// 
    /// The ID of the resource. This string consists of the resource type and unique     identifier.
    /// 
    /// Auto Scaling group - The resource type is autoScalingGroup and the unique identifier is the        name of the Auto Scaling group. Example: autoScalingGroup/my-asg.               ECS service - The resource type is service and the unique identifier is the cluster name         and service name. Example: service/default/sample-webapp.               Spot Fleet request - The resource type is spot-fleet-request and the unique identifier is the         Spot Fleet request ID. Example: spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE.               DynamoDB table - The resource type is table and the unique identifier is the resource ID.         Example: table/my-table.               DynamoDB global secondary index - The resource type is index and the unique identifier is the resource ID.         Example: table/my-table/index/my-table-index.               Aurora DB cluster - The resource type is cluster and the unique identifier is the cluster name.        Example: cluster:my-db-cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: String,


    /// 
    /// The scalable dimension associated with the resource.
    /// 
    /// autoscaling:autoScalingGroup:DesiredCapacity - The desired capacity of an Auto Scaling group.                        ecs:service:DesiredCount - The desired task count of an ECS service.                        ec2:spot-fleet-request:TargetCapacity - The target capacity of a Spot Fleet request.                        dynamodb:table:ReadCapacityUnits - The provisioned read capacity for a DynamoDB table.                        dynamodb:table:WriteCapacityUnits - The provisioned write capacity for a DynamoDB table.                        dynamodb:index:ReadCapacityUnits - The provisioned read capacity for a DynamoDB global secondary index.                        dynamodb:index:WriteCapacityUnits - The provisioned write capacity for a DynamoDB global secondary index.                        rds:cluster:ReadReplicaCount - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: autoscaling:autoScalingGroup:DesiredCapacity | dynamodb:index:ReadCapacityUnits | dynamodb:index:WriteCapacityUnits | dynamodb:table:ReadCapacityUnits | dynamodb:table:WriteCapacityUnits | ec2:spot-fleet-request:TargetCapacity | ecs:service:DesiredCount | rds:cluster:ReadReplicaCount
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: ScalingInstructionScalableDimensionEnum,


    /// 
    /// Controls whether your scaling policies that are external to AWS Auto Scaling are     deleted and new target tracking scaling policies created. The default value is       KeepExternalPolicies.
    /// 
    /// Valid only when configuring dynamic scaling.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: KeepExternalPolicies | ReplaceExternalPolicies
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingPolicyUpdateBehavior")]
    pub scaling_policy_update_behavior: Option<ScalingInstructionScalingPolicyUpdateBehaviorEnum>,


    /// 
    /// The amount of time, in seconds, to buffer the run time of scheduled scaling actions when     scaling out. For example, if the forecast says to add capacity at 10:00 AM, and the buffer     time is 5 minutes, then the run time of the corresponding scheduled scaling action will be     9:55 AM. The intention is to give resources time to be provisioned. For example, it can     take a few minutes to launch an EC2 instance. The actual amount of time required depends on     several factors, such as the size of the instance and whether there are startup scripts to     complete.
    /// 
    /// The value must be less than the forecast interval duration of 3600 seconds (60 minutes).     The default is 300 seconds.
    /// 
    /// Valid only when configuring predictive scaling.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduledActionBufferTime")]
    pub scheduled_action_buffer_time: Option<i64>,


    /// 
    /// The namespace of the AWS service.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: autoscaling | dynamodb | ec2 | ecs | rds
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: ScalingInstructionServiceNamespaceEnum,


    /// 
    /// The target tracking configurations (up to 10). Each of these structures must specify a     unique scaling metric and a target value for the metric.
    /// 
    /// Required: Yes
    ///
    /// Type: List of TargetTrackingConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTrackingConfigurations")]
    pub target_tracking_configurations: Vec<TargetTrackingConfiguration>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingInstructionPredictiveScalingMaxCapacityBehaviorEnum {

    /// SetForecastCapacityToMaxCapacity
    #[serde(rename = "SetForecastCapacityToMaxCapacity")]
    Setforecastcapacitytomaxcapacity,

    /// SetMaxCapacityAboveForecastCapacity
    #[serde(rename = "SetMaxCapacityAboveForecastCapacity")]
    Setmaxcapacityaboveforecastcapacity,

    /// SetMaxCapacityToForecastCapacity
    #[serde(rename = "SetMaxCapacityToForecastCapacity")]
    Setmaxcapacitytoforecastcapacity,

}

impl Default for ScalingInstructionPredictiveScalingMaxCapacityBehaviorEnum {
    fn default() -> Self {
        ScalingInstructionPredictiveScalingMaxCapacityBehaviorEnum::Setforecastcapacitytomaxcapacity
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingInstructionServiceNamespaceEnum {

    /// autoscaling
    #[serde(rename = "autoscaling")]
    Autoscaling,

    /// dynamodb
    #[serde(rename = "dynamodb")]
    Dynamodb,

    /// ec2
    #[serde(rename = "ec2")]
    Ec2,

    /// ecs
    #[serde(rename = "ecs")]
    Ecs,

    /// rds
    #[serde(rename = "rds")]
    Rds,

}

impl Default for ScalingInstructionServiceNamespaceEnum {
    fn default() -> Self {
        ScalingInstructionServiceNamespaceEnum::Autoscaling
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingInstructionPredictiveScalingModeEnum {

    /// ForecastAndScale
    #[serde(rename = "ForecastAndScale")]
    Forecastandscale,

    /// ForecastOnly
    #[serde(rename = "ForecastOnly")]
    Forecastonly,

}

impl Default for ScalingInstructionPredictiveScalingModeEnum {
    fn default() -> Self {
        ScalingInstructionPredictiveScalingModeEnum::Forecastandscale
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingInstructionScalingPolicyUpdateBehaviorEnum {

    /// KeepExternalPolicies
    #[serde(rename = "KeepExternalPolicies")]
    Keepexternalpolicies,

    /// ReplaceExternalPolicies
    #[serde(rename = "ReplaceExternalPolicies")]
    Replaceexternalpolicies,

}

impl Default for ScalingInstructionScalingPolicyUpdateBehaviorEnum {
    fn default() -> Self {
        ScalingInstructionScalingPolicyUpdateBehaviorEnum::Keepexternalpolicies
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingInstructionScalableDimensionEnum {

    /// autoscaling:autoScalingGroup:DesiredCapacity
    #[serde(rename = "autoscaling:autoScalingGroup:DesiredCapacity")]
    Autoscalingautoscalinggroupdesiredcapacity,

    /// dynamodb:index:ReadCapacityUnits
    #[serde(rename = "dynamodb:index:ReadCapacityUnits")]
    Dynamodbindexreadcapacityunits,

    /// dynamodb:index:WriteCapacityUnits
    #[serde(rename = "dynamodb:index:WriteCapacityUnits")]
    Dynamodbindexwritecapacityunits,

    /// dynamodb:table:ReadCapacityUnits
    #[serde(rename = "dynamodb:table:ReadCapacityUnits")]
    Dynamodbtablereadcapacityunits,

    /// dynamodb:table:WriteCapacityUnits
    #[serde(rename = "dynamodb:table:WriteCapacityUnits")]
    Dynamodbtablewritecapacityunits,

    /// ec2:spot-fleet-request:TargetCapacity
    #[serde(rename = "ec2:spot-fleet-request:TargetCapacity")]
    Ec2spotfleetrequesttargetcapacity,

    /// ecs:service:DesiredCount
    #[serde(rename = "ecs:service:DesiredCount")]
    Ecsservicedesiredcount,

    /// rds:cluster:ReadReplicaCount
    #[serde(rename = "rds:cluster:ReadReplicaCount")]
    Rdsclusterreadreplicacount,

}

impl Default for ScalingInstructionScalableDimensionEnum {
    fn default() -> Self {
        ScalingInstructionScalableDimensionEnum::Autoscalingautoscalinggroupdesiredcapacity
    }
}



/// TagFilter is a subproperty of ApplicationSource that specifies a tag for an application source to use with     AWS Auto Scaling (Auto Scaling Plans).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFilter {


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
    pub key: String,


    /// 
    /// The tag values (0 to 20).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}




/// TargetTrackingConfiguration is a subproperty of ScalingInstruction that specifies a target tracking configuration to use with       AWS Auto Scaling (Auto Scaling Plans).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetTrackingConfiguration {


    /// 
    /// A customized metric. You can specify either a predefined metric or a customized metric.
    /// 
    /// Required: No
    ///
    /// Type: CustomizedScalingMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedScalingMetricSpecification")]
    pub customized_scaling_metric_specification: Option<CustomizedScalingMetricSpecification>,


    /// 
    /// Indicates whether scale in by the target tracking scaling policy is disabled. If the     value is true, scale in is disabled and the target tracking scaling policy     doesn't remove capacity from the scalable resource. Otherwise, scale in is enabled and the     target tracking scaling policy can remove capacity from the scalable resource.
    /// 
    /// The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,


    /// 
    /// The estimated time, in seconds, until a newly launched instance can contribute to the     CloudWatch metrics. This value is used only if the resource is an Auto Scaling group.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EstimatedInstanceWarmup")]
    pub estimated_instance_warmup: Option<i64>,


    /// 
    /// A predefined metric. You can specify either a predefined metric or a customized     metric.
    /// 
    /// Required: No
    ///
    /// Type: PredefinedScalingMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedScalingMetricSpecification")]
    pub predefined_scaling_metric_specification: Option<PredefinedScalingMetricSpecification>,


    /// 
    /// The amount of time, in seconds, after a scale-in activity completes before another scale     in activity can start. This value is not used if the scalable resource is an Auto Scaling     group.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<i64>,


    /// 
    /// The amount of time, in seconds, after a scale-out activity completes before another     scale-out activity can start. This value is not used if the scalable resource is an Auto     Scaling group.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<i64>,


    /// 
    /// The target value for the metric. Although this property accepts numbers of type Double,     it won't accept values that are either too small or too large. Values must be in the range     of -2^360 to 2^360.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}


