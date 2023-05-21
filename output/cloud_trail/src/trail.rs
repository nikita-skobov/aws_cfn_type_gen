

/// Creates a trail that specifies the settings for delivery of log data to an Amazon S3 bucket.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrail {


    /// 
    /// Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that     represents the log group to which CloudTrail logs are delivered. You must use a log     group that exists in your account.
    /// 
    /// Not required unless you specify CloudWatchLogsRoleArn.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    pub cloud_watch_logs_log_group_arn: Option<String>,


    /// 
    /// Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's     log group. You must use a role that exists in your account.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: Option<String>,


    /// 
    /// Specifies whether log file validation is enabled. The default is false.
    /// 
    /// NoteWhen you disable log file integrity validation, the chain of digest files is broken       after one hour. CloudTrail does not create digest files for log files that were       delivered during a period in which log file integrity validation was disabled. For       example, if you enable log file integrity validation at noon on January 1, disable it at       noon on January 2, and re-enable it at noon on January 10, digest files will not be       created for the log files delivered from noon on January 2 to noon on January 10. The       same applies whenever you stop CloudTrail logging or delete a trail.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableLogFileValidation")]
    pub enable_log_file_validation: Option<bool>,


    /// 
    /// Use event selectors to further specify the management and data event settings for your     trail. By default, trails created without specific event selectors will be configured to     log all read and write management events, and no data events. When an event occurs in your     account, CloudTrail evaluates the event selector for all trails. For each trail, if the     event matches any event selector, the trail processes and logs the event. If the event     doesn't match any event selector, the trail doesn't log the event.
    /// 
    /// You can configure up to five event selectors for a trail.
    /// 
    /// For more information about how to configure event selectors, see Examples      and Configuring event selectors in the      AWS CloudTrail User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of EventSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventSelectors")]
    pub event_selectors: Option<Vec<EventSelector>>,


    /// 
    /// Specifies whether the trail is publishing events from global services such as IAM to the log files.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeGlobalServiceEvents")]
    pub include_global_service_events: Option<bool>,


    /// 
    /// A JSON string that contains the insight types you want to log on a trail.       ApiCallRateInsight and ApiErrorRateInsight are valid Insight     types.
    /// 
    /// The ApiCallRateInsight Insights type analyzes write-only     management API calls that are aggregated per minute against a baseline API call volume.
    /// 
    /// The ApiErrorRateInsight Insights type analyzes management     API calls that result in error codes. The error is shown if the API call is     unsuccessful.
    /// 
    /// Required: No
    ///
    /// Type: List of InsightSelector
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightSelectors")]
    pub insight_selectors: Option<Vec<InsightSelector>>,


    /// 
    /// Whether the CloudTrail trail is currently logging AWS API     calls.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsLogging")]
    pub is_logging: bool,


    /// 
    /// Specifies whether the trail applies only to the current region or to all regions. The     default is false. If the trail exists only in the current region and this value is set to     true, shadow trails (replications of the trail) will be created in the other regions. If     the trail exists in all regions and this value is set to false, the trail will remain in     the region where it was created, and its shadow trails in other regions will be deleted. As     a best practice, consider using trails that log events in all regions.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsMultiRegionTrail")]
    pub is_multi_region_trail: Option<bool>,


    /// 
    /// Specifies whether the trail is applied to all accounts in an organization in AWS Organizations, or only for the current AWS account. The default is false,     and cannot be true unless the call is made on behalf of an AWS account that     is the management account or delegated administrator account for an organization in AWS Organizations. If the trail is not an organization trail and this is set to       true, the trail will be created in all AWS accounts that     belong to the organization. If the trail is an organization trail and this is set to       false, the trail will remain in the current AWS account but     be deleted from all member accounts in the organization.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IsOrganizationTrail")]
    pub is_organization_trail: Option<bool>,


    /// 
    /// Specifies the AWS KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be an alias name prefixed by "alias/", a fully specified ARN to     an alias, a fully specified ARN to a key, or a globally unique identifier.
    /// 
    /// CloudTrail also supports AWS KMS multi-Region keys. For more     information about multi-Region keys, see Using multi-Region       keys in the         AWS Key Management Service Developer Guide.
    /// 
    /// Examples:
    /// 
    /// alias/MyAliasName               arn:aws:kms:us-east-2:123456789012:alias/MyAliasName               arn:aws:kms:us-east-2:123456789012:key/12345678-1234-1234-1234-123456789012               12345678-1234-1234-1234-123456789012
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KMSKeyId")]
    pub kmskey_id: Option<String>,


    /// 
    /// Specifies the name of the Amazon S3 bucket designated for publishing log files.     See Amazon S3       Bucket Naming Requirements.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,


    /// 
    /// Specifies the Amazon S3 key prefix that comes after the name of the bucket you     have designated for log file delivery. For more information, see Finding Your CloudTrail Log Files. The maximum length is 200     characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3KeyPrefix")]
    pub s3_key_prefix: Option<String>,


    /// 
    /// Specifies the name of the Amazon SNS topic defined for notification of log file     delivery. The maximum length is 256 characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicName")]
    pub sns_topic_name: Option<String>,


    /// 
    /// A custom set of tags (key-value pairs) for this trail.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Specifies the name of the trail. The name must meet the following requirements:
    /// 
    /// Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores        (_), or dashes (-)               Start with a letter or number, and end with a letter or number               Be between 3 and 128 characters               Have no adjacent periods, underscores or dashes. Names like          my-_namespace and my--namespace are not valid.               Not be in IP address format (for example, 192.168.5.4)
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrailName")]
    pub trail_name: Option<String>,

}



impl cfn_resources::CfnResource for CfnTrail {
    fn type_string() -> &'static str {
        "AWS::CloudTrail::Trail"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The Amazon S3 buckets, AWS Lambda functions, or Amazon DynamoDB tables that you specify     in event selectors in your AWS CloudFormation template for your trail to log data events. Data events provide information     about the resource operations performed on or within a resource itself. These are also     known as data plane operations. You can specify up to 250 data resources for a     trail. Currently, advanced event selectors for      data events are not supported in AWS CloudFormation templates.
///
/// The following example demonstrates how logging works when you configure logging of all data events     for an S3 bucket named bucket-1. In this example, the CloudTrail user specified an empty prefix,    and the option to log both Read and Write data events.
///
/// The following example demonstrates how logging works when you configure logging of AWS Lambda data events for a      Lambda function named MyLambdaFunction, but not for all Lambda functions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataResource {


    /// 
    /// The resource type in which you want to log data events. You can specify     the following basic event selector resource types:
    /// 
    /// AWS::S3::Object                                AWS::Lambda::Function                                AWS::DynamoDB::Table
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// An array of Amazon Resource Name (ARN) strings or partial ARN strings for the specified objects.
    /// 
    /// To log data events for all objects in all S3 buckets in your AWS account, specify the      prefix as arn:aws:s3.        NoteThis also enables logging of data event activity performed by any user or role in your AWS account,      even if that activity is performed on a bucket that belongs to another AWS account.                        To log data events for all objects in an S3 bucket, specify the bucket and an empty object prefix such as          arn:aws:s3:::bucket-1/. The trail logs data events for all objects in this S3 bucket.                       To log data events for specific objects, specify the S3 bucket and object prefix such     as arn:aws:s3:::bucket-1/example-images. The trail logs data events for     objects in this S3 bucket that match the prefix.               To log data events for all Lambda functions in your AWS account, specify the prefix as          arn:aws:lambda.        NoteThis also enables logging of Invoke activity performed by any user or role in your AWS account,      even if that activity is performed on a function that belongs to another AWS account.               To log data events for a specific Lambda function, specify the function ARN.        NoteLambda function ARNs are exact. For example, if you specify a       function ARN arn:aws:lambda:us-west-2:111111111111:function:helloworld, data events will       only be logged for arn:aws:lambda:us-west-2:111111111111:function:helloworld. They will       not be logged for arn:aws:lambda:us-west-2:111111111111:function:helloworld2.               To log data events for all DynamoDB tables in your AWS account, specify the prefix        as arn:aws:dynamodb.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Values")]
    pub values: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for DataResource {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Use event selectors to further specify the management and data event settings for your     trail. By default, trails created without specific event selectors will be configured to     log all read and write management events, and no data events. When an event occurs in your     account, CloudTrail evaluates the event selector for all trails. For each trail, if     the event matches any event selector, the trail processes and logs the event. If the event     doesn't match any event selector, the trail doesn't log the event.
///
/// You can configure up to five event selectors for a trail.
///
/// You cannot apply both event selectors and advanced event selectors to a trail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventSelector {


    /// 
    /// In AWS CloudFormation, CloudTrail supports data event logging for Amazon S3 objects,      Amazon DynamoDB tables, and AWS Lambda functions. Currently, advanced event selectors for      data events are not supported in AWS CloudFormation templates.      You can specify      up to 250 resources for an individual event selector, but the total number of data resources cannot exceed      250 across all event selectors in a trail. This limit does not apply if you configure resource logging for all data events.
    /// 
    /// For more information, see       Logging data events and Limits in AWS CloudTrail        in the AWS CloudTrail User Guide.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of DataResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataResources")]
    pub data_resources: Option<Vec<DataResource>>,


    /// 
    /// An optional list of service event sources from which you do not want management events     to be logged on your trail. In this release, the list can be empty (disables the filter),     or it can filter out AWS Key Management Service or Amazon RDS Data API events by     containing kms.amazonaws.com or rdsdata.amazonaws.com. By     default, ExcludeManagementEventSources is empty, and AWS KMS and       Amazon RDS Data API events are logged to your trail. You can exclude management     event sources only in regions that support the event source.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeManagementEventSources")]
    pub exclude_management_event_sources: Option<Vec<String>>,


    /// 
    /// Specify if you want your event selector to include management events for your     trail.
    /// 
    /// For more information, see Management Events in the         AWS CloudTrail User     Guide.
    /// 
    /// By default, the value is true.
    /// 
    /// The first copy of management events is free. You are charged for additional copies of     management events that you are logging on any subsequent trail in the same region. For more     information about CloudTrail pricing, see AWS CloudTrail Pricing.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeManagementEvents")]
    pub include_management_events: Option<bool>,


    /// 
    /// Specify if you want your trail to log read-only events, write-only events, or all. For     example, the EC2 GetConsoleOutput is a read-only API operation and       RunInstances is a write-only API operation.
    /// 
    /// By default, the value is All.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: All | ReadOnly | WriteOnly
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReadWriteType")]
    pub read_write_type: Option<EventSelectorReadWriteTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EventSelectorReadWriteTypeEnum {

    /// All
    #[serde(rename = "All")]
    All,

    /// ReadOnly
    #[serde(rename = "ReadOnly")]
    Readonly,

    /// WriteOnly
    #[serde(rename = "WriteOnly")]
    Writeonly,

}

impl Default for EventSelectorReadWriteTypeEnum {
    fn default() -> Self {
        EventSelectorReadWriteTypeEnum::All
    }
}


impl cfn_resources::CfnResource for EventSelector {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// A JSON string that contains a list of Insights types that are logged on a trail.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InsightSelector {


    /// 
    /// The type of Insights events to log on a trail. ApiCallRateInsight and       ApiErrorRateInsight are valid Insight types.
    /// 
    /// The ApiCallRateInsight Insights type analyzes write-only     management API calls that are aggregated per minute against a baseline API call volume.
    /// 
    /// The ApiErrorRateInsight Insights type analyzes management     API calls that result in error codes. The error is shown if the API call is     unsuccessful.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ApiCallRateInsight | ApiErrorRateInsight
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsightType")]
    pub insight_type: Option<InsightSelectorInsightTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum InsightSelectorInsightTypeEnum {

    /// ApiCallRateInsight
    #[serde(rename = "ApiCallRateInsight")]
    Apicallrateinsight,

    /// ApiErrorRateInsight
    #[serde(rename = "ApiErrorRateInsight")]
    Apierrorrateinsight,

}

impl Default for InsightSelectorInsightTypeEnum {
    fn default() -> Self {
        InsightSelectorInsightTypeEnum::Apicallrateinsight
    }
}


impl cfn_resources::CfnResource for InsightSelector {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}



impl cfn_resources::CfnResource for Tag {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}