

/// Specifies a transit gateway.
///
/// You can use a transit gateway to interconnect your virtual private clouds (VPC) and     on-premises networks. After the transit gateway enters the available state,     you can attach your VPCs and VPN connections to the transit gateway.
///
/// To attach your VPCs, use AWS::EC2::TransitGatewayAttachment.
///
/// To attach a VPN connection, use AWS::EC2::CustomerGateway to create a customer gateway and specify the ID of     the customer gateway and the ID of the transit gateway in a call to AWS::EC2::VPNConnection.
///
/// When you create a transit gateway, we create a default transit gateway route table and     use it as the default association route table and the default propagation route table. You     can use AWS::EC2::TransitGatewayRouteTable to create additional transit gateway route     tables. If you disable automatic route propagation, we do not create a default transit     gateway route table. You can use AWS::EC2::TransitGatewayRouteTablePropagation to propagate routes from a     resource attachment to a transit gateway route table. If you disable automatic     associations, you can use AWS::EC2::TransitGatewayRouteTableAssociation to associate a resource     attachment with a transit gateway route table.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTransitGateway {


    /// 
    /// A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range     is 64512 to 65534 for 16-bit ASNs. The default is 64512.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "AmazonSideAsn")]
    pub amazon_side_asn: Option<i64>,


    /// 
    /// The ID of the default association route table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AssociationDefaultRouteTableId")]
    pub association_default_route_table_id: Option<String>,


    /// 
    /// Enable or disable automatic acceptance of attachment requests. Disabled by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAcceptSharedAttachments")]
    pub auto_accept_shared_attachments: Option<TransitGatewayAutoAcceptSharedAttachmentsEnum>,


    /// 
    /// Enable or disable automatic association with the default association route table. Enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRouteTableAssociation")]
    pub default_route_table_association: Option<TransitGatewayDefaultRouteTableAssociationEnum>,


    /// 
    /// Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultRouteTablePropagation")]
    pub default_route_table_propagation: Option<TransitGatewayDefaultRouteTablePropagationEnum>,


    /// 
    /// The description of the transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// Enable or disable DNS support. Enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsSupport")]
    pub dns_support: Option<TransitGatewayDnsSupportEnum>,


    /// 
    /// Indicates whether multicast is enabled on the transit gateway
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: Replacement
    #[serde(rename = "MulticastSupport")]
    pub multicast_support: Option<TransitGatewayMulticastSupportEnum>,


    /// 
    /// The ID of the default propagation route table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PropagationDefaultRouteTableId")]
    pub propagation_default_route_table_id: Option<String>,


    /// 
    /// The tags for the transit gateway.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The transit gateway CIDR blocks.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TransitGatewayCidrBlocks")]
    pub transit_gateway_cidr_blocks: Option<Vec<String>>,


    /// 
    /// Enable or disable Equal Cost Multipath Protocol support. Enabled by default.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpnEcmpSupport")]
    pub vpn_ecmp_support: Option<TransitGatewayVpnEcmpSupportEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayDefaultRouteTableAssociationEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayDefaultRouteTableAssociationEnum {
    fn default() -> Self {
        TransitGatewayDefaultRouteTableAssociationEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayAutoAcceptSharedAttachmentsEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayAutoAcceptSharedAttachmentsEnum {
    fn default() -> Self {
        TransitGatewayAutoAcceptSharedAttachmentsEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayDefaultRouteTablePropagationEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayDefaultRouteTablePropagationEnum {
    fn default() -> Self {
        TransitGatewayDefaultRouteTablePropagationEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayDnsSupportEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayDnsSupportEnum {
    fn default() -> Self {
        TransitGatewayDnsSupportEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayMulticastSupportEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayMulticastSupportEnum {
    fn default() -> Self {
        TransitGatewayMulticastSupportEnum::Disable
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum TransitGatewayVpnEcmpSupportEnum {

    /// disable
    #[serde(rename = "disable")]
    Disable,

    /// enable
    #[serde(rename = "enable")]
    Enable,

}

impl Default for TransitGatewayVpnEcmpSupportEnum {
    fn default() -> Self {
        TransitGatewayVpnEcmpSupportEnum::Disable
    }
}


impl cfn_resources::CfnResource for CfnTransitGateway {
    fn type_string() -> &'static str {
        "AWS::EC2::TransitGateway"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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


