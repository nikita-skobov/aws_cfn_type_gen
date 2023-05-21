/// The AWS::RDS::EventSubscription resource allows you to receive       notifications for Amazon Relational Database Service events through the Amazon Simple       Notification Service (Amazon SNS). For more information, see Using Amazon RDS Event         Notification in the Amazon RDS User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventSubscription {
    ///
    /// A value that indicates whether to activate the subscription. If the event notification subscription isn't activated, the subscription is created but not active.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

    ///
    /// A list of event categories for a particular source type (SourceType)       that you want to subscribe to. You can see a list of the categories for a given source type in the "Amazon RDS event categories and event messages" section of the Amazon RDS User Guide or the         Amazon Aurora User Guide.         You can also see this list by using the DescribeEventCategories operation.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventCategories")]
    pub event_categories: Option<Vec<String>>,

    ///
    /// The Amazon Resource Name (ARN) of the SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,

    ///
    /// The list of identifiers of the event sources for which events are returned. If not specified, then all sources are included in the response.      An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens. It can't end with a hyphen or contain two consecutive hyphens.
    ///
    /// Constraints:
    ///
    /// If a SourceIds value is supplied, SourceType must also be provided.               If the source type is a DB instance, a DBInstanceIdentifier value must be supplied.               If the source type is a DB cluster, a DBClusterIdentifier value must be supplied.                      If the source type is a DB parameter group, a DBParameterGroupName value must be supplied.               If the source type is a DB security group, a DBSecurityGroupName value must be supplied.               If the source type is a DB snapshot, a DBSnapshotIdentifier value must be supplied.               If the source type is a DB cluster snapshot, a DBClusterSnapshotIdentifier value must be supplied.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIds")]
    pub source_ids: Option<Vec<String>>,

    ///
    /// The type of source that is generating the events. For example, if you want to be       notified of events generated by a DB instance, set this parameter to         db-instance. If this value isn't specified, all events are       returned.
    ///
    /// Valid values: db-instance | db-cluster |         db-parameter-group | db-security-group |         db-snapshot | db-cluster-snapshot
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,

    ///
    /// The name of the subscription.
    ///
    /// Constraints: The name must be less than 255 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: Option<String>,

    ///
    /// An optional array of key-value pairs to apply to this subscription.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnEventSubscription {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::EventSubscription"
    }

    fn properties(&self) -> serde_json::Value {
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
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
