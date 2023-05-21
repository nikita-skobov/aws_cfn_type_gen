

/// The AWS::Lightsail::LoadBalancer resource specifies a load balancer that     can be used with Lightsail instances.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoadBalancer {


    /// 
    /// The Lightsail instances to attach to the load balancer.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AttachedInstances")]
    pub attached_instances: Option<Vec<String>>,


    /// 
    /// The path on the attached instance where the health check will be performed. If no path     is specified, the load balancer tries to make a request to the default (root) page       (/index.html).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: .*\S.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,


    /// 
    /// The port that the load balancer uses to direct traffic to your Lightsail     instances. For HTTP traffic, specify port 80. For HTTPS traffic, specify port     443.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "InstancePort")]
    pub instance_port: i64,


    /// 
    /// The IP address type of the load balancer.
    /// 
    /// The possible values are ipv4 for IPv4 only, and dualstack for     both IPv4 and IPv6.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dualstack | ipv4
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<LoadBalancerIpAddressTypeEnum>,


    /// 
    /// The name of the load balancer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Pattern: \w[\w\-]*\w
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: String,


    /// 
    /// A Boolean value indicating whether session stickiness is enabled.
    /// 
    /// Enable session stickiness (also known as session affinity) to bind     a user's session to a specific instance. This ensures that all requests from the user     during the session are sent to the same instance.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionStickinessEnabled")]
    pub session_stickiness_enabled: Option<bool>,


    /// 
    /// The time period, in seconds, after which the load balancer session stickiness cookie     should be considered stale. If you do not specify this parameter, the default value is 0,     which indicates that the sticky session should last for the duration of the browser     session.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SessionStickinessLBCookieDurationSeconds")]
    pub session_stickiness_lbcookie_duration_seconds: Option<String>,


    /// 
    /// An array of key-value pairs to apply to this resource.
    /// 
    /// For more information, see Tag     in the AWS CloudFormation User Guide.
    /// 
    /// NoteThe Value of Tags is optional for Lightsail resources.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the TLS security policy for the load balancer.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: \w[\w\-]*\w
    ///
    /// Update requires: No interruption
    #[serde(rename = "TlsPolicyName")]
    pub tls_policy_name: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum LoadBalancerIpAddressTypeEnum {

    /// dualstack
    #[serde(rename = "dualstack")]
    Dualstack,

    /// ipv4
    #[serde(rename = "ipv4")]
    Ipv4,

}

impl Default for LoadBalancerIpAddressTypeEnum {
    fn default() -> Self {
        LoadBalancerIpAddressTypeEnum::Dualstack
    }
}


impl cfn_resources::CfnResource for CfnLoadBalancer {
    fn type_string() -> &'static str {
        "AWS::Lightsail::LoadBalancer"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


