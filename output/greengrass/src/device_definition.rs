

/// The     AWS::Greengrass::DeviceDefinition resource represents a device definition for AWS IoT Greengrass. 			Device definitions are used to organize your device definition versions.
///
/// Device definitions can reference multiple device definition versions. All device definition versions      must be associated with a device definition. Each device definition version can contain one or more devices.
#[derive(Default, serde::Serialize)]
pub struct CfnDeviceDefinition {


    /// 
    /// The device definition version to include when the device definition is created.          A device definition version contains a list of          device property types.
    /// 
    /// NoteTo associate a device definition version after the device definition is created, 				   create an AWS::Greengrass::DeviceDefinitionVersion 				   resource and specify the ID of this device definition.
    /// 
    /// Required: No
    ///
    /// Type: DeviceDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    pub initial_version: Option<DeviceDefinitionVersion>,


    /// 
    /// The name of the device definition.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Application-specific metadata to attach to the device definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
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
    pub tags: Option<serde_json::Value>,

}


/// A device is an AWS IoT device (thing) that's added to a Greengrass group. 	 Greengrass devices can communicate with the Greengrass core in the same group. For more information,   see What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Devices 		 property of the DeviceDefinitionVersion property type contains a list       of Device property types.
#[derive(Default, serde::Serialize)]
pub struct Device {


    /// 
    /// Indicates whether the device's local shadow is synced       with the cloud automatically.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncShadow")]
    pub sync_shadow: Option<bool>,


    /// 
    /// The Amazon Resource Name (ARN) of the device certificate for the device. This X.509 certificate is used to authenticate           the device with AWS IoT and AWS IoT Greengrass services.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,


    /// 
    /// A descriptive or arbitrary ID for the device. This value must be unique within       the device definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The ARN of the device, which is an AWS IoT device (thing).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,

}


/// A device definition version contains a list of devices.
///
/// In an AWS CloudFormation template, DeviceDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::DeviceDefinition resource.
#[derive(Default, serde::Serialize)]
pub struct DeviceDefinitionVersion {


    /// 
    /// The devices in this version.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Device
    ///
    /// Update requires: Replacement
    #[serde(rename = "Devices")]
    pub devices: Vec<Device>,

}
