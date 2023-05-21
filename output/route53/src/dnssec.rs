

/// The AWS::Route53::DNSSEC resource is used to enable DNSSEC signing in a hosted zone.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDNSSEC {


    /// 
    /// A unique string (ID) that is used to identify a hosted zone. For example: Z00001111A1ABCaaABC11.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: String,

}

impl cfn_resources::CfnResource for CfnDNSSEC {
    fn type_string() -> &'static str {
        "AWS::Route53::DNSSEC"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
