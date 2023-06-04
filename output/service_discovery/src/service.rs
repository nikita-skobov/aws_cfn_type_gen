/// A complex type that contains information about a service, which defines the configuration of the following  entities:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnService {
    ///
    /// The description of the service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// A complex type that contains information about the Route 53 DNS records that you want  AWS Cloud Map to create when you register an instance.
    ///
    /// ImportantThe record types of a service can only be changed by deleting the service and recreating it   with a new Dnsconfig.
    ///
    /// Required: No
    ///
    /// Type: DnsConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,

    ///
    /// Public DNS and HTTP namespaces only. A complex type that contains  settings for an optional health check. If you specify settings for a health check, AWS Cloud Map  associates the health check with the records that you specify in DnsConfig.
    ///
    /// For information about the charges for health checks, see Amazon Route 53 Pricing.
    ///
    /// Required: No
    ///
    /// Type: HealthCheckConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,

    ///
    /// A complex type that contains information about an optional custom health check.
    ///
    /// ImportantIf you specify a health check configuration, you can specify either   HealthCheckCustomConfig or HealthCheckConfig but not both.
    ///
    /// Required: No
    ///
    /// Type: HealthCheckCustomConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,

    ///
    /// The name of the service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Pattern: ((?=^.{1,127}$)^([a-zA-Z0-9_][a-zA-Z0-9-_]{0,61}[a-zA-Z0-9_]|[a-zA-Z0-9])(\.([a-zA-Z0-9_][a-zA-Z0-9-_]{0,61}[a-zA-Z0-9_]|[a-zA-Z0-9]))*$)|(^\.$)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the namespace that was used to create the service.
    ///
    /// ImportantYou must specify a value for NamespaceId either for the service properties or for DnsConfig.   Don't specify a value in both places.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<cfn_resources::StrVal>,

    ///
    /// The tags for the service. Each tag consists of a key and an optional value, both of which you define. Tag keys  can have a maximum character length of 128 characters, and tag values can have a maximum length of 256  characters.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// If present, specifies that the service instances are only discoverable using the   DiscoverInstances API operation. No DNS records is registered for the service  instances. The only valid value is HTTP.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<ServiceTypeEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnServicearn,

    #[serde(skip_serializing)]
    pub att_id: CfnServiceid,

    #[serde(skip_serializing)]
    pub att_name: CfnServicename,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ServiceTypeEnum {
    /// HTTP
    #[serde(rename = "HTTP")]
    Http,
}

impl Default for ServiceTypeEnum {
    fn default() -> Self {
        ServiceTypeEnum::Http
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServicearn;
impl CfnServicearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceid;
impl CfnServiceid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServicename;
impl CfnServicename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnService {
    fn type_string(&self) -> &'static str {
        "AWS::ServiceDiscovery::Service"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        self.dns_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.health_check_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.health_check_custom_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.namespace_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'namespace_id'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// A complex type that contains information about the Amazon Route 53 DNS records that you want  AWS Cloud Map to create when you register an instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DnsConfig {
    ///
    /// An array that contains one DnsRecord object for each Route 53 DNS record that you  want AWS Cloud Map to create when you register an instance.
    ///
    /// Required: Yes
    ///
    /// Type: List of DnsRecord
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsRecords")]
    pub dns_records: Vec<DnsRecord>,

    ///
    /// The ID of the namespace to use for DNS configuration.
    ///
    /// ImportantYou must specify a value for NamespaceId either for DnsConfig or for the service   properties. Don't specify a value in both places.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<cfn_resources::StrVal>,

    ///
    /// The routing policy that you want to apply to all Route 53 DNS records that AWS Cloud Map creates  when you register an instance and specify this service.
    ///
    /// NoteIf you want to use this service to register instances that create alias records, specify   WEIGHTED for the routing policy.
    ///
    /// You can specify the following values:
    ///
    /// MULTIVALUE                  If you define a health check for the service and the health check is healthy, Route 53    returns the applicable value for up to eight instances.          For example, suppose that the service includes configurations for one A    record and a health check. You use the service to register 10 instances. Route 53 responds to DNS    queries with IP addresses for up to eight healthy instances. If fewer than eight instances are    healthy, Route 53 responds to every DNS query with the IP addresses for all of the healthy    instances.          If you don't define a health check for the service, Route 53 assumes that all instances are    healthy and returns the values for up to eight instances.          For more information about the multivalue routing policy, see Multivalue    Answer Routing in the Route 53 Developer Guide.                       WEIGHTED                  Route 53 returns the applicable value from one randomly selected instance from among the    instances that you registered using the same service. Currently, all records have the same    weight, so you can't route more or less traffic to any instances.          For example, suppose that the service includes configurations for one A    record and a health check. You use the service to register 10 instances. Route 53 responds to DNS    queries with the IP address for one randomly selected instance from among the healthy    instances. If no instances are healthy, Route 53 responds to DNS queries as if all of the    instances were healthy.          If you don't define a health check for the service, Route 53 assumes that all instances are    healthy and returns the applicable value for one randomly selected instance.          For more information about the weighted routing policy, see Weighted    Routing in the Route 53 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: MULTIVALUE | WEIGHTED
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoutingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy: Option<DnsConfigRoutingPolicyEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DnsConfigRoutingPolicyEnum {
    /// MULTIVALUE
    #[serde(rename = "MULTIVALUE")]
    Multivalue,

    /// WEIGHTED
    #[serde(rename = "WEIGHTED")]
    Weighted,
}

impl Default for DnsConfigRoutingPolicyEnum {
    fn default() -> Self {
        DnsConfigRoutingPolicyEnum::Multivalue
    }
}

impl cfn_resources::CfnResource for DnsConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.namespace_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'namespace_id'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A complex type that contains information about the Route 53 DNS records that you want  AWS Cloud Map to create when you register an instance.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DnsRecord {
    ///
    /// The amount of time, in seconds, that you want DNS resolvers to cache the settings for this  record.
    ///
    /// NoteAlias records don't include a TTL because Route 53 uses the TTL for the AWS resource that an   alias record routes traffic to. If you include the AWS_ALIAS_DNS_NAME attribute   when you submit a RegisterInstance request, the   TTL value is ignored. Always specify a TTL for the service; you can use a service   to register instances that create either alias or non-alias records.
    ///
    /// Required: Yes
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TTL")]
    pub ttl: f64,

    ///
    /// The type of the resource, which indicates the type of value that Route 53 returns in response to DNS queries.  You can specify values for Type in the following combinations:
    ///
    /// A     AAAA     A and AAAA     SRV     CNAME
    ///
    /// If you want AWS Cloud Map to create a Route 53 alias record when you register an instance, specify   A or AAAA for Type.
    ///
    /// You specify other settings, such as the IP address for A and AAAA records, when you  register an instance. For more information, see RegisterInstance.
    ///
    /// The following values are supported:
    ///
    /// A      Route 53 returns the IP address of the resource in IPv4 format, such as 192.0.2.44.        AAAA      Route 53 returns the IP address of the resource in IPv6 format, such as    2001:0db8:85a3:0000:0000:abcd:0001:2345.        CNAME      Route 53 returns the domain name of the resource, such as www.example.com. Note the following:                      You specify the domain name that you want to route traffic to when you register an instance. For more     information, see Attributes in the topic RegisterInstance.        You must specify WEIGHTED for the value of RoutingPolicy.        You can't specify both CNAME for Type and settings for     HealthCheckConfig. If you do, the request will fail with an InvalidInput     error.           SRV      Route 53 returns the value for an SRV record. The value for an SRV record uses the    following values:   priority weight port service-hostname   Note the following about the values:                      The values of priority and weight are both set to 1 and can't be     changed.        The value of port comes from the value that you specify for the AWS_INSTANCE_PORT     attribute when you submit a RegisterInstance request.        The value of service-hostname is a concatenation of the following values:                            The value that you specify for InstanceId when you register an instance.          The name of the service.          The name of the namespace.         For example, if the value of InstanceId is test, the name of the service is     backend, and the name of the namespace is example.com, the value of     service-hostname is:    test.backend.example.com       If you specify settings for an SRV record and if you specify values for    AWS_INSTANCE_IPV4, AWS_INSTANCE_IPV6, or both in the RegisterInstance    request, AWS Cloud Map automatically creates A and/or AAAA records that have the    same name as the value of service-hostname in the SRV record. You can ignore these    records.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: A | AAAA | CNAME | SRV
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: DnsRecordTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DnsRecordTypeEnum {
    /// A
    #[serde(rename = "A")]
    A,

    /// AAAA
    #[serde(rename = "AAAA")]
    Aaaa,

    /// CNAME
    #[serde(rename = "CNAME")]
    Cname,

    /// SRV
    #[serde(rename = "SRV")]
    Srv,
}

impl Default for DnsRecordTypeEnum {
    fn default() -> Self {
        DnsRecordTypeEnum::A
    }
}

impl cfn_resources::CfnResource for DnsRecord {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Public DNS and HTTP namespaces only. A complex type that contains  settings for an optional health check. If you specify settings for a health check, AWS Cloud Map  associates the health check with the records that you specify in DnsConfig.
///
/// Health checks are basic Route 53 health checks that monitor an AWS endpoint. For  information about pricing for health checks, see Amazon Route 53 Pricing.
///
/// Note the following about configuring health checks.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HealthCheckConfig {
    ///
    /// The number of consecutive health checks that an endpoint must pass or fail for Route 53 to  change the current status of the endpoint from unhealthy to healthy or the other way around. For  more information, see How Route 53   Determines Whether an Endpoint Is Healthy in the  Route 53 Developer Guide.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<f64>,

    ///
    /// The path that you want Route 53 to request when performing health checks. The path can be any  value that your endpoint returns an HTTP status code of a 2xx or 3xx format for when the endpoint  is healthy. An example file is /docs/route53-health-check.html. Route 53 automatically  adds the DNS name for the service. If you don't specify a value for ResourcePath,  the default value is /.
    ///
    /// If you specify TCP for Type, you must not  specify a value for ResourcePath.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<cfn_resources::StrVal>,

    ///
    /// The type of health check that you want to create, which indicates how Route 53 determines  whether an endpoint is healthy.
    ///
    /// ImportantYou can't change the value of Type after you create a health check.
    ///
    /// You can create the following types of health checks:
    ///
    /// HTTP: Route 53 tries to establish a TCP connection. If   successful, Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater   and less than 400.                        HTTPS: Route 53 tries to establish a TCP connection. If   successful, Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater   and less than 400.        ImportantIf you specify HTTPS for the value of Type, the endpoint must support TLS    v1.0 or later.                        TCP: Route 53 tries to establish a TCP connection.        If you specify TCP for Type, don't specify a value for    ResourcePath.
    ///
    /// For more information, see How Route 53   Determines Whether an Endpoint Is Healthy in the  Route 53 Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: HTTP | HTTPS | TCP
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: HealthCheckConfigTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum HealthCheckConfigTypeEnum {
    /// HTTP
    #[serde(rename = "HTTP")]
    Http,

    /// HTTPS
    #[serde(rename = "HTTPS")]
    Https,

    /// TCP
    #[serde(rename = "TCP")]
    Tcp,
}

impl Default for HealthCheckConfigTypeEnum {
    fn default() -> Self {
        HealthCheckConfigTypeEnum::Http
    }
}

impl cfn_resources::CfnResource for HealthCheckConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.failure_threshold {
            if *the_val > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'failure_threshold'. {} is greater than 10",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.failure_threshold {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'failure_threshold'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.resource_path {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'resource_path'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// A complex type that contains information about an optional custom health check. A custom  health check, which requires that you use a third-party health checker to evaluate the health of  your resources, is useful in the following circumstances:
///
/// To change the status of a custom health check, submit an   UpdateInstanceCustomHealthStatus request. AWS Cloud Map doesn't monitor the status of the  resource, it just keeps a record of the status specified in the most recent   UpdateInstanceCustomHealthStatus request.
///
/// Here's how custom health checks work:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HealthCheckCustomConfig {
    ///
    /// ImportantThis parameter is no longer supported and is always set to 1. AWS Cloud Map waits for   approximately 30 seconds after receiving an UpdateInstanceCustomHealthStatus   request before changing the status of the service instance.
    ///
    /// The number of 30-second intervals that you want AWS Cloud Map to wait after receiving an   UpdateInstanceCustomHealthStatus request before it changes the health status of a  service instance.
    ///
    /// Sending a second or subsequent UpdateInstanceCustomHealthStatus request with  the same value before 30 seconds has passed doesn't accelerate the change. AWS Cloud Map still waits   30 seconds after the first request to make the change.
    ///
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Minimum: 1
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<f64>,
}

impl cfn_resources::CfnResource for HealthCheckCustomConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.failure_threshold {
            if *the_val > 10 as _ {
                return Err(format!(
                    "Max validation failed on field 'failure_threshold'. {} is greater than 10",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.failure_threshold {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'failure_threshold'. {} is less than 1",
                    the_val
                ));
            }
        }

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
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
