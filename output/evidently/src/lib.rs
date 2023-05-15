
pub mod cfn_experiment {

#[derive(serde::Serialize, Default)]
pub struct CfnExperiment {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Project")]
    pub project: String,
    /// No documentation provided by AWS
    #[serde(rename = "Segment")]
    pub segment: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Start Experiment. Default is False
    #[serde(rename = "RunningStatus")]
    pub running_status: Option<RunningStatusObject>,
    /// No documentation provided by AWS
    #[serde(rename = "RandomizationSalt")]
    pub randomization_salt: Option<String>,
    /// List of TreatmentObject
    #[serde(rename = "Treatments")]
    pub treatments: Vec<TreatmentObject>,
    /// List of MetricGoalObject
    #[serde(rename = "MetricGoals")]
    pub metric_goals: Vec<MetricGoalObject>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// No documentation provided by AWS
    #[serde(rename = "RemoveSegment")]
    pub remove_segment: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "SamplingRate")]
    pub sampling_rate: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "OnlineAbConfig")]
    pub online_ab_config: OnlineAbConfigObject,

}


#[derive(serde::Serialize, Default)]
pub struct RunningStatusObject {
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,
    #[serde(rename = "Reason")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "AnalysisCompleteTime")]
    pub analysis_complete_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TreatmentObject {
    #[serde(rename = "TreatmentName")]
    pub treatment_name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Variation")]
    pub variation: String,
    #[serde(rename = "Feature")]
    pub feature: String,

}

#[derive(serde::Serialize, Default)]
pub struct MetricGoalObject {
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<String>,
    #[serde(rename = "UnitLabel")]
    pub unit_label: Option<String>,
    #[serde(rename = "EntityIdKey")]
    pub entity_id_key: String,
    #[serde(rename = "DesiredChange")]
    pub desired_change: String,
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "ValueKey")]
    pub value_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct OnlineAbConfigObject {
    #[serde(rename = "ControlTreatmentName")]
    pub control_treatment_name: Option<String>,
    #[serde(rename = "TreatmentWeights")]
    pub treatment_weights: Option<Vec<TreatmentToWeight>>,

}

#[derive(serde::Serialize, Default)]
pub struct TreatmentToWeight {
    #[serde(rename = "SplitWeight")]
    pub split_weight: usize,
    #[serde(rename = "Treatment")]
    pub treatment: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_feature {

#[derive(serde::Serialize, Default)]
pub struct CfnFeature {
    /// No documentation provided by AWS
    #[serde(rename = "EvaluationStrategy")]
    pub evaluation_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultVariation")]
    pub default_variation: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of EntityOverride
    #[serde(rename = "EntityOverrides")]
    pub entity_overrides: Option<Vec<EntityOverride>>,
    /// List of VariationObject
    #[serde(rename = "Variations")]
    pub variations: Vec<VariationObject>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Project")]
    pub project: String,

}


#[derive(serde::Serialize, Default)]
pub struct EntityOverride {
    #[serde(rename = "Variation")]
    pub variation: Option<String>,
    #[serde(rename = "EntityId")]
    pub entity_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VariationObject {
    #[serde(rename = "BooleanValue")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "VariationName")]
    pub variation_name: Option<String>,
    #[serde(rename = "LongValue")]
    pub long_value: Option<f64>,
    #[serde(rename = "DoubleValue")]
    pub double_value: Option<f64>,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_launch {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunch {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Start or Stop Launch Launch. Default is not started.
    #[serde(rename = "ExecutionStatus")]
    pub execution_status: Option<ExecutionStatusObject>,
    /// List of StepConfig
    #[serde(rename = "ScheduledSplitsConfig")]
    pub scheduled_splits_config: Vec<StepConfig>,
    /// List of LaunchGroupObject
    #[serde(rename = "Groups")]
    pub groups: Vec<LaunchGroupObject>,
    /// No documentation provided by AWS
    #[serde(rename = "RandomizationSalt")]
    pub randomization_salt: Option<String>,
    /// List of MetricDefinitionObject
    #[serde(rename = "MetricMonitors")]
    pub metric_monitors: Option<Vec<MetricDefinitionObject>>,
    /// No documentation provided by AWS
    #[serde(rename = "Project")]
    pub project: String,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct LaunchGroupObject {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Variation")]
    pub variation: String,
    #[serde(rename = "GroupName")]
    pub group_name: String,
    #[serde(rename = "Feature")]
    pub feature: String,

}

#[derive(serde::Serialize, Default)]
pub struct StepConfig {
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "GroupWeights")]
    pub group_weights: Vec<GroupToWeight>,
    #[serde(rename = "SegmentOverrides")]
    pub segment_overrides: Option<Vec<SegmentOverride>>,

}

#[derive(serde::Serialize, Default)]
pub struct MetricDefinitionObject {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "EntityIdKey")]
    pub entity_id_key: String,
    #[serde(rename = "UnitLabel")]
    pub unit_label: Option<String>,
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<String>,
    #[serde(rename = "ValueKey")]
    pub value_key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ExecutionStatusObject {
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct GroupToWeight {
    #[serde(rename = "GroupName")]
    pub group_name: String,
    #[serde(rename = "SplitWeight")]
    pub split_weight: usize,

}

#[derive(serde::Serialize, Default)]
pub struct SegmentOverride {
    #[serde(rename = "Segment")]
    pub segment: String,
    #[serde(rename = "EvaluationOrder")]
    pub evaluation_order: usize,
    #[serde(rename = "Weights")]
    pub weights: Vec<GroupToWeight>,

}


}

pub mod cfn_project {

#[derive(serde::Serialize, Default)]
pub struct CfnProject {
    /// No documentation provided by AWS
    #[serde(rename = "DataDelivery")]
    pub data_delivery: Option<DataDeliveryObject>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AppConfigResource")]
    pub app_config_resource: Option<AppConfigResourceObject>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct DataDeliveryObject {
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,
    #[serde(rename = "S3")]
    pub s3: Option<S3Destination>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Destination {
    #[serde(rename = "BucketName")]
    pub bucket_name: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AppConfigResourceObject {
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
    #[serde(rename = "ApplicationId")]
    pub application_id: String,

}


}

pub mod cfn_segment {

#[derive(serde::Serialize, Default)]
pub struct CfnSegment {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Pattern")]
    pub pattern: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}
