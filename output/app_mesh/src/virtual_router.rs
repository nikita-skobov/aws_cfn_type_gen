/// Creates a virtual router within a service mesh.
///
/// Specify a listener for any inbound traffic that your virtual router     receives. Create a virtual router for each protocol and port that you need to route.     Virtual routers handle traffic for one or more virtual services within your mesh. After you     create your virtual router, create and associate routes for your virtual router that direct     incoming requests to different virtual nodes.
///
/// For more information about virtual routers, see Virtual routers.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVirtualRouter {
    ///
    /// The name of the service mesh to create the virtual router in.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeshName")]
    pub mesh_name: cfn_resources::StrVal,

    ///
    /// The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then        the account that you specify must share the mesh with your account before you can create        the resource in the service mesh. For more information about mesh sharing, see Working with shared meshes.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 12
    ///
    /// Maximum: 12
    ///
    /// Update requires: Replacement
    #[serde(rename = "MeshOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<cfn_resources::StrVal>,

    ///
    /// The virtual router specification to apply.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualRouterSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: VirtualRouterSpec,

    ///
    /// Optional metadata that you can apply to the virtual router to assist with categorization     and organization. Each tag consists of a key and an optional value, both of which you     define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The name to use for the virtual router.
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
    #[serde(rename = "VirtualRouterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_router_name: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_arn: CfnVirtualRouterarn,

    #[serde(skip_serializing)]
    pub att_mesh_name: CfnVirtualRoutermeshname,

    #[serde(skip_serializing)]
    pub att_mesh_owner: CfnVirtualRoutermeshowner,

    #[serde(skip_serializing)]
    pub att_resource_owner: CfnVirtualRouterresourceowner,

    #[serde(skip_serializing)]
    pub att_uid: CfnVirtualRouteruid,

    #[serde(skip_serializing)]
    pub att_virtual_router_name: CfnVirtualRoutervirtualroutername,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRouterarn;
impl CfnVirtualRouterarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRoutermeshname;
impl CfnVirtualRoutermeshname {
    pub fn att_name(&self) -> &'static str {
        r#"MeshName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRoutermeshowner;
impl CfnVirtualRoutermeshowner {
    pub fn att_name(&self) -> &'static str {
        r#"MeshOwner"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRouterresourceowner;
impl CfnVirtualRouterresourceowner {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceOwner"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRouteruid;
impl CfnVirtualRouteruid {
    pub fn att_name(&self) -> &'static str {
        r#"Uid"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVirtualRoutervirtualroutername;
impl CfnVirtualRoutervirtualroutername {
    pub fn att_name(&self) -> &'static str {
        r#"VirtualRouterName"#
    }
}

impl cfn_resources::CfnResource for CfnVirtualRouter {
    fn type_string(&self) -> &'static str {
        "AWS::AppMesh::VirtualRouter"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.mesh_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'mesh_name'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.mesh_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'mesh_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.mesh_owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 12 as _ {
                    return Err(format!(
                        "Max validation failed on field 'mesh_owner'. {} is greater than 12",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.mesh_owner {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 12 as _ {
                    return Err(format!(
                        "Min validation failed on field 'mesh_owner'. {} is less than 12",
                        s.len()
                    ));
                }
            }
        }

        self.spec.validate()?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 50",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.virtual_router_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'virtual_router_name'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.virtual_router_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'virtual_router_name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// An object representing a virtual router listener port mapping.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PortMapping {
    ///
    /// The port used for the port mapping.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: i64,

    ///
    /// The protocol used for the port mapping. Specify one protocol.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: grpc | http | http2 | tcp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: PortMappingProtocolEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PortMappingProtocolEnum {
    /// grpc
    #[serde(rename = "grpc")]
    Grpc,

    /// http
    #[serde(rename = "http")]
    Http,

    /// http2
    #[serde(rename = "http2")]
    Http2,

    /// tcp
    #[serde(rename = "tcp")]
    Tcp,
}

impl Default for PortMappingProtocolEnum {
    fn default() -> Self {
        PortMappingProtocolEnum::Grpc
    }
}

impl cfn_resources::CfnResource for PortMapping {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.port;

        if *the_val > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'port'. {} is greater than 65535",
                the_val
            ));
        }

        let the_val = &self.port;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'port'. {} is less than 1",
                the_val
            ));
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
#[serde(default)]
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

/// An object that represents a virtual router listener.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualRouterListener {
    ///
    /// The port mapping information for the listener.
    ///
    /// Required: Yes
    ///
    /// Type: PortMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortMapping")]
    pub port_mapping: PortMapping,
}

impl cfn_resources::CfnResource for VirtualRouterListener {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.port_mapping.validate()?;

        Ok(())
    }
}

/// An object that represents the specification of a virtual router.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VirtualRouterSpec {
    ///
    /// The listeners that the virtual router is expected to receive inbound traffic     from.
    ///
    /// Required: Yes
    ///
    /// Type: List of VirtualRouterListener
    ///
    /// Update requires: No interruption
    #[serde(rename = "Listeners")]
    pub listeners: Vec<VirtualRouterListener>,
}

impl cfn_resources::CfnResource for VirtualRouterSpec {
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
