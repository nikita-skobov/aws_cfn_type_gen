
pub mod cfn_custom_action_type {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomActionType {
    /// The configuration properties for the custom action.
    #[serde(rename = "ConfigurationProperties")]
    pub configuration_properties: Option<Vec<ConfigurationProperties>>,
    /// The category of the custom action, such as a build action or a test action.
    #[serde(rename = "Category")]
    pub category: String,
    /// The provider of the service used in the custom action, such as AWS CodeDeploy.
    #[serde(rename = "Provider")]
    pub provider: String,
    /// The version identifier of the custom action.
    #[serde(rename = "Version")]
    pub version: String,
    /// The details of the input artifact for the action, such as its commit ID.
    #[serde(rename = "InputArtifactDetails")]
    pub input_artifact_details: ArtifactDetails,
    /// URLs that provide users information about this custom action.
    #[serde(rename = "Settings")]
    pub settings: Option<Settings>,
    /// The details of the output artifact of the action, such as its commit ID.
    #[serde(rename = "OutputArtifactDetails")]
    pub output_artifact_details: ArtifactDetails,
    /// Any tags assigned to the custom action.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct ConfigurationProperties {
    #[serde(rename = "Required")]
    pub required: bool,
    #[serde(rename = "Secret")]
    pub secret: bool,
    #[serde(rename = "Queryable")]
    pub queryable: Option<bool>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "Key")]
    pub key: bool,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Settings {
    #[serde(rename = "EntityUrlTemplate")]
    pub entity_url_template: Option<String>,
    #[serde(rename = "ExecutionUrlTemplate")]
    pub execution_url_template: Option<String>,
    #[serde(rename = "RevisionUrlTemplate")]
    pub revision_url_template: Option<String>,
    #[serde(rename = "ThirdPartyConfigurationUrl")]
    pub third_party_configuration_url: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArtifactDetails {
    #[serde(rename = "MinimumCount")]
    pub minimum_count: usize,
    #[serde(rename = "MaximumCount")]
    pub maximum_count: usize,

}


}

pub mod cfn_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnPipeline {
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// List of StageDeclaration
    #[serde(rename = "Stages")]
    pub stages: Vec<StageDeclaration>,
    /// List of StageTransition
    #[serde(rename = "DisableInboundStageTransitions")]
    pub disable_inbound_stage_transitions: Option<Vec<StageTransition>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ArtifactStore")]
    pub artifact_store: Option<ArtifactStore>,
    /// List of ArtifactStoreMap
    #[serde(rename = "ArtifactStores")]
    pub artifact_stores: Option<Vec<ArtifactStoreMap>>,
    /// No documentation provided by AWS
    #[serde(rename = "RestartExecutionOnUpdate")]
    pub restart_execution_on_update: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct ActionTypeId {
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Provider")]
    pub provider: String,

}

#[derive(serde::Serialize, Default)]
pub struct ArtifactStoreMap {
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "ArtifactStore")]
    pub artifact_store: ArtifactStore,

}

#[derive(serde::Serialize, Default)]
pub struct EncryptionKey {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct InputArtifact {
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct StageTransition {
    #[serde(rename = "StageName")]
    pub stage_name: String,
    #[serde(rename = "Reason")]
    pub reason: String,

}

#[derive(serde::Serialize, Default)]
pub struct OutputArtifact {
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ActionDeclaration {
    #[serde(rename = "InputArtifacts")]
    pub input_artifacts: Option<Vec<InputArtifact>>,
    #[serde(rename = "Configuration")]
    pub configuration: Option<()>,
    #[serde(rename = "RunOrder")]
    pub run_order: Option<usize>,
    #[serde(rename = "OutputArtifacts")]
    pub output_artifacts: Option<Vec<OutputArtifact>>,
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    #[serde(rename = "ActionTypeId")]
    pub action_type_id: ActionTypeId,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct BlockerDeclaration {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct StageDeclaration {
    #[serde(rename = "Blockers")]
    pub blockers: Option<Vec<BlockerDeclaration>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<ActionDeclaration>,

}

#[derive(serde::Serialize, Default)]
pub struct ArtifactStore {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<EncryptionKey>,
    #[serde(rename = "Location")]
    pub location: String,

}


}

pub mod cfn_webhook {

#[derive(serde::Serialize, Default)]
pub struct CfnWebhook {
    /// List of WebhookFilterRule
    #[serde(rename = "Filters")]
    pub filters: Vec<WebhookFilterRule>,
    /// No documentation provided by AWS
    #[serde(rename = "Authentication")]
    pub authentication: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetAction")]
    pub target_action: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetPipeline")]
    pub target_pipeline: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthenticationConfiguration")]
    pub authentication_configuration: WebhookAuthConfiguration,
    /// No documentation provided by AWS
    #[serde(rename = "RegisterWithThirdParty")]
    pub register_with_third_party: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetPipelineVersion")]
    pub target_pipeline_version: usize,

}


#[derive(serde::Serialize, Default)]
pub struct WebhookAuthConfiguration {
    #[serde(rename = "AllowedIPRange")]
    pub allowed_iprange: Option<String>,
    #[serde(rename = "SecretToken")]
    pub secret_token: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct WebhookFilterRule {
    #[serde(rename = "JsonPath")]
    pub json_path: String,
    #[serde(rename = "MatchEquals")]
    pub match_equals: Option<String>,

}


}
