/// Associates an Elastic IP address with an instance or a network interface. Before you can       use an Elastic IP address, you must allocate it to your account. For more information about       working with Elastic IP addresses, see         Elastic IP address concepts and rules.
///
/// You must specify AllocationId and either InstanceId,       NetworkInterfaceId, or PrivateIpAddress.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnEIPAssociation {
    ///
    /// The allocation ID. This is required.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AllocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<cfn_resources::StrVal>,

    ///
    /// Deprecated.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the instance. The instance must have exactly one attached network interface.    You can specify either the instance ID or the network interface ID, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the network interface. If the instance has more than one network interface, you must specify a network interface ID.
    ///
    /// You can specify either the instance ID or the network interface ID, but not both.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<cfn_resources::StrVal>,

    ///
    /// The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnEIPAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::EIPAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
