

/// Specifies a network insights analysis.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnNetworkInsightsAnalysis {


    /// 
    /// The member accounts that contain resources that the path can traverse.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalAccounts")]
    pub additional_accounts: Option<Vec<String>>,


    /// 
    /// The Amazon Resource Names (ARN) of the resources that the path must traverse.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterInArns")]
    pub filter_in_arns: Option<Vec<String>>,


    /// 
    /// The ID of the path.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInsightsPathId")]
    pub network_insights_path_id: String,


    /// 
    /// The tags to apply.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnNetworkInsightsAnalysis {
    fn type_string() -> &'static str {
        "AWS::EC2::NetworkInsightsAnalysis"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes an additional detail for a path analysis. For more information, see Reachability Analyzer additional detail codes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AdditionalDetail {


    /// 
    /// The additional detail code.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalDetailType")]
    pub additional_detail_type: Option<String>,


    /// 
    /// The path component.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,


    /// 
    /// The load balancers.
    /// 
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancers")]
    pub load_balancers: Option<Vec<AnalysisComponent>>,


    /// 
    /// The name of the VPC endpoint service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

}




/// Describes an potential intermediate component of a feasible path.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AlternatePathHint {


    /// 
    /// The Amazon Resource Name (ARN) of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,


    /// 
    /// The ID of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentId")]
    pub component_id: Option<String>,

}




/// Describes a network access control (ACL) rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisAclRule {


    /// 
    /// The IPv4 address range, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,


    /// 
    /// Indicates whether the rule is an outbound rule.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Egress")]
    pub egress: Option<bool>,


    /// 
    /// The range of ports.
    /// 
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,


    /// 
    /// The protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// Indicates whether to allow or deny traffic that matches the rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleAction")]
    pub rule_action: Option<String>,


    /// 
    /// The rule number.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleNumber")]
    pub rule_number: Option<i64>,

}




/// Describes a path component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisComponent {


    /// 
    /// The Amazon Resource Name (ARN) of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Arn")]
    pub arn: Option<String>,


    /// 
    /// The ID of the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: Option<String>,

}




/// Describes a load balancer listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisLoadBalancerListener {


    /// 
    /// [Classic Load Balancers] The back-end port for the listener.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstancePort")]
    pub instance_port: Option<i64>,


    /// 
    /// The port on which the load balancer is listening.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerPort")]
    pub load_balancer_port: Option<i64>,

}




/// Describes a load balancer target.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisLoadBalancerTarget {


    /// 
    /// The IP address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,


    /// 
    /// The Availability Zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// Information about the instance.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Instance")]
    pub instance: Option<AnalysisComponent>,


    /// 
    /// The port on which the target is listening.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,

}




/// Describes a header. Reflects any changes made by a component as traffic passes through.     The fields of an inbound header are null except for the first component of a path.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisPacketHeader {


    /// 
    /// The destination addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationAddresses")]
    pub destination_addresses: Option<Vec<String>>,


    /// 
    /// The destination port ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPortRanges")]
    pub destination_port_ranges: Option<Vec<PortRange>>,


    /// 
    /// The protocol.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// The source addresses.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceAddresses")]
    pub source_addresses: Option<Vec<String>>,


    /// 
    /// The source port ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePortRanges")]
    pub source_port_ranges: Option<Vec<PortRange>>,

}




/// Describes a route table route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisRouteTableRoute {


    /// 
    /// The ID of a NAT gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: Option<String>,


    /// 
    /// The ID of a network interface.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,


    /// 
    /// Describes how the route was created. The following are the possible values:
    /// 
    /// CreateRouteTable - The route was automatically created when the route table was created.               CreateRoute - The route was manually added to the route table.               EnableVgwRoutePropagation - The route was propagated by route propagation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Origin")]
    pub origin: Option<String>,


    /// 
    /// The state. The following are the possible values:
    /// 
    /// active               blackhole
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<String>,


    /// 
    /// The ID of a transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: Option<String>,


    /// 
    /// The ID of a VPC peering connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: Option<String>,


    /// 
    /// The destination IPv4 address, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "destinationCidr")]
    pub destination_cidr: Option<String>,


    /// 
    /// The prefix of the AWS service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "destinationPrefixListId")]
    pub destination_prefix_list_id: Option<String>,


    /// 
    /// The ID of an egress-only internet gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "egressOnlyInternetGatewayId")]
    pub egress_only_internet_gateway_id: Option<String>,


    /// 
    /// The ID of the gateway, such as an internet gateway or virtual private gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "gatewayId")]
    pub gateway_id: Option<String>,


    /// 
    /// The ID of the instance, such as a NAT instance.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,

}




/// Describes a security group rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AnalysisSecurityGroupRule {


    /// 
    /// The IPv4 address range, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,


    /// 
    /// The direction. The following are the possible values:
    /// 
    /// egress               ingress
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: Option<String>,


    /// 
    /// The port range.
    /// 
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,


    /// 
    /// The prefix list ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListId")]
    pub prefix_list_id: Option<String>,


    /// 
    /// The protocol name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// The security group ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: Option<String>,

}




/// Describes an explanation code for an unreachable path. For more information, see Reachability Analyzer explanation codes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Explanation {


    /// 
    /// The network ACL.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Acl")]
    pub acl: Option<AnalysisComponent>,


    /// 
    /// The network ACL rule.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisAclRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclRule")]
    pub acl_rule: Option<AnalysisAclRule>,


    /// 
    /// The IPv4 address, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Address")]
    pub address: Option<String>,


    /// 
    /// The IPv4 addresses, in CIDR notation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Addresses")]
    pub addresses: Option<Vec<String>>,


    /// 
    /// The resource to which the component is attached.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<AnalysisComponent>,


    /// 
    /// The Availability Zones.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,


    /// 
    /// The CIDR ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidrs")]
    pub cidrs: Option<Vec<String>>,


    /// 
    /// The listener for a Classic Load Balancer.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisLoadBalancerListener
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClassicLoadBalancerListener")]
    pub classic_load_balancer_listener: Option<AnalysisLoadBalancerListener>,


    /// 
    /// The component.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,


    /// 
    /// The AWS account for the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \d{12}
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentAccount")]
    pub component_account: Option<String>,


    /// 
    /// The Region for the component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: [a-z]{2}-[a-z]+-[1-9]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ComponentRegion")]
    pub component_region: Option<String>,


    /// 
    /// The customer gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomerGateway")]
    pub customer_gateway: Option<AnalysisComponent>,


    /// 
    /// The destination.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destination")]
    pub destination: Option<AnalysisComponent>,


    /// 
    /// The destination VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationVpc")]
    pub destination_vpc: Option<AnalysisComponent>,


    /// 
    /// The direction. The following are the possible values:
    /// 
    /// egress               ingress
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Direction")]
    pub direction: Option<String>,


    /// 
    /// The load balancer listener.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticLoadBalancerListener")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,


    /// 
    /// The explanation code.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExplanationCode")]
    pub explanation_code: Option<String>,


    /// 
    /// The route table.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "IngressRouteTable")]
    pub ingress_route_table: Option<AnalysisComponent>,


    /// 
    /// The internet gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternetGateway")]
    pub internet_gateway: Option<AnalysisComponent>,


    /// 
    /// The Amazon Resource Name (ARN) of the load balancer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1283
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: Option<String>,


    /// 
    /// The listener port of the load balancer.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerListenerPort")]
    pub load_balancer_listener_port: Option<i64>,


    /// 
    /// The target.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisLoadBalancerTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTarget")]
    pub load_balancer_target: Option<AnalysisLoadBalancerTarget>,


    /// 
    /// The target group.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetGroup")]
    pub load_balancer_target_group: Option<AnalysisComponent>,


    /// 
    /// The target groups.
    /// 
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetGroups")]
    pub load_balancer_target_groups: Option<Vec<AnalysisComponent>>,


    /// 
    /// The target port.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerTargetPort")]
    pub load_balancer_target_port: Option<i64>,


    /// 
    /// The missing component.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MissingComponent")]
    pub missing_component: Option<String>,


    /// 
    /// The NAT gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "NatGateway")]
    pub nat_gateway: Option<AnalysisComponent>,


    /// 
    /// The network interface.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "NetworkInterface")]
    pub network_interface: Option<AnalysisComponent>,


    /// 
    /// The packet field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PacketField")]
    pub packet_field: Option<String>,


    /// 
    /// The port.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The port ranges.
    /// 
    /// Required: No
    ///
    /// Type: List of PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRanges")]
    pub port_ranges: Option<Vec<PortRange>>,


    /// 
    /// The prefix list.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixList")]
    pub prefix_list: Option<AnalysisComponent>,


    /// 
    /// The protocols.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<String>>,


    /// 
    /// The route table.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTable")]
    pub route_table: Option<AnalysisComponent>,


    /// 
    /// The route table route.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableRoute")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,


    /// 
    /// The security group.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroup")]
    pub security_group: Option<AnalysisComponent>,


    /// 
    /// The security group rule.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisSecurityGroupRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupRule")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,


    /// 
    /// The security groups.
    /// 
    /// Required: No
    ///
    /// Type: List of AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<AnalysisComponent>>,


    /// 
    /// The source VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVpc")]
    pub source_vpc: Option<AnalysisComponent>,


    /// 
    /// The state.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<String>,


    /// 
    /// The subnet.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnet")]
    pub subnet: Option<AnalysisComponent>,


    /// 
    /// The route table for the subnet.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetRouteTable")]
    pub subnet_route_table: Option<AnalysisComponent>,


    /// 
    /// The transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGateway")]
    pub transit_gateway: Option<AnalysisComponent>,


    /// 
    /// The transit gateway attachment.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayAttachment")]
    pub transit_gateway_attachment: Option<AnalysisComponent>,


    /// 
    /// The transit gateway route table.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTable")]
    pub transit_gateway_route_table: Option<AnalysisComponent>,


    /// 
    /// The transit gateway route table route.
    /// 
    /// Required: No
    ///
    /// Type: TransitGatewayRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,


    /// 
    /// The component VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vpc")]
    pub vpc: Option<AnalysisComponent>,


    /// 
    /// The VPC peering connection.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcPeeringConnection")]
    pub vpc_peering_connection: Option<AnalysisComponent>,


    /// 
    /// The VPN connection.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnConnection")]
    pub vpn_connection: Option<AnalysisComponent>,


    /// 
    /// The VPN gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnGateway")]
    pub vpn_gateway: Option<AnalysisComponent>,


    /// 
    /// The VPC endpoint.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "vpcEndpoint")]
    pub vpc_endpoint: Option<AnalysisComponent>,

}




/// Describes a path component.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PathComponent {


    /// 
    /// The network ACL rule.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisAclRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "AclRule")]
    pub acl_rule: Option<AnalysisAclRule>,


    /// 
    /// The additional details.
    /// 
    /// Required: No
    ///
    /// Type: List of AdditionalDetail
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: Option<Vec<AdditionalDetail>>,


    /// 
    /// The component.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,


    /// 
    /// The destination VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationVpc")]
    pub destination_vpc: Option<AnalysisComponent>,


    /// 
    /// The load balancer listener.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "ElasticLoadBalancerListener")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,


    /// 
    /// The explanation codes.
    /// 
    /// Required: No
    ///
    /// Type: List of Explanation
    ///
    /// Update requires: No interruption
    #[serde(rename = "Explanations")]
    pub explanations: Option<Vec<Explanation>>,


    /// 
    /// The inbound header.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisPacketHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "InboundHeader")]
    pub inbound_header: Option<AnalysisPacketHeader>,


    /// 
    /// The outbound header.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisPacketHeader
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutboundHeader")]
    pub outbound_header: Option<AnalysisPacketHeader>,


    /// 
    /// The route table route.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteTableRoute")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,


    /// 
    /// The security group rule.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisSecurityGroupRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupRule")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,


    /// 
    /// The sequence number.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<i64>,


    /// 
    /// The name of the VPC endpoint service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,


    /// 
    /// The source VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceVpc")]
    pub source_vpc: Option<AnalysisComponent>,


    /// 
    /// The subnet.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnet")]
    pub subnet: Option<AnalysisComponent>,


    /// 
    /// The transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGateway")]
    pub transit_gateway: Option<AnalysisComponent>,


    /// 
    /// The route in a transit gateway route table.
    /// 
    /// Required: No
    ///
    /// Type: TransitGatewayRouteTableRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,


    /// 
    /// The component VPC.
    /// 
    /// Required: No
    ///
    /// Type: AnalysisComponent
    ///
    /// Update requires: No interruption
    #[serde(rename = "Vpc")]
    pub vpc: Option<AnalysisComponent>,

}




/// Describes a range of ports.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PortRange {


    /// 
    /// The first port in the range.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    pub from: Option<i64>,


    /// 
    /// The last port in the range.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "To")]
    pub to: Option<i64>,

}




/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}




/// Describes a route in a transit gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TransitGatewayRouteTableRoute {


    /// 
    /// The ID of the route attachment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachmentId")]
    pub attachment_id: Option<String>,


    /// 
    /// The CIDR block used for destination matches.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationCidr")]
    pub destination_cidr: Option<String>,


    /// 
    /// The ID of the prefix list.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListId")]
    pub prefix_list_id: Option<String>,


    /// 
    /// The ID of the resource for the route attachment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,


    /// 
    /// The resource type for the route attachment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,


    /// 
    /// The route origin. The following are the possible values:
    /// 
    /// static               propagated
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RouteOrigin")]
    pub route_origin: Option<String>,


    /// 
    /// The state of the route.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: Option<String>,

}


