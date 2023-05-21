/// Creates a configuration for running a SageMaker image as a KernelGateway app. The     configuration specifies the Amazon Elastic File System (EFS) storage volume on the image, and a list of the     kernels in the image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAppImageConfig {
    ///
    /// The name of the AppImageConfig. Must be unique to your account.
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
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: cfn_resources::StrVal,

    ///
    /// The configuration for the file system and kernels in the SageMaker image.
    ///
    /// Required: No
    ///
    /// Type: KernelGatewayImageConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_gateway_image_config: Option<KernelGatewayImageConfig>,

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
}

impl cfn_resources::CfnResource for CfnAppImageConfig {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::AppImageConfig"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.app_image_config_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'app_image_config_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        self.kernel_gateway_image_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The Amazon Elastic File System (EFS) storage configuration for a SageMaker image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileSystemConfig {
    ///
    /// The default POSIX group ID (GID). If not specified, defaults to 100.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultGid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_gid: Option<i64>,

    ///
    /// The default POSIX user ID (UID). If not specified, defaults to 1000.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_uid: Option<i64>,

    ///
    /// The path within the image to mount the user's EFS home directory. The directory     should be empty. If not specified, defaults to /home/sagemaker-user.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^\/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "MountPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FileSystemConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.default_gid {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'default_gid'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.default_gid {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'default_gid'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.default_uid {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'default_uid'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.default_uid {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'default_uid'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.mount_path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'mount_path'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// The configuration for the file system and kernels in a SageMaker image running as a     KernelGateway app.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KernelGatewayImageConfig {
    ///
    /// The Amazon Elastic File System (EFS) storage configuration for a SageMaker image.
    ///
    /// Required: No
    ///
    /// Type: FileSystemConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_config: Option<FileSystemConfig>,

    ///
    /// The specification of the Jupyter kernels in the image.
    ///
    /// Required: Yes
    ///
    /// Type: List of KernelSpec
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelSpecs")]
    pub kernel_specs: Vec<KernelSpec>,
}

impl cfn_resources::CfnResource for KernelGatewayImageConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file_system_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.kernel_specs;

        if the_val.len() > 1 as _ {
            return Err(format!(
                "Max validation failed on field 'kernel_specs'. {} is greater than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// The specification of a Jupyter kernel.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct KernelSpec {
    ///
    /// The display name of the kernel.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Jupyter kernel in the image. This value is case sensitive.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for KernelSpec {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.display_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'display_name'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 1024",
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
