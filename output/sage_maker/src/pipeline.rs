

/// The AWS::SageMaker::Pipeline resource creates shell scripts that run when       you create and/or start a SageMaker Pipeline. For information about SageMaker Pipelines,       see SageMaker         Pipelines in the Amazon SageMaker Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPipeline {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ParallelismConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ParallelismConfiguration")]
    pub parallelism_configuration: Option<ParallelismConfiguration>,


    /// 
    /// The definition of the pipeline. This can be either a JSON string or an Amazon S3       location.
    /// 
    /// Required: Yes
    ///
    /// Type: PipelineDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineDefinition")]
    pub pipeline_definition: PipelineDefinition,


    /// 
    /// The description of the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 3072
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineDescription")]
    pub pipeline_description: Option<String>,


    /// 
    /// The display name of the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,255}
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineDisplayName")]
    pub pipeline_display_name: Option<String>,


    /// 
    /// The name of the pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,255}
    ///
    /// Update requires: Replacement
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role used to execute the pipeline.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// 
    /// The tags of the pipeline.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnPipeline {
    fn type_string() -> &'static str {
        "AWS::SageMaker::Pipeline"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Configuration that controls the parallelism of the pipeline.       By default, the parallelism configuration specified applies to all       executions of the pipeline unless overridden.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ParallelismConfiguration {


    /// 
    /// The max number of steps that can be executed in parallel.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxParallelExecutionSteps")]
    pub max_parallel_execution_steps: i64,

}




/// The PipelineDefinition property type specifies Property description not available. for an AWS::SageMaker::Pipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PipelineDefinition {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineDefinitionBody")]
    pub pipeline_definition_body: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: S3Location
    ///
    /// Update requires: No interruption
    #[serde(rename = "PipelineDefinitionS3Location")]
    pub pipeline_definition_s3_location: Option<S3Location>,

}




/// The S3Location property type specifies Property description not available. for an AWS::SageMaker::Pipeline.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Location {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Bucket")]
    pub bucket: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ETag")]
    pub etag: Option<String>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: Option<String>,

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


