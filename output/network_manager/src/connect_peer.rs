

/// Creates a core network Connect peer for a specified core network connect attachment between a core network and an appliance.     The peer address and transit gateway address must be the same IP address family (IPv4 or IPv6).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectPeer {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: BgpOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "BgpOptions")]
    pub bgp_options: Option<BgpOptions>,


    /// 
    /// The IP address of the Connect peer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerAddress")]
    pub peer_address: String,


    /// 
    /// The IP address of a core network.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,


    /// 
    /// The inside IP addresses used for a Connect peer configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InsideCidrBlocks")]
    pub inside_cidr_blocks: Vec<String>,


    /// 
    /// The ID of the attachment to connect.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^attachment-([0-9a-f]{8,17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectAttachmentId")]
    pub connect_attachment_id: String,


    /// 
    /// The list of key-value tags associated with the Connect peer.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnConnectPeer {
    fn type_string() -> &'static str {
        "AWS::NetworkManager::ConnectPeer"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Describes a core network BGP configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectPeerBgpConfiguration {


    /// 
    /// The ASN of the Coret Network.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoreNetworkAsn")]
    pub core_network_asn: Option<f64>,


    /// 
    /// The address of a core network Connect peer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeerAddress")]
    pub peer_address: Option<String>,


    /// 
    /// The address of a core network.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,


    /// 
    /// The ASN of the Connect peer.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeerAsn")]
    pub peer_asn: Option<f64>,

}




/// Describes the BGP options.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BgpOptions {


    /// 
    /// The Peer ASN of the BGP.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerAsn")]
    pub peer_asn: Option<f64>,

}




/// Describes a core network Connect peer configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectPeerConfiguration {


    /// 
    /// The IP address of a core network.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoreNetworkAddress")]
    pub core_network_address: Option<String>,


    /// 
    /// The Connect peer BGP configurations.
    /// 
    /// Required: No
    ///
    /// Type: List of ConnectPeerBgpConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BgpConfigurations")]
    pub bgp_configurations: Option<Vec<ConnectPeerBgpConfiguration>>,


    /// 
    /// The protocol used for a Connect peer configuration.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GRE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<ConnectPeerConfigurationProtocolEnum>,


    /// 
    /// The inside IP addresses used for a Connect peer configuration.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsideCidrBlocks")]
    pub inside_cidr_blocks: Option<Vec<String>>,


    /// 
    /// The IP address of the Connect peer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeerAddress")]
    pub peer_address: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectPeerConfigurationProtocolEnum {

    /// GRE
    #[serde(rename = "GRE")]
    Gre,

}

impl Default for ConnectPeerConfigurationProtocolEnum {
    fn default() -> Self {
        ConnectPeerConfigurationProtocolEnum::Gre
    }
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


