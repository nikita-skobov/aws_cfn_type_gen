

/// Associates an Elastic IP address with an instance or a network interface. Before you can       use an Elastic IP address, you must allocate it to your account. For more information about       working with Elastic IP addresses, see         Elastic IP address concepts and rules.
///
/// You must specify AllocationId and either InstanceId,       NetworkInterfaceId, or PrivateIpAddress.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub allocation_id: Option<String>,


    /// 
    /// Deprecated.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "EIP")]
    pub eip: Option<String>,


    /// 
    /// The ID of the instance. The instance must have exactly one attached network interface.    You can specify either the instance ID or the network interface ID, but not both.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,


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
    pub network_interface_id: Option<String>,


    /// 
    /// The primary or secondary private IP address to associate with the Elastic IP address. If no private IP address is specified, the Elastic IP address is associated with the primary private IP address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,

}



impl cfn_resources::CfnResource for CfnEIPAssociation {
    fn type_string() -> &'static str {
        "AWS::EC2::EIPAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
