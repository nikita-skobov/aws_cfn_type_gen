/// The     AWS::Greengrass::ResourceDefinition resource represents a resource definition for AWS IoT Greengrass.   Resource definitions are used to organize your resource definition versions.
///
/// Resource definitions can reference multiple resource definition versions. All resource definition versions      must be associated with a resource definition. Each resource definition version can contain one or more resources. 		(In AWS CloudFormation, resources are named resource instances.)
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnResourceDefinition {
    ///
    /// The resource definition version to include when the resource definition is created.          A resource definition version contains a list of          resource instance property types.
    ///
    /// NoteTo associate a resource definition version after the resource definition is created, 				   create an AWS::Greengrass::ResourceDefinitionVersion 				   resource and specify the ID of this resource definition.
    ///
    /// Required: No
    ///
    /// Type: ResourceDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ResourceDefinitionVersion>,

    ///
    /// The name of the resource definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Application-specific metadata to attach to the resource definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    ///
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnResourceDefinitionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnResourceDefinitionid,

    #[serde(skip_serializing)]
    pub att_latest_version_arn: CfnResourceDefinitionlatestversionarn,

    #[serde(skip_serializing)]
    pub att_name: CfnResourceDefinitionname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDefinitionarn;
impl CfnResourceDefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDefinitionid;
impl CfnResourceDefinitionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDefinitionlatestversionarn;
impl CfnResourceDefinitionlatestversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"LatestVersionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResourceDefinitionname;
impl CfnResourceDefinitionname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnResourceDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::ResourceDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.initial_version
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Settings that define additional 		Linux OS group permissions to give to the Lambda function process. You can give the permissions of the Linux group that 		owns the resource or choose another Linux group. These permissions are in addition to the function's RunAs permissions.
///
/// In an AWS CloudFormation template, GroupOwnerSetting is a property of the 		 LocalDeviceResourceData  		 and LocalVolumeResourceData property types.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_owner: Option<cfn_resources::StrVal>,
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub source_path: cfn_resources::StrVal,
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub destination_path: cfn_resources::StrVal,

    ///
    /// Settings that define additional 		Linux OS group permissions to give to the Lambda function process.
    ///
    /// Required: No
    ///
    /// Type: GroupOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupOwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub source_path: cfn_resources::StrVal,
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// A resource definition version contains a list of resources. 		(In AWS CloudFormation, resources are named resource instances.)
///
/// In an AWS CloudFormation template, ResourceDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::ResourceDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ResourceDefinitionVersion {
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

impl cfn_resources::CfnResource for ResourceDefinitionVersion {
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

/// The owner setting for a downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, ResourceDownloadOwnerSetting is the property type of the OwnerSetting property for the S3MachineLearningModelResourceData and SageMakerMachineLearningModelResourceData property types.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub group_owner: cfn_resources::StrVal,

    ///
    /// The permissions that the group owner has to the machine learning resource. Valid values are rw (read-write) or ro (read-only).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupPermission")]
    pub group_permission: cfn_resources::StrVal,
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
/// In an AWS CloudFormation template, the Resources 		 property of the AWS::Greengrass::ResourceDefinition resource contains a      list of ResourceInstance property types.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub id: cfn_resources::StrVal,

    ///
    /// The descriptive resource name, which is displayed on the AWS IoT Greengrass console. Maximum length 128 characters with pattern [a-zA-Z0-9:_-]+. This must be unique within a Greengrass group.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub destination_path: cfn_resources::StrVal,

    ///
    /// The owner setting for the downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ResourceDownloadOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub s3_uri: cfn_resources::StrVal,
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub destination_path: cfn_resources::StrVal,

    ///
    /// The owner setting for the downloaded machine learning resource. For more information, see 	  Access Machine Learning Resources from Lambda 	  Functions in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: ResourceDownloadOwnerSetting
    ///
    /// Update requires: Replacement
    #[serde(rename = "OwnerSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub sage_maker_job_arn: cfn_resources::StrVal,
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub arn: cfn_resources::StrVal,

    ///
    /// The staging labels whose values you want to make available on the core, in addition to AWSCURRENT.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AdditionalStagingLabelsToDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
