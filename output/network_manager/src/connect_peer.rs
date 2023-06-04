/// Creates a core network Connect peer for a specified core network connect attachment between a core network and an appliance.     The peer address and transit gateway address must be the same IP address family (IPv4 or IPv6).
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeer {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: BgpOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "BgpOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_options: Option<BgpOptions>,

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
    pub connect_attachment_id: cfn_resources::StrVal,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<cfn_resources::StrVal>,

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
    pub peer_address: cfn_resources::StrVal,

    ///
    /// The list of key-value tags associated with the Connect peer.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_configuration_core_network_address: CfnConnectPeerconfigurationcorenetworkaddress,

    #[serde(skip_serializing)]
    pub att_configuration_peer_address: CfnConnectPeerconfigurationpeeraddress,

    #[serde(skip_serializing)]
    pub att_configuration_protocol: CfnConnectPeerconfigurationprotocol,

    #[serde(skip_serializing)]
    pub att_connect_peer_id: CfnConnectPeerconnectpeerid,

    #[serde(skip_serializing)]
    pub att_core_network_id: CfnConnectPeercorenetworkid,

    #[serde(skip_serializing)]
    pub att_created_at: CfnConnectPeercreatedat,

    #[serde(skip_serializing)]
    pub att_edge_location: CfnConnectPeeredgelocation,

    #[serde(skip_serializing)]
    pub att_state: CfnConnectPeerstate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeerconfigurationcorenetworkaddress;
impl CfnConnectPeerconfigurationcorenetworkaddress {
    pub fn att_name(&self) -> &'static str {
        r#"Configuration.CoreNetworkAddress"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeerconfigurationpeeraddress;
impl CfnConnectPeerconfigurationpeeraddress {
    pub fn att_name(&self) -> &'static str {
        r#"Configuration.PeerAddress"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeerconfigurationprotocol;
impl CfnConnectPeerconfigurationprotocol {
    pub fn att_name(&self) -> &'static str {
        r#"Configuration.Protocol"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeerconnectpeerid;
impl CfnConnectPeerconnectpeerid {
    pub fn att_name(&self) -> &'static str {
        r#"ConnectPeerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeercorenetworkid;
impl CfnConnectPeercorenetworkid {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeercreatedat;
impl CfnConnectPeercreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeeredgelocation;
impl CfnConnectPeeredgelocation {
    pub fn att_name(&self) -> &'static str {
        r#"EdgeLocation"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConnectPeerstate;
impl CfnConnectPeerstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnConnectPeer {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::ConnectPeer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.bgp_options
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.connect_attachment_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'connect_attachment_id'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.connect_attachment_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'connect_attachment_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!("Max validation failed on field 'core_network_address'. {} is greater than 50", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'core_network_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.peer_address;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'peer_address'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.peer_address;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'peer_address'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes the BGP options.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<f64>,
}

impl cfn_resources::CfnResource for BgpOptions {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Describes a core network BGP configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ConnectPeerBgpConfiguration {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<cfn_resources::StrVal>,

    ///
    /// The ASN of the Coret Network.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "CoreNetworkAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_address: Option<cfn_resources::StrVal>,

    ///
    /// The ASN of the Connect peer.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "PeerAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_asn: Option<f64>,
}

impl cfn_resources::CfnResource for ConnectPeerBgpConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!("Max validation failed on field 'core_network_address'. {} is greater than 50", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'core_network_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.peer_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!(
                        "Max validation failed on field 'peer_address'. {} is greater than 50",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.peer_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'peer_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Describes a core network Connect peer configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ConnectPeerConfiguration {
    ///
    /// The Connect peer BGP configurations.
    ///
    /// Required: No
    ///
    /// Type: List of ConnectPeerBgpConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BgpConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_configurations: Option<Vec<ConnectPeerBgpConfiguration>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_address: Option<cfn_resources::StrVal>,

    ///
    /// The inside IP addresses used for a Connect peer configuration.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_address: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ConnectPeerConfigurationProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

impl cfn_resources::CfnResource for ConnectPeerConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!("Max validation failed on field 'core_network_address'. {} is greater than 50", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.core_network_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'core_network_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.peer_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 50 as _ {
                    return Err(format!(
                        "Max validation failed on field 'peer_address'. {} is greater than 50",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.peer_address {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'peer_address'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
