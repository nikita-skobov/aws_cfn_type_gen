/// The AWS::SageMaker::MonitoringSchedule resource is an Amazon SageMaker       resource type that regularly starts SageMaker processing Jobs to monitor the data       captured for a SageMaker endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMonitoringSchedule {
    ///
    /// The name of the endpoint using the monitoring schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,

    ///
    /// Contains the reason a monitoring job failed, if it failed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    ///
    /// Describes metadata on the last execution to run, if there was one.
    ///
    /// Required: No
    ///
    /// Type: MonitoringExecutionSummary
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastMonitoringExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_monitoring_execution_summary: Option<MonitoringExecutionSummary>,

    ///
    /// The configuration object that specifies the monitoring schedule and defines the       monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringScheduleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleConfig,

    ///
    /// The name of the monitoring schedule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,

    ///
    /// The status of the monitoring schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Failed | Pending | Scheduled | Stopped
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringScheduleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_schedule_status: Option<MonitoringScheduleMonitoringScheduleStatusEnum>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringScheduleMonitoringScheduleStatusEnum {
    /// Failed
    #[serde(rename = "Failed")]
    Failed,

    /// Pending
    #[serde(rename = "Pending")]
    Pending,

    /// Scheduled
    #[serde(rename = "Scheduled")]
    Scheduled,

    /// Stopped
    #[serde(rename = "Stopped")]
    Stopped,
}

impl Default for MonitoringScheduleMonitoringScheduleStatusEnum {
    fn default() -> Self {
        MonitoringScheduleMonitoringScheduleStatusEnum::Failed
    }
}

impl cfn_resources::CfnResource for CfnMonitoringSchedule {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::MonitoringSchedule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.endpoint_name {
            if the_val.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    the_val.len()
                ));
            }
        }

        self.last_monitoring_execution_summary
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.monitoring_schedule_config.validate()?;

        let the_val = &self.monitoring_schedule_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'monitoring_schedule_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.monitoring_schedule_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'monitoring_schedule_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Baseline configuration used to validate that the data conforms to the specified       constraints and statistics.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BaselineConfig {
    ///
    /// The Amazon S3 URI for the constraints resource.
    ///
    /// Required: No
    ///
    /// Type: ConstraintsResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConstraintsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints_resource: Option<ConstraintsResource>,

    ///
    /// The baseline statistics file in Amazon S3 that the current monitoring job should be       validated against.
    ///
    /// Required: No
    ///
    /// Type: StatisticsResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatisticsResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_resource: Option<StatisticsResource>,
}

impl cfn_resources::CfnResource for BaselineConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.constraints_resource
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.statistics_resource
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The BatchTransformInput property type specifies Property description not available. for an AWS::SageMaker::MonitoringSchedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BatchTransformInput {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataCapturedDestinationS3Uri")]
    pub data_captured_destination_s3_uri: String,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: DatasetFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatasetFormat")]
    pub dataset_format: DatasetFormat,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalPath")]
    pub local_path: String,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3DataDistributionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_distribution_type: Option<String>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_mode: Option<String>,
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

/// Configuration for the cluster used to run model monitoring jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClusterConfig {
    ///
    /// The number of ML compute instances to use in the model monitoring job. For distributed       processing jobs, specify a value greater than 1. The default value is 1.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

    ///
    /// The ML compute instance type for the processing job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: String,

    ///
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt data on the storage volume attached to the ML compute       instance(s) that run the model monitoring job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,

    ///
    /// The size of the ML storage volume, in gigabytes, that you want to provision. You must       specify sufficient ML storage for your scenario.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
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

/// The Amazon S3 URI for the constraints resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConstraintsResource {
    ///
    /// The Amazon S3 URI for the constraints resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
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

/// The Csv property type specifies Property description not available. for an AWS::SageMaker::MonitoringSchedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Csv {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
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

/// The DatasetFormat property type specifies Property description not available. for an AWS::SageMaker::MonitoringSchedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatasetFormat {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Csv
    ///
    /// Update requires: No interruption
    #[serde(rename = "Csv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<Csv>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Json>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    pub endpoint_name: String,

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
    /// Update requires: No interruption
    #[serde(rename = "LocalPath")]
    pub local_path: String,

    ///
    /// Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key.     Defaults to FullyReplicated
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FullyReplicated | ShardedByS3Key
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "S3InputMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_mode: Option<EndpointInputS3InputModeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.local_path;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'local_path'. {} is greater than 256",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The Json property type specifies Property description not available. for an AWS::SageMaker::MonitoringSchedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Json {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
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

/// Container image configuration object for the monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringAppSpecification {
    ///
    /// An array of arguments for the container used to run the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arguments: Option<Vec<String>>,

    ///
    /// Specifies the entrypoint for a container used to run the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContainerEntrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_entrypoint: Option<Vec<String>>,

    ///
    /// The container image to be run by the monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageUri")]
    pub image_uri: String,

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
    /// Update requires: No interruption
    #[serde(rename = "PostAnalyticsProcessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_analytics_processor_source_uri: Option<String>,

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
    /// Update requires: No interruption
    #[serde(rename = "RecordPreprocessorSourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_preprocessor_source_uri: Option<String>,
}

impl cfn_resources::CfnResource for MonitoringAppSpecification {
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

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'image_uri'. {} is greater than 255",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.post_analytics_processor_source_uri {
            if the_val.len() > 1024 as _ {
                return Err(format!("Max validation failed on field 'post_analytics_processor_source_uri'. {} is greater than 1024", the_val.len()));
            }
        }

        if let Some(the_val) = &self.record_preprocessor_source_uri {
            if the_val.len() > 1024 as _ {
                return Err(format!("Max validation failed on field 'record_preprocessor_source_uri'. {} is greater than 1024", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Summary of information about the last monitoring job to run.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringExecutionSummary {
    ///
    /// The time at which the monitoring job was created.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreationTime")]
    pub creation_time: String,

    ///
    /// The name of the endpoint used to run the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,

    ///
    /// Contains the reason a monitoring job failed, if it failed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    ///
    /// A timestamp that indicates the last time the monitoring job was modified.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: String,

    ///
    /// The status of the monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Completed | CompletedWithViolations | Failed | InProgress | Pending | Stopped | Stopping
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringExecutionStatus")]
    pub monitoring_execution_status: MonitoringExecutionSummaryMonitoringExecutionStatusEnum,

    ///
    /// The name of the monitoring schedule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringScheduleName")]
    pub monitoring_schedule_name: String,

    ///
    /// The Amazon Resource Name (ARN) of the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws[a-z\-]*:sagemaker:[a-z0-9\-]*:[0-9]{12}:processing-job/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProcessingJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_job_arn: Option<String>,

    ///
    /// The time the monitoring job was scheduled.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduledTime")]
    pub scheduled_time: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringExecutionSummaryMonitoringExecutionStatusEnum {
    /// Completed
    #[serde(rename = "Completed")]
    Completed,

    /// CompletedWithViolations
    #[serde(rename = "CompletedWithViolations")]
    Completedwithviolations,

    /// Failed
    #[serde(rename = "Failed")]
    Failed,

    /// InProgress
    #[serde(rename = "InProgress")]
    Inprogress,

    /// Pending
    #[serde(rename = "Pending")]
    Pending,

    /// Stopped
    #[serde(rename = "Stopped")]
    Stopped,

    /// Stopping
    #[serde(rename = "Stopping")]
    Stopping,
}

impl Default for MonitoringExecutionSummaryMonitoringExecutionStatusEnum {
    fn default() -> Self {
        MonitoringExecutionSummaryMonitoringExecutionStatusEnum::Completed
    }
}

impl cfn_resources::CfnResource for MonitoringExecutionSummary {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.endpoint_name {
            if the_val.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.failure_reason {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'failure_reason'. {} is greater than 1024",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.monitoring_schedule_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'monitoring_schedule_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.monitoring_schedule_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'monitoring_schedule_name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.processing_job_arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'processing_job_arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The inputs for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringInput {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: BatchTransformInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "BatchTransformInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_transform_input: Option<BatchTransformInput>,

    ///
    /// The endpoint for a monitoring job.
    ///
    /// Required: No
    ///
    /// Type: EndpointInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_input: Option<EndpointInput>,
}

impl cfn_resources::CfnResource for MonitoringInput {
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

/// Defines the monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringJobDefinition {
    ///
    /// Baseline configuration used to validate that the data conforms to the specified     constraints and statistics
    ///
    /// Required: No
    ///
    /// Type: BaselineConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaselineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_config: Option<BaselineConfig>,

    ///
    /// Sets the environment variables in the Docker container.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<std::collections::HashMap<String, String>>,

    ///
    /// Configures the monitoring job to run a specified Docker container image.
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringAppSpecification
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringAppSpecification")]
    pub monitoring_app_specification: MonitoringAppSpecification,

    ///
    /// The array of inputs for the monitoring job. Currently we support monitoring an Amazon SageMaker     Endpoint.
    ///
    /// Required: Yes
    ///
    /// Type: List of MonitoringInput
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringInputs")]
    pub monitoring_inputs: Vec<MonitoringInput>,

    ///
    /// The array of outputs from the monitoring job to be uploaded to Amazon Simple Storage     Service (Amazon S3).
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringOutputConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringOutputConfig")]
    pub monitoring_output_config: MonitoringOutputConfig,

    ///
    /// Identifies the resources, ML compute instances, and ML storage volumes to deploy for a     monitoring job. In distributed processing, you specify more than one instance.
    ///
    /// Required: Yes
    ///
    /// Type: MonitoringResources
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringResources")]
    pub monitoring_resources: MonitoringResources,

    ///
    /// Specifies networking options for an monitoring job.
    ///
    /// Required: No
    ///
    /// Type: NetworkConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_config: Option<NetworkConfig>,

    ///
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume to perform tasks on     your behalf.
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
    /// Specifies a time limit for how long the monitoring job is allowed to run.
    ///
    /// Required: No
    ///
    /// Type: StoppingCondition
    ///
    /// Update requires: No interruption
    #[serde(rename = "StoppingCondition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_condition: Option<StoppingCondition>,
}

impl cfn_resources::CfnResource for MonitoringJobDefinition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.baseline_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.monitoring_app_specification.validate()?;

        let the_val = &self.monitoring_inputs;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'monitoring_inputs'. {} is greater than 1",
                the_val.len()
            ));
        }

        self.monitoring_output_config.validate()?;

        self.monitoring_resources.validate()?;

        self.network_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.role_arn;

        if the_val.len() > 2048 as _ {
            return Err(format!(
                "Max validation failed on field 'role_arn'. {} is greater than 2048",
                the_val.len()
            ));
        }

        let the_val = &self.role_arn;

        if the_val.len() < 20 as _ {
            return Err(format!(
                "Min validation failed on field 'role_arn'. {} is less than 20",
                the_val.len()
            ));
        }

        self.stopping_condition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The output object for a monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringOutput {
    ///
    /// The Amazon S3 storage location where the results of a monitoring job are saved.
    ///
    /// Required: Yes
    ///
    /// Type: S3Output
    ///
    /// Update requires: No interruption
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,

    ///
    /// Monitoring outputs for monitoring jobs. This is where the output of the periodic     monitoring jobs is uploaded.
    ///
    /// Required: Yes
    ///
    /// Type: List of MonitoringOutput
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
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
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                    the_val.len()
                ));
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
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringResources {
    ///
    /// The configuration for the cluster resources used to run the processing job.
    ///
    /// Required: Yes
    ///
    /// Type: ClusterConfig
    ///
    /// Update requires: No interruption
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

/// Configures the monitoring schedule and defines the monitoring job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MonitoringScheduleConfig {
    ///
    /// Defines the monitoring job.
    ///
    /// Required: No
    ///
    /// Type: MonitoringJobDefinition
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringJobDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_job_definition: Option<MonitoringJobDefinition>,

    ///
    /// The name of the monitoring job definition to schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringJobDefinitionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_job_definition_name: Option<String>,

    ///
    /// The type of the monitoring job definition to schedule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DataQuality | ModelBias | ModelExplainability | ModelQuality
    ///
    /// Update requires: No interruption
    #[serde(rename = "MonitoringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_type: Option<MonitoringScheduleConfigMonitoringTypeEnum>,

    ///
    /// Configures the monitoring schedule.
    ///
    /// Required: No
    ///
    /// Type: ScheduleConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfig>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MonitoringScheduleConfigMonitoringTypeEnum {
    /// DataQuality
    #[serde(rename = "DataQuality")]
    Dataquality,

    /// ModelBias
    #[serde(rename = "ModelBias")]
    Modelbias,

    /// ModelExplainability
    #[serde(rename = "ModelExplainability")]
    Modelexplainability,

    /// ModelQuality
    #[serde(rename = "ModelQuality")]
    Modelquality,
}

impl Default for MonitoringScheduleConfigMonitoringTypeEnum {
    fn default() -> Self {
        MonitoringScheduleConfigMonitoringTypeEnum::Dataquality
    }
}

impl cfn_resources::CfnResource for MonitoringScheduleConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.monitoring_job_definition
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.monitoring_job_definition_name {
            if the_val.len() > 63 as _ {
                return Err(format!("Max validation failed on field 'monitoring_job_definition_name'. {} is greater than 63", the_val.len()));
            }
        }

        if let Some(the_val) = &self.monitoring_job_definition_name {
            if the_val.len() < 1 as _ {
                return Err(format!("Min validation failed on field 'monitoring_job_definition_name'. {} is less than 1", the_val.len()));
            }
        }

        self.schedule_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Networking options for a job, such as network traffic encryption between containers,     whether to allow inbound and outbound network calls to and from containers, and the VPC     subnets and security groups to use for VPC-enabled jobs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct NetworkConfig {
    ///
    /// Whether to encrypt all communications between distributed processing jobs. Choose       True to encrypt communications. Encryption provides greater security for distributed       processing jobs, but the processing might take longer.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
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
    /// Update requires: No interruption
    #[serde(rename = "EnableNetworkIsolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_isolation: Option<bool>,

    ///
    /// Specifies a VPC that your training jobs and hosted models have access to. Control       access to and from your training and model containers by configuring the VPC. For more       information, see Protect Endpoints by Using an Amazon Virtual Private Cloud and Protect Training Jobs         by Using an Amazon Virtual Private Cloud.
    ///
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
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

/// Information about where and how you want to store the results of a monitoring       job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Output {
    ///
    /// The local path to the S3 storage location where SageMaker saves the results of a       monitoring job. LocalPath is an absolute path for the output data.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalPath")]
    pub local_path: String,

    ///
    /// Whether to upload the results of the monitoring job continuously or after the job       completes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3UploadMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_upload_mode: Option<String>,

    ///
    /// A URI that identifies the S3 storage location where SageMaker saves the results of a       monitoring job.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
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

/// Configuration details about the monitoring schedule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScheduleConfig {
    ///
    /// A cron expression that describes details about the monitoring schedule.
    ///
    /// Currently the only supported cron expressions are:
    ///
    /// If you want to set the job to start every hour, please use the following:                  Hourly: cron(0 * ? * * *)                       If you want to start the job daily:                  cron(0 [00-23] ? * * *)
    ///
    /// For example, the following are valid cron expressions:
    ///
    /// Daily at noon UTC: cron(0 12 ? * * *)                       Daily at midnight UTC: cron(0 0 ? * * *)
    ///
    /// To support running every 6, 12 hours, the following are also supported:
    ///
    /// cron(0 [00-23]/[01-24] ? * * *)
    ///
    /// For example, the following are valid cron expressions:
    ///
    /// Every 12 hours, starting at 5pm UTC: cron(0 17/12 ? * * *)                       Every two hours starting at midnight: cron(0 0/2 ? * * *)
    ///
    /// Note                                 Even though the cron expression is set to start at 5PM UTC, note that there          could be a delay of 0-20 minutes from the actual requested time to run the          execution.                  We recommend that if you would like a daily schedule, you do not provide this          parameter. Amazon SageMaker will pick a time for running every day.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,
}

impl cfn_resources::CfnResource for ScheduleConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.schedule_expression;

        if the_val.len() > 256 as _ {
            return Err(format!(
                "Max validation failed on field 'schedule_expression'. {} is greater than 256",
                the_val.len()
            ));
        }

        let the_val = &self.schedule_expression;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'schedule_expression'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The baseline statistics file in Amazon S3 that the current monitoring job should be       validated against.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct StatisticsResource {
    ///
    /// The S3 URI for the statistics resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Update requires: No interruption
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Update requires: No interruption
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
    /// Update requires: No interruption
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
