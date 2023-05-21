/// Creates a virtual gateway.
///
/// A virtual gateway allows resources outside your mesh to communicate to resources that     are inside your mesh. The virtual gateway represents an Envoy proxy running in an Amazon ECS task, in a Kubernetes service, or on an Amazon EC2 instance. Unlike a     virtual node, which represents an Envoy running with an application, a virtual gateway     represents Envoy deployed by itself.
///
/// For more information about virtual gateways, see Virtual gateways.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVirtualGateway {
    ///
    /// The name of the service mesh that the virtual gateway resides in.
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
    /// The AWS IAM account ID of the service mesh owner. If the account ID is not your own, then it's        the ID of the account that shared the mesh with your account. For more information about mesh sharing, see Working with shared meshes.
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
    /// The specifications of the virtual gateway.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewaySpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: VirtualGatewaySpec,

    ///
    /// Optional metadata that you can apply to the virtual gateway to assist with     categorization and organization. Each tag consists of a key and an optional value, both of     which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
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
    /// The name of the virtual gateway.
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
    #[serde(rename = "VirtualGatewayName")]
    pub virtual_gateway_name: Option<String>,
}

impl cfn_resources::CfnResource for CfnVirtualGateway {
    fn type_string(&self) -> &'static str {
        "AWS::AppMesh::VirtualGateway"
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

        if let Some(the_val) = &self.virtual_gateway_name {
            if the_val.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'virtual_gateway_name'. {} is greater than 255",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.virtual_gateway_name {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'virtual_gateway_name'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

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

/// The access log configuration for a virtual gateway.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayAccessLog {
    ///
    /// The file object to send virtual gateway access logs to.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayFileAccessLog
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayFileAccessLog>,
}

impl cfn_resources::CfnResource for VirtualGatewayAccessLog {
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

/// An object that represents the default properties for a backend.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayBackendDefaults {
    ///
    /// A reference to an object that represents a client policy.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayClientPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ClientPolicy")]
    pub client_policy: Option<VirtualGatewayClientPolicy>,
}

impl cfn_resources::CfnResource for VirtualGatewayBackendDefaults {
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
pub struct VirtualGatewayClientPolicy {
    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) client policy.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayClientPolicyTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLS")]
    pub tls: Option<VirtualGatewayClientPolicyTls>,
}

impl cfn_resources::CfnResource for VirtualGatewayClientPolicy {
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

/// An object that represents a Transport Layer Security (TLS) client policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayClientPolicyTls {
    ///
    /// A reference to an object that represents a virtual gateway's client's Transport Layer Security (TLS)     certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayClientTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: Option<VirtualGatewayClientTlsCertificate>,

    ///
    /// Whether the policy is enforced. The default is True, if a value isn't     specified.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enforce")]
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
    pub ports: Option<Vec<i64>>,

    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) validation context.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewayTlsValidationContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validation")]
    pub validation: VirtualGatewayTlsValidationContext,
}

impl cfn_resources::CfnResource for VirtualGatewayClientPolicyTls {
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

/// An object that represents the virtual gateway's client's Transport Layer Security (TLS) certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayClientTlsCertificate {
    ///
    /// An object that represents a local file certificate. The certificate must meet specific     requirements and you must have proxy authorization enabled. For more information, see       Transport Layer Security (TLS)     .
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsFileCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,

    ///
    /// A reference to an object that represents a virtual gateway's client's Secret Discovery     Service certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsSdsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
}

impl cfn_resources::CfnResource for VirtualGatewayClientTlsCertificate {
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

/// An object that represents the type of virtual gateway connection pool.
///
/// Only one protocol is used at a time and should be the same protocol as the one chosen     under port mapping.
///
/// If not present the default value for maxPendingRequests is       2147483647.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayConnectionPool {
    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayGrpcConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "GRPC")]
    pub grpc: Option<VirtualGatewayGrpcConnectionPool>,

    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayHttpConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP")]
    pub http: Option<VirtualGatewayHttpConnectionPool>,

    ///
    /// An object that represents a type of connection pool.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayHttp2ConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP2")]
    pub http2: Option<VirtualGatewayHttp2ConnectionPool>,
}

impl cfn_resources::CfnResource for VirtualGatewayConnectionPool {
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

        Ok(())
    }
}

/// An object that represents an access log file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayFileAccessLog {
    ///
    /// The specified format for the virtual gateway access logs. It can be either       json_format or text_format.
    ///
    /// Required: No
    ///
    /// Type: LoggingFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<LoggingFormat>,

    ///
    /// The file path to write access logs to. You can use /dev/stdout to send     access logs to standard out and configure your Envoy container to use a log driver, such as       awslogs, to export the access logs to a log storage service such as Amazon     CloudWatch Logs. You can also specify a path in the Envoy container's file system to write     the files to disk.
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

impl cfn_resources::CfnResource for VirtualGatewayFileAccessLog {
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

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayGrpcConnectionPool {
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

impl cfn_resources::CfnResource for VirtualGatewayGrpcConnectionPool {
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

/// An object that represents the health check policy for a virtual gateway's     listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayHealthCheckPolicy {
    ///
    /// The number of consecutive successful health checks that must occur before declaring the     listener healthy.
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
    pub port: Option<i64>,

    ///
    /// The protocol for the health check request. If you specify grpc, then your     service must conform to the GRPC Health       Checking Protocol.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: grpc | http | http2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: VirtualGatewayHealthCheckPolicyProtocolEnum,

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
    /// The number of consecutive failed health checks that must occur before declaring a     virtual gateway unhealthy.
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
pub enum VirtualGatewayHealthCheckPolicyProtocolEnum {
    /// grpc
    #[serde(rename = "grpc")]
    Grpc,

    /// http
    #[serde(rename = "http")]
    Http,

    /// http2
    #[serde(rename = "http2")]
    Http2,
}

impl Default for VirtualGatewayHealthCheckPolicyProtocolEnum {
    fn default() -> Self {
        VirtualGatewayHealthCheckPolicyProtocolEnum::Grpc
    }
}

impl cfn_resources::CfnResource for VirtualGatewayHealthCheckPolicy {
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

/// An object that represents a type of connection pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayHttp2ConnectionPool {
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

impl cfn_resources::CfnResource for VirtualGatewayHttp2ConnectionPool {
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
pub struct VirtualGatewayHttpConnectionPool {
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
    pub max_pending_requests: Option<i64>,
}

impl cfn_resources::CfnResource for VirtualGatewayHttpConnectionPool {
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

/// An object that represents a listener for a virtual gateway.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayListener {
    ///
    /// The connection pool information for the listener.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionPool")]
    pub connection_pool: Option<VirtualGatewayConnectionPool>,

    ///
    /// The health check information for the listener.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayHealthCheckPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<VirtualGatewayHealthCheckPolicy>,

    ///
    /// The port mapping information for the listener.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewayPortMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortMapping")]
    pub port_mapping: VirtualGatewayPortMapping,

    ///
    /// A reference to an object that represents the Transport Layer Security (TLS) properties for the listener.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLS")]
    pub tls: Option<VirtualGatewayListenerTls>,
}

impl cfn_resources::CfnResource for VirtualGatewayListener {
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

        self.port_mapping.validate()?;

        self.tls.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the Transport Layer Security (TLS) properties for a listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayListenerTls {
    ///
    /// An object that represents a Transport Layer Security (TLS) certificate.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewayListenerTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: VirtualGatewayListenerTlsCertificate,

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
    pub mode: VirtualGatewayListenerTlsModeEnum,

    ///
    /// A reference to an object that represents a virtual gateway's listener's Transport Layer Security (TLS) validation     context.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsValidationContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validation")]
    pub validation: Option<VirtualGatewayListenerTlsValidationContext>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VirtualGatewayListenerTlsModeEnum {
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

impl Default for VirtualGatewayListenerTlsModeEnum {
    fn default() -> Self {
        VirtualGatewayListenerTlsModeEnum::Disabled
    }
}

impl cfn_resources::CfnResource for VirtualGatewayListenerTls {
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
pub struct VirtualGatewayListenerTlsAcmCertificate {
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

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsAcmCertificate {
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
pub struct VirtualGatewayListenerTlsCertificate {
    ///
    /// A reference to an object that represents an AWS Certificate Manager certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsAcmCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
    pub acm: Option<VirtualGatewayListenerTlsAcmCertificate>,

    ///
    /// A reference to an object that represents a local file certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsFileCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayListenerTlsFileCertificate>,

    ///
    /// A reference to an object that represents a virtual gateway's listener's Secret Discovery     Service certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayListenerTlsSdsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayListenerTlsSdsCertificate>,
}

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsCertificate {
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
pub struct VirtualGatewayListenerTlsFileCertificate {
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
    /// The private key for a certificate stored on the file system of the mesh endpoint that     the proxy is running on.
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

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsFileCertificate {
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

/// An object that represents the virtual gateway's listener's Secret Discovery Service     certificate.The proxy must be configured with a local SDS provider via a Unix Domain     Socket. See App MeshTLS       documentation for more info.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayListenerTlsSdsCertificate {
    ///
    /// A reference to an object that represents the name of the secret secret requested from     the Secret Discovery Service provider representing Transport Layer Security (TLS) materials like a certificate or     certificate chain.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsSdsCertificate {
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

/// An object that represents a virtual gateway's listener's Transport Layer Security (TLS) validation     context.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayListenerTlsValidationContext {
    ///
    /// A reference to an object that represents the SANs for a virtual gateway listener's Transport Layer Security (TLS)     validation context.
    ///
    /// Required: No
    ///
    /// Type: SubjectAlternativeNames
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

    ///
    /// A reference to where to retrieve the trust chain when validating a peer’s Transport Layer Security (TLS)     certificate.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewayListenerTlsValidationContextTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trust")]
    pub trust: VirtualGatewayListenerTlsValidationContextTrust,
}

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsValidationContext {
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

/// An object that represents a virtual gateway's listener's Transport Layer Security (TLS) validation context     trust.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayListenerTlsValidationContextTrust {
    ///
    /// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayTlsValidationContextFileTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,

    ///
    /// A reference to an object that represents a virtual gateway's listener's Transport Layer Security (TLS) Secret     Discovery Service validation context trust.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayTlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
}

impl cfn_resources::CfnResource for VirtualGatewayListenerTlsValidationContextTrust {
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

/// An object that represents logging information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayLogging {
    ///
    /// The access log configuration.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayAccessLog
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLog")]
    pub access_log: Option<VirtualGatewayAccessLog>,
}

impl cfn_resources::CfnResource for VirtualGatewayLogging {
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

/// An object that represents a port mapping.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayPortMapping {
    ///
    /// The port used for the port mapping. Specify one protocol.
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
    /// The protocol used for the port mapping.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: grpc | http | http2
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: VirtualGatewayPortMappingProtocolEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VirtualGatewayPortMappingProtocolEnum {
    /// grpc
    #[serde(rename = "grpc")]
    Grpc,

    /// http
    #[serde(rename = "http")]
    Http,

    /// http2
    #[serde(rename = "http2")]
    Http2,
}

impl Default for VirtualGatewayPortMappingProtocolEnum {
    fn default() -> Self {
        VirtualGatewayPortMappingProtocolEnum::Grpc
    }
}

impl cfn_resources::CfnResource for VirtualGatewayPortMapping {
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

/// An object that represents the specification of a service mesh resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewaySpec {
    ///
    /// A reference to an object that represents the defaults for backends.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayBackendDefaults
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackendDefaults")]
    pub backend_defaults: Option<VirtualGatewayBackendDefaults>,

    ///
    /// The listeners that the mesh endpoint is expected to receive inbound traffic from. You     can specify one listener.
    ///
    /// Required: Yes
    ///
    /// Type: List of VirtualGatewayListener
    ///
    /// Update requires: No interruption
    #[serde(rename = "Listeners")]
    pub listeners: Vec<VirtualGatewayListener>,

    ///
    /// An object that represents logging information.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayLogging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    pub logging: Option<VirtualGatewayLogging>,
}

impl cfn_resources::CfnResource for VirtualGatewaySpec {
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

        Ok(())
    }
}

/// An object that represents a Transport Layer Security (TLS) validation context.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayTlsValidationContext {
    ///
    /// A reference to an object that represents the SANs for a virtual gateway's listener's     Transport Layer Security (TLS) validation context.
    ///
    /// Required: No
    ///
    /// Type: SubjectAlternativeNames
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

    ///
    /// A reference to where to retrieve the trust chain when validating a peer’s Transport Layer Security (TLS)     certificate.
    ///
    /// Required: Yes
    ///
    /// Type: VirtualGatewayTlsValidationContextTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trust")]
    pub trust: VirtualGatewayTlsValidationContextTrust,
}

impl cfn_resources::CfnResource for VirtualGatewayTlsValidationContext {
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
pub struct VirtualGatewayTlsValidationContextAcmTrust {
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

impl cfn_resources::CfnResource for VirtualGatewayTlsValidationContextAcmTrust {
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
pub struct VirtualGatewayTlsValidationContextFileTrust {
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

impl cfn_resources::CfnResource for VirtualGatewayTlsValidationContextFileTrust {
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

/// An object that represents a virtual gateway's listener's Transport Layer Security (TLS) Secret Discovery Service     validation context trust. The proxy must be configured with a local SDS provider via a Unix     Domain Socket. See App Mesh     TLS       documentation for more info.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualGatewayTlsValidationContextSdsTrust {
    ///
    /// A reference to an object that represents the name of the secret for a virtual gateway's     Transport Layer Security (TLS) Secret Discovery Service validation context trust.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretName")]
    pub secret_name: String,
}

impl cfn_resources::CfnResource for VirtualGatewayTlsValidationContextSdsTrust {
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
pub struct VirtualGatewayTlsValidationContextTrust {
    ///
    /// A reference to an object that represents a Transport Layer Security (TLS) validation context trust for an AWS Certificate Manager certificate.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayTlsValidationContextAcmTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
    pub acm: Option<VirtualGatewayTlsValidationContextAcmTrust>,

    ///
    /// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayTlsValidationContextFileTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<VirtualGatewayTlsValidationContextFileTrust>,

    ///
    /// A reference to an object that represents a virtual gateway's Transport Layer Security (TLS) Secret Discovery     Service validation context trust.
    ///
    /// Required: No
    ///
    /// Type: VirtualGatewayTlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<VirtualGatewayTlsValidationContextSdsTrust>,
}

impl cfn_resources::CfnResource for VirtualGatewayTlsValidationContextTrust {
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
