

/// In IPAM, an allocation is a CIDR assignment from an IPAM pool to another IPAM pool or to a resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIPAMAllocation {


    /// 
    /// The CIDR you would like to allocate from the IPAM pool. Note the following:
    /// 
    /// If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.               If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.
    /// 
    /// Possible values: Any available IPv4 or IPv6 CIDR.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Cidr")]
    pub cidr: Option<String>,


    /// 
    /// The ID of the IPAM pool from which you would like to allocate a CIDR.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamPoolId")]
    pub ipam_pool_id: String,


    /// 
    /// The netmask length of the CIDR you would like to allocate from the IPAM pool. Note the following:
    /// 
    /// If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.               If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.
    /// 
    /// Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetmaskLength")]
    pub netmask_length: Option<i64>,


    /// 
    /// A description for the allocation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    pub description: Option<String>,

}



impl cfn_resources::CfnResource for CfnIPAMAllocation {
    fn type_string() -> &'static str {
        "AWS::EC2::IPAMAllocation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
