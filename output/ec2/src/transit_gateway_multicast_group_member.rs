

/// Registers members (network interfaces) with the transit gateway multicast group. A     member is a network interface associated with a supported EC2 instance that receives     multicast traffic. For information about supported instances, see Multicast Consideration in Amazon VPC Transit       Gateways.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayMulticastGroupMember {


    /// 
    /// The IP address assigned to the transit gateway multicast group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupIpAddress")]
    pub group_ip_address: String,


    /// 
    /// The group members' network interface IDs to register with the transit gateway multicast group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,


    /// 
    /// The ID of the transit gateway multicast domain.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,

}



impl cfn_resources::CfnResource for CfnTransitGatewayMulticastGroupMember {
    fn type_string() -> &'static str {
        "AWS::EC2::TransitGatewayMulticastGroupMember"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}