
pub mod cfn_flywheel {

#[derive(serde::Serialize, Default)]
pub struct CfnFlywheel {
    /// No documentation provided by AWS
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "DataLakeS3Uri")]
    pub data_lake_s3_uri: String,
    /// No documentation provided by AWS
    #[serde(rename = "DataSecurityConfig")]
    pub data_security_config: Option<DataSecurityConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "FlywheelName")]
    pub flywheel_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "TaskConfig")]
    pub task_config: Option<TaskConfig>,
    /// No documentation provided by AWS
    #[serde(rename = "ActiveModelArn")]
    pub active_model_arn: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "ModelType")]
    pub model_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct EntityTypesListItem {
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct EntityRecognitionConfig {
    #[serde(rename = "EntityTypes")]
    pub entity_types: Option<Vec<EntityTypesListItem>>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct DocumentClassificationConfig {
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "Mode")]
    pub mode: String,

}
pub type KmsKeyId = String;
#[derive(serde::Serialize, Default)]
pub struct TaskConfig {
    #[serde(rename = "EntityRecognitionConfig")]
    pub entity_recognition_config: Option<EntityRecognitionConfig>,
    #[serde(rename = "DocumentClassificationConfig")]
    pub document_classification_config: Option<DocumentClassificationConfig>,
    #[serde(rename = "LanguageCode")]
    pub language_code: String,

}

#[derive(serde::Serialize, Default)]
pub struct DataSecurityConfig {
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<KmsKeyId>,
    #[serde(rename = "DataLakeKmsKeyId")]
    pub data_lake_kms_key_id: Option<KmsKeyId>,
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,
    #[serde(rename = "ModelKmsKeyId")]
    pub model_kms_key_id: Option<KmsKeyId>,

}

#[derive(serde::Serialize, Default)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,

}


}
