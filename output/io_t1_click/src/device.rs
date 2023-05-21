

/// The AWS::IoT1Click::Device resource controls the enabled state of an AWS IoT 1-Click compatible device. For more information,      see Device in the AWS IoT 1-Click Devices API Reference.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub device_id: String,


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

}



impl cfn_resources::CfnResource for CfnDevice {
    fn type_string() -> &'static str {
        "AWS::IoT1Click::Device"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
