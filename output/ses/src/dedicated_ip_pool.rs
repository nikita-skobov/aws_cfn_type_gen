

/// Create a new pool of dedicated IP addresses. A pool can include one or more dedicated       IP addresses that are associated with your AWS account. You can       associate a pool with a configuration set. When you send an email that uses that       configuration set, the message is sent from one of the addresses in the associated       pool.
#[derive(Default, serde::Serialize)]
pub struct CfnDedicatedIpPool {


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
    pub scaling_mode: Option<String>,


    /// 
    /// The name of the dedicated IP pool that the IP address is associated with.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PoolName")]
    pub pool_name: Option<String>,

}
