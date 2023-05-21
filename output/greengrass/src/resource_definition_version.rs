/// The     AWS::Greengrass::ResourceDefinitionVersion resource represents a resource definition version for AWS IoT Greengrass.     A resource definition version contains a list of resources. (In AWS CloudFormation, resources are named resource instances.)
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnResourceDefinitionVersion {
    ///
    /// The ID of the resource definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceDefinitionId")]
    pub resource_definition_id: String,

    ///
    /// The resources in this version.
    ///
    /// Required: Yes
    ///
    /// Type: List of ResourceInstance
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resources")]
    pub resources: Vec<ResourceInstance>,
}

impl cfn_resources::CfnResource for CfnResourceDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::ResourceDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Settings that define additional 		Linux OS group permissions to give to the Lambda function process. You can give the permissions of the Linux group that 		owns the resource or choose another Linux group. These permissions are in addition to the function's RunAs permissions.
///
/// In an AWS CloudFormation template, GroupOwnerSetting is a property of the 		 LocalDeviceResourceData  		 and LocalVolumeResourceData property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GroupOwnerSetting {
    ///
    /// Indicates whether to give the privileges of the Linux group that owns the resource to the Lambda process. This gives 	 the Lambda process the file access permissions of the Linux group.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AutoAddGroupOwner")]
    pub auto_add_group_owner: bool,

    ///
    /// The name of the Linux group whose privileges you want to add to the Lambda process. This value is ignored if 	 AutoAddGroupOwner is true.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupOwner")]
    pub group_owner: Option<String>,
}

impl cfn_resources::CfnResource for GroupOwnerSetting {
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

/// Settings for a 		local device resource, which represents a file under /dev. 		 For more information,   see Access Local Resources with Lambda Functions in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, LocalDeviceResourceData can be used in the ResourceDataContainer property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LocalDeviceResourceData {
    ///
    /// Settings that define additional 		Linux OS group permissions to give to the Lambda function process.
    ///
    /// Required: No
    ///
    /// Type: GroupOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,

    ///
    /// The local absolute path of the device resource. The source path for a device resource can refer 				 only to a character device or block device under /dev.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePath")]
    pub source_path: String,
}

impl cfn_resources::CfnResource for LocalDeviceResourceData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.group_owner_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for a 		local volume resource, which represents a file or directory on the root file system. 		 For more information,   see Access Local Resources with Lambda Functions in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, LocalVolumeResourceData can be used in the ResourceDataContainer  		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LocalVolumeResourceData {
    ///
    /// The absolute local path of the resource in the Lambda environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,

    ///
    /// Settings that define additional 		Linux OS group permissions to give to the Lambda function process.
    ///
    /// Required: No
    ///
    /// Type: GroupOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupOwnerSetting")]
    pub group_owner_setting: Option<GroupOwnerSetting>,

    ///
    /// The local absolute path of the volume resource on the host. The source path for a volume resource 				 type cannot start with /sys.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePath")]
    pub source_path: String,
}

impl cfn_resources::CfnResource for LocalVolumeResourceData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.group_owner_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A container for resource data, which  		defines the resource type. The container takes only one of the following supported resource data types: 		LocalDeviceResourceData, LocalVolumeResourceData, SageMakerMachineLearningModelResourceData, 		S3MachineLearningModelResourceData, or SecretsManagerSecretResourceData.
///
/// In an AWS CloudFormation template, ResourceDataContainer is a property of the ResourceInstance  		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceDataContainer {
    ///
    /// Settings for a local device resource.
    ///
    /// Required: No
    ///
    /// Type: LocalDeviceResourceData
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalDeviceResourceData")]
    pub local_device_resource_data: Option<LocalDeviceResourceData>,

    ///
    /// Settings for a local volume resource.
    ///
    /// Required: No
    ///
    /// Type: LocalVolumeResourceData
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalVolumeResourceData")]
    pub local_volume_resource_data: Option<LocalVolumeResourceData>,

    ///
    /// Settings for a machine learning resource stored in Amazon S3.
    ///
    /// Required: No
    ///
    /// Type: S3MachineLearningModelResourceData
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3MachineLearningModelResourceData")]
    pub s3_machine_learning_model_resource_data: Option<S3MachineLearningModelResourceData>,

    ///
    /// Settings for a machine learning resource saved as an SageMaker training job.
    ///
    /// Required: No
    ///
    /// Type: SageMakerMachineLearningModelResourceData
    ///
    /// Update requires: Replacement
    #[serde(rename = "SageMakerMachineLearningModelResourceData")]
    pub sage_maker_machine_learning_model_resource_data:
        Option<SageMakerMachineLearningModelResourceData>,

    ///
    /// Settings for a secret resource.
    ///
    /// Required: No
    ///
    /// Type: SecretsManagerSecretResourceData
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecretsManagerSecretResourceData")]
    pub secrets_manager_secret_resource_data: Option<SecretsManagerSecretResourceData>,
}

impl cfn_resources::CfnResource for ResourceDataContainer {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.local_device_resource_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.local_volume_resource_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.s3_machine_learning_model_resource_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.sage_maker_machine_learning_model_resource_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.secrets_manager_secret_resource_data
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The owner setting for a downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, ResourceDownloadOwnerSetting is the property type of the OwnerSetting property for the S3MachineLearningModelResourceData and SageMakerMachineLearningModelResourceData property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceDownloadOwnerSetting {
    ///
    /// The group owner of the machine learning resource. This is the group ID (GID) of an existing Linux OS group on the system.     The group's permissions are added to the Lambda process.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupOwner")]
    pub group_owner: String,

    ///
    /// The permissions that the group owner has to the machine learning resource. Valid values are rw (read-write) or ro (read-only).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupPermission")]
    pub group_permission: String,
}

impl cfn_resources::CfnResource for ResourceDownloadOwnerSetting {
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

/// A local resource, 		machine learning resource, or secret resource. 	For more information,   see Access Local Resources with Lambda Functions, 	Perform Machine Learning Inference, and 	Deploy Secrets to the AWS IoT Greengrass Core in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Resources 		 property of the AWS::Greengrass::ResourceDefinitionVersion resource contains a      list of ResourceInstance property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceInstance {
    ///
    /// A descriptive or arbitrary ID for the resource. This value must be unique within       the resource definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,

    ///
    /// The descriptive resource name, which is displayed on the AWS IoT Greengrass console. Maximum length 128 characters with pattern [a-zA-Z0-9:_-]+. This must be unique within a Greengrass group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// A container for resource data. The container takes only one of the following supported resource data types: 				 LocalDeviceResourceData, LocalVolumeResourceData, 				 SageMakerMachineLearningModelResourceData, S3MachineLearningModelResourceData, or SecretsManagerSecretResourceData.
    ///
    /// NoteOnly one resource type can be defined for a ResourceDataContainer instance.
    ///
    /// Required: Yes
    ///
    /// Type: ResourceDataContainer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceDataContainer")]
    pub resource_data_container: ResourceDataContainer,
}

impl cfn_resources::CfnResource for ResourceInstance {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.resource_data_container.validate()?;

        Ok(())
    }
}

/// Settings for an 		Amazon S3 machine learning resource. 		 For more information,   see Perform Machine Learning Inference in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, S3MachineLearningModelResourceData can be used in the 		 ResourceDataContainer 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3MachineLearningModelResourceData {
    ///
    /// The absolute local path of the resource inside the Lambda environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,

    ///
    /// The owner setting for the downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ResourceDownloadOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,

    ///
    /// The URI of the source model in an Amazon S3 bucket. The model package must be in tar.gz 				 or .zip format.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "S3Uri")]
    pub s3_uri: String,
}

impl cfn_resources::CfnResource for S3MachineLearningModelResourceData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.owner_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for an 		Secrets Manager machine learning resource. 		 For more information,   see Perform Machine Learning Inference in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, SageMakerMachineLearningModelResourceData can be used in the ResourceDataContainer 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SageMakerMachineLearningModelResourceData {
    ///
    /// The absolute local path of the resource inside the Lambda environment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPath")]
    pub destination_path: String,

    ///
    /// The owner setting for the downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ResourceDownloadOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerSetting")]
    pub owner_setting: Option<ResourceDownloadOwnerSetting>,

    ///
    /// The Amazon Resource Name (ARN) of the Amazon SageMaker training job that represents the source model.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SageMakerJobArn")]
    pub sage_maker_job_arn: String,
}

impl cfn_resources::CfnResource for SageMakerMachineLearningModelResourceData {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.owner_setting
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings for a secret resource, 		which references a secret from AWS Secrets Manager. AWS IoT Greengrass stores a local, encrypted copy of the secret on the Greengrass core, 		where it can be securely accessed by connectors and Lambda functions. 		For more information,   see Deploy Secrets to the AWS IoT Greengrass Core in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, SecretsManagerSecretResourceData can be used in the 		 ResourceDataContainer 		 property type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SecretsManagerSecretResourceData {
    ///
    /// The Amazon Resource Name (ARN) of the Secrets Manager secret to make available on the core. The value of the secret's 				 latest version (represented by the AWSCURRENT staging label) is included by default.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ARN")]
    pub arn: String,

    ///
    /// The staging labels whose values you want to make available on the core, in addition to AWSCURRENT.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    pub additional_staging_labels_to_download: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for SecretsManagerSecretResourceData {
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
