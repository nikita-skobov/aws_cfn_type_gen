/// Describes an association between a local gateway route table and a virtual interface group.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociation {


    /// 
    /// The ID of the local gateway route table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalGatewayRouteTableId")]

    pub local_gateway_route_table_id: cfn_resources::StrVal,


    /// 
    /// The ID of the virtual interface group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LocalGatewayVirtualInterfaceGroupId")]

    pub local_gateway_virtual_interface_group_id: cfn_resources::StrVal,


    /// 
    /// The tags assigned to the association.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,


    #[serde(skip_serializing)]
    pub att_local_gateway_id: CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayid,

    #[serde(skip_serializing)]
    pub att_local_gateway_route_table_arn: CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablearn,

    #[serde(skip_serializing)]
    pub att_local_gateway_route_table_virtual_interface_group_association_id: CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablevirtualinterfacegroupassociationid,

    #[serde(skip_serializing)]
    pub att_owner_id: CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationownerid,

    #[serde(skip_serializing)]
    pub att_state: CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationstate,

}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayid;
impl CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayid {
    pub fn att_name(&self) -> &'static str {
        r#"LocalGatewayId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablearn;
impl CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablearn {
    pub fn att_name(&self) -> &'static str {
        r#"LocalGatewayRouteTableArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablevirtualinterfacegroupassociationid;
impl CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationlocalgatewayroutetablevirtualinterfacegroupassociationid {
    pub fn att_name(&self) -> &'static str {
        r#"LocalGatewayRouteTableVirtualInterfaceGroupAssociationId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationownerid;
impl CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationownerid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationstate;
impl CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociationstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnLocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::LocalGatewayRouteTableVirtualInterfaceGroupAssociation"
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
