/// The AWS::Lightsail::StaticIp resource specifies a static IP that can be     attached to an Amazon Lightsail instance that is in the same AWS Region and Availability Zone.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnStaticIp {
    ///
    /// The instance that the static IP is attached to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachedTo")]
    pub attached_to: Option<String>,

    ///
    /// The name of the static IP.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "StaticIpName")]
    pub static_ip_name: String,
}

impl cfn_resources::CfnResource for CfnStaticIp {
    fn type_string(&self) -> &'static str {
        "AWS::Lightsail::StaticIp"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
