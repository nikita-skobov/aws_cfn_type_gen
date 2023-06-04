/// [IPv6 only] Specifies an egress-only internet gateway for your VPC. An egress-only     internet gateway is used to enable outbound communication over IPv6 from instances in your     VPC to the internet, and prevents hosts outside of your VPC from initiating an IPv6     connection with your instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub vpc_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_id: CfnEgressOnlyInternetGatewayid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnEgressOnlyInternetGatewayid;
impl CfnEgressOnlyInternetGatewayid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnEgressOnlyInternetGateway {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::EgressOnlyInternetGateway"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
