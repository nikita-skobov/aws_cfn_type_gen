/// Creates or updates an Evidently experiment. Before you create an experiment,       you must create the feature to use for the experiment.
///
/// An experiment helps you make feature design       decisions based on evidence and data. An experiment can test as       many as five variations at once. Evidently collects experiment data and analyzes it by statistical methods, and provides       clear recommendations about which variations perform better.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnExperiment {
    ///
    /// An optional description of the experiment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// An array of structures that defines the metrics used for the experiment, and whether a higher       or lower value for each metric is the goal. You can use up to three metrics in an experiment.
    ///
    /// Required: Yes
    ///
    /// Type: List of MetricGoalObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricGoals")]
    pub metric_goals: Vec<MetricGoalObject>,

    ///
    /// A name for the new experiment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// A structure that contains the configuration of which variation to use as the "control"       version. The "control" version is used for comparison with other variations. This structure       also specifies how much experiment traffic is allocated to each variation.
    ///
    /// Required: Yes
    ///
    /// Type: OnlineAbConfigObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "OnlineAbConfig")]
    pub online_ab_config: OnlineAbConfigObject,

    ///
    /// The name or the ARN of the project where this experiment is to be created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Project")]
    pub project: String,

    ///
    /// When Evidently assigns a particular user session to an experiment, it must use a randomization ID       to determine which variation the user session is served. This randomization ID is a combination of the entity ID       and randomizationSalt. If you omit randomizationSalt, Evidently uses       the experiment name as the randomizationSalt.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RandomizationSalt")]
    pub randomization_salt: Option<String>,

    ///
    /// Set this to true to remove the segment that is associated with this experiment. You can't     use this parameter if the experiment is currently running.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RemoveSegment")]
    pub remove_segment: Option<bool>,

    /// A structure that you can use     to start and stop the experiment.
    ///
    /// Required: No
    ///
    /// Type: RunningStatusObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunningStatus")]
    pub running_status: Option<RunningStatusObject>,

    ///
    /// The portion of the available audience that you want to allocate to this experiment, in thousandths of a percent.         The available audience       is the total audience minus the audience that you have allocated to overrides or current launches of       this feature.
    ///
    /// This is represented in thousandths of a percent. For example, specify 10,000 to allocate 10% of the         available audience.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: Option<i64>,

    ///
    /// Specifies an audience segment to use in the experiment. When a segment is used in       an experiment, only user sessions that match the segment pattern are used in the experiment.
    ///
    /// For more information, see       Segment rule pattern syntax.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Segment")]
    pub segment: Option<String>,

    ///
    /// Assigns one or more tags (key-value pairs) to the experiment.
    ///
    /// Tags can help you organize and categorize your resources. You can also use them to scope user         permissions by granting a user         permission to access or change only resources with certain tag values.
    ///
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    ///
    /// You can associate as many as 50 tags with an experiment.
    ///
    /// For more information, see Tagging AWS resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// An array of structures that describe the configuration of each feature variation used in the experiment.
    ///
    /// Required: Yes
    ///
    /// Type: List of TreatmentObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "Treatments")]
    pub treatments: Vec<TreatmentObject>,
}

impl cfn_resources::CfnResource for CfnExperiment {
    fn type_string(&self) -> &'static str {
        "AWS::Evidently::Experiment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.online_ab_config.validate()?;

        self.running_status
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Use this structure to tell Evidently whether higher or lower values are desired for a metric that is       used in an experiment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetricGoalObject {
    ///
    /// INCREASE means that a variation with a higher number for this metric is performing       better.
    ///
    /// DECREASE means that a variation with a lower number for this metric is performing       better.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredChange")]
    pub desired_change: String,

    ///
    /// The entity, such as a user or session, that does an action that causes a metric     value to be recorded. An example is userDetails.userID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityIdKey")]
    pub entity_id_key: String,

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
    pub event_pattern: Option<String>,

    ///
    /// A name for the metric. It can include up to 255 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetricName")]
    pub metric_name: String,

    ///
    /// A label for the units that the metric is measuring.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnitLabel")]
    pub unit_label: Option<String>,

    ///
    /// The JSON path to reference the numerical metric value in the event.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ValueKey")]
    pub value_key: String,
}

impl cfn_resources::CfnResource for MetricGoalObject {
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

/// A structure that contains the configuration of which variation to use as the "control"       version. The "control" version is used for comparison with other variations. This structure       also specifies how much experiment traffic is allocated to each variation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OnlineAbConfigObject {
    ///
    /// The name of the variation that is to be the default variation that the other variations are compared to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ControlTreatmentName")]
    pub control_treatment_name: Option<String>,

    ///
    /// A set of key-value pairs. The keys are treatment names, and the values are the portion       of experiment traffic to be assigned to that treatment. Specify the traffic portion in       thousandths of a percent, so 20,000 for a variation would allocate 20% of the experiment       traffic to that variation.
    ///
    /// Required: No
    ///
    /// Type: List of TreatmentToWeight
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatmentWeights")]
    pub treatment_weights: Option<Vec<TreatmentToWeight>>,
}

impl cfn_resources::CfnResource for OnlineAbConfigObject {
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

/// Use this structure to start and stop the experiment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RunningStatusObject {
    /// If you are using     AWS CloudFormation to start the experiment, use this field to specify when the experiment is to end. The format   is as a UNIX timestamp. For more information about this format, see     The Current Epoch Unix Timestamp.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AnalysisCompleteTime")]
    pub analysis_complete_time: Option<String>,

    /// If you are using AWS CloudFormation to stop this     experiment, specify either COMPLETED or CANCELLED here to indicate how to classify this   experiment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,

    /// If you are using AWS CloudFormation to stop this   experiment, this is an optional field that you can use to record why the experiment is being stopped or cancelled.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

    /// To start the experiment now, specify START   for this parameter. If this experiment is currently running and you want to stop it now, specify STOP.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: String,
}

impl cfn_resources::CfnResource for RunningStatusObject {
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

/// A structure that defines one treatment in an experiment. A treatment is a variation of the feature       that you are including in the experiment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TreatmentObject {
    ///
    /// The description of the treatment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name of the feature for this experiment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Feature")]
    pub feature: String,

    ///
    /// A name for this treatment. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TreatmentName")]
    pub treatment_name: String,

    ///
    /// The name of the variation to use for this treatment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Variation")]
    pub variation: String,
}

impl cfn_resources::CfnResource for TreatmentObject {
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

/// This structure defines how much experiment traffic to allocate to     one treatment used in the experiment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TreatmentToWeight {
    ///
    /// The portion of experiment traffic to allocate to this treatment.     Specify the traffic portion in     thousandths of a percent, so 20,000 allocated to a treatment would allocate 20% of the experiment     traffic to that treatment.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SplitWeight")]
    pub split_weight: i64,

    ///
    /// The name of the treatment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Treatment")]
    pub treatment: String,
}

impl cfn_resources::CfnResource for TreatmentToWeight {
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
