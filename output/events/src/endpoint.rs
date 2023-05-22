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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Whether event replication was enabled or disabled for this endpoint. The default state is ENABLED which means you must supply a RoleArn.     If you don't have a RoleArn or you don't want event replication enabled, set the state to DISABLED.
    ///
    /// Required: No
    ///
    /// Type: ReplicationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ReplicationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

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

    #[serde(skip_serializing)]
    pub att_arn: CfnEndpointarn,

    #[serde(skip_serializing)]
    pub att_endpoint_id: CfnEndpointendpointid,

    #[serde(skip_serializing)]
    pub att_endpoint_url: CfnEndpointendpointurl,

    #[serde(skip_serializing)]
    pub att_state: CfnEndpointstate,

    #[serde(skip_serializing)]
    pub att_state_reason: CfnEndpointstatereason,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointarn;
impl CfnEndpointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointendpointid;
impl CfnEndpointendpointid {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointendpointurl;
impl CfnEndpointendpointurl {
    pub fn att_name(&self) -> &'static str {
        r#"EndpointUrl"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointstate;
impl CfnEndpointstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnEndpointstatereason;
impl CfnEndpointstatereason {
    pub fn att_name(&self) -> &'static str {
        r#"StateReason"#
    }
}

impl cfn_resources::CfnResource for CfnEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::Events::Endpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 512 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 512",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.replication_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'role_arn'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.routing_config.validate()?;

        Ok(())
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
    pub event_bus_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EndpointEventBus {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.event_bus_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'event_bus_arn'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        let the_val = &self.event_bus_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'event_bus_arn'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for FailoverConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.primary.validate()?;

        self.secondary.validate()?;

        Ok(())
    }
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
    pub health_check: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Primary {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.health_check;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1600 as _ {
                return Err(format!(
                    "Max validation failed on field 'health_check'. {} is greater than 1600",
                    s.len()
                ));
            }
        }

        let the_val = &self.health_check;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'health_check'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
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

impl cfn_resources::CfnResource for ReplicationConfig {
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

impl cfn_resources::CfnResource for RoutingConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.failover_config.validate()?;

        Ok(())
    }
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
    pub route: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Secondary {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.route;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 20 as _ {
                return Err(format!(
                    "Max validation failed on field 'route'. {} is greater than 20",
                    s.len()
                ));
            }
        }

        let the_val = &self.route;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 9 as _ {
                return Err(format!(
                    "Min validation failed on field 'route'. {} is less than 9",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
