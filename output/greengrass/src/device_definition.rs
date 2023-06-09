/// The     AWS::Greengrass::DeviceDefinition resource represents a device definition for AWS IoT Greengrass. 			Device definitions are used to organize your device definition versions.
///
/// Device definitions can reference multiple device definition versions. All device definition versions      must be associated with a device definition. Each device definition version can contain one or more devices.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub name: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnDeviceDefinitionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnDeviceDefinitionid,

    #[serde(skip_serializing)]
    pub att_latest_version_arn: CfnDeviceDefinitionlatestversionarn,

    #[serde(skip_serializing)]
    pub att_name: CfnDeviceDefinitionname,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDeviceDefinitionarn;
impl CfnDeviceDefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDeviceDefinitionid;
impl CfnDeviceDefinitionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDeviceDefinitionlatestversionarn;
impl CfnDeviceDefinitionlatestversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"LatestVersionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDeviceDefinitionname;
impl CfnDeviceDefinitionname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnDeviceDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::DeviceDefinition"
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

/// A device is an AWS IoT device (thing) that's added to a Greengrass group. 	 Greengrass devices can communicate with the Greengrass core in the same group. For more information,   see What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Devices 		 property of the DeviceDefinitionVersion property type contains a list       of Device property types.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Device {
    ///
    /// The Amazon Resource Name (ARN) of the device certificate for the device. This X.509 certificate is used to authenticate           the device with AWS IoT and AWS IoT Greengrass services.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: cfn_resources::StrVal,

    ///
    /// A descriptive or arbitrary ID for the device. This value must be unique within       the device definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// Indicates whether the device's local shadow is synced       with the cloud automatically.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "SyncShadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_shadow: Option<bool>,

    ///
    /// The ARN of the device, which is an AWS IoT device (thing).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingArn")]
    pub thing_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Device {
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

/// A device definition version contains a list of devices.
///
/// In an AWS CloudFormation template, DeviceDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::DeviceDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

impl cfn_resources::CfnResource for DeviceDefinitionVersion {
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
