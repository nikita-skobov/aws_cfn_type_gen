

/// Specifies a network route to add to a Client VPN endpoint. Each Client VPN endpoint has     a route table that describes the available destination network routes. Each route in the     route table specifies the path for traffic to specific resources or networks.
///
/// A target network association must be created before you can specify a route. If you're     setting up all the components of a Client VPN endpoint at the same time, you must use the       DependsOn       Attribute to declare a dependency on the       AWS::EC2::ClientVpnTargetNetworkAssociation resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnClientVpnRoute {


    /// 
    /// The ID of the Client VPN endpoint to which to add the route.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,


    /// 
    /// A brief description of the route.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The IPv4 address range, in CIDR notation, of the route destination. For example:
    /// 
    /// To add a route for Internet access, enter 0.0.0.0/0                       To add a route for a peered VPC, enter the peered VPC's IPv4 CIDR range               To add a route for an on-premises network, enter the AWS Site-to-Site VPN connection's IPv4 CIDR range               To add a route for the local network, enter the client CIDR range
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,


    /// 
    /// The ID of the subnet through which you want to route traffic. The specified subnet must be 			an existing target network of the Client VPN endpoint.
    /// 
    /// Alternatively, if you're adding a route for the local network, specify local.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetVpcSubnetId")]
    pub target_vpc_subnet_id: String,

}



impl cfn_resources::CfnResource for CfnClientVpnRoute {
    fn type_string() -> &'static str {
        "AWS::EC2::ClientVpnRoute"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}