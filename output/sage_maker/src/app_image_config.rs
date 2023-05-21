

/// Creates a configuration for running a SageMaker image as a KernelGateway app. The     configuration specifies the Amazon Elastic File System (EFS) storage volume on the image, and a list of the     kernels in the image.
#[derive(Default, serde::Serialize)]
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
    pub app_image_config_name: String,


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
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The configuration for the file system and kernels in the SageMaker image.
    /// 
    /// Required: No
    ///
    /// Type: KernelGatewayImageConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayImageConfig")]
    pub kernel_gateway_image_config: Option<KernelGatewayImageConfig>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}


/// The specification of a Jupyter kernel.
#[derive(Default, serde::Serialize)]
pub struct KernelSpec {


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
    pub name: String,


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
    pub display_name: Option<String>,

}


/// The configuration for the file system and kernels in a SageMaker image running as a     KernelGateway app.
#[derive(Default, serde::Serialize)]
pub struct KernelGatewayImageConfig {


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


    /// 
    /// The Amazon Elastic File System (EFS) storage configuration for a SageMaker image.
    /// 
    /// Required: No
    ///
    /// Type: FileSystemConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileSystemConfig")]
    pub file_system_config: Option<FileSystemConfig>,

}


/// The Amazon Elastic File System (EFS) storage configuration for a SageMaker image.
#[derive(Default, serde::Serialize)]
pub struct FileSystemConfig {


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
    pub mount_path: Option<String>,


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
    pub default_uid: Option<i64>,

}
