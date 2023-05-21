/// The AWS::GameLift::Alias resource creates an alias for an Amazon GameLift    (GameLift) fleet destination. There are two types of routing strategies for aliases: simple    and terminal. A simple alias points to an active fleet. A terminal alias displays a message    instead of routing players to an active fleet. For example, a terminal alias might display a    URL link that directs players to an upgrade site. You can use aliases to define destinations    in a game session queue or when requesting new game sessions.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAlias {
    ///
    /// A human-readable description of the alias.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A descriptive label that is associated with an alias. Alias names do not need to be unique.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The routing configuration, including routing type and fleet target, for the alias.
    ///
    /// Required: Yes
    ///
    /// Type: RoutingStrategy
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingStrategy")]
    pub routing_strategy: RoutingStrategy,
}

impl cfn_resources::CfnResource for CfnAlias {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::Alias"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.routing_strategy.validate()?;

        Ok(())
    }
}

/// The routing configuration for a fleet alias.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RoutingStrategy {
    ///
    /// A unique identifier for a fleet that the alias points to. If you specify    SIMPLE for the Type property, you must specify this    property.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Pattern: ^fleet-\S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<cfn_resources::StrVal>,

    ///
    /// The message text to be used with a terminal routing strategy. If you specify    TERMINAL for the Type property, you must specify this    property.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<cfn_resources::StrVal>,

    ///
    /// A type of routing strategy.
    ///
    /// Possible routing types include the following:
    ///
    /// SIMPLE - The alias resolves to one specific fleet. Use      this type when routing to active fleets.              TERMINAL - The alias does not resolve to a fleet but      instead can be used to display a message to the user. A terminal alias throws a      TerminalRoutingStrategyException with the message that you specified in the      Message property.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: SIMPLE | TERMINAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: RoutingStrategyTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum RoutingStrategyTypeEnum {
    /// SIMPLE
    #[serde(rename = "SIMPLE")]
    Simple,

    /// TERMINAL
    #[serde(rename = "TERMINAL")]
    Terminal,
}

impl Default for RoutingStrategyTypeEnum {
    fn default() -> Self {
        RoutingStrategyTypeEnum::Simple
    }
}

impl cfn_resources::CfnResource for RoutingStrategy {
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
