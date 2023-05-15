
pub mod cfn_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnPipeline {
    /// List of PipelineTag
    #[serde(rename = "PipelineTags")]
    pub pipeline_tags: Option<Vec<PipelineTag>>,
    /// List of PipelineObject
    #[serde(rename = "PipelineObjects")]
    pub pipeline_objects: Option<Vec<PipelineObject>>,
    /// List of ParameterObject
    #[serde(rename = "ParameterObjects")]
    pub parameter_objects: Vec<ParameterObject>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Activate")]
    pub activate: Option<bool>,
    /// List of ParameterValue
    #[serde(rename = "ParameterValues")]
    pub parameter_values: Option<Vec<ParameterValue>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct ParameterObject {
    #[serde(rename = "Attributes")]
    pub attributes: Vec<ParameterAttribute>,
    #[serde(rename = "Id")]
    pub id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Field {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "RefValue")]
    pub ref_value: Option<String>,
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PipelineTag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterAttribute {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "StringValue")]
    pub string_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ParameterValue {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "StringValue")]
    pub string_value: String,

}

#[derive(serde::Serialize, Default)]
pub struct PipelineObject {
    #[serde(rename = "Fields")]
    pub fields: Vec<Field>,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,

}


}
