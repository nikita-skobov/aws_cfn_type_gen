

/// The AWS::SageMaker::NotebookInstance resource creates an Amazon SageMaker       notebook instance. A notebook instance is a machine learning (ML) compute instance       running on a Jupyter notebook. For more information, see Use Notebook Instances.
#[derive(Default, serde::Serialize)]
pub struct CfnNotebookInstance {


    /// 
    /// Information on the IMDS configuration of the notebook instance
    /// 
    /// Required: No
    ///
    /// Type: InstanceMetadataServiceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceMetadataServiceConfiguration")]
    pub instance_metadata_service_configuration: Option<InstanceMetadataServiceConfiguration>,


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
    pub direct_internet_access: Option<String>,


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
    pub role_arn: String,


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
    pub additional_code_repositories: Option<Vec<String>>,


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
    pub security_group_ids: Option<Vec<String>>,


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
    pub volume_size_in_gb: Option<i64>,


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
    pub instance_type: String,


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
    pub lifecycle_config_name: Option<String>,


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
    pub tags: Option<Vec<Tag>>,


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
    pub default_code_repository: Option<String>,


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
    pub accelerator_types: Option<Vec<String>>,


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
    pub platform_identifier: Option<String>,


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
    pub kms_key_id: Option<String>,


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
    pub subnet_id: Option<String>,


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
    pub root_access: Option<String>,


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
    pub notebook_instance_name: Option<String>,

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


/// Information on the IMDS configuration of the notebook instance
#[derive(Default, serde::Serialize)]
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
    pub minimum_instance_metadata_service_version: String,

}
