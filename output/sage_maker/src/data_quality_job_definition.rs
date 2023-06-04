/// Creates a definition for a job that monitors data quality and drift. For information     about model monitor, see Amazon SageMaker Model Monitor.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDataQualityJobDefinition {
    ///
    /// Specifies the container that runs the monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: DataQualityAppSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataQualityAppSpecification")]
    pub data_quality_app_specification: DataQualityAppSpecification,

    ///
    /// Configures the constraints and baselines for the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: DataQualityBaselineConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataQualityBaselineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality_baseline_config: Option<DataQualityBaselineConfig>,

    ///
    /// A list of inputs for the monitoring job. Currently endpoints are supported as       monitoring inputs.
    ///
    /// Required: Yes
    ///
    /// Type: DataQualityJobInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataQualityJobInput")]
    pub data_quality_job_input: DataQualityJobInput,

    ///
    /// The output configuration for monitoring jobs.
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringOutputConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataQualityJobOutputConfig")]
    pub data_quality_job_output_config: MonitoringOutputConfig,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<cfn_resources::StrVal>,

    ///
    /// The name for the monitoring job definition.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<cfn_resources::StrVal>,

    ///
    /// Identifies the resources to deploy for a monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringResources
    ///
    /// Update requires: Replacement
    #[serde(rename = "JobResources")]
    pub job_resources: MonitoringResources,

    ///
    /// Specifies networking configuration for the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,

    ///
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to       perform tasks on your behalf.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// A time limit for how long the monitoring job is allowed to run before stopping.
    ///
    /// Required: No
    ///
    /// Type: StoppingCondition
    ///
    /// Update requires: Replacement
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<StoppingCondition>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnDataQualityJobDefinitioncreationtime,

    #[serde(skip_serializing)]
    pub att_job_definition_arn: CfnDataQualityJobDefinitionjobdefinitionarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDataQualityJobDefinitioncreationtime;
impl CfnDataQualityJobDefinitioncreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDataQualityJobDefinitionjobdefinitionarn;
impl CfnDataQualityJobDefinitionjobdefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"JobDefinitionArn"#
    }
}

impl cfn_resources::CfnResource for CfnDataQualityJobDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::DataQualityJobDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_quality_app_specification.validate()?;

        self.data_quality_baseline_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.data_quality_job_input.validate()?;

        self.data_quality_job_output_config.validate()?;

        self.job_resources.validate()?;

        self.network_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.stopping_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The BatchTransformInput property type specifies Property description not available. for an AWS::SageMaker::DataQualityJobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BatchTransformInput {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: DatasetFormat
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalPath")]
    pub local_path: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for BatchTransformInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dataset_format.validate()?;

        Ok(())
    }
}

/// The configuration for the cluster of resources used to run the processing job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ClusterConfig {
    ///
    /// The number of ML compute instances to use in the model monitoring job. For distributed       processing jobs, specify a value greater than 1. The default value is 1.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

    ///
    /// The ML compute instance type for the processing job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,

    ///
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute       instance(s) that run the model monitoring job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The size of the ML storage volume, in gigabytes, that you want to provision. You must       specify sufficient ML storage for your scenario.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: i64,
}

impl cfn_resources::CfnResource for ClusterConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The constraints resource for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConstraintsResource {
    ///
    /// The Amazon S3 URI for the constraints resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for ConstraintsResource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The Csv property type specifies Property description not available. for an AWS::SageMaker::DataQualityJobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Csv {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<bool>,
}

impl cfn_resources::CfnResource for Csv {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Information about the container that a data quality monitoring job runs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataQualityAppSpecification {
    ///
    /// The arguments to send to the container that the monitoring job runs.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arguments: Option<Vec<String>>,

    ///
    /// The entrypoint for a container used to run a monitoring job.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerEntrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_entrypoint: Option<Vec<String>>,

    ///
    /// Sets the environment variables in the container that the monitoring job runs.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<std::collections::HashMap<String, String>>,

    ///
    /// The container image that the data quality monitoring job runs.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ImageUri")]
    pub image_uri: cfn_resources::StrVal,

    ///
    /// An Amazon S3 URI to a script that is called after analysis has been performed.     Applicable only for the built-in (first party) containers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_analytics_processor_source_uri: Option<cfn_resources::StrVal>,

    ///
    /// An Amazon S3 URI to a script that is called per row prior to running analysis. It can     base64 decode the payload and convert it into a flatted json so that the built-in container     can use the converted data. Applicable only for the built-in (first party)     containers.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^(https|s3)://([^/]+)/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "RecordPreprocessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_preprocessor_source_uri: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DataQualityAppSpecification {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.container_arguments {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'container_arguments'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.container_entrypoint {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'container_entrypoint'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.image_uri;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'image_uri'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.post_analytics_processor_source_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'post_analytics_processor_source_uri'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.record_preprocessor_source_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'record_preprocessor_source_uri'. {} is greater than 1024", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Configuration for monitoring constraints and monitoring statistics. These baseline     resources are compared against the results of the current job from the series of jobs     scheduled to collect data periodically.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataQualityBaselineConfig {
    ///
    /// The name of the job that performs baselining for the data quality monitoring job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "BaseliningJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baselining_job_name: Option<cfn_resources::StrVal>,

    ///
    /// The constraints resource for a monitoring job.
    ///
    /// Required: No
    ///
    /// Type: ConstraintsResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConstraintsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints_resource: Option<ConstraintsResource>,

    ///
    /// Configuration for monitoring constraints and monitoring statistics. These baseline       resources are compared against the results of the current job from the series of jobs       scheduled to collect data periodically.
    ///
    /// Required: No
    ///
    /// Type: StatisticsResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "StatisticsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_resource: Option<StatisticsResource>,
}

impl cfn_resources::CfnResource for DataQualityBaselineConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.baselining_job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'baselining_job_name'. {} is greater than 63", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.baselining_job_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'baselining_job_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.constraints_resource
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.statistics_resource
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The input for the data quality monitoring job. Currently endpoints are supported for     input.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataQualityJobInput {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: BatchTransformInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "BatchTransformInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_transform_input: Option<BatchTransformInput>,

    ///
    /// Input object for the endpoint
    ///
    /// Required: No
    ///
    /// Type: EndpointInput
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_input: Option<EndpointInput>,
}

impl cfn_resources::CfnResource for DataQualityJobInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.batch_transform_input
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.endpoint_input
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The DatasetFormat property type specifies Property description not available. for an AWS::SageMaker::DataQualityJobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DatasetFormat {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Csv
    ///
    /// Update requires: Replacement
    #[serde(rename = "Csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<Csv>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Json>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parquet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet: Option<bool>,
}

impl cfn_resources::CfnResource for DatasetFormat {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.csv.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.json.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Input object for the endpoint
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EndpointInput {
    ///
    /// An endpoint in customer's account which has enabled DataCaptureConfig     enabled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: Replacement
    #[serde(rename = "EndpointName")]
    pub endpoint_name: cfn_resources::StrVal,

    ///
    /// Path to the filesystem where the endpoint data is available to the container.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalPath")]
    pub local_path: cfn_resources::StrVal,

    ///
    /// Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key.     Defaults to FullyReplicated
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FullyReplicated | ShardedByS3Key
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<EndpointInputS3DataDistributionTypeEnum>,

    ///
    /// Whether the Pipe or File is used as the input mode for     transferring data for the monitoring job. Pipe mode is recommended for large     datasets. File mode is useful for small files that fit in memory. Defaults to       File.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: File | Pipe
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_mode: Option<EndpointInputS3InputModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EndpointInputS3DataDistributionTypeEnum {
    /// FullyReplicated
    #[serde(rename = "FullyReplicated")]
    Fullyreplicated,

    /// ShardedByS3Key
    #[serde(rename = "ShardedByS3Key")]
    Shardedbys3key,
}

impl Default for EndpointInputS3DataDistributionTypeEnum {
    fn default() -> Self {
        EndpointInputS3DataDistributionTypeEnum::Fullyreplicated
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EndpointInputS3InputModeEnum {
    /// File
    #[serde(rename = "File")]
    File,

    /// Pipe
    #[serde(rename = "Pipe")]
    Pipe,
}

impl Default for EndpointInputS3InputModeEnum {
    fn default() -> Self {
        EndpointInputS3InputModeEnum::File
    }
}

impl cfn_resources::CfnResource for EndpointInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.endpoint_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.local_path;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'local_path'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The Json property type specifies Property description not available. for an AWS::SageMaker::DataQualityJobDefinition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Json {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<bool>,
}

impl cfn_resources::CfnResource for Json {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The output object for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MonitoringOutput {
    ///
    /// The Amazon S3 storage location where the results of a monitoring job are saved.
    ///
    /// Required: Yes
    ///
    /// Type: S3Output
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Output")]
    pub s3_output: S3Output,
}

impl cfn_resources::CfnResource for MonitoringOutput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_output.validate()?;

        Ok(())
    }
}

/// The output configuration for monitoring jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MonitoringOutputConfig {
    ///
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model     artifacts at rest using Amazon S3 server-side encryption.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Monitoring outputs for monitoring jobs. This is where the output of the periodic     monitoring jobs is uploaded.
    ///
    /// Required: Yes
    ///
    /// Type: List of MonitoringOutput
    ///
    /// Maximum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "MonitoringOutputs")]
    pub monitoring_outputs: Vec<MonitoringOutput>,
}

impl cfn_resources::CfnResource for MonitoringOutputConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.monitoring_outputs;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'monitoring_outputs'. {} is greater than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Identifies the resources to deploy for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MonitoringResources {
    ///
    /// The configuration for the cluster resources used to run the processing job.
    ///
    /// Required: Yes
    ///
    /// Type: ClusterConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterConfig")]
    pub cluster_config: ClusterConfig,
}

impl cfn_resources::CfnResource for MonitoringResources {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cluster_config.validate()?;

        Ok(())
    }
}

/// Networking options for a job, such as network traffic encryption between containers,     whether to allow inbound and outbound network calls to and from containers, and the VPC     subnets and security groups to use for VPC-enabled jobs.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    ///
    /// Whether to encrypt all communications between distributed processing jobs. Choose       True to encrypt communications. Encryption provides greater security for distributed       processing jobs, but the processing might take longer.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableInterContainerTrafficEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inter_container_traffic_encryption: Option<bool>,

    ///
    /// Whether to allow inbound and outbound network calls to and from the containers used for     the processing job.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,

    ///
    /// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC.
    ///
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

impl cfn_resources::CfnResource for NetworkConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.vpc_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Amazon S3 storage location where the results of a monitoring job are saved.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3Output {
    ///
    /// The local path to the Amazon S3 storage location where Amazon SageMaker saves the       results of a monitoring job. LocalPath is an absolute path for the output data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalPath")]
    pub local_path: cfn_resources::StrVal,

    ///
    /// Whether to upload the results of the monitoring job continuously or after the job       completes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3UploadMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_upload_mode: Option<cfn_resources::StrVal>,

    ///
    /// A URI that identifies the Amazon S3 storage location where Amazon SageMaker saves the       results of a monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for S3Output {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The statistics resource for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StatisticsResource {
    ///
    /// The Amazon S3 URI for the statistics resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for StatisticsResource {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies a limit to how long a model training job or model compilation job can run.       It also specifies how long a managed spot training job has to complete. When the job       reaches the time limit, SageMaker ends the training or compilation job. Use this API to cap       model training costs.
///
/// To stop a training job, SageMaker sends the algorithm the SIGTERM signal,       which delays job termination for 120 seconds. Algorithms can use this 120-second window       to save the model artifacts, so the results of training are not lost.
///
/// The training algorithms provided by SageMaker automatically save the intermediate results       of a model training job when possible. This attempt to save artifacts is only a best       effort case as model might not be in a state from which it can be saved. For example, if       training has just started, the model might not be ready to save. When saved, this       intermediate data is a valid model artifact. You can use it to create a model with         CreateModel.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StoppingCondition {
    ///
    /// The maximum length of time, in seconds, that a training or compilation job can run       before it is stopped.
    ///
    /// For compilation jobs, if the job does not complete during this time, a         TimeOut error is generated. We recommend starting with 900 seconds and       increasing as necessary based on your model.
    ///
    /// For all other jobs, if the job does not complete during this time, SageMaker ends the job.       When RetryStrategy is specified in the job request,         MaxRuntimeInSeconds specifies the maximum time for all of the attempts       in total, not each individual attempt. The default value is 1 day. The maximum value is       28 days.
    ///
    /// The maximum time that a TrainingJob can run in total, including any time       spent publishing metrics or archiving and uploading models after it has been stopped, is       30 days.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxRuntimeInSeconds")]
    pub max_runtime_in_seconds: i64,
}

impl cfn_resources::CfnResource for StoppingCondition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.max_runtime_in_seconds;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'max_runtime_in_seconds'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC. For more       information, see Protect Endpoints by Using an Amazon Virtual Private Cloud and Protect Training Jobs         by Using an Amazon Virtual Private Cloud.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VpcConfig {
    ///
    /// The VPC security group IDs, in the form sg-xxxxxxxx. Specify the security groups for       the VPC that is specified in the Subnets field.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

    ///
    /// The ID of the subnets in the VPC to which you want to connect your training job or       model. For information about the availability of specific instance types, see Supported         Instance Types and Availability Zones.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 16
    ///
    /// Update requires: Replacement
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

impl cfn_resources::CfnResource for VpcConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.security_group_ids;

        if the_val.len() > 5 as _ {
            return Err(format!(
                "Max validation failed on field 'security_group_ids'. {} is greater than 5",
                the_val.len()
            ));
        }

        let the_val = &self.subnets;

        if the_val.len() > 16 as _ {
            return Err(format!(
                "Max validation failed on field 'subnets'. {} is greater than 16",
                the_val.len()
            ));
        }

        Ok(())
    }
}
