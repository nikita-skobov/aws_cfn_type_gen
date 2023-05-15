
pub mod cfn_pipeline {

#[derive(serde::Serialize, Default)]
pub struct CfnPipeline {
    /// The minimum pipeline capacity, in Ingestion Compute Units (ICUs).
    #[serde(rename = "MinUnits")]
    pub min_units: usize,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Data Prepper pipeline configuration in YAML format.
    #[serde(rename = "PipelineConfigurationBody")]
    pub pipeline_configuration_body: String,
    /// Key-value pairs to configure log publishing.
    #[serde(rename = "LogPublishingOptions")]
    pub log_publishing_options: Option<LogPublishingOptions>,
    /// Container for the values required to configure VPC access for the pipeline. If you don't specify these values, OpenSearch Ingestion Service creates the pipeline with a public endpoint.
    #[serde(rename = "VpcOptions")]
    pub vpc_options: Option<VpcOptions>,
    /// Name of the OpenSearch Ingestion Service pipeline to create. Pipeline names are unique across the pipelines owned by an account within an AWS Region.
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,
    /// The maximum pipeline capacity, in Ingestion Compute Units (ICUs).
    #[serde(rename = "MaxUnits")]
    pub max_units: usize,

}


#[derive(serde::Serialize, Default)]
pub struct VpcOptions {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct LogPublishingOptions {
    #[serde(rename = "CloudWatchLogDestination")]
    pub cloud_watch_log_destination: Option<()>,
    #[serde(rename = "IsLoggingEnabled")]
    pub is_logging_enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct VpcEndpoint {
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcOptions")]
    pub vpc_options: Option<VpcOptions>,

}


}
