

/// Creates a transit gateway route table attachment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGatewayRouteTableAttachment {


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
    /// This property is read-only. Values can't be assigned to it.
    /// 
    /// Required: No
    ///
    /// Type: ProposedSegmentChange
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProposedSegmentChange")]
    pub proposed_segment_change: Option<ProposedSegmentChange>,


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

}



impl cfn_resources::CfnResource for CfnTransitGatewayRouteTableAttachment {
    fn type_string(&self) -> &'static str {
        "AWS::NetworkManager::TransitGatewayRouteTableAttachment"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.peering_id;

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'peering_id'. {} is greater than 50", the_val.len()));
        }

        
        let the_val = &self.peering_id;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'peering_id'. {} is less than 0", the_val.len()));
        }

        
        self.proposed_segment_change.as_ref().map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.transit_gateway_route_table_arn;

        if the_val.len() > 500 as _ {
            return Err(format!("Max validation failed on field 'transit_gateway_route_table_arn'. {} is greater than 500", the_val.len()));
        }

        
        let the_val = &self.transit_gateway_route_table_arn;

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'transit_gateway_route_table_arn'. {} is less than 0", the_val.len()));
        }

        
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



impl cfn_resources::CfnResource for ProposedSegmentChange {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.segment_name {

        if the_val.len() > 256 as _ {
            return Err(format!("Max validation failed on field 'segment_name'. {} is greater than 256", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.segment_name {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'segment_name'. {} is less than 0", the_val.len()));
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
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}