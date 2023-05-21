/// Creates a virtual service within a service mesh.
///
/// A virtual service is an abstraction of a real service that is provided by a virtual node     directly or indirectly by means of a virtual router. Dependent services call your virtual     service by its virtualServiceName, and those requests are routed to the     virtual node or virtual router that is specified as the provider for the virtual     service.
///
/// For more information about virtual services, see Virtual services.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVirtualService {
    ///
    /// The name of the service mesh to create the virtual service in.
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
    pub mesh_name: String,

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
    pub mesh_owner: Option<String>,

    ///
    /// The virtual service specification to apply.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualServiceSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: VirtualServiceSpec,

    ///
    /// Optional metadata that you can apply to the virtual service to assist with     categorization and organization. Each tag consists of a key and an optional value, both of     which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
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

    ///
    /// The name to use for the virtual service.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,
}

impl cfn_resources::CfnResource for CfnVirtualService {
    fn type_string(&self) -> &'static str {
        "AWS::AppMesh::VirtualService"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.mesh_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'mesh_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.mesh_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'mesh_name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.mesh_owner {
            if the_val.len() > 12 as _ {
                return Err(format!(
                    "Max validation failed on field 'mesh_owner'. {} is greater than 12",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.mesh_owner {
            if the_val.len() < 12 as _ {
                return Err(format!(
                    "Min validation failed on field 'mesh_owner'. {} is less than 12",
                    the_val.len()
                ));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// An object that represents a virtual node service provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeServiceProvider {
    ///
    /// The name of the virtual node that is acting as a service provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualNodeName")]
    pub virtual_node_name: String,
}

impl cfn_resources::CfnResource for VirtualNodeServiceProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.virtual_node_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'virtual_node_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.virtual_node_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'virtual_node_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents a virtual node service provider.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualRouterServiceProvider {
    ///
    /// The name of the virtual router that is acting as a service provider.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualRouterName")]
    pub virtual_router_name: String,
}

impl cfn_resources::CfnResource for VirtualRouterServiceProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.virtual_router_name;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'virtual_router_name'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.virtual_router_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'virtual_router_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents the provider for a virtual service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualServiceProvider {
    ///
    /// The virtual node associated with a virtual service.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeServiceProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualNode")]
    pub virtual_node: Option<VirtualNodeServiceProvider>,

    ///
    /// The virtual router associated with a virtual service.
    ///
    /// Required: No
    ///
    /// Type: VirtualRouterServiceProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualRouter")]
    pub virtual_router: Option<VirtualRouterServiceProvider>,
}

impl cfn_resources::CfnResource for VirtualServiceProvider {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.virtual_node
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.virtual_router
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the specification of a virtual service.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualServiceSpec {
    ///
    /// The App Mesh object that is acting as the provider for a virtual service. You     can specify a single virtual node or virtual router.
    ///
    /// Required: No
    ///
    /// Type: VirtualServiceProvider
    ///
    /// Update requires: No interruption
    #[serde(rename = "Provider")]
    pub provider: Option<VirtualServiceProvider>,
}

impl cfn_resources::CfnResource for VirtualServiceSpec {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.provider
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
