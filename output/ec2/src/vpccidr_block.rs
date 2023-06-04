/// Associates a CIDR block with your VPC. You can only associate a single IPv6 CIDR block     with your VPC. The IPv6 CIDR block size is fixed at /56.
///
/// For more information about associating CIDR blocks with your VPC and applicable     restrictions, see VPC and Subnet Sizing in     the Amazon VPC User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnVPCCidrBlock {
    ///
    /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IPv6 addresses, or the size of the CIDR block.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AmazonProvidedIpv6CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_provided_ipv6_cidr_block: Option<bool>,

    ///
    /// An IPv4 CIDR block to associate with the VPC.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// Associate a CIDR allocated from an IPv4 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv4IpamPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_ipam_pool_id: Option<cfn_resources::StrVal>,

    ///
    /// The netmask length of the IPv4 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv4NetmaskLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_netmask_length: Option<i64>,

    ///
    /// An IPv6 CIDR block from the IPv6 address pool. You must also specify Ipv6Pool in the request.
    ///
    /// To let Amazon choose the IPv6 CIDR block for you, omit this parameter.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// Associates a CIDR allocated from an IPv6 IPAM pool to a VPC. For more information about Amazon VPC IP Address Manager (IPAM), see What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6IpamPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_ipam_pool_id: Option<cfn_resources::StrVal>,

    ///
    /// The netmask length of the IPv6 CIDR you would like to associate from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6NetmaskLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_netmask_length: Option<i64>,

    ///
    /// The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6Pool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_pool: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the VPC.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnVPCCidrBlock {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPCCidrBlock"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
