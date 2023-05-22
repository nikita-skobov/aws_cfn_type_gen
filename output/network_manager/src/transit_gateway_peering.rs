/// Creates a transit gateway peering connection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeering {
    ///
    /// The ID of the core network.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CoreNetworkId")]
    pub core_network_id: cfn_resources::StrVal,

    ///
    /// The list of key-value tags associated with the peering.
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
    /// The ARN of the transit gateway.
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
    #[serde(rename = "TransitGatewayArn")]
    pub transit_gateway_arn: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_core_network_arn: CfnTransitGatewayPeeringcorenetworkarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnTransitGatewayPeeringcreatedat,

    #[serde(skip_serializing)]
    pub att_edge_location: CfnTransitGatewayPeeringedgelocation,

    #[serde(skip_serializing)]
    pub att_owner_account_id: CfnTransitGatewayPeeringowneraccountid,

    #[serde(skip_serializing)]
    pub att_peering_id: CfnTransitGatewayPeeringpeeringid,

    #[serde(skip_serializing)]
    pub att_peering_type: CfnTransitGatewayPeeringpeeringtype,

    #[serde(skip_serializing)]
    pub att_resource_arn: CfnTransitGatewayPeeringresourcearn,

    #[serde(skip_serializing)]
    pub att_state: CfnTransitGatewayPeeringstate,

    #[serde(skip_serializing)]
    pub att_transit_gateway_peering_attachment_id:
        CfnTransitGatewayPeeringtransitgatewaypeeringattachmentid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringcorenetworkarn;
impl CfnTransitGatewayPeeringcorenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"CoreNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringcreatedat;
impl CfnTransitGatewayPeeringcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringedgelocation;
impl CfnTransitGatewayPeeringedgelocation {
    pub fn att_name(&self) -> &'static str {
        r#"EdgeLocation"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringowneraccountid;
impl CfnTransitGatewayPeeringowneraccountid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerAccountId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringpeeringid;
impl CfnTransitGatewayPeeringpeeringid {
    pub fn att_name(&self) -> &'static str {
        r#"PeeringId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringpeeringtype;
impl CfnTransitGatewayPeeringpeeringtype {
    pub fn att_name(&self) -> &'static str {
        r#"PeeringType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringresourcearn;
impl CfnTransitGatewayPeeringresourcearn {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringstate;
impl CfnTransitGatewayPeeringstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayPeeringtransitgatewaypeeringattachmentid;
impl CfnTransitGatewayPeeringtransitgatewaypeeringattachmentid {
    pub fn att_name(&self) -> &'static str {
        r#"TransitGatewayPeeringAttachmentId"#
    }
}

impl cfn_resources::CfnResource for CfnTransitGatewayPeering {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::TransitGatewayPeering"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.transit_gateway_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 500 as _ {
                return Err(format!(
                    "Max validation failed on field 'transit_gateway_arn'. {} is greater than 500",
                    s.len()
                ));
            }
        }

        let the_val = &self.transit_gateway_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 0 as _ {
                return Err(format!(
                    "Min validation failed on field 'transit_gateway_arn'. {} is less than 0",
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
