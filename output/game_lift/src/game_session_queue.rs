/// The AWS::GameLift::GameSessionQueue resource creates a placement queue    that processes requests for new game sessions. A queue uses FleetIQ algorithms to determine    the best placement locations and find an available game server, then prompts the game server    to start a new game session. Queues can have destinations (GameLift fleets or aliases), which    determine where the queue can place new game sessions. A queue can have destinations with    varied fleet type (Spot and On-Demand), instance type, and AWS Region.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnGameSessionQueue {
    ///
    /// Information to be added to all events that are related to this game session       queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEventData")]
    pub custom_event_data: Option<String>,

    ///
    /// A list of fleets and/or fleet aliases that can be used to fulfill game session placement requests in the queue.   Destinations are identified by either a fleet ARN or a fleet alias ARN, and are listed in order of placement preference.
    ///
    /// Required: No
    ///
    /// Type: List of Destination
    ///
    /// Update requires: No interruption
    #[serde(rename = "Destinations")]
    pub destinations: Option<Vec<Destination>>,

    ///
    /// A list of locations where a queue is allowed to place new game sessions. Locations       are specified in the form of AWS Region codes, such as us-west-2. If this parameter is       not set, game sessions can be placed in any queue location.
    ///
    /// Required: No
    ///
    /// Type: FilterConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "FilterConfiguration")]
    pub filter_configuration: Option<FilterConfiguration>,

    ///
    /// A descriptive label that is associated with game session queue. Queue names must be unique within each Region.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// An SNS topic ARN that is set up to receive game session placement notifications. See         Setting up         notifications for game session placement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 300
    ///
    /// Pattern: [a-zA-Z0-9:_-]*(\.fifo)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTarget")]
    pub notification_target: Option<String>,

    ///
    /// A set of policies that act as a sliding cap on player latency. FleetIQ works to       deliver low latency for most players in a game session. These policies ensure that no       individual player can be placed into a game with unreasonably high latency. Use multiple       policies to gradually relax latency requirements a step at a time. Multiple policies are applied based on their       maximum allowed latency, starting with the lowest value.
    ///
    /// Required: No
    ///
    /// Type: List of PlayerLatencyPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "PlayerLatencyPolicies")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,

    ///
    /// Custom settings to use when prioritizing destinations and locations for game session placements. This       configuration replaces the FleetIQ default prioritization process. Priority types that are not explicitly       named will be automatically applied at the end of the prioritization process.
    ///
    /// Required: No
    ///
    /// Type: PriorityConfiguration
    ///
    /// Update requires: No interruption
    #[serde(rename = "PriorityConfiguration")]
    pub priority_configuration: Option<PriorityConfiguration>,

    ///
    /// A list of labels to assign to the new game session queue resource. Tags are developer-defined    key-value pairs. Tagging    AWS resources are useful for resource management, access management and cost allocation.    For more information, see Tagging AWS Resources in the        AWS General Reference. Once the resource is created, you can    use TagResource, UntagResource, and    ListTagsForResource to add, remove, and view tags. The    maximum tag limit may be lower than stated. See the AWS General Reference for actual    tagging limits.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a TIMED_OUT status. By default, this property is set to 600.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeoutInSeconds")]
    pub timeout_in_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for CfnGameSessionQueue {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::GameSessionQueue"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.custom_event_data {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'custom_event_data'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.custom_event_data {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'custom_event_data'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.filter_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.name;

        if the_val.len() > 128 as _ {
            return Err(format!(
                "Max validation failed on field 'name'. {} is greater than 128",
                the_val.len()
            ));
        }

        let the_val = &self.name;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'name'. {} is less than 1",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.notification_target {
            if the_val.len() > 300 as _ {
                return Err(format!(
                    "Max validation failed on field 'notification_target'. {} is greater than 300",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.notification_target {
            if the_val.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'notification_target'. {} is less than 0",
                    the_val.len()
                ));
            }
        }

        self.priority_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.timeout_in_seconds {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'timeout_in_seconds'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// A fleet or alias designated in a game session queue. Queues fulfill requests for new       game sessions by placing a new game session on any of the queue's destinations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Destination {
    ///
    /// The Amazon Resource Name (ARN) that is assigned to fleet or fleet alias. ARNs, which       include a fleet ID or alias ID and a Region name, provide a unique identifier across all       Regions.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9:/-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationArn")]
    pub destination_arn: Option<String>,
}

impl cfn_resources::CfnResource for Destination {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.destination_arn {
            if the_val.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'destination_arn'. {} is greater than 256",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.destination_arn {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'destination_arn'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A list of fleet locations where a game session queue can place new game sessions. You    can use a filter to temporarily turn off placements for specific locations. For queues    that have multi-location fleets, you can use a filter configuration allow placement with    some, but not all of these locations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FilterConfiguration {
    ///
    /// A list of locations to allow game session placement in, in the form of AWS Region       codes such as us-west-2.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowedLocations")]
    pub allowed_locations: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for FilterConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.allowed_locations {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'allowed_locations'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The queue setting that determines the highest latency allowed for individual    players when placing a game session. When a latency policy is in force, a game session cannot    be placed with any fleet in a Region where a player reports latency higher than the cap.    Latency policies are only enforced when the placement request contains player latency    information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PlayerLatencyPolicy {
    ///
    /// The maximum latency value that is allowed for any player, in milliseconds. All       policies must have a value set for this property.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaximumIndividualPlayerLatencyMilliseconds")]
    pub maximum_individual_player_latency_milliseconds: Option<i64>,

    ///
    /// The length of time, in seconds, that the policy is enforced while placing a new game       session. A null value for this property means that the policy is enforced until the       queue times out.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyDurationSeconds")]
    pub policy_duration_seconds: Option<i64>,
}

impl cfn_resources::CfnResource for PlayerLatencyPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.maximum_individual_player_latency_milliseconds {
            if *the_val < 0 as _ {
                return Err(format!("Min validation failed on field 'maximum_individual_player_latency_milliseconds'. {} is less than 0", the_val));
            }
        }

        if let Some(the_val) = &self.policy_duration_seconds {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'policy_duration_seconds'. {} is less than 0",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Custom prioritization settings for use by a game session queue when placing new game    sessions with available game servers. When defined, this configuration replaces the    default FleetIQ prioritization process, which is as follows:
///
/// Changing the priority order will affect how game sessions are placed.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PriorityConfiguration {
    ///
    /// The prioritization order to use for fleet locations, when the         PriorityOrder property includes LOCATION. Locations are       identified by AWS Region codes such as us-west-2. Each location can only       be listed once.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationOrder")]
    pub location_order: Option<Vec<String>>,

    ///
    /// The recommended sequence to use when prioritizing where to place new game sessions.       Each type can only be listed once.
    ///
    /// LATENCY -- FleetIQ prioritizes locations where the average player           latency (provided in each game session request) is lowest.                         COST -- FleetIQ prioritizes destinations with the lowest current           hosting costs. Cost is evaluated based on the location, instance type, and fleet           type (Spot or On-Demand) for each destination in the queue.                        DESTINATION -- FleetIQ prioritizes based on the order that           destinations are listed in the queue configuration.                        LOCATION -- FleetIQ prioritizes based on the provided order of           locations, as defined in LocationOrder.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 4
    ///
    /// Update requires: No interruption
    #[serde(rename = "PriorityOrder")]
    pub priority_order: Option<Vec<String>>,
}

impl cfn_resources::CfnResource for PriorityConfiguration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.location_order {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'location_order'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.priority_order {
            if the_val.len() > 4 as _ {
                return Err(format!(
                    "Max validation failed on field 'priority_order'. {} is greater than 4",
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
