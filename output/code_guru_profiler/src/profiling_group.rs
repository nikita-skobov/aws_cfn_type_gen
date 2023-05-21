/// Creates a profiling group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProfilingGroup {
    ///
    /// The agent permissions attached to this profiling group. This action group grants       ConfigureAgent and PostAgentProfile permissions to perform     actions required by the profiling agent. The Json consists of key     Principals.
    ///
    /// Principals: A list of string ARNs for the roles and users you want     to grant access to the profiling group. Wildcards are not supported in the ARNs. You are     allowed to provide up to 50 ARNs. An empty list is not permitted. This is a required key.
    ///
    /// For more information, see Resource-based policies       in CodeGuru Profiler in the Amazon CodeGuru Profiler user       guide, ConfigureAgent, and       PostAgentProfile.
    ///
    /// Required: No
    ///
    /// Type: AgentPermissions
    ///
    /// Update requires: No interruption
    #[serde(rename = "AgentPermissions")]
    pub agent_permissions: Option<AgentPermissions>,

    ///
    /// Adds anomaly notifications for a profiling group.
    ///
    /// Required: No
    ///
    /// Type: List of Channel
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnomalyDetectionNotificationConfiguration")]
    pub anomaly_detection_notification_configuration: Option<Vec<Channel>>,

    ///
    /// The compute platform of the profiling group. Use AWSLambda if your     application runs on AWS Lambda. Use Default if your application runs on a     compute platform that is not AWS Lambda, such an Amazon EC2 instance, an on-premises     server, or a different platform. If not specified, Default is used. This     property is immutable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComputePlatform")]
    pub compute_platform: Option<String>,

    ///
    /// The name of the profiling group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfilingGroupName")]
    pub profiling_group_name: String,

    ///
    /// A list of tags to add to the created profiling group.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnProfilingGroup {
    fn type_string(&self) -> &'static str {
        "AWS::CodeGuruProfiler::ProfilingGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.agent_permissions
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The AgentPermissions property type specifies Property description not available. for an AWS::CodeGuruProfiler::ProfilingGroup.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AgentPermissions {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principals")]
    pub principals: Vec<String>,
}

impl cfn_resources::CfnResource for AgentPermissions {
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

/// Notification medium for users to get alerted for events that occur in application profile. We support SNS topic as a notification channel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Channel {
    ///
    /// The channel ID.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,

    ///
    /// The channel URI.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "channelUri")]
    pub channel_uri: String,
}

impl cfn_resources::CfnResource for Channel {
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
