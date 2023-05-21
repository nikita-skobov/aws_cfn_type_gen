

/// The AWS::ApplicationAutoScaling::ScalingPolicy resource defines a scaling    policy that Application Auto Scaling uses to adjust the capacity of a scalable target.
///
/// For more information, see Target     tracking scaling policies and Step scaling policies in the Application Auto Scaling User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnScalingPolicy {


    /// 
    /// The name of the scaling policy.
    /// 
    /// Updates to the name of a target tracking scaling policy are not supported, unless you also    update the metric used for scaling. To change only a target tracking scaling policy's name,    first delete the policy by removing the existing     AWS::ApplicationAutoScaling::ScalingPolicy resource from the template and    updating the stack. Then, recreate the resource with the same settings and a different    name.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: \p{Print}+
    ///
    /// Update requires: Replacement
    #[serde(rename = "PolicyName")]
    pub policy_name: String,


    /// 
    /// The scaling policy type.
    /// 
    /// The following policy types are supported:
    /// 
    /// TargetTrackingScaling—Not supported for Amazon EMR
    /// 
    /// StepScaling—Not supported for DynamoDB, Amazon Comprehend, Lambda, Amazon Keyspaces, Amazon MSK, Amazon ElastiCache, or    Neptune.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: StepScaling | TargetTrackingScaling
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyType")]
    pub policy_type: ScalingPolicyPolicyTypeEnum,


    /// 
    /// The identifier of the resource associated with the scaling policy.    This string consists of the resource type and unique identifier.
    /// 
    /// ECS service - The resource type is service and the unique identifier is the cluster name         and service name. Example: service/default/sample-webapp.               Spot Fleet - The resource type is spot-fleet-request and the unique identifier is the         Spot Fleet request ID. Example: spot-fleet-request/sfr-73fbd2ce-aa30-494c-8788-1cee4EXAMPLE.               EMR cluster - The resource type is instancegroup and the unique identifier is the cluster ID and instance group ID.        Example: instancegroup/j-2EEZNYKUA1NTV/ig-1791Y4E1L8YI0.               AppStream 2.0 fleet - The resource type is fleet and the unique identifier is the fleet name.        Example: fleet/sample-fleet.               DynamoDB table - The resource type is table and the unique identifier is the table name.         Example: table/my-table.               DynamoDB global secondary index - The resource type is index and the unique identifier is the index name.         Example: table/my-table/index/my-table-index.               Aurora DB cluster - The resource type is cluster and the unique identifier is the cluster name.        Example: cluster:my-db-cluster.               SageMaker endpoint variant - The resource type is variant and the unique identifier is the resource ID.        Example: endpoint/my-end-point/variant/KMeansClustering.               Custom resources are not supported with a resource type. This parameter must specify the OutputValue from the CloudFormation template stack used to access the resources. The unique identifier is defined by the service provider. More information        is available in our GitHub          repository.               Amazon Comprehend document classification endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: arn:aws:comprehend:us-west-2:123456789012:document-classifier-endpoint/EXAMPLE.               Amazon Comprehend entity recognizer endpoint - The resource type and unique identifier are specified using the endpoint ARN. Example: arn:aws:comprehend:us-west-2:123456789012:entity-recognizer-endpoint/EXAMPLE.               Lambda provisioned concurrency - The resource type is function and the unique identifier is the function name with a function version or alias name suffix that is not $LATEST.         Example: function:my-function:prod or function:my-function:1.               Amazon Keyspaces table - The resource type is table and the unique identifier is the table name.         Example: keyspace/mykeyspace/table/mytable.               Amazon MSK cluster - The resource type and unique identifier are specified using the cluster ARN.         Example: arn:aws:kafka:us-east-1:123456789012:cluster/demo-cluster-1/6357e0b2-0e6a-4b86-a0b4-70df934c2e31-5.               Amazon ElastiCache replication group - The resource type is replication-group and the unique identifier is the replication group name.        Example: replication-group/mycluster.               Neptune cluster - The resource type is cluster and the unique identifier is the cluster name. Example: cluster:mycluster.               SageMaker Serverless endpoint - The resource type is variant and the unique identifier is the resource ID.        Example: endpoint/my-end-point/variant/KMeansClustering.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,


    /// 
    /// The scalable dimension. This string consists of the service namespace, resource type, and scaling property.
    /// 
    /// ecs:service:DesiredCount - The desired task count of an ECS service.                        elasticmapreduce:instancegroup:InstanceCount - The instance count of an EMR Instance Group.                        ec2:spot-fleet-request:TargetCapacity - The target capacity of a Spot Fleet.                        appstream:fleet:DesiredCapacity - The desired capacity of an AppStream 2.0 fleet.                        dynamodb:table:ReadCapacityUnits - The provisioned read capacity for a DynamoDB table.                        dynamodb:table:WriteCapacityUnits - The provisioned write capacity for a DynamoDB table.                        dynamodb:index:ReadCapacityUnits - The provisioned read capacity for a DynamoDB global secondary index.                        dynamodb:index:WriteCapacityUnits - The provisioned write capacity for a DynamoDB global secondary index.                        rds:cluster:ReadReplicaCount - The count of Aurora Replicas in an Aurora DB cluster. Available for Aurora MySQL-compatible edition and Aurora PostgreSQL-compatible edition.                        sagemaker:variant:DesiredInstanceCount - The number of EC2 instances for a SageMaker model endpoint variant.                        custom-resource:ResourceType:Property - The scalable dimension for a custom resource provided by your own application or service.                        comprehend:document-classifier-endpoint:DesiredInferenceUnits - The number of inference units for an Amazon Comprehend document classification endpoint.                        comprehend:entity-recognizer-endpoint:DesiredInferenceUnits - The number of inference units for an Amazon Comprehend entity recognizer endpoint.                        lambda:function:ProvisionedConcurrency - The provisioned concurrency for a Lambda function.                        cassandra:table:ReadCapacityUnits - The provisioned read capacity for an Amazon Keyspaces table.                        cassandra:table:WriteCapacityUnits - The provisioned write capacity for an Amazon Keyspaces table.                        kafka:broker-storage:VolumeSize - The provisioned volume size (in GiB) for brokers in an Amazon MSK cluster.                        elasticache:replication-group:NodeGroups - The number of node groups for an Amazon ElastiCache replication group.                        elasticache:replication-group:Replicas - The number of replicas per node group for an Amazon ElastiCache replication group.                        neptune:cluster:ReadReplicaCount - The count of read replicas in an Amazon Neptune DB cluster.                        sagemaker:variant:DesiredProvisionedConcurrency - The provisioned concurrency for a SageMaker Serverless endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: appstream:fleet:DesiredCapacity | cassandra:table:ReadCapacityUnits | cassandra:table:WriteCapacityUnits | comprehend:document-classifier-endpoint:DesiredInferenceUnits | comprehend:entity-recognizer-endpoint:DesiredInferenceUnits | custom-resource:ResourceType:Property | dynamodb:index:ReadCapacityUnits | dynamodb:index:WriteCapacityUnits | dynamodb:table:ReadCapacityUnits | dynamodb:table:WriteCapacityUnits | ec2:spot-fleet-request:TargetCapacity | ecs:service:DesiredCount | elasticache:replication-group:NodeGroups | elasticache:replication-group:Replicas | elasticmapreduce:instancegroup:InstanceCount | kafka:broker-storage:VolumeSize | lambda:function:ProvisionedConcurrency | neptune:cluster:ReadReplicaCount | rds:cluster:ReadReplicaCount | sagemaker:variant:DesiredInstanceCount | sagemaker:variant:DesiredProvisionedConcurrency
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScalableDimension")]
    pub scalable_dimension: Option<ScalingPolicyScalableDimensionEnum>,


    /// 
    /// The CloudFormation-generated ID of an Application Auto Scaling scalable target. For more    information about the ID, see the Return Value section of the     AWS::ApplicationAutoScaling::ScalableTarget resource.
    /// 
    /// ImportantYou must specify either the ScalingTargetId property, or the      ResourceId, ScalableDimension, and ServiceNamespace     properties, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScalingTargetId")]
    pub scaling_target_id: Option<String>,


    /// 
    /// The namespace of the AWS service that provides the resource, or a       custom-resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: appstream | cassandra | comprehend | custom-resource | dynamodb | ec2 | ecs | elasticache | elasticmapreduce | kafka | lambda | neptune | rds | sagemaker
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceNamespace")]
    pub service_namespace: Option<ScalingPolicyServiceNamespaceEnum>,


    /// 
    /// A step scaling policy.
    /// 
    /// Required: No
    ///
    /// Type: StepScalingPolicyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepScalingPolicyConfiguration")]
    pub step_scaling_policy_configuration: Option<StepScalingPolicyConfiguration>,


    /// 
    /// A target tracking scaling policy.
    /// 
    /// Required: No
    ///
    /// Type: TargetTrackingScalingPolicyConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetTrackingScalingPolicyConfiguration")]
    pub target_tracking_scaling_policy_configuration: Option<TargetTrackingScalingPolicyConfiguration>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingPolicyPolicyTypeEnum {

    /// StepScaling
    #[serde(rename = "StepScaling")]
    Stepscaling,

    /// TargetTrackingScaling
    #[serde(rename = "TargetTrackingScaling")]
    Targettrackingscaling,

}

impl Default for ScalingPolicyPolicyTypeEnum {
    fn default() -> Self {
        ScalingPolicyPolicyTypeEnum::Stepscaling
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingPolicyScalableDimensionEnum {

    /// appstream:fleet:DesiredCapacity
    #[serde(rename = "appstream:fleet:DesiredCapacity")]
    Appstreamfleetdesiredcapacity,

    /// cassandra:table:ReadCapacityUnits
    #[serde(rename = "cassandra:table:ReadCapacityUnits")]
    Cassandratablereadcapacityunits,

    /// cassandra:table:WriteCapacityUnits
    #[serde(rename = "cassandra:table:WriteCapacityUnits")]
    Cassandratablewritecapacityunits,

    /// comprehend:document-classifier-endpoint:DesiredInferenceUnits
    #[serde(rename = "comprehend:document-classifier-endpoint:DesiredInferenceUnits")]
    Comprehenddocumentclassifierendpointdesiredinferenceunits,

    /// comprehend:entity-recognizer-endpoint:DesiredInferenceUnits
    #[serde(rename = "comprehend:entity-recognizer-endpoint:DesiredInferenceUnits")]
    Comprehendentityrecognizerendpointdesiredinferenceunits,

    /// custom-resource:ResourceType:Property
    #[serde(rename = "custom-resource:ResourceType:Property")]
    Customresourceresourcetypeproperty,

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

    /// elasticache:replication-group:NodeGroups
    #[serde(rename = "elasticache:replication-group:NodeGroups")]
    Elasticachereplicationgroupnodegroups,

    /// elasticache:replication-group:Replicas
    #[serde(rename = "elasticache:replication-group:Replicas")]
    Elasticachereplicationgroupreplicas,

    /// elasticmapreduce:instancegroup:InstanceCount
    #[serde(rename = "elasticmapreduce:instancegroup:InstanceCount")]
    Elasticmapreduceinstancegroupinstancecount,

    /// kafka:broker-storage:VolumeSize
    #[serde(rename = "kafka:broker-storage:VolumeSize")]
    Kafkabrokerstoragevolumesize,

    /// lambda:function:ProvisionedConcurrency
    #[serde(rename = "lambda:function:ProvisionedConcurrency")]
    Lambdafunctionprovisionedconcurrency,

    /// neptune:cluster:ReadReplicaCount
    #[serde(rename = "neptune:cluster:ReadReplicaCount")]
    Neptuneclusterreadreplicacount,

    /// rds:cluster:ReadReplicaCount
    #[serde(rename = "rds:cluster:ReadReplicaCount")]
    Rdsclusterreadreplicacount,

    /// sagemaker:variant:DesiredInstanceCount
    #[serde(rename = "sagemaker:variant:DesiredInstanceCount")]
    Sagemakervariantdesiredinstancecount,

    /// sagemaker:variant:DesiredProvisionedConcurrency
    #[serde(rename = "sagemaker:variant:DesiredProvisionedConcurrency")]
    Sagemakervariantdesiredprovisionedconcurrency,

}

impl Default for ScalingPolicyScalableDimensionEnum {
    fn default() -> Self {
        ScalingPolicyScalableDimensionEnum::Appstreamfleetdesiredcapacity
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ScalingPolicyServiceNamespaceEnum {

    /// appstream
    #[serde(rename = "appstream")]
    Appstream,

    /// cassandra
    #[serde(rename = "cassandra")]
    Cassandra,

    /// comprehend
    #[serde(rename = "comprehend")]
    Comprehend,

    /// custom-resource
    #[serde(rename = "custom-resource")]
    Customresource,

    /// dynamodb
    #[serde(rename = "dynamodb")]
    Dynamodb,

    /// ec2
    #[serde(rename = "ec2")]
    Ec2,

    /// ecs
    #[serde(rename = "ecs")]
    Ecs,

    /// elasticache
    #[serde(rename = "elasticache")]
    Elasticache,

    /// elasticmapreduce
    #[serde(rename = "elasticmapreduce")]
    Elasticmapreduce,

    /// kafka
    #[serde(rename = "kafka")]
    Kafka,

    /// lambda
    #[serde(rename = "lambda")]
    Lambda,

    /// neptune
    #[serde(rename = "neptune")]
    Neptune,

    /// rds
    #[serde(rename = "rds")]
    Rds,

    /// sagemaker
    #[serde(rename = "sagemaker")]
    Sagemaker,

}

impl Default for ScalingPolicyServiceNamespaceEnum {
    fn default() -> Self {
        ScalingPolicyServiceNamespaceEnum::Appstream
    }
}


impl cfn_resources::CfnResource for CfnScalingPolicy {
    fn type_string() -> &'static str {
        "AWS::ApplicationAutoScaling::ScalingPolicy"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains customized metric specification information for a target tracking scaling policy    for Application Auto Scaling.
///
/// For information about the available metrics for a service, see AWS services that publish CloudWatch metrics in the Amazon     CloudWatch User Guide.
///
/// To create your customized metric specification:
///
/// For an example of how creating new metrics can be useful, see Scaling based on Amazon SQS    in the Amazon EC2 Auto Scaling User Guide. This topic mentions Auto    Scaling groups, but the same scenario for Amazon SQS can apply to the target tracking scaling    policies that you create for a Spot Fleet by using Application Auto Scaling.
///
/// For more information about the CloudWatch terminology below, see Amazon CloudWatch concepts.
///
/// CustomizedMetricSpecification is a property of the AWS::ApplicationAutoScaling::ScalingPolicy TargetTrackingScalingPolicyConfiguration    property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomizedMetricSpecification {


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
    /// The name of the metric. To get the exact metric name, namespace, and dimensions, inspect     the Metric object that's returned by a call to ListMetrics.
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



/// MetricDimension specifies a name/value pair that is part of the identity of a    CloudWatch metric for the Dimensions property of the AWS::ApplicationAutoScaling::ScalingPolicy CustomizedMetricSpecification property    type. Duplicate dimensions are not allowed.
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




/// Contains predefined metric specification information for a target tracking scaling policy    for Application Auto Scaling.
///
/// PredefinedMetricSpecification is a property of the AWS::ApplicationAutoScaling::ScalingPolicy TargetTrackingScalingPolicyConfiguration    property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PredefinedMetricSpecification {


    /// 
    /// The metric type. The ALBRequestCountPerTarget metric type applies only to    Spot fleet requests and ECS services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALBRequestCountPerTarget | AppStreamAverageCapacityUtilization | CassandraReadCapacityUtilization | CassandraWriteCapacityUtilization | ComprehendInferenceUtilization | DynamoDBReadCapacityUtilization | DynamoDBWriteCapacityUtilization | EC2SpotFleetRequestAverageCPUUtilization | EC2SpotFleetRequestAverageNetworkIn | EC2SpotFleetRequestAverageNetworkOut | ECSServiceAverageCPUUtilization | ECSServiceAverageMemoryUtilization | ElastiCacheDatabaseMemoryUsageCountedForEvictPercentage | ElastiCachePrimaryEngineCPUUtilization | ElastiCacheReplicaEngineCPUUtilization | KafkaBrokerStorageUtilization | LambdaProvisionedConcurrencyUtilization | NeptuneReaderAverageCPUUtilization | RDSReaderAverageCPUUtilization | RDSReaderAverageDatabaseConnections | SageMakerVariantInvocationsPerInstance | SageMakerVariantProvisionedConcurrencyUtilization
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricType")]
    pub predefined_metric_type: PredefinedMetricSpecificationPredefinedMetricTypeEnum,


    /// 
    /// Identifies the resource associated with the metric type. You can't specify a resource     label unless the metric type is ALBRequestCountPerTarget and there is a target     group attached to the Spot Fleet or ECS service.
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
    /// Update requires: No interruption
    #[serde(rename = "ResourceLabel")]
    pub resource_label: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PredefinedMetricSpecificationPredefinedMetricTypeEnum {

    /// ALBRequestCountPerTarget
    #[serde(rename = "ALBRequestCountPerTarget")]
    Albrequestcountpertarget,

    /// AppStreamAverageCapacityUtilization
    #[serde(rename = "AppStreamAverageCapacityUtilization")]
    Appstreamaveragecapacityutilization,

    /// CassandraReadCapacityUtilization
    #[serde(rename = "CassandraReadCapacityUtilization")]
    Cassandrareadcapacityutilization,

    /// CassandraWriteCapacityUtilization
    #[serde(rename = "CassandraWriteCapacityUtilization")]
    Cassandrawritecapacityutilization,

    /// ComprehendInferenceUtilization
    #[serde(rename = "ComprehendInferenceUtilization")]
    Comprehendinferenceutilization,

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

    /// ElastiCacheDatabaseMemoryUsageCountedForEvictPercentage
    #[serde(rename = "ElastiCacheDatabaseMemoryUsageCountedForEvictPercentage")]
    Elasticachedatabasememoryusagecountedforevictpercentage,

    /// ElastiCachePrimaryEngineCPUUtilization
    #[serde(rename = "ElastiCachePrimaryEngineCPUUtilization")]
    Elasticacheprimaryenginecpuutilization,

    /// ElastiCacheReplicaEngineCPUUtilization
    #[serde(rename = "ElastiCacheReplicaEngineCPUUtilization")]
    Elasticachereplicaenginecpuutilization,

    /// KafkaBrokerStorageUtilization
    #[serde(rename = "KafkaBrokerStorageUtilization")]
    Kafkabrokerstorageutilization,

    /// LambdaProvisionedConcurrencyUtilization
    #[serde(rename = "LambdaProvisionedConcurrencyUtilization")]
    Lambdaprovisionedconcurrencyutilization,

    /// NeptuneReaderAverageCPUUtilization
    #[serde(rename = "NeptuneReaderAverageCPUUtilization")]
    Neptunereaderaveragecpuutilization,

    /// RDSReaderAverageCPUUtilization
    #[serde(rename = "RDSReaderAverageCPUUtilization")]
    Rdsreaderaveragecpuutilization,

    /// RDSReaderAverageDatabaseConnections
    #[serde(rename = "RDSReaderAverageDatabaseConnections")]
    Rdsreaderaveragedatabaseconnections,

    /// SageMakerVariantInvocationsPerInstance
    #[serde(rename = "SageMakerVariantInvocationsPerInstance")]
    Sagemakervariantinvocationsperinstance,

    /// SageMakerVariantProvisionedConcurrencyUtilization
    #[serde(rename = "SageMakerVariantProvisionedConcurrencyUtilization")]
    Sagemakervariantprovisionedconcurrencyutilization,

}

impl Default for PredefinedMetricSpecificationPredefinedMetricTypeEnum {
    fn default() -> Self {
        PredefinedMetricSpecificationPredefinedMetricTypeEnum::Albrequestcountpertarget
    }
}



/// StepAdjustment specifies a step adjustment for the StepAdjustments    property of the AWS::ApplicationAutoScaling::ScalingPolicy StepScalingPolicyConfiguration property    type.
///
/// For the following examples, suppose that you have an alarm with a breach threshold of 50:
///
/// For more information, see Step adjustments in the Application Auto Scaling User    Guide.
///
/// You can find a sample template snippet in the Examples section of the AWS::ApplicationAutoScaling::ScalingPolicy    documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepAdjustment {


    /// 
    /// The lower bound for the difference between the alarm threshold and the CloudWatch metric.    If the metric value is above the breach threshold, the lower bound is inclusive (the metric    must be greater than or equal to the threshold plus the lower bound). Otherwise, it is    exclusive (the metric must be greater than the threshold plus the lower bound). A null value    indicates negative infinity.
    /// 
    /// You must specify at least one upper or lower bound.
    /// 
    /// Required: Conditional
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricIntervalLowerBound")]
    pub metric_interval_lower_bound: Option<f64>,


    /// 
    /// The upper bound for the difference between the alarm threshold and the CloudWatch metric.    If the metric value is above the breach threshold, the upper bound is exclusive (the metric    must be less than the threshold plus the upper bound). Otherwise, it is inclusive (the metric    must be less than or equal to the threshold plus the upper bound). A null value indicates    positive infinity.
    /// 
    /// You must specify at least one upper or lower bound.
    /// 
    /// Required: Conditional
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricIntervalUpperBound")]
    pub metric_interval_upper_bound: Option<f64>,


    /// 
    /// The amount by which to scale. The adjustment is based on the value that you specified in    the AdjustmentType property (either an absolute number or a percentage). A    positive value adds to the current capacity and a negative number subtracts from the current    capacity.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScalingAdjustment")]
    pub scaling_adjustment: i64,

}




/// StepScalingPolicyConfiguration is a property of the AWS::ApplicationAutoScaling::ScalingPolicy resource that specifies a step scaling    policy configuration for Application Auto Scaling.
///
/// For more information, see Step scaling policies in the Application Auto Scaling User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepScalingPolicyConfiguration {


    /// 
    /// Specifies whether the ScalingAdjustment value in the     StepAdjustment property is an absolute number or a percentage of the current    capacity.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ChangeInCapacity | ExactCapacity | PercentChangeInCapacity
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdjustmentType")]
    pub adjustment_type: Option<StepScalingPolicyConfigurationAdjustmentTypeEnum>,


    /// 
    /// The amount of time, in seconds, to wait for a previous scaling activity to take effect. If    not specified, the default value is 300. For more information, see Cooldown period in the Application Auto Scaling User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cooldown")]
    pub cooldown: Option<i64>,


    /// 
    /// The aggregation type for the CloudWatch metrics. Valid values are Minimum,       Maximum, and Average. If the aggregation type is null, the     value is treated as Average.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Average | Maximum | Minimum
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricAggregationType")]
    pub metric_aggregation_type: Option<StepScalingPolicyConfigurationMetricAggregationTypeEnum>,


    /// 
    /// The minimum value to scale by when the adjustment type is       PercentChangeInCapacity. For example, suppose that you create a step     scaling policy to scale out an Amazon ECS service by 25 percent and you specify a       MinAdjustmentMagnitude of 2. If the service has 4 tasks and the scaling     policy is performed, 25 percent of 4 is 1. However, because you specified a       MinAdjustmentMagnitude of 2, Application Auto Scaling scales out the service by 2     tasks.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinAdjustmentMagnitude")]
    pub min_adjustment_magnitude: Option<i64>,


    /// 
    /// A set of adjustments that enable you to scale based on the size of the alarm     breach.
    /// 
    /// At least one step adjustment is required if you are adding a new step scaling policy     configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of StepAdjustment
    ///
    /// Update requires: No interruption
    #[serde(rename = "StepAdjustments")]
    pub step_adjustments: Option<Vec<StepAdjustment>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum StepScalingPolicyConfigurationAdjustmentTypeEnum {

    /// ChangeInCapacity
    #[serde(rename = "ChangeInCapacity")]
    Changeincapacity,

    /// ExactCapacity
    #[serde(rename = "ExactCapacity")]
    Exactcapacity,

    /// PercentChangeInCapacity
    #[serde(rename = "PercentChangeInCapacity")]
    Percentchangeincapacity,

}

impl Default for StepScalingPolicyConfigurationAdjustmentTypeEnum {
    fn default() -> Self {
        StepScalingPolicyConfigurationAdjustmentTypeEnum::Changeincapacity
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum StepScalingPolicyConfigurationMetricAggregationTypeEnum {

    /// Average
    #[serde(rename = "Average")]
    Average,

    /// Maximum
    #[serde(rename = "Maximum")]
    Maximum,

    /// Minimum
    #[serde(rename = "Minimum")]
    Minimum,

}

impl Default for StepScalingPolicyConfigurationMetricAggregationTypeEnum {
    fn default() -> Self {
        StepScalingPolicyConfigurationMetricAggregationTypeEnum::Average
    }
}



/// TargetTrackingScalingPolicyConfiguration is a property of the AWS::ApplicationAutoScaling::ScalingPolicy resource that specifies a target    tracking scaling policy configuration for Application Auto Scaling. Use a target tracking    scaling policy to adjust the capacity of the specified scalable target in response to actual    workloads, so that resource utilization remains at or near the target utilization value.
///
/// For more information, see Target     tracking scaling policies in the Application Auto Scaling User     Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetTrackingScalingPolicyConfiguration {


    /// 
    /// A customized metric. You can specify either a predefined metric or a customized     metric.
    /// 
    /// Required: No
    ///
    /// Type: CustomizedMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomizedMetricSpecification")]
    pub customized_metric_specification: Option<CustomizedMetricSpecification>,


    /// 
    /// Indicates whether scale in by the target tracking scaling policy is disabled. If the     value is true, scale in is disabled and the target tracking scaling policy     won't remove capacity from the scalable target. Otherwise, scale in is enabled and the     target tracking scaling policy can remove capacity from the scalable target. The default     value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableScaleIn")]
    pub disable_scale_in: Option<bool>,


    /// 
    /// A predefined metric. You can specify either a predefined metric or a customized     metric.
    /// 
    /// Required: No
    ///
    /// Type: PredefinedMetricSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "PredefinedMetricSpecification")]
    pub predefined_metric_specification: Option<PredefinedMetricSpecification>,


    /// 
    /// The amount of time, in seconds, after a scale-in activity completes before another    scale-in activity can start. For more information and for default values, see Define cooldown periods in the Application Auto Scaling User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleInCooldown")]
    pub scale_in_cooldown: Option<i64>,


    /// 
    /// The amount of time, in seconds, to wait for a previous scale-out activity to take effect.    For more information and for default values, see Define cooldown periods in the Application Auto Scaling User Guide.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScaleOutCooldown")]
    pub scale_out_cooldown: Option<i64>,


    /// 
    /// The target value for the metric. Although this property accepts numbers of type Double, it    won't accept values that are either too small or too large. Values must be in the range of    -2^360 to 2^360. The value must be a valid number based on the choice of metric. For example,    if the metric is CPU utilization, then the target value is a percent value that represents how    much of the CPU can be used before scaling out.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetValue")]
    pub target_value: f64,

}


