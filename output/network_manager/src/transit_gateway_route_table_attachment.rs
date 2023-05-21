

/// Creates a transit gateway route table attachment.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayRouteTableAttachment {


    /// 
    /// The ARN of the transit gateway attachment route table. For example, "TransitGatewayRouteTableArn": "arn:aws:ec2:us-west-2:123456789012:transit-gateway-route-table/tgw-rtb-9876543210123456".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Pattern: [\s\S]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayRouteTableArn")]
    pub transit_gateway_route_table_arn: String,


    /// 
    /// The ID of the transit gateway peering.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 50
    ///
    /// Pattern: ^peering-([0-9a-f]{8,17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeeringId")]
    pub peering_id: String,


    /// 
    /// The list of key-value pairs associated with the transit gateway route table attachment.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// This property is read-only. Values can't be assigned to it.
    /// 
    /// Required: No
    ///
    /// Type: ProposedSegmentChange
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProposedSegmentChange")]
    pub proposed_segment_change: Option<ProposedSegmentChange>,

}


/// Describes a proposed segment change. In some cases, the segment change must first be evaluated and accepted.
#[derive(Default, serde::Serialize)]
pub struct ProposedSegmentChange {


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