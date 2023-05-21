/// Creates a Traffic Mirror session.
///
/// A Traffic Mirror session actively copies packets from a Traffic Mirror source to a     Traffic Mirror target. Create a filter, and then assign it to the session to define a     subset of the traffic to mirror, for example all TCP traffic.
///
/// The Traffic Mirror source and the Traffic Mirror target (monitoring appliances) can be     in the same VPC, or in a different VPC connected via VPC peering or a transit gateway.
///
/// By default, no traffic is mirrored. Use AWS::EC2::TrafficMirrorFilterRule to specify filter rules that specify the     traffic to mirror.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTrafficMirrorSession {
    ///
    /// The description of the Traffic Mirror session.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    ///
    /// The ID of the source network interface.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkInterfaceId")]
    pub network_interface_id: String,

    ///
    /// The number of bytes in each packet to mirror. These are bytes after the VXLAN header. Do     not specify this parameter when you want to mirror the entire packet. To mirror a subset of     the packet, set this to the length (in bytes) that you want to mirror. For example, if you     set this value to 100, then the first 100 bytes that meet the filter criteria are copied to     the target.
    ///
    /// If you do not want to mirror the entire packet, use the PacketLength parameter to specify the number of bytes in each packet to mirror.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "PacketLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_length: Option<i64>,

    ///
    /// The session number determines the order in which sessions are evaluated when an interface is used by multiple sessions. The first session with a matching filter is the one that mirrors the packets.
    ///
    /// Valid values are 1-32766.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionNumber")]
    pub session_number: i64,

    ///
    /// The tags to assign to a Traffic Mirror session.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The ID of the Traffic Mirror filter.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficMirrorFilterId")]
    pub traffic_mirror_filter_id: String,

    ///
    /// The ID of the Traffic Mirror target.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficMirrorTargetId")]
    pub traffic_mirror_target_id: String,

    ///
    /// The VXLAN ID for the Traffic Mirror session. For more information about the VXLAN     protocol, see RFC 7348. If you do     not specify a VirtualNetworkId, an account-wide unique id is chosen at     random.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "VirtualNetworkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<i64>,
}

impl cfn_resources::CfnResource for CfnTrafficMirrorSession {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TrafficMirrorSession"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
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
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
