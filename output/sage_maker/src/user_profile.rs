/// Creates a user profile. A user profile represents a single user within a domain, and       is the main way to reference a "person" for the purposes of sharing, reporting, and       other user-oriented features. This entity is created when a user onboards to Amazon       SageMaker Studio. If an administrator invites a person by email or imports them from         IAM Identity Center, a user profile is automatically created. A user profile is the       primary holder of settings for an individual user and has a reference to the user's       private Amazon Elastic File System (EFS) home directory.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnUserProfile {
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
    /// A specifier for the type of value specified in SingleSignOnUserValue. Currently, the       only supported value is "UserName". If the Domain's AuthMode is IAM Identity Center, this       field is required. If the Domain's AuthMode is not IAM Identity Center, this field cannot       be specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SingleSignOnUserIdentifier")]
    pub single_sign_on_user_identifier: Option<String>,

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
    /// A collection of settings that apply to users of Amazon SageMaker Studio.
    ///
    /// Required: No
    ///
    /// Type: UserSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserSettings")]
    pub user_settings: Option<UserSettings>,
}

impl cfn_resources::CfnResource for CfnUserProfile {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::UserProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain_id;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'domain_id'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.user_profile_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'user_profile_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        self.user_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A custom SageMaker image. For more information, see    Bring your own SageMaker image.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub app_image_config_name: String,

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

impl cfn_resources::CfnResource for CustomImage {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.app_image_config_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'app_image_config_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.image_name;

        if the_val.len() > 63 as _ {
            return Err(format!(
                "Max validation failed on field 'image_name'. {} is greater than 63",
                the_val.len()
            ));
        }

        let the_val = &self.image_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'image_name'. {} is less than 1",
                the_val.len()
            ));
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

/// The JupyterServer app settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for JupyterServerAppSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.default_resource_spec
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The KernelGateway app settings.
#[derive(Clone, Debug, Default, serde::Serialize)]
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

impl cfn_resources::CfnResource for KernelGatewayAppSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// Update requires: Replacement
    #[serde(rename = "AccessStatus")]
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
    /// Update requires: Replacement
    #[serde(rename = "UserGroup")]
    pub user_group: Option<RStudioServerProAppSettingsUserGroupEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Serialize)]
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and the instance type that   the version runs on.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub instance_type: Option<ResourceSpecInstanceTypeEnum>,

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
}

#[derive(Clone, Debug, serde::Serialize)]
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.sage_maker_image_arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'sage_maker_image_arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.sage_maker_image_version_arn {
            if the_val.len() > 256 as _ {
                return Err(format!("Max validation failed on field 'sage_maker_image_version_arn'. {} is greater than 256", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Specifies options when sharing an Amazon SageMaker Studio notebook. These settings are       specified as part of DefaultUserSettings when the CreateDomain API is called, and as part of UserSettings when       the CreateUserProfile API is called.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
}

#[derive(Clone, Debug, serde::Serialize)]
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.s3_kms_key_id {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_kms_key_id'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.s3_output_path {
            if the_val.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 's3_output_path'. {} is greater than 1024",
                    the_val.len()
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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A collection of settings that apply to users of Amazon SageMaker Studio. These       settings are specified when the CreateUserProfile API is called, and as DefaultUserSettings       when the CreateDomain API       is called.
///
/// SecurityGroups is aggregated when specified in both calls. For all other       settings in UserSettings, the values specified in         CreateUserProfile take precedence over those specified in         CreateDomain.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UserSettings {
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
    /// Specifies options for sharing SageMaker Studio notebooks.
    ///
    /// Required: No
    ///
    /// Type: SharingSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharingSettings")]
    pub sharing_settings: Option<SharingSettings>,
}

impl cfn_resources::CfnResource for UserSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.execution_role {
            if the_val.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'execution_role'. {} is greater than 2048",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.execution_role {
            if the_val.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'execution_role'. {} is less than 20",
                    the_val.len()
                ));
            }
        }

        self.jupyter_server_app_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.kernel_gateway_app_settings
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
