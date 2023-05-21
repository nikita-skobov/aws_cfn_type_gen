

/// The AWS::Lightsail::StaticIp resource specifies a static IP that can be     attached to an Amazon Lightsail instance that is in the same AWS Region and Availability Zone.
#[derive(Default, serde::Serialize)]
pub struct CfnStaticIp {


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

}
