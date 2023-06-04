/// Create a new pool of dedicated IP addresses. A pool can include one or more dedicated       IP addresses that are associated with your AWS account. You can       associate a pool with a configuration set. When you send an email that uses that       configuration set, the message is sent from one of the addresses in the associated       pool.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDedicatedIpPool {
    ///
    /// The name of the dedicated IP pool that the IP address is associated with.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PoolName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub pool_name: Option<cfn_resources::StrVal>,

    ///
    /// The type of scaling mode.
    ///
    /// The following options are available:
    ///
    /// STANDARD - The customer controls which IPs are part of the           dedicated IP pool.               MANAGED - The reputation and number of IPs is automatically           managed by Amazon SES.
    ///
    /// The STANDARD option is selected by default if no value is       specified.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ScalingMode")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub scaling_mode: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for CfnDedicatedIpPool {
    fn type_string(&self) -> &'static str {
        "AWS::SES::DedicatedIpPool"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
