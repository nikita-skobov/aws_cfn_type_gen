

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
    pub tags: Option<Vec<Tag>>,


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
    pub virtual_node_name: Option<String>,


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
    pub mesh_owner: Option<String>,

}



impl cfn_resources::CfnResource for CfnVirtualNode {
    fn type_string() -> &'static str {
        "AWS::AppMesh::VirtualNode"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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
    pub connection_pool: Option<VirtualNodeConnectionPool>,


    /// 
    /// An object that represents timeouts for different protocols.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<ListenerTimeout>,


    /// 
    /// The outlier detection information for the listener.
    /// 
    /// Required: No
    ///
    /// Type: OutlierDetection
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutlierDetection")]
    pub outlier_detection: Option<OutlierDetection>,


    /// 
    /// A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTls
    ///
    /// Update requires: No interruption
    #[serde(rename = "TLS")]
    pub tls: Option<ListenerTls>,


    /// 
    /// The health check information for the listener.
    /// 
    /// Required: No
    ///
    /// Type: HealthCheck
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheck>,


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



/// An object that represents the outlier detection for a virtual node's listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct OutlierDetection {


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
    /// The time interval between ejection sweep analysis.
    /// 
    /// Required: Yes
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: Duration,

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




/// An object that represents the service discovery information for a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceDiscovery {


    /// 
    /// Specifies the DNS information for the virtual node.
    /// 
    /// Required: No
    ///
    /// Type: DnsServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "DNS")]
    pub dns: Option<DnsServiceDiscovery>,


    /// 
    /// Specifies any AWS Cloud Map information for the virtual node.
    /// 
    /// Required: No
    ///
    /// Type: AwsCloudMapServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "AWSCloudMap")]
    pub awscloud_map: Option<AwsCloudMapServiceDiscovery>,

}




/// An object that represents the AWS Cloud Map service discovery information for     your virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AwsCloudMapServiceDiscovery {


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
    pub ip_preference: Option<AwsCloudMapServiceDiscoveryIpPreferenceEnum>,


    /// 
    /// A string map that contains attributes with values that you can use to filter instances     by any custom attribute that you specified when you registered the instance. Only instances     that match all of the specified key/value pairs will be returned.
    /// 
    /// Required: No
    ///
    /// Type: List of AwsCloudMapInstanceAttribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Option<Vec<AwsCloudMapInstanceAttribute>>,


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



/// An object that represents types of timeouts.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpTimeout {


    /// 
    /// An object that represents a per request timeout. The default value is 15 seconds. If you set a higher timeout, then make sure that the higher value is set for each App Mesh                  resource in a conversation. For example, if a virtual node backend uses a virtual router provider to route to another virtual node, then the timeout should be greater than 15                  seconds for the source and destination virtual node and the route.
    /// 
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerRequest")]
    pub per_request: Option<Duration>,


    /// 
    /// An object that represents an idle timeout. An idle timeout bounds the amount of time that a connection may be idle. The default value is none.
    /// 
    /// Required: No
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Idle")]
    pub idle: Option<Duration>,

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
    pub max_pending_requests: Option<i64>,

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
    pub idle: Option<Duration>,

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
    pub client_policy: Option<ClientPolicy>,

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
    pub per_request: Option<Duration>,

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
    pub file: Option<FileAccessLog>,

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
    pub tls: Option<ClientPolicyTls>,

}




/// An object that represents the key value pairs for the JSON.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JsonFormatRef {


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
    /// Type: VirtualNodeHttpConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP")]
    pub http: Option<VirtualNodeHttpConnectionPool>,


    /// 
    /// An object that represents a type of connection pool.
    /// 
    /// Required: No
    ///
    /// Type: VirtualNodeGrpcConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "GRPC")]
    pub grpc: Option<VirtualNodeGrpcConnectionPool>,


    /// 
    /// An object that represents a type of connection pool.
    /// 
    /// Required: No
    ///
    /// Type: VirtualNodeTcpConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "TCP")]
    pub tcp: Option<VirtualNodeTcpConnectionPool>,


    /// 
    /// An object that represents a type of connection pool.
    /// 
    /// Required: No
    ///
    /// Type: VirtualNodeHttp2ConnectionPool
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP2")]
    pub http2: Option<VirtualNodeHttp2ConnectionPool>,

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




/// A reference to an object that represents a Transport Layer Security (TLS) client policy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientPolicyTls {


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


    /// 
    /// A reference to an object that represents a client's TLS certificate.
    /// 
    /// Required: No
    ///
    /// Type: ClientTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
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
    pub sds: Option<ListenerTlsSdsCertificate>,

}




/// An object that represents a Transport Layer Security (TLS) validation context trust.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TlsValidationContextTrust {


    /// 
    /// A reference to an object that represents a Transport Layer Security (TLS) Secret Discovery Service validation     context trust.
    /// 
    /// Required: No
    ///
    /// Type: TlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<TlsValidationContextSdsTrust>,


    /// 
    /// A reference to an object that represents a Transport Layer Security (TLS) validation context trust for an AWS Certificate Manager certificate.
    /// 
    /// Required: No
    ///
    /// Type: TlsValidationContextAcmTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
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
    pub file: Option<TlsValidationContextFileTrust>,

}




/// An object that represents a listener's Transport Layer Security (TLS) validation context.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsValidationContext {


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


    /// 
    /// A reference to an object that represents the SANs for a listener's Transport Layer Security (TLS) validation     context.
    /// 
    /// Required: No
    ///
    /// Type: SubjectAlternativeNames
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubjectAlternativeNames")]
    pub subject_alternative_names: Option<SubjectAlternativeNames>,

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
    pub access_log: Option<AccessLog>,

}




/// An object that represents the specification of a virtual node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VirtualNodeSpec {


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
    pub backends: Option<Vec<Backend>>,


    /// 
    /// The inbound and outbound access logging information for the virtual node.
    /// 
    /// Required: No
    ///
    /// Type: Logging
    ///
    /// Update requires: No interruption
    #[serde(rename = "Logging")]
    pub logging: Option<Logging>,


    /// 
    /// A reference to an object that represents the defaults for backends.
    /// 
    /// Required: No
    ///
    /// Type: BackendDefaults
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackendDefaults")]
    pub backend_defaults: Option<BackendDefaults>,


    /// 
    /// The service discovery information for the virtual node. If your virtual node does not     expect ingress traffic, you can omit this parameter. If you specify a     listener, then you must specify service discovery information.
    /// 
    /// Required: No
    ///
    /// Type: ServiceDiscovery
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceDiscovery")]
    pub service_discovery: Option<ServiceDiscovery>,


    /// 
    /// The listener that the virtual node is expected to receive inbound traffic from. You can     specify one listener.
    /// 
    /// Required: No
    ///
    /// Type: List of Listener
    ///
    /// Update requires: No interruption
    #[serde(rename = "Listeners")]
    pub listeners: Option<Vec<Listener>>,

}




/// An object that represents the Transport Layer Security (TLS) properties for a listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTls {


    /// 
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) validation context.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTlsValidationContext
    ///
    /// Update requires: No interruption
    #[serde(rename = "Validation")]
    pub validation: Option<ListenerTlsValidationContext>,


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
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) certificate.
    /// 
    /// Required: Yes
    ///
    /// Type: ListenerTlsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "Certificate")]
    pub certificate: ListenerTlsCertificate,

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
    pub virtual_service: Option<VirtualServiceBackend>,

}




/// An object that represents an access log file.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FileAccessLog {


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


    /// 
    /// The specified format for the logs. The format is either json_format or       text_format.
    /// 
    /// Required: No
    ///
    /// Type: LoggingFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Format")]
    pub format: Option<LoggingFormat>,

}




/// An object that represents timeouts for different protocols.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTimeout {


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: HttpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP2")]
    pub http2: Option<HttpTimeout>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: HttpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "HTTP")]
    pub http: Option<HttpTimeout>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: TcpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "TCP")]
    pub tcp: Option<TcpTimeout>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: GrpcTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "GRPC")]
    pub grpc: Option<GrpcTimeout>,

}




/// An object that represents a listener's Transport Layer Security (TLS) validation context trust.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsValidationContextTrust {


    /// 
    /// A reference to an object that represents a listener's Transport Layer Security (TLS) Secret Discovery Service     validation context trust.
    /// 
    /// Required: No
    ///
    /// Type: TlsValidationContextSdsTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<TlsValidationContextSdsTrust>,


    /// 
    /// An object that represents a Transport Layer Security (TLS) validation context trust for a local file.
    /// 
    /// Required: No
    ///
    /// Type: TlsValidationContextFileTrust
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<TlsValidationContextFileTrust>,

}




/// An object that represents the DNS service discovery information for your virtual     node.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DnsServiceDiscovery {


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
    pub response_type: Option<DnsServiceDiscoveryResponseTypeEnum>,


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
    pub ip_preference: Option<DnsServiceDiscoveryIpPreferenceEnum>,

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




/// An object that represents a listener's Transport Layer Security (TLS) certificate.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListenerTlsCertificate {


    /// 
    /// A reference to an object that represents a local file certificate.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTlsFileCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "File")]
    pub file: Option<ListenerTlsFileCertificate>,


    /// 
    /// A reference to an object that represents an AWS Certificate Manager certificate.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTlsAcmCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "ACM")]
    pub acm: Option<ListenerTlsAcmCertificate>,


    /// 
    /// A reference to an object that represents a listener's Secret Discovery Service     certificate.
    /// 
    /// Required: No
    ///
    /// Type: ListenerTlsSdsCertificate
    ///
    /// Update requires: No interruption
    #[serde(rename = "SDS")]
    pub sds: Option<ListenerTlsSdsCertificate>,

}


