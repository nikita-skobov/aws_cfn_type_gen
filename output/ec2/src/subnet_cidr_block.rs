

/// Associates a CIDR block with your subnet. You can associate a single IPv6 CIDR block     with your subnet. An IPv6 CIDR block must have a prefix length of /64.
#[derive(Default, serde::Serialize)]
pub struct CfnSubnetCidrBlock {


    /// 
    /// The ID of the subnet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,


    /// 
    /// The IPv6 network range for the subnet, in CIDR notation. The subnet size must use a       /64 prefix length.
    /// 
    /// This parameter is required for an IPv6 only subnet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: String,

}
