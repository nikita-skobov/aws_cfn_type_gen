

/// Allocates a fully dedicated physical server for launching EC2 instances. Because the     host is fully dedicated for your use, it can help you address compliance requirements and     reduce costs by allowing you to use your existing server-bound software licenses. For more     information, see Dedicated Hosts in     the Amazon EC2 User Guide for Linux Instances.
#[derive(Default, serde::Serialize)]
pub struct CfnHost {


    /// 
    /// The instance family supported by the Dedicated Host. For example,       m5.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceFamily")]
    pub instance_family: Option<String>,


    /// 
    /// Indicates whether host maintenance is enabled or disabled for the Dedicated       Host.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: off | on
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostMaintenance")]
    pub host_maintenance: Option<String>,


    /// 
    /// Indicates whether the host accepts any untargeted instance launches that match its       instance type configuration, or if it only accepts Host tenancy instance launches that       specify its unique host ID. For more information, see Understanding auto-placement and affinity in the         Amazon EC2 User Guide.
    /// 
    /// Default: on
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: off | on
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoPlacement")]
    pub auto_placement: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) of the AWS Outpost on which the       Dedicated Host is allocated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostArn")]
    pub outpost_arn: Option<String>,


    /// 
    /// Indicates whether to enable or disable host recovery for the Dedicated Host. Host       recovery is disabled by default. For more information, see Host recovery       in the Amazon EC2 User Guide.
    /// 
    /// Default: off
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: off | on
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostRecovery")]
    pub host_recovery: Option<String>,


    /// 
    /// The Availability Zone in which to allocate the Dedicated Host.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,


    /// 
    /// Specifies the instance type to be supported by the Dedicated Hosts. If you specify an     instance type, the Dedicated Hosts support instances of the specified instance type     only.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: Option<String>,

}
