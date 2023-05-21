

/// A flywheel is an AWS resource that orchestrates the ongoing training of a model for custom classification     or custom entity recognition. You can create a flywheel to start with an existing trained model, or     Comprehend can create and train a new model.
///
/// When you create the flywheel, Comprehend creates a data lake in your account. The data lake holds the training     data and test data for all versions of the model.
///
/// To use a flywheel with an existing trained model, you specify the active model version. Comprehend copies the model's     training data and test data into the flywheel's data lake.
///
/// To use the flywheel with a new model, you need to provide a dataset for training data (and optional test data)     when you create the flywheel.
///
/// For more information about flywheels, see   Flywheel overview in the Amazon Comprehend Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlywheel {


    /// 
    /// The Amazon Resource Number (ARN) of the active model version.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws(-[^:]+)?:comprehend:[a-zA-Z0-9-]*:[0-9]{12}:(document-classifier|entity-recognizer)/[a-zA-Z0-9](-*[a-zA-Z0-9])*(/version/[a-zA-Z0-9](-*[a-zA-Z0-9])*)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActiveModelArn")]
    pub active_model_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the IAM role that    grants Amazon Comprehend permission to access the flywheel data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws(-[^:]+)?:iam::[0-9]{12}:role/.+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataAccessRoleArn")]
    pub data_access_role_arn: String,


    /// 
    /// Amazon S3 URI of the data lake location.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: s3://[a-z0-9][\.\-a-z0-9]{1,61}[a-z0-9](/.*)?
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataLakeS3Uri")]
    pub data_lake_s3_uri: String,


    /// 
    /// Data security configuration.
    /// 
    /// Required: No
    ///
    /// Type: DataSecurityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSecurityConfig")]
    pub data_security_config: Option<DataSecurityConfig>,


    /// 
    /// Name for the flywheel.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*$
    ///
    /// Update requires: Replacement
    #[serde(rename = "FlywheelName")]
    pub flywheel_name: String,


    /// 
    /// Model type of the flywheel's model.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DOCUMENT_CLASSIFIER | ENTITY_RECOGNIZER
    ///
    /// Update requires: Replacement
    #[serde(rename = "ModelType")]
    pub model_type: Option<FlywheelModelTypeEnum>,


    /// 
    /// Tags associated with the endpoint being created. A tag is a key-value pair that adds    metadata to the endpoint. For example, a tag with "Sales" as the key might be added to an    endpoint to indicate its use by the sales department.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Configuration about the model associated with a flywheel.
    /// 
    /// Required: No
    ///
    /// Type: TaskConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "TaskConfig")]
    pub task_config: Option<TaskConfig>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FlywheelModelTypeEnum {

    /// DOCUMENT_CLASSIFIER
    #[serde(rename = "DOCUMENT_CLASSIFIER")]
    Documentclassifier,

    /// ENTITY_RECOGNIZER
    #[serde(rename = "ENTITY_RECOGNIZER")]
    Entityrecognizer,

}

impl Default for FlywheelModelTypeEnum {
    fn default() -> Self {
        FlywheelModelTypeEnum::Documentclassifier
    }
}


impl cfn_resources::CfnResource for CfnFlywheel {
    fn type_string() -> &'static str {
        "AWS::Comprehend::Flywheel"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Data security configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataSecurityConfig {


    /// 
    /// ID for the AWS KMS key that Amazon Comprehend uses to encrypt the data in the data lake.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^\p{ASCII}+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLakeKmsKeyId")]
    pub data_lake_kms_key_id: Option<String>,


    /// 
    /// ID for the AWS KMS key that Amazon Comprehend uses to encrypt    trained custom models. The ModelKmsKeyId can be either of the following formats:
    /// 
    /// KMS Key ID: "1234abcd-12ab-34cd-56ef-1234567890ab"                       Amazon Resource Name (ARN) of a KMS Key:       "arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^\p{ASCII}+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ModelKmsKeyId")]
    pub model_kms_key_id: Option<String>,


    /// 
    /// ID for the AWS KMS key that Amazon Comprehend uses to encrypt the volume.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: ^\p{ASCII}+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeKmsKeyId")]
    pub volume_kms_key_id: Option<String>,


    /// 
    /// Configuration parameters for an optional private Virtual Private Cloud (VPC) containing    the resources you are using for the job. For more information, see Amazon     VPC.
    /// 
    /// Required: No
    ///
    /// Type: VpcConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcConfig")]
    pub vpc_config: Option<VpcConfig>,

}




/// Configuration required for a document classification model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DocumentClassificationConfig {


    /// 
    /// One or more labels to associate with the custom classifier.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Labels")]
    pub labels: Option<Vec<String>>,


    /// 
    /// Classification mode indicates whether the documents are MULTI_CLASS or MULTI_LABEL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: MULTI_CLASS | MULTI_LABEL
    ///
    /// Update requires: Replacement
    #[serde(rename = "Mode")]
    pub mode: DocumentClassificationConfigModeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DocumentClassificationConfigModeEnum {

    /// MULTI_CLASS
    #[serde(rename = "MULTI_CLASS")]
    Multiclass,

    /// MULTI_LABEL
    #[serde(rename = "MULTI_LABEL")]
    Multilabel,

}

impl Default for DocumentClassificationConfigModeEnum {
    fn default() -> Self {
        DocumentClassificationConfigModeEnum::Multiclass
    }
}



/// Configuration required for an entity recognition model.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EntityRecognitionConfig {


    /// 
    /// Up to 25 entity types that the model is trained to recognize.
    /// 
    /// Required: No
    ///
    /// Type: List of EntityTypesListItem
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntityTypes")]
    pub entity_types: Option<Vec<EntityTypesListItem>>,

}




/// An entity type within a labeled training dataset that Amazon Comprehend uses to train a    custom entity recognizer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EntityTypesListItem {


    /// 
    /// An entity type within a labeled training dataset that Amazon Comprehend uses to train a    custom entity recognizer.
    /// 
    /// Entity types must not contain the following invalid characters: \n (line break), \\n    (escaped line break, \r (carriage return), \\r (escaped carriage return), \t (tab), \\t    (escaped tab), space, and , (comma).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: ^(?![^\n\r\t,]*\\n|\\r|\\t)[^\n\r\t,]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

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




/// Configuration about the model associated with a flywheel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TaskConfig {


    /// 
    /// Configuration required for a document classification model.
    /// 
    /// Required: No
    ///
    /// Type: DocumentClassificationConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "DocumentClassificationConfig")]
    pub document_classification_config: Option<DocumentClassificationConfig>,


    /// 
    /// Configuration required for an entity recognition model.
    /// 
    /// Required: No
    ///
    /// Type: EntityRecognitionConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "EntityRecognitionConfig")]
    pub entity_recognition_config: Option<EntityRecognitionConfig>,


    /// 
    /// Language code for the language that the model supports.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ar | de | en | es | fr | hi | it | ja | ko | pt | zh | zh-TW
    ///
    /// Update requires: Replacement
    #[serde(rename = "LanguageCode")]
    pub language_code: TaskConfigLanguageCodeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskConfigLanguageCodeEnum {

    /// ar
    #[serde(rename = "ar")]
    Ar,

    /// de
    #[serde(rename = "de")]
    De,

    /// en
    #[serde(rename = "en")]
    En,

    /// es
    #[serde(rename = "es")]
    Es,

    /// fr
    #[serde(rename = "fr")]
    Fr,

    /// hi
    #[serde(rename = "hi")]
    Hi,

    /// it
    #[serde(rename = "it")]
    It,

    /// ja
    #[serde(rename = "ja")]
    Ja,

    /// ko
    #[serde(rename = "ko")]
    Ko,

    /// pt
    #[serde(rename = "pt")]
    Pt,

    /// zh
    #[serde(rename = "zh")]
    Zh,

    /// zh-TW
    #[serde(rename = "zh-TW")]
    Zhtw,

}

impl Default for TaskConfigLanguageCodeEnum {
    fn default() -> Self {
        TaskConfigLanguageCodeEnum::Ar
    }
}



/// Configuration parameters for an optional private Virtual Private Cloud (VPC) containing    the resources you are using for the job. For more information, see Amazon     VPC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfig {


    /// 
    /// The ID number for a security group on an instance of your private VPC. Security groups on    your VPC function serve as a virtual firewall to control inbound and outbound traffic and    provides security for the resources that you’ll be accessing on the VPC. This ID number is    preceded by "sg-", for instance: "sg-03b388029b0a285ea". For more information, see Security     Groups for your VPC.
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
    /// The ID for each subnet being used in your private VPC. This subnet is a subset of the a    range of IPv4 addresses used by the VPC and is specific to a given availability zone in the    VPC’s Region. This ID number is preceded by "subnet-", for instance:    "subnet-04ccf456919e69055". For more information, see VPCs and     Subnets.
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


