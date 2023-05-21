/// Creates or updates a launch of a given feature. Before you create a launch, you       must create the feature to use for the launch.
///
/// You can use a launch to safely validate new features by serving them to a specified       percentage of your users while you roll out the feature. You can monitor the performance of       the new feature to help you decide when to ramp up traffic to more users. This helps you       reduce risk and identify unintended consequences before you fully launch the feature.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLaunch {
    ///
    /// An optional description for the launch.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    /// A structure that you can use to start and stop     the launch.
    ///
    /// Required: No
    ///
    /// Type: ExecutionStatusObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExecutionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<ExecutionStatusObject>,

    ///
    /// An array of structures that contains the feature and variations that are to be used for the launch.     You can up to five launch groups in a launch.
    ///
    /// Required: Yes
    ///
    /// Type: List of LaunchGroupObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "Groups")]
    pub groups: Vec<LaunchGroupObject>,

    ///
    /// An array of structures that define the metrics that will be used to monitor       the launch performance. You can have up to three metric monitors in the array.
    ///
    /// Required: No
    ///
    /// Type: List of MetricDefinitionObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricMonitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_monitors: Option<Vec<MetricDefinitionObject>>,

    ///
    /// The name for the launch. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The name or ARN of the project that you want to create the launch in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Project")]
    pub project: cfn_resources::StrVal,

    ///
    /// When Evidently assigns a particular user session to a launch, it must use a randomization ID       to determine which variation the user session is served. This randomization ID is a combination of the entity ID       and randomizationSalt. If you omit randomizationSalt, Evidently uses       the launch name as the randomizationsSalt.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RandomizationSalt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomization_salt: Option<cfn_resources::StrVal>,

    ///
    /// An array of structures that define the traffic allocation percentages among the feature       variations during each step of the launch.
    ///
    /// Required: Yes
    ///
    /// Type: List of StepConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduledSplitsConfig")]
    pub scheduled_splits_config: Vec<StepConfig>,

    ///
    /// Assigns one or more tags (key-value pairs) to the launch.
    ///
    /// Tags can help you organize and categorize your resources. You can also use them to scope user       permissions by granting a user       permission to access or change only resources with certain tag values.
    ///
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    ///
    /// You can associate as many as 50 tags with a launch.
    ///
    /// For more information, see Tagging AWS resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnLaunch {
    fn type_string(&self) -> &'static str {
        "AWS::Evidently::Launch"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.execution_status
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use this structure to start and stop     the launch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExecutionStatusObject {
    ///
    /// If you are using AWS CloudFormation to stop this       launch, specify either COMPLETED or CANCELLED here to indicate how to classify this       experiment. If you omit this parameter, the default of COMPLETED is used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<cfn_resources::StrVal>,

    /// If you are using AWS CloudFormation to stop this     launch, this is an optional field that you can use to record why the launch is being stopped or cancelled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<cfn_resources::StrVal>,

    /// To start the launch now, specify START     for this parameter. If this launch is currently running and you want to stop it now, specify STOP.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ExecutionStatusObject {
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

/// A structure containing the percentage of launch traffic to allocate to one launch group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GroupToWeight {
    ///
    /// The name of the launch group. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: cfn_resources::StrVal,

    ///
    /// The portion of launch traffic to allocate to this launch group.
    ///
    /// This is represented in thousandths of a percent. For example, specify 20,000 to allocate 20% of the         launch audience to this launch group.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SplitWeight")]
    pub split_weight: i64,
}

impl cfn_resources::CfnResource for GroupToWeight {
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

/// A structure that defines one launch group in a launch. A launch group is a variation of       the feature that you are including in the launch.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LaunchGroupObject {
    ///
    /// A description of the launch group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The feature that this launch is using.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Feature")]
    pub feature: cfn_resources::StrVal,

    ///
    /// A name for this launch group. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupName")]
    pub group_name: cfn_resources::StrVal,

    ///
    /// The feature variation to use for this launch group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variation")]
    pub variation: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LaunchGroupObject {
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

/// This structure defines a metric that you want to use to evaluate the variations       during a launch or experiment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricDefinitionObject {
    ///
    /// The entity, such as a user or session, that does an action that causes a metric       value to be recorded. An example is userDetails.userID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityIdKey")]
    pub entity_id_key: cfn_resources::StrVal,

    ///
    /// The EventBridge event pattern that defines how the metric is recorded.
    ///
    /// For more information about EventBridge event patterns, see       Amazon EventBridge event patterns.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<cfn_resources::StrVal>,

    ///
    /// A name for the metric. It can include up to 255 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: cfn_resources::StrVal,

    ///
    /// A label for the units that the metric is measuring.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnitLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<cfn_resources::StrVal>,

    ///
    /// The value that is tracked to produce the metric.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueKey")]
    pub value_key: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for MetricDefinitionObject {
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

/// Use this structure to specify different traffic splits for one or more audience segments. A       segment is a portion of your audience that share one or more characteristics. Examples could be       Chrome browser users, users in Europe, or Firefox browser users in Europe who also fit       other criteria that your application collects, such as age.
///
/// For more information,       see         Use segments to focus your audience.
///
/// This sructure is an array of up to six segment override objects. Each of these objects specifies a       segment that you have already created, and defines the traffic split for that segment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SegmentOverride {
    ///
    /// A number indicating the order to use to evaluate segment overrides, if there are more than       one. Segment overrides with lower numbers are evaluated first.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EvaluationOrder")]
    pub evaluation_order: i64,

    ///
    /// The ARN of the segment to use for this override.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Segment")]
    pub segment: cfn_resources::StrVal,

    ///
    /// The traffic allocation percentages among the feature variations to assign to this segment.       This is a set of key-value pairs. The keys are variation names. The values       represent the amount of traffic to allocate to that variation for this segment.       This is expressed in thousandths of a percent, so a weight of 50000 represents 50% of traffic.
    ///
    /// Required: Yes
    ///
    /// Type: List of GroupToWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "Weights")]
    pub weights: Vec<GroupToWeight>,
}

impl cfn_resources::CfnResource for SegmentOverride {
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

/// A structure that defines when each step of the launch is to start, and how much launch traffic     is to be allocated to each variation during each step.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StepConfig {
    ///
    /// An array of structures that define how much launch traffic to allocate to each launch group     during this step of the launch.
    ///
    /// Required: Yes
    ///
    /// Type: List of GroupToWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "GroupWeights")]
    pub group_weights: Vec<GroupToWeight>,

    ///
    /// An array of structures that you can use to specify different traffic splits for one or more audience segments. A       segment is a portion of your audience that share one or more characteristics. Examples could be       Chrome browser users, users in Europe, or Firefox browser users in Europe who also fit       other criteria that your application collects, such as age.
    ///
    /// For more information,       see         Use segments to focus your audience.
    ///
    /// Required: No
    ///
    /// Type: List of SegmentOverride
    ///
    /// Update requires: No interruption
    #[serde(rename = "SegmentOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_overrides: Option<Vec<SegmentOverride>>,

    ///
    /// The date and time to start this step of the launch. Use UTC format, yyyy-MM-ddTHH:mm:ssZ. For example,       2025-11-25T23:59:59Z
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    pub start_time: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StepConfig {
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
