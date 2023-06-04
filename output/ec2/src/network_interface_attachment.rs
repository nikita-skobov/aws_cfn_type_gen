/// Attaches an elastic network interface (ENI) to an Amazon EC2 instance. You can use this       resource type to attach additional network interfaces to an instance without       interruption.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkInterfaceAttachment {
    ///
    /// Whether to delete the network interface when the instance terminates. By default, this       value is set to true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeleteOnTermination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,

    ///
    /// The network interface's position in the attachment order. For example, the first       attached network interface has a DeviceIndex of 0.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DeviceIndex")]
    pub device_index: cfn_resources::StrVal,

    ///
    /// The ID of the instance to which you will attach the ENI.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstanceId")]
    pub instance_id: cfn_resources::StrVal,

    ///
    /// The ID of the ENI that you want to attach.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnNetworkInterfaceAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkInterfaceAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
