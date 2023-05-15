
pub mod cfn_link {

#[derive(serde::Serialize, Default)]
pub struct CfnLink {
    /// Tags to apply to the link
    #[serde(rename = "Tags")]
    pub tags: Option<()>,
    /// List of ResourceType
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Vec<ResourceType>,
    /// No documentation provided by AWS
    #[serde(rename = "LabelTemplate")]
    pub label_template: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SinkIdentifier")]
    pub sink_identifier: String,

}

pub type ResourceType = String;

}

pub mod cfn_sink {

#[derive(serde::Serialize, Default)]
pub struct CfnSink {
    /// The policy of this ObservabilityAccessManager Sink.
    #[serde(rename = "Policy")]
    pub policy: Option<()>,
    /// The name of the ObservabilityAccessManager Sink.
    #[serde(rename = "Name")]
    pub name: String,
    /// Tags to apply to the sink
    #[serde(rename = "Tags")]
    pub tags: Option<()>,

}



}
