
pub mod cfn_detector {

#[derive(serde::Serialize, Default)]
pub struct CfnDetector {
    /// No documentation provided by AWS
    #[serde(rename = "RuleExecutionMode")]
    pub rule_execution_mode: Option<String>,
    /// Tags associated with this detector.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The description of the detector.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of Rule
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
    /// The ID of the detector
    #[serde(rename = "DetectorId")]
    pub detector_id: String,
    /// The event type to associate this detector with.
    #[serde(rename = "EventType")]
    pub event_type: EventType,
    /// The desired detector version status for the detector
    #[serde(rename = "DetectorVersionStatus")]
    pub detector_version_status: Option<String>,
    /// The models to associate with this detector.
    #[serde(rename = "AssociatedModels")]
    pub associated_models: Option<Vec<Model>>,

}


#[derive(serde::Serialize, Default)]
pub struct Rule {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Language")]
    pub language: Option<String>,
    #[serde(rename = "DetectorId")]
    pub detector_id: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "RuleId")]
    pub rule_id: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Outcomes")]
    pub outcomes: Option<Vec<Outcome>>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "RuleVersion")]
    pub rule_version: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Label {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Model {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Outcome {
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EntityType {
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EventVariable {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EventType {
    #[serde(rename = "EntityTypes")]
    pub entity_types: Option<Vec<EntityType>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "EventVariables")]
    pub event_variables: Option<Vec<EventVariable>>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_entity_type {

#[derive(serde::Serialize, Default)]
pub struct CfnEntityType {
    /// The name of the entity type.
    #[serde(rename = "Name")]
    pub name: String,
    /// Tags associated with this entity type.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The entity type description.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_event_type {

#[derive(serde::Serialize, Default)]
pub struct CfnEventType {
    /// List of EventVariable
    #[serde(rename = "EventVariables")]
    pub event_variables: Vec<EventVariable>,
    /// The name for the event type
    #[serde(rename = "Name")]
    pub name: String,
    /// List of Label
    #[serde(rename = "Labels")]
    pub labels: Vec<Label>,
    /// Tags associated with this event type.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of EntityType
    #[serde(rename = "EntityTypes")]
    pub entity_types: Vec<EntityType>,
    /// The description of the event type.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EventVariable {
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "DefaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EntityType {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Label {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "CreatedTime")]
    pub created_time: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Inline")]
    pub inline: Option<bool>,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


}

pub mod cfn_label {

#[derive(serde::Serialize, Default)]
pub struct CfnLabel {
    /// The name of the label.
    #[serde(rename = "Name")]
    pub name: String,
    /// The label description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Tags associated with this label.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_list {

#[derive(serde::Serialize, Default)]
pub struct CfnList {
    /// The description of the list.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The variable type of the list.
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,
    /// The name of the list.
    #[serde(rename = "Name")]
    pub name: String,
    /// Tags associated with this list.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The elements in this list.
    #[serde(rename = "Elements")]
    pub elements: Option<Vec<Element>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Element = String;

}

pub mod cfn_outcome {

#[derive(serde::Serialize, Default)]
pub struct CfnOutcome {
    /// Tags associated with this outcome.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the outcome.
    #[serde(rename = "Name")]
    pub name: String,
    /// The outcome description.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_variable {

#[derive(serde::Serialize, Default)]
pub struct CfnVariable {
    /// The name of the variable.
    #[serde(rename = "Name")]
    pub name: String,
    /// The variable type. For more information see https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types
    #[serde(rename = "VariableType")]
    pub variable_type: Option<String>,
    /// The default value for the variable when no value is received.
    #[serde(rename = "DefaultValue")]
    pub default_value: String,
    /// The source of the data.
    #[serde(rename = "DataSource")]
    pub data_source: String,
    /// The description.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The data type.
    #[serde(rename = "DataType")]
    pub data_type: String,
    /// Tags associated with this variable.
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
