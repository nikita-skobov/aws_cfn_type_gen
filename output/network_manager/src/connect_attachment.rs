

/// Creates a core network Connect attachment from a specified core network attachment.
///
/// A core network Connect attachment is a GRE-based tunnel attachment that you can use to     establish a connection between a core network and an appliance. A core network Connect     attachment uses an existing VPC attachment as the underlying transport mechanism.
#[derive(Default, serde::Serialize)]
pub struct CfnConnectAttachment {


    /// 
    /// Options for connecting an attachment.
    /// 
    /// Required: Yes
    ///
    /// Type: ConnectAttachmentOptions
    ///
    /// Update requires: Replacement
    #[serde(rename = "Options")]
    pub options: ConnectAttachmentOptions,


    /// 
    /// The ID of the core network where the Connect attachment is located.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: String,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The Region where the edge is located.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 63
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EdgeLocation")]
    pub edge_location: String,


    /// 
    /// The ID of the transport attachment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^attachment-([0-9a-f]{8,17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransportAttachmentId")]
    pub transport_attachment_id: String,

}


/// Describes a proposed segment change. In some cases, the segment change must first be evaluated and accepted.
#[derive(Default, serde::Serialize)]
pub struct ProposedSegmentChange {


    /// 
    /// The name of the segment to change.
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
    #[serde(rename = "SegmentName")]
    pub segment_name: Option<String>,


    /// 
    /// The rule number in the policy document that applies to this change.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachmentPolicyRuleNumber")]
    pub attachment_policy_rule_number: Option<i64>,


    /// 
    /// The list of key-value tags that changed for the segment.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


/// Describes a core network Connect attachment options.
#[derive(Default, serde::Serialize)]
pub struct ConnectAttachmentOptions {


    /// 
    /// The protocol used for the attachment connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GRE
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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
