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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDomain {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_network_access_type: Option<DomainAppNetworkAccessTypeEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_security_group_management: Option<DomainAppSecurityGroupManagementEnum>,

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
    pub auth_mode: DomainAuthModeEnum,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DefaultSpaceSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_space_settings: Option<DefaultSpaceSettings>,

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
    pub domain_name: cfn_resources::StrVal,

    ///
    /// A collection of settings that apply to the SageMaker Domain. These       settings are specified through the CreateDomain API call.
    ///
    /// Required: No
    ///
    /// Type: DomainSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_settings: Option<DomainSettings>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

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
    pub vpc_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_domain_arn: CfnDomaindomainarn,

    #[serde(skip_serializing)]
    pub att_domain_id: CfnDomaindomainid,

    #[serde(skip_serializing)]
    pub att_home_efs_file_system_id: CfnDomainhomeefsfilesystemid,

    #[serde(skip_serializing)]
    pub att_security_group_id_for_domain_boundary: CfnDomainsecuritygroupidfordomainboundary,

    #[serde(skip_serializing)]
    pub att_single_sign_on_managed_application_instance_id:
        CfnDomainsinglesignonmanagedapplicationinstanceid,

    #[serde(skip_serializing)]
    pub att_url: CfnDomainurl,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DomainAppNetworkAccessTypeEnum {
    /// PublicInternetOnly
    #[serde(rename = "PublicInternetOnly")]
    Publicinternetonly,

    /// VpcOnly
    #[serde(rename = "VpcOnly")]
    Vpconly,
}

impl Default for DomainAppNetworkAccessTypeEnum {
    fn default() -> Self {
        DomainAppNetworkAccessTypeEnum::Publicinternetonly
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DomainAppSecurityGroupManagementEnum {
    /// Service
    #[serde(rename = "Service")]
    Service,

    /// Customer
    #[serde(rename = "Customer")]
    Customer,
}

impl Default for DomainAppSecurityGroupManagementEnum {
    fn default() -> Self {
        DomainAppSecurityGroupManagementEnum::Service
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DomainAuthModeEnum {
    /// SSO
    #[serde(rename = "SSO")]
    Sso,

    /// IAM
    #[serde(rename = "IAM")]
    Iam,
}

impl Default for DomainAuthModeEnum {
    fn default() -> Self {
        DomainAuthModeEnum::Sso
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomaindomainarn;
impl CfnDomaindomainarn {
    pub fn att_name(&self) -> &'static str {
        r#"DomainArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomaindomainid;
impl CfnDomaindomainid {
    pub fn att_name(&self) -> &'static str {
        r#"DomainId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainhomeefsfilesystemid;
impl CfnDomainhomeefsfilesystemid {
    pub fn att_name(&self) -> &'static str {
        r#"HomeEfsFileSystemId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainsecuritygroupidfordomainboundary;
impl CfnDomainsecuritygroupidfordomainboundary {
    pub fn att_name(&self) -> &'static str {
        r#"SecurityGroupIdForDomainBoundary"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainsinglesignonmanagedapplicationinstanceid;
impl CfnDomainsinglesignonmanagedapplicationinstanceid {
    pub fn att_name(&self) -> &'static str {
        r#"SingleSignOnManagedApplicationInstanceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDomainurl;
impl CfnDomainurl {
    pub fn att_name(&self) -> &'static str {
        r#"Url"#
    }
}

impl cfn_resources::CfnResource for CfnDomain {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::Domain"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_space_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.default_user_settings.validate()?;

        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        self.domain_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A custom SageMaker image. For more information, see    Bring your own SageMaker image.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CustomImage {
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
    pub app_image_config_name: cfn_resources::StrVal,

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
    pub image_name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_version_number: Option<i64>,
}

impl cfn_resources::CfnResource for CustomImage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
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

        let the_val = &self.image_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'image_name'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.image_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'image_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.image_version_number {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'image_version_number'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// A collection of settings that apply to spaces created in the Domain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DefaultSpaceSettings {
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
    pub execution_role: cfn_resources::StrVal,

    ///
    /// The JupyterServer app settings.
    ///
    /// Required: No
    ///
    /// Type: JupyterServerAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "JupyterServerAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,

    ///
    /// The KernelGateway app settings.
    ///
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for DefaultSpaceSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.execution_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'execution_role'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.execution_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'execution_role'. {} is less than 20",
                    s.len()
                ));
            }
        }

        self.jupyter_server_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kernel_gateway_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.security_groups {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_groups'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A collection of settings that apply to the SageMaker Domain. These       settings are specified through the CreateDomain API call.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DomainSettings {
    ///
    /// A collection of settings that configure the RStudioServerPro Domain-level       app.
    ///
    /// Required: No
    ///
    /// Type: RStudioServerProDomainSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioServerProDomainSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rstudio_server_pro_domain_settings: Option<RStudioServerProDomainSettings>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for DomainSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.rstudio_server_pro_domain_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 3 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 3",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The JupyterServer app settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

impl cfn_resources::CfnResource for JupyterServerAppSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_resource_spec
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The KernelGateway app settings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

impl cfn_resources::CfnResource for KernelGatewayAppSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_images {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_images'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        self.default_resource_spec
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A collection of settings that apply to an RSessionGateway app.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RSessionAppSettings {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_images: Option<Vec<CustomImage>>,

    ///
    /// Specifies the ARNs of a SageMaker image and SageMaker image version, and the instance       type that the version runs on.
    ///
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,
}

impl cfn_resources::CfnResource for RSessionAppSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_images {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_images'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        self.default_resource_spec
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A collection of settings that configure user interaction with the         RStudioServerPro app. RStudioServerProAppSettings cannot       be updated. The RStudioServerPro app must be deleted and a new one created       to make any changes.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_status: Option<RStudioServerProAppSettingsAccessStatusEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group: Option<RStudioServerProAppSettingsUserGroupEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RStudioServerProAppSettingsAccessStatusEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,
}

impl Default for RStudioServerProAppSettingsAccessStatusEnum {
    fn default() -> Self {
        RStudioServerProAppSettingsAccessStatusEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RStudioServerProAppSettingsUserGroupEnum {
    /// R_STUDIO_ADMIN
    #[serde(rename = "R_STUDIO_ADMIN")]
    Rstudioadmin,

    /// R_STUDIO_USER
    #[serde(rename = "R_STUDIO_USER")]
    Rstudiouser,
}

impl Default for RStudioServerProAppSettingsUserGroupEnum {
    fn default() -> Self {
        RStudioServerProAppSettingsUserGroupEnum::Rstudioadmin
    }
}

impl cfn_resources::CfnResource for RStudioServerProAppSettings {
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

/// A collection of settings that configure the RStudioServerPro Domain-level       app.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RStudioServerProDomainSettings {
    ///
    /// A collection that defines the default InstanceType,         SageMakerImageArn, and SageMakerImageVersionArn for the       Domain.
    ///
    /// Required: No
    ///
    /// Type: ResourceSpec
    ///
    /// Update requires: Replacement
    #[serde(rename = "DefaultResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_spec: Option<ResourceSpec>,

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
    pub domain_execution_role_arn: cfn_resources::StrVal,

    ///
    /// A URL pointing to an RStudio Connect server.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioConnectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rstudio_connect_url: Option<cfn_resources::StrVal>,

    ///
    /// A URL pointing to an RStudio Package Manager server.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioPackageManagerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rstudio_package_manager_url: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RStudioServerProDomainSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_resource_spec
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.domain_execution_role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!("Max validation failed on field 'domain_execution_role_arn'. {} is greater than 2048", s.len()));
            }
        }

        let the_val = &self.domain_execution_role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!("Min validation failed on field 'domain_execution_role_arn'. {} is less than 20", s.len()));
            }
        }

        Ok(())
    }
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that   the version runs on.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<ResourceSpecInstanceTypeEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_config_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_image_arn: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_image_version_arn: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResourceSpecInstanceTypeEnum {
    /// ml.c5.12xlarge
    #[serde(rename = "ml.c5.12xlarge")]
    Mlc512xlarge,

    /// ml.c5.18xlarge
    #[serde(rename = "ml.c5.18xlarge")]
    Mlc518xlarge,

    /// ml.c5.24xlarge
    #[serde(rename = "ml.c5.24xlarge")]
    Mlc524xlarge,

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

    /// ml.geospatial.interactive
    #[serde(rename = "ml.geospatial.interactive")]
    Mlgeospatialinteractive,

    /// ml.m5.12xlarge
    #[serde(rename = "ml.m5.12xlarge")]
    Mlm512xlarge,

    /// ml.m5.16xlarge
    #[serde(rename = "ml.m5.16xlarge")]
    Mlm516xlarge,

    /// ml.m5.24xlarge
    #[serde(rename = "ml.m5.24xlarge")]
    Mlm524xlarge,

    /// ml.m5.2xlarge
    #[serde(rename = "ml.m5.2xlarge")]
    Mlm52xlarge,

    /// ml.m5.4xlarge
    #[serde(rename = "ml.m5.4xlarge")]
    Mlm54xlarge,

    /// ml.m5.8xlarge
    #[serde(rename = "ml.m5.8xlarge")]
    Mlm58xlarge,

    /// ml.m5.large
    #[serde(rename = "ml.m5.large")]
    Mlm5large,

    /// ml.m5.xlarge
    #[serde(rename = "ml.m5.xlarge")]
    Mlm5xlarge,

    /// ml.m5d.12xlarge
    #[serde(rename = "ml.m5d.12xlarge")]
    Mlm5d12xlarge,

    /// ml.m5d.16xlarge
    #[serde(rename = "ml.m5d.16xlarge")]
    Mlm5d16xlarge,

    /// ml.m5d.24xlarge
    #[serde(rename = "ml.m5d.24xlarge")]
    Mlm5d24xlarge,

    /// ml.m5d.2xlarge
    #[serde(rename = "ml.m5d.2xlarge")]
    Mlm5d2xlarge,

    /// ml.m5d.4xlarge
    #[serde(rename = "ml.m5d.4xlarge")]
    Mlm5d4xlarge,

    /// ml.m5d.8xlarge
    #[serde(rename = "ml.m5d.8xlarge")]
    Mlm5d8xlarge,

    /// ml.m5d.large
    #[serde(rename = "ml.m5d.large")]
    Mlm5dlarge,

    /// ml.m5d.xlarge
    #[serde(rename = "ml.m5d.xlarge")]
    Mlm5dxlarge,

    /// ml.p3.16xlarge
    #[serde(rename = "ml.p3.16xlarge")]
    Mlp316xlarge,

    /// ml.p3.2xlarge
    #[serde(rename = "ml.p3.2xlarge")]
    Mlp32xlarge,

    /// ml.p3.8xlarge
    #[serde(rename = "ml.p3.8xlarge")]
    Mlp38xlarge,

    /// ml.p3dn.24xlarge
    #[serde(rename = "ml.p3dn.24xlarge")]
    Mlp3dn24xlarge,

    /// ml.p4d.24xlarge
    #[serde(rename = "ml.p4d.24xlarge")]
    Mlp4d24xlarge,

    /// ml.p4de.24xlarge
    #[serde(rename = "ml.p4de.24xlarge")]
    Mlp4de24xlarge,

    /// ml.r5.12xlarge
    #[serde(rename = "ml.r5.12xlarge")]
    Mlr512xlarge,

    /// ml.r5.16xlarge
    #[serde(rename = "ml.r5.16xlarge")]
    Mlr516xlarge,

    /// ml.r5.24xlarge
    #[serde(rename = "ml.r5.24xlarge")]
    Mlr524xlarge,

    /// ml.r5.2xlarge
    #[serde(rename = "ml.r5.2xlarge")]
    Mlr52xlarge,

    /// ml.r5.4xlarge
    #[serde(rename = "ml.r5.4xlarge")]
    Mlr54xlarge,

    /// ml.r5.8xlarge
    #[serde(rename = "ml.r5.8xlarge")]
    Mlr58xlarge,

    /// ml.r5.large
    #[serde(rename = "ml.r5.large")]
    Mlr5large,

    /// ml.r5.xlarge
    #[serde(rename = "ml.r5.xlarge")]
    Mlr5xlarge,

    /// ml.t3.2xlarge
    #[serde(rename = "ml.t3.2xlarge")]
    Mlt32xlarge,

    /// ml.t3.large
    #[serde(rename = "ml.t3.large")]
    Mlt3large,

    /// ml.t3.medium
    #[serde(rename = "ml.t3.medium")]
    Mlt3medium,

    /// ml.t3.micro
    #[serde(rename = "ml.t3.micro")]
    Mlt3micro,

    /// ml.t3.small
    #[serde(rename = "ml.t3.small")]
    Mlt3small,

    /// ml.t3.xlarge
    #[serde(rename = "ml.t3.xlarge")]
    Mlt3xlarge,

    /// system
    #[serde(rename = "system")]
    System,
}

impl Default for ResourceSpecInstanceTypeEnum {
    fn default() -> Self {
        ResourceSpecInstanceTypeEnum::Mlc512xlarge
    }
}

impl cfn_resources::CfnResource for ResourceSpec {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.lifecycle_config_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'lifecycle_config_arn'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.sage_maker_image_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'sage_maker_image_arn'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.sage_maker_image_version_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'sage_maker_image_version_arn'. {} is greater than 256", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// Specifies options when sharing an Amazon SageMaker Studio notebook. These settings are       specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when       the CreateUserProfile API is called.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SharingSettings {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_output_option: Option<SharingSettingsNotebookOutputOptionEnum>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_kms_key_id: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_path: Option<cfn_resources::StrVal>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SharingSettingsNotebookOutputOptionEnum {
    /// Allowed
    #[serde(rename = "Allowed")]
    Allowed,

    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for SharingSettingsNotebookOutputOptionEnum {
    fn default() -> Self {
        SharingSettingsNotebookOutputOptionEnum::Allowed
    }
}

impl cfn_resources::CfnResource for SharingSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.s3_kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_kms_key_id'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.s3_output_path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 's3_output_path'. {} is greater than 1024",
                        s.len()
                    ));
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

/// A collection of settings that apply to users of Amazon SageMaker Studio. These       settings are specified when the CreateUserProfile API is called, and as DefaultUserSettings       when the CreateDomain API       is called.
///
/// SecurityGroups is aggregated when specified in both calls. For all other       settings in UserSettings, the values specified in         CreateUserProfile take precedence over those specified in         CreateDomain.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserSettings {
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
    pub execution_role: cfn_resources::StrVal,

    ///
    /// The Jupyter server's app settings.
    ///
    /// Required: No
    ///
    /// Type: JupyterServerAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "JupyterServerAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jupyter_server_app_settings: Option<JupyterServerAppSettings>,

    ///
    /// The kernel gateway app settings.
    ///
    /// Required: No
    ///
    /// Type: KernelGatewayAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "KernelGatewayAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_gateway_app_settings: Option<KernelGatewayAppSettings>,

    ///
    /// A collection of settings that configure the RSessionGateway app.
    ///
    /// Required: No
    ///
    /// Type: RSessionAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RSessionAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsession_app_settings: Option<RSessionAppSettings>,

    ///
    /// A collection of settings that configure user interaction with the         RStudioServerPro app.
    ///
    /// Required: No
    ///
    /// Type: RStudioServerProAppSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "RStudioServerProAppSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rstudio_server_pro_app_settings: Option<RStudioServerProAppSettings>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// Specifies options for sharing SageMaker Studio notebooks.
    ///
    /// Required: No
    ///
    /// Type: SharingSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_settings: Option<SharingSettings>,
}

impl cfn_resources::CfnResource for UserSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.execution_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'execution_role'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.execution_role;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'execution_role'. {} is less than 20",
                    s.len()
                ));
            }
        }

        self.jupyter_server_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kernel_gateway_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rsession_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rstudio_server_pro_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.security_groups {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_groups'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        self.sharing_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
