
pub mod cfn_load_balancer {

#[derive(serde::Serialize, Default)]
pub struct CfnLoadBalancer {
    /// No documentation provided by AWS
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// List of LBCookieStickinessPolicy
    #[serde(rename = "LBCookieStickinessPolicy")]
    pub lbcookie_stickiness_policy: Option<Vec<LBCookieStickinessPolicy>>,
    /// No documentation provided by AWS
    #[serde(rename = "AvailabilityZones")]
    pub availability_zones: Option<Vec<String>>,
    /// List of Policies
    #[serde(rename = "Policies")]
    pub policies: Option<Vec<Policies>>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceSecurityGroupGroupName")]
    pub source_security_group_group_name: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "LoadBalancerName")]
    pub load_balancer_name: Option<String>,
    /// List of AppCookieStickinessPolicy
    #[serde(rename = "AppCookieStickinessPolicy")]
    pub app_cookie_stickiness_policy: Option<Vec<AppCookieStickinessPolicy>>,
    /// List of Listeners
    #[serde(rename = "Listeners")]
    pub listeners: Vec<Listeners>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionDrainingPolicy")]
    pub connection_draining_policy: Option<ConnectionDrainingPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "CrossZone")]
    pub cross_zone: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "AccessLoggingPolicy")]
    pub access_logging_policy: Option<AccessLoggingPolicy>,
    /// No documentation provided by AWS
    #[serde(rename = "Instances")]
    pub instances: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ConnectionSettings")]
    pub connection_settings: Option<ConnectionSettings>,
    /// No documentation provided by AWS
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheck>,
    /// No documentation provided by AWS
    #[serde(rename = "SourceSecurityGroupOwnerAlias")]
    pub source_security_group_owner_alias: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Subnets")]
    pub subnets: Option<Vec<String>>,
    /// List of Tag
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(serde::Serialize, Default)]
pub struct ConnectionDrainingPolicy {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Timeout")]
    pub timeout: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Policies {
    #[serde(rename = "InstancePorts")]
    pub instance_ports: Option<Vec<String>>,
    #[serde(rename = "LoadBalancerPorts")]
    pub load_balancer_ports: Option<Vec<String>>,
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "Attributes")]
    pub attributes: (),
    #[serde(rename = "PolicyType")]
    pub policy_type: String,

}

#[derive(serde::Serialize, Default)]
pub struct LBCookieStickinessPolicy {
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,
    #[serde(rename = "CookieExpirationPeriod")]
    pub cookie_expiration_period: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct HealthCheck {
    #[serde(rename = "Timeout")]
    pub timeout: String,
    #[serde(rename = "Interval")]
    pub interval: String,
    #[serde(rename = "UnhealthyThreshold")]
    pub unhealthy_threshold: String,
    #[serde(rename = "HealthyThreshold")]
    pub healthy_threshold: String,
    #[serde(rename = "Target")]
    pub target: String,

}

#[derive(serde::Serialize, Default)]
pub struct Listeners {
    #[serde(rename = "PolicyNames")]
    pub policy_names: Option<Vec<String>>,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "SSLCertificateId")]
    pub sslcertificate_id: Option<String>,
    #[serde(rename = "InstancePort")]
    pub instance_port: String,
    #[serde(rename = "InstanceProtocol")]
    pub instance_protocol: Option<String>,
    #[serde(rename = "LoadBalancerPort")]
    pub load_balancer_port: String,

}

#[derive(serde::Serialize, Default)]
pub struct AccessLoggingPolicy {
    #[serde(rename = "S3BucketPrefix")]
    pub s3_bucket_prefix: Option<String>,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    #[serde(rename = "EmitInterval")]
    pub emit_interval: Option<usize>,

}

#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}

#[derive(serde::Serialize, Default)]
pub struct ConnectionSettings {
    #[serde(rename = "IdleTimeout")]
    pub idle_timeout: usize,

}

#[derive(serde::Serialize, Default)]
pub struct AppCookieStickinessPolicy {
    #[serde(rename = "CookieName")]
    pub cookie_name: String,
    #[serde(rename = "PolicyName")]
    pub policy_name: String,

}


}
