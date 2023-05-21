/// Creates an inference experiment using the configurations specified in the request.
///
/// Use this API to setup and schedule an experiment to compare model variants on a Amazon SageMaker inference endpoint. For      more information about inference experiments, see Shadow tests.
///
/// Amazon SageMaker begins your experiment at the scheduled time and routes traffic to your endpoint's model variants based      on your specified configuration.
///
/// While the experiment is in progress or after it has concluded, you can view metrics that compare your model      variants. For more information, see View, monitor, and edit shadow tests.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnInferenceExperiment {
    ///
    /// The Amazon S3 location and configuration for storing inference request and response data.
    ///
    /// Required: No
    ///
    /// Type: DataStorageConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataStorageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage_config: Option<DataStorageConfig>,

    ///
    /// The description of the inference experiment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The desired state of the experiment after stopping. The possible states are the following:
    ///
    /// Completed: The experiment completed successfully                        Cancelled: The experiment was canceled
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Cancelled | Completed
    ///
    /// Update requires: No interruption
    #[serde(rename = "DesiredState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<InferenceExperimentDesiredStateEnum>,

    ///
    /// The name of the endpoint.
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
    /// The AWS Key Management Service key that Amazon SageMaker uses to encrypt captured data at rest using Amazon S3      server-side encryption.
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
    #[serde(rename = "KmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<cfn_resources::StrVal>,

    ///
    /// An array of ModelVariantConfigSummary objects. There is one for each variant in the inference      experiment. Each ModelVariantConfigSummary object in the array describes the infrastructure      configuration for deploying the corresponding variant.
    ///
    /// Required: Yes
    ///
    /// Type: List of ModelVariantConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelVariants")]
    pub model_variants: Vec<ModelVariantConfig>,

    ///
    /// The name of the inference experiment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 120
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,119}
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The ARN of the IAM role that Amazon SageMaker can assume to access model artifacts and container images, and manage      Amazon SageMaker Inference endpoints for model deployment.
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
    /// Update requires: Replacement
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// The duration for which the inference experiment ran or will run.
    ///
    /// The maximum duration that you can set for an inference experiment is 30 days.
    ///
    /// Required: No
    ///
    /// Type: InferenceExperimentSchedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<InferenceExperimentSchedule>,

    ///
    /// The configuration of ShadowMode inference experiment type, which shows the production variant      that takes all the inference requests, and the shadow variant to which Amazon SageMaker replicates a percentage of the      inference requests. For the shadow variant it also shows the percentage of requests that Amazon SageMaker replicates.
    ///
    /// Required: No
    ///
    /// Type: ShadowModeConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_mode_config: Option<ShadowModeConfig>,

    ///
    /// The error message for the inference experiment status result.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<cfn_resources::StrVal>,

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

    ///
    /// The type of the inference experiment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ShadowMode
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: InferenceExperimentTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InferenceExperimentDesiredStateEnum {
    /// Cancelled
    #[serde(rename = "Cancelled")]
    Cancelled,

    /// Completed
    #[serde(rename = "Completed")]
    Completed,
}

impl Default for InferenceExperimentDesiredStateEnum {
    fn default() -> Self {
        InferenceExperimentDesiredStateEnum::Cancelled
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum InferenceExperimentTypeEnum {
    /// ShadowMode
    #[serde(rename = "ShadowMode")]
    Shadowmode,
}

impl Default for InferenceExperimentTypeEnum {
    fn default() -> Self {
        InferenceExperimentTypeEnum::Shadowmode
    }
}

impl cfn_resources::CfnResource for CfnInferenceExperiment {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::InferenceExperiment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_storage_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.endpoint_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.kms_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 120 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 120",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        self.schedule
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.shadow_mode_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.status_reason {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'status_reason'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Configuration specifying how to treat different headers. If no headers are specified SageMaker      will by default base64 encode when capturing the data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptureContentTypeHeader {
    ///
    /// The list of all content type headers that SageMaker will treat as CSV and capture accordingly.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "CsvContentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_content_types: Option<Vec<String>>,

    ///
    /// The list of all content type headers that SageMaker will treat as JSON and capture accordingly.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "JsonContentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_content_types: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CaptureContentTypeHeader {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.csv_content_types {
            if the_val.len() > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'csv_content_types'. {} is greater than 10",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.json_content_types {
            if the_val.len() > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'json_content_types'. {} is greater than 10",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The Amazon S3 location and configuration for storing inference request and response data.
///
/// This is an optional parameter that you can use for data capture. For more information, see Capture data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataStorageConfig {
    ///
    /// Configuration specifying how to treat different headers. If no headers are specified SageMaker will by default base64 encode when capturing the data.
    ///
    /// Required: No
    ///
    /// Type: CaptureContentTypeHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "ContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CaptureContentTypeHeader>,

    ///
    /// The Amazon S3 bucket where the inference request and response data is stored.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^(https|s3)://([^/])/?(.*)$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: cfn_resources::StrVal,

    ///
    /// The AWS Key Management Service key that Amazon SageMaker uses to encrypt captured data at rest using Amazon S3      server-side encryption.
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
    #[serde(rename = "KmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DataStorageConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.content_type
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.destination;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.kms_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The metadata of the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EndpointMetadata {
    ///
    /// The name of the endpoint configuration.
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
    #[serde(rename = "EndpointConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_config_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the endpoint.
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
    pub endpoint_name: cfn_resources::StrVal,

    ///
    /// The status of the endpoint. For possible values of the status of an endpoint, see       https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-endpointmetadata.html#cfn-sagemaker-inferenceexperiment-endpointmetadata-endpointstatus.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Creating | Deleting | Failed | InService | OutOfService | RollingBack | SystemUpdating | Updating
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndpointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<EndpointMetadataEndpointStatusEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EndpointMetadataEndpointStatusEnum {
    /// Creating
    #[serde(rename = "Creating")]
    Creating,

    /// Deleting
    #[serde(rename = "Deleting")]
    Deleting,

    /// Failed
    #[serde(rename = "Failed")]
    Failed,

    /// InService
    #[serde(rename = "InService")]
    Inservice,

    /// OutOfService
    #[serde(rename = "OutOfService")]
    Outofservice,

    /// RollingBack
    #[serde(rename = "RollingBack")]
    Rollingback,

    /// SystemUpdating
    #[serde(rename = "SystemUpdating")]
    Systemupdating,

    /// Updating
    #[serde(rename = "Updating")]
    Updating,
}

impl Default for EndpointMetadataEndpointStatusEnum {
    fn default() -> Self {
        EndpointMetadataEndpointStatusEnum::Creating
    }
}

impl cfn_resources::CfnResource for EndpointMetadata {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.endpoint_config_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'endpoint_config_name'. {} is greater than 63", s.len()));
                }
            }
        }

        let the_val = &self.endpoint_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'endpoint_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The start and end times of an inference experiment.
///
/// The maximum duration that you can set for an inference experiment is 30 days.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InferenceExperimentSchedule {
    ///
    /// The timestamp at which the inference experiment ended or will end.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<cfn_resources::StrVal>,

    ///
    /// The timestamp at which the inference experiment started or will start.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for InferenceExperimentSchedule {
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

/// The configuration for the infrastructure that the model will be deployed to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelInfrastructureConfig {
    ///
    /// The inference option to which to deploy your model. Possible values are the following:
    ///
    /// RealTime: Deploy to real-time inference.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: RealTimeInference
    ///
    /// Update requires: No interruption
    #[serde(rename = "InfrastructureType")]
    pub infrastructure_type: ModelInfrastructureConfigInfrastructureTypeEnum,

    ///
    /// The infrastructure configuration for deploying the model to real-time inference.
    ///
    /// Required: Yes
    ///
    /// Type: RealTimeInferenceConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RealTimeInferenceConfig")]
    pub real_time_inference_config: RealTimeInferenceConfig,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ModelInfrastructureConfigInfrastructureTypeEnum {
    /// RealTimeInference
    #[serde(rename = "RealTimeInference")]
    Realtimeinference,
}

impl Default for ModelInfrastructureConfigInfrastructureTypeEnum {
    fn default() -> Self {
        ModelInfrastructureConfigInfrastructureTypeEnum::Realtimeinference
    }
}

impl cfn_resources::CfnResource for ModelInfrastructureConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.real_time_inference_config.validate()?;

        Ok(())
    }
}

/// Contains information about the deployment options of a model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ModelVariantConfig {
    ///
    /// The configuration for the infrastructure that the model will be deployed to.
    ///
    /// Required: Yes
    ///
    /// Type: ModelInfrastructureConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "InfrastructureConfig")]
    pub infrastructure_config: ModelInfrastructureConfig,

    ///
    /// The name of the Amazon SageMaker Model entity.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelName")]
    pub model_name: cfn_resources::StrVal,

    ///
    /// The name of the variant.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "VariantName")]
    pub variant_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ModelVariantConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.infrastructure_config.validate()?;

        let the_val = &self.model_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'model_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.variant_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'variant_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The infrastructure configuration for deploying the model to a real-time inference endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RealTimeInferenceConfig {
    ///
    /// The number of instances of the type specified by InstanceType.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceCount")]
    pub instance_count: i64,

    ///
    /// The instance type the model is deployed to.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.xlarge | ml.c5d.18xlarge | ml.c5d.2xlarge | ml.c5d.4xlarge | ml.c5d.9xlarge | ml.c5d.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.16xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.8xlarge | ml.m5d.large | ml.m5d.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p3dn.24xlarge | ml.r5.12xlarge | ml.r5.16xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.8xlarge | ml.r5.large | ml.r5.xlarge | ml.t2.2xlarge | ml.t2.large | ml.t2.medium | ml.t2.xlarge | ml.t3.2xlarge | ml.t3.large | ml.t3.medium | ml.t3.xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: RealTimeInferenceConfigInstanceTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RealTimeInferenceConfigInstanceTypeEnum {
    /// ml.c4.2xlarge
    #[serde(rename = "ml.c4.2xlarge")]
    Mlc42xlarge,

    /// ml.c4.4xlarge
    #[serde(rename = "ml.c4.4xlarge")]
    Mlc44xlarge,

    /// ml.c4.8xlarge
    #[serde(rename = "ml.c4.8xlarge")]
    Mlc48xlarge,

    /// ml.c4.xlarge
    #[serde(rename = "ml.c4.xlarge")]
    Mlc4xlarge,

    /// ml.c5.18xlarge
    #[serde(rename = "ml.c5.18xlarge")]
    Mlc518xlarge,

    /// ml.c5.2xlarge
    #[serde(rename = "ml.c5.2xlarge")]
    Mlc52xlarge,

    /// ml.c5.4xlarge
    #[serde(rename = "ml.c5.4xlarge")]
    Mlc54xlarge,

    /// ml.c5.9xlarge
    #[serde(rename = "ml.c5.9xlarge")]
    Mlc59xlarge,

    /// ml.c5.xlarge
    #[serde(rename = "ml.c5.xlarge")]
    Mlc5xlarge,

    /// ml.c5d.18xlarge
    #[serde(rename = "ml.c5d.18xlarge")]
    Mlc5d18xlarge,

    /// ml.c5d.2xlarge
    #[serde(rename = "ml.c5d.2xlarge")]
    Mlc5d2xlarge,

    /// ml.c5d.4xlarge
    #[serde(rename = "ml.c5d.4xlarge")]
    Mlc5d4xlarge,

    /// ml.c5d.9xlarge
    #[serde(rename = "ml.c5d.9xlarge")]
    Mlc5d9xlarge,

    /// ml.c5d.xlarge
    #[serde(rename = "ml.c5d.xlarge")]
    Mlc5dxlarge,

    /// ml.g4dn.12xlarge
    #[serde(rename = "ml.g4dn.12xlarge")]
    Mlg4dn12xlarge,

    /// ml.g4dn.16xlarge
    #[serde(rename = "ml.g4dn.16xlarge")]
    Mlg4dn16xlarge,

    /// ml.g4dn.2xlarge
    #[serde(rename = "ml.g4dn.2xlarge")]
    Mlg4dn2xlarge,

    /// ml.g4dn.4xlarge
    #[serde(rename = "ml.g4dn.4xlarge")]
    Mlg4dn4xlarge,

    /// ml.g4dn.8xlarge
    #[serde(rename = "ml.g4dn.8xlarge")]
    Mlg4dn8xlarge,

    /// ml.g4dn.xlarge
    #[serde(rename = "ml.g4dn.xlarge")]
    Mlg4dnxlarge,

    /// ml.g5.12xlarge
    #[serde(rename = "ml.g5.12xlarge")]
    Mlg512xlarge,

    /// ml.g5.16xlarge
    #[serde(rename = "ml.g5.16xlarge")]
    Mlg516xlarge,

    /// ml.g5.24xlarge
    #[serde(rename = "ml.g5.24xlarge")]
    Mlg524xlarge,

    /// ml.g5.2xlarge
    #[serde(rename = "ml.g5.2xlarge")]
    Mlg52xlarge,

    /// ml.g5.48xlarge
    #[serde(rename = "ml.g5.48xlarge")]
    Mlg548xlarge,

    /// ml.g5.4xlarge
    #[serde(rename = "ml.g5.4xlarge")]
    Mlg54xlarge,

    /// ml.g5.8xlarge
    #[serde(rename = "ml.g5.8xlarge")]
    Mlg58xlarge,

    /// ml.g5.xlarge
    #[serde(rename = "ml.g5.xlarge")]
    Mlg5xlarge,

    /// ml.m4.10xlarge
    #[serde(rename = "ml.m4.10xlarge")]
    Mlm410xlarge,

    /// ml.m4.16xlarge
    #[serde(rename = "ml.m4.16xlarge")]
    Mlm416xlarge,

    /// ml.m4.2xlarge
    #[serde(rename = "ml.m4.2xlarge")]
    Mlm42xlarge,

    /// ml.m4.4xlarge
    #[serde(rename = "ml.m4.4xlarge")]
    Mlm44xlarge,

    /// ml.m4.xlarge
    #[serde(rename = "ml.m4.xlarge")]
    Mlm4xlarge,

    /// ml.m5.12xlarge
    #[serde(rename = "ml.m5.12xlarge")]
    Mlm512xlarge,

    /// ml.m5.24xlarge
    #[serde(rename = "ml.m5.24xlarge")]
    Mlm524xlarge,

    /// ml.m5.2xlarge
    #[serde(rename = "ml.m5.2xlarge")]
    Mlm52xlarge,

    /// ml.m5.4xlarge
    #[serde(rename = "ml.m5.4xlarge")]
    Mlm54xlarge,

    /// ml.m5.xlarge
    #[serde(rename = "ml.m5.xlarge")]
    Mlm5xlarge,

    /// ml.m5d.12xlarge
    #[serde(rename = "ml.m5d.12xlarge")]
    Mlm5d12xlarge,

    /// ml.m5d.16xlarge
    #[serde(rename = "ml.m5d.16xlarge")]
    Mlm5d16xlarge,

    /// ml.m5d.24xlarge
    #[serde(rename = "ml.m5d.24xlarge")]
    Mlm5d24xlarge,

    /// ml.m5d.2xlarge
    #[serde(rename = "ml.m5d.2xlarge")]
    Mlm5d2xlarge,

    /// ml.m5d.4xlarge
    #[serde(rename = "ml.m5d.4xlarge")]
    Mlm5d4xlarge,

    /// ml.m5d.8xlarge
    #[serde(rename = "ml.m5d.8xlarge")]
    Mlm5d8xlarge,

    /// ml.m5d.large
    #[serde(rename = "ml.m5d.large")]
    Mlm5dlarge,

    /// ml.m5d.xlarge
    #[serde(rename = "ml.m5d.xlarge")]
    Mlm5dxlarge,

    /// ml.p2.16xlarge
    #[serde(rename = "ml.p2.16xlarge")]
    Mlp216xlarge,

    /// ml.p2.8xlarge
    #[serde(rename = "ml.p2.8xlarge")]
    Mlp28xlarge,

    /// ml.p2.xlarge
    #[serde(rename = "ml.p2.xlarge")]
    Mlp2xlarge,

    /// ml.p3.16xlarge
    #[serde(rename = "ml.p3.16xlarge")]
    Mlp316xlarge,

    /// ml.p3.2xlarge
    #[serde(rename = "ml.p3.2xlarge")]
    Mlp32xlarge,

    /// ml.p3.8xlarge
    #[serde(rename = "ml.p3.8xlarge")]
    Mlp38xlarge,

    /// ml.p3dn.24xlarge
    #[serde(rename = "ml.p3dn.24xlarge")]
    Mlp3dn24xlarge,

    /// ml.r5.12xlarge
    #[serde(rename = "ml.r5.12xlarge")]
    Mlr512xlarge,

    /// ml.r5.16xlarge
    #[serde(rename = "ml.r5.16xlarge")]
    Mlr516xlarge,

    /// ml.r5.24xlarge
    #[serde(rename = "ml.r5.24xlarge")]
    Mlr524xlarge,

    /// ml.r5.2xlarge
    #[serde(rename = "ml.r5.2xlarge")]
    Mlr52xlarge,

    /// ml.r5.4xlarge
    #[serde(rename = "ml.r5.4xlarge")]
    Mlr54xlarge,

    /// ml.r5.8xlarge
    #[serde(rename = "ml.r5.8xlarge")]
    Mlr58xlarge,

    /// ml.r5.large
    #[serde(rename = "ml.r5.large")]
    Mlr5large,

    /// ml.r5.xlarge
    #[serde(rename = "ml.r5.xlarge")]
    Mlr5xlarge,

    /// ml.t2.2xlarge
    #[serde(rename = "ml.t2.2xlarge")]
    Mlt22xlarge,

    /// ml.t2.large
    #[serde(rename = "ml.t2.large")]
    Mlt2large,

    /// ml.t2.medium
    #[serde(rename = "ml.t2.medium")]
    Mlt2medium,

    /// ml.t2.xlarge
    #[serde(rename = "ml.t2.xlarge")]
    Mlt2xlarge,

    /// ml.t3.2xlarge
    #[serde(rename = "ml.t3.2xlarge")]
    Mlt32xlarge,

    /// ml.t3.large
    #[serde(rename = "ml.t3.large")]
    Mlt3large,

    /// ml.t3.medium
    #[serde(rename = "ml.t3.medium")]
    Mlt3medium,

    /// ml.t3.xlarge
    #[serde(rename = "ml.t3.xlarge")]
    Mlt3xlarge,
}

impl Default for RealTimeInferenceConfigInstanceTypeEnum {
    fn default() -> Self {
        RealTimeInferenceConfigInstanceTypeEnum::Mlc42xlarge
    }
}

impl cfn_resources::CfnResource for RealTimeInferenceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_count;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'instance_count'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// The configuration of ShadowMode inference experiment type, which specifies a production variant      to take all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the      inference requests. For the shadow variant it also specifies the percentage of requests that Amazon SageMaker replicates.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShadowModeConfig {
    ///
    /// List of shadow variant configurations.
    ///
    /// Required: Yes
    ///
    /// Type: List of ShadowModelVariantConfig
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModelVariants")]
    pub shadow_model_variants: Vec<ShadowModelVariantConfig>,

    ///
    /// The name of the production variant, which takes all the inference requests.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceModelVariantName")]
    pub source_model_variant_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ShadowModeConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.shadow_model_variants;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'shadow_model_variants'. {} is greater than 1",
                the_val.len()
            ));
        }

        let the_val = &self.source_model_variant_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!("Max validation failed on field 'source_model_variant_name'. {} is greater than 63", s.len()));
            }
        }

        Ok(())
    }
}

/// The name and sampling percentage of a shadow variant.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ShadowModelVariantConfig {
    ///
    /// The percentage of inference requests that Amazon SageMaker replicates from the production variant to the shadow variant.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "SamplingPercentage")]
    pub sampling_percentage: i64,

    ///
    /// The name of the shadow variant.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([\-a-zA-Z0-9]*[a-zA-Z0-9])?
    ///
    /// Update requires: No interruption
    #[serde(rename = "ShadowModelVariantName")]
    pub shadow_model_variant_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ShadowModelVariantConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.sampling_percentage;

        if *the_val > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'sampling_percentage'. {} is greater than 100",
                the_val
            ));
        }

        let the_val = &self.shadow_model_variant_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!("Max validation failed on field 'shadow_model_variant_name'. {} is greater than 63", s.len()));
            }
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
