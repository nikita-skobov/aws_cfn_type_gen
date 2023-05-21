/// Creates a virtual node within a service mesh.
///
/// A virtual node acts as a logical pointer to a particular task group, such as an Amazon ECS service or a Kubernetes deployment. When you create a virtual node, you can     specify the service discovery information for your task group, and whether the proxy     running in a task group will communicate with other proxies using Transport Layer Security     (TLS).
///
/// You define a listener for any inbound traffic that your virtual node     expects. Any virtual service that your virtual node expects to communicate to is specified     as a backend.
///
/// The response metadata for your new virtual node contains the arn that is     associated with the virtual node. Set this value to the full ARN; for example,       arn:aws:appmesh:us-west-2:123456789012:myMesh/default/virtualNode/myApp)     as the APPMESH_RESOURCE_ARN environment variable for your task group's Envoy     proxy container in your task definition or pod spec. This is then mapped to the       node.id and node.cluster Envoy parameters.
///
/// For more information about virtual nodes, see Virtual nodes. You must be using 1.15.0 or later of the Envoy image when     setting these variables. For more information aboutApp Mesh Envoy variables, see       Envoy       image in the AWS App Mesh User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVirtualNode {
    ///
    /// The name of the service mesh to create the virtual node in.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_owner: Option<String>,

    ///
    /// The virtual node specification to apply.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualNodeSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: VirtualNodeSpec,

    ///
    /// Optional metadata that you can apply to the virtual node to assist with categorization     and organization. Each tag consists of a key and an optional value, both of which you     define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
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
    /// The name to use for the virtual node.
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
    #[serde(rename = "VirtualNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_node_name: Option<String>,
}

impl cfn_resources::CfnResource for CfnVirtualNode {
    fn type_string(&self) -> &'static str {
        "AWS::AppMesh::VirtualNode"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
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

        if let Some(the_val) = &self.virtual_node_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'virtual_node_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.virtual_node_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'virtual_node_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An object that represents the access logging information for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLog {
    ///
    /// The file object to send virtual node access logs to.
    ///
    /// Required: No
    ///
    /// Type: FileAccessLog
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileAccessLog>,
}

impl cfn_resources::CfnResource for AccessLog {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the AWS Cloud Map attribute information for your     virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsCloudMapInstanceAttribute {
    ///
    /// The name of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service instance that contains the specified key and value is     returned.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: ^[a-zA-Z0-9!-~]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The value of an AWS Cloud Map service instance attribute key. Any AWS Cloud Map service instance that contains the specified key and value is     returned.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^([a-zA-Z0-9!-~][ a-zA-Z0-9!-~]*){0,1}[a-zA-Z0-9!-~]{0,1}$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for AwsCloudMapInstanceAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'key'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'key'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents the AWS Cloud Map service discovery information for     your virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsCloudMapServiceDiscovery {
    ///
    /// A string map that contains attributes with values that you can use to filter instances     by any custom attribute that you specified when you registered the instance. Only instances     that match all of the specified key/value pairs will be returned.
    ///
    /// Required: No
    ///
    /// Type: List of AwsCloudMapInstanceAttribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AwsCloudMapInstanceAttribute>>,

    ///
    /// The preferred IP version that this virtual node uses. Setting the IP preference on the     virtual node only overrides the IP preference set for the mesh on this specific     node.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IPv4_ONLY | IPv4_PREFERRED | IPv6_ONLY | IPv6_PREFERRED
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_preference: Option<AwsCloudMapServiceDiscoveryIpPreferenceEnum>,

    ///
    /// The HTTP name of the AWS Cloud Map namespace to use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,

    ///
    /// The name of the AWS Cloud Map service to use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum AwsCloudMapServiceDiscoveryIpPreferenceEnum {
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

impl Default for AwsCloudMapServiceDiscoveryIpPreferenceEnum {
    fn default() -> Self {
        AwsCloudMapServiceDiscoveryIpPreferenceEnum::Ipv4only
    }
}

impl cfn_resources::CfnResource for AwsCloudMapServiceDiscovery {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.namespace_name;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'namespace_name'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.namespace_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'namespace_name'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.service_name;

        if the_val.len() > 1024 as _ {
            return Err(format!(
                "Max validation failed on field 'service_name'. {} is greater than 1024",
                the_val.len()
            ));
        }

        let the_val = &self.service_name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'service_name'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents the backends that a virtual node is expected to send outbound     traffic to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Backend {
    ///
    /// Specifies a virtual service to use as a backend.
    ///
    /// Required: No
    ///
    /// Type: VirtualServiceBackend
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_service: Option<VirtualServiceBackend>,
}

impl cfn_resources::CfnResource for Backend {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.virtual_service
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the default properties for a backend.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BackendDefaults {
    ///
    /// A reference to an object that represents a client policy.
    ///
    /// Required: No
    ///
    /// Type: ClientPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,
}

impl cfn_resources::CfnResource for BackendDefaults {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.client_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a client policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientPolicy {
    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) client policy.
    ///
    /// Required: No
    ///
    /// Type: ClientPolicyTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClientPolicyTls>,
}

impl cfn_resources::CfnResource for ClientPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.tls.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// A reference to an object that represents a Transport Layer Security (TLS) client policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientPolicyTls {
    ///
    /// A reference to an object that represents a client's TLS certificate.
    ///
    /// Required: No
    ///
    /// Type: ClientTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ClientTlsCertificate>,

    ///
    /// Whether the policy is enforced. The default is True, if a value isn't specified.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,

    ///
    /// One or more ports that the policy is enforced for.
    ///
    /// Required: No
    ///
    /// Type: List of Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,

    ///
    /// A reference to an object that represents a TLS validation context.
    ///
    /// Required: Yes
    ///
    /// Type: TlsValidationContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validation")]
    pub validation: TlsValidationContext,
}

impl cfn_resources::CfnResource for ClientPolicyTls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.certificate
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.validation.validate()?;

        Ok(())
    }
}

/// An object that represents the client's certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientTlsCertificate {
    ///
    /// An object that represents a local file certificate. The certificate must meet specific     requirements and you must have proxy authorization enabled. For more information, see       Transport Layer Security (TLS).
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsFileCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ListenerTlsFileCertificate>,

    ///
    /// A reference to an object that represents a client's TLS Secret Discovery Service     certificate.
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsSdsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<ListenerTlsSdsCertificate>,
}

impl cfn_resources::CfnResource for ClientTlsCertificate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sds.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the DNS service discovery information for your virtual     node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DnsServiceDiscovery {
    ///
    /// Specifies the DNS service discovery hostname for the virtual node.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: String,

    ///
    /// The preferred IP version that this virtual node uses. Setting the IP preference on the     virtual node only overrides the IP preference set for the mesh on this specific     node.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: IPv4_ONLY | IPv4_PREFERRED | IPv6_ONLY | IPv6_PREFERRED
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_preference: Option<DnsServiceDiscoveryIpPreferenceEnum>,

    ///
    /// Specifies the DNS response type for the virtual node.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ENDPOINTS | LOADBALANCER
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResponseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<DnsServiceDiscoveryResponseTypeEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DnsServiceDiscoveryIpPreferenceEnum {
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

impl Default for DnsServiceDiscoveryIpPreferenceEnum {
    fn default() -> Self {
        DnsServiceDiscoveryIpPreferenceEnum::Ipv4only
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DnsServiceDiscoveryResponseTypeEnum {
    /// ENDPOINTS
    #[serde(rename = "ENDPOINTS")]
    Endpoints,

    /// LOADBALANCER
    #[serde(rename = "LOADBALANCER")]
    Loadbalancer,
}

impl Default for DnsServiceDiscoveryResponseTypeEnum {
    fn default() -> Self {
        DnsServiceDiscoveryResponseTypeEnum::Endpoints
    }
}

impl cfn_resources::CfnResource for DnsServiceDiscovery {
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

/// An object that represents a duration of time.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Duration {
    ///
    /// A unit of time.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ms | s
    ///
    /// Update requires: No interruption
    #[serde(rename = "Unit")]
    pub unit: DurationUnitEnum,

    ///
    /// A number of time units.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DurationUnitEnum {
    /// ms
    #[serde(rename = "ms")]
    Ms,

    /// s
    #[serde(rename = "s")]
    S,
}

impl Default for DurationUnitEnum {
    fn default() -> Self {
        DurationUnitEnum::Ms
    }
}

impl cfn_resources::CfnResource for Duration {
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

/// An object that represents an access log file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileAccessLog {
    ///
    /// The specified format for the logs. The format is either json_format or       text_format.
    ///
    /// Required: No
    ///
    /// Type: LoggingFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<LoggingFormat>,

    ///
    /// The file path to write access logs to. You can use /dev/stdout to send     access logs to standard out and configure your Envoy container to use a log driver, such as       awslogs, to export the access logs to a log storage service such as Amazon     CloudWatch Logs. You can also specify a path in the Envoy container's file system to write     the files to disk.
    ///
    /// NoteThe Envoy process must have write permissions to the path that you specify here.       Otherwise, Envoy fails to bootstrap properly.
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
    #[serde(rename = "Path")]
    pub path: String,
}

impl cfn_resources::CfnResource for FileAccessLog {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.format.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.path;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'path'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.path;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'path'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents types of timeouts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcTimeout {
    ///
    /// An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.
    ///
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,

    ///
    /// An object that represents a per request timeout. The default value is 15 seconds. If you set a higher timeout, then make sure that the higher value is set for each App Mesh                  resource in a conversation. For example, if a virtual node backend uses a virtual router provider to route to another virtual node, then the timeout should be greater than 15                  seconds for the source and destination virtual node and the route.
    ///
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

impl cfn_resources::CfnResource for GrpcTimeout {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.per_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the health check policy for a virtual node's listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheck {
    ///
    /// The number of consecutive successful health checks that must occur before declaring     listener healthy.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: i64,

    ///
    /// The time period in milliseconds between each health check execution.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntervalMillis")]
    pub interval_millis: i64,

    ///
    /// The destination path for the health check request. This value is only used if the     specified protocol is HTTP or HTTP/2. For any other protocol, this value is ignored.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    ///
    /// The destination port for the health check request. This port must match the port defined     in the PortMapping for the listener.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    ///
    /// The protocol for the health check request. If you specify grpc, then your     service must conform to the GRPC Health       Checking Protocol.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: grpc | http | http2 | tcp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: HealthCheckProtocolEnum,

    ///
    /// The amount of time to wait when receiving a response from the health check, in     milliseconds.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutMillis")]
    pub timeout_millis: i64,

    ///
    /// The number of consecutive failed health checks that must occur before declaring a     virtual node unhealthy.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum HealthCheckProtocolEnum {
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

impl Default for HealthCheckProtocolEnum {
    fn default() -> Self {
        HealthCheckProtocolEnum::Grpc
    }
}

impl cfn_resources::CfnResource for HealthCheck {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.healthy_threshold;

        if *the_val > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'healthy_threshold'. {} is greater than 10",
                the_val
            ));
        }

        let the_val = &self.healthy_threshold;

        if *the_val < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'healthy_threshold'. {} is less than 2",
                the_val
            ));
        }

        if let Some(the_val) = &self.port {
            if *the_val > 65535 as _ {
                return Err(format!(
                    "Max validation failed on field 'port'. {} is greater than 65535",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.port {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'port'. {} is less than 1",
                    the_val
                ));
            }
        }

        let the_val = &self.unhealthy_threshold;

        if *the_val > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'unhealthy_threshold'. {} is greater than 10",
                the_val
            ));
        }

        let the_val = &self.unhealthy_threshold;

        if *the_val < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'unhealthy_threshold'. {} is less than 2",
                the_val
            ));
        }

        Ok(())
    }
}

/// An object that represents types of timeouts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpTimeout {
    ///
    /// An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.
    ///
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,

    ///
    /// An object that represents a per request timeout. The default value is 15 seconds. If you set a higher timeout, then make sure that the higher value is set for each App Mesh                  resource in a conversation. For example, if a virtual node backend uses a virtual router provider to route to another virtual node, then the timeout should be greater than 15                  seconds for the source and destination virtual node and the route.
    ///
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_request: Option<Duration>,
}

impl cfn_resources::CfnResource for HttpTimeout {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.per_request
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the key value pairs for the JSON.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonFormatRef {
    ///
    /// The specified key for the JSON.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: String,

    ///
    /// The specified value for the JSON.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,
}

impl cfn_resources::CfnResource for JsonFormatRef {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'key'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'key'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'value'. {} is greater than 100",
                the_val.len()
            ));
        }

        let the_val = &self.value;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'value'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents a listener for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Listener {
    ///
    /// The connection pool information for the listener.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool: Option<VirtualNodeConnectionPool>,

    ///
    /// The health check information for the listener.
    ///
    /// Required: No
    ///
    /// Type: HealthCheck
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,

    ///
    /// The outlier detection information for the listener.
    ///
    /// Required: No
    ///
    /// Type: OutlierDetection
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlierDetection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlier_detection: Option<OutlierDetection>,

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

    ///
    /// A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.
    ///
    /// Required: No
    ///
    /// Type: ListenerTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<ListenerTls>,

    ///
    /// An object that represents timeouts for different protocols.
    ///
    /// Required: No
    ///
    /// Type: ListenerTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ListenerTimeout>,
}

impl cfn_resources::CfnResource for Listener {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.connection_pool
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.health_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.outlier_detection
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.port_mapping.validate()?;

        self.tls.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.timeout.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents timeouts for different protocols.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTimeout {
    ///
    /// An object that represents types of timeouts.
    ///
    /// Required: No
    ///
    /// Type: GrpcTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "GRPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GrpcTimeout>,

    ///
    /// An object that represents types of timeouts.
    ///
    /// Required: No
    ///
    /// Type: HttpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpTimeout>,

    ///
    /// An object that represents types of timeouts.
    ///
    /// Required: No
    ///
    /// Type: HttpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<HttpTimeout>,

    ///
    /// An object that represents types of timeouts.
    ///
    /// Required: No
    ///
    /// Type: TcpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "TCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpTimeout>,
}

impl cfn_resources::CfnResource for ListenerTimeout {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.grpc.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http2.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.tcp.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the Transport Layer Security (TLS) properties for a listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTls {
    ///
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) certificate.
    ///
    /// Required: Yes
    ///
    /// Type: ListenerTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: ListenerTlsCertificate,

    ///
    /// Specify one of the following modes.
    ///
    /// STRICT – Listener only accepts connections with TLS        enabled.            PERMISSIVE – Listener accepts connections with or        without TLS enabled.            DISABLED – Listener only accepts connections without        TLS.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | PERMISSIVE | STRICT
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: ListenerTlsModeEnum,

    ///
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) validation context.
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsValidationContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ListenerTlsValidationContext>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ListenerTlsModeEnum {
    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// PERMISSIVE
    #[serde(rename = "PERMISSIVE")]
    Permissive,

    /// STRICT
    #[serde(rename = "STRICT")]
    Strict,
}

impl Default for ListenerTlsModeEnum {
    fn default() -> Self {
        ListenerTlsModeEnum::Disabled
    }
}

impl cfn_resources::CfnResource for ListenerTls {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.certificate.validate()?;

        self.validation
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents an AWS Certificate Manager certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsAcmCertificate {
    ///
    /// The Amazon Resource Name (ARN) for the certificate. The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see Transport Layer Security (TLS).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

impl cfn_resources::CfnResource for ListenerTlsAcmCertificate {
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

/// An object that represents a listener's Transport Layer Security (TLS) certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsCertificate {
    ///
    /// A reference to an object that represents an AWS Certificate Manager certificate.
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsAcmCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<ListenerTlsAcmCertificate>,

    ///
    /// A reference to an object that represents a local file certificate.
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsFileCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ListenerTlsFileCertificate>,

    ///
    /// A reference to an object that represents a listener's Secret Discovery Service     certificate.
    ///
    /// Required: No
    ///
    /// Type: ListenerTlsSdsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<ListenerTlsSdsCertificate>,
}

impl cfn_resources::CfnResource for ListenerTlsCertificate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acm.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.file.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sds.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a local file certificate.     The certificate must meet specific requirements and you must have proxy authorization enabled. For more information, see Transport Layer Security (TLS).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsFileCertificate {
    ///
    /// The certificate chain for the certificate.
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
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,

    ///
    /// The private key for a certificate stored on the file system of the virtual node that the     proxy is running on.
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
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
}

impl cfn_resources::CfnResource for ListenerTlsFileCertificate {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.certificate_chain;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'certificate_chain'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.certificate_chain;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'certificate_chain'. {} is less than 1",
                the_val.len()
            ));
        }

        let the_val = &self.private_key;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'private_key'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.private_key;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'private_key'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents the listener's Secret Discovery Service certificate. The proxy     must be configured with a local SDS provider via a Unix Domain Socket. See App Mesh     TLS       documentation for more info.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsSdsCertificate {
    ///
    /// A reference to an object that represents the name of the secret requested from the     Secret Discovery Service provider representing Transport Layer Security (TLS) materials like a certificate or     certificate chain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl cfn_resources::CfnResource for ListenerTlsSdsCertificate {
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

/// An object that represents a listener's Transport Layer Security (TLS) validation context.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsValidationContext {
    ///
    /// A reference to an object that represents the SANs for a listener's Transport Layer Security (TLS) validation     context.
    ///
    /// Required: No
    ///
    /// Type: SubjectAlternativeNames
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

    ///
    /// A reference to where to retrieve the trust chain when validating a peer’s Transport Layer Security (TLS)     certificate.
    ///
    /// Required: Yes
    ///
    /// Type: ListenerTlsValidationContextTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trust")]
    pub trust: ListenerTlsValidationContextTrust,
}

impl cfn_resources::CfnResource for ListenerTlsValidationContext {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.subject_alternative_names
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.trust.validate()?;

        Ok(())
    }
}

/// An object that represents a listener's Transport Layer Security (TLS) validation context trust.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsValidationContextTrust {
    ///
    /// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
    ///
    /// Required: No
    ///
    /// Type: TlsValidationContextFileTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TlsValidationContextFileTrust>,

    ///
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) Secret Discovery Service     validation context trust.
    ///
    /// Required: No
    ///
    /// Type: TlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<TlsValidationContextSdsTrust>,
}

impl cfn_resources::CfnResource for ListenerTlsValidationContextTrust {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.file.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sds.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the logging information for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Logging {
    ///
    /// The access log configuration for a virtual node.
    ///
    /// Required: No
    ///
    /// Type: AccessLog
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AccessLog>,
}

impl cfn_resources::CfnResource for Logging {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_log
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the format for the logs.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LoggingFormat {
    ///
    /// The logging format for JSON.
    ///
    /// Required: No
    ///
    /// Type: List of JsonFormatRef
    ///
    /// Update requires: No interruption
    #[serde(rename = "Json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<JsonFormatRef>>,

    ///
    /// The logging format for text.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl cfn_resources::CfnResource for LoggingFormat {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.text {
            if the_val.len() > 1000 as _ {
                return Err(format!(
                    "Max validation failed on field 'text'. {} is greater than 1000",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.text {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'text'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An object that represents the outlier detection for a virtual node's listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutlierDetection {
    ///
    /// The base amount of time for which a host is ejected.
    ///
    /// Required: Yes
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseEjectionDuration")]
    pub base_ejection_duration: Duration,

    ///
    /// The time interval between ejection sweep analysis.
    ///
    /// Required: Yes
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Duration,

    ///
    /// Maximum percentage of hosts in load balancing pool for upstream service that can be     ejected. Will eject at least one host regardless of the value.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxEjectionPercent")]
    pub max_ejection_percent: i64,

    ///
    /// Number of consecutive 5xx errors required for ejection.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxServerErrors")]
    pub max_server_errors: i64,
}

impl cfn_resources::CfnResource for OutlierDetection {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.base_ejection_duration.validate()?;

        self.interval.validate()?;

        let the_val = &self.max_ejection_percent;

        if *the_val > 100 as _ {
            return Err(format!(
                "Max validation failed on field 'max_ejection_percent'. {} is greater than 100",
                the_val
            ));
        }

        let the_val = &self.max_ejection_percent;

        if *the_val < 0 as _ {
            return Err(format!(
                "Min validation failed on field 'max_ejection_percent'. {} is less than 0",
                the_val
            ));
        }

        Ok(())
    }
}

/// An object representing a virtual node or virtual router listener port mapping.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    /// The protocol used for the port mapping. Specify http, http2,       grpc, or tcp.
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

#[derive(Clone, Debug, serde::Serialize)]
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

/// An object that represents the service discovery information for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceDiscovery {
    ///
    /// Specifies any AWS Cloud Map information for the virtual node.
    ///
    /// Required: No
    ///
    /// Type: AwsCloudMapServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "AWSCloudMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awscloud_map: Option<AwsCloudMapServiceDiscovery>,

    ///
    /// Specifies the DNS information for the virtual node.
    ///
    /// Required: No
    ///
    /// Type: DnsServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "DNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsServiceDiscovery>,
}

impl cfn_resources::CfnResource for ServiceDiscovery {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.awscloud_map
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.dns.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the methods by which a subject alternative name on a peer     Transport Layer Security (TLS) certificate can be matched.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SubjectAlternativeNameMatchers {
    ///
    /// The values sent must match the specified values exactly.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for SubjectAlternativeNameMatchers {
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

/// An object that represents the subject alternative names secured by the     certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SubjectAlternativeNames {
    ///
    /// An object that represents the criteria for determining a SANs match.
    ///
    /// Required: Yes
    ///
    /// Type: SubjectAlternativeNameMatchers
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: SubjectAlternativeNameMatchers,
}

impl cfn_resources::CfnResource for SubjectAlternativeNames {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.cfn_match.validate()?;

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

/// An object that represents types of timeouts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TcpTimeout {
    ///
    /// An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.
    ///
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<Duration>,
}

impl cfn_resources::CfnResource for TcpTimeout {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents how the proxy will validate its peer during Transport Layer Security (TLS)     negotiation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContext {
    ///
    /// A reference to an object that represents the SANs for a Transport Layer Security (TLS) validation context. If you     don't specify SANs on the terminating mesh endpoint, the Envoy proxy     for that node doesn't verify the SAN on a peer client certificate. If you don't specify     SANs on the originating mesh endpoint, the SAN on the certificate     provided by the terminating endpoint must match the mesh endpoint service discovery     configuration. Since SPIRE vended certificates have a SPIFFE ID as a name, you must set the     SAN since the name doesn't match the service discovery name.
    ///
    /// Required: No
    ///
    /// Type: SubjectAlternativeNames
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

    ///
    /// A reference to where to retrieve the trust chain when validating a peer’s Transport Layer Security (TLS)     certificate.
    ///
    /// Required: Yes
    ///
    /// Type: TlsValidationContextTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trust")]
    pub trust: TlsValidationContextTrust,
}

impl cfn_resources::CfnResource for TlsValidationContext {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.subject_alternative_names
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.trust.validate()?;

        Ok(())
    }
}

/// An object that represents a Transport Layer Security (TLS) validation context trust for an AWS Certificate Manager     certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContextAcmTrust {
    ///
    /// One or more ACM Amazon Resource Name (ARN)s.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Maximum: 3
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateAuthorityArns")]
    pub certificate_authority_arns: Vec<String>,
}

impl cfn_resources::CfnResource for TlsValidationContextAcmTrust {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.certificate_authority_arns;

        if the_val.len() > 3 as _ {
            return Err(format!(
                "Max validation failed on field 'certificate_authority_arns'. {} is greater than 3",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContextFileTrust {
    ///
    /// The certificate trust chain for a certificate stored on the file system of the virtual     node that the proxy is running on.
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
    #[serde(rename = "CertificateChain")]
    pub certificate_chain: String,
}

impl cfn_resources::CfnResource for TlsValidationContextFileTrust {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.certificate_chain;

        if the_val.len() > 255 as _ {
            return Err(format!(
                "Max validation failed on field 'certificate_chain'. {} is greater than 255",
                the_val.len()
            ));
        }

        let the_val = &self.certificate_chain;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'certificate_chain'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// An object that represents a Transport Layer Security (TLS) Secret Discovery Service validation context trust. The     proxy must be configured with a local SDS provider via a Unix Domain Socket. See App Mesh     TLS       documentation for more info.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContextSdsTrust {
    ///
    /// A reference to an object that represents the name of the secret for a Transport Layer Security (TLS) Secret     Discovery Service validation context trust.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl cfn_resources::CfnResource for TlsValidationContextSdsTrust {
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

/// An object that represents a Transport Layer Security (TLS) validation context trust.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContextTrust {
    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) validation context trust for an AWS Certificate Manager certificate.
    ///
    /// Required: No
    ///
    /// Type: TlsValidationContextAcmTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm: Option<TlsValidationContextAcmTrust>,

    ///
    /// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
    ///
    /// Required: No
    ///
    /// Type: TlsValidationContextFileTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TlsValidationContextFileTrust>,

    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) Secret Discovery Service validation     context trust.
    ///
    /// Required: No
    ///
    /// Type: TlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sds: Option<TlsValidationContextSdsTrust>,
}

impl cfn_resources::CfnResource for TlsValidationContextTrust {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acm.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.file.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.sds.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the type of virtual node connection pool.
///
/// Only one protocol is used at a time and should be the same protocol as the one chosen     under port mapping.
///
/// If not present the default value for maxPendingRequests is       2147483647.
///
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeConnectionPool {
    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeGrpcConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "GRPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualNodeGrpcConnectionPool>,

    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeHttpConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualNodeHttpConnectionPool>,

    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeHttp2ConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<VirtualNodeHttp2ConnectionPool>,

    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualNodeTcpConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "TCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp: Option<VirtualNodeTcpConnectionPool>,
}

impl cfn_resources::CfnResource for VirtualNodeConnectionPool {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.grpc.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http2.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.tcp.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeGrpcConnectionPool {
    ///
    /// Maximum number of inflight requests Envoy can concurrently support across hosts in     upstream cluster.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRequests")]
    pub max_requests: i64,
}

impl cfn_resources::CfnResource for VirtualNodeGrpcConnectionPool {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.max_requests;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'max_requests'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeHttp2ConnectionPool {
    ///
    /// Maximum number of inflight requests Envoy can concurrently support across hosts in     upstream cluster.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRequests")]
    pub max_requests: i64,
}

impl cfn_resources::CfnResource for VirtualNodeHttp2ConnectionPool {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.max_requests;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'max_requests'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeHttpConnectionPool {
    ///
    /// Maximum number of outbound TCP connections Envoy can establish concurrently with all     hosts in upstream cluster.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConnections")]
    pub max_connections: i64,

    ///
    /// Number of overflowing requests after max_connections Envoy will queue to     upstream cluster.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxPendingRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pending_requests: Option<i64>,
}

impl cfn_resources::CfnResource for VirtualNodeHttpConnectionPool {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.max_connections;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'max_connections'. {} is less than 1",
                the_val
            ));
        }

        if let Some(the_val) = &self.max_pending_requests {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'max_pending_requests'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// An object that represents the specification of a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeSpec {
    ///
    /// A reference to an object that represents the defaults for backends.
    ///
    /// Required: No
    ///
    /// Type: BackendDefaults
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackendDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_defaults: Option<BackendDefaults>,

    ///
    /// The backends that the virtual node is expected to send outbound traffic to.
    ///
    /// ImportantApp Mesh doesn't validate the existence of those virtual services specified in       backends. This is to prevent a cyclic dependency between virtual nodes and virtual       services creation. Make sure the virtual service name is correct. The virtual service       can be created afterwards if it doesn't already exist.
    ///
    /// Required: No
    ///
    /// Type: List of Backend
    ///
    /// Update requires: No interruption
    #[serde(rename = "Backends")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<Backend>>,

    ///
    /// The listener that the virtual node is expected to receive inbound traffic from. You can     specify one listener.
    ///
    /// Required: No
    ///
    /// Type: List of Listener
    ///
    /// Update requires: No interruption
    #[serde(rename = "Listeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,

    ///
    /// The inbound and outbound access logging information for the virtual node.
    ///
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,

    ///
    /// The service discovery information for the virtual node. If your virtual node does not     expect ingress traffic, you can omit this parameter. If you specify a     listener, then you must specify service discovery information.
    ///
    /// Required: No
    ///
    /// Type: ServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery: Option<ServiceDiscovery>,
}

impl cfn_resources::CfnResource for VirtualNodeSpec {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.backend_defaults
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.logging.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.service_discovery
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeTcpConnectionPool {
    ///
    /// Maximum number of outbound TCP connections Envoy can establish concurrently with all     hosts in upstream cluster.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxConnections")]
    pub max_connections: i64,
}

impl cfn_resources::CfnResource for VirtualNodeTcpConnectionPool {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.max_connections;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'max_connections'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// An object that represents a virtual service backend for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualServiceBackend {
    ///
    /// A reference to an object that represents the client policy for a backend.
    ///
    /// Required: No
    ///
    /// Type: ClientPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_policy: Option<ClientPolicy>,

    ///
    /// The name of the virtual service that is acting as a virtual node backend.
    ///
    /// ImportantApp Mesh doesn't validate the existence of those virtual services specified in       backends. This is to prevent a cyclic dependency between virtual nodes and virtual       services creation. Make sure the virtual service name is correct. The virtual service       can be created afterwards if it doesn't already exist.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,
}

impl cfn_resources::CfnResource for VirtualServiceBackend {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.client_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
