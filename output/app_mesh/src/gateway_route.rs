

/// Creates a gateway route.
///
/// A gateway route is attached to a virtual gateway and routes traffic to an existing     virtual service. If a route matches a request, it can distribute traffic to a target     virtual service.
///
/// For more information about gateway routes, see Gateway routes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGatewayRoute {


    /// 
    /// Optional metadata that you can apply to the gateway route to assist with categorization     and organization. Each tag consists of a key and an optional value, both of which you     define. Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
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
    /// The specifications of the gateway route.
    /// 
    /// Required: Yes
    ///
    /// Type: GatewayRouteSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: GatewayRouteSpec,


    /// 
    /// The virtual gateway that the gateway route is associated with.
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
    #[serde(rename = "VirtualGatewayName")]
    pub virtual_gateway_name: String,


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
    /// The name of the service mesh that the resource resides in.
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
    /// The name of the gateway route.
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
    #[serde(rename = "GatewayRouteName")]
    pub gateway_route_name: Option<String>,

}

impl cfn_resources::CfnResource for CfnGatewayRoute {
    fn type_string() -> &'static str {
        "AWS::AppMesh::GatewayRoute"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// An object representing the query parameter to match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpQueryParameterMatch {


    /// 
    /// The exact query parameter to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


/// An object that represents an HTTP gateway route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRoute {


    /// 
    /// An object that represents the criteria for determining a request match.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpGatewayRouteMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: HttpGatewayRouteMatch,


    /// 
    /// An object that represents the action to take if a match is determined.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpGatewayRouteAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: HttpGatewayRouteAction,

}


/// An object representing the path to match in the request.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpPathMatch {


    /// 
    /// The regex used to match the path.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regex")]
    pub regex: Option<String>,


    /// 
    /// The exact path to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


/// An object that represents a gateway route specification. Specify one gateway route     type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteSpec {


    /// 
    /// The ordering of the gateway routes spec.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: Option<i64>,


    /// 
    /// An object that represents the specification of an HTTP gateway route.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRoute")]
    pub http_route: Option<HttpGatewayRoute>,


    /// 
    /// An object that represents the specification of a gRPC gateway route.
    /// 
    /// Required: No
    ///
    /// Type: GrpcGatewayRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrpcRoute")]
    pub grpc_route: Option<GrpcGatewayRoute>,


    /// 
    /// An object that represents the specification of an HTTP/2 gateway route.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Http2Route")]
    pub http2_route: Option<HttpGatewayRoute>,

}


/// An object that represents the range of values to match on. The first character of the range is included in the range, though the last character is not. For example, if the range specified were 1-100, only values 1-99 would be matched.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteRangeMatch {


    /// 
    /// The start of the range.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Start")]
    pub start: i64,


    /// 
    /// The end of the range.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "End")]
    pub end: i64,

}


/// An object that represents the action to take if a match is determined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRouteAction {


    /// 
    /// The gateway route action to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRouteRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rewrite")]
    pub rewrite: Option<HttpGatewayRouteRewrite>,


    /// 
    /// An object that represents the target that traffic is routed to when a request matches the gateway route.
    /// 
    /// Required: Yes
    ///
    /// Type: GatewayRouteTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: GatewayRouteTarget,

}


/// An object that represents a gRPC gateway route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcGatewayRoute {


    /// 
    /// An object that represents the action to take if a match is determined.
    /// 
    /// Required: Yes
    ///
    /// Type: GrpcGatewayRouteAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: GrpcGatewayRouteAction,


    /// 
    /// An object that represents the criteria for determining a request match.
    /// 
    /// Required: Yes
    ///
    /// Type: GrpcGatewayRouteMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: GrpcGatewayRouteMatch,

}


/// An object representing the gateway route host name to rewrite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteHostnameRewrite {


    /// 
    /// The default target host name to write to.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultTargetHostname")]
    pub default_target_hostname: Option<String>,

}


/// An object that represents a gateway route target.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteTarget {


    /// 
    /// An object that represents a virtual service gateway route target.
    /// 
    /// Required: Yes
    ///
    /// Type: GatewayRouteVirtualService
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualService")]
    pub virtual_service: GatewayRouteVirtualService,


    /// 
    /// The port number of the gateway route target.
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


/// An object representing the gateway route to rewrite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRouteRewrite {


    /// 
    /// The specified beginning characters to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRoutePrefixRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<HttpGatewayRoutePrefixRewrite>,


    /// 
    /// The path to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRoutePathRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<HttpGatewayRoutePathRewrite>,


    /// 
    /// The host name to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteHostnameRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,

}


/// An object that represents the query parameter in the request.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct QueryParameter {


    /// 
    /// The query parameter to match on.
    /// 
    /// Required: No
    ///
    /// Type: HttpQueryParameterMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<HttpQueryParameterMatch>,


    /// 
    /// A name for the query parameter that will be matched on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// An object that represents the action to take if a match is determined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcGatewayRouteAction {


    /// 
    /// An object that represents the target that traffic is routed to when a request matches the gateway route.
    /// 
    /// Required: Yes
    ///
    /// Type: GatewayRouteTarget
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: GatewayRouteTarget,


    /// 
    /// The gateway route action to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: GrpcGatewayRouteRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Rewrite")]
    pub rewrite: Option<GrpcGatewayRouteRewrite>,

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


/// An object representing the beginning characters of the route to rewrite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRoutePrefixRewrite {


    /// 
    /// The default prefix used to replace the incoming route prefix when rewritten.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultPrefix")]
    pub default_prefix: Option<String>,


    /// 
    /// The value used to replace the incoming route prefix when rewritten.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,

}


/// An object representing the gateway route host name to match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteHostnameMatch {


    /// 
    /// The specified ending characters of the host name to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// The exact host name to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 253
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


/// An object that represents the criteria for determining a request match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcGatewayRouteMatch {


    /// 
    /// The gateway route host name to be matched on.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteHostnameMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameMatch>,


    /// 
    /// The fully qualified domain name for the service to match from the request.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,


    /// 
    /// The gateway route metadata to be matched on.
    /// 
    /// Required: No
    ///
    /// Type: List of GrpcGatewayRouteMetadata
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: Option<Vec<GrpcGatewayRouteMetadata>>,


    /// 
    /// The gateway route port to be matched on.
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


/// An object representing the metadata of the gateway route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcGatewayRouteMetadata {


    /// 
    /// The criteria for determining a metadata match.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteMetadataMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<GatewayRouteMetadataMatch>,


    /// 
    /// A name for the gateway route metadata.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// Specify True to match anything except the match criteria. The default value     is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,

}


/// An object that represents the path to rewrite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRoutePathRewrite {


    /// 
    /// The exact path to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,

}


/// An object that represents the criteria for determining a request match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRouteMatch {


    /// 
    /// The host name to match on.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteHostnameMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameMatch>,


    /// 
    /// The query parameter to match on.
    /// 
    /// Required: No
    ///
    /// Type: List of QueryParameter
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "QueryParameters")]
    pub query_parameters: Option<Vec<QueryParameter>>,


    /// 
    /// The path to match on.
    /// 
    /// Required: No
    ///
    /// Type: HttpPathMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<HttpPathMatch>,


    /// 
    /// The method to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONNECT | DELETE | GET | HEAD | OPTIONS | PATCH | POST | PUT | TRACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Method")]
    pub method: Option<String>,


    /// 
    /// The client request headers to match on.
    /// 
    /// Required: No
    ///
    /// Type: List of HttpGatewayRouteHeader
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpGatewayRouteHeader>>,


    /// 
    /// Specifies the path to match requests with. This parameter must always start with       /, which by itself matches all requests to the virtual service name. You     can also match for path-based routing of requests. For example, if your virtual service     name is my-service.local and you want the route to match requests to       my-service.local/metrics, your prefix should be     /metrics.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The port number to match on.
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


/// An object representing the method header to be matched.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteMetadataMatch {


    /// 
    /// The exact method header to be matched on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,


    /// 
    /// The specified beginning characters of the method header to be matched on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The specified ending characters of the method header to match on.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// The regex used to match the method header.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regex")]
    pub regex: Option<String>,


    /// 
    /// An object that represents the range of values to match on.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteRangeMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<GatewayRouteRangeMatch>,

}


/// An object that represents the virtual service that traffic is routed to.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GatewayRouteVirtualService {


    /// 
    /// The name of the virtual service that traffic is routed to.
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
    #[serde(rename = "VirtualServiceName")]
    pub virtual_service_name: String,

}


/// An object that represents the HTTP header in the gateway route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRouteHeader {


    /// 
    /// Specify True to match anything except the match criteria. The default value     is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,


    /// 
    /// An object that represents the method and value to match with the header value sent in a     request. Specify one match method.
    /// 
    /// Required: No
    ///
    /// Type: HttpGatewayRouteHeaderMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<HttpGatewayRouteHeaderMatch>,


    /// 
    /// A name for the HTTP header in the gateway route that will be matched on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

}


/// An object that represents the gateway route to rewrite.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcGatewayRouteRewrite {


    /// 
    /// The host name of the gateway route to rewrite.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteHostnameRewrite
    ///
    /// Update requires: No interruption
    #[serde(rename = "Hostname")]
    pub hostname: Option<GatewayRouteHostnameRewrite>,

}


/// An object that represents the method and value to match with the header value sent in a     request. Specify one match method.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpGatewayRouteHeaderMatch {


    /// 
    /// The value sent by the client must end with the specified characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,


    /// 
    /// The value sent by the client must include the specified characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regex")]
    pub regex: Option<String>,


    /// 
    /// The value sent by the client must begin with the specified characters.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,


    /// 
    /// The value sent by the client must match the specified value exactly.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "Exact")]
    pub exact: Option<String>,


    /// 
    /// An object that represents the range of values to match on.
    /// 
    /// Required: No
    ///
    /// Type: GatewayRouteRangeMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<GatewayRouteRangeMatch>,

}
