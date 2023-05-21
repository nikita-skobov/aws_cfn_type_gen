

/// A global endpoint used to improve your application's availability by making it regional-fault tolerant. For more information about global endpoints, see Making applications Regional-fault tolerant with global endpoints and event replication in the Amazon EventBridge User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpoint {


    /// 
    /// A description for the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The event buses being used by the endpoint.
    /// 
    /// Exactly: 2
    /// 
    /// Required: Yes
    ///
    /// Type: List of EndpointEventBus
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBuses")]
    pub event_buses: Vec<EndpointEventBus>,


    /// 
    /// The name of the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Pattern: [\.\-_A-Za-z0-9]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// Whether event replication was enabled or disabled for this endpoint. The default state is ENABLED which means you must supply a RoleArn.     If you don't have a RoleArn or you don't want event replication enabled, set the state to DISABLED.
    /// 
    /// Required: No
    ///
    /// Type: ReplicationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationConfig")]
    pub replication_config: Option<ReplicationConfig>,


    /// 
    /// The ARN of the role used by event replication for the endpoint.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^arn:aws[a-z-]*:iam::\d{12}:role\/[\w+=,.@/-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


    /// 
    /// The routing configuration of the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: RoutingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingConfig")]
    pub routing_config: RoutingConfig,

}



impl cfn_resources::CfnResource for CfnEndpoint {
    fn type_string() -> &'static str {
        "AWS::Events::Endpoint"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The event buses the endpoint is associated with.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EndpointEventBus {


    /// 
    /// The ARN of the event bus the endpoint is associated with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 512
    ///
    /// Pattern: ^arn:aws[a-z-]*:events:[a-z]{2}-[a-z-]+-\d+:\d{12}:event-bus/[\w.-]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBusArn")]
    pub event_bus_arn: String,

}




/// The failover configuration for an endpoint. This includes what triggers failover and what happens when it's triggered.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FailoverConfig {


    /// 
    /// The main Region of the endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: Primary
    ///
    /// Update requires: No interruption
    #[serde(rename = "Primary")]
    pub primary: Primary,


    /// 
    /// The Region that events are routed to when failover is triggered or event replication is enabled.
    /// 
    /// Required: Yes
    ///
    /// Type: Secondary
    ///
    /// Update requires: No interruption
    #[serde(rename = "Secondary")]
    pub secondary: Secondary,

}




/// The primary Region of the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Primary {


    /// 
    /// The ARN of the health check used by the endpoint to determine whether failover is triggered.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1600
    ///
    /// Pattern: ^arn:aws([a-z]|\-)*:route53:::healthcheck/[\-a-z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheck")]
    pub health_check: String,

}




/// Endpoints can replicate all events to the secondary Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ReplicationConfig {


    /// 
    /// The state of event replication.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "State")]
    pub state: ReplicationConfigStateEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ReplicationConfigStateEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

}

impl Default for ReplicationConfigStateEnum {
    fn default() -> Self {
        ReplicationConfigStateEnum::Disabled
    }
}



/// The routing configuration of the endpoint.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoutingConfig {


    /// 
    /// The failover configuration for an endpoint. This includes what triggers failover and what happens when it's triggered.
    /// 
    /// Required: Yes
    ///
    /// Type: FailoverConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailoverConfig")]
    pub failover_config: FailoverConfig,

}




/// The secondary Region that processes events when failover is triggered or replication is enabled.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Secondary {


    /// 
    /// Defines the secondary Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 9
    ///
    /// Maximum: 20
    ///
    /// Pattern: ^[\-a-z0-9]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Route")]
    pub route: String,

}


