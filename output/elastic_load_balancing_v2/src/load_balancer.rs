/// Specifies an Application Load Balancer, a Network Load Balancer, or a Gateway Load     Balancer.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnLoadBalancer {
    ///
    /// The IP address type. The possible values are ipv4 (for IPv4 addresses) and     dualstack (for IPv4 and IPv6 addresses).    You can’t specify     dualstack for a load balancer with a UDP or TCP_UDP listener.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dualstack | ipv4
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpAddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<LoadBalancerIpAddressTypeEnum>,

    ///
    /// The load balancer attributes.
    ///
    /// Required: No
    ///
    /// Type: List of LoadBalancerAttribute
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<Vec<LoadBalancerAttribute>>,

    ///
    /// The name of the load balancer. This name must be unique per region per account, can have     a maximum of 32 characters, must contain only alphanumeric characters or hyphens, must not     begin or end with a hyphen, and must not begin with "internal-".
    ///
    /// If you don't specify a name, AWS CloudFormation generates a unique     physical ID for the load balancer. If you specify a name, you cannot perform updates that     require replacement of this resource, but you can perform other updates. To replace the     resource, specify a new name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The nodes of an Internet-facing load balancer have public IP addresses. The DNS name of an    Internet-facing load balancer is publicly resolvable to the public IP addresses of the nodes.    Therefore, Internet-facing load balancers can route requests from clients over the    internet.
    ///
    /// The nodes of an internal load balancer have only private IP addresses. The DNS name of an    internal load balancer is publicly resolvable to the private IP addresses of the nodes.    Therefore, internal load balancers can route requests only from clients with access to the VPC    for the load balancer.
    ///
    /// The default is an Internet-facing load balancer.
    ///
    /// You cannot specify a scheme for a Gateway Load Balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: internal | internet-facing
    ///
    /// Update requires: Replacement
    #[serde(rename = "Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<LoadBalancerSchemeEnum>,

    ///
    /// [Application Load Balancers] The IDs of the security groups for the load balancer.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    ///
    /// The IDs of the public subnets. You can specify only one subnet per Availability Zone. You    must specify either subnets or subnet mappings, but not both.
    ///
    /// [Application Load Balancers] You must specify subnets from at least two Availability    Zones. You cannot specify Elastic IP addresses for your subnets.
    ///
    /// [Application Load Balancers on Outposts] You must specify one Outpost subnet.
    ///
    /// [Application Load Balancers on Local Zones] You can specify subnets from one or more Local    Zones.
    ///
    /// [Network Load Balancers] You can specify subnets from one or more Availability Zones. You    can specify one Elastic IP address per subnet if you need static IP addresses for your    internet-facing load balancer. For internal load balancers, you can specify one private IP    address per subnet from the IPv4 range of the subnet. For internet-facing load balancer, you    can specify one IPv6 address per subnet.
    ///
    /// [Gateway Load Balancers] You can specify subnets from one or more Availability Zones. You    cannot specify Elastic IP addresses for your subnets.
    ///
    /// Required: Conditional
    ///
    /// Type: List of SubnetMapping
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<SubnetMapping>>,

    ///
    /// The IDs of the public subnets. You can specify only one subnet per Availability Zone. You    must specify either subnets or subnet mappings, but not both. To specify an Elastic IP    address, specify subnet mappings instead of subnets.
    ///
    /// [Application Load Balancers] You must specify subnets from at least two Availability    Zones.
    ///
    /// [Application Load Balancers on Outposts] You must specify one Outpost subnet.
    ///
    /// [Application Load Balancers on Local Zones] You can specify subnets from one or more Local    Zones.
    ///
    /// [Network Load Balancers] You can specify subnets from one or more Availability    Zones.
    ///
    /// [Gateway Load Balancers] You can specify subnets from one or more Availability    Zones.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,

    ///
    /// The tags to assign to the load balancer.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The type of load balancer. The default is application.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: application | gateway | network
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<LoadBalancerTypeEnum>,

    #[serde(skip_serializing)]
    pub att_canonical_hosted_zone_id: CfnLoadBalancercanonicalhostedzoneid,

    #[serde(skip_serializing)]
    pub att_dnsname: CfnLoadBalancerdnsname,

    #[serde(skip_serializing)]
    pub att_load_balancer_full_name: CfnLoadBalancerloadbalancerfullname,

    #[serde(skip_serializing)]
    pub att_load_balancer_name: CfnLoadBalancerloadbalancername,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum LoadBalancerSchemeEnum {
    /// internal
    #[serde(rename = "internal")]
    Internal,

    /// internet-facing
    #[serde(rename = "internet-facing")]
    Internetfacing,
}

impl Default for LoadBalancerSchemeEnum {
    fn default() -> Self {
        LoadBalancerSchemeEnum::Internal
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum LoadBalancerTypeEnum {
    /// application
    #[serde(rename = "application")]
    Application,

    /// gateway
    #[serde(rename = "gateway")]
    Gateway,

    /// network
    #[serde(rename = "network")]
    Network,
}

impl Default for LoadBalancerTypeEnum {
    fn default() -> Self {
        LoadBalancerTypeEnum::Application
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLoadBalancercanonicalhostedzoneid;
impl CfnLoadBalancercanonicalhostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"CanonicalHostedZoneID"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLoadBalancerdnsname;
impl CfnLoadBalancerdnsname {
    pub fn att_name(&self) -> &'static str {
        r#"DNSName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLoadBalancerloadbalancerfullname;
impl CfnLoadBalancerloadbalancerfullname {
    pub fn att_name(&self) -> &'static str {
        r#"LoadBalancerFullName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnLoadBalancerloadbalancername;
impl CfnLoadBalancerloadbalancername {
    pub fn att_name(&self) -> &'static str {
        r#"LoadBalancerName"#
    }
}

impl cfn_resources::CfnResource for CfnLoadBalancer {
    fn type_string(&self) -> &'static str {
        "AWS::ElasticLoadBalancingV2::LoadBalancer"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.load_balancer_attributes {
            if the_val.len() > 20 as _ {
                return Err(format!("Max validation failed on field 'load_balancer_attributes'. {} is greater than 20", the_val.len()));
            }
        }

        Ok(())
    }
}

/// Specifies an attribute for an Application Load Balancer, a Network Load Balancer, or a     Gateway Load Balancer.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LoadBalancerAttribute {
    ///
    /// The name of the attribute.
    ///
    /// The following attributes are supported by all load balancers:
    ///
    /// deletion_protection.enabled - Indicates whether deletion protection is      enabled. The value is true or false. The default is       false.                        load_balancing.cross_zone.enabled - Indicates whether cross-zone load      balancing is enabled. The possible values are true and false.      The default for Network Load Balancers and Gateway Load Balancers is false.      The default for Application Load Balancers is true, and cannot be changed.
    ///
    /// The following attributes are supported by both Application Load Balancers and Network Load    Balancers:
    ///
    /// access_logs.s3.enabled - Indicates whether access logs are enabled. The      value is true or false. The default is      false.                        access_logs.s3.bucket - The name of the S3 bucket for the access logs.      This attribute is required if access logs are enabled. The bucket must exist in the same      region as the load balancer and have a bucket policy that grants Elastic Load Balancing      permissions to write to the bucket.                        access_logs.s3.prefix - The prefix for the location in the S3 bucket for the      access logs.                        ipv6.deny_all_igw_traffic - Blocks internet gateway (IGW) access to the      load balancer. It is set to false for internet-facing load balancers and       true for internal load balancers, preventing unintended access to your      internal load balancer through an internet gateway.
    ///
    /// The following attributes are supported by only Application Load Balancers:
    ///
    /// idle_timeout.timeout_seconds - The idle timeout value, in seconds. The      valid range is 1-4000 seconds. The default is 60 seconds.                        routing.http.desync_mitigation_mode - Determines how the load balancer      handles requests that might pose a security risk to your application. The possible values      are monitor, defensive, and strictest. The default      is defensive.                        routing.http.drop_invalid_header_fields.enabled - Indicates whether HTTP      headers with invalid header fields are removed by the load balancer (true) or      routed to targets (false). The default is false.                        routing.http.preserve_host_header.enabled - Indicates whether the      Application Load Balancer should preserve the Host header in the HTTP request      and send it to the target without any change. The possible values are true      and false. The default is false.                        routing.http.x_amzn_tls_version_and_cipher_suite.enabled - Indicates      whether the two headers (x-amzn-tls-version and       x-amzn-tls-cipher-suite), which contain information about the negotiated      TLS version and cipher suite, are added to the client request before sending it to the      target. The x-amzn-tls-version header has information about the TLS protocol      version negotiated with the client, and the x-amzn-tls-cipher-suite header      has information about the cipher suite negotiated with the client. Both headers are in      OpenSSL format. The possible values for the attribute are true and       false. The default is false.                        routing.http.xff_client_port.enabled - Indicates whether the       X-Forwarded-For header should preserve the source port that the client used      to connect to the load balancer. The possible values are true and       false. The default is false.                        routing.http.xff_header_processing.mode - Enables you to modify,      preserve, or remove the X-Forwarded-For header in the HTTP request before the      Application Load Balancer sends the request to the target. The possible values are       append, preserve, and remove. The default is       append.                                                         If the value is append, the Application Load Balancer adds the client        IP address (of the last hop) to the X-Forwarded-For header in the HTTP        request before it sends it to targets.                     If the value is preserve the Application Load Balancer preserves the         X-Forwarded-For header in the HTTP request, and sends it to targets        without any change.                     If the value is remove, the Application Load Balancer removes the         X-Forwarded-For header in the HTTP request before it sends it to        targets.                                  routing.http2.enabled - Indicates whether HTTP/2 is enabled. The possible      values are true and false. The default is true.      Elastic Load Balancing requires that message header names contain only alphanumeric      characters and hyphens.                        waf.fail_open.enabled - Indicates whether to allow a WAF-enabled load      balancer to route requests to targets if it is unable to forward the request to AWS WAF. The possible values are true and false. The      default is false.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: ^[a-zA-Z0-9._]+$
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// The value of the attribute.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for LoadBalancerAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'value'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Specifies a subnet for a load balancer.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SubnetMapping {
    ///
    /// [Network Load Balancers] The allocation ID of the Elastic IP address for an    internet-facing load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllocationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<cfn_resources::StrVal>,

    ///
    /// [Network Load Balancers] The IPv6 address.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<cfn_resources::StrVal>,

    ///
    /// [Network Load Balancers] The private IPv4 address for an internal load balancer.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrivateIPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv4_address: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for SubnetMapping {
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

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
