/// The AWS::GameLift::MatchmakingConfiguration resource defines a new    matchmaking configuration for use with FlexMatch. Whether you're using FlexMatch with GameLift hosting or as a    standalone matchmaking service, the matchmaking configuration sets out rules for matching players and forming teams.    If you're using GameLift hosting, it also defines how to start game sessions for each match. Your matchmaking system    can use multiple configurations to handle different game scenarios. All matchmaking requests identify the    matchmaking configuration to use and provide player attributes that are consistent with that configuration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMatchmakingConfiguration {
    ///
    /// A flag that determines whether a match that was created with this configuration must       be accepted by the matched players. To require acceptance, set to TRUE.       With this option enabled, matchmaking tickets use the status         REQUIRES_ACCEPTANCE to indicate when a completed potential match is       waiting for player acceptance.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptanceRequired")]
    pub acceptance_required: bool,

    ///
    /// The length of time (in seconds) to wait for players to accept a proposed match, if       acceptance is required.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 600
    ///
    /// Update requires: No interruption
    #[serde(rename = "AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,

    ///
    /// The number of player slots in a match to keep open for future players. For example, if the configuration's rule set specifies       a match for a single 10-person team, and the additional player count is set to 2, 10 players will be selected for the match and 2 more player slots will be open for future players. This parameter is not used if FlexMatchMode is set to         STANDALONE.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Update requires: No interruption
    #[serde(rename = "AdditionalPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_player_count: Option<i64>,

    ///
    /// The method used to backfill game sessions that are created with this matchmaking    configuration. Specify MANUAL when your game manages backfill requests manually    or does not use the match backfill feature. Specify AUTOMATIC to have GameLift    create a StartMatchBackfill request whenever a game session has one or more open    slots. Learn more about manual and automatic backfill in Backfill Existing Games with     FlexMatch. Automatic backfill is not    available when FlexMatchMode is set to STANDALONE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | MANUAL
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackfillMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_mode: Option<MatchmakingConfigurationBackfillModeEnum>,

    ///
    /// Information to add to all events related to the matchmaking configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 256
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomEventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event_data: Option<cfn_resources::StrVal>,

    ///
    /// A description for the matchmaking configuration.
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
    /// Indicates whether this matchmaking configuration is being used with Amazon GameLift hosting or       as a standalone matchmaking solution.
    ///
    /// STANDALONE - FlexMatch forms matches and           returns match information, including players and team assignments, in a MatchmakingSucceeded event.                        WITH_QUEUE - FlexMatch forms matches and uses           the specified Amazon GameLift queue to start a game session for the match.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: STANDALONE | WITH_QUEUE
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlexMatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_match_mode: Option<MatchmakingConfigurationFlexMatchModeEnum>,

    ///
    /// A set of custom properties for a game session, formatted as key-value pairs. These    properties are passed to a game server process with a request to start a new game session. See    Start a Game Session.    This parameter is not used if FlexMatchMode is set to STANDALONE.
    ///
    /// Required: No
    ///
    /// Type: List of GameProperty
    ///
    /// Maximum: 16
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,

    ///
    /// A set of custom game session properties, formatted as a single string value. This    data is passed to a game server process with a request to start a new game session.    See Start a Game Session.    This parameter is not used ifFlexMatchMode is set to STANDALONE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 4096
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<cfn_resources::StrVal>,

    ///
    /// The Amazon Resource Name (ARN) that is assigned to a Amazon GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Format is arn:aws:gamelift:<region>::gamesessionqueue/<queue name>. Queues can be located in any Region. Queues are used to start new       Amazon GameLift-hosted game sessions for matches that are created with this matchmaking       configuration. If FlexMatchMode is set to STANDALONE, do not       set this parameter.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GameSessionQueueArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,

    ///
    /// A unique identifier for the matchmaking configuration. This name is used to identify the configuration associated with a matchmaking       request or ticket.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 128
    ///
    /// Pattern: [a-zA-Z0-9-\.]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// An SNS topic ARN that is set up to receive matchmaking notifications. See         Setting up notifications for matchmaking for more information.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 300
    ///
    /// Pattern: [a-zA-Z0-9:_/-]*(.fifo)?
    ///
    /// Update requires: No interruption
    #[serde(rename = "NotificationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target: Option<cfn_resources::StrVal>,

    ///
    /// The maximum duration, in seconds, that a matchmaking ticket can remain in process       before timing out. Requests that fail due to timing out can be resubmitted as       needed.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 43200
    ///
    /// Update requires: No interruption
    #[serde(rename = "RequestTimeoutSeconds")]
    pub request_timeout_seconds: i64,

    ///
    /// A unique identifier for the matchmaking rule set to use with this configuration. You can use either the rule set name or ARN       value. A matchmaking configuration can only use rule sets that are defined in the same       Region.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9-\.]*|^arn:.*:matchmakingruleset\/[a-zA-Z0-9-\.]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleSetName")]
    pub rule_set_name: cfn_resources::StrVal,

    ///
    /// A list of labels to assign to the new matchmaking configuration resource. Tags are developer-defined    key-value pairs. Tagging    AWS resources are useful for resource management, access management and cost allocation.    For more information, see Tagging AWS Resources in the        AWS General Reference. Once the resource is created, you can    use TagResource, UntagResource, and    ListTagsForResource to add, remove, and view tags. The    maximum tag limit may be lower than stated. See the AWS General Reference for actual    tagging limits.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnMatchmakingConfigurationarn,

    #[serde(skip_serializing)]
    pub att_name: CfnMatchmakingConfigurationname,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MatchmakingConfigurationBackfillModeEnum {
    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// MANUAL
    #[serde(rename = "MANUAL")]
    Manual,
}

impl Default for MatchmakingConfigurationBackfillModeEnum {
    fn default() -> Self {
        MatchmakingConfigurationBackfillModeEnum::Automatic
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum MatchmakingConfigurationFlexMatchModeEnum {
    /// STANDALONE
    #[serde(rename = "STANDALONE")]
    Standalone,

    /// WITH_QUEUE
    #[serde(rename = "WITH_QUEUE")]
    Withqueue,
}

impl Default for MatchmakingConfigurationFlexMatchModeEnum {
    fn default() -> Self {
        MatchmakingConfigurationFlexMatchModeEnum::Standalone
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMatchmakingConfigurationarn;
impl CfnMatchmakingConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMatchmakingConfigurationname;
impl CfnMatchmakingConfigurationname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnMatchmakingConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::GameLift::MatchmakingConfiguration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.acceptance_timeout_seconds {
            if *the_val > 600 as _ {
                return Err(format!("Max validation failed on field 'acceptance_timeout_seconds'. {} is greater than 600", the_val));
            }
        }

        if let Some(the_val) = &self.acceptance_timeout_seconds {
            if *the_val < 1 as _ {
                return Err(format!("Min validation failed on field 'acceptance_timeout_seconds'. {} is less than 1", the_val));
            }
        }

        if let Some(the_val) = &self.additional_player_count {
            if *the_val < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'additional_player_count'. {} is less than 0",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.custom_event_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!("Max validation failed on field 'custom_event_data'. {} is greater than 256", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.custom_event_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'custom_event_data'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

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

        if let Some(the_val) = &self.game_properties {
            if the_val.len() > 16 as _ {
                return Err(format!(
                    "Max validation failed on field 'game_properties'. {} is greater than 16",
                    the_val.len()
                ));
            }
        }

        if let Some(the_val) = &self.game_session_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 4096 as _ {
                    return Err(format!("Max validation failed on field 'game_session_data'. {} is greater than 4096", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.game_session_data {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'game_session_data'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 128 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 128",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.notification_target {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 300 as _ {
                    return Err(format!("Max validation failed on field 'notification_target'. {} is greater than 300", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.notification_target {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'notification_target'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.request_timeout_seconds;

        if *the_val > 43200 as _ {
            return Err(format!("Max validation failed on field 'request_timeout_seconds'. {} is greater than 43200", the_val));
        }

        let the_val = &self.request_timeout_seconds;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'request_timeout_seconds'. {} is less than 1",
                the_val
            ));
        }

        let the_val = &self.rule_set_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'rule_set_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.rule_set_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'rule_set_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// Set of key-value pairs that contain information about a game session. When included in       a game session request, these properties communicate details to be used when setting up       the new game session. For example, a game property might specify a game mode, level, or       map. Game properties are passed to the game server process when initiating a new game       session. For more information, see the Amazon GameLift Developer Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct GameProperty {
    ///
    /// The game property identifier.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The game property value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 96
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for GameProperty {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.key;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'key'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.value;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 96 as _ {
                return Err(format!(
                    "Max validation failed on field 'value'. {} is greater than 96",
                    s.len()
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
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
