/// The AWS::IoT1Click::Device resource controls the enabled state of an AWS IoT 1-Click compatible device. For more information,      see Device in the AWS IoT 1-Click Devices API Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDevice {
    ///
    /// The ID of the device, such as G030PX0312744DWM.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DeviceId")]
    pub device_id: cfn_resources::StrVal,

    ///
    /// A Boolean value indicating whether the device is enabled (true) or not (false).
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    #[serde(skip_serializing)]
    pub att_arn: CfnDevicearn,

    #[serde(skip_serializing)]
    pub att_device_id: CfnDevicedeviceid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDevicearn;
impl CfnDevicearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDevicedeviceid;
impl CfnDevicedeviceid {
    pub fn att_name(&self) -> &'static str {
        r#"DeviceId"#
    }
}

impl cfn_resources::CfnResource for CfnDevice {
    fn type_string(&self) -> &'static str {
        "AWS::IoT1Click::Device"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
