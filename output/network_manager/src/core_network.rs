/// Describes a core network.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetwork {
    ///
    /// The description of a core network.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the global network that your core network is a part of.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "GlobalNetworkId")]
    pub global_network_id: cfn_resources::StrVal,

    ///
    /// Describes a core network policy. For more information, see Core network policies.
    ///
    /// If you update the policy document, CloudFormation will apply the core network change set generated from the updated policy document, and then set it as the LIVE policy.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<serde_json::Value>,

    ///
    /// The list of key-value tags associated with a core network.
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
    pub att_core_network_arn: CfnCoreNetworkcorenetworkarn,

    #[serde(skip_serializing)]
    pub att_core_network_id: CfnCoreNetworkcorenetworkid,

    #[serde(skip_serializing)]
    pub att_created_at: CfnCoreNetworkcreatedat,

    #[serde(skip_serializing)]
    pub att_owner_account: CfnCoreNetworkowneraccount,

    #[serde(skip_serializing)]
    pub att_state: CfnCoreNetworkstate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetworkcorenetworkarn;
impl CfnCoreNetworkcorenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetworkcorenetworkid;
impl CfnCoreNetworkcorenetworkid {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetworkcreatedat;
impl CfnCoreNetworkcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetworkowneraccount;
impl CfnCoreNetworkowneraccount {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerAccount"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnCoreNetworkstate;
impl CfnCoreNetworkstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnCoreNetwork {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::CoreNetwork"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.global_network_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'global_network_id'. {} is greater than 50",
                    s.len()
                ));
            }
        }

        let the_val = &self.global_network_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'global_network_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes a core network edge.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CoreNetworkEdge {
    ///
    /// The ASN of a core network edge.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "Asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<f64>,

    ///
    /// The Region where a core network edge is located.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "EdgeLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<cfn_resources::StrVal>,

    ///
    /// The inside IP addresses used for core network edges.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InsideCidrBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inside_cidr_blocks: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CoreNetworkEdge {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.edge_location {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 63 as _ {
                    return Err(format!(
                        "Max validation failed on field 'edge_location'. {} is greater than 63",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.edge_location {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'edge_location'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Describes a core network segment, which are dedicated routes. Only attachments within this segment can communicate with each other.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CoreNetworkSegment {
    ///
    /// The Regions where the edges are located.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EdgeLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_locations: Option<Vec<String>>,

    ///
    /// The name of a core network segment.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The shared segments of a core network.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SharedSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_segments: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for CoreNetworkSegment {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 0",
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
