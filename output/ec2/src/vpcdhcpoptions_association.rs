

/// Associates a set of DHCP options with a VPC, or associates no DHCP options with the     VPC.
///
/// After you associate the options with the VPC, any existing instances and all new     instances that you launch in that VPC use the options. You don't need to restart or     relaunch the instances. They automatically pick up the changes within a few hours,     depending on how frequently the instance renews its DHCP lease. You can explicitly renew     the lease using the operating system on the instance.
#[derive(Default, serde::Serialize)]
pub struct CfnVPCDHCPOptionsAssociation {


    /// 
    /// The ID of the VPC.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,


    /// 
    /// The ID of the DHCP options set, or default to associate     no DHCP options with the VPC.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DhcpOptionsId")]
    pub dhcp_options_id: String,

}