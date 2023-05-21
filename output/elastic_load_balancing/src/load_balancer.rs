/// Specifies a Classic Load Balancer.
///
/// You can specify the AvailabilityZones or Subnets property, but not both.
///
/// If this resource has a public IP address and is also in a VPC that is defined in the same template, you must use     the DependsOn attribute    to declare a dependency on the VPC-gateway attachment.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnLoadBalancer {
    ///
    /// Information about where and how access logs are stored for the load balancer.
    ///
    /// Required: No
    ///
    /// Type: AccessLoggingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessLoggingPolicy")]
    pub access_logging_policy: Option<AccessLoggingPolicy>,

    ///
    /// Information about a policy for application-controlled session stickiness.
    ///
    /// Required: No
    ///
    /// Type: List of AppCookieStickinessPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppCookieStickinessPolicy")]
    pub app_cookie_stickiness_policy: Option<Vec<AppCookieStickinessPolicy>>,

    ///
    /// The Availability Zones for the load balancer. For load balancers in a VPC, specify Subnets instead.
    ///
    /// Update requires replacement if you did not previously specify an Availability Zone or if you are removing all Availability Zones.    Otherwise, update requires no interruption.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,

    ///
    /// If enabled, the load balancer allows existing requests to complete before the load balancer shifts traffic away from a deregistered or unhealthy instance.
    ///
    /// For more information, see Configure Connection Draining       in the Classic Load Balancers Guide.
    ///
    /// Required: No
    ///
    /// Type: ConnectionDrainingPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionDrainingPolicy")]
    pub connection_draining_policy: Option<ConnectionDrainingPolicy>,

    ///
    /// If enabled, the load balancer allows the connections to remain idle (no data is sent over the connection) for the specified duration.
    ///
    /// By default, Elastic Load Balancing maintains a 60-second idle connection timeout for both front-end and back-end connections of your load balancer.       For more information, see Configure Idle Connection Timeout       in the Classic Load Balancers Guide.
    ///
    /// Required: No
    ///
    /// Type: ConnectionSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionSettings")]
    pub connection_settings: Option<ConnectionSettings>,

    ///
    /// If enabled, the load balancer routes the request traffic evenly across all instances regardless of the Availability Zones.
    ///
    /// For more information, see Configure Cross-Zone Load Balancing       in the Classic Load Balancers Guide.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "CrossZone")]
    pub cross_zone: Option<bool>,

    ///
    /// The health check settings to use when evaluating the health of your EC2 instances.
    ///
    /// Update requires replacement if you did not previously specify health check settings or if you are removing the health check settings.    Otherwise, update requires no interruption.
    ///
    /// Required: No
    ///
    /// Type: HealthCheck
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheck>,

    ///
    /// The IDs of the instances for the load balancer.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Instances")]
    pub instances: Option<Vec<String>>,

    ///
    /// Information about a policy for duration-based session stickiness.
    ///
    /// Required: No
    ///
    /// Type: List of LBCookieStickinessPolicy
    ///
    /// Update requires: No interruption
    #[serde(rename = "LBCookieStickinessPolicy")]
    pub lbcookie_stickiness_policy: Option<Vec<LBCookieStickinessPolicy>>,

    ///
    /// The listeners for the load balancer. You can specify at most one listener per port.
    ///
    /// If you update the properties for a listener, AWS CloudFormation deletes the existing listener and     creates a new one with the specified properties. While the new listener is being created, clients     cannot connect to the load balancer.
    ///
    /// Required: Yes
    ///
    /// Type: List of Listeners
    ///
    /// Update requires: No interruption
    #[serde(rename = "Listeners")]
    pub listeners: Vec<Listeners>,

    ///
    /// The name of the load balancer. This name must be unique within your set of load balancers for the region.
    ///
    /// If you don't specify a name, AWS CloudFormation generates a unique physical ID for the load balancer.    For more information, see Name Type.    If you specify a name, you cannot perform updates that require replacement of this resource, but you can perform     other updates. To replace the resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: Option<String>,

    ///
    /// The policies defined for your Classic Load Balancer. Specify only back-end server policies.
    ///
    /// Required: No
    ///
    /// Type: List of Policies
    ///
    /// Update requires: No interruption
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policies>>,

    ///
    /// The type of load balancer. Valid only for load balancers in a VPC.
    ///
    /// If Scheme is internet-facing, the load balancer      has a public DNS name that resolves to a public IP address.
    ///
    /// If Scheme is internal, the load balancer has a public       DNS name that resolves to a private IP address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,

    ///
    /// The security groups for the load balancer. Valid only for load balancers in a VPC.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The IDs of the subnets for the load balancer. You can specify at most one subnet per Availability Zone.
    ///
    /// Update requires replacement if you did not previously specify a subnet or if you are removing all subnets.    Otherwise, update requires no interruption. To update to a different subnet in the current Availability Zone,    you must first update to a subnet in a different Availability Zone, then update to the new subnet in the original    Availability Zone.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,

    ///
    /// The tags associated with a load balancer.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnLoadBalancer {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticLoadBalancing::LoadBalancer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        self.access_logging_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.connection_draining_policy
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.connection_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.health_check
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies where and how access logs are stored for your Classic Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AccessLoggingPolicy {
    ///
    /// The interval for publishing the access logs. You can specify an interval of either 5 minutes or 60 minutes.
    ///
    /// Default: 60 minutes
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "EmitInterval")]
    pub emit_interval: Option<i64>,

    ///
    /// Specifies whether access logs are enabled for the load balancer.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The name of the Amazon S3 bucket where the access logs are stored.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,

    ///
    /// The logical hierarchy you created for your Amazon S3 bucket, for example my-bucket-prefix/prod.        If the prefix is not provided, the log is placed at the root level of the bucket.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3BucketPrefix")]
    pub s3_bucket_prefix: Option<String>,
}

impl cfn_resources::CfnResource for AccessLoggingPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies a policy for application-controlled session stickiness for your Classic Load Balancer.
///
/// To associate a policy with a listener, use the PolicyNames     property for the listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AppCookieStickinessPolicy {
    ///
    /// The name of the application cookie used for stickiness.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CookieName")]
    pub cookie_name: String,

    ///
    /// The mnemonic name for the policy being created. The name must be unique within a set of policies for this load balancer.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

impl cfn_resources::CfnResource for AppCookieStickinessPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the connection draining settings for your Classic Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionDrainingPolicy {
    ///
    /// Specifies whether connection draining is enabled for the load balancer.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    ///
    /// The maximum time, in seconds, to keep the existing connections open before deregistering the instances.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}

impl cfn_resources::CfnResource for ConnectionDrainingPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies the idle timeout value for your Classic Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionSettings {
    ///
    /// The time, in seconds, that the connection is allowed to be idle (no data has been sent over the connection) before it is closed by the load balancer.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 3600
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdleTimeout")]
    pub idle_timeout: i64,
}

impl cfn_resources::CfnResource for ConnectionSettings {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.idle_timeout;

        if *the_val > 3600 as _ {
            return Err(format!(
                "Max validation failed on field 'idle_timeout'. {} is greater than 3600",
                the_val
            ));
        }

        let the_val = &self.idle_timeout;

        if *the_val < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'idle_timeout'. {} is less than 1",
                the_val
            ));
        }

        Ok(())
    }
}

/// Specifies health check settings for your Classic Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheck {
    ///
    /// The number of consecutive health checks successes required before moving the instance to the Healthy state.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: String,

    ///
    /// The approximate interval, in seconds, between health checks of an individual instance.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 5
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "Interval")]
    pub interval: String,

    ///
    /// The instance being checked. The protocol is either TCP, HTTP, HTTPS, or SSL. The range of valid ports is one (1) through 65535.
    ///
    /// TCP is the default, specified as a TCP: port pair, for example "TCP:5000". In this case, a health check simply attempts to open a TCP connection to the instance on the specified port. Failure to connect within the configured timeout is considered unhealthy.
    ///
    /// SSL is also specified as SSL: port pair, for example, SSL:5000.
    ///
    /// For HTTP/HTTPS, you must include a ping path in the string. HTTP is specified as a HTTP:port;/;PathToPing; grouping, for example "HTTP:80/weather/us/wa/seattle". In this case, a HTTP GET request is issued to the instance on the given port and path. Any answer other than "200 OK" within the timeout period is considered unhealthy.
    ///
    /// The total length of the HTTP ping target must be 1024 16-bit Unicode characters or less.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Target")]
    pub target: String,

    ///
    /// The amount of time, in seconds, during which no response means a failed health check.
    ///
    /// This value must be less than the Interval value.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 60
    ///
    /// Update requires: No interruption
    #[serde(rename = "Timeout")]
    pub timeout: String,

    ///
    /// The number of consecutive health check failures required before moving the instance to the Unhealthy state.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: String,
}

impl cfn_resources::CfnResource for HealthCheck {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.healthy_threshold;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'healthy_threshold'. {} is greater than 10",
                the_val.len()
            ));
        }

        let the_val = &self.healthy_threshold;

        if the_val.len() < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'healthy_threshold'. {} is less than 2",
                the_val.len()
            ));
        }

        let the_val = &self.interval;

        if the_val.len() > 300 as _ {
            return Err(format!(
                "Max validation failed on field 'interval'. {} is greater than 300",
                the_val.len()
            ));
        }

        let the_val = &self.interval;

        if the_val.len() < 5 as _ {
            return Err(format!(
                "Min validation failed on field 'interval'. {} is less than 5",
                the_val.len()
            ));
        }

        let the_val = &self.timeout;

        if the_val.len() > 60 as _ {
            return Err(format!(
                "Max validation failed on field 'timeout'. {} is greater than 60",
                the_val.len()
            ));
        }

        let the_val = &self.timeout;

        if the_val.len() < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'timeout'. {} is less than 2",
                the_val.len()
            ));
        }

        let the_val = &self.unhealthy_threshold;

        if the_val.len() > 10 as _ {
            return Err(format!(
                "Max validation failed on field 'unhealthy_threshold'. {} is greater than 10",
                the_val.len()
            ));
        }

        let the_val = &self.unhealthy_threshold;

        if the_val.len() < 2 as _ {
            return Err(format!(
                "Min validation failed on field 'unhealthy_threshold'. {} is less than 2",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Specifies a policy for duration-based session stickiness for your Classic Load Balancer.
///
/// To associate a policy with a listener, use the PolicyNames     property for the listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LBCookieStickinessPolicy {
    ///
    /// The time period, in seconds, after which the cookie should be considered stale. If this parameter is not specified, the stickiness session lasts for the duration of the browser session.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CookieExpirationPeriod")]
    pub cookie_expiration_period: Option<String>,

    ///
    /// The name of the policy. This name must be unique within the set of policies for this load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,
}

impl cfn_resources::CfnResource for LBCookieStickinessPolicy {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Specifies a listener for your Classic Load Balancer.
///
/// Modifying any property replaces the listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Listeners {
    ///
    /// The port on which the instance is listening.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstancePort")]
    pub instance_port: String,

    ///
    /// The protocol to use for routing traffic to instances: HTTP, HTTPS, TCP, or SSL.
    ///
    /// If the front-end protocol is TCP or SSL, the back-end protocol must be TCP or SSL.       If the front-end protocol is HTTP or HTTPS, the back-end protocol must be HTTP or HTTPS.
    ///
    /// If there is another listener with the same InstancePort whose InstanceProtocol is secure,      (HTTPS or SSL), the listener's InstanceProtocol must also be secure.
    ///
    /// If there is another listener with the same InstancePort whose InstanceProtocol is HTTP or TCP,      the listener's InstanceProtocol must be HTTP or TCP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceProtocol")]
    pub instance_protocol: Option<String>,

    ///
    /// The port on which the load balancer is listening. On EC2-VPC, you can specify any port from the range 1-65535. On EC2-Classic, you can specify any port from the following list: 25, 80, 443, 465, 587, 1024-65535.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "LoadBalancerPort")]
    pub load_balancer_port: String,

    ///
    /// The names of the policies to associate with the listener.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "PolicyNames")]
    pub policy_names: Option<Vec<String>>,

    ///
    /// The load balancer transport protocol to use for routing: HTTP, HTTPS, TCP, or SSL.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "Protocol")]
    pub protocol: String,

    ///
    /// The Amazon Resource Name (ARN) of the server certificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SSLCertificateId")]
    pub sslcertificate_id: Option<String>,
}

impl cfn_resources::CfnResource for Listeners {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.instance_port;

        if the_val.len() > 65535 as _ {
            return Err(format!(
                "Max validation failed on field 'instance_port'. {} is greater than 65535",
                the_val.len()
            ));
        }

        let the_val = &self.instance_port;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'instance_port'. {} is less than 1",
                the_val.len()
            ));
        }

        Ok(())
    }
}

/// Specifies policies for your Classic Load Balancer.
///
/// To associate policies with a listener, use the PolicyNames     property for the listener.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Policies {
    ///
    /// The policy attributes.
    ///
    /// Required: Yes
    ///
    /// Type: List of Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Attributes")]
    pub attributes: Vec<serde_json::Value>,

    ///
    /// The instance ports for the policy. Required only for some policy types.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "InstancePorts")]
    pub instance_ports: Option<Vec<String>>,

    ///
    /// The load balancer ports for the policy. Required only for some policy types.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerPorts")]
    pub load_balancer_ports: Option<Vec<String>>,

    ///
    /// The name of the policy.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

    ///
    /// The name of the policy type.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
}

impl cfn_resources::CfnResource for Policies {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
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

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
