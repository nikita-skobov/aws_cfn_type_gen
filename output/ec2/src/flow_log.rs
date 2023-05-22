/// Specifies a VPC flow log that captures IP traffic for a specified network interface,     subnet, or VPC. To view the log data, use Amazon CloudWatch Logs (CloudWatch Logs) to help     troubleshoot connection issues. For example, you can use a flow log to investigate why     certain traffic isn't reaching an instance, which can help you diagnose overly restrictive     security group rules. For more information, see VPC Flow Logs in the Amazon       VPC User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlowLog {
    ///
    /// The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in       your account.
    ///
    /// This parameter is required if the destination type is cloud-watch-logs       and unsupported otherwise.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeliverLogsPermissionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_logs_permission_arn: Option<cfn_resources::StrVal>,

    ///
    /// The destination options. The following options are supported:
    ///
    /// FileFormat - The format for the flow log (plain-text |       parquet). The default is plain-text.HiveCompatiblePartitions - Indicates whether to use Hive-compatible prefixes for       flow logs stored in Amazon S3 (true | false). The default       is false.PerHourPartition - Indicates whether to partition the flow log per hour       (true | false). The default is       false.
    ///
    /// Required: No
    ///
    /// Type: DestinationOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_options: Option<DestinationOptions>,

    ///
    /// The destination for the flow log data. The meaning of this parameter depends on the destination type.
    ///
    /// If the destination type is cloud-watch-logs, specify the ARN of a CloudWatch Logs log group. For example:        arn:aws:logs:region:account_id:log-group:my_group                Alternatively, use the LogGroupName parameter.               If the destination type is s3, specify the ARN of an S3 bucket. For example:        arn:aws:s3:::my_bucket/my_subfolder/        The subfolder is optional. Note that you can't use AWSLogs as a subfolder name.               If the destination type is kinesis-data-firehose, specify the ARN of a Kinesis Data Firehose delivery stream. For example:        arn:aws:firehose:region:account_id:deliverystream:my_stream
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination: Option<cfn_resources::StrVal>,

    ///
    /// The type of destination for the flow log data.
    ///
    /// Default: cloud-watch-logs
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: cloud-watch-logs | kinesis-data-firehose | s3
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogDestinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination_type: Option<FlowLogLogDestinationTypeEnum>,

    ///
    /// The fields to include in the flow log record, in the order in which they should appear.      If you omit this parameter, the flow log is created using the default format. If you specify      this parameter, you must include at least one field. For more information about the available fields,      see Flow log       records in the Amazon VPC User Guide or Transit Gateway         Flow Log records in the AWS Transit Gateway Guide.
    ///
    /// Specify the fields using the ${field-id} format, separated by     spaces.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<cfn_resources::StrVal>,

    ///
    /// The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.
    ///
    /// This parameter is valid only if the destination type is cloud-watch-logs.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<cfn_resources::StrVal>,

    ///
    /// The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record.       The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes).       This parameter must be 60 seconds for transit gateway resource types.
    ///
    /// When a network interface is attached to a Nitro-based         instance, the aggregation interval is always 60 seconds or less, regardless       of the value that you specify.
    ///
    /// Default: 600
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxAggregationInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_aggregation_interval: Option<i64>,

    ///
    /// The ID of the resource to monitor. For example, if the resource type is     VPC, specify the ID of the VPC.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceId")]
    pub resource_id: cfn_resources::StrVal,

    ///
    /// The type of resource to monitor.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: NetworkInterface | Subnet | TransitGateway | TransitGatewayAttachment | VPC
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: FlowLogResourceTypeEnum,

    ///
    /// The tags to apply to the flow logs.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic).       This parameter is not supported for transit gateway resource types. It is required for       the other resource types.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACCEPT | ALL | REJECT
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrafficType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<FlowLogTrafficTypeEnum>,

    #[serde(skip_serializing)]
    pub att_id: CfnFlowLogid,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FlowLogLogDestinationTypeEnum {
    /// cloud-watch-logs
    #[serde(rename = "cloud-watch-logs")]
    Cloudwatchlogs,

    /// kinesis-data-firehose
    #[serde(rename = "kinesis-data-firehose")]
    Kinesisdatafirehose,

    /// s3
    #[serde(rename = "s3")]
    S3,
}

impl Default for FlowLogLogDestinationTypeEnum {
    fn default() -> Self {
        FlowLogLogDestinationTypeEnum::Cloudwatchlogs
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FlowLogResourceTypeEnum {
    /// NetworkInterface
    #[serde(rename = "NetworkInterface")]
    Networkinterface,

    /// Subnet
    #[serde(rename = "Subnet")]
    Subnet,

    /// TransitGateway
    #[serde(rename = "TransitGateway")]
    Transitgateway,

    /// TransitGatewayAttachment
    #[serde(rename = "TransitGatewayAttachment")]
    Transitgatewayattachment,

    /// VPC
    #[serde(rename = "VPC")]
    Vpc,
}

impl Default for FlowLogResourceTypeEnum {
    fn default() -> Self {
        FlowLogResourceTypeEnum::Networkinterface
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FlowLogTrafficTypeEnum {
    /// ACCEPT
    #[serde(rename = "ACCEPT")]
    Accept,

    /// ALL
    #[serde(rename = "ALL")]
    All,

    /// REJECT
    #[serde(rename = "REJECT")]
    Reject,
}

impl Default for FlowLogTrafficTypeEnum {
    fn default() -> Self {
        FlowLogTrafficTypeEnum::Accept
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlowLogid;
impl CfnFlowLogid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnFlowLog {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::FlowLog"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the destination options for a flow log.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationOptions {
    ///
    /// The format for the flow log. The default is plain-text.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: parquet | plain-text
    ///
    /// Update requires: Replacement
    #[serde(rename = "FileFormat")]
    pub file_format: DestinationOptionsFileFormatEnum,

    ///
    /// Indicates whether to use Hive-compatible prefixes for flow logs stored in Amazon S3.       The default is false.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "HiveCompatiblePartitions")]
    pub hive_compatible_partitions: bool,

    ///
    /// Indicates whether to partition the flow log per hour. This reduces the cost and response       time for queries. The default is false.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "PerHourPartition")]
    pub per_hour_partition: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DestinationOptionsFileFormatEnum {
    /// parquet
    #[serde(rename = "parquet")]
    Parquet,

    /// plain-text
    #[serde(rename = "plain-text")]
    Plaintext,
}

impl Default for DestinationOptionsFileFormatEnum {
    fn default() -> Self {
        DestinationOptionsFileFormatEnum::Parquet
    }
}

impl cfn_resources::CfnResource for DestinationOptions {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
