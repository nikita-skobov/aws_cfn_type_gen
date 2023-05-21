

/// Creates a Traffic Mirror filter rule.
///
/// A Traffic Mirror rule defines the Traffic Mirror source traffic to mirror.
///
/// You need the Traffic Mirror filter ID when you create the rule.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrafficMirrorFilterRule {


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
    /// The destination port range.
    /// 
    /// Required: No
    ///
    /// Type: TrafficMirrorPortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPortRange")]
    pub destination_port_range: Option<TrafficMirrorPortRange>,


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
    /// The ID of the filter that this rule is associated with.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TrafficMirrorFilterId")]
    pub traffic_mirror_filter_id: String,


    /// 
    /// The description of the Traffic Mirror rule.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The destination CIDR block to assign to the Traffic Mirror rule.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationCidrBlock")]
    pub destination_cidr_block: String,


    /// 
    /// The source CIDR block to assign to the Traffic Mirror rule.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceCidrBlock")]
    pub source_cidr_block: String,


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
    pub protocol: Option<i64>,


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
    /// The source port range.
    /// 
    /// Required: No
    ///
    /// Type: TrafficMirrorPortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePortRange")]
    pub source_port_range: Option<TrafficMirrorPortRange>,

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
    fn type_string() -> &'static str {
        "AWS::EC2::TrafficMirrorFilterRule"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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


