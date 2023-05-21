

/// Creates a Domain used by Amazon SageMaker Studio. A domain consists of an associated   Amazon Elastic File System (EFS) volume, a list of authorized users, and a variety of security, application,   policy, and Amazon Virtual Private Cloud (VPC) configurations.   Users within a domain can share notebook files and other artifacts with each other.
///
/// EFS storage
///
/// When a domain is created, an EFS volume is created for use by all of the users within the   domain. Each user receives a private home directory within the EFS volume for notebooks,   Git repositories, and data files.
///
/// SageMaker uses the AWS Key Management Service (AWS KMS) to encrypt the EFS volume attached to the domain with   an AWS managed key by default. For more control, you can specify a   customer managed key. For more information, see   Protect Data at    Rest Using Encryption.
///
/// VPC configuration
///
/// All SageMaker Studio traffic between the domain and the EFS volume is through the specified   VPC and subnets. For other Studio traffic, you can specify the AppNetworkAccessType   parameter. AppNetworkAccessType corresponds to the network access type that you   choose when you onboard to Studio. The following options are available:
///
/// For more information, see   Connect    SageMaker Studio Notebooks to Resources in a VPC.
#[derive(Default, serde::Serialize)]
pub struct CfnDomain {


    /// 
    /// The VPC subnets that Studio uses for communication.
    /// 
    /// Length Constraints: Maximum length of 32.
    /// 
    /// Array members: Minimum number of 1 item. Maximum number of 16       items.
    /// 
    /// Pattern: [-0-9a-zA-Z]+
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// 
    /// The entity that creates and manages the required security groups for inter-app       communication in VpcOnly mode. Required when         CreateDomain.AppNetworkAccessType is VpcOnly and         DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn is       provided. If setting up the domain for use with RStudio, this value must be set to Service.
    /// 
    /// Allowed Values: Service | Customer
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppSecurityGroupManagement")]
    pub app_security_group_management: Option<String>,


    /// 
    /// Specifies the VPC used for non-EFS traffic. The default value is         PublicInternetOnly.
    /// 
    /// PublicInternetOnly - Non-EFS traffic is through a VPC managed by             Amazon SageMaker, which allows direct internet access               VpcOnly - All Studio traffic is through the specified VPC and           subnets
    /// 
    /// Valid Values: PublicInternetOnly | VpcOnly
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AppNetworkAccessType")]
    pub app_network_access_type: Option<String>,


    /// 
    /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) that Studio uses for       communication.
    /// 
    /// Length Constraints: Maximum length of 32.
    /// 
    /// Pattern: [-0-9a-zA-Z]+
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DefaultSpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultSpaceSettings")]
    pub default_space_settings: Option<DefaultSpaceSettings>,


    /// 
    /// The mode of authentication that members use to access the Domain.
    /// 
    /// Valid Values: SSO | IAM
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AuthMode")]
    pub auth_mode: String,


    /// 
    /// Tags to associated with the Domain. Each tag consists of a key and an optional value.       Tag keys must be unique per resource. Tags are searchable using the Search API.
    /// 
    /// Tags that you specify for the Domain are also added to all apps that are launched in       the Domain.
    /// 
    /// Array members: Minimum number of 0 items. Maximum number of 50       items.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The default user settings.
    /// 
    /// Required: Yes
    ///
    /// Type: UserSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultUserSettings")]
    pub default_user_settings: UserSettings,


    /// 
    /// SageMaker uses AWS KMS to encrypt the EFS volume attached to the       Domain with an AWS managed customer master key (CMK) by default. For       more control, specify a customer managed CMK.
    /// 
    /// Length Constraints: Maximum length of 2048.
    /// 
    /// Pattern: .*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,


    /// 
    /// The domain name.
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
    #[serde(rename = "DomainName")]
    pub domain_name: String,


    /// 
    /// A collection of settings that apply to the SageMaker Domain. These       settings are specified through the CreateDomain API call.
    /// 
    /// Required: No
    ///
    /// Type: DomainSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainSettings")]
    pub domain_settings: Option<DomainSettings>,

}


/// The KernelGateway app settings.
#[derive(Default, serde::Serialize)]
pub struct KernelGatewayAppSettings {


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
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,


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
    /// Update requires: Some interruptions
    #[serde(rename = "SageMakerImageArn")]
    pub sage_maker_image_arn: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the Lifecycle Configuration attached to the Resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: arn:aws[a-z\-]*:sagemaker:[a-z0-9\-]*:[0-9]{12}:studio-lifecycle-config/.*
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "LifecycleConfigArn")]
    pub lifecycle_config_arn: Option<String>,


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
    /// Update requires: Some interruptions
    #[serde(rename = "SageMakerImageVersionArn")]
    pub sage_maker_image_version_arn: Option<String>,

}


/// A collection of settings that apply to the SageMaker Domain. These       settings are specified through the CreateDomain API call.
#[derive(Default, serde::Serialize)]
pub struct DomainSettings {


    /// 
    /// The security groups for the Amazon Virtual Private Cloud that the Domain uses for       communication between Domain-level apps and user apps.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// A collection of settings that configure the RStudioServerPro Domain-level       app.
    /// 
    /// Required: No
    ///
    /// Type: RStudioServerProDomainSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioServerProDomainSettings")]
    pub rstudio_server_pro_domain_settings: Option<RStudioServerProDomainSettings>,

}


/// A collection of settings that apply to an RSessionGateway app.
#[derive(Default, serde::Serialize)]
pub struct RSessionAppSettings {


    /// 
    /// Specifies the ARNs of a SageMaker image and SageMaker image version, and the instance       type that the version runs on.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,


    /// 
    /// A list of custom SageMaker images that are configured to run as a RSession app.
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


/// A collection of settings that configure the RStudioServerPro Domain-level       app.
#[derive(Default, serde::Serialize)]
pub struct RStudioServerProDomainSettings {


    /// 
    /// A URL pointing to an RStudio Package Manager server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioPackageManagerUrl")]
    pub rstudio_package_manager_url: Option<String>,


    /// 
    /// The ARN of the execution role for the RStudioServerPro Domain-level       app.
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
    /// Update requires: No interruption
    #[serde(rename = "DomainExecutionRoleArn")]
    pub domain_execution_role_arn: String,


    /// 
    /// A URL pointing to an RStudio Connect server.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioConnectUrl")]
    pub rstudio_connect_url: Option<String>,


    /// 
    /// A collection that defines the default InstanceType,         SageMakerImageArn, and SageMakerImageVersionArn for the       Domain.
    /// 
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: Replacement
    #[serde(rename = "DefaultResourceSpec")]
    pub default_resource_spec: Option<ResourceSpec>,

}


/// A collection of settings that apply to users of Amazon SageMaker Studio. These       settings are specified when the CreateUserProfile API is called, and as DefaultUserSettings       when the CreateDomain API       is called.
///
/// SecurityGroups is aggregated when specified in both calls. For all other       settings in UserSettings, the values specified in         CreateUserProfile take precedence over those specified in         CreateDomain.
#[derive(Default, serde::Serialize)]
pub struct UserSettings {


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
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,


    /// 
    /// A collection of settings that configure the RSessionGateway app.
    /// 
    /// Required: No
    ///
    /// Type: RSessionAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RSessionAppSettings")]
    pub rsession_app_settings: Option<RSessionAppSettings>,


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
    /// The kernel gateway app settings.
    /// 
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,


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


/// A collection of settings that apply to spaces created in the Domain.
#[derive(Default, serde::Serialize)]
pub struct DefaultSpaceSettings {


    /// 
    /// The JupyterServer app settings.
    /// 
    /// Required: No
    ///
    /// Type: JupyterServerAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "JupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,


    /// 
    /// The security group IDs for the Amazon Virtual Private Cloud that the space uses for communication.
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
    /// The KernelGateway app settings.
    /// 
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,


    /// 
    /// The ARN of the execution role for the space.
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
    /// Update requires: No interruption
    #[serde(rename = "ExecutionRole")]
    pub execution_role: String,

}


/// A custom SageMaker image. For more information, see    Bring your own SageMaker image.
#[derive(Default, serde::Serialize)]
pub struct CustomImage {


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

}


/// Specifies options when sharing an Amazon SageMaker Studio notebook. These settings are       specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when       the CreateUserProfile API is called.
#[derive(Default, serde::Serialize)]
pub struct SharingSettings {


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


/// A collection of settings that configure user interaction with the         RStudioServerPro app. RStudioServerProAppSettings cannot       be updated. The RStudioServerPro app must be deleted and a new one created       to make any changes.
#[derive(Default, serde::Serialize)]
pub struct RStudioServerProAppSettings {


    /// 
    /// Indicates whether the current user has access to the RStudioServerPro       app.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessStatus")]
    pub access_status: Option<String>,


    /// 
    /// The level of permissions that the user has within the RStudioServerPro       app. This value defaults to `User`. The `Admin` value allows the user access to the       RStudio Administrative Dashboard.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: R_STUDIO_ADMIN | R_STUDIO_USER
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserGroup")]
    pub user_group: Option<String>,

}