
pub mod cfn_replication_set {

#[derive(serde::Serialize, Default)]
pub struct CfnReplicationSet {
    /// The ReplicationSet configuration.
    #[serde(rename = "Regions")]
    pub regions: RegionList,
    /// Configures the ReplicationSet deletion protection.
    #[serde(rename = "DeletionProtected")]
    pub deletion_protected: Option<DeletionProtected>,
    /// The tags to apply to the replication set.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct RegionList {

}
pub type RegionName = String;
#[derive(serde::Serialize, Default)]
pub struct ReplicationRegion {
    #[serde(rename = "RegionName")]
    pub region_name: Option<RegionName>,
    #[serde(rename = "RegionConfiguration")]
    pub region_configuration: Option<RegionConfiguration>,

}
pub type Arn = String;
#[derive(serde::Serialize, Default)]
pub struct RegionConfiguration {
    #[serde(rename = "SseKmsKeyId")]
    pub sse_kms_key_id: Arn,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type DeletionProtected = bool;

}

pub mod cfn_response_plan {

#[derive(serde::Serialize, Default)]
pub struct CfnResponsePlan {
    /// The chat channel configuration.
    #[serde(rename = "ChatChannel")]
    pub chat_channel: Option<ChatChannel>,
    /// The display name of the response plan.
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// The list of engagements to use.
    #[serde(rename = "Engagements")]
    pub engagements: Option<Vec<SSMContact>>,
    /// The list of actions.
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,
    /// The list of integrations.
    #[serde(rename = "Integrations")]
    pub integrations: Option<Vec<Integration>>,
    /// The tags to apply to the response plan.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The incident template configuration.
    #[serde(rename = "IncidentTemplate")]
    pub incident_template: IncidentTemplate,
    /// The name of the response plan.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct SsmAutomation {
    #[serde(rename = "DocumentName")]
    pub document_name: String,
    #[serde(rename = "TargetAccount")]
    pub target_account: Option<String>,
    #[serde(rename = "DynamicParameters")]
    pub dynamic_parameters: Option<Vec<DynamicSsmParameter>>,
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<SsmParameter>>,
    #[serde(rename = "DocumentVersion")]
    pub document_version: Option<String>,
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ChatChannel {
    #[serde(rename = "ChatbotSns")]
    pub chatbot_sns: Option<ChatbotSns>,

}
pub type VariableType = String;
#[derive(serde::Serialize, Default)]
pub struct DynamicSsmParameterValue {
    #[serde(rename = "Variable")]
    pub variable: Option<VariableType>,

}

#[derive(serde::Serialize, Default)]
pub struct PagerDutyConfiguration {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "PagerDutyIncidentConfiguration")]
    pub pager_duty_incident_configuration: PagerDutyIncidentConfiguration,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct SsmParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Values")]
    pub values: Vec<SsmParameterValue>,

}

#[derive(serde::Serialize, Default)]
pub struct DynamicSsmParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: DynamicSsmParameterValue,

}
pub type SsmParameterValue = String;
#[derive(serde::Serialize, Default)]
pub struct ChatbotSns {

}
pub type SSMContact = String;
#[derive(serde::Serialize, Default)]
pub struct Integration {
    #[serde(rename = "PagerDutyConfiguration")]
    pub pager_duty_configuration: Option<PagerDutyConfiguration>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Action {
    #[serde(rename = "SsmAutomation")]
    pub ssm_automation: Option<SsmAutomation>,

}

#[derive(serde::Serialize, Default)]
pub struct IncidentTemplate {
    #[serde(rename = "Summary")]
    pub summary: Option<String>,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Impact")]
    pub impact: usize,
    #[serde(rename = "IncidentTags")]
    pub incident_tags: Option<Vec<Tag>>,
    #[serde(rename = "NotificationTargets")]
    pub notification_targets: Option<Vec<NotificationTargetItem>>,
    #[serde(rename = "DedupeString")]
    pub dedupe_string: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NotificationTargetItem {
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: Option<SnsArn>,

}
pub type SnsArn = String;
#[derive(serde::Serialize, Default)]
pub struct PagerDutyIncidentConfiguration {
    #[serde(rename = "ServiceId")]
    pub service_id: String,

}


}
