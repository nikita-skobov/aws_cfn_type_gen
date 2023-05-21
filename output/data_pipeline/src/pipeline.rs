

/// The AWS::DataPipeline::Pipeline resource specifies a data pipeline that you can use to    automate the movement and transformation of data. In each pipeline, you define pipeline    objects, such as activities, schedules, data nodes, and resources. For information about    pipeline objects and components that you can use, see Pipeline Object     Reference in the AWS Data Pipeline Developer     Guide.
///
/// The AWS::DataPipeline::Pipeline resource adds tasks, schedules, and    preconditions to the specified pipeline. You can use PutPipelineDefinition to    populate a new pipeline.
///
/// PutPipelineDefinition also validates the configuration as it adds it to the pipeline. Changes to the pipeline are saved unless one       of the following validation errors exist in the pipeline.
///
/// Pipeline object definitions are passed to the PutPipelineDefinition action and returned by the GetPipelineDefinition action.
#[derive(Default, serde::Serialize)]
pub struct CfnPipeline {


    /// 
    /// A description of the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Indicates whether to validate and start the pipeline or stop an active pipeline. By    default, the value is set to true.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Activate")]
    pub activate: Option<bool>,


    /// 
    /// The parameter objects used with the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: List of ParameterObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterObjects")]
    pub parameter_objects: Vec<ParameterObject>,


    /// 
    /// The objects that define the pipeline. These objects overwrite the existing pipeline definition. Not all objects, fields, and values    can be updated. For information about restrictions, see    Editing Your Pipeline    in the AWS Data Pipeline Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of PipelineObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineObjects")]
    pub pipeline_objects: Option<Vec<PipelineObject>>,


    /// 
    /// A list of arbitrary tags (key-value pairs) to associate with the pipeline, which you   can use to control permissions. For more information, see Controlling Access to    Pipelines and Resources in the   AWS Data Pipeline Developer Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of PipelineTag
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineTags")]
    pub pipeline_tags: Option<Vec<PipelineTag>>,


    /// 
    /// The parameter values used with the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: List of ParameterValue
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParameterValues")]
    pub parameter_values: Option<Vec<ParameterValue>>,


    /// 
    /// The name of the pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\n\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetags.html
#[derive(Default, serde::Serialize)]
pub struct PipelineTag {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetags.html#cfn-datapipeline-pipeline-pipelinetags-key
    #[serde(rename = "Key")]
    pub key: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetags.html#cfn-datapipeline-pipeline-pipelinetags-value
    #[serde(rename = "Value")]
    pub value: String,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalues.html
#[derive(Default, serde::Serialize)]
pub struct ParameterValue {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalues.html#cfn-datapipeline-pipeline-parametervalues-stringvalue
    #[serde(rename = "StringValue")]
    pub string_value: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalues.html#cfn-datapipeline-pipeline-parametervalues-id
    #[serde(rename = "Id")]
    pub id: String,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects-attributes.html
#[derive(Default, serde::Serialize)]
pub struct ParameterAttribute {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects-attributes.html#cfn-datapipeline-pipeline-parameterobjects-attribtues-key
    #[serde(rename = "Key")]
    pub key: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects-attributes.html#cfn-datapipeline-pipeline-parameterobjects-attribtues-stringvalue
    #[serde(rename = "StringValue")]
    pub string_value: String,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html
#[derive(Default, serde::Serialize)]
pub struct PipelineObject {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html#cfn-datapipeline-pipeline-pipelineobjects-fields
    #[serde(rename = "Fields")]
    pub fields: Vec<Field>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html#cfn-datapipeline-pipeline-pipelineobjects-name
    #[serde(rename = "Name")]
    pub name: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html#cfn-datapipeline-pipeline-pipelineobjects-id
    #[serde(rename = "Id")]
    pub id: String,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects-fields.html
#[derive(Default, serde::Serialize)]
pub struct Field {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects-fields.html#cfn-datapipeline-pipeline-pipelineobjects-fields-key
    #[serde(rename = "Key")]
    pub key: String,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects-fields.html#cfn-datapipeline-pipeline-pipelineobjects-fields-refvalue
    #[serde(rename = "RefValue")]
    pub ref_value: Option<String>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects-fields.html#cfn-datapipeline-pipeline-pipelineobjects-fields-stringvalue
    #[serde(rename = "StringValue")]
    pub string_value: Option<String>,

}


/// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects.html
#[derive(Default, serde::Serialize)]
pub struct ParameterObject {


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects.html#cfn-datapipeline-pipeline-parameterobjects-attributes
    #[serde(rename = "Attributes")]
    pub attributes: Vec<ParameterAttribute>,


    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects.html#cfn-datapipeline-pipeline-parameterobjects-id
    #[serde(rename = "Id")]
    pub id: String,

}
