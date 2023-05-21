///
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEventSubscription {
    ///
    /// A boolean value; set to true to activate the subscription, and set to         false to create the subscription but not activate it.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

    ///
    /// Specifies the Amazon Redshift event categories to be published by the event notification       subscription.
    ///
    /// Values: configuration, management, monitoring, security, pending
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventCategories")]
    pub event_categories: Option<Vec<String>>,

    ///
    /// Specifies the Amazon Redshift event severity to be published by the event notification       subscription.
    ///
    /// Values: ERROR, INFO
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "Severity")]
    pub severity: Option<String>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic used to transmit the event       notifications. The ARN is created by Amazon SNS when you create a topic and subscribe to       it.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<String>,

    ///
    /// A list of one or more identifiers of Amazon Redshift source objects. All of the objects       must be of the same type as was specified in the source type parameter. The event       subscription will return only events generated by the specified objects. If not       specified, then events are returned for all objects within the source type       specified.
    ///
    /// Example: my-cluster-1, my-cluster-2
    ///
    /// Example: my-snapshot-20131010
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceIds")]
    pub source_ids: Option<Vec<String>>,

    ///
    /// The type of source that will be generating the events. For example, if you want to       be notified of events generated by a cluster, you would set this parameter to cluster.       If this value is not specified, events are returned for all Amazon Redshift objects in your       AWS account. You must specify a source type in order to specify source IDs.
    ///
    /// Valid values: cluster, cluster-parameter-group, cluster-security-group, cluster-snapshot, and scheduled-action.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceType")]
    pub source_type: Option<String>,

    ///
    /// The name of the event subscription to be created.
    ///
    /// Constraints:
    ///
    /// Cannot be null, empty, or blank.               Must contain from 1 to 255 alphanumeric characters or hyphens.               First character must be a letter.               Cannot end with a hyphen or contain two consecutive hyphens.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 2147483647
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,

    ///
    /// A list of tag instances.
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
        "AWS::Redshift::EventSubscription"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.severity {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'severity'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sns_topic_arn {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'sns_topic_arn'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.source_type {
            if the_val.len() > 2147483647 as _ {
                return Err(format!(
                    "Max validation failed on field 'source_type'. {} is greater than 2147483647",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.subscription_name;

        if the_val.len() > 2147483647 as _ {
            return Err(format!(
                "Max validation failed on field 'subscription_name'. {} is greater than 2147483647",
                the_val.len()
            ));
        }

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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
