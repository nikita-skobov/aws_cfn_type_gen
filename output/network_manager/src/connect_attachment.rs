/// Creates a core network Connect attachment from a specified core network attachment.
///
/// A core network Connect attachment is a GRE-based tunnel attachment that you can use to     establish a connection between a core network and an appliance. A core network Connect     attachment uses an existing VPC attachment as the underlying transport mechanism.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachment {
    ///
    /// The ID of the core network where the Connect attachment is located.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: cfn_resources::StrVal,

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
    pub edge_location: cfn_resources::StrVal,

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

    /// Property description not available.
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
    pub transport_attachment_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_attachment_id: CfnConnectAttachmentattachmentid,

    #[serde(skip_serializing)]
    pub att_attachment_type: CfnConnectAttachmentattachmenttype,

    #[serde(skip_serializing)]
    pub att_core_network_arn: CfnConnectAttachmentcorenetworkarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnConnectAttachmentcreatedat,

    #[serde(skip_serializing)]
    pub att_owner_account_id: CfnConnectAttachmentowneraccountid,

    #[serde(skip_serializing)]
    pub att_proposed_segment_change_segment_name:
        CfnConnectAttachmentproposedsegmentchangesegmentname,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnConnectAttachmentresourcearn,

    #[serde(skip_serializing)]
    pub att_segment_name: CfnConnectAttachmentsegmentname,

    #[serde(skip_serializing)]
    pub att_state: CfnConnectAttachmentstate,

    #[serde(skip_serializing)]
    pub att_updated_at: CfnConnectAttachmentupdatedat,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentattachmentid;
impl CfnConnectAttachmentattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentattachmenttype;
impl CfnConnectAttachmentattachmenttype {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentcorenetworkarn;
impl CfnConnectAttachmentcorenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentcreatedat;
impl CfnConnectAttachmentcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentowneraccountid;
impl CfnConnectAttachmentowneraccountid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentproposedsegmentchangesegmentname;
impl CfnConnectAttachmentproposedsegmentchangesegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"ProposedSegmentChange.SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentresourcearn;
impl CfnConnectAttachmentresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentsegmentname;
impl CfnConnectAttachmentsegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentstate;
impl CfnConnectAttachmentstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectAttachmentupdatedat;
impl CfnConnectAttachmentupdatedat {
    pub fn att_name(&self) -> &'static str {
        r#"UpdatedAt"#
    }
}

impl cfn_resources::CfnResource for CfnConnectAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::ConnectAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.edge_location;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'edge_location'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.edge_location;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'edge_location'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.options.validate()?;

        let the_val = &self.transport_attachment_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 50 as _ {
                return Err(format!("Max validation failed on field 'transport_attachment_id'. {} is greater than 50", s.len()));
            }
        }

        let the_val = &self.transport_attachment_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'transport_attachment_id'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes a core network Connect attachment options.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ConnectAttachmentOptionsProtocolEnum>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectAttachmentOptionsProtocolEnum {
    /// GRE
    #[serde(rename = "GRE")]
    Gre,
}

impl Default for ConnectAttachmentOptionsProtocolEnum {
    fn default() -> Self {
        ConnectAttachmentOptionsProtocolEnum::Gre
    }
}

impl cfn_resources::CfnResource for ConnectAttachmentOptions {
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

/// Describes a proposed segment change. In some cases, the segment change must first be evaluated and accepted.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<cfn_resources::StrVal>,

    ///
    /// The list of key-value tags that changed for the segment.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for ProposedSegmentChange {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.segment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'segment_name'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.segment_name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'segment_name'. {} is less than 0",
                        s.len()
                    ));
                }
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
