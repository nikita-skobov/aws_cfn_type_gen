

/// Creates a user profile. A user profile represents a single user within a domain, and       is the main way to reference a "person" for the purposes of sharing, reporting, and       other user-oriented features. This entity is created when a user onboards to Amazon       SageMaker Studio. If an administrator invites a person by email or imports them from         IAM Identity Center, a user profile is automatically created. A user profile is the       primary holder of settings for an individual user and has a reference to the user's       private Amazon Elastic File System (EFS) home directory.
#[derive(Default, serde::Serialize)]
pub struct CfnUserProfile {


    /// 
    /// The username of the associated AWS Single Sign-On User for this       UserProfile. If the Domain's AuthMode is IAM Identity Center, this field is required, and       must match a valid username of a user in your directory. If the Domain's AuthMode is not         IAM Identity Center, this field cannot be specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleSignOnUserValue")]
    pub single_sign_on_user_value: Option<String>,


    /// 
    /// A collection of settings that apply to users of Amazon SageMaker Studio.
    /// 
    /// Required: No
    ///
    /// Type: UserSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSettings")]
    pub user_settings: Option<UserSettings>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// Tags that you specify for the User Profile are also added to all apps that the User       Profile launches.
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
    /// The domain ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainId")]
    pub domain_id: String,


    /// 
    /// The user profile name.
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
    #[serde(rename = "UserProfileName")]
    pub user_profile_name: String,


    /// 
    /// A specifier for the type of value specified in SingleSignOnUserValue. Currently, the       only supported value is "UserName". If the Domain's AuthMode is IAM Identity Center, this       field is required. If the Domain's AuthMode is not IAM Identity Center, this field cannot       be specified.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleSignOnUserIdentifier")]
    pub single_sign_on_user_identifier: Option<String>,

}


/// A collection of settings that configure user interaction with the         RStudioServerPro app. RStudioServerProAppSettings cannot       be updated. The RStudioServerPro app must be deleted and a new one created       to make any changes.
#[derive(Default, serde::Serialize)]
pub struct RStudioServerProAppSettings {


    /// 
    /// The level of permissions that the user has within the RStudioServerPro       app. This value defaults to `User`. The `Admin` value allows the user access to the       RStudio Administrative Dashboard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: R_STUDIO_ADMIN | R_STUDIO_USER
    ///
    /// Update requires: Replacement
    #[serde(rename = "UserGroup")]
    pub user_group: Option<String>,


    /// 
    /// Indicates whether the current user has access to the RStudioServerPro       app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: Replacement
    #[serde(rename = "AccessStatus")]
    pub access_status: Option<String>,

}


/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that   the version runs on.
#[derive(Default, serde::Serialize)]
pub struct ResourceSpec {


    /// 
    /// The instance type that the image version runs on.
    /// 
    /// Note        JupyterServer apps only support the system value.For KernelGateway apps, the system       value is translated to ml.t3.medium. KernelGateway apps also support all other values for available       instance types.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c5.12xlarge | ml.c5.18xlarge | ml.c5.24xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.large | ml.c5.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.geospatial.interactive | ml.m5.12xlarge | ml.m5.16xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.8xlarge | ml.m5.large | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.16xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.8xlarge | ml.m5d.large | ml.m5d.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p3dn.24xlarge | ml.p4d.24xlarge | ml.p4de.24xlarge | ml.r5.12xlarge | ml.r5.16xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.8xlarge | ml.r5.large | ml.r5.xlarge | ml.t3.2xlarge | ml.t3.large | ml.t3.medium | ml.t3.micro | ml.t3.small | ml.t3.xlarge | system
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


    /// 
    /// The ARN of the image version created on the instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws(-[\w]+)*:sagemaker:.+:[0-9]{12}:image-version/[a-z0-9]([-.]?[a-z0-9])*/[0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,


    /// 
    /// The ARN of the SageMaker image that the image version belongs to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws(-[\w]+)*:sagemaker:.+:[0-9]{12}:image/[a-z0-9]([-.]?[a-z0-9])*$
    ///
    /// Update requires: No interruption
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,

}


/// A collection of settings that apply to users of Amazon SageMaker Studio. These       settings are specified when the CreateUserProfile API is called, and as DefaultUserSettings       when the CreateDomain API       is called.
///
/// SecurityGroups is aggregated when specified in both calls. For all other       settings in UserSettings, the values specified in         CreateUserProfile take precedence over those specified in         CreateDomain.
#[derive(Default, serde::Serialize)]
pub struct UserSettings {


    /// 
    /// Specifies options for sharing SageMaker Studio notebooks.
    /// 
    /// Required: No
    ///
    /// Type: SharingSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharingSettings")]
    pub sharing_settings: Option<SharingSettings>,


    /// 
    /// The execution role for the user.
    /// 
    /// Required: No
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
    #[serde(rename = "ExecutionRole")]
    pub execution_role: Option<String>,


    /// 
    /// The security groups for the Amazon Virtual Private Cloud (VPC) that Studio uses for communication.
    /// 
    /// Optional when the CreateDomain.AppNetworkAccessType parameter is set to     PublicInternetOnly.
    /// 
    /// Required when the CreateDomain.AppNetworkAccessType parameter is set to      VpcOnly, unless specified as part of the DefaultUserSettings for the domain.
    /// 
    /// Amazon SageMaker adds a security group to allow NFS traffic from SageMaker Studio. Therefore, the     number of security groups that you can specify is one less than the maximum number shown.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,


    /// 
    /// The Jupyter server's app settings.
    /// 
    /// Required: No
    ///
    /// Type: JupyterServerAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,


    /// 
    /// A collection of settings that configure user interaction with the         RStudioServerPro app.
    /// 
    /// Required: No
    ///
    /// Type: RStudioServerProAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioServerProAppSettings")]
    pub rstudio_server_pro_app_settings: Option<RStudioServerProAppSettings>,


    /// 
    /// The kernel gateway app settings.
    /// 
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,

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


/// The KernelGateway app settings.
#[derive(Default, serde::Serialize)]
pub struct KernelGatewayAppSettings {


    /// 
    /// The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.
    /// 
    /// NoteThe Amazon SageMaker Studio UI does not use the default instance type value set here. The default      instance type set here is used when Apps are created using the AWS Command Line Interface or AWS CloudFormation       and the instance type parameter value is not passed.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,


    /// 
    /// A list of custom SageMaker images that are configured to run as a KernelGateway app.
    /// 
    /// Required: No
    ///
    /// Type: List of CustomImage
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomImages")]
    pub custom_images: Option<Vec<CustomImage>>,

}


/// A custom SageMaker image. For more information, see    Bring your own SageMaker image.
#[derive(Default, serde::Serialize)]
pub struct CustomImage {


    /// 
    /// The name of the CustomImage. Must be unique to your account.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9]([-.]?[a-zA-Z0-9]){0,62}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageName")]
    pub image_name: String,


    /// 
    /// The name of the AppImageConfig.
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
    #[serde(rename = "AppImageConfigName")]
    pub app_image_config_name: String,


    /// 
    /// The version number of the CustomImage.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "ImageVersionNumber")]
    pub image_version_number: Option<i64>,

}


/// The JupyterServer app settings.
#[derive(Default, serde::Serialize)]
pub struct JupyterServerAppSettings {


    /// 
    /// The default instance type and the Amazon Resource Name (ARN) of the default SageMaker       image used by the JupyterServer app.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}


/// Specifies options when sharing an Amazon SageMaker Studio notebook. These settings are       specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when       the CreateUserProfile API is called.
#[derive(Default, serde::Serialize)]
pub struct SharingSettings {


    /// 
    /// When NotebookOutputOption is Allowed, the Amazon S3 bucket used     to store the shared notebook snapshots.
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
    #[serde(rename = "S3OutputPath")]
    pub s3_output_path: Option<String>,


    /// 
    /// Whether to include the notebook cell output when sharing the notebook. The default     is Disabled.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Allowed | Disabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotebookOutputOption")]
    pub notebook_output_option: Option<String>,


    /// 
    /// When NotebookOutputOption is Allowed, the AWS Key Management Service (KMS)     encryption key ID used to encrypt the notebook cell output in the Amazon S3 bucket.
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
    #[serde(rename = "S3KmsKeyId")]
    pub s3_kms_key_id: Option<String>,

}
