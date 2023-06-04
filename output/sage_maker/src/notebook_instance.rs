/// The AWS::SageMaker::NotebookInstance resource creates an Amazon SageMaker       notebook instance. A notebook instance is a machine learning (ML) compute instance       running on a Jupyter notebook. For more information, see Use Notebook Instances.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNotebookInstance {
    ///
    /// A list of Amazon Elastic Inference (EI) instance types to associate with the notebook       instance. Currently, only one instance type can be associated with a notebook instance.       For more information, see Using Elastic Inference in Amazon SageMaker.
    ///
    /// Valid Values:       ml.eia1.medium | ml.eia1.large | ml.eia1.xlarge | ml.eia2.medium | ml.eia2.large |         ml.eia2.xlarge.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceleratorTypes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub accelerator_types: Option<Vec<String>>,

    ///
    /// An array of up to three Git repositories associated with the notebook instance. These       can be either the names of Git repositories stored as resources in your account, or the       URL of Git repositories in AWS CodeCommit       or in any other Git repository. These repositories are cloned at the same level as the       default repository of your notebook instance. For more information, see Associating Git         Repositories with SageMaker Notebook Instances.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub additional_code_repositories: Option<Vec<String>>,

    ///
    /// The Git repository associated with the notebook instance as its default code       repository. This can be either the name of a Git repository stored as a resource in your       account, or the URL of a Git repository in AWS CodeCommit       or in any other Git repository. When you open a notebook instance, it opens in the       directory that contains this repository. For more information, see Associating Git         Repositories with SageMaker Notebook Instances.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^https://([^/]+)/?(.*)$|^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub default_code_repository: Option<cfn_resources::StrVal>,

    ///
    /// Sets whether SageMaker provides internet access to the notebook instance. If you set this       to Disabled this notebook instance is able to access resources only in your       VPC, and is not be able to connect to SageMaker training and endpoint services unless you       configure a NAT Gateway in your VPC.
    ///
    /// For more information, see Notebook Instances Are Internet-Enabled by Default. You can set the value       of this parameter to Disabled only if you set a value for the         SubnetId parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: Replacement
    #[serde(rename = "DirectInternetAccess")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub direct_internet_access: Option<NotebookInstanceDirectInternetAccessEnum>,

    ///
    /// Information on the IMDS configuration of the notebook instance
    ///
    /// Required: No
    ///
    /// Type: InstanceMetadataServiceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMetadataServiceConfiguration")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub instance_metadata_service_configuration: Option<InstanceMetadataServiceConfiguration>,

    ///
    /// The type of ML compute instance to launch for the notebook instance.
    ///
    /// NoteExpect some interruption of service if this parameter is changed as CloudFormation         stops a notebook instance and starts it up again to update it.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ml.c4.2xlarge | ml.c4.4xlarge | ml.c4.8xlarge | ml.c4.xlarge | ml.c5.18xlarge | ml.c5.2xlarge | ml.c5.4xlarge | ml.c5.9xlarge | ml.c5.xlarge | ml.c5d.18xlarge | ml.c5d.2xlarge | ml.c5d.4xlarge | ml.c5d.9xlarge | ml.c5d.xlarge | ml.g4dn.12xlarge | ml.g4dn.16xlarge | ml.g4dn.2xlarge | ml.g4dn.4xlarge | ml.g4dn.8xlarge | ml.g4dn.xlarge | ml.g5.12xlarge | ml.g5.16xlarge | ml.g5.24xlarge | ml.g5.2xlarge | ml.g5.48xlarge | ml.g5.4xlarge | ml.g5.8xlarge | ml.g5.xlarge | ml.m4.10xlarge | ml.m4.16xlarge | ml.m4.2xlarge | ml.m4.4xlarge | ml.m4.xlarge | ml.m5.12xlarge | ml.m5.24xlarge | ml.m5.2xlarge | ml.m5.4xlarge | ml.m5.xlarge | ml.m5d.12xlarge | ml.m5d.16xlarge | ml.m5d.24xlarge | ml.m5d.2xlarge | ml.m5d.4xlarge | ml.m5d.8xlarge | ml.m5d.large | ml.m5d.xlarge | ml.p2.16xlarge | ml.p2.8xlarge | ml.p2.xlarge | ml.p3.16xlarge | ml.p3.2xlarge | ml.p3.8xlarge | ml.p3dn.24xlarge | ml.r5.12xlarge | ml.r5.16xlarge | ml.r5.24xlarge | ml.r5.2xlarge | ml.r5.4xlarge | ml.r5.8xlarge | ml.r5.large | ml.r5.xlarge | ml.t2.2xlarge | ml.t2.large | ml.t2.medium | ml.t2.xlarge | ml.t3.2xlarge | ml.t3.large | ml.t3.medium | ml.t3.xlarge
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceType")]
    pub instance_type: NotebookInstanceInstanceTypeEnum,

    ///
    /// The Amazon Resource Name (ARN) of a AWS Key Management Service key that       SageMaker uses to encrypt data on the storage volume attached to your notebook instance. The       KMS key you provide must be enabled. For information, see Enabling and Disabling         Keys in the         AWS Key Management Service Developer         Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub kms_key_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of a lifecycle configuration to associate with the notebook instance. For       information about lifecycle configurations, see Customize a Notebook         Instance in the Amazon SageMaker Developer       Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LifecycleConfigName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub lifecycle_config_name: Option<cfn_resources::StrVal>,

    ///
    /// The name of the new notebook instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^[a-zA-Z0-9](-*[a-zA-Z0-9])*
    ///
    /// Update requires: Replacement
    #[serde(rename = "NotebookInstanceName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub notebook_instance_name: Option<cfn_resources::StrVal>,

    ///
    /// The platform identifier of the notebook instance runtime environment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^(notebook-al1-v1|notebook-al2-v1|notebook-al2-v2)$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlatformIdentifier")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub platform_identifier: Option<cfn_resources::StrVal>,

    ///
    /// When you send any requests to AWS resources from the notebook       instance, SageMaker assumes this role to perform tasks on your behalf. You must grant this       role necessary permissions so SageMaker can perform these tasks. The policy must allow the       SageMaker service principal (sagemaker.amazonaws.com) permissions to assume this role. For       more information, see SageMaker Roles.
    ///
    /// NoteTo be able to pass this role to SageMaker, the caller of this API must have the           iam:PassRole permission.
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
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// Whether root access is enabled or disabled for users of the notebook instance. The       default value is Enabled.
    ///
    /// NoteLifecycle configurations need root access to be able to set up a notebook         instance. Because of this, lifecycle configurations associated with a notebook         instance always run with root access even if you disable root access for         users.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Disabled | Enabled
    ///
    /// Update requires: No interruption
    #[serde(rename = "RootAccess")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub root_access: Option<NotebookInstanceRootAccessEnum>,

    ///
    /// The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be       for the same VPC as specified in the subnet.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The ID of the subnet in a VPC to which you would like to have a connectivity from       your ML compute instance.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Pattern: [-0-9a-zA-Z]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subnet_id: Option<cfn_resources::StrVal>,

    ///
    /// A list of key-value pairs to apply to this resource.
    ///
    /// For more information, see Resource         Tag and Using         Cost Allocation Tags.
    ///
    /// You can add tags later by using the CreateTags API.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The size, in GB, of the ML storage volume to attach to the notebook instance. The       default value is 5 GB.
    ///
    /// NoteExpect some interruption of service if this parameter is changed as CloudFormation         stops a notebook instance and starts it up again to update it.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 5
    ///
    /// Maximum: 16384
    ///
    /// Update requires: No interruption
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub volume_size_in_gb: Option<i64>,

    #[serde(skip_serializing)]
    pub att_notebook_instance_name: CfnNotebookInstancenotebookinstancename,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotebookInstanceDirectInternetAccessEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for NotebookInstanceDirectInternetAccessEnum {
    fn default() -> Self {
        NotebookInstanceDirectInternetAccessEnum::Disabled
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotebookInstanceInstanceTypeEnum {
    /// ml.c4.2xlarge
    #[serde(rename = "ml.c4.2xlarge")]
    Mlc42xlarge,

    /// ml.c4.4xlarge
    #[serde(rename = "ml.c4.4xlarge")]
    Mlc44xlarge,

    /// ml.c4.8xlarge
    #[serde(rename = "ml.c4.8xlarge")]
    Mlc48xlarge,

    /// ml.c4.xlarge
    #[serde(rename = "ml.c4.xlarge")]
    Mlc4xlarge,

    /// ml.c5.18xlarge
    #[serde(rename = "ml.c5.18xlarge")]
    Mlc518xlarge,

    /// ml.c5.2xlarge
    #[serde(rename = "ml.c5.2xlarge")]
    Mlc52xlarge,

    /// ml.c5.4xlarge
    #[serde(rename = "ml.c5.4xlarge")]
    Mlc54xlarge,

    /// ml.c5.9xlarge
    #[serde(rename = "ml.c5.9xlarge")]
    Mlc59xlarge,

    /// ml.c5.xlarge
    #[serde(rename = "ml.c5.xlarge")]
    Mlc5xlarge,

    /// ml.c5d.18xlarge
    #[serde(rename = "ml.c5d.18xlarge")]
    Mlc5d18xlarge,

    /// ml.c5d.2xlarge
    #[serde(rename = "ml.c5d.2xlarge")]
    Mlc5d2xlarge,

    /// ml.c5d.4xlarge
    #[serde(rename = "ml.c5d.4xlarge")]
    Mlc5d4xlarge,

    /// ml.c5d.9xlarge
    #[serde(rename = "ml.c5d.9xlarge")]
    Mlc5d9xlarge,

    /// ml.c5d.xlarge
    #[serde(rename = "ml.c5d.xlarge")]
    Mlc5dxlarge,

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

    /// ml.m4.10xlarge
    #[serde(rename = "ml.m4.10xlarge")]
    Mlm410xlarge,

    /// ml.m4.16xlarge
    #[serde(rename = "ml.m4.16xlarge")]
    Mlm416xlarge,

    /// ml.m4.2xlarge
    #[serde(rename = "ml.m4.2xlarge")]
    Mlm42xlarge,

    /// ml.m4.4xlarge
    #[serde(rename = "ml.m4.4xlarge")]
    Mlm44xlarge,

    /// ml.m4.xlarge
    #[serde(rename = "ml.m4.xlarge")]
    Mlm4xlarge,

    /// ml.m5.12xlarge
    #[serde(rename = "ml.m5.12xlarge")]
    Mlm512xlarge,

    /// ml.m5.24xlarge
    #[serde(rename = "ml.m5.24xlarge")]
    Mlm524xlarge,

    /// ml.m5.2xlarge
    #[serde(rename = "ml.m5.2xlarge")]
    Mlm52xlarge,

    /// ml.m5.4xlarge
    #[serde(rename = "ml.m5.4xlarge")]
    Mlm54xlarge,

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

    /// ml.p2.16xlarge
    #[serde(rename = "ml.p2.16xlarge")]
    Mlp216xlarge,

    /// ml.p2.8xlarge
    #[serde(rename = "ml.p2.8xlarge")]
    Mlp28xlarge,

    /// ml.p2.xlarge
    #[serde(rename = "ml.p2.xlarge")]
    Mlp2xlarge,

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

    /// ml.t2.2xlarge
    #[serde(rename = "ml.t2.2xlarge")]
    Mlt22xlarge,

    /// ml.t2.large
    #[serde(rename = "ml.t2.large")]
    Mlt2large,

    /// ml.t2.medium
    #[serde(rename = "ml.t2.medium")]
    Mlt2medium,

    /// ml.t2.xlarge
    #[serde(rename = "ml.t2.xlarge")]
    Mlt2xlarge,

    /// ml.t3.2xlarge
    #[serde(rename = "ml.t3.2xlarge")]
    Mlt32xlarge,

    /// ml.t3.large
    #[serde(rename = "ml.t3.large")]
    Mlt3large,

    /// ml.t3.medium
    #[serde(rename = "ml.t3.medium")]
    Mlt3medium,

    /// ml.t3.xlarge
    #[serde(rename = "ml.t3.xlarge")]
    Mlt3xlarge,
}

impl Default for NotebookInstanceInstanceTypeEnum {
    fn default() -> Self {
        NotebookInstanceInstanceTypeEnum::Mlc42xlarge
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NotebookInstanceRootAccessEnum {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled,

    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled,
}

impl Default for NotebookInstanceRootAccessEnum {
    fn default() -> Self {
        NotebookInstanceRootAccessEnum::Disabled
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNotebookInstancenotebookinstancename;
impl CfnNotebookInstancenotebookinstancename {
    pub fn att_name(&self) -> &'static str {
        r#"NotebookInstanceName"#
    }
}

impl cfn_resources::CfnResource for CfnNotebookInstance {
    fn type_string(&self) -> &'static str {
        "AWS::SageMaker::NotebookInstance"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.additional_code_repositories {
            if the_val.len() > 3 as _ {
                return Err(format!("Max validation failed on field 'additional_code_repositories'. {} is greater than 3", the_val.len()));
            }
        }

        if let Some(the_val) = &self.default_code_repository {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!("Max validation failed on field 'default_code_repository'. {} is greater than 1024", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.default_code_repository {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'default_code_repository'. {} is less than 1", s.len()));
                }
            }
        }

        self.instance_metadata_service_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.kms_key_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'kms_key_id'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.lifecycle_config_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'lifecycle_config_name'. {} is greater than 63", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.notebook_instance_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!("Max validation failed on field 'notebook_instance_name'. {} is greater than 63", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.platform_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 15 as _ {
                    return Err(format!("Max validation failed on field 'platform_identifier'. {} is greater than 15", s.len()));
                }
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 2048 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 2048",
                    s.len()
                ));
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 20 as _ {
                return Err(format!(
                    "Min validation failed on field 'role_arn'. {} is less than 20",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.security_group_ids {
            if the_val.len() > 5 as _ {
                return Err(format!(
                    "Max validation failed on field 'security_group_ids'. {} is greater than 5",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.subnet_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 32 as _ {
                    return Err(format!(
                        "Max validation failed on field 'subnet_id'. {} is greater than 32",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.volume_size_in_gb {
            if *the_val > 16384 as _ {
                return Err(format!(
                    "Max validation failed on field 'volume_size_in_gb'. {} is greater than 16384",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.volume_size_in_gb {
            if *the_val < 5 as _ {
                return Err(format!(
                    "Min validation failed on field 'volume_size_in_gb'. {} is less than 5",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Information on the IMDS configuration of the notebook instance
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InstanceMetadataServiceConfiguration {
    ///
    /// Indicates the minimum IMDS version that the notebook instance supports. When passed as part of CreateNotebookInstance, if no value is selected, then it defaults to IMDSv1. This means that both IMDSv1 and IMDSv2 are supported. If passed as part of UpdateNotebookInstance, there is no default.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1
    ///
    /// Pattern: 1|2
    ///
    /// Update requires: No interruption
    #[serde(rename = "MinimumInstanceMetadataServiceVersion")]
    pub minimum_instance_metadata_service_version: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for InstanceMetadataServiceConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.minimum_instance_metadata_service_version;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1 as _ {
                return Err(format!("Max validation failed on field 'minimum_instance_metadata_service_version'. {} is greater than 1", s.len()));
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
