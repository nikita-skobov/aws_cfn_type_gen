/// Creates a service mesh.
///
/// A service mesh is a logical boundary for network traffic between services that are     represented by resources within the mesh. After you create your service mesh, you can     create virtual services, virtual nodes, virtual routers, and routes to distribute traffic     between the applications in your mesh.
///
/// For more information about service meshes, see Service meshes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnMesh {
    ///
    /// The name to use for the service mesh.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeshName")]
    pub mesh_name: Option<String>,

    ///
    /// The service mesh specification to apply.
    ///
    /// Required: No
    ///
    /// Type: MeshSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: Option<MeshSpec>,

    ///
    /// Optional metadata that you can apply to the service mesh to assist with categorization     and organization. Each tag consists of a key and an optional value, both of which you     define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnMesh {
    fn type_string(&self) -> &'static str {
        "AWS::AppMesh::Mesh"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.mesh_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'mesh_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.mesh_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'mesh_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        self.spec.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An object that represents the egress filter rules for a service mesh.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EgressFilter {
    ///
    /// The egress filter type. By default, the type is DROP_ALL, which allows     egress only from virtual nodes to other defined resources in the service mesh (and any     traffic to *.amazonaws.com for AWS API calls). You can set the     egress filter type to ALLOW_ALL to allow egress to any endpoint inside or     outside of the service mesh.
    ///
    /// NoteIf you specify any backends on a virtual node when using ALLOW_ALL, you       must specifiy all egress for that virtual node as backends. Otherwise,        ALLOW_ALL will no longer work for that virtual node.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALLOW_ALL | DROP_ALL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: EgressFilterTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum EgressFilterTypeEnum {
    /// ALLOW_ALL
    #[serde(rename = "ALLOW_ALL")]
    Allowall,

    /// DROP_ALL
    #[serde(rename = "DROP_ALL")]
    Dropall,
}

impl Default for EgressFilterTypeEnum {
    fn default() -> Self {
        EgressFilterTypeEnum::Allowall
    }
}

impl cfn_resources::CfnResource for EgressFilter {
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

/// An object that represents the service discovery information for a service mesh.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MeshServiceDiscovery {
    ///
    /// The IP version to use to control traffic within the mesh.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IPv4_ONLY | IPv4_PREFERRED | IPv6_ONLY | IPv6_PREFERRED
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpPreference")]
    pub ip_preference: Option<MeshServiceDiscoveryIpPreferenceEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum MeshServiceDiscoveryIpPreferenceEnum {
    /// IPv4_ONLY
    #[serde(rename = "IPv4_ONLY")]
    Ipv4only,

    /// IPv4_PREFERRED
    #[serde(rename = "IPv4_PREFERRED")]
    Ipv4preferred,

    /// IPv6_ONLY
    #[serde(rename = "IPv6_ONLY")]
    Ipv6only,

    /// IPv6_PREFERRED
    #[serde(rename = "IPv6_PREFERRED")]
    Ipv6preferred,
}

impl Default for MeshServiceDiscoveryIpPreferenceEnum {
    fn default() -> Self {
        MeshServiceDiscoveryIpPreferenceEnum::Ipv4only
    }
}

impl cfn_resources::CfnResource for MeshServiceDiscovery {
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

/// An object that represents the specification of a service mesh.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MeshSpec {
    ///
    /// The egress filter rules for the service mesh.
    ///
    /// Required: No
    ///
    /// Type: EgressFilter
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressFilter")]
    pub egress_filter: Option<EgressFilter>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: MeshServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceDiscovery")]
    pub service_discovery: Option<MeshServiceDiscovery>,
}

impl cfn_resources::CfnResource for MeshSpec {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.egress_filter
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.service_discovery
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

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
