

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
    fn type_string() -> &'static str {
        "AWS::AppMesh::Mesh"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    pub cfn_type: String,

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
    pub ip_preference: Option<String>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
