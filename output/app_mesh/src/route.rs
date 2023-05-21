

/// Creates a route that is associated with a virtual router.
///
/// You can route several different protocols and define a retry policy for a route.     Traffic can be routed to one or more virtual nodes.
///
/// For more information about routes, see Routes.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnRoute {


    /// 
    /// The name of the service mesh to create the route in.
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
    /// The name to use for the route.
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
    #[serde(rename = "RouteName")]
    pub route_name: Option<String>,


    /// 
    /// The route specification to apply.
    /// 
    /// Required: Yes
    ///
    /// Type: RouteSpec
    ///
    /// Update requires: No interruption
    #[serde(rename = "Spec")]
    pub spec: RouteSpec,


    /// 
    /// Optional metadata that you can apply to the route to assist with categorization and     organization. Each tag consists of a key and an optional value, both of which you define.     Tag keys can have a maximum character length of 128 characters, and tag values can have       a maximum length of 256 characters.
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
    /// The name of the virtual router in which to create the route. If the virtual router is in     a shared mesh, then you must be the owner of the virtual router resource.
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
    #[serde(rename = "VirtualRouterName")]
    pub virtual_router_name: String,

}



impl cfn_resources::CfnResource for CfnRoute {
    fn type_string() -> &'static str {
        "AWS::AppMesh::Route"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.mesh_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'mesh_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.mesh_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'mesh_name'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.mesh_owner {

        if the_val.len() > 12 as _ {
            return Err(format!("Max validation failed on field 'mesh_owner'. {} is greater than 12", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.mesh_owner {

        if the_val.len() < 12 as _ {
            return Err(format!("Min validation failed on field 'mesh_owner'. {} is less than 12", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.route_name {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'route_name'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.route_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'route_name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.spec.validate()?;

        if let Some(the_val) = &self.tags {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 50", the_val.len()));
        }

        }
        
        let the_val = &self.virtual_router_name;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'virtual_router_name'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.virtual_router_name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'virtual_router_name'. {} is less than 1", the_val.len()));
        }

        
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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// An object that represents a retry policy. Specify at least one value for at least one of the types of RetryEvents, a value for maxRetries, and a value for perRetryTimeout.         Both server-error and gateway-error under httpRetryEvents include the Envoy reset policy. For more information on the         reset policy, see the Envoy documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRetryPolicy {


    /// 
    /// Specify at least one of the valid values.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 5
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrpcRetryEvents")]
    pub grpc_retry_events: Option<Vec<String>>,


    /// 
    /// Specify at least one of the following values.
    /// 
    /// server-error – HTTP status codes 500, 501,          502, 503, 504, 505, 506, 507, 508, 510, and 511               gateway-error – HTTP status codes 502,          503, and 504               client-error – HTTP status code 409               stream-error – Retry on refused          stream
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRetryEvents")]
    pub http_retry_events: Option<Vec<String>>,


    /// 
    /// The maximum number of retry attempts.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    pub max_retries: i64,


    /// 
    /// The timeout for each retry attempt.
    /// 
    /// Required: Yes
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerRetryTimeout")]
    pub per_retry_timeout: Duration,


    /// 
    /// Specify a valid value. The event occurs before any processing of a request has started and is encountered when the upstream is temporarily or permanently unavailable.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "TcpRetryEvents")]
    pub tcp_retry_events: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for GrpcRetryPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.grpc_retry_events {

        if the_val.len() > 5 as _ {
            return Err(format!("Max validation failed on field 'grpc_retry_events'. {} is greater than 5", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.http_retry_events {

        if the_val.len() > 25 as _ {
            return Err(format!("Max validation failed on field 'http_retry_events'. {} is greater than 25", the_val.len()));
        }

        }
        
        self.per_retry_timeout.validate()?;

        if let Some(the_val) = &self.tcp_retry_events {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'tcp_retry_events'. {} is greater than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// An object that represents a gRPC route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRoute {


    /// 
    /// An object that represents the action to take if a match is determined.
    /// 
    /// Required: Yes
    ///
    /// Type: GrpcRouteAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: GrpcRouteAction,


    /// 
    /// An object that represents the criteria for determining a request match.
    /// 
    /// Required: Yes
    ///
    /// Type: GrpcRouteMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: GrpcRouteMatch,


    /// 
    /// An object that represents a retry policy.
    /// 
    /// Required: No
    ///
    /// Type: GrpcRetryPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<GrpcRetryPolicy>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: GrpcTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<GrpcTimeout>,

}



impl cfn_resources::CfnResource for GrpcRoute {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.action.validate()?;

        self.cfn_match.validate()?;

        self.retry_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.timeout.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the action to take if a match is determined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRouteAction {


    /// 
    /// An object that represents the targets that traffic is routed to when a request matches the route.
    /// 
    /// Required: Yes
    ///
    /// Type: List of WeightedTarget
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}



impl cfn_resources::CfnResource for GrpcRouteAction {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.weighted_targets;

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'weighted_targets'. {} is greater than 10", the_val.len()));
        }

        
        Ok(())
    }
}

/// An object that represents the criteria for determining a request match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRouteMatch {


    /// 
    /// An object that represents the data to match from the request.
    /// 
    /// Required: No
    ///
    /// Type: List of GrpcRouteMetadata
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Metadata")]
    pub metadata: Option<Vec<GrpcRouteMetadata>>,


    /// 
    /// The method name to match from the request. If you specify a name, you must also specify     a serviceName.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "MethodName")]
    pub method_name: Option<String>,


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

}



impl cfn_resources::CfnResource for GrpcRouteMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.metadata {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'metadata'. {} is greater than 10", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.method_name {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'method_name'. {} is greater than 50", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.method_name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'method_name'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.port {

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'port'. {} is greater than 65535", the_val));
        }

        }
        
        if let Some(the_val) = &self.port {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'port'. {} is less than 1", the_val));
        }

        }
        
        Ok(())
    }
}

/// An object that represents the match metadata for the route.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRouteMetadata {


    /// 
    /// Specify True to match anything except the match criteria. The default value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,


    /// 
    /// An object that represents the data to match from the request.
    /// 
    /// Required: No
    ///
    /// Type: GrpcRouteMetadataMatchMethod
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<GrpcRouteMetadataMatchMethod>,


    /// 
    /// The name of the route.
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



impl cfn_resources::CfnResource for GrpcRouteMetadata {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cfn_match.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// An object that represents the match method. Specify one of the match values.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GrpcRouteMetadataMatchMethod {


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
    /// An object that represents the range of values to match on.
    /// 
    /// Required: No
    ///
    /// Type: MatchRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<MatchRange>,


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

}



impl cfn_resources::CfnResource for GrpcRouteMetadataMatchMethod {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.exact {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'exact'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.exact {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'exact'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.prefix {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'prefix'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.prefix {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'prefix'. {} is less than 1", the_val.len()));
        }

        }
        
        self.range.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.regex {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'regex'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.regex {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'regex'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.suffix {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'suffix'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.suffix {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'suffix'. {} is less than 1", the_val.len()));
        }

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



impl cfn_resources::CfnResource for GrpcTimeout {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.per_request.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the method and value to match with the header value sent in a     request. Specify one match method.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HeaderMatchMethod {


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
    /// An object that represents the range of values to match on.
    /// 
    /// Required: No
    ///
    /// Type: MatchRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "Range")]
    pub range: Option<MatchRange>,


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

}



impl cfn_resources::CfnResource for HeaderMatchMethod {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.exact {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'exact'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.exact {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'exact'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.prefix {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'prefix'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.prefix {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'prefix'. {} is less than 1", the_val.len()));
        }

        }
        
        self.range.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.regex {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'regex'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.regex {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'regex'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.suffix {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'suffix'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.suffix {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'suffix'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// An object representing the path to match in the request.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpPathMatch {


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

}



impl cfn_resources::CfnResource for HttpPathMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.exact {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'exact'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.exact {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'exact'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.regex {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'regex'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.regex {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'regex'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
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



impl cfn_resources::CfnResource for HttpQueryParameterMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// An object that represents a retry policy. Specify at least one value for at least one of the types of RetryEvents, a value for maxRetries, and a value for perRetryTimeout.         Both server-error and gateway-error under httpRetryEvents include the Envoy reset policy. For more information on the         reset policy, see the Envoy documentation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpRetryPolicy {


    /// 
    /// Specify at least one of the following values.
    /// 
    /// server-error – HTTP status codes 500, 501,          502, 503, 504, 505, 506, 507, 508, 510, and 511               gateway-error – HTTP status codes 502,          503, and 504               client-error – HTTP status code 409               stream-error – Retry on refused          stream
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 25
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRetryEvents")]
    pub http_retry_events: Option<Vec<String>>,


    /// 
    /// The maximum number of retry attempts.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxRetries")]
    pub max_retries: i64,


    /// 
    /// The timeout for each retry attempt.
    /// 
    /// Required: Yes
    ///
    /// Type: Duration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PerRetryTimeout")]
    pub per_retry_timeout: Duration,


    /// 
    /// Specify a valid value. The event occurs before any processing of a request has started and is encountered when the upstream is temporarily or permanently unavailable.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "TcpRetryEvents")]
    pub tcp_retry_events: Option<Vec<String>>,

}



impl cfn_resources::CfnResource for HttpRetryPolicy {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.http_retry_events {

        if the_val.len() > 25 as _ {
            return Err(format!("Max validation failed on field 'http_retry_events'. {} is greater than 25", the_val.len()));
        }

        }
        
        self.per_retry_timeout.validate()?;

        if let Some(the_val) = &self.tcp_retry_events {

        if the_val.len() > 1 as _ {
            return Err(format!("Max validation failed on field 'tcp_retry_events'. {} is greater than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}

/// An object that represents an HTTP or HTTP/2 route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpRoute {


    /// 
    /// An object that represents the action to take if a match is determined.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpRouteAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: HttpRouteAction,


    /// 
    /// An object that represents the criteria for determining a request match.
    /// 
    /// Required: Yes
    ///
    /// Type: HttpRouteMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: HttpRouteMatch,


    /// 
    /// An object that represents a retry policy.
    /// 
    /// Required: No
    ///
    /// Type: HttpRetryPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RetryPolicy")]
    pub retry_policy: Option<HttpRetryPolicy>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: HttpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<HttpTimeout>,

}



impl cfn_resources::CfnResource for HttpRoute {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.action.validate()?;

        self.cfn_match.validate()?;

        self.retry_policy.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.timeout.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the action to take if a match is determined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpRouteAction {


    /// 
    /// An object that represents the targets that traffic is routed to when a request matches the route.
    /// 
    /// Required: Yes
    ///
    /// Type: List of WeightedTarget
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}



impl cfn_resources::CfnResource for HttpRouteAction {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.weighted_targets;

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'weighted_targets'. {} is greater than 10", the_val.len()));
        }

        
        Ok(())
    }
}

/// An object that represents the HTTP header in the request.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpRouteHeader {


    /// 
    /// Specify True to match anything except the match criteria. The default value is False.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Invert")]
    pub invert: Option<bool>,


    /// 
    /// The HeaderMatchMethod object.
    /// 
    /// Required: No
    ///
    /// Type: HeaderMatchMethod
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<HeaderMatchMethod>,


    /// 
    /// A name for the HTTP header in the client request that will be matched on.
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



impl cfn_resources::CfnResource for HttpRouteHeader {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cfn_match.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        
        Ok(())
    }
}

/// An object that represents the requirements for a route to match HTTP requests for a     virtual router.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HttpRouteMatch {


    /// 
    /// The client request headers to match on.
    /// 
    /// Required: No
    ///
    /// Type: List of HttpRouteHeader
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<HttpRouteHeader>>,


    /// 
    /// The client request method to match on. Specify only one.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CONNECT | DELETE | GET | HEAD | OPTIONS | PATCH | POST | PUT | TRACE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Method")]
    pub method: Option<HttpRouteMatchMethodEnum>,


    /// 
    /// The client request path to match on.
    /// 
    /// Required: No
    ///
    /// Type: HttpPathMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<HttpPathMatch>,


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
    /// The client request query parameters to match on.
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
    /// The client request scheme to match on. Specify only one. Applicable only for HTTP2     routes.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: http | https
    ///
    /// Update requires: No interruption
    #[serde(rename = "Scheme")]
    pub scheme: Option<HttpRouteMatchSchemeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum HttpRouteMatchMethodEnum {

    /// CONNECT
    #[serde(rename = "CONNECT")]
    Connect,

    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,

    /// GET
    #[serde(rename = "GET")]
    Get,

    /// HEAD
    #[serde(rename = "HEAD")]
    Head,

    /// OPTIONS
    #[serde(rename = "OPTIONS")]
    Options,

    /// PATCH
    #[serde(rename = "PATCH")]
    Patch,

    /// POST
    #[serde(rename = "POST")]
    Post,

    /// PUT
    #[serde(rename = "PUT")]
    Put,

    /// TRACE
    #[serde(rename = "TRACE")]
    Trace,

}

impl Default for HttpRouteMatchMethodEnum {
    fn default() -> Self {
        HttpRouteMatchMethodEnum::Connect
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum HttpRouteMatchSchemeEnum {

    /// http
    #[serde(rename = "http")]
    Http,

    /// https
    #[serde(rename = "https")]
    Https,

}

impl Default for HttpRouteMatchSchemeEnum {
    fn default() -> Self {
        HttpRouteMatchSchemeEnum::Http
    }
}


impl cfn_resources::CfnResource for HttpRouteMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.headers {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'headers'. {} is greater than 10", the_val.len()));
        }

        }
        
        self.path.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.port {

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'port'. {} is greater than 65535", the_val));
        }

        }
        
        if let Some(the_val) = &self.port {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'port'. {} is less than 1", the_val));
        }

        }
        
        if let Some(the_val) = &self.query_parameters {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'query_parameters'. {} is greater than 10", the_val.len()));
        }

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



impl cfn_resources::CfnResource for HttpTimeout {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.per_request.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the range of values to match on. The first character of the range is included in the range, though the last character is not. For example, if the range specified were 1-100, only values 1-99 would be matched.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MatchRange {


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

}



impl cfn_resources::CfnResource for MatchRange {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
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



impl cfn_resources::CfnResource for QueryParameter {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.cfn_match.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a route specification. Specify one route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RouteSpec {


    /// 
    /// An object that represents the specification of a gRPC route.
    /// 
    /// Required: No
    ///
    /// Type: GrpcRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrpcRoute")]
    pub grpc_route: Option<GrpcRoute>,


    /// 
    /// An object that represents the specification of an HTTP/2 route.
    /// 
    /// Required: No
    ///
    /// Type: HttpRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "Http2Route")]
    pub http2_route: Option<HttpRoute>,


    /// 
    /// An object that represents the specification of an HTTP route.
    /// 
    /// Required: No
    ///
    /// Type: HttpRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpRoute")]
    pub http_route: Option<HttpRoute>,


    /// 
    /// The priority for the route. Routes are matched based on the specified value, where 0 is     the highest priority.
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
    /// An object that represents the specification of a TCP route.
    /// 
    /// Required: No
    ///
    /// Type: TcpRoute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TcpRoute")]
    pub tcp_route: Option<TcpRoute>,

}



impl cfn_resources::CfnResource for RouteSpec {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.grpc_route.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http2_route.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.http_route.as_ref().map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.priority {

        if *the_val > 1000 as _ {
            return Err(format!("Max validation failed on field 'priority'. {} is greater than 1000", the_val));
        }

        }
        
        if let Some(the_val) = &self.priority {

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'priority'. {} is less than 0", the_val));
        }

        }
        
        self.tcp_route.as_ref().map_or(Ok(()), |val| val.validate())?;

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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}

/// An object that represents a TCP route type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TcpRoute {


    /// 
    /// The action to take if a match is determined.
    /// 
    /// Required: Yes
    ///
    /// Type: TcpRouteAction
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: TcpRouteAction,


    /// 
    /// An object that represents the criteria for determining a request match.
    /// 
    /// Required: No
    ///
    /// Type: TcpRouteMatch
    ///
    /// Update requires: No interruption
    #[serde(rename = "Match")]
    pub cfn_match: Option<TcpRouteMatch>,


    /// 
    /// An object that represents types of timeouts.
    /// 
    /// Required: No
    ///
    /// Type: TcpTimeout
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<TcpTimeout>,

}



impl cfn_resources::CfnResource for TcpRoute {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.action.validate()?;

        self.cfn_match.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.timeout.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents the action to take if a match is determined.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TcpRouteAction {


    /// 
    /// An object that represents the targets that traffic is routed to when a request matches the route.
    /// 
    /// Required: Yes
    ///
    /// Type: List of WeightedTarget
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "WeightedTargets")]
    pub weighted_targets: Vec<WeightedTarget>,

}



impl cfn_resources::CfnResource for TcpRouteAction {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.weighted_targets;

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'weighted_targets'. {} is greater than 10", the_val.len()));
        }

        
        Ok(())
    }
}

/// An object representing the TCP route to match.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TcpRouteMatch {


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



impl cfn_resources::CfnResource for TcpRouteMatch {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.port {

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'port'. {} is greater than 65535", the_val));
        }

        }
        
        if let Some(the_val) = &self.port {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'port'. {} is less than 1", the_val));
        }

        }
        
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
    pub idle: Option<Duration>,

}



impl cfn_resources::CfnResource for TcpTimeout {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.idle.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// An object that represents a target and its relative weight. Traffic is distributed     across targets according to their relative weight. For example, a weighted target with a     relative weight of 50 receives five times as much traffic as one with a relative weight of     10. The total weight for all targets combined must be less than or equal to 100.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct WeightedTarget {


    /// 
    /// The targeted port of the weighted object.
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
    /// The virtual node to associate with the weighted target.
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
    #[serde(rename = "VirtualNode")]
    pub virtual_node: String,


    /// 
    /// The relative weight of the weighted target.
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
    #[serde(rename = "Weight")]
    pub weight: i64,

}



impl cfn_resources::CfnResource for WeightedTarget {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.port {

        if *the_val > 65535 as _ {
            return Err(format!("Max validation failed on field 'port'. {} is greater than 65535", the_val));
        }

        }
        
        if let Some(the_val) = &self.port {

        if *the_val < 1 as _ {
            return Err(format!("Min validation failed on field 'port'. {} is less than 1", the_val));
        }

        }
        
        let the_val = &self.virtual_node;

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'virtual_node'. {} is greater than 255", the_val.len()));
        }

        
        let the_val = &self.virtual_node;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'virtual_node'. {} is less than 1", the_val.len()));
        }

        
        let the_val = &self.weight;

        if *the_val > 100 as _ {
            return Err(format!("Max validation failed on field 'weight'. {} is greater than 100", the_val));
        }

        
        let the_val = &self.weight;

        if *the_val < 0 as _ {
            return Err(format!("Min validation failed on field 'weight'. {} is less than 0", the_val));
        }

        
        Ok(())
    }
}