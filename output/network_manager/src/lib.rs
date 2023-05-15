
pub mod cfn_connect_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnConnectAttachment {
    /// Id of transport attachment
    #[serde(rename = "TransportAttachmentId")]
    pub transport_attachment_id: String,
    /// Protocol options for connect attachment
    #[serde(rename = "Options")]
    pub options: ConnectAttachmentOptions,
    /// Edge location of the attachment.
    #[serde(rename = "EdgeLocation")]
    pub edge_location: String,
    /// ID of the CoreNetwork that the attachment will be attached to.
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: String,
    /// Tags for the attachment.
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

#[derive(serde::Serialize, Default)]
pub struct ConnectAttachmentOptions {
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct ProposedSegmentChange {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "SegmentName")]
    pub segment_name: Option<String>,
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    pub attachment_policy_rule_number: Option<usize>,

}


}

pub mod cfn_connect_peer {

#[derive(serde::Serialize, Default)]
pub struct CfnConnectPeer {
    /// The IP address of the Connect peer.
    #[serde(rename = "PeerAddress")]
    pub peer_address: String,
    /// The ID of the attachment to connect.
    #[serde(rename = "ConnectAttachmentId")]
    pub connect_attachment_id: String,
    /// Bgp options for connect peer.
    #[serde(rename = "BgpOptions")]
    pub bgp_options: Option<BgpOptions>,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The inside IP addresses used for a Connect peer configuration.
    #[serde(rename = "InsideCidrBlocks")]
    pub inside_cidr_blocks: Vec<String>,
    /// The IP address of a core network.
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectPeerBgpConfiguration {
    #[serde(rename = "PeerAsn")]
    pub peer_asn: Option<f64>,
    #[serde(rename = "CoreNetworkAsn")]
    pub core_network_asn: Option<f64>,
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,
    #[serde(rename = "PeerAddress")]
    pub peer_address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectPeerConfiguration {
    #[serde(rename = "BgpConfigurations")]
    pub bgp_configurations: Option<Vec<ConnectPeerBgpConfiguration>>,
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,
    #[serde(rename = "Protocol")]
    pub protocol: Option<TunnelProtocol>,
    #[serde(rename = "PeerAddress")]
    pub peer_address: Option<String>,
    #[serde(rename = "InsideCidrBlocks")]
    pub inside_cidr_blocks: Option<Vec<String>>,

}
pub type TunnelProtocol = String;
#[derive(serde::Serialize, Default)]
pub struct BgpOptions {
    #[serde(rename = "PeerAsn")]
    pub peer_asn: Option<f64>,

}


}

pub mod cfn_core_network {

#[derive(serde::Serialize, Default)]
pub struct CfnCoreNetwork {
    /// The ID of the global network that your core network is a part of.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// Live policy document for the core network, you must provide PolicyDocument in Json Format
    #[serde(rename = "PolicyDocument")]
    pub policy_document: Option<()>,
    /// The tags for the global network.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The description of core network
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

#[derive(serde::Serialize, Default)]
pub struct CoreNetworkSegment {
    #[serde(rename = "SharedSegments")]
    pub shared_segments: Option<Vec<String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "EdgeLocations")]
    pub edge_locations: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct CoreNetworkEdge {
    #[serde(rename = "Asn")]
    pub asn: Option<f64>,
    #[serde(rename = "InsideCidrBlocks")]
    pub inside_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "EdgeLocation")]
    pub edge_location: Option<String>,

}


}

pub mod cfn_customer_gateway_association {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomerGatewayAssociation {
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The ID of the device
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// The ID of the link
    #[serde(rename = "LinkId")]
    pub link_id: Option<String>,
    /// The Amazon Resource Name (ARN) of the customer gateway.
    #[serde(rename = "CustomerGatewayArn")]
    pub customer_gateway_arn: String,

}



}

pub mod cfn_device {

#[derive(serde::Serialize, Default)]
pub struct CfnDevice {
    /// The device type.
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// The device serial number.
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,
    /// The tags for the device.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The site location.
    #[serde(rename = "Location")]
    pub location: Option<Location>,
    /// The description of the device.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The device vendor.
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,
    /// The site ID.
    #[serde(rename = "SiteId")]
    pub site_id: Option<String>,
    /// The device model
    #[serde(rename = "Model")]
    pub model: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "Longitude")]
    pub longitude: Option<String>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_global_network {

#[derive(serde::Serialize, Default)]
pub struct CfnGlobalNetwork {
    /// The tags for the global network.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The description of the global network.
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_link {

#[derive(serde::Serialize, Default)]
pub struct CfnLink {
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The description of the link.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The Bandwidth for the link.
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Bandwidth,
    /// The provider of the link.
    #[serde(rename = "Provider")]
    pub provider: Option<String>,
    /// The ID of the site
    #[serde(rename = "SiteId")]
    pub site_id: String,
    /// The type of the link.
    #[serde(rename = "Type")]
    pub ty: Option<String>,
    /// The tags for the link.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct Bandwidth {
    #[serde(rename = "DownloadSpeed")]
    pub download_speed: Option<usize>,
    #[serde(rename = "UploadSpeed")]
    pub upload_speed: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


}

pub mod cfn_link_association {

#[derive(serde::Serialize, Default)]
pub struct CfnLinkAssociation {
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The ID of the link
    #[serde(rename = "LinkId")]
    pub link_id: String,
    /// The ID of the device
    #[serde(rename = "DeviceId")]
    pub device_id: String,

}



}

pub mod cfn_site {

#[derive(serde::Serialize, Default)]
pub struct CfnSite {
    /// The tags for the site.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The description of the site.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The location of the site.
    #[serde(rename = "Location")]
    pub location: Option<Location>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Location {
    #[serde(rename = "Longitude")]
    pub longitude: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<String>,

}


}

pub mod cfn_site_to_site_vpn_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnSiteToSiteVpnAttachment {
    /// The ID of a core network where you're creating a site-to-site VPN attachment.
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: String,
    /// The ARN of the site-to-site VPN attachment.
    #[serde(rename = "VpnConnectionArn")]
    pub vpn_connection_arn: String,
    /// Tags for the attachment.
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

#[derive(serde::Serialize, Default)]
pub struct ProposedSegmentChange {
    #[serde(rename = "SegmentName")]
    pub segment_name: Option<String>,
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    pub attachment_policy_rule_number: Option<usize>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


}

pub mod cfn_transit_gateway_peering {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayPeering {
    /// The Id of the core network that you want to peer a transit gateway to.
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The ARN (Amazon Resource Name) of the transit gateway that you will peer to a core network
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}

pub mod cfn_transit_gateway_registration {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRegistration {
    /// The ID of the global network.
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: String,
    /// The Amazon Resource Name (ARN) of the transit gateway.
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: String,

}



}

pub mod cfn_transit_gateway_route_table_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnTransitGatewayRouteTableAttachment {
    /// The Arn of transit gateway route table.
    #[serde(rename = "TransitGatewayRouteTableArn")]
    pub transit_gateway_route_table_arn: String,
    /// An array of key-value pairs to apply to this resource.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The attachment to move from one segment to another.
    #[serde(rename = "ProposedSegmentChange")]
    pub proposed_segment_change: Option<ProposedSegmentChange>,
    /// The Id of peering between transit gateway and core network.
    #[serde(rename = "PeeringId")]
    pub peering_id: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct ProposedSegmentChange {
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "SegmentName")]
    pub segment_name: Option<String>,
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    pub attachment_policy_rule_number: Option<usize>,

}


}

pub mod cfn_vpc_attachment {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcAttachment {
    /// The ID of a core network for the VPC attachment.
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: String,
    /// Tags for the attachment.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// Subnet Arn list
    #[serde(rename = "SubnetArns")]
    pub subnet_arns: Vec<String>,
    /// Vpc options of the attachment.
    #[serde(rename = "Options")]
    pub options: Option<VpcOptions>,
    /// The ARN of the VPC.
    #[serde(rename = "VpcArn")]
    pub vpc_arn: String,

}


#[derive(serde::Serialize, Default)]
pub struct VpcOptions {
    #[serde(rename = "Ipv6Support")]
    pub ipv6_support: Option<bool>,
    #[serde(rename = "ApplianceModeSupport")]
    pub appliance_mode_support: Option<bool>,

}

#[derive(serde::Serialize, Default)]
pub struct ProposedSegmentChange {
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    pub attachment_policy_rule_number: Option<usize>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "SegmentName")]
    pub segment_name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
