

/// An Image Builder image recipe is a document that defines the base image and the     components to be applied to the base image to produce the desired configuration for the     output image. You can use an image recipe to duplicate builds. Image Builder image recipes     can be shared, branched, and edited using the console wizard, the AWS CLI, or the API. You     can use image recipes with your version control software to maintain shareable versioned     image recipes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnImageRecipe {


    /// 
    /// The components of the image recipe. Components are orchestration documents that define a 			sequence of steps for downloading, installing, configuring, and testing software packages. 			They also define validation and security hardening steps. A component is defined using a 			YAML document format.
    /// 
    /// Required: Yes
    ///
    /// Type: List of ComponentConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "Components")]
    pub components: Vec<ComponentConfiguration>,


    /// 
    /// The block device mappings to apply when creating images from this recipe.
    /// 
    /// Required: No
    ///
    /// Type: List of InstanceBlockDeviceMapping
    ///
    /// Update requires: Replacement
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,


    /// 
    /// The working directory to be used during build and test workflows.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,


    /// 
    /// Before you create a new AMI, Image Builder launches temporary Amazon EC2 instances to build and test 			your image configuration. Instance configuration adds a layer of control over those 			instances. You can define settings and add scripts to run when an instance is launched 			from your AMI.
    /// 
    /// Required: No
    ///
    /// Type: AdditionalInstanceConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalInstanceConfiguration")]
    pub additional_instance_configuration: Option<AdditionalInstanceConfiguration>,


    /// 
    /// The name of the image recipe.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[-_A-Za-z-0-9][-_A-Za-z0-9 ]{1,126}[-_A-Za-z-0-9]$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The tags of the image recipe.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The semantic version of the image recipe.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: ^[0-9]+\.[0-9]+\.[0-9]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Version")]
    pub version: String,


    /// 
    /// The parent image of the image recipe. The string must be either an Image ARN or an AMI ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "ParentImage")]
    pub parent_image: String,


    /// 
    /// The description of the image recipe.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



impl cfn_resources::CfnResource for CfnImageRecipe {
    fn type_string() -> &'static str {
        "AWS::ImageBuilder::ImageRecipe"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains a key/value pair that sets the named component parameter.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentParameter {


    /// 
    /// Sets the value for the named component parameter.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: Vec<String>,


    /// 
    /// The name of the component parameter to set.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [^\x00]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}




/// In addition to your infrastructure configuration, these settings provide an extra 			layer of control over your build instances. You can also specify commands to run on 			launch for all of your build instances.
///
/// Image Builder does not automatically install the Systems Manager agent on Windows instances. If your base 			image includes the Systems Manager agent, then the AMI that you create will also include the 			agent. For Linux instances, if the base image does not already include the Systems Manager agent, 			Image Builder installs it. For Linux instances where Image Builder installs the Systems Manager agent, you can 			choose whether to keep it for the AMI that you create.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdditionalInstanceConfiguration {


    /// 
    /// Use this property to provide commands or a command script to run when you launch your 			build instance.
    /// 
    /// The userDataOverride property replaces any commands that Image Builder might have added to 			ensure that Systems Manager is installed on your Linux build instance. If you override the user 			data, make sure that you add commands to install Systems Manager, if it is not pre-installed on 			your base image.
    /// 
    /// NoteThe user data is always base 64 encoded. For example, the following commands are 				encoded as 				IyEvYmluL2Jhc2gKbWtkaXIgLXAgL3Zhci9iYi8KdG91Y2ggL3Zhci$:        #!/bin/bash       mkdir -p /var/bb/touch /var
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 21847
    ///
    /// Pattern: ^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$
    ///
    /// Update requires: No interruption
    #[serde(rename = "UserDataOverride")]
    pub user_data_override: Option<String>,


    /// 
    /// Contains settings for the Systems Manager agent on your build instance.
    /// 
    /// Required: No
    ///
    /// Type: SystemsManagerAgent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SystemsManagerAgent")]
    pub systems_manager_agent: Option<SystemsManagerAgent>,

}




/// Configuration details of the component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComponentConfiguration {


    /// 
    /// The Amazon Resource Name (ARN) of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ^arn:aws[^:]*:imagebuilder:[^:]+:(?:[0-9]{12}|aws):component/[a-z0-9-_]+/(?:(?:([0-9]+|x)\.([0-9]+|x)\.([0-9]+|x))|(?:[0-9]+\.[0-9]+\.[0-9]+/[0-9]+))$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,


    /// 
    /// A group of parameter settings that Image Builder uses to configure the component for a specific 			recipe.
    /// 
    /// Required: No
    ///
    /// Type: List of ComponentParameter
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parameters")]
    pub parameters: Option<Vec<ComponentParameter>>,

}




/// The image recipe EBS instance block device specification includes the Amazon     EBS-specific block device mapping specifications for the image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EbsInstanceBlockDeviceSpecification {


    /// 
    /// Configures delete on termination of the associated device.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,


    /// 
    /// Use to configure device IOPS.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 100
    ///
    /// Maximum: 64000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iops")]
    pub iops: Option<i64>,


    /// 
    /// For GP3 volumes only – The throughput in MiB/s 			that the volume supports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 125
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Throughput")]
    pub throughput: Option<i64>,


    /// 
    /// Overrides the volume size of the device.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 16000
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<i64>,


    /// 
    /// The snapshot that defines the device contents.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,


    /// 
    /// Use to configure device encryption.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,


    /// 
    /// Overrides the volume type of the device.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: gp2 | gp3 | io1 | io2 | sc1 | st1 | standard
    ///
    /// Update requires: Replacement
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<EbsInstanceBlockDeviceSpecificationVolumeTypeEnum>,


    /// 
    /// Use to configure the KMS key to use when encrypting the device.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum EbsInstanceBlockDeviceSpecificationVolumeTypeEnum {

    /// gp2
    #[serde(rename = "gp2")]
    Gp2,

    /// gp3
    #[serde(rename = "gp3")]
    Gp3,

    /// io1
    #[serde(rename = "io1")]
    Io1,

    /// io2
    #[serde(rename = "io2")]
    Io2,

    /// sc1
    #[serde(rename = "sc1")]
    Sc1,

    /// st1
    #[serde(rename = "st1")]
    St1,

    /// standard
    #[serde(rename = "standard")]
    Standard,

}

impl Default for EbsInstanceBlockDeviceSpecificationVolumeTypeEnum {
    fn default() -> Self {
        EbsInstanceBlockDeviceSpecificationVolumeTypeEnum::Gp2
    }
}



/// Defines block device mappings for the instance used to configure your image.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InstanceBlockDeviceMapping {


    /// 
    /// Manages the instance ephemeral devices.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,


    /// 
    /// Enter an empty string to remove a mapping from the parent image.
    /// 
    /// The following is an example of an empty string value in the NoDevice field.
    /// 
    /// NoDevice:""
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,


    /// 
    /// The device to which these mappings apply.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,


    /// 
    /// Use to manage Amazon EBS-specific configuration for this mapping.
    /// 
    /// Required: No
    ///
    /// Type: EbsInstanceBlockDeviceSpecification
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsInstanceBlockDeviceSpecification>,

}




/// Contains settings for the Systems Manager agent on your build instance.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SystemsManagerAgent {


    /// 
    /// Controls whether the Systems Manager agent is removed from your final build image, prior to 			creating the new AMI. If this is set to true, then the agent is removed from the final 			image. If it's set to false, then the agent is left in, so that it is included in the 			new AMI. The default value is false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UninstallAfterBuild")]
    pub uninstall_after_build: Option<bool>,

}


