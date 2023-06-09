/// Requests a transit gateway peering attachment between the specified transit gateway     (requester) and a peer transit gateway (accepter). The peer transit gateway can be in your      account or a different AWS account.
///
/// After you create the peering attachment, the owner of the accepter transit gateway must     accept the attachment request.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnTransitGatewayPeeringAttachment {
    ///
    /// The ID of the AWS account that owns the transit gateway.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerAccountId")]
    pub peer_account_id: cfn_resources::StrVal,

    ///
    /// The Region of the transit gateway.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerRegion")]
    pub peer_region: cfn_resources::StrVal,

    ///
    /// The ID of the transit gateway.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PeerTransitGatewayId")]
    pub peer_transit_gateway_id: cfn_resources::StrVal,

    ///
    /// The tags for the transit gateway peering attachment.
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
    /// The ID of the transit gateway peering attachment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnTransitGatewayPeeringAttachmentcreationtime,

    #[serde(skip_serializing)]
    pub att_state: CfnTransitGatewayPeeringAttachmentstate,

    #[serde(skip_serializing)]
    pub att_status_code: CfnTransitGatewayPeeringAttachmentstatuscode,

    #[serde(skip_serializing)]
    pub att_status_message: CfnTransitGatewayPeeringAttachmentstatusmessage,

    #[serde(skip_serializing)]
    pub att_transit_gateway_attachment_id:
        CfnTransitGatewayPeeringAttachmenttransitgatewayattachmentid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayPeeringAttachmentcreationtime;
impl CfnTransitGatewayPeeringAttachmentcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayPeeringAttachmentstate;
impl CfnTransitGatewayPeeringAttachmentstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayPeeringAttachmentstatuscode;
impl CfnTransitGatewayPeeringAttachmentstatuscode {
    pub fn att_name(&self) -> &'static str {
        r#"Status.Code"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayPeeringAttachmentstatusmessage;
impl CfnTransitGatewayPeeringAttachmentstatusmessage {
    pub fn att_name(&self) -> &'static str {
        r#"Status.Message"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnTransitGatewayPeeringAttachmenttransitgatewayattachmentid;
impl CfnTransitGatewayPeeringAttachmenttransitgatewayattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayAttachmentId"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayPeeringAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::TransitGatewayPeeringAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The status of the transit gateway peering attachment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PeeringAttachmentStatus {
    ///
    /// The status code.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<cfn_resources::StrVal>,

    ///
    /// The status message, if applicable.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for PeeringAttachmentStatus {
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
