
pub mod cfn_destination {

#[derive(serde::Serialize, Default)]
pub struct CfnDestination {
    /// The ARN of the physical target where the log events are delivered (for example, a Kinesis stream)
    #[serde(rename = "TargetArn")]
    pub target_arn: String,
    /// An IAM policy document that governs which AWS accounts can create subscription filters against this destination.
    #[serde(rename = "DestinationPolicy")]
    pub destination_policy: Option<String>,
    /// The name of the destination resource
    #[serde(rename = "DestinationName")]
    pub destination_name: String,
    /// The ARN of an IAM role that permits CloudWatch Logs to send data to the specified AWS resource
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



}

pub mod cfn_log_group {

#[derive(serde::Serialize, Default)]
pub struct CfnLogGroup {
    /// The number of days to retain the log events in the specified log group. Possible values are: 1, 3, 5, 7, 14, 30, 60, 90, 120, 150, 180, 365, 400, 545, 731, 1096, 1827, and 3653.
    #[serde(rename = "RetentionInDays")]
    pub retention_in_days: Option<usize>,
    /// The body of the policy document you want to use for this topic.
    /// 
    /// You can only add one policy per topic.
    /// 
    /// The policy must be in JSON string format.
    /// 
    /// Length Constraints: Maximum length of 30720
    #[serde(rename = "DataProtectionPolicy")]
    pub data_protection_policy: Option<()>,
    /// The name of the log group. If you don't specify a name, AWS CloudFormation generates a unique ID for the log group.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,
    /// The Amazon Resource Name (ARN) of the CMK to use when encrypting log data.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_log_stream {

#[derive(serde::Serialize, Default)]
pub struct CfnLogStream {
    /// The name of the log group where the log stream is created.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,
    /// The name of the log stream. The name must be unique wihtin the log group.
    #[serde(rename = "LogStreamName")]
    pub log_stream_name: Option<String>,

}



}

pub mod cfn_metric_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnMetricFilter {
    /// A name for the metric filter.
    #[serde(rename = "FilterName")]
    pub filter_name: Option<String>,
    /// A collection of information that defines how metric data gets emitted.
    #[serde(rename = "MetricTransformations")]
    pub metric_transformations: Vec<MetricTransformation>,
    /// Existing log group that you want to associate with this filter.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,
    /// Pattern that Logs follows to interpret each entry in a log.
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: String,

}


#[derive(serde::Serialize, Default)]
pub struct Dimension {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricTransformation {
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<f64>,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "MetricValue")]
    pub metric_value: String,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricNamespace")]
    pub metric_namespace: String,

}


}

pub mod cfn_query_definition {

#[derive(serde::Serialize, Default)]
pub struct CfnQueryDefinition {
    /// The query string to use for this definition
    #[serde(rename = "QueryString")]
    pub query_string: String,
    /// Optionally define specific log groups as part of your query definition
    #[serde(rename = "LogGroupNames")]
    pub log_group_names: Option<Vec<LogGroup>>,
    /// A name for the saved query definition
    #[serde(rename = "Name")]
    pub name: String,

}

pub type LogGroup = String;

}

pub mod cfn_resource_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnResourcePolicy {
    /// A name for resource policy
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    /// The policy document
    #[serde(rename = "PolicyDocument")]
    pub policy_document: String,

}



}

pub mod cfn_subscription_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnSubscriptionFilter {
    /// The Amazon Resource Name (ARN) of the destination.
    #[serde(rename = "DestinationArn")]
    pub destination_arn: String,
    /// The filtering expressions that restrict what gets delivered to the destination AWS resource.
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: String,
    /// The ARN of an IAM role that grants CloudWatch Logs permissions to deliver ingested log events to the destination stream. You don't need to provide the ARN when you are working with a logical destination for cross-account delivery.
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// The name of the filter generated by resource.
    #[serde(rename = "FilterName")]
    pub filter_name: Option<String>,
    /// The method used to distribute log data to the destination. By default, log data is grouped by log stream, but the grouping can be set to random for a more even distribution. This property is only applicable when the destination is an Amazon Kinesis stream.
    #[serde(rename = "Distribution")]
    pub distribution: Option<String>,
    /// Existing log group that you want to associate with this filter.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: String,

}



}
