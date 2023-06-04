/// Creates an Amazon Web Services site-to-site VPN attachment on an edge location of a core network.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachment {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: cfn_resources::StrVal,

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
    /// The ARN of the site-to-site VPN attachment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 500
    ///
    /// Pattern: ^arn:[^:]{1,63}:ec2:[^:]{0,63}:[^:]{0,63}:vpn-connection\/vpn-[0-9a-f]{8,17}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpnConnectionArn")]
    pub vpn_connection_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_attachment_id: CfnSiteToSiteVpnAttachmentattachmentid,

    #[serde(skip_serializing)]
    pub att_attachment_type: CfnSiteToSiteVpnAttachmentattachmenttype,

    #[serde(skip_serializing)]
    pub att_core_network_arn: CfnSiteToSiteVpnAttachmentcorenetworkarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnSiteToSiteVpnAttachmentcreatedat,

    #[serde(skip_serializing)]
    pub att_edge_location: CfnSiteToSiteVpnAttachmentedgelocation,

    #[serde(skip_serializing)]
    pub att_owner_account_id: CfnSiteToSiteVpnAttachmentowneraccountid,

    #[serde(skip_serializing)]
    pub att_proposed_segment_change_segment_name:
        CfnSiteToSiteVpnAttachmentproposedsegmentchangesegmentname,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnSiteToSiteVpnAttachmentresourcearn,

    #[serde(skip_serializing)]
    pub att_segment_name: CfnSiteToSiteVpnAttachmentsegmentname,

    #[serde(skip_serializing)]
    pub att_state: CfnSiteToSiteVpnAttachmentstate,

    #[serde(skip_serializing)]
    pub att_updated_at: CfnSiteToSiteVpnAttachmentupdatedat,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentattachmentid;
impl CfnSiteToSiteVpnAttachmentattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentattachmenttype;
impl CfnSiteToSiteVpnAttachmentattachmenttype {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentcorenetworkarn;
impl CfnSiteToSiteVpnAttachmentcorenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentcreatedat;
impl CfnSiteToSiteVpnAttachmentcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentedgelocation;
impl CfnSiteToSiteVpnAttachmentedgelocation {
    pub fn att_name(&self) -> &'static str {
        r#"EdgeLocation"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentowneraccountid;
impl CfnSiteToSiteVpnAttachmentowneraccountid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentproposedsegmentchangesegmentname;
impl CfnSiteToSiteVpnAttachmentproposedsegmentchangesegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"ProposedSegmentChange.SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentresourcearn;
impl CfnSiteToSiteVpnAttachmentresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentsegmentname;
impl CfnSiteToSiteVpnAttachmentsegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentstate;
impl CfnSiteToSiteVpnAttachmentstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSiteToSiteVpnAttachmentupdatedat;
impl CfnSiteToSiteVpnAttachmentupdatedat {
    pub fn att_name(&self) -> &'static str {
        r#"UpdatedAt"#
    }
}

impl cfn_resources::CfnResource for CfnSiteToSiteVpnAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::SiteToSiteVpnAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.vpn_connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'vpn_connection_arn'. {} is greater than 500",
                    s.len()
                ));
            }
        }

        let the_val = &self.vpn_connection_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'vpn_connection_arn'. {} is less than 0",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Describes a proposed segment change. In some cases, the segment change must first be evaluated and accepted.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
