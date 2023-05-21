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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<cfn_resources::StrVal>,

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
    pub data_access_role_arn: cfn_resources::StrVal,

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
    pub data_lake_s3_uri: cfn_resources::StrVal,

    ///
    /// Data security configuration.
    ///
    /// Required: No
    ///
    /// Type: DataSecurityConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataSecurityConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub flywheel_name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn type_string(&self) -> &'static str {
        "AWS::Comprehend::Flywheel"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.active_model_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'active_model_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.data_access_role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'data_access_role_arn'. {} is greater than 2048", s.len()));
            }
        }

        let the_val = &self.data_access_role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'data_access_role_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        let the_val = &self.data_lake_s3_uri;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'data_lake_s3_uri'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        self.data_security_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.flywheel_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'flywheel_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        self.task_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_kms_key_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// Configuration parameters for an optional private Virtual Private Cloud (VPC) containing    the resources you are using for the job. For more information, see Amazon     VPC.
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

impl cfn_resources::CfnResource for DataSecurityConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.data_lake_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'data_lake_kms_key_id'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.model_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'model_kms_key_id'. {} is greater than 2048", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.volume_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!("Max validation failed on field 'volume_kms_key_id'. {} is greater than 2048", s.len()));
                }
            }
        }

        self.vpc_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl cfn_resources::CfnResource for DocumentClassificationConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.labels {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'labels'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityTypesListItem>>,
}

impl cfn_resources::CfnResource for EntityRecognitionConfig {
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
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EntityTypesListItem {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.cfn_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'cfn_type'. {} is greater than 64",
                    s.len()
                ));
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl cfn_resources::CfnResource for TaskConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.document_classification_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.entity_recognition_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
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
