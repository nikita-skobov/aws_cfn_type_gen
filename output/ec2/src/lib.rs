
pub mod cfn_capacity_reservation {

#[derive(serde::Serialize, Default)]
pub struct CfnCapacityReservation {
    /// No documentation provided by AWS
    #[serde(rename = "InstancePlatform")]
    pub instance_platform: String,
    /// No documentation provided by AWS
    #[serde(rename = "OutPostArn")]
    pub out_post_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EphemeralStorage")]
    pub ephemeral_storage: Option<bool>,
    /// List of TagSpecification
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "EndDateType")]
    pub end_date_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceMatchCriteria")]
    pub instance_match_criteria: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// No documentation provided by AWS
    #[serde(rename = "PlacementGroupArn")]
    pub placement_group_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// No documentation provided by AWS
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct TagSpecification {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_capacity_reservation_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnCapacityReservationFleet {
    /// No documentation provided by AWS
    #[serde(rename = "NoRemoveEndDate")]
    pub no_remove_end_date: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RemoveEndDate")]
    pub remove_end_date: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceMatchCriteria")]
    pub instance_match_criteria: Option<String>,
    /// List of TagSpecification
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,
    /// List of InstanceTypeSpecification
    #[serde(rename = "InstanceTypeSpecifications")]
    pub instance_type_specifications: Option<Vec<InstanceTypeSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TotalTargetCapacity")]
    pub total_target_capacity: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct InstanceTypeSpecification {
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "Priority")]
    pub priority: Option<usize>,
    #[serde(rename = "Weight")]
    pub weight: Option<f64>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "InstancePlatform")]
    pub instance_platform: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct TagSpecification {
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


}

pub mod cfn_carrier_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnCarrierGateway {
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The tags for the carrier gateway.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_client_vpn_authorization_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnClientVpnAuthorizationRule {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessGroupId")]
    pub access_group_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TargetNetworkCidr")]
    pub target_network_cidr: String,
    /// No documentation provided by AWS
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizeAllGroups")]
    pub authorize_all_groups: Option<bool>,

}



}

pub mod cfn_client_vpn_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnClientVpnEndpoint {
    /// No documentation provided by AWS
    #[serde(rename = "VpnPort")]
    pub vpn_port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientConnectOptions")]
    pub client_connect_options: Option<ClientConnectOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// List of ClientAuthenticationRequest
    #[serde(rename = "AuthenticationOptions")]
    pub authentication_options: Vec<ClientAuthenticationRequest>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientCidrBlock")]
    pub client_cidr_block: String,
    /// No documentation provided by AWS
    #[serde(rename = "DnsServers")]
    pub dns_servers: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SplitTunnel")]
    pub split_tunnel: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TransportProtocol")]
    pub transport_protocol: Option<String>,
    /// List of TagSpecification
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServerCertificateArn")]
    pub server_certificate_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionLogOptions")]
    pub connection_log_options: ConnectionLogOptions,
    /// No documentation provided by AWS
    #[serde(rename = "ClientLoginBannerOptions")]
    pub client_login_banner_options: Option<ClientLoginBannerOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "SessionTimeoutHours")]
    pub session_timeout_hours: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "SelfServicePortal")]
    pub self_service_portal: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct TagSpecification {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct FederatedAuthenticationRequest {
    #[serde(rename = "SelfServiceSAMLProviderArn")]
    pub self_service_samlprovider_arn: Option<String>,
    #[serde(rename = "SAMLProviderArn")]
    pub samlprovider_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct DirectoryServiceAuthenticationRequest {
    #[serde(rename = "DirectoryId")]
    pub directory_id: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionLogOptions {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "CloudwatchLogStream")]
    pub cloudwatch_log_stream: Option<String>,
    #[serde(rename = "CloudwatchLogGroup")]
    pub cloudwatch_log_group: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CertificateAuthenticationRequest {
    #[serde(rename = "ClientRootCertificateChainArn")]
    pub client_root_certificate_chain_arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ClientConnectOptions {
    #[serde(rename = "LambdaFunctionArn")]
    pub lambda_function_arn: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ClientLoginBannerOptions {
    #[serde(rename = "BannerText")]
    pub banner_text: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,

}

#[derive(serde::Serialize, Default)]
pub struct ClientAuthenticationRequest {
    #[serde(rename = "ActiveDirectory")]
    pub active_directory: Option<DirectoryServiceAuthenticationRequest>,
    #[serde(rename = "FederatedAuthentication")]
    pub federated_authentication: Option<FederatedAuthenticationRequest>,
    #[serde(rename = "MutualAuthentication")]
    pub mutual_authentication: Option<CertificateAuthenticationRequest>,
    #[serde(rename = "Type")]
    pub ty: String,

}


}

pub mod cfn_client_vpn_route {

#[derive(serde::Serialize, Default)]
pub struct CfnClientVpnRoute {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,
    /// No documentation provided by AWS
    #[serde(rename = "TargetVpcSubnetId")]
    pub target_vpc_subnet_id: String,

}



}

pub mod cfn_client_vpn_target_network_association {

#[derive(serde::Serialize, Default)]
pub struct CfnClientVpnTargetNetworkAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ClientVpnEndpointId")]
    pub client_vpn_endpoint_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,

}



}

pub mod cfn_customer_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomerGateway {
    /// A name for the customer gateway device.
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,
    /// One or more tags for the customer gateway.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// For devices that support BGP, the customer gateway's BGP ASN.
    #[serde(rename = "BgpAsn")]
    pub bgp_asn: usize,
    /// The internet-routable IP address for the customer gateway's outside interface. The address must be static.
    #[serde(rename = "IpAddress")]
    pub ip_address: String,
    /// The type of VPN connection that this customer gateway supports.
    #[serde(rename = "Type")]
    pub ty: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_dhcpoptions {

#[derive(serde::Serialize, Default)]
pub struct CfnDHCPOptions {
    /// This value is used to complete unqualified DNS hostnames.
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    /// Any tags assigned to the DHCP options set.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The IPv4 addresses of up to four domain name servers, or AmazonProvidedDNS.
    #[serde(rename = "DomainNameServers")]
    pub domain_name_servers: Option<Vec<String>>,
    /// The IPv4 addresses of up to four NetBIOS name servers.
    #[serde(rename = "NetbiosNameServers")]
    pub netbios_name_servers: Option<Vec<String>>,
    /// The NetBIOS node type (1, 2, 4, or 8).
    #[serde(rename = "NetbiosNodeType")]
    pub netbios_node_type: Option<usize>,
    /// The IPv4 addresses of up to four Network Time Protocol (NTP) servers.
    #[serde(rename = "NtpServers")]
    pub ntp_servers: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_ec2_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnEC2Fleet {
    /// No documentation provided by AWS
    #[serde(rename = "TargetCapacitySpecification")]
    pub target_capacity_specification: TargetCapacitySpecificationRequest,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ValidFrom")]
    pub valid_from: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Context")]
    pub context: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ReplaceUnhealthyInstances")]
    pub replace_unhealthy_instances: Option<bool>,
    /// List of TagSpecification
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "TerminateInstancesWithExpiration")]
    pub terminate_instances_with_expiration: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ExcessCapacityTerminationPolicy")]
    pub excess_capacity_termination_policy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ValidUntil")]
    pub valid_until: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "OnDemandOptions")]
    pub on_demand_options: Option<OnDemandOptionsRequest>,
    /// No documentation provided by AWS
    #[serde(rename = "SpotOptions")]
    pub spot_options: Option<SpotOptionsRequest>,
    /// List of FleetLaunchTemplateConfigRequest
    #[serde(rename = "LaunchTemplateConfigs")]
    pub launch_template_configs: Vec<FleetLaunchTemplateConfigRequest>,

}


#[derive(serde::Serialize, Default)]
pub struct VCpuCountRangeRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotOptionsRequest {
    #[serde(rename = "SingleInstanceType")]
    pub single_instance_type: Option<bool>,
    #[serde(rename = "InstanceInterruptionBehavior")]
    pub instance_interruption_behavior: Option<String>,
    #[serde(rename = "MaxTotalPrice")]
    pub max_total_price: Option<String>,
    #[serde(rename = "InstancePoolsToUseCount")]
    pub instance_pools_to_use_count: Option<usize>,
    #[serde(rename = "MaintenanceStrategies")]
    pub maintenance_strategies: Option<MaintenanceStrategies>,
    #[serde(rename = "SingleAvailabilityZone")]
    pub single_availability_zone: Option<bool>,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "MinTargetCapacity")]
    pub min_target_capacity: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryGiBPerVCpuRequest {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorTotalMemoryMiBRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceStrategies {
    #[serde(rename = "CapacityRebalance")]
    pub capacity_rebalance: Option<CapacityRebalance>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityRebalance {
    #[serde(rename = "TerminationDelay")]
    pub termination_delay: Option<usize>,
    #[serde(rename = "ReplacementStrategy")]
    pub replacement_strategy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceRequirementsRequest {
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<String>,
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCountRequest>,
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<String>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCountRangeRequest>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpuRequest>,
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiBRequest>,
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGBRequest>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryMiBRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TagSpecification {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FleetLaunchTemplateOverridesRequest {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<f64>,
    #[serde(rename = "MaxPrice")]
    pub max_price: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "Priority")]
    pub priority: Option<f64>,
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "Placement")]
    pub placement: Option<Placement>,

}

#[derive(serde::Serialize, Default)]
pub struct OnDemandOptionsRequest {
    #[serde(rename = "MinTargetCapacity")]
    pub min_target_capacity: Option<usize>,
    #[serde(rename = "SingleAvailabilityZone")]
    pub single_availability_zone: Option<bool>,
    #[serde(rename = "SingleInstanceType")]
    pub single_instance_type: Option<bool>,
    #[serde(rename = "MaxTotalPrice")]
    pub max_total_price: Option<String>,
    #[serde(rename = "CapacityReservationOptions")]
    pub capacity_reservation_options: Option<CapacityReservationOptionsRequest>,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetCapacitySpecificationRequest {
    #[serde(rename = "DefaultTargetCapacityType")]
    pub default_target_capacity_type: Option<String>,
    #[serde(rename = "SpotTargetCapacity")]
    pub spot_target_capacity: Option<usize>,
    #[serde(rename = "TargetCapacityUnitType")]
    pub target_capacity_unit_type: Option<String>,
    #[serde(rename = "OnDemandTargetCapacity")]
    pub on_demand_target_capacity: Option<usize>,
    #[serde(rename = "TotalTargetCapacity")]
    pub total_target_capacity: usize,

}

#[derive(serde::Serialize, Default)]
pub struct FleetLaunchTemplateSpecificationRequest {
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaceCountRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BaselineEbsBandwidthMbpsRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Placement {
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<usize>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
    #[serde(rename = "SpreadDomain")]
    pub spread_domain: Option<String>,
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct FleetLaunchTemplateConfigRequest {
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: Option<FleetLaunchTemplateSpecificationRequest>,
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<FleetLaunchTemplateOverridesRequest>>,

}

#[derive(serde::Serialize, Default)]
pub struct TotalLocalStorageGBRequest {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityReservationOptionsRequest {
    #[serde(rename = "UsageStrategy")]
    pub usage_strategy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkBandwidthGbpsRequest {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorCountRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}


}

pub mod cfn_egress_only_internet_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnEgressOnlyInternetGateway {
    /// The ID of the VPC for which to create the egress-only internet gateway.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}



}

pub mod cfn_eip {

#[derive(serde::Serialize, Default)]
pub struct CfnEIP {
    /// Any tags assigned to the EIP.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Indicates whether the Elastic IP address is for use with instances in a VPC or instance in EC2-Classic.
    #[serde(rename = "Domain")]
    pub domain: Option<String>,
    /// A unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.
    #[serde(rename = "NetworkBorderGroup")]
    pub network_border_group: Option<String>,
    /// The ID of the instance.
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,
    /// The PublicIP of the EIP generated by resource through transfer from another account
    #[serde(rename = "TransferAddress")]
    pub transfer_address: Option<String>,
    /// The ID of an address pool that you own. Use this parameter to let Amazon EC2 select an address from the address pool.
    #[serde(rename = "PublicIpv4Pool")]
    pub public_ipv4_pool: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_eipassociation {

#[derive(serde::Serialize, Default)]
pub struct CfnEIPAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "EIP")]
    pub eip: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AllocationId")]
    pub allocation_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,

}



}

pub mod cfn_enclave_certificate_iam_role_association {

#[derive(serde::Serialize, Default)]
pub struct CfnEnclaveCertificateIamRoleAssociation {
    /// The Amazon Resource Name (ARN) of the ACM certificate with which to associate the IAM role.
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    /// The Amazon Resource Name (ARN) of the IAM role to associate with the ACM certificate. You can associate up to 16 IAM roles with an ACM certificate.
    #[serde(rename = "RoleArn")]
    pub role_arn: String,

}



}

pub mod cfn_flow_log {

#[derive(serde::Serialize, Default)]
pub struct CfnFlowLog {
    /// Specifies the destination to which the flow log data is to be published. Flow log data can be published to a CloudWatch Logs log group, an Amazon S3 bucket, or a Kinesis Firehose stream. The value specified for this parameter depends on the value specified for LogDestinationType.
    #[serde(rename = "LogDestination")]
    pub log_destination: Option<String>,
    /// The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. You can specify 60 seconds (1 minute) or 600 seconds (10 minutes).
    #[serde(rename = "MaxAggregationInterval")]
    pub max_aggregation_interval: Option<usize>,
    /// The ARN for the IAM role that permits Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account. If you specify LogDestinationType as s3 or kinesis-data-firehose, do not specify DeliverLogsPermissionArn or LogGroupName.
    #[serde(rename = "DeliverLogsPermissionArn")]
    pub deliver_logs_permission_arn: Option<String>,
    /// The ID of the subnet, network interface, or VPC for which you want to create a flow log.
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// The type of traffic to log. You can log traffic that the resource accepts or rejects, or all traffic.
    #[serde(rename = "TrafficType")]
    pub traffic_type: Option<String>,
    /// The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs. If you specify LogDestinationType as s3 or kinesis-data-firehose, do not specify DeliverLogsPermissionArn or LogGroupName.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: Option<String>,
    /// The fields to include in the flow log record, in the order in which they should appear.
    #[serde(rename = "LogFormat")]
    pub log_format: Option<String>,
    /// The type of resource for which to create the flow log. For example, if you specified a VPC ID for the ResourceId property, specify VPC for this property.
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// The tags to apply to the flow logs.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationOptions")]
    pub destination_options: Option<()>,
    /// Specifies the type of destination to which the flow log data is to be published. Flow log data can be published to CloudWatch Logs or Amazon S3.
    #[serde(rename = "LogDestinationType")]
    pub log_destination_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_gateway_route_table_association {

#[derive(serde::Serialize, Default)]
pub struct CfnGatewayRouteTableAssociation {
    /// The ID of the route table.
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,
    /// The ID of the gateway.
    #[serde(rename = "GatewayId")]
    pub gateway_id: String,

}



}

pub mod cfn_host {

#[derive(serde::Serialize, Default)]
pub struct CfnHost {
    /// Automatically allocates a new dedicated host and moves your instances on to it if a degradation is detected on your current host.
    #[serde(rename = "HostMaintenance")]
    pub host_maintenance: Option<String>,
    /// Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.
    #[serde(rename = "InstanceFamily")]
    pub instance_family: Option<String>,
    /// The Availability Zone in which to allocate the Dedicated Host.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID.
    #[serde(rename = "AutoPlacement")]
    pub auto_placement: Option<String>,
    /// The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host.
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,
    /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    /// Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default.
    #[serde(rename = "HostRecovery")]
    pub host_recovery: Option<String>,

}



}

pub mod cfn_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnInstance {
    /// List of Volume
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<Volume>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    /// List of NetworkInterface
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// List of InstanceIpv6Address
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PropagateTagsToVolumeOnCreation")]
    pub propagate_tags_to_volume_on_creation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CpuOptions")]
    pub cpu_options: Option<CpuOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PrivateDnsNameOptions")]
    pub private_dns_name_options: Option<PrivateDnsNameOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DisableApiTermination")]
    pub disable_api_termination: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Monitoring")]
    pub monitoring: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "EnclaveOptions")]
    pub enclave_options: Option<EnclaveOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "CreditSpecification")]
    pub credit_specification: Option<CreditSpecification>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<usize>,
    /// List of BlockDeviceMapping
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    /// List of ElasticGpuSpecification
    #[serde(rename = "ElasticGpuSpecifications")]
    pub elastic_gpu_specifications: Option<Vec<ElasticGpuSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PlacementGroupName")]
    pub placement_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceDestCheck")]
    pub source_dest_check: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<String>,
    /// List of SsmAssociation
    #[serde(rename = "SsmAssociations")]
    pub ssm_associations: Option<Vec<SsmAssociation>>,
    /// No documentation provided by AWS
    #[serde(rename = "RamdiskId")]
    pub ramdisk_id: Option<String>,
    /// List of LicenseSpecification
    #[serde(rename = "LicenseSpecifications")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "HibernationOptions")]
    pub hibernation_options: Option<HibernationOptions>,
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,
    /// List of ElasticInferenceAccelerator
    #[serde(rename = "ElasticInferenceAccelerators")]
    pub elastic_inference_accelerators: Option<Vec<ElasticInferenceAccelerator>>,
    /// No documentation provided by AWS
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    pub instance_initiated_shutdown_behavior: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<usize>,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "DeviceIndex")]
    pub device_index: String,
    #[serde(rename = "GroupSet")]
    pub group_set: Option<Vec<String>>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "AssociateCarrierIpAddress")]
    pub associate_carrier_ip_address: Option<bool>,
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<usize>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification>>,

}

#[derive(serde::Serialize, Default)]
pub struct Ebs {
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "Version")]
    pub version: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceIpv6Address {
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: String,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticGpuSpecification {
    #[serde(rename = "Type")]
    pub ty: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct CpuOptions {
    #[serde(rename = "ThreadsPerCore")]
    pub threads_per_core: Option<usize>,
    #[serde(rename = "CoreCount")]
    pub core_count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct AssociationParameter {
    #[serde(rename = "Value")]
    pub value: Vec<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct Volume {
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
    #[serde(rename = "Device")]
    pub device: String,

}

#[derive(serde::Serialize, Default)]
pub struct PrivateDnsNameOptions {
    #[serde(rename = "HostnameType")]
    pub hostname_type: Option<String>,
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,
    #[serde(rename = "EnableResourceNameDnsARecord")]
    pub enable_resource_name_dns_arecord: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct HibernationOptions {
    #[serde(rename = "Configured")]
    pub configured: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct PrivateIpAddressSpecification {
    #[serde(rename = "Primary")]
    pub primary: bool,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: String,

}

#[derive(serde::Serialize, Default)]
pub struct BlockDeviceMapping {
    #[serde(rename = "Ebs")]
    pub ebs: Option<Ebs>,
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<NoDevice>,

}

#[derive(serde::Serialize, Default)]
pub struct SsmAssociation {
    #[serde(rename = "AssociationParameters")]
    pub association_parameters: Option<Vec<AssociationParameter>>,
    #[serde(rename = "DocumentName")]
    pub document_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticInferenceAccelerator {
    #[serde(rename = "Type")]
    pub ty: String,
    #[serde(rename = "Count")]
    pub count: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct NoDevice {

}

#[derive(serde::Serialize, Default)]
pub struct EnclaveOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct CreditSpecification {
    #[serde(rename = "CPUCredits")]
    pub cpucredits: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LicenseSpecification {
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,

}


}

pub mod cfn_internet_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnInternetGateway {
    /// Any tags to assign to the internet gateway.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_ipam {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAM {
    /// The Id of the default resource discovery, created with this IPAM.
    #[serde(rename = "DefaultResourceDiscoveryId")]
    pub default_resource_discovery_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The regions IPAM is enabled for. Allows pools to be created in these regions, as well as enabling monitoring
    #[serde(rename = "OperatingRegions")]
    pub operating_regions: Option<Vec<IpamOperatingRegion>>,
    /// The count of resource discoveries associated with this IPAM.
    #[serde(rename = "ResourceDiscoveryAssociationCount")]
    pub resource_discovery_association_count: Option<usize>,
    /// The Id of the default association to the default resource discovery, created with this IPAM.
    #[serde(rename = "DefaultResourceDiscoveryAssociationId")]
    pub default_resource_discovery_association_id: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct IpamOperatingRegion {
    #[serde(rename = "RegionName")]
    pub region_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_ipamallocation {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMAllocation {
    /// Represents a single IPv4 or IPv6 CIDR
    #[serde(rename = "Cidr")]
    pub cidr: Option<Cidr>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The desired netmask length of the allocation. If set, IPAM will choose a block of free space with this size and return the CIDR representing it.
    #[serde(rename = "NetmaskLength")]
    pub netmask_length: Option<usize>,
    /// Id of the IPAM Pool.
    #[serde(rename = "IpamPoolId")]
    pub ipam_pool_id: String,

}

pub type Cidr = String;

}

pub mod cfn_ipampool {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMPool {
    /// Limits which service in Amazon Web Services that the pool can be used in.
    #[serde(rename = "AwsService")]
    pub aws_service: Option<String>,
    /// Determines whether or not address space from this pool is publicly advertised. Must be set if and only if the pool is IPv6.
    #[serde(rename = "PubliclyAdvertisable")]
    pub publicly_advertisable: Option<bool>,
    /// The maximum allowed netmask length for allocations made from this pool.
    #[serde(rename = "AllocationMaxNetmaskLength")]
    pub allocation_max_netmask_length: Option<usize>,
    /// The Id of the scope this pool is a part of.
    #[serde(rename = "IpamScopeId")]
    pub ipam_scope_id: String,
    /// The default netmask length for allocations made from this pool. This value is used when the netmask length of an allocation isn't specified.
    #[serde(rename = "AllocationDefaultNetmaskLength")]
    pub allocation_default_netmask_length: Option<usize>,
    /// When specified, an allocation will not be allowed unless a resource has a matching set of tags.
    #[serde(rename = "AllocationResourceTags")]
    pub allocation_resource_tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Determines what to do if IPAM discovers resources that haven't been assigned an allocation. If set to true, an allocation will be made automatically.
    #[serde(rename = "AutoImport")]
    pub auto_import: Option<bool>,
    /// The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Default is `byoip`.
    #[serde(rename = "PublicIpSource")]
    pub public_ip_source: Option<String>,
    /// The Id of this pool's source. If set, all space provisioned in this pool must be free space provisioned in the parent pool.
    #[serde(rename = "SourceIpamPoolId")]
    pub source_ipam_pool_id: Option<String>,
    /// The minimum allowed netmask length for allocations made from this pool.
    #[serde(rename = "AllocationMinNetmaskLength")]
    pub allocation_min_netmask_length: Option<usize>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The address family of the address space in this pool. Either IPv4 or IPv6.
    #[serde(rename = "AddressFamily")]
    pub address_family: String,
    /// The region of this pool. If not set, this will default to "None" which will disable non-custom allocations. If the locale has been specified for the source pool, this value must match.
    #[serde(rename = "Locale")]
    pub locale: Option<String>,
    /// A list of cidrs representing the address space available for allocation in this pool.
    #[serde(rename = "ProvisionedCidrs")]
    pub provisioned_cidrs: Option<Vec<ProvisionedCidr>>,

}

pub type Cidr = String;
#[derive(serde::Serialize, Default)]
pub struct ProvisionedCidr {
    #[serde(rename = "Cidr")]
    pub cidr: Cidr,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_ipampool_cidr {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMPoolCidr {
    /// Id of the IPAM Pool.
    #[serde(rename = "IpamPoolId")]
    pub ipam_pool_id: String,
    /// The desired netmask length of the provision. If set, IPAM will choose a block of free space with this size and return the CIDR representing it.
    #[serde(rename = "NetmaskLength")]
    pub netmask_length: Option<usize>,
    /// Represents a single IPv4 or IPv6 CIDR
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,

}



}

pub mod cfn_ipamresource_discovery {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMResourceDiscovery {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The regions Resource Discovery is enabled for. Allows resource discoveries to be created in these regions, as well as enabling monitoring
    #[serde(rename = "OperatingRegions")]
    pub operating_regions: Option<Vec<IpamOperatingRegion>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct IpamOperatingRegion {
    #[serde(rename = "RegionName")]
    pub region_name: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_ipamresource_discovery_association {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMResourceDiscoveryAssociation {
    /// The Id of the IPAM this Resource Discovery is associated to.
    #[serde(rename = "IpamId")]
    pub ipam_id: String,
    /// The Amazon Resource Name (ARN) of the IPAM Resource Discovery Association.
    #[serde(rename = "IpamResourceDiscoveryId")]
    pub ipam_resource_discovery_id: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_ipamscope {

#[derive(serde::Serialize, Default)]
pub struct CfnIPAMScope {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Id of the IPAM this scope is a part of.
    #[serde(rename = "IpamId")]
    pub ipam_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_key_pair {

#[derive(serde::Serialize, Default)]
pub struct CfnKeyPair {
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the SSH key pair
    #[serde(rename = "KeyName")]
    pub key_name: String,
    /// Plain text public key to import
    #[serde(rename = "PublicKeyMaterial")]
    pub public_key_material: Option<String>,
    /// The title of the TPS report is a mandatory element.
    #[serde(rename = "KeyType")]
    pub key_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_launch_template {

#[derive(serde::Serialize, Default)]
pub struct CfnLaunchTemplate {
    /// No documentation provided by AWS
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LaunchTemplateData")]
    pub launch_template_data: LaunchTemplateData,
    /// List of LaunchTemplateTagSpecification
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<LaunchTemplateTagSpecification>>,
    /// No documentation provided by AWS
    #[serde(rename = "VersionDescription")]
    pub version_description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PrivateIpAdd {
    #[serde(rename = "Primary")]
    pub primary: Option<bool>,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Ipv6PrefixSpecification {
    #[serde(rename = "Ipv6Prefix")]
    pub ipv6_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct EnclaveOptions {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ElasticGpuSpecification {
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateTagSpecification {
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

#[derive(serde::Serialize, Default)]
pub struct TagSpecification {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotOptions {
    #[serde(rename = "InstanceInterruptionBehavior")]
    pub instance_interruption_behavior: Option<String>,
    #[serde(rename = "SpotInstanceType")]
    pub spot_instance_type: Option<String>,
    #[serde(rename = "BlockDurationMinutes")]
    pub block_duration_minutes: Option<usize>,
    #[serde(rename = "ValidUntil")]
    pub valid_until: Option<String>,
    #[serde(rename = "MaxPrice")]
    pub max_price: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CreditSpecification {
    #[serde(rename = "CpuCredits")]
    pub cpu_credits: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VCpuCount {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateElasticInferenceAccelerator {
    #[serde(rename = "Count")]
    pub count: Option<usize>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TotalLocalStorageGB {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateData {
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "ElasticGpuSpecifications")]
    pub elastic_gpu_specifications: Option<Vec<ElasticGpuSpecification>>,
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,
    #[serde(rename = "LicenseSpecifications")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,
    #[serde(rename = "DisableApiTermination")]
    pub disable_api_termination: Option<bool>,
    #[serde(rename = "HibernationOptions")]
    pub hibernation_options: Option<HibernationOptions>,
    #[serde(rename = "DisableApiStop")]
    pub disable_api_stop: Option<bool>,
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirements>,
    #[serde(rename = "InstanceMarketOptions")]
    pub instance_market_options: Option<InstanceMarketOptions>,
    #[serde(rename = "ElasticInferenceAccelerators")]
    pub elastic_inference_accelerators: Option<Vec<LaunchTemplateElasticInferenceAccelerator>>,
    #[serde(rename = "Placement")]
    pub placement: Option<Placement>,
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,
    #[serde(rename = "PrivateDnsNameOptions")]
    pub private_dns_name_options: Option<PrivateDnsNameOptions>,
    #[serde(rename = "Monitoring")]
    pub monitoring: Option<Monitoring>,
    #[serde(rename = "CpuOptions")]
    pub cpu_options: Option<CpuOptions>,
    #[serde(rename = "MetadataOptions")]
    pub metadata_options: Option<MetadataOptions>,
    #[serde(rename = "CreditSpecification")]
    pub credit_specification: Option<CreditSpecification>,
    #[serde(rename = "EnclaveOptions")]
    pub enclave_options: Option<EnclaveOptions>,
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    #[serde(rename = "MaintenanceOptions")]
    pub maintenance_options: Option<MaintenanceOptions>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "RamDiskId")]
    pub ram_disk_id: Option<String>,
    #[serde(rename = "CapacityReservationSpecification")]
    pub capacity_reservation_specification: Option<CapacityReservationSpecification>,
    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    pub instance_initiated_shutdown_behavior: Option<String>,
    #[serde(rename = "ImageId")]
    pub image_id: Option<String>,
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<TagSpecification>>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<IamInstanceProfile>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorCount {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LicenseSpecification {
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceRequirements {
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpu>,
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCount>,
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGB>,
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<String>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbps>,
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<String>,
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCount>,
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiB>,
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiB>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbps>,
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCount>,
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterface {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Ipv6Prefixes")]
    pub ipv6_prefixes: Option<Vec<Ipv6PrefixSpecification>>,
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "AssociateCarrierIpAddress")]
    pub associate_carrier_ip_address: Option<bool>,
    #[serde(rename = "NetworkCardIndex")]
    pub network_card_index: Option<usize>,
    #[serde(rename = "Ipv6PrefixCount")]
    pub ipv6_prefix_count: Option<usize>,
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "Ipv4PrefixCount")]
    pub ipv4_prefix_count: Option<usize>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "Ipv4Prefixes")]
    pub ipv4_prefixes: Option<Vec<Ipv4PrefixSpecification>>,
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<usize>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<usize>,
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<PrivateIpAdd>>,
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<String>,
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<Ipv6Add>>,
    #[serde(rename = "DeviceIndex")]
    pub device_index: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceMarketOptions {
    #[serde(rename = "MarketType")]
    pub market_type: Option<String>,
    #[serde(rename = "SpotOptions")]
    pub spot_options: Option<SpotOptions>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaceCount {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Placement {
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    #[serde(rename = "HostResourceGroupArn")]
    pub host_resource_group_arn: Option<String>,
    #[serde(rename = "Affinity")]
    pub affinity: Option<String>,
    #[serde(rename = "SpreadDomain")]
    pub spread_domain: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<usize>,
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityReservationTarget {
    #[serde(rename = "CapacityReservationId")]
    pub capacity_reservation_id: Option<String>,
    #[serde(rename = "CapacityReservationResourceGroupArn")]
    pub capacity_reservation_resource_group_arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Ipv6Add {
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryGiBPerVCpu {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct CpuOptions {
    #[serde(rename = "CoreCount")]
    pub core_count: Option<usize>,
    #[serde(rename = "ThreadsPerCore")]
    pub threads_per_core: Option<usize>,
    #[serde(rename = "AmdSevSnp")]
    pub amd_sev_snp: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Monitoring {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct IamInstanceProfile {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkBandwidthGbps {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct BaselineEbsBandwidthMbps {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PrivateDnsNameOptions {
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    pub enable_resource_name_dns_aaaarecord: Option<bool>,
    #[serde(rename = "HostnameType")]
    pub hostname_type: Option<String>,
    #[serde(rename = "EnableResourceNameDnsARecord")]
    pub enable_resource_name_dns_arecord: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryMiB {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Ipv4PrefixSpecification {
    #[serde(rename = "Ipv4Prefix")]
    pub ipv4_prefix: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CapacityReservationSpecification {
    #[serde(rename = "CapacityReservationPreference")]
    pub capacity_reservation_preference: Option<String>,
    #[serde(rename = "CapacityReservationTarget")]
    pub capacity_reservation_target: Option<CapacityReservationTarget>,

}

#[derive(serde::Serialize, Default)]
pub struct HibernationOptions {
    #[serde(rename = "Configured")]
    pub configured: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MetadataOptions {
    #[serde(rename = "HttpEndpoint")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpTokens")]
    pub http_tokens: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    pub http_put_response_hop_limit: Option<usize>,
    #[serde(rename = "HttpProtocolIpv6")]
    pub http_protocol_ipv6: Option<String>,
    #[serde(rename = "InstanceMetadataTags")]
    pub instance_metadata_tags: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorTotalMemoryMiB {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct BlockDeviceMapping {
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,
    #[serde(rename = "Ebs")]
    pub ebs: Option<Ebs>,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct MaintenanceOptions {
    #[serde(rename = "AutoRecovery")]
    pub auto_recovery: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Ebs {
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,

}


}

pub mod cfn_local_gateway_route {

#[derive(serde::Serialize, Default)]
pub struct CfnLocalGatewayRoute {
    /// The ID of the local gateway route table.
    #[serde(rename = "LocalGatewayRouteTableId")]
    pub local_gateway_route_table_id: Option<String>,
    /// The CIDR block used for destination matches.
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: Option<String>,
    /// The ID of the virtual interface group.
    #[serde(rename = "LocalGatewayVirtualInterfaceGroupId")]
    pub local_gateway_virtual_interface_group_id: Option<String>,
    /// The ID of the network interface.
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,

}



}

pub mod cfn_local_gateway_route_table {

#[derive(serde::Serialize, Default)]
pub struct CfnLocalGatewayRouteTable {
    /// The tags for the local gateway route table.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The mode of the local gateway route table.
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    /// The ID of the local gateway.
    #[serde(rename = "LocalGatewayId")]
    pub local_gateway_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_local_gateway_route_table_virtual_interface_group_association {

#[derive(serde::Serialize, Default)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    /// The tags for the local gateway route table virtual interface group association.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The ID of the local gateway route table virtual interface group.
    #[serde(rename = "LocalGatewayVirtualInterfaceGroupId")]
    pub local_gateway_virtual_interface_group_id: String,
    /// The ID of the local gateway route table.
    #[serde(rename = "LocalGatewayRouteTableId")]
    pub local_gateway_route_table_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_local_gateway_route_table_vpcassociation {

#[derive(serde::Serialize, Default)]
pub struct CfnLocalGatewayRouteTableVPCAssociation {
    /// The ID of the local gateway route table.
    #[serde(rename = "LocalGatewayRouteTableId")]
    pub local_gateway_route_table_id: String,
    /// The tags for the association.
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_nat_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnNatGateway {
    /// No documentation provided by AWS
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MaxDrainDurationSeconds")]
    pub max_drain_duration_seconds: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectivityType")]
    pub connectivity_type: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SecondaryPrivateIpAddresses")]
    pub secondary_private_ip_addresses: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "SecondaryAllocationIds")]
    pub secondary_allocation_ids: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "AllocationId")]
    pub allocation_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_network_acl {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkAcl {
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The tags to assign to the network ACL.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_network_acl_entry {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkAclEntry {
    /// The IPv4 CIDR range to allow or deny, in CIDR notation (for example, 172.16.0.0/24). Requirement is conditional: You must specify the CidrBlock or Ipv6CidrBlock property
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    /// The IPv4 network range to allow or deny, in CIDR notation (for example 172.16.0.0/24). We modify the specified CIDR block to its canonical form; for example, if you specify 100.68.0.18/18, we modify it to 100.68.0.0/18
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,
    /// Indicates whether this is an egress rule (rule is applied to traffic leaving the subnet)
    #[serde(rename = "Egress")]
    pub egress: Option<bool>,
    /// Rule number to assign to the entry, such as 100. ACL entries are processed in ascending order by rule number. Entries can't use the same rule number unless one is an egress rule and the other is an ingress rule
    #[serde(rename = "RuleNumber")]
    pub rule_number: usize,
    /// The protocol number. A value of "-1" means all protocols. If you specify "-1" or a protocol number other than "6" (TCP), "17" (UDP), or "1" (ICMP), traffic on all ports is allowed, regardless of any ports or ICMP types or codes that you specify. If you specify protocol "58" (ICMPv6) and specify an IPv4 CIDR block, traffic for all ICMP types and codes allowed, regardless of any that you specify. If you specify protocol "58" (ICMPv6) and specify an IPv6 CIDR block, you must specify an ICMP type and code
    #[serde(rename = "Protocol")]
    pub protocol: usize,
    /// The Internet Control Message Protocol (ICMP) code and type. Requirement is conditional: Required if specifying 1 (ICMP) for the protocol parameter
    #[serde(rename = "Icmp")]
    pub icmp: Option<Icmp>,
    /// Indicates whether to allow or deny the traffic that matches the rule
    #[serde(rename = "RuleAction")]
    pub rule_action: String,
    /// The ID of the network ACL
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: String,
    /// The IPv6 network range to allow or deny, in CIDR notation (for example 2001:db8:1234:1a00::/64)
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Icmp {
    #[serde(rename = "Type")]
    pub ty: Option<usize>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PortRange {
    #[serde(rename = "From")]
    pub from: Option<usize>,
    #[serde(rename = "To")]
    pub to: Option<usize>,

}


}

pub mod cfn_network_insights_access_scope {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInsightsAccessScope {
    /// List of AccessScopePathRequest
    #[serde(rename = "ExcludePaths")]
    pub exclude_paths: Option<Vec<AccessScopePathRequest>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of AccessScopePathRequest
    #[serde(rename = "MatchPaths")]
    pub match_paths: Option<Vec<AccessScopePathRequest>>,

}

pub type Protocol = String;
#[derive(serde::Serialize, Default)]
pub struct ResourceStatementRequest {
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "ResourceTypes")]
    pub resource_types: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct AccessScopePathRequest {
    #[serde(rename = "Source")]
    pub source: Option<PathStatementRequest>,
    #[serde(rename = "ThroughResources")]
    pub through_resources: Option<Vec<ThroughResourcesStatementRequest>>,
    #[serde(rename = "Destination")]
    pub destination: Option<PathStatementRequest>,

}

#[derive(serde::Serialize, Default)]
pub struct PathStatementRequest {
    #[serde(rename = "ResourceStatement")]
    pub resource_statement: Option<ResourceStatementRequest>,
    #[serde(rename = "PacketHeaderStatement")]
    pub packet_header_statement: Option<PacketHeaderStatementRequest>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct PacketHeaderStatementRequest {
    #[serde(rename = "DestinationPrefixLists")]
    pub destination_prefix_lists: Option<Vec<String>>,
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<Protocol>>,
    #[serde(rename = "SourceAddresses")]
    pub source_addresses: Option<Vec<String>>,
    #[serde(rename = "DestinationPorts")]
    pub destination_ports: Option<Vec<String>>,
    #[serde(rename = "SourcePorts")]
    pub source_ports: Option<Vec<String>>,
    #[serde(rename = "DestinationAddresses")]
    pub destination_addresses: Option<Vec<String>>,
    #[serde(rename = "SourcePrefixLists")]
    pub source_prefix_lists: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct ThroughResourcesStatementRequest {
    #[serde(rename = "ResourceStatement")]
    pub resource_statement: Option<ResourceStatementRequest>,

}


}

pub mod cfn_network_insights_access_scope_analysis {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInsightsAccessScopeAnalysis {
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInsightsAccessScopeId")]
    pub network_insights_access_scope_id: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_network_insights_analysis {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInsightsAnalysis {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// List of ResourceArn
    #[serde(rename = "FilterInArns")]
    pub filter_in_arns: Option<Vec<ResourceArn>>,
    /// No documentation provided by AWS
    #[serde(rename = "AdditionalAccounts")]
    pub additional_accounts: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInsightsPathId")]
    pub network_insights_path_id: String,

}

pub type IpAddress = String;
#[derive(serde::Serialize, Default)]
pub struct AnalysisRouteTableRoute {
    #[serde(rename = "VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: Option<String>,
    #[serde(rename = "Origin")]
    pub origin: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "destinationPrefixListId")]
    pub destination_prefix_list_id: Option<String>,
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: Option<String>,
    #[serde(rename = "destinationCidr")]
    pub destination_cidr: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,
    #[serde(rename = "gatewayId")]
    pub gateway_id: Option<String>,
    #[serde(rename = "egressOnlyInternetGatewayId")]
    pub egress_only_internet_gateway_id: Option<String>,
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisPacketHeader {
    #[serde(rename = "SourceAddresses")]
    pub source_addresses: Option<Vec<IpAddress>>,
    #[serde(rename = "DestinationPortRanges")]
    pub destination_port_ranges: Option<Vec<PortRange>>,
    #[serde(rename = "SourcePortRanges")]
    pub source_port_ranges: Option<Vec<PortRange>>,
    #[serde(rename = "DestinationAddresses")]
    pub destination_addresses: Option<Vec<IpAddress>>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,

}

#[derive(serde::Serialize, Default)]
pub struct PathComponent {
    #[serde(rename = "DestinationVpc")]
    pub destination_vpc: Option<AnalysisComponent>,
    #[serde(rename = "RouteTableRoute")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,
    #[serde(rename = "AclRule")]
    pub acl_rule: Option<AnalysisAclRule>,
    #[serde(rename = "SecurityGroupRule")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,
    #[serde(rename = "ElasticLoadBalancerListener")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,
    #[serde(rename = "OutboundHeader")]
    pub outbound_header: Option<AnalysisPacketHeader>,
    #[serde(rename = "TransitGateway")]
    pub transit_gateway: Option<AnalysisComponent>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "Subnet")]
    pub subnet: Option<AnalysisComponent>,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<usize>,
    #[serde(rename = "AdditionalDetails")]
    pub additional_details: Option<Vec<AdditionalDetail>>,
    #[serde(rename = "Vpc")]
    pub vpc: Option<AnalysisComponent>,
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,
    #[serde(rename = "InboundHeader")]
    pub inbound_header: Option<AnalysisPacketHeader>,
    #[serde(rename = "Explanations")]
    pub explanations: Option<Vec<Explanation>>,
    #[serde(rename = "SourceVpc")]
    pub source_vpc: Option<AnalysisComponent>,
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisLoadBalancerListener {
    #[serde(rename = "LoadBalancerPort")]
    pub load_balancer_port: Option<Port>,
    #[serde(rename = "InstancePort")]
    pub instance_port: Option<Port>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type ResourceArn = String;
#[derive(serde::Serialize, Default)]
pub struct AnalysisSecurityGroupRule {
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: Option<String>,
    #[serde(rename = "Direction")]
    pub direction: Option<String>,
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,
    #[serde(rename = "PrefixListId")]
    pub prefix_list_id: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisComponent {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AdditionalDetail {
    #[serde(rename = "AdditionalDetailType")]
    pub additional_detail_type: Option<String>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "LoadBalancers")]
    pub load_balancers: Option<Vec<AnalysisComponent>>,
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}

#[derive(serde::Serialize, Default)]
pub struct Explanation {
    #[serde(rename = "Direction")]
    pub direction: Option<String>,
    #[serde(rename = "InternetGateway")]
    pub internet_gateway: Option<AnalysisComponent>,
    #[serde(rename = "Component")]
    pub component: Option<AnalysisComponent>,
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "IngressRouteTable")]
    pub ingress_route_table: Option<AnalysisComponent>,
    #[serde(rename = "Address")]
    pub address: Option<IpAddress>,
    #[serde(rename = "Acl")]
    pub acl: Option<AnalysisComponent>,
    #[serde(rename = "SecurityGroupRule")]
    pub security_group_rule: Option<AnalysisSecurityGroupRule>,
    #[serde(rename = "TransitGatewayRouteTableRoute")]
    pub transit_gateway_route_table_route: Option<TransitGatewayRouteTableRoute>,
    #[serde(rename = "Subnet")]
    pub subnet: Option<AnalysisComponent>,
    #[serde(rename = "CustomerGateway")]
    pub customer_gateway: Option<AnalysisComponent>,
    #[serde(rename = "Vpc")]
    pub vpc: Option<AnalysisComponent>,
    #[serde(rename = "TransitGatewayRouteTable")]
    pub transit_gateway_route_table: Option<AnalysisComponent>,
    #[serde(rename = "PrefixList")]
    pub prefix_list: Option<AnalysisComponent>,
    #[serde(rename = "ElasticLoadBalancerListener")]
    pub elastic_load_balancer_listener: Option<AnalysisComponent>,
    #[serde(rename = "VpcPeeringConnection")]
    pub vpc_peering_connection: Option<AnalysisComponent>,
    #[serde(rename = "ExplanationCode")]
    pub explanation_code: Option<String>,
    #[serde(rename = "SourceVpc")]
    pub source_vpc: Option<AnalysisComponent>,
    #[serde(rename = "LoadBalancerTargetGroup")]
    pub load_balancer_target_group: Option<AnalysisComponent>,
    #[serde(rename = "TransitGatewayAttachment")]
    pub transit_gateway_attachment: Option<AnalysisComponent>,
    #[serde(rename = "vpcEndpoint")]
    pub vpc_endpoint: Option<AnalysisComponent>,
    #[serde(rename = "Cidrs")]
    pub cidrs: Option<Vec<String>>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<AnalysisComponent>>,
    #[serde(rename = "LoadBalancerTargetPort")]
    pub load_balancer_target_port: Option<Port>,
    #[serde(rename = "Port")]
    pub port: Option<Port>,
    #[serde(rename = "ClassicLoadBalancerListener")]
    pub classic_load_balancer_listener: Option<AnalysisLoadBalancerListener>,
    #[serde(rename = "LoadBalancerTarget")]
    pub load_balancer_target: Option<AnalysisLoadBalancerTarget>,
    #[serde(rename = "LoadBalancerTargetGroups")]
    pub load_balancer_target_groups: Option<Vec<AnalysisComponent>>,
    #[serde(rename = "LoadBalancerListenerPort")]
    pub load_balancer_listener_port: Option<Port>,
    #[serde(rename = "NatGateway")]
    pub nat_gateway: Option<AnalysisComponent>,
    #[serde(rename = "SubnetRouteTable")]
    pub subnet_route_table: Option<AnalysisComponent>,
    #[serde(rename = "RouteTableRoute")]
    pub route_table_route: Option<AnalysisRouteTableRoute>,
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: Option<ResourceArn>,
    #[serde(rename = "DestinationVpc")]
    pub destination_vpc: Option<AnalysisComponent>,
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<AnalysisComponent>,
    #[serde(rename = "Protocols")]
    pub protocols: Option<Vec<Protocol>>,
    #[serde(rename = "NetworkInterface")]
    pub network_interface: Option<AnalysisComponent>,
    #[serde(rename = "AclRule")]
    pub acl_rule: Option<AnalysisAclRule>,
    #[serde(rename = "RouteTable")]
    pub route_table: Option<AnalysisComponent>,
    #[serde(rename = "TransitGateway")]
    pub transit_gateway: Option<AnalysisComponent>,
    #[serde(rename = "ComponentAccount")]
    pub component_account: Option<String>,
    #[serde(rename = "ComponentRegion")]
    pub component_region: Option<String>,
    #[serde(rename = "MissingComponent")]
    pub missing_component: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: Option<AnalysisComponent>,
    #[serde(rename = "PortRanges")]
    pub port_ranges: Option<Vec<PortRange>>,
    #[serde(rename = "Addresses")]
    pub addresses: Option<Vec<IpAddress>>,
    #[serde(rename = "VpnGateway")]
    pub vpn_gateway: Option<AnalysisComponent>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "VpnConnection")]
    pub vpn_connection: Option<AnalysisComponent>,
    #[serde(rename = "SecurityGroup")]
    pub security_group: Option<AnalysisComponent>,
    #[serde(rename = "PacketField")]
    pub packet_field: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AlternatePathHint {
    #[serde(rename = "ComponentArn")]
    pub component_arn: Option<String>,
    #[serde(rename = "ComponentId")]
    pub component_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PortRange {
    #[serde(rename = "To")]
    pub to: Option<usize>,
    #[serde(rename = "From")]
    pub from: Option<usize>,

}
pub type Port = usize;
#[derive(serde::Serialize, Default)]
pub struct AnalysisAclRule {
    #[serde(rename = "Egress")]
    pub egress: Option<bool>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,
    #[serde(rename = "RuleAction")]
    pub rule_action: Option<String>,
    #[serde(rename = "RuleNumber")]
    pub rule_number: Option<usize>,
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AnalysisLoadBalancerTarget {
    #[serde(rename = "Port")]
    pub port: Option<Port>,
    #[serde(rename = "Instance")]
    pub instance: Option<AnalysisComponent>,
    #[serde(rename = "Address")]
    pub address: Option<IpAddress>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TransitGatewayRouteTableRoute {
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "DestinationCidr")]
    pub destination_cidr: Option<String>,
    #[serde(rename = "RouteOrigin")]
    pub route_origin: Option<String>,
    #[serde(rename = "PrefixListId")]
    pub prefix_list_id: Option<String>,
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "AttachmentId")]
    pub attachment_id: Option<String>,

}
pub type Protocol = String;

}

pub mod cfn_network_insights_path {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInsightsPath {
    /// No documentation provided by AWS
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FilterAtDestination")]
    pub filter_at_destination: Option<PathFilter>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationIp")]
    pub destination_ip: Option<IpAddress>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationPort")]
    pub destination_port: Option<Port>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Source")]
    pub source: String,
    /// No documentation provided by AWS
    #[serde(rename = "FilterAtSource")]
    pub filter_at_source: Option<PathFilter>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceIp")]
    pub source_ip: Option<IpAddress>,
    /// No documentation provided by AWS
    #[serde(rename = "Protocol")]
    pub protocol: Protocol,

}

pub type Protocol = String;
#[derive(serde::Serialize, Default)]
pub struct FilterPortRange {
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}
pub type IpAddress = String;pub type Port = usize;
#[derive(serde::Serialize, Default)]
pub struct PathFilter {
    #[serde(rename = "DestinationPortRange")]
    pub destination_port_range: Option<FilterPortRange>,
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Option<IpAddress>,
    #[serde(rename = "SourceAddress")]
    pub source_address: Option<IpAddress>,
    #[serde(rename = "SourcePortRange")]
    pub source_port_range: Option<FilterPortRange>,

}

#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_network_interface {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInterface {
    /// The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range. To specify specific IPv6 addresses, use the Ipv6Addresses property and don't specify this property.
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<usize>,
    /// An arbitrary set of tags (key-value pairs) for this network interface.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Indicates whether traffic to or from the instance is validated.
    #[serde(rename = "SourceDestCheck")]
    pub source_dest_check: Option<bool>,
    /// A description for the network interface.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// A list of security group IDs associated with this network interface.
    #[serde(rename = "GroupSet")]
    pub group_set: Option<Vec<String>>,
    /// One or more specific IPv6 addresses from the IPv6 CIDR block range of your subnet to associate with the network interface. If you're specifying a number of IPv6 addresses, use the Ipv6AddressCount property and don't specify this property.
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,
    /// The ID of the subnet to associate with the network interface.
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// The number of secondary private IPv4 addresses to assign to a network interface. When you specify a number of secondary IPv4 addresses, Amazon EC2 selects these IP addresses within the subnet's IPv4 CIDR range. You can't specify this option and specify more than one private IP address using privateIpAddresses
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<usize>,
    /// Indicates the type of network interface.
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct PrivateIpAddressSpecification {
    #[serde(rename = "Primary")]
    pub primary: bool,
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceIpv6Address {
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: String,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_network_interface_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInterfaceAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "DeviceIndex")]
    pub device_index: String,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

}



}

pub mod cfn_network_interface_permission {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkInterfacePermission {
    /// No documentation provided by AWS
    #[serde(rename = "Permission")]
    pub permission: String,
    /// No documentation provided by AWS
    #[serde(rename = "AwsAccountId")]
    pub aws_account_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

}



}

pub mod cfn_network_performance_metric_subscription {

#[derive(serde::Serialize, Default)]
pub struct CfnNetworkPerformanceMetricSubscription {
    /// The target Region or Availability Zone for the metric to subscribe to.
    #[serde(rename = "Destination")]
    pub destination: String,
    /// The starting Region or Availability Zone for metric to subscribe to.
    #[serde(rename = "Source")]
    pub source: String,
    /// The metric type to subscribe to.
    #[serde(rename = "Metric")]
    pub metric: String,
    /// The statistic to subscribe to.
    #[serde(rename = "Statistic")]
    pub statistic: String,

}



}

pub mod cfn_placement_group {

#[derive(serde::Serialize, Default)]
pub struct CfnPlacementGroup {
    /// The placement strategy.
    #[serde(rename = "Strategy")]
    pub strategy: Option<String>,
    /// The Spread Level of Placement Group is an enum where it accepts either host or rack when strategy is spread
    #[serde(rename = "SpreadLevel")]
    pub spread_level: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The number of partitions. Valid only when **Strategy** is set to `partition`
    #[serde(rename = "PartitionCount")]
    pub partition_count: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_prefix_list {

#[derive(serde::Serialize, Default)]
pub struct CfnPrefixList {
    /// Max Entries of Prefix List.
    #[serde(rename = "MaxEntries")]
    pub max_entries: usize,
    /// Tags for Prefix List
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Ip Version of Prefix List.
    #[serde(rename = "AddressFamily")]
    pub address_family: String,
    /// Name of Prefix List.
    #[serde(rename = "PrefixListName")]
    pub prefix_list_name: String,
    /// Entries of Prefix List.
    #[serde(rename = "Entries")]
    pub entries: Option<Vec<Entry>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Entry {
    #[serde(rename = "Cidr")]
    pub cidr: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


}

pub mod cfn_route {

#[derive(serde::Serialize, Default)]
pub struct CfnRoute {
    /// No documentation provided by AWS
    #[serde(rename = "LocalGatewayId")]
    pub local_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationIpv6CidrBlock")]
    pub destination_ipv6_cidr_block: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CarrierGatewayId")]
    pub carrier_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GatewayId")]
    pub gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcEndpointId")]
    pub vpc_endpoint_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "EgressOnlyInternetGatewayId")]
    pub egress_only_internet_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "InstanceId")]
    pub instance_id: Option<String>,

}



}

pub mod cfn_route_table {

#[derive(serde::Serialize, Default)]
pub struct CfnRouteTable {
    /// Any tags assigned to the route table.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_security_group {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityGroup {
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupDescription")]
    pub group_description: String,
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    /// List of Egress
    #[serde(rename = "SecurityGroupEgress")]
    pub security_group_egress: Option<Vec<Egress>>,
    /// List of Ingress
    #[serde(rename = "SecurityGroupIngress")]
    pub security_group_ingress: Option<Vec<Ingress>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Ingress {
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,
    #[serde(rename = "SourceSecurityGroupOwnerId")]
    pub source_security_group_owner_id: Option<String>,
    #[serde(rename = "SourceSecurityGroupName")]
    pub source_security_group_name: Option<String>,
    #[serde(rename = "SourcePrefixListId")]
    pub source_prefix_list_id: Option<String>,
    #[serde(rename = "SourceSecurityGroupId")]
    pub source_security_group_id: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: String,
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    #[serde(rename = "CidrIpv6")]
    pub cidr_ipv6: Option<String>,
    #[serde(rename = "CidrIp")]
    pub cidr_ip: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Egress {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "CidrIpv6")]
    pub cidr_ipv6: Option<String>,
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: String,
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    #[serde(rename = "DestinationSecurityGroupId")]
    pub destination_security_group_id: Option<String>,
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,
    #[serde(rename = "DestinationPrefixListId")]
    pub destination_prefix_list_id: Option<String>,
    #[serde(rename = "CidrIp")]
    pub cidr_ip: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_security_group_egress {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityGroupEgress {
    /// No documentation provided by AWS
    #[serde(rename = "CidrIp")]
    pub cidr_ip: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: String,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationSecurityGroupId")]
    pub destination_security_group_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrIpv6")]
    pub cidr_ipv6: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupId")]
    pub group_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationPrefixListId")]
    pub destination_prefix_list_id: Option<String>,

}



}

pub mod cfn_security_group_ingress {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityGroupIngress {
    /// No documentation provided by AWS
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: String,
    /// No documentation provided by AWS
    #[serde(rename = "SourcePrefixListId")]
    pub source_prefix_list_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceSecurityGroupOwnerId")]
    pub source_security_group_owner_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "FromPort")]
    pub from_port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "ToPort")]
    pub to_port: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrIp")]
    pub cidr_ip: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceSecurityGroupId")]
    pub source_security_group_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceSecurityGroupName")]
    pub source_security_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrIpv6")]
    pub cidr_ipv6: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,

}



}

pub mod cfn_spot_fleet {

#[derive(serde::Serialize, Default)]
pub struct CfnSpotFleet {
    /// No documentation provided by AWS
    #[serde(rename = "SpotFleetRequestConfigData")]
    pub spot_fleet_request_config_data: SpotFleetRequestConfigData,

}


#[derive(serde::Serialize, Default)]
pub struct BlockDeviceMapping {
    #[serde(rename = "VirtualName")]
    pub virtual_name: Option<String>,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "Ebs")]
    pub ebs: Option<EbsBlockDevice>,
    #[serde(rename = "NoDevice")]
    pub no_device: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroup {
    #[serde(rename = "Arn")]
    pub arn: String,

}

#[derive(serde::Serialize, Default)]
pub struct ClassicLoadBalancer {
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceRequirementsRequest {
    #[serde(rename = "AllowedInstanceTypes")]
    pub allowed_instance_types: Option<Vec<String>>,
    #[serde(rename = "AcceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "AcceleratorManufacturers")]
    pub accelerator_manufacturers: Option<Vec<String>>,
    #[serde(rename = "LocalStorage")]
    pub local_storage: Option<String>,
    #[serde(rename = "AcceleratorNames")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    pub accelerator_total_memory_mi_b: Option<AcceleratorTotalMemoryMiBRequest>,
    #[serde(rename = "InstanceGenerations")]
    pub instance_generations: Option<Vec<String>>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    pub spot_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "AcceleratorCount")]
    pub accelerator_count: Option<AcceleratorCountRequest>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    pub baseline_ebs_bandwidth_mbps: Option<BaselineEbsBandwidthMbpsRequest>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<usize>,
    #[serde(rename = "VCpuCount")]
    pub vcpu_count: Option<VCpuCountRangeRequest>,
    #[serde(rename = "BareMetal")]
    pub bare_metal: Option<String>,
    #[serde(rename = "MemoryMiB")]
    pub memory_mi_b: Option<MemoryMiBRequest>,
    #[serde(rename = "BurstablePerformance")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "CpuManufacturers")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    pub memory_gi_bper_vcpu: Option<MemoryGiBPerVCpuRequest>,
    #[serde(rename = "NetworkBandwidthGbps")]
    pub network_bandwidth_gbps: Option<NetworkBandwidthGbpsRequest>,
    #[serde(rename = "ExcludedInstanceTypes")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "NetworkInterfaceCount")]
    pub network_interface_count: Option<NetworkInterfaceCountRequest>,
    #[serde(rename = "LocalStorageTypes")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "TotalLocalStorageGB")]
    pub total_local_storage_gb: Option<TotalLocalStorageGBRequest>,
    #[serde(rename = "RequireHibernateSupport")]
    pub require_hibernate_support: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotMaintenanceStrategies {
    #[serde(rename = "CapacityRebalance")]
    pub capacity_rebalance: Option<SpotCapacityRebalance>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorTotalMemoryMiBRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct PrivateIpAddressSpecification {
    #[serde(rename = "PrivateIpAddress")]
    pub private_ip_address: String,
    #[serde(rename = "Primary")]
    pub primary: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct TargetGroupsConfig {
    #[serde(rename = "TargetGroups")]
    pub target_groups: Vec<TargetGroup>,

}

#[derive(serde::Serialize, Default)]
pub struct IamInstanceProfileSpecification {
    #[serde(rename = "Arn")]
    pub arn: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct VCpuCountRangeRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateConfig {
    #[serde(rename = "Overrides")]
    pub overrides: Option<Vec<LaunchTemplateOverrides>>,
    #[serde(rename = "LaunchTemplateSpecification")]
    pub launch_template_specification: Option<FleetLaunchTemplateSpecification>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotCapacityRebalance {
    #[serde(rename = "ReplacementStrategy")]
    pub replacement_strategy: Option<String>,
    #[serde(rename = "TerminationDelay")]
    pub termination_delay: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceIpv6Address {
    #[serde(rename = "Ipv6Address")]
    pub ipv6_address: String,

}

#[derive(serde::Serialize, Default)]
pub struct SpotFleetRequestConfigData {
    #[serde(rename = "TargetCapacity")]
    pub target_capacity: usize,
    #[serde(rename = "AllocationStrategy")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "LoadBalancersConfig")]
    pub load_balancers_config: Option<LoadBalancersConfig>,
    #[serde(rename = "OnDemandAllocationStrategy")]
    pub on_demand_allocation_strategy: Option<String>,
    #[serde(rename = "TerminateInstancesWithExpiration")]
    pub terminate_instances_with_expiration: Option<bool>,
    #[serde(rename = "IamFleetRole")]
    pub iam_fleet_role: String,
    #[serde(rename = "InstancePoolsToUseCount")]
    pub instance_pools_to_use_count: Option<usize>,
    #[serde(rename = "ReplaceUnhealthyInstances")]
    pub replace_unhealthy_instances: Option<bool>,
    #[serde(rename = "SpotPrice")]
    pub spot_price: Option<String>,
    #[serde(rename = "ValidUntil")]
    pub valid_until: Option<String>,
    #[serde(rename = "TargetCapacityUnitType")]
    pub target_capacity_unit_type: Option<String>,
    #[serde(rename = "OnDemandMaxTotalPrice")]
    pub on_demand_max_total_price: Option<String>,
    #[serde(rename = "ValidFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "ExcessCapacityTerminationPolicy")]
    pub excess_capacity_termination_policy: Option<String>,
    #[serde(rename = "SpotMaintenanceStrategies")]
    pub spot_maintenance_strategies: Option<SpotMaintenanceStrategies>,
    #[serde(rename = "SpotMaxTotalPrice")]
    pub spot_max_total_price: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    #[serde(rename = "LaunchSpecifications")]
    pub launch_specifications: Option<Vec<SpotFleetLaunchSpecification>>,
    #[serde(rename = "InstanceInterruptionBehavior")]
    pub instance_interruption_behavior: Option<String>,
    #[serde(rename = "LaunchTemplateConfigs")]
    pub launch_template_configs: Option<Vec<LaunchTemplateConfig>>,
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<SpotFleetTagSpecification>>,
    #[serde(rename = "Context")]
    pub context: Option<String>,
    #[serde(rename = "OnDemandTargetCapacity")]
    pub on_demand_target_capacity: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotFleetLaunchSpecification {
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Option<Vec<InstanceNetworkInterfaceSpecification>>,
    #[serde(rename = "UserData")]
    pub user_data: Option<String>,
    #[serde(rename = "EbsOptimized")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "KernelId")]
    pub kernel_id: Option<String>,
    #[serde(rename = "Placement")]
    pub placement: Option<SpotPlacement>,
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,
    #[serde(rename = "SpotPrice")]
    pub spot_price: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    pub tag_specifications: Option<Vec<SpotFleetTagSpecification>>,
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<GroupIdentifier>>,
    #[serde(rename = "RamdiskId")]
    pub ramdisk_id: Option<String>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "BlockDeviceMappings")]
    pub block_device_mappings: Option<Vec<BlockDeviceMapping>>,
    #[serde(rename = "IamInstanceProfile")]
    pub iam_instance_profile: Option<IamInstanceProfileSpecification>,
    #[serde(rename = "ImageId")]
    pub image_id: String,
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,
    #[serde(rename = "Monitoring")]
    pub monitoring: Option<SpotFleetMonitoring>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct FleetLaunchTemplateSpecification {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "LaunchTemplateId")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    pub launch_template_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct InstanceNetworkInterfaceSpecification {
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "AssociatePublicIpAddress")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Ipv6AddressCount")]
    pub ipv6_address_count: Option<usize>,
    #[serde(rename = "PrivateIpAddresses")]
    pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification>>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "Groups")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    pub secondary_private_ip_address_count: Option<usize>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "DeviceIndex")]
    pub device_index: Option<usize>,
    #[serde(rename = "Ipv6Addresses")]
    pub ipv6_addresses: Option<Vec<InstanceIpv6Address>>,

}

#[derive(serde::Serialize, Default)]
pub struct EbsBlockDevice {
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "VolumeSize")]
    pub volume_size: Option<usize>,
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    #[serde(rename = "DeleteOnTermination")]
    pub delete_on_termination: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotFleetTagSpecification {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct LaunchTemplateOverrides {
    #[serde(rename = "Priority")]
    pub priority: Option<f64>,
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,
    #[serde(rename = "SpotPrice")]
    pub spot_price: Option<String>,
    #[serde(rename = "InstanceRequirements")]
    pub instance_requirements: Option<InstanceRequirementsRequest>,
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,
    #[serde(rename = "WeightedCapacity")]
    pub weighted_capacity: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryMiBRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotFleetMonitoring {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct MemoryGiBPerVCpuRequest {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaceCountRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct TotalLocalStorageGBRequest {
    #[serde(rename = "Max")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    pub min: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct ClassicLoadBalancersConfig {
    #[serde(rename = "ClassicLoadBalancers")]
    pub classic_load_balancers: Vec<ClassicLoadBalancer>,

}

#[derive(serde::Serialize, Default)]
pub struct SpotPlacement {
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    #[serde(rename = "GroupName")]
    pub group_name: Option<String>,
    #[serde(rename = "Tenancy")]
    pub tenancy: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct AcceleratorCountRequest {
    #[serde(rename = "Min")]
    pub min: Option<usize>,
    #[serde(rename = "Max")]
    pub max: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadBalancersConfig {
    #[serde(rename = "TargetGroupsConfig")]
    pub target_groups_config: Option<TargetGroupsConfig>,
    #[serde(rename = "ClassicLoadBalancersConfig")]
    pub classic_load_balancers_config: Option<ClassicLoadBalancersConfig>,

}

#[derive(serde::Serialize, Default)]
pub struct NetworkBandwidthGbpsRequest {
    #[serde(rename = "Min")]
    pub min: Option<f64>,
    #[serde(rename = "Max")]
    pub max: Option<f64>,

}

#[derive(serde::Serialize, Default)]
pub struct BaselineEbsBandwidthMbpsRequest {
    #[serde(rename = "Max")]
    pub max: Option<usize>,
    #[serde(rename = "Min")]
    pub min: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct GroupIdentifier {
    #[serde(rename = "GroupId")]
    pub group_id: String,

}


}

pub mod cfn_subnet {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnet {
    /// No documentation provided by AWS
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "EnableDns64")]
    pub enable_dns64: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PrivateDnsNameOptionsOnLaunch")]
    pub private_dns_name_options_on_launch: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6Native")]
    pub ipv6_native: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "MapPublicIpOnLaunch")]
    pub map_public_ip_on_launch: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "AssignIpv6AddressOnCreation")]
    pub assign_ipv6_address_on_creation: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZoneId")]
    pub availability_zone_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_subnet_cidr_block {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetCidrBlock {
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: String,

}



}

pub mod cfn_subnet_network_acl_association {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetNetworkAclAssociation {
    /// The ID of the subnet
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// The ID of the network ACL
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: String,

}



}

pub mod cfn_subnet_route_table_association {

#[derive(serde::Serialize, Default)]
pub struct CfnSubnetRouteTableAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "RouteTableId")]
    pub route_table_id: String,

}



}

pub mod cfn_traffic_mirror_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnTrafficMirrorFilter {
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkServices")]
    pub network_services: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_traffic_mirror_filter_rule {

#[derive(serde::Serialize, Default)]
pub struct CfnTrafficMirrorFilterRule {
    /// No documentation provided by AWS
    #[serde(rename = "SourceCidrBlock")]
    pub source_cidr_block: String,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,
    /// No documentation provided by AWS
    #[serde(rename = "TrafficMirrorFilterId")]
    pub traffic_mirror_filter_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "RuleAction")]
    pub rule_action: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "RuleNumber")]
    pub rule_number: usize,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationPortRange")]
    pub destination_port_range: Option<TrafficMirrorPortRange>,
    /// No documentation provided by AWS
    #[serde(rename = "TrafficDirection")]
    pub traffic_direction: String,
    /// No documentation provided by AWS
    #[serde(rename = "SourcePortRange")]
    pub source_port_range: Option<TrafficMirrorPortRange>,
    /// No documentation provided by AWS
    #[serde(rename = "Protocol")]
    pub protocol: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct TrafficMirrorPortRange {
    #[serde(rename = "ToPort")]
    pub to_port: usize,
    #[serde(rename = "FromPort")]
    pub from_port: usize,

}


}

pub mod cfn_traffic_mirror_session {

#[derive(serde::Serialize, Default)]
pub struct CfnTrafficMirrorSession {
    /// No documentation provided by AWS
    #[serde(rename = "TrafficMirrorFilterId")]
    pub traffic_mirror_filter_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PacketLength")]
    pub packet_length: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "TrafficMirrorTargetId")]
    pub traffic_mirror_target_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SessionNumber")]
    pub session_number: usize,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "VirtualNetworkId")]
    pub virtual_network_id: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_traffic_mirror_target {

#[derive(serde::Serialize, Default)]
pub struct CfnTrafficMirrorTarget {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkLoadBalancerArn")]
    pub network_load_balancer_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GatewayLoadBalancerEndpointId")]
    pub gateway_load_balancer_endpoint_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_transit_gateway {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGateway {
    /// No documentation provided by AWS
    #[serde(rename = "AmazonSideAsn")]
    pub amazon_side_asn: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "MulticastSupport")]
    pub multicast_support: Option<String>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRouteTableAssociation")]
    pub default_route_table_association: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PropagationDefaultRouteTableId")]
    pub propagation_default_route_table_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DefaultRouteTablePropagation")]
    pub default_route_table_propagation: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpnEcmpSupport")]
    pub vpn_ecmp_support: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DnsSupport")]
    pub dns_support: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayCidrBlocks")]
    pub transit_gateway_cidr_blocks: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AssociationDefaultRouteTableId")]
    pub association_default_route_table_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "AutoAcceptSharedAttachments")]
    pub auto_accept_shared_attachments: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_transit_gateway_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayAttachment {
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The options for the transit gateway vpc attachment.
    #[serde(rename = "Options")]
    pub options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_transit_gateway_connect {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayConnect {
    /// The ID of the attachment from which the Connect attachment was created.
    #[serde(rename = "TransportTransitGatewayAttachmentId")]
    pub transport_transit_gateway_attachment_id: String,
    /// The Connect attachment options.
    #[serde(rename = "Options")]
    pub options: TransitGatewayConnectOptions,
    /// The tags for the attachment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct TransitGatewayConnectOptions {
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_transit_gateway_multicast_domain {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayMulticastDomain {
    /// The ID of the transit gateway.
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,
    /// The options for the transit gateway multicast domain.
    #[serde(rename = "Options")]
    pub options: Option<()>,
    /// The tags for the transit gateway multicast domain.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Key")]
    pub key: Option<String>,

}


}

pub mod cfn_transit_gateway_multicast_domain_association {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayMulticastDomainAssociation {
    /// The IDs of the subnets to associate with the transit gateway multicast domain.
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    /// The ID of the transit gateway multicast domain.
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,
    /// The ID of the transit gateway attachment.
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,

}



}

pub mod cfn_transit_gateway_multicast_group_member {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayMulticastGroupMember {
    /// The ID of the transit gateway multicast domain.
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,
    /// The IP address assigned to the transit gateway multicast group.
    #[serde(rename = "GroupIpAddress")]
    pub group_ip_address: String,
    /// The ID of the transit gateway attachment.
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

}



}

pub mod cfn_transit_gateway_multicast_group_source {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayMulticastGroupSource {
    /// The IP address assigned to the transit gateway multicast group.
    #[serde(rename = "GroupIpAddress")]
    pub group_ip_address: String,
    /// The ID of the transit gateway attachment.
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,
    /// The ID of the transit gateway multicast domain.
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    pub transit_gateway_multicast_domain_id: String,

}



}

pub mod cfn_transit_gateway_peering_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayPeeringAttachment {
    /// The ID of the peer transit gateway.
    #[serde(rename = "PeerTransitGatewayId")]
    pub peer_transit_gateway_id: String,
    /// The ID of the peer account
    #[serde(rename = "PeerAccountId")]
    pub peer_account_id: String,
    /// The ID of the transit gateway.
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,
    /// The tags for the transit gateway peering attachment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Peer Region
    #[serde(rename = "PeerRegion")]
    pub peer_region: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct PeeringAttachmentStatus {
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,

}


}

pub mod cfn_transit_gateway_route {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRoute {
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Blackhole")]
    pub blackhole: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: Option<String>,

}



}

pub mod cfn_transit_gateway_route_table {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRouteTable {
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_transit_gateway_route_table_association {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRouteTableAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: String,

}



}

pub mod cfn_transit_gateway_route_table_propagation {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRouteTablePropagation {
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayRouteTableId")]
    pub transit_gateway_route_table_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayAttachmentId")]
    pub transit_gateway_attachment_id: String,

}



}

pub mod cfn_transit_gateway_vpc_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayVpcAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// The options for the transit gateway vpc attachment.
    #[serde(rename = "Options")]
    pub options: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "RemoveSubnetIds")]
    pub remove_subnet_ids: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "AddSubnetIds")]
    pub add_subnet_ids: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_verified_access_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnVerifiedAccessEndpoint {
    /// The type of AWS Verified Access endpoint. Incoming application requests will be sent to an IP address, load balancer or a network interface depending on the endpoint type specified.The type of AWS Verified Access endpoint. Incoming application requests will be sent to an IP address, load balancer or a network interface depending on the endpoint type specified.
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// The IDs of the security groups for the endpoint.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<SecurityGroupId>>,
    /// A description for the AWS Verified Access endpoint.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The type of attachment used to provide connectivity between the AWS Verified Access endpoint and the application.
    #[serde(rename = "AttachmentType")]
    pub attachment_type: String,
    /// The AWS Verified Access policy document.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<String>,
    /// The DNS name for users to reach your application.
    #[serde(rename = "ApplicationDomain")]
    pub application_domain: String,
    /// A custom identifier that gets prepended to a DNS name that is generated for the endpoint.
    #[serde(rename = "EndpointDomainPrefix")]
    pub endpoint_domain_prefix: String,
    /// The load balancer details if creating the AWS Verified Access endpoint as load-balancer type.
    #[serde(rename = "LoadBalancerOptions")]
    pub load_balancer_options: Option<LoadBalancerOptions>,
    /// The options for network-interface type endpoint.
    #[serde(rename = "NetworkInterfaceOptions")]
    pub network_interface_options: Option<NetworkInterfaceOptions>,
    /// The status of the Verified Access policy.
    #[serde(rename = "PolicyEnabled")]
    pub policy_enabled: Option<bool>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the AWS Verified Access group.
    #[serde(rename = "VerifiedAccessGroupId")]
    pub verified_access_group_id: String,
    /// The ARN of a public TLS/SSL certificate imported into or created with ACM.
    #[serde(rename = "DomainCertificateArn")]
    pub domain_certificate_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct NetworkInterfaceOptions {
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct LoadBalancerOptions {
    #[serde(rename = "LoadBalancerArn")]
    pub load_balancer_arn: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<SubnetId>>,
    #[serde(rename = "Port")]
    pub port: Option<usize>,

}
pub type SecurityGroupId = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type SubnetId = String;

}

pub mod cfn_verified_access_group {

#[derive(serde::Serialize, Default)]
pub struct CfnVerifiedAccessGroup {
    /// A description for the AWS Verified Access group.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The AWS Verified Access policy document.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the AWS Verified Access instance.
    #[serde(rename = "VerifiedAccessInstanceId")]
    pub verified_access_instance_id: String,
    /// The status of the Verified Access policy.
    #[serde(rename = "PolicyEnabled")]
    pub policy_enabled: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_verified_access_instance {

#[derive(serde::Serialize, Default)]
pub struct CfnVerifiedAccessInstance {
    /// AWS Verified Access trust providers.
    #[serde(rename = "VerifiedAccessTrustProviders")]
    pub verified_access_trust_providers: Option<Vec<VerifiedAccessTrustProvider>>,
    /// The IDs of the AWS Verified Access trust providers.
    #[serde(rename = "VerifiedAccessTrustProviderIds")]
    pub verified_access_trust_provider_ids: Option<Vec<VerifiedAccessTrustProviderId>>,
    /// The configuration options for AWS Verified Access instances.
    #[serde(rename = "LoggingConfigurations")]
    pub logging_configurations: Option<VerifiedAccessLogs>,
    /// A description for the AWS Verified Access instance.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct VerifiedAccessLogs {
    #[serde(rename = "KinesisDataFirehose")]
    pub kinesis_data_firehose: Option<()>,
    #[serde(rename = "CloudWatchLogs")]
    pub cloud_watch_logs: Option<()>,
    #[serde(rename = "S3")]
    pub s3: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct VerifiedAccessTrustProvider {
    #[serde(rename = "DeviceTrustProviderType")]
    pub device_trust_provider_type: Option<String>,
    #[serde(rename = "VerifiedAccessTrustProviderId")]
    pub verified_access_trust_provider_id: Option<String>,
    #[serde(rename = "TrustProviderType")]
    pub trust_provider_type: Option<String>,
    #[serde(rename = "UserTrustProviderType")]
    pub user_trust_provider_type: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,

}
pub type VerifiedAccessTrustProviderId = String;

}

pub mod cfn_verified_access_trust_provider {

#[derive(serde::Serialize, Default)]
pub struct CfnVerifiedAccessTrustProvider {
    /// Type of trust provider. Possible values: user|device
    #[serde(rename = "TrustProviderType")]
    pub trust_provider_type: String,
    /// The type of device-based trust provider. Possible values: oidc|iam-identity-center
    #[serde(rename = "UserTrustProviderType")]
    pub user_trust_provider_type: Option<String>,
    /// The OpenID Connect details for an oidc -type, user-identity based trust provider.
    #[serde(rename = "OidcOptions")]
    pub oidc_options: Option<OidcOptions>,
    /// The options for device identity based trust providers.
    #[serde(rename = "DeviceOptions")]
    pub device_options: Option<DeviceOptions>,
    /// The identifier to be used when working with policy rules.
    #[serde(rename = "PolicyReferenceName")]
    pub policy_reference_name: String,
    /// A description for the Amazon Web Services Verified Access trust provider.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The type of device-based trust provider. Possible values: jamf|crowdstrike
    #[serde(rename = "DeviceTrustProviderType")]
    pub device_trust_provider_type: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct DeviceOptions {
    #[serde(rename = "TenantId")]
    pub tenant_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct OidcOptions {
    #[serde(rename = "ClientSecret")]
    pub client_secret: Option<String>,
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,
    #[serde(rename = "ClientId")]
    pub client_id: Option<String>,
    #[serde(rename = "UserInfoEndpoint")]
    pub user_info_endpoint: Option<String>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "TokenEndpoint")]
    pub token_endpoint: Option<String>,
    #[serde(rename = "AuthorizationEndpoint")]
    pub authorization_endpoint: Option<String>,

}


}

pub mod cfn_volume {

#[derive(serde::Serialize, Default)]
pub struct CfnVolume {
    /// Specifies whether the volume should be encrypted. The effect of setting the encryption state to true depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see Encryption by default in the Amazon Elastic Compute Cloud User Guide. Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption. For more information, see Supported instance types.
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    /// The tags to apply to the volume during creation.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The Amazon Resource Name (ARN) of the Outpost.
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,
    /// The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size.  Constraints: 1-16,384 for gp2, 4-16,384 for io1 and io2, 500-16,384 for st1, 500-16,384 for sc1, and 1-1,024 for standard. If you specify a snapshot, the volume size must be equal to or larger than the snapshot size. Default: If you're creating the volume from a snapshot and don't specify a volume size, the default is the snapshot size.
    #[serde(rename = "Size")]
    pub size: Option<usize>,
    /// The Availability Zone in which to create the volume.
    #[serde(rename = "AutoEnableIO")]
    pub auto_enable_io: Option<bool>,
    /// The throughput that the volume supports, in MiB/s.
    #[serde(rename = "Throughput")]
    pub throughput: Option<usize>,
    /// The number of I/O operations per second (IOPS) to provision for an io1 or io2 volume, with a maximum ratio of 50 IOPS/GiB for io1, and 500 IOPS/GiB for io2. Range is 100 to 64,000 IOPS for volumes in most Regions. Maximum IOPS of 64,000 is guaranteed only on Nitro-based instances. Other instance families guarantee performance up to 32,000 IOPS. For more information, see Amazon EBS volume types in the Amazon Elastic Compute Cloud User Guide. This parameter is valid only for Provisioned IOPS SSD (io1 and io2) volumes.
    #[serde(rename = "Iops")]
    pub iops: Option<usize>,
    /// The Availability Zone in which to create the volume.
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: Option<String>,
    /// The volume type. This parameter can be one of the following values: General Purpose SSD: gp2 | gp3, Provisioned IOPS SSD: io1 | io2, Throughput Optimized HDD: st1, Cold HDD: sc1, Magnetic: standard
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<String>,
    /// Indicates whether Amazon EBS Multi-Attach is enabled.
    #[serde(rename = "MultiAttachEnabled")]
    pub multi_attach_enabled: Option<bool>,
    /// The identifier of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use for Amazon EBS encryption. If KmsKeyId is specified, the encrypted state must be true. If you omit this property and your account is enabled for encryption by default, or Encrypted is set to true, then the volume is encrypted using the default CMK specified for your account. If your account does not have a default CMK, then the volume is encrypted using the AWS managed CMK.  Alternatively, if you want to specify a different CMK, you can specify one of the following:  Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab. Key alias. Specify the alias for the CMK, prefixed with alias/. For example, for a CMK with the alias my_cmk, use alias/my_cmk. Or to specify the AWS managed CMK, use alias/aws/ebs. Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab. Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_volume_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnVolumeAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Device")]
    pub device: String,
    /// No documentation provided by AWS
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

}



}

pub mod cfn_vpc {

#[derive(serde::Serialize, Default)]
pub struct CfnVPC {
    /// Indicates whether the instances launched in the VPC get DNS hostnames. If enabled, instances in the VPC get DNS hostnames; otherwise, they do not. Disabled by default for nondefault VPCs.
    #[serde(rename = "EnableDnsHostnames")]
    pub enable_dns_hostnames: Option<bool>,
    /// The allowed tenancy of instances launched into the VPC.
    /// 
    /// "default": An instance launched into the VPC runs on shared hardware by default, unless you explicitly specify a different tenancy during instance launch.
    /// 
    /// "dedicated": An instance launched into the VPC is a Dedicated Instance by default, unless you explicitly specify a tenancy of host during instance launch. You cannot specify a tenancy of default during instance launch.
    /// 
    /// Updating InstanceTenancy requires no replacement only if you are updating its value from "dedicated" to "default". Updating InstanceTenancy from "default" to "dedicated" requires replacement.
    #[serde(rename = "InstanceTenancy")]
    pub instance_tenancy: Option<String>,
    /// The tags for the VPC.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The netmask length of the IPv4 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool
    #[serde(rename = "Ipv4NetmaskLength")]
    pub ipv4_netmask_length: Option<usize>,
    /// The primary IPv4 CIDR block for the VPC.
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,
    /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR
    #[serde(rename = "Ipv4IpamPoolId")]
    pub ipv4_ipam_pool_id: Option<String>,
    /// Indicates whether the DNS resolution is supported for the VPC. If enabled, queries to the Amazon provided DNS server at the 169.254.169.253 IP address, or the reserved IP address at the base of the VPC network range "plus two" succeed. If disabled, the Amazon provided DNS service in the VPC that resolves public DNS hostnames to IP addresses is not enabled. Enabled by default.
    #[serde(rename = "EnableDnsSupport")]
    pub enable_dns_support: Option<bool>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_vpccidr_block {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCCidrBlock {
    /// No documentation provided by AWS
    #[serde(rename = "Ipv4NetmaskLength")]
    pub ipv4_netmask_length: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "AmazonProvidedIpv6CidrBlock")]
    pub amazon_provided_ipv6_cidr_block: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6NetmaskLength")]
    pub ipv6_netmask_length: Option<usize>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv4IpamPoolId")]
    pub ipv4_ipam_pool_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6Pool")]
    pub ipv6_pool: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Ipv6IpamPoolId")]
    pub ipv6_ipam_pool_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,

}



}

pub mod cfn_vpcdhcpoptions_association {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCDHCPOptionsAssociation {
    /// The ID of the DHCP options set, or default to associate no DHCP options with the VPC.
    #[serde(rename = "DhcpOptionsId")]
    pub dhcp_options_id: String,
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}



}

pub mod cfn_vpcendpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCEndpoint {
    /// One or more route table IDs.
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcEndpointType")]
    pub vpc_endpoint_type: Option<String>,
    /// The ID of the VPC in which the endpoint will be used.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The ID of one or more subnets in which to create an endpoint network interface.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Option<Vec<String>>,
    /// The ID of one or more security groups to associate with the endpoint network interface.
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// A policy to attach to the endpoint that controls access to the service.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<()>,
    /// Indicate whether to associate a private hosted zone with the specified VPC.
    #[serde(rename = "PrivateDnsEnabled")]
    pub private_dns_enabled: Option<bool>,
    /// The service name.
    #[serde(rename = "ServiceName")]
    pub service_name: String,

}



}

pub mod cfn_vpcendpoint_connection_notification {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCEndpointConnectionNotification {
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionEvents")]
    pub connection_events: Vec<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionNotificationArn")]
    pub connection_notification_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "VPCEndpointId")]
    pub vpcendpoint_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceId")]
    pub service_id: Option<String>,

}



}

pub mod cfn_vpcendpoint_service {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCEndpointService {
    /// No documentation provided by AWS
    #[serde(rename = "NetworkLoadBalancerArns")]
    pub network_load_balancer_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ContributorInsightsEnabled")]
    pub contributor_insights_enabled: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "PayerResponsibility")]
    pub payer_responsibility: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "GatewayLoadBalancerArns")]
    pub gateway_load_balancer_arns: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AcceptanceRequired")]
    pub acceptance_required: Option<bool>,

}



}

pub mod cfn_vpcendpoint_service_permissions {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCEndpointServicePermissions {
    /// No documentation provided by AWS
    #[serde(rename = "AllowedPrincipals")]
    pub allowed_principals: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "ServiceId")]
    pub service_id: String,

}



}

pub mod cfn_vpcgateway_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCGatewayAttachment {
    /// No documentation provided by AWS
    #[serde(rename = "InternetGatewayId")]
    pub internet_gateway_id: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// No documentation provided by AWS
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: Option<String>,

}



}

pub mod cfn_vpcpeering_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnVPCPeeringConnection {
    /// The Amazon Resource Name (ARN) of the VPC peer role for the peering connection in another AWS account.
    #[serde(rename = "PeerRoleArn")]
    pub peer_role_arn: Option<String>,
    /// The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.
    #[serde(rename = "PeerRegion")]
    pub peer_region: Option<String>,
    /// The ID of the VPC.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The AWS account ID of the owner of the accepter VPC.
    #[serde(rename = "PeerOwnerId")]
    pub peer_owner_id: Option<String>,
    /// The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: String,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_vpnconnection {

#[derive(serde::Serialize, Default)]
pub struct CfnVPNConnection {
    /// The ID of the customer gateway at your end of the VPN connection.
    #[serde(rename = "CustomerGatewayId")]
    pub customer_gateway_id: String,
    /// The ID of the virtual private gateway at the AWS side of the VPN connection.
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: Option<String>,
    /// Indicates whether the VPN connection uses static routes only.
    #[serde(rename = "StaticRoutesOnly")]
    pub static_routes_only: Option<bool>,
    /// Any tags assigned to the VPN connection.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ID of the transit gateway associated with the VPN connection.
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: Option<String>,
    /// The tunnel options for the VPN connection.
    #[serde(rename = "VpnTunnelOptionsSpecifications")]
    pub vpn_tunnel_options_specifications: Option<Vec<VpnTunnelOptionsSpecification>>,
    /// The type of VPN connection.
    #[serde(rename = "Type")]
    pub ty: String,

}


#[derive(serde::Serialize, Default)]
pub struct VpnTunnelOptionsSpecification {
    #[serde(rename = "TunnelInsideCidr")]
    pub tunnel_inside_cidr: Option<String>,
    #[serde(rename = "PreSharedKey")]
    pub pre_shared_key: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_vpnconnection_route {

#[derive(serde::Serialize, Default)]
pub struct CfnVPNConnectionRoute {
    /// The CIDR block associated with the local subnet of the customer network.
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,
    /// The ID of the VPN connection.
    #[serde(rename = "VpnConnectionId")]
    pub vpn_connection_id: String,

}



}

pub mod cfn_vpngateway {

#[derive(serde::Serialize, Default)]
pub struct CfnVPNGateway {
    /// Any tags assigned to the virtual private gateway.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The type of VPN connection the virtual private gateway supports.
    #[serde(rename = "Type")]
    pub ty: String,
    /// The private Autonomous System Number (ASN) for the Amazon side of a BGP session.
    #[serde(rename = "AmazonSideAsn")]
    pub amazon_side_asn: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_vpngateway_route_propagation {

#[derive(serde::Serialize, Default)]
pub struct CfnVPNGatewayRoutePropagation {
    /// The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to
    #[serde(rename = "RouteTableIds")]
    pub route_table_ids: Vec<String>,
    /// The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with
    #[serde(rename = "VpnGatewayId")]
    pub vpn_gateway_id: String,

}



}
