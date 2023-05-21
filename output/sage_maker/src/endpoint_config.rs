

/// The AWS::SageMaker::EndpointConfig resource creates a configuration       for an Amazon SageMaker endpoint. For more information, see CreateEndpointConfig       in the SageMaker Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointConfig {


    /// 
    /// Specifies configuration for how an endpoint performs asynchronous inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "AsyncInferenceConfig")]
    pub async_inference_config: Option<AsyncInferenceConfig>,


    /// 
    /// Specifies how to capture endpoint data for model monitor. The data capture       configuration applies to all production variants hosted at the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: DataCaptureConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCaptureConfig")]
    pub data_capture_config: Option<DataCaptureConfig>,


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
    /// Update requires: Replacement
    #[serde(rename = "EndpointConfigName")]
    pub endpoint_config_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ExplainerConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ExplainerConfig")]
    pub explainer_config: Option<ExplainerConfig>,


    /// 
    /// The Amazon Resource Name (ARN) of an AWS Key Management Service key       that Amazon SageMaker uses to encrypt data on the storage volume attached to the ML       compute instance that hosts the endpoint.
    /// 
    /// Key ID: 1234abcd-12ab-34cd-56ef-1234567890ab               Key ARN:             arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab               Alias name: alias/ExampleAlias               Alias name ARN:             arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias
    /// 
    /// The KMS key policy must grant permission to the IAM role that you specify in your         CreateEndpoint, UpdateEndpoint requests. For more       information, refer to the AWS Key Management Service section Using Key         Policies in AWS KMS
    /// 
    /// NoteCertain Nitro-based instances include local storage, dependent on the instance         type. Local storage volumes are encrypted using a hardware module on the instance.         You can't request a KmsKeyId when using an instance type with local         storage. If any of the models that you specify in the           ProductionVariants parameter use nitro-based instances with local         storage, do not specify a value for the KmsKeyId parameter. If you         specify a value for KmsKeyId when using any nitro-based instances with         local storage, the call to CreateEndpointConfig fails.For a list of instance types that support local instance storage, see Instance Store Volumes.For more information about local instance storage encryption, see SSD           Instance Store Volumes.
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
    pub kms_key_id: Option<String>,


    /// 
    /// A list of ProductionVariant objects, one for each model that you want       to host at this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ProductionVariant
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProductionVariants")]
    pub production_variants: Vec<ProductionVariant>,


    /// 
    /// Array of ProductionVariant objects. There is one for each model that you       want to host at this endpoint in shadow mode with production traffic replicated from the       model specified on ProductionVariants. If you use this field, you can only       specify one variant for ProductionVariants and one variant for         ShadowProductionVariants.
    /// 
    /// Required: No
    ///
    /// Type: List of ProductionVariant
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShadowProductionVariants")]
    pub shadow_production_variants: Option<Vec<ProductionVariant>>,


    /// 
    /// A list of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags.
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



impl cfn_resources::CfnResource for CfnEndpointConfig {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::EndpointConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.async_inference_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.data_capture_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.endpoint_config_name {

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'endpoint_config_name'. {} is greater than 63", the_val.len()));
        }

        }
        
        self.explainer_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.kms_key_id {

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'kms_key_id'. {} is greater than 2048", the_val.len()));
        }

        }
        
        let the_val = &self.production_variants;

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'production_variants'. {} is greater than 10", the_val.len()));
        }

        
        if let Some(the_val) = &self.shadow_production_variants {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'shadow_production_variants'. {} is greater than 10", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Configures the behavior of the client used by SageMaker to interact with the model       container during asynchronous inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceClientConfig {


    /// 
    /// The maximum number of concurrent requests sent by the SageMaker client to the model       container. If no value is provided, SageMaker will choose an optimal value for       you.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxConcurrentInvocationsPerInstance")]
    pub max_concurrent_invocations_per_instance: Option<i64>,

}



impl cfn_resources::CfnResource for AsyncInferenceClientConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies configuration for how an endpoint performs asynchronous inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceConfig {


    /// 
    /// Configures the behavior of the client used by SageMaker to interact with the model       container during asynchronous inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceClientConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<AsyncInferenceClientConfig>,


    /// 
    /// Specifies the configuration for asynchronous inference invocation outputs.
    /// 
    /// Required: Yes
    ///
    /// Type: AsyncInferenceOutputConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutputConfig")]
    pub output_config: AsyncInferenceOutputConfig,

}



impl cfn_resources::CfnResource for AsyncInferenceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.client_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.output_config.validate()?;

        Ok(())
    }
}

/// Specifies the configuration for notifications of inference results for asynchronous       inference.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceNotificationConfig {


    /// 
    /// Amazon SNS topic to post a notification to when an inference fails. If no topic is       provided, no notification is sent on failure.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ErrorTopic")]
    pub error_topic: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IncludeInferenceResponseIn")]
    pub include_inference_response_in: Option<Vec<String>>,


    /// 
    /// Amazon SNS topic to post a notification to when an inference completes successfully.       If no topic is provided, no notification is sent on success.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SuccessTopic")]
    pub success_topic: Option<String>,

}



impl cfn_resources::CfnResource for AsyncInferenceNotificationConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies the configuration for asynchronous inference invocation outputs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AsyncInferenceOutputConfig {


    /// 
    /// The AWS Key Management Service (AWS KMS) key that Amazon       SageMaker uses to encrypt the asynchronous inference output in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// Specifies the configuration for notifications of inference results for asynchronous       inference.
    /// 
    /// Required: No
    ///
    /// Type: AsyncInferenceNotificationConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotificationConfig")]
    pub notification_config: Option<AsyncInferenceNotificationConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3FailurePath")]
    pub s3_failure_path: Option<String>,


    /// 
    /// The Amazon S3 location to upload inference responses to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,

}



impl cfn_resources::CfnResource for AsyncInferenceOutputConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.notification_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the JSON and CSV content types of the data that the endpoint       captures.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptureContentTypeHeader {


    /// 
    /// A list of the CSV content types of the data that the endpoint captures. For the       endpoint to capture the data, you must also specify the content type when you invoke the       endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "CsvContentTypes")]
    pub csv_content_types: Option<Vec<String>>,


    /// 
    /// A list of the JSON content types of the data that the endpoint captures. For the       endpoint to capture the data, you must also specify the content type when you invoke the       endpoint.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: Replacement
    #[serde(rename = "JsonContentTypes")]
    pub json_content_types: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for CaptureContentTypeHeader {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.csv_content_types {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'csv_content_types'. {} is greater than 10", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.json_content_types {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'json_content_types'. {} is greater than 10", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// Specifies whether the endpoint captures input data or output data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CaptureOption {


    /// 
    /// Specifies whether the endpoint captures input data or output data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Input | Output
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureMode")]
    pub capture_mode: CaptureOptionCaptureModeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CaptureOptionCaptureModeEnum {

    /// Input
    #[serde(rename = "Input")]
    Input,

    /// Output
    #[serde(rename = "Output")]
    Output,

}

impl Default for CaptureOptionCaptureModeEnum {
    fn default() -> Self {
        CaptureOptionCaptureModeEnum::Input
    }
}


impl cfn_resources::CfnResource for CaptureOption {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ClarifyExplainerConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyExplainerConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableExplanations")]
    pub enable_explanations: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyInferenceConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "InferenceConfig")]
    pub inference_config: Option<ClarifyInferenceConfig>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ClarifyShapConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapConfig")]
    pub shap_config: ClarifyShapConfig,

}



impl cfn_resources::CfnResource for ClarifyExplainerConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.inference_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.shap_config.validate()?;

        Ok(())
    }
}

/// The ClarifyFeatureType property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyFeatureType {

}



impl cfn_resources::CfnResource for ClarifyFeatureType {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ClarifyHeader property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyHeader {

}



impl cfn_resources::CfnResource for ClarifyHeader {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ClarifyInferenceConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyInferenceConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContentTemplate")]
    pub content_template: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureHeaders")]
    pub feature_headers: Option<Vec<ClarifyHeader>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyFeatureType
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeatureTypes")]
    pub feature_types: Option<Vec<ClarifyFeatureType>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FeaturesAttribute")]
    pub features_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelAttribute")]
    pub label_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of ClarifyHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelHeaders")]
    pub label_headers: Option<Vec<ClarifyHeader>>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "LabelIndex")]
    pub label_index: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxPayloadInMB")]
    pub max_payload_in_mb: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxRecordCount")]
    pub max_record_count: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityAttribute")]
    pub probability_attribute: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProbabilityIndex")]
    pub probability_index: Option<i64>,

}



impl cfn_resources::CfnResource for ClarifyInferenceConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ClarifyShapBaselineConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyShapBaselineConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MimeType")]
    pub mime_type: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaseline")]
    pub shap_baseline: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaselineUri")]
    pub shap_baseline_uri: Option<String>,

}



impl cfn_resources::CfnResource for ClarifyShapBaselineConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// The ClarifyShapConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyShapConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NumberOfSamples")]
    pub number_of_samples: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Seed")]
    pub seed: Option<i64>,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ClarifyShapBaselineConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ShapBaselineConfig")]
    pub shap_baseline_config: ClarifyShapBaselineConfig,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyTextConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TextConfig")]
    pub text_config: Option<ClarifyTextConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "UseLogit")]
    pub use_logit: Option<bool>,

}



impl cfn_resources::CfnResource for ClarifyShapConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.shap_baseline_config.validate()?;

        self.text_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The ClarifyTextConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClarifyTextConfig {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Granularity")]
    pub granularity: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Language")]
    pub language: String,

}



impl cfn_resources::CfnResource for ClarifyTextConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// Specifies the configuration of your endpoint for model monitor data capture.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataCaptureConfig {


    /// 
    /// A list of the JSON and CSV content type that the endpoint captures.
    /// 
    /// Required: No
    ///
    /// Type: CaptureContentTypeHeader
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureContentTypeHeader")]
    pub capture_content_type_header: Option<CaptureContentTypeHeader>,


    /// 
    /// Specifies whether the endpoint captures input data to your model, output data from       your model, or both.
    /// 
    /// Required: Yes
    ///
    /// Type: List of CaptureOption
    ///
    /// Maximum: 2
    ///
    /// Update requires: Replacement
    #[serde(rename = "CaptureOptions")]
    pub capture_options: Vec<CaptureOption>,


    /// 
    /// The S3 bucket where model monitor stores captured data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^(https|s3)://([^/])/?(.*)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationS3Uri")]
    pub destination_s3_uri: String,


    /// 
    /// Set to True to enable data capture.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableCapture")]
    pub enable_capture: Option<bool>,


    /// 
    /// The percentage of data to capture.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialSamplingPercentage")]
    pub initial_sampling_percentage: i64,


    /// 
    /// The AWS Key Management Service (AWS KMS) key that       Amazon SageMaker uses to encrypt the captured data at rest using Amazon S3 server-side       encryption. The KmsKeyId can be any of the following formats: Key ID:       1234abcd-12ab-34cd-56ef-1234567890ab Key ARN:       arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab Alias name:       alias/ExampleAlias Alias name ARN: arn:aws:kms:us-west-2:111122223333:alias/ExampleAlias       If you don't provide a KMS key ID, Amazon SageMaker uses the default KMS key for Amazon       S3 for your role's account. For more information, see KMS-Managed Encryption Keys       (https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingKMSEncryption.html) in the Amazon       Simple Storage Service Developer Guide. The KMS key policy must grant permission to the       IAM role that you specify in your CreateModel       (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateModel.html)       request. For more information, see Using Key Policies in AWS KMS       (http://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html) in the AWS Key Management Service Developer Guide.
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
    pub kms_key_id: Option<String>,

}



impl cfn_resources::CfnResource for DataCaptureConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.capture_content_type_header.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.capture_options;

        if the_val.len() > 2 as _ {
            return Err(format!("Max validation failed on field 'capture_options'. {} is greater than 2", the_val.len()));
        }

        
        let the_val = &self.destination_s3_uri;

        if the_val.len() > 512 as _ {
            return Err(format!("Max validation failed on field 'destination_s3_uri'. {} is greater than 512", the_val.len()));
        }

        
        let the_val = &self.initial_sampling_percentage;

        if *the_val > 100 as _ {
            return Err(format!("Max validation failed on field 'initial_sampling_percentage'. {} is greater than 100", the_val));
        }

        
        let the_val = &self.initial_sampling_percentage;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'initial_sampling_percentage'. {} is less than 0", the_val));
        }

        
        if let Some(the_val) = &self.kms_key_id {

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'kms_key_id'. {} is greater than 2048", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// The ExplainerConfig property type specifies Property description not available. for an AWS::SageMaker::EndpointConfig.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExplainerConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: ClarifyExplainerConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClarifyExplainerConfig")]
    pub clarify_explainer_config: Option<ClarifyExplainerConfig>,

}



impl cfn_resources::CfnResource for ExplainerConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.clarify_explainer_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies a model that you want to host and the resources to deploy for hosting it.       If you are deploying multiple models, tell Amazon SageMaker how to distribute traffic       among the models by specifying the InitialVariantWeight objects.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ProductionVariant {


    /// 
    /// The size of the Elastic Inference (EI) instance to use for the production variant. EI       instances provide on-demand GPU computing for inference. For more information, see         Using Elastic         Inference in Amazon SageMaker. For more information, see Using Elastic Inference in         Amazon SageMaker.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.eia1.large | ml.eia1.medium | ml.eia1.xlarge | ml.eia2.large | ml.eia2.medium | ml.eia2.xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "AcceleratorType")]
    pub accelerator_type: Option<ProductionVariantAcceleratorTypeEnum>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ContainerStartupHealthCheckTimeoutInSeconds")]
    pub container_startup_health_check_timeout_in_seconds: Option<i64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "EnableSSMAccess")]
    pub enable_ssmaccess: Option<bool>,


    /// 
    /// Number of instances to launch initially.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialInstanceCount")]
    pub initial_instance_count: Option<i64>,


    /// 
    /// Determines initial traffic distribution among all of the models that you specify in       the endpoint configuration. The traffic to a production variant is determined by the       ratio of the VariantWeight to the sum of all VariantWeight       values across all ProductionVariants. If unspecified, it defaults to 1.0.
    /// 
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVariantWeight")]
    pub initial_variant_weight: f64,


    /// 
    /// The ML compute instance type.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.large | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.large | ml.c5.xlarge | ml.c5d.18xlarge | ml.c5d.2xlarge | ml.c5d.4xlarge | ml.c5d.9xlarge | ml.c5d.large | ml.c5d.xlarge | ml.c6g.12xlarge | ml.c6g.16xlarge | ml.c6g.2xlarge | ml.c6g.4xlarge | ml.c6g.8xlarge | ml.c6g.large | ml.c6g.xlarge | ml.c6gd.12xlarge | ml.c6gd.16xlarge | ml.c6gd.2xlarge | ml.c6gd.4xlarge | ml.c6gd.8xlarge | ml.c6gd.large | ml.c6gd.xlarge | ml.c6gn.12xlarge | ml.c6gn.16xlarge | ml.c6gn.2xlarge | ml.c6gn.4xlarge | ml.c6gn.8xlarge | ml.c6gn.large | ml.c6gn.xlarge | ml.c6i.12xlarge | ml.c6i.16xlarge | ml.c6i.24xlarge | ml.c6i.2xlarge | ml.c6i.32xlarge | ml.c6i.4xlarge | ml.c6i.8xlarge | ml.c6i.large | ml.c6i.xlarge | ml.c7g.12xlarge | ml.c7g.16xlarge | ml.c7g.2xlarge | ml.c7g.4xlarge | ml.c7g.8xlarge | ml.c7g.large | ml.c7g.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.inf1.24xlarge | ml.inf1.2xlarge | ml.inf1.6xlarge | ml.inf1.xlarge | ml.inf2.24xlarge | ml.inf2.48xlarge | ml.inf2.8xlarge | ml.inf2.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.large | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.large | ml.m5d.xlarge | ml.m6g.12xlarge | ml.m6g.16xlarge | ml.m6g.2xlarge | ml.m6g.4xlarge | ml.m6g.8xlarge | ml.m6g.large | ml.m6g.xlarge | ml.m6gd.12xlarge | ml.m6gd.16xlarge | ml.m6gd.2xlarge | ml.m6gd.4xlarge | ml.m6gd.8xlarge | ml.m6gd.large | ml.m6gd.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p4d.24xlarge | ml.p4de.24xlarge | ml.r5.12xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.large | ml.r5.xlarge | ml.r5d.12xlarge | ml.r5d.24xlarge | ml.r5d.2xlarge | ml.r5d.4xlarge | ml.r5d.large | ml.r5d.xlarge | ml.r6g.12xlarge | ml.r6g.16xlarge | ml.r6g.2xlarge | ml.r6g.4xlarge | ml.r6g.8xlarge | ml.r6g.large | ml.r6g.xlarge | ml.r6gd.12xlarge | ml.r6gd.16xlarge | ml.r6gd.2xlarge | ml.r6gd.4xlarge | ml.r6gd.8xlarge | ml.r6gd.large | ml.r6gd.xlarge | ml.t2.2xlarge | ml.t2.large | ml.t2.medium | ml.t2.xlarge | ml.trn1.2xlarge | ml.trn1.32xlarge
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<ProductionVariantInstanceTypeEnum>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelDataDownloadTimeoutInSeconds")]
    pub model_data_download_timeout_in_seconds: Option<i64>,


    /// 
    /// The name of the model that you want to host. This is the name that you specified       when creating the model.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelName")]
    pub model_name: String,


    /// 
    /// The serverless configuration for an endpoint. Specifies a serverless endpoint configuration instead of an instance-based endpoint configuration.
    /// 
    /// Required: No
    ///
    /// Type: ServerlessConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerlessConfig")]
    pub serverless_config: Option<ServerlessConfig>,


    /// 
    /// The name of the production variant.
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
    #[serde(rename = "VariantName")]
    pub variant_name: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSizeInGB")]
    pub volume_size_in_gb: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ProductionVariantAcceleratorTypeEnum {

    /// ml.eia1.large
    #[serde(rename = "ml.eia1.large")]
    Mleia1large,

    /// ml.eia1.medium
    #[serde(rename = "ml.eia1.medium")]
    Mleia1medium,

    /// ml.eia1.xlarge
    #[serde(rename = "ml.eia1.xlarge")]
    Mleia1xlarge,

    /// ml.eia2.large
    #[serde(rename = "ml.eia2.large")]
    Mleia2large,

    /// ml.eia2.medium
    #[serde(rename = "ml.eia2.medium")]
    Mleia2medium,

    /// ml.eia2.xlarge
    #[serde(rename = "ml.eia2.xlarge")]
    Mleia2xlarge,

}

impl Default for ProductionVariantAcceleratorTypeEnum {
    fn default() -> Self {
        ProductionVariantAcceleratorTypeEnum::Mleia1large
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ProductionVariantInstanceTypeEnum {

    /// ml.c4.2xlarge
    #[serde(rename = "ml.c4.2xlarge")]
    Mlc42xlarge,

    /// ml.c4.4xlarge
    #[serde(rename = "ml.c4.4xlarge")]
    Mlc44xlarge,

    /// ml.c4.8xlarge
    #[serde(rename = "ml.c4.8xlarge")]
    Mlc48xlarge,

    /// ml.c4.large
    #[serde(rename = "ml.c4.large")]
    Mlc4large,

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

    /// ml.c5.large
    #[serde(rename = "ml.c5.large")]
    Mlc5large,

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

    /// ml.c5d.large
    #[serde(rename = "ml.c5d.large")]
    Mlc5dlarge,

    /// ml.c5d.xlarge
    #[serde(rename = "ml.c5d.xlarge")]
    Mlc5dxlarge,

    /// ml.c6g.12xlarge
    #[serde(rename = "ml.c6g.12xlarge")]
    Mlc6g12xlarge,

    /// ml.c6g.16xlarge
    #[serde(rename = "ml.c6g.16xlarge")]
    Mlc6g16xlarge,

    /// ml.c6g.2xlarge
    #[serde(rename = "ml.c6g.2xlarge")]
    Mlc6g2xlarge,

    /// ml.c6g.4xlarge
    #[serde(rename = "ml.c6g.4xlarge")]
    Mlc6g4xlarge,

    /// ml.c6g.8xlarge
    #[serde(rename = "ml.c6g.8xlarge")]
    Mlc6g8xlarge,

    /// ml.c6g.large
    #[serde(rename = "ml.c6g.large")]
    Mlc6glarge,

    /// ml.c6g.xlarge
    #[serde(rename = "ml.c6g.xlarge")]
    Mlc6gxlarge,

    /// ml.c6gd.12xlarge
    #[serde(rename = "ml.c6gd.12xlarge")]
    Mlc6gd12xlarge,

    /// ml.c6gd.16xlarge
    #[serde(rename = "ml.c6gd.16xlarge")]
    Mlc6gd16xlarge,

    /// ml.c6gd.2xlarge
    #[serde(rename = "ml.c6gd.2xlarge")]
    Mlc6gd2xlarge,

    /// ml.c6gd.4xlarge
    #[serde(rename = "ml.c6gd.4xlarge")]
    Mlc6gd4xlarge,

    /// ml.c6gd.8xlarge
    #[serde(rename = "ml.c6gd.8xlarge")]
    Mlc6gd8xlarge,

    /// ml.c6gd.large
    #[serde(rename = "ml.c6gd.large")]
    Mlc6gdlarge,

    /// ml.c6gd.xlarge
    #[serde(rename = "ml.c6gd.xlarge")]
    Mlc6gdxlarge,

    /// ml.c6gn.12xlarge
    #[serde(rename = "ml.c6gn.12xlarge")]
    Mlc6gn12xlarge,

    /// ml.c6gn.16xlarge
    #[serde(rename = "ml.c6gn.16xlarge")]
    Mlc6gn16xlarge,

    /// ml.c6gn.2xlarge
    #[serde(rename = "ml.c6gn.2xlarge")]
    Mlc6gn2xlarge,

    /// ml.c6gn.4xlarge
    #[serde(rename = "ml.c6gn.4xlarge")]
    Mlc6gn4xlarge,

    /// ml.c6gn.8xlarge
    #[serde(rename = "ml.c6gn.8xlarge")]
    Mlc6gn8xlarge,

    /// ml.c6gn.large
    #[serde(rename = "ml.c6gn.large")]
    Mlc6gnlarge,

    /// ml.c6gn.xlarge
    #[serde(rename = "ml.c6gn.xlarge")]
    Mlc6gnxlarge,

    /// ml.c6i.12xlarge
    #[serde(rename = "ml.c6i.12xlarge")]
    Mlc6i12xlarge,

    /// ml.c6i.16xlarge
    #[serde(rename = "ml.c6i.16xlarge")]
    Mlc6i16xlarge,

    /// ml.c6i.24xlarge
    #[serde(rename = "ml.c6i.24xlarge")]
    Mlc6i24xlarge,

    /// ml.c6i.2xlarge
    #[serde(rename = "ml.c6i.2xlarge")]
    Mlc6i2xlarge,

    /// ml.c6i.32xlarge
    #[serde(rename = "ml.c6i.32xlarge")]
    Mlc6i32xlarge,

    /// ml.c6i.4xlarge
    #[serde(rename = "ml.c6i.4xlarge")]
    Mlc6i4xlarge,

    /// ml.c6i.8xlarge
    #[serde(rename = "ml.c6i.8xlarge")]
    Mlc6i8xlarge,

    /// ml.c6i.large
    #[serde(rename = "ml.c6i.large")]
    Mlc6ilarge,

    /// ml.c6i.xlarge
    #[serde(rename = "ml.c6i.xlarge")]
    Mlc6ixlarge,

    /// ml.c7g.12xlarge
    #[serde(rename = "ml.c7g.12xlarge")]
    Mlc7g12xlarge,

    /// ml.c7g.16xlarge
    #[serde(rename = "ml.c7g.16xlarge")]
    Mlc7g16xlarge,

    /// ml.c7g.2xlarge
    #[serde(rename = "ml.c7g.2xlarge")]
    Mlc7g2xlarge,

    /// ml.c7g.4xlarge
    #[serde(rename = "ml.c7g.4xlarge")]
    Mlc7g4xlarge,

    /// ml.c7g.8xlarge
    #[serde(rename = "ml.c7g.8xlarge")]
    Mlc7g8xlarge,

    /// ml.c7g.large
    #[serde(rename = "ml.c7g.large")]
    Mlc7glarge,

    /// ml.c7g.xlarge
    #[serde(rename = "ml.c7g.xlarge")]
    Mlc7gxlarge,

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

    /// ml.inf1.24xlarge
    #[serde(rename = "ml.inf1.24xlarge")]
    Mlinf124xlarge,

    /// ml.inf1.2xlarge
    #[serde(rename = "ml.inf1.2xlarge")]
    Mlinf12xlarge,

    /// ml.inf1.6xlarge
    #[serde(rename = "ml.inf1.6xlarge")]
    Mlinf16xlarge,

    /// ml.inf1.xlarge
    #[serde(rename = "ml.inf1.xlarge")]
    Mlinf1xlarge,

    /// ml.inf2.24xlarge
    #[serde(rename = "ml.inf2.24xlarge")]
    Mlinf224xlarge,

    /// ml.inf2.48xlarge
    #[serde(rename = "ml.inf2.48xlarge")]
    Mlinf248xlarge,

    /// ml.inf2.8xlarge
    #[serde(rename = "ml.inf2.8xlarge")]
    Mlinf28xlarge,

    /// ml.inf2.xlarge
    #[serde(rename = "ml.inf2.xlarge")]
    Mlinf2xlarge,

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

    /// ml.m5.large
    #[serde(rename = "ml.m5.large")]
    Mlm5large,

    /// ml.m5.xlarge
    #[serde(rename = "ml.m5.xlarge")]
    Mlm5xlarge,

    /// ml.m5d.12xlarge
    #[serde(rename = "ml.m5d.12xlarge")]
    Mlm5d12xlarge,

    /// ml.m5d.24xlarge
    #[serde(rename = "ml.m5d.24xlarge")]
    Mlm5d24xlarge,

    /// ml.m5d.2xlarge
    #[serde(rename = "ml.m5d.2xlarge")]
    Mlm5d2xlarge,

    /// ml.m5d.4xlarge
    #[serde(rename = "ml.m5d.4xlarge")]
    Mlm5d4xlarge,

    /// ml.m5d.large
    #[serde(rename = "ml.m5d.large")]
    Mlm5dlarge,

    /// ml.m5d.xlarge
    #[serde(rename = "ml.m5d.xlarge")]
    Mlm5dxlarge,

    /// ml.m6g.12xlarge
    #[serde(rename = "ml.m6g.12xlarge")]
    Mlm6g12xlarge,

    /// ml.m6g.16xlarge
    #[serde(rename = "ml.m6g.16xlarge")]
    Mlm6g16xlarge,

    /// ml.m6g.2xlarge
    #[serde(rename = "ml.m6g.2xlarge")]
    Mlm6g2xlarge,

    /// ml.m6g.4xlarge
    #[serde(rename = "ml.m6g.4xlarge")]
    Mlm6g4xlarge,

    /// ml.m6g.8xlarge
    #[serde(rename = "ml.m6g.8xlarge")]
    Mlm6g8xlarge,

    /// ml.m6g.large
    #[serde(rename = "ml.m6g.large")]
    Mlm6glarge,

    /// ml.m6g.xlarge
    #[serde(rename = "ml.m6g.xlarge")]
    Mlm6gxlarge,

    /// ml.m6gd.12xlarge
    #[serde(rename = "ml.m6gd.12xlarge")]
    Mlm6gd12xlarge,

    /// ml.m6gd.16xlarge
    #[serde(rename = "ml.m6gd.16xlarge")]
    Mlm6gd16xlarge,

    /// ml.m6gd.2xlarge
    #[serde(rename = "ml.m6gd.2xlarge")]
    Mlm6gd2xlarge,

    /// ml.m6gd.4xlarge
    #[serde(rename = "ml.m6gd.4xlarge")]
    Mlm6gd4xlarge,

    /// ml.m6gd.8xlarge
    #[serde(rename = "ml.m6gd.8xlarge")]
    Mlm6gd8xlarge,

    /// ml.m6gd.large
    #[serde(rename = "ml.m6gd.large")]
    Mlm6gdlarge,

    /// ml.m6gd.xlarge
    #[serde(rename = "ml.m6gd.xlarge")]
    Mlm6gdxlarge,

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

    /// ml.p4d.24xlarge
    #[serde(rename = "ml.p4d.24xlarge")]
    Mlp4d24xlarge,

    /// ml.p4de.24xlarge
    #[serde(rename = "ml.p4de.24xlarge")]
    Mlp4de24xlarge,

    /// ml.r5.12xlarge
    #[serde(rename = "ml.r5.12xlarge")]
    Mlr512xlarge,

    /// ml.r5.24xlarge
    #[serde(rename = "ml.r5.24xlarge")]
    Mlr524xlarge,

    /// ml.r5.2xlarge
    #[serde(rename = "ml.r5.2xlarge")]
    Mlr52xlarge,

    /// ml.r5.4xlarge
    #[serde(rename = "ml.r5.4xlarge")]
    Mlr54xlarge,

    /// ml.r5.large
    #[serde(rename = "ml.r5.large")]
    Mlr5large,

    /// ml.r5.xlarge
    #[serde(rename = "ml.r5.xlarge")]
    Mlr5xlarge,

    /// ml.r5d.12xlarge
    #[serde(rename = "ml.r5d.12xlarge")]
    Mlr5d12xlarge,

    /// ml.r5d.24xlarge
    #[serde(rename = "ml.r5d.24xlarge")]
    Mlr5d24xlarge,

    /// ml.r5d.2xlarge
    #[serde(rename = "ml.r5d.2xlarge")]
    Mlr5d2xlarge,

    /// ml.r5d.4xlarge
    #[serde(rename = "ml.r5d.4xlarge")]
    Mlr5d4xlarge,

    /// ml.r5d.large
    #[serde(rename = "ml.r5d.large")]
    Mlr5dlarge,

    /// ml.r5d.xlarge
    #[serde(rename = "ml.r5d.xlarge")]
    Mlr5dxlarge,

    /// ml.r6g.12xlarge
    #[serde(rename = "ml.r6g.12xlarge")]
    Mlr6g12xlarge,

    /// ml.r6g.16xlarge
    #[serde(rename = "ml.r6g.16xlarge")]
    Mlr6g16xlarge,

    /// ml.r6g.2xlarge
    #[serde(rename = "ml.r6g.2xlarge")]
    Mlr6g2xlarge,

    /// ml.r6g.4xlarge
    #[serde(rename = "ml.r6g.4xlarge")]
    Mlr6g4xlarge,

    /// ml.r6g.8xlarge
    #[serde(rename = "ml.r6g.8xlarge")]
    Mlr6g8xlarge,

    /// ml.r6g.large
    #[serde(rename = "ml.r6g.large")]
    Mlr6glarge,

    /// ml.r6g.xlarge
    #[serde(rename = "ml.r6g.xlarge")]
    Mlr6gxlarge,

    /// ml.r6gd.12xlarge
    #[serde(rename = "ml.r6gd.12xlarge")]
    Mlr6gd12xlarge,

    /// ml.r6gd.16xlarge
    #[serde(rename = "ml.r6gd.16xlarge")]
    Mlr6gd16xlarge,

    /// ml.r6gd.2xlarge
    #[serde(rename = "ml.r6gd.2xlarge")]
    Mlr6gd2xlarge,

    /// ml.r6gd.4xlarge
    #[serde(rename = "ml.r6gd.4xlarge")]
    Mlr6gd4xlarge,

    /// ml.r6gd.8xlarge
    #[serde(rename = "ml.r6gd.8xlarge")]
    Mlr6gd8xlarge,

    /// ml.r6gd.large
    #[serde(rename = "ml.r6gd.large")]
    Mlr6gdlarge,

    /// ml.r6gd.xlarge
    #[serde(rename = "ml.r6gd.xlarge")]
    Mlr6gdxlarge,

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

    /// ml.trn1.2xlarge
    #[serde(rename = "ml.trn1.2xlarge")]
    Mltrn12xlarge,

    /// ml.trn1.32xlarge
    #[serde(rename = "ml.trn1.32xlarge")]
    Mltrn132xlarge,

}

impl Default for ProductionVariantInstanceTypeEnum {
    fn default() -> Self {
        ProductionVariantInstanceTypeEnum::Mlc42xlarge
    }
}


impl cfn_resources::CfnResource for ProductionVariant {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.initial_instance_count {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'initial_instance_count'. {} is less than 1", the_val));
        }

        }
        
        let the_val = &self.model_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'model_name'. {} is greater than 63", the_val.len()));
        }

        
        self.serverless_config.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.variant_name;

        if the_val.len() > 63 as _ {
            return Err(format!("Max validation failed on field 'variant_name'. {} is greater than 63", the_val.len()));
        }

        
        Ok(())
    }
}

/// Specifies the serverless configuration for an endpoint variant.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServerlessConfig {


    /// 
    /// The maximum number of concurrent invocations your serverless endpoint can process.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Update requires: Replacement
    #[serde(rename = "MaxConcurrency")]
    pub max_concurrency: i64,


    /// 
    /// The memory size of your serverless endpoint. Valid values are in 1 GB increments: 1024 MB, 2048 MB, 3072 MB, 4096 MB, 5120 MB, or 6144 MB.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1024
    ///
    /// Maximum: 6144
    ///
    /// Update requires: Replacement
    #[serde(rename = "MemorySizeInMB")]
    pub memory_size_in_mb: i64,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProvisionedConcurrency")]
    pub provisioned_concurrency: Option<i64>,

}



impl cfn_resources::CfnResource for ServerlessConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.max_concurrency;

        if *the_val > 200 as _ {
            return Err(format!("Max validation failed on field 'max_concurrency'. {} is greater than 200", the_val));
        }

        
        let the_val = &self.max_concurrency;

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'max_concurrency'. {} is less than 1", the_val));
        }

        
        let the_val = &self.memory_size_in_mb;

        if *the_val > 6144 as _ {
            return Err(format!("Max validation failed on field 'memory_size_in_mb'. {} is greater than 6144", the_val));
        }

        
        let the_val = &self.memory_size_in_mb;

        if *the_val < 1024 as _ {
            return Err(format!("Min validation failed on field 'memory_size_in_mb'. {} is less than 1024", the_val));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}