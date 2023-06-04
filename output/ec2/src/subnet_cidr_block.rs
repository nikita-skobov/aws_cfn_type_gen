/// Associates a CIDR block with your subnet. You can associate a single IPv6 CIDR block     with your subnet. An IPv6 CIDR block must have a prefix length of /64.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSubnetCidrBlock {
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
    pub ipv6_cidr_block: cfn_resources::StrVal,

    ///
    /// The ID of the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnSubnetCidrBlock {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::SubnetCidrBlock"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
