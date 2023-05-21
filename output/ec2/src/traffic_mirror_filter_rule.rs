/// Creates a Traffic Mirror filter rule.
///
/// A Traffic Mirror rule defines the Traffic Mirror source traffic to mirror.
///
/// You need the Traffic Mirror filter ID when you create the rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrafficMirrorFilterRule {
    ///
    /// The description of the Traffic Mirror rule.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The destination CIDR block to assign to the Traffic Mirror rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: cfn_resources::StrVal,

    ///
    /// The destination port range.
    ///
    /// Required: No
    ///
    /// Type: TrafficMirrorPortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<TrafficMirrorPortRange>,

    ///
    /// The protocol, for example UDP, to assign to the Traffic Mirror rule.
    ///
    /// For information about the protocol value, see Protocol Numbers on the Internet Assigned Numbers Authority (IANA) website.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<i64>,

    ///
    /// The action to take on the filtered traffic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: accept | reject
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleAction")]
    pub rule_action: TrafficMirrorFilterRuleRuleActionEnum,

    ///
    /// The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given     direction. The rules are processed in ascending order by rule number.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleNumber")]
    pub rule_number: i64,

    ///
    /// The source CIDR block to assign to the Traffic Mirror rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceCidrBlock")]
    pub source_cidr_block: cfn_resources::StrVal,

    ///
    /// The source port range.
    ///
    /// Required: No
    ///
    /// Type: TrafficMirrorPortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<TrafficMirrorPortRange>,

    ///
    /// The type of traffic.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: egress | ingress
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficDirection")]
    pub traffic_direction: TrafficMirrorFilterRuleTrafficDirectionEnum,

    ///
    /// The ID of the filter that this rule is associated with.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrafficMirrorFilterId")]
    pub traffic_mirror_filter_id: cfn_resources::StrVal,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TrafficMirrorFilterRuleRuleActionEnum {
    /// accept
    #[serde(rename = "accept")]
    Accept,

    /// reject
    #[serde(rename = "reject")]
    Reject,
}

impl Default for TrafficMirrorFilterRuleRuleActionEnum {
    fn default() -> Self {
        TrafficMirrorFilterRuleRuleActionEnum::Accept
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TrafficMirrorFilterRuleTrafficDirectionEnum {
    /// egress
    #[serde(rename = "egress")]
    Egress,

    /// ingress
    #[serde(rename = "ingress")]
    Ingress,
}

impl Default for TrafficMirrorFilterRuleTrafficDirectionEnum {
    fn default() -> Self {
        TrafficMirrorFilterRuleTrafficDirectionEnum::Egress
    }
}

impl cfn_resources::CfnResource for CfnTrafficMirrorFilterRule {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TrafficMirrorFilterRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.destination_port_range
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.source_port_range
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the Traffic Mirror port range.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrafficMirrorPortRange {
    ///
    /// The start of the Traffic Mirror port range. This applies to the TCP and UDP protocols.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    pub from_port: i64,

    ///
    /// The end of the Traffic Mirror port range. This applies to the TCP and UDP protocols.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    pub to_port: i64,
}

impl cfn_resources::CfnResource for TrafficMirrorPortRange {
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
