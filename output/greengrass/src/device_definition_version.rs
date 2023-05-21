/// The     AWS::Greengrass::DeviceDefinitionVersion resource represents a device definition version for AWS IoT Greengrass.      A device definition version contains a list of devices.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDeviceDefinitionVersion {
    ///
    /// The ID of the device definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceDefinitionId")]
    pub device_definition_id: String,

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

impl cfn_resources::CfnResource for CfnDeviceDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::DeviceDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// A device is an AWS IoT device (thing) that's added to a Greengrass group. 	 Greengrass devices can communicate with the Greengrass core in the same group. 	 For more information, see What Is AWS IoT Greengrass? in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Devices 		 property of the AWS::Greengrass::DeviceDefinitionVersion resource contains a      list of Device property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Device {
    ///
    /// The ARN of the device certificate for the device. This X.509 certificate is used to authenticate           the device with AWS IoT and AWS IoT Greengrass services.
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
    /// The Amazon Resource Name (ARN) of the device, which is an AWS IoT device (thing).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ThingArn")]
    pub thing_arn: String,
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
