
pub mod cfn_experiment_template {

#[derive(serde::Serialize, Default)]
pub struct CfnExperimentTemplate {
    /// No documentation provided by AWS
    #[serde(rename = "Targets")]
    pub targets: ExperimentTemplateTargetMap,
    /// No documentation provided by AWS
    #[serde(rename = "LogConfiguration")]
    pub log_configuration: Option<ExperimentTemplateLogConfiguration>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: ExperimentTemplateDescription,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: (),
    /// No documentation provided by AWS
    #[serde(rename = "Actions")]
    pub actions: Option<ExperimentTemplateActionMap>,
    /// No documentation provided by AWS
    #[serde(rename = "StopConditions")]
    pub stop_conditions: ExperimentTemplateStopConditionList,
    /// The Amazon Resource Name (ARN) of an IAM role that grants the AWS FIS service permission to perform service actions on your behalf.
    #[serde(rename = "RoleArn")]
    pub role_arn: RoleArn,

}


#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateTarget {
    #[serde(rename = "Filters")]
    pub filters: Option<ExperimentTemplateTargetFilterList>,
    #[serde(rename = "ResourceTags")]
    pub resource_tags: Option<()>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    #[serde(rename = "SelectionMode")]
    pub selection_mode: ExperimentTemplateTargetSelectionMode,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceType,
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Option<ResourceArnList>,

}

#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateActionMap {

}

#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateStopConditionList {

}
pub type ActionId = String;pub type ExperimentTemplateActionItemTarget = String;pub type ExperimentTemplateId = String;
#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateStopCondition {
    #[serde(rename = "Value")]
    pub value: Option<StopConditionValue>,
    #[serde(rename = "Source")]
    pub source: StopConditionSource,

}

#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateTargetMap {

}
pub type RoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct ResourceArnList {

}

#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateTargetFilterValues {

}
pub type StopConditionValue = String;
#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateTargetFilter {
    #[serde(rename = "Path")]
    pub path: ExperimentTemplateTargetFilterPath,
    #[serde(rename = "Values")]
    pub values: ExperimentTemplateTargetFilterValues,

}
pub type ResourceType = String;pub type ExperimentTemplateActionItemParameter = String;pub type ExperimentTemplateActionItemStartAfter = String;pub type StopConditionSource = String;pub type ExperimentTemplateTargetFilterPath = String;
#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateTargetFilterList {

}

#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateAction {
    #[serde(rename = "ActionId")]
    pub action_id: ActionId,
    #[serde(rename = "Description")]
    pub description: Option<ExperimentTemplateActionItemDescription>,
    #[serde(rename = "Targets")]
    pub targets: Option<()>,
    #[serde(rename = "StartAfter")]
    pub start_after: Option<ExperimentTemplateActionItemStartAfterList>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,

}
pub type ExperimentTemplateActionItemDescription = String;pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateLogConfiguration {
    #[serde(rename = "CloudWatchLogsConfiguration")]
    pub cloud_watch_logs_configuration: Option<()>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: Option<()>,
    #[serde(rename = "LogSchemaVersion")]
    pub log_schema_version: usize,

}
pub type ExperimentTemplateTargetSelectionMode = String;pub type ExperimentTemplateTargetFilterValue = String;
#[derive(serde::Serialize, Default)]
pub struct ExperimentTemplateActionItemStartAfterList {

}
pub type ExperimentTemplateDescription = String;

}
