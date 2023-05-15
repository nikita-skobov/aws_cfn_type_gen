
pub mod cfn_canary {

#[derive(serde::Serialize, Default)]
pub struct CfnCanary {
    /// Frequency to run your canaries
    #[serde(rename = "Schedule")]
    pub schedule: Schedule,
    /// Provide artifact configuration
    #[serde(rename = "ArtifactConfig")]
    pub artifact_config: Option<ArtifactConfig>,
    /// Retention period of failed canary runs represented in number of days
    #[serde(rename = "FailureRetentionPeriod")]
    pub failure_retention_period: Option<usize>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Retention period of successful canary runs represented in number of days
    #[serde(rename = "SuccessRetentionPeriod")]
    pub success_retention_period: Option<usize>,
    /// Lambda Execution role used to run your canaries
    #[serde(rename = "ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// Runtime version of Synthetics Library
    #[serde(rename = "RuntimeVersion")]
    pub runtime_version: String,
    /// Provide VPC Configuration if enabled.
    #[serde(rename = "VPCConfig")]
    pub vpcconfig: Option<VPCConfig>,
    /// Provide the s3 bucket output location for test results
    #[serde(rename = "ArtifactS3Location")]
    pub artifact_s3_location: String,
    /// Visual reference configuration for visual testing
    #[serde(rename = "VisualReference")]
    pub visual_reference: Option<VisualReference>,
    /// Deletes associated lambda resources created by Synthetics if set to True. Default is False
    #[serde(rename = "DeleteLambdaResourcesOnCanaryDeletion")]
    pub delete_lambda_resources_on_canary_deletion: Option<bool>,
    /// Runs canary if set to True. Default is False
    #[serde(rename = "StartCanaryAfterCreation")]
    pub start_canary_after_creation: Option<bool>,
    /// Provide the canary script source
    #[serde(rename = "Code")]
    pub code: Code,
    /// Name of the canary.
    #[serde(rename = "Name")]
    pub name: String,
    /// Provide canary run configuration
    #[serde(rename = "RunConfig")]
    pub run_config: Option<RunConfig>,

}


#[derive(serde::Serialize, Default)]
pub struct RunConfig {
    #[serde(rename = "MemoryInMB")]
    pub memory_in_mb: Option<usize>,
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Option<()>,
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<usize>,
    #[serde(rename = "ActiveTracing")]
    pub active_tracing: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct VisualReference {
    #[serde(rename = "BaseScreenshots")]
    pub base_screenshots: Option<Vec<BaseScreenshot>>,
    #[serde(rename = "BaseCanaryRunId")]
    pub base_canary_run_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct Code {
    #[serde(rename = "Script")]
    pub script: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "Handler")]
    pub handler: String,
    #[serde(rename = "SourceLocationArn")]
    pub source_location_arn: Option<String>,
    #[serde(rename = "S3Key")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3Bucket")]
    pub s3_bucket: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ArtifactConfig {
    #[serde(rename = "S3Encryption")]
    pub s3_encryption: Option<S3Encryption>,

}

#[derive(serde::Serialize, Default)]
pub struct VPCConfig {
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct S3Encryption {
    #[serde(rename = "EncryptionMode")]
    pub encryption_mode: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    pub kms_key_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct BaseScreenshot {
    #[serde(rename = "IgnoreCoordinates")]
    pub ignore_coordinates: Option<Vec<String>>,
    #[serde(rename = "ScreenshotName")]
    pub screenshot_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Schedule {
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<String>,

}


}

pub mod cfn_group {

#[derive(serde::Serialize, Default)]
pub struct CfnGroup {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Name of the group.
    #[serde(rename = "Name")]
    pub name: String,
    /// List of ResourceArn
    #[serde(rename = "ResourceArns")]
    pub resource_arns: Option<Vec<ResourceArn>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type ResourceArn = String;

}
