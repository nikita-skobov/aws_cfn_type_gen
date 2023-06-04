/// The AWS::ECR::Repository resource specifies an Amazon Elastic Container       Registry (Amazon ECR) repository, where users can push and pull Docker images, Open       Container Initiative (OCI) images, and OCI compatible artifacts. For more information,       see Amazon ECR private repositories in the Amazon ECR User         Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRepository {
    ///
    /// The encryption configuration for the repository. This determines how the contents of       your repository are encrypted at rest.
    ///
    /// Required: No
    ///
    /// Type: EncryptionConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,

    ///
    /// The image scanning configuration for the repository. This determines whether images       are scanned for known vulnerabilities after being pushed to the repository.
    ///
    /// Required: No
    ///
    /// Type: ImageScanningConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageScanningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,

    ///
    /// The tag mutability setting for the repository. If this parameter is omitted, the       default setting of MUTABLE will be used which will allow image tags to be       overwritten. If IMMUTABLE is specified, all image tags within the       repository will be immutable which will prevent them from being overwritten.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IMMUTABLE | MUTABLE
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageTagMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<RepositoryImageTagMutabilityEnum>,

    ///
    /// Creates or updates a lifecycle policy. For information about lifecycle policy syntax,       see Lifecycle policy template.
    ///
    /// Required: No
    ///
    /// Type: LifecyclePolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecyclePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<LifecyclePolicy>,

    ///
    /// The name to use for the repository. The repository name may be specified on its own       (such as nginx-web-app) or it can be prepended with a namespace to group       the repository into a category (such as project-a/nginx-web-app). If you       don't specify a name, AWS CloudFormation generates a unique physical ID and uses       that ID for the repository name. For more information, see Name       type.
    ///
    /// The repository name must start with a letter and can only contain lowercase letters,       numbers, hyphens, underscores, and forward slashes.
    ///
    /// NoteIf you specify a name, you cannot perform updates that require replacement of this         resource. You can perform updates that require no or some interruption. If you must         replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 256
    ///
    /// Pattern: (?:[a-z0-9]+(?:[._-][a-z0-9]+)*/)*[a-z0-9]+(?:[._-][a-z0-9]+)*
    ///
    /// Update requires: Replacement
    #[serde(rename = "RepositoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<cfn_resources::StrVal>,

    ///
    /// The JSON repository policy text to apply to the repository. For more information, see         Amazon ECR repository         policies in the Amazon Elastic Container Registry User Guide.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Minimum: 0
    ///
    /// Maximum: 10240
    ///
    /// Update requires: No interruption
    #[serde(rename = "RepositoryPolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy_text: Option<serde_json::Value>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnRepositoryarn,

    #[serde(skip_serializing)]
    pub att_repository_uri: CfnRepositoryrepositoryuri,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RepositoryImageTagMutabilityEnum {
    /// IMMUTABLE
    #[serde(rename = "IMMUTABLE")]
    Immutable,

    /// MUTABLE
    #[serde(rename = "MUTABLE")]
    Mutable,
}

impl Default for RepositoryImageTagMutabilityEnum {
    fn default() -> Self {
        RepositoryImageTagMutabilityEnum::Immutable
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRepositoryarn;
impl CfnRepositoryarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnRepositoryrepositoryuri;
impl CfnRepositoryrepositoryuri {
    pub fn att_name(&self) -> &'static str {
        r#"RepositoryUri"#
    }
}

impl cfn_resources::CfnResource for CfnRepository {
    fn type_string(&self) -> &'static str {
        "AWS::ECR::Repository"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.image_scanning_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.lifecycle_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.repository_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'repository_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.repository_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 2 as _ {
                    return Err(format!(
                        "Min validation failed on field 'repository_name'. {} is less than 2",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The encryption configuration for the repository. This determines how the contents of       your repository are encrypted at rest.
///
/// By default, when no encryption configuration is set or the AES256       encryption type is used, Amazon ECR uses server-side encryption with Amazon S3-managed encryption       keys which encrypts your data at rest using an AES-256 encryption algorithm. This does       not require any action on your part.
///
/// For more control over the encryption of the contents of your repository, you can use       server-side encryption with AWS Key Management Service key stored in AWS Key Management Service (AWS KMS) to encrypt your       images. For more information, see Amazon ECR encryption at         rest in the Amazon Elastic Container Registry User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionConfiguration {
    ///
    /// The encryption type to use.
    ///
    /// If you use the KMS encryption type, the contents of the repository will       be encrypted using server-side encryption with AWS Key Management Service key stored in AWS KMS. When you       use AWS KMS to encrypt your data, you can either use the default AWS managed AWS KMS key       for Amazon ECR, or specify your own AWS KMS key, which you already created. For more       information, see Protecting data using server-side         encryption with an AWS KMS key stored in AWS Key Management Service (SSE-KMS) in the         Amazon Simple Storage Service Console Developer Guide.
    ///
    /// If you use the AES256 encryption type, Amazon ECR uses server-side encryption       with Amazon S3-managed encryption keys which encrypts the images in the repository using an       AES-256 encryption algorithm. For more information, see Protecting data using         server-side encryption with Amazon S3-managed encryption keys (SSE-S3) in the         Amazon Simple Storage Service Console Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: AES256 | KMS
    ///
    /// Update requires: Replacement
    #[serde(rename = "EncryptionType")]
    pub encryption_type: EncryptionConfigurationEncryptionTypeEnum,

    ///
    /// If you use the KMS encryption type, specify the AWS KMS key to use for       encryption. The alias, key ID, or full ARN of the AWS KMS key can be specified. The key       must exist in the same Region as the repository. If no key is specified, the default       AWS managed AWS KMS key for Amazon ECR will be used.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 2048
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EncryptionConfigurationEncryptionTypeEnum {
    /// AES256
    #[serde(rename = "AES256")]
    Aes256,

    /// KMS
    #[serde(rename = "KMS")]
    Kms,
}

impl Default for EncryptionConfigurationEncryptionTypeEnum {
    fn default() -> Self {
        EncryptionConfigurationEncryptionTypeEnum::Aes256
    }
}

impl cfn_resources::CfnResource for EncryptionConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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

        if let Some(the_val) = &self.kms_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'kms_key'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The image scanning configuration for a repository.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ImageScanningConfiguration {
    ///
    /// The setting that determines whether images are scanned after being pushed to a       repository. If set to true, images will be scanned after being pushed. If       this parameter is not specified, it will default to false and images will       not be scanned unless a scan is manually started.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScanOnPush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_on_push: Option<bool>,
}

impl cfn_resources::CfnResource for ImageScanningConfiguration {
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

/// The LifecyclePolicy property type specifies a lifecycle policy. For       information about lifecycle policy syntax, see Lifecycle policy         template in the Amazon ECR User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LifecyclePolicy {
    ///
    /// The JSON repository policy text to apply to the repository.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 100
    ///
    /// Maximum: 30720
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecyclePolicyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<cfn_resources::StrVal>,

    ///
    /// The AWS account ID associated with the registry that contains the repository. If you       do  not specify a registry, the default registry is assumed.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [0-9]{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegistryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LifecyclePolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.lifecycle_policy_text {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 30720 as _ {
                    return Err(format!("Max validation failed on field 'lifecycle_policy_text'. {} is greater than 30720", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.lifecycle_policy_text {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 100 as _ {
                    return Err(format!("Min validation failed on field 'lifecycle_policy_text'. {} is less than 100", s.len()));
                }
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
