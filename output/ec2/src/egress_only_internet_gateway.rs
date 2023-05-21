

/// [IPv6 only] Specifies an egress-only internet gateway for your VPC. An egress-only     internet gateway is used to enable outbound communication over IPv6 from instances in your     VPC to the internet, and prevents hosts outside of your VPC from initiating an IPv6     connection with your instance.
#[derive(Default, serde::Serialize)]
pub struct CfnEgressOnlyInternetGateway {


    /// 
    /// The ID of the VPC for which to create the egress-only internet gateway.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}