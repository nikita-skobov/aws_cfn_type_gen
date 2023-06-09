/// Creates a VPC attachment on an edge location of a core network.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnVpcAttachment {
    ///
    /// The core network ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: cfn_resources::StrVal,

    ///
    /// Options for creating the VPC attachment.
    ///
    /// Required: No
    ///
    /// Type: VpcOptions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VpcOptions>,

    ///
    /// The subnet ARNs.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetArns")]
    pub subnet_arns: Vec<String>,

    ///
    /// The tags associated with the VPC attachment.
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
    /// The ARN of the VPC attachment.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcArn")]
    pub vpc_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_attachment_id: CfnVpcAttachmentattachmentid,

    #[serde(skip_serializing)]
    pub att_attachment_type: CfnVpcAttachmentattachmenttype,

    #[serde(skip_serializing)]
    pub att_core_network_arn: CfnVpcAttachmentcorenetworkarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnVpcAttachmentcreatedat,

    #[serde(skip_serializing)]
    pub att_edge_location: CfnVpcAttachmentedgelocation,

    #[serde(skip_serializing)]
    pub att_owner_account_id: CfnVpcAttachmentowneraccountid,

    #[serde(skip_serializing)]
    pub att_proposed_segment_change_segment_name: CfnVpcAttachmentproposedsegmentchangesegmentname,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnVpcAttachmentresourcearn,

    #[serde(skip_serializing)]
    pub att_segment_name: CfnVpcAttachmentsegmentname,

    #[serde(skip_serializing)]
    pub att_state: CfnVpcAttachmentstate,

    #[serde(skip_serializing)]
    pub att_updated_at: CfnVpcAttachmentupdatedat,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentattachmentid;
impl CfnVpcAttachmentattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentattachmenttype;
impl CfnVpcAttachmentattachmenttype {
    pub fn att_name(&self) -> &'static str {
        r#"AttachmentType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentcorenetworkarn;
impl CfnVpcAttachmentcorenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentcreatedat;
impl CfnVpcAttachmentcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentedgelocation;
impl CfnVpcAttachmentedgelocation {
    pub fn att_name(&self) -> &'static str {
        r#"EdgeLocation"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentowneraccountid;
impl CfnVpcAttachmentowneraccountid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentproposedsegmentchangesegmentname;
impl CfnVpcAttachmentproposedsegmentchangesegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"ProposedSegmentChange.SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentresourcearn;
impl CfnVpcAttachmentresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentsegmentname;
impl CfnVpcAttachmentsegmentname {
    pub fn att_name(&self) -> &'static str {
        r#"SegmentName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentstate;
impl CfnVpcAttachmentstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVpcAttachmentupdatedat;
impl CfnVpcAttachmentupdatedat {
    pub fn att_name(&self) -> &'static str {
        r#"UpdatedAt"#
    }
}

impl cfn_resources::CfnResource for CfnVpcAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::VpcAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.options.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes a proposed segment change. In some cases, the segment change must first be evaluated and accepted.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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

/// Describes the VPC options.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VpcOptions {
    ///
    /// Indicates whether appliance mode is supported. If enabled, traffic flow between a source and destination use the same Availability Zone for the VPC attachment for the lifetime of that flow. The default value is false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplianceModeSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appliance_mode_support: Option<bool>,

    ///
    /// Indicates whether IPv6 is supported.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6Support")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_support: Option<bool>,
}

impl cfn_resources::CfnResource for VpcOptions {
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
