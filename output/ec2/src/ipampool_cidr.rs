

/// A CIDR provisioned to an IPAM pool.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIPAMPoolCidr {


    /// 
    /// The netmask length of the CIDR you'd like to provision to a pool. Can be used for provisioning Amazon-provided IPv6 CIDRs to top-level pools and for provisioning CIDRs to pools with source pools. Cannot be used to provision BYOIP CIDRs to top-level pools. "NetmaskLength" or "Cidr" is required.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetmaskLength")]
    pub netmask_length: Option<i64>,


    /// 
    /// The ID of the IPAM pool.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamPoolId")]
    pub ipam_pool_id: String,


    /// 
    /// The CIDR provisioned to the IPAM pool. A CIDR is a representation of an IP address and its associated network mask (or netmask)      and refers to a range of IP addresses. An IPv4 CIDR example is 10.24.34.0/23. An IPv6 CIDR example is 2001:DB8::/32.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,

}



impl cfn_resources::CfnResource for CfnIPAMPoolCidr {
    fn type_string() -> &'static str {
        "AWS::EC2::IPAMPoolCidr"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
