

/// Specifies a target group for an Application Load Balancer, a Network Load Balancer, or a      Gateway Load Balancer.
///
/// Before you register a Lambda function as a target, you must create a       AWS::Lambda::Permission resource that grants the Elastic Load Balancing     service principal permission to invoke the Lambda function.
#[derive(Default, serde::Serialize)]
pub struct CfnTargetGroup {


    /// 
    /// [HTTP/HTTPS health checks] The HTTP or gRPC codes to use when checking for a successful    response from a target. For target groups with a protocol of TCP, TCP_UDP, UDP or TLS the range    is 200-599. For target groups with a protocol of HTTP or HTTPS, the range is 200-499. For target    groups with a protocol of GENEVE, the range is 200-399.
    /// 
    /// Required: No
    ///
    /// Type: Matcher
    ///
    /// Update requires: No interruption
    #[serde(rename = "Matcher")]
    pub matcher: Option<Matcher>,


    /// 
    /// The protocol the load balancer uses when performing health checks on targets. For    Application Load Balancers, the default is HTTP. For Network Load Balancers and Gateway Load    Balancers, the default is TCP. The TCP protocol is not supported for health checks if the    protocol of the target group is HTTP or HTTPS. The GENEVE, TLS, UDP, and TCP_UDP protocols are    not supported for health checks.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: GENEVE | HTTP | HTTPS | TCP | TCP_UDP | TLS | UDP
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckProtocol")]
    pub health_check_protocol: Option<String>,


    /// 
    /// The targets.
    /// 
    /// Required: No
    ///
    /// Type: List of TargetDescription
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<TargetDescription>>,


    /// 
    /// The name of the target group.
    /// 
    /// This name must be unique per region per account, can have a maximum of 32 characters, must    contain only alphanumeric characters or hyphens, and must not begin or end with a    hyphen.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The number of consecutive health check successes required before considering a target healthy. The range is    2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 5. For target groups    with a protocol of GENEVE, the default is 5. If the target type    is lambda, the default is 5.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThresholdCount")]
    pub healthy_threshold_count: Option<i64>,


    /// 
    /// The approximate amount of time, in seconds, between health checks of an individual target. The range is 5-300.    If the target group protocol is TCP, TLS, UDP, TCP_UDP, HTTP or HTTPS, the default is 30 seconds.    If the target group protocol is GENEVE, the default is 10 seconds.    If the target type is lambda, the default is 35 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 5
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<i64>,


    /// 
    /// The port on which the targets receive traffic. This port is used unless you specify a port    override when registering the target. If the target is a Lambda function, this parameter does    not apply. If the protocol is GENEVE, the supported port is 6081.
    /// 
    /// Required: Conditional
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// [HTTP/HTTPS health checks] The destination for health checks on the targets.
    /// 
    /// [HTTP1 or HTTP2 protocol version] The ping path. The default is /.
    /// 
    /// [GRPC protocol version] The path of a custom health check method with the format    /package.service/method. The default is /AWS.ALB/healthcheck.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckPath")]
    pub health_check_path: Option<String>,


    /// 
    /// The identifier of the virtual private cloud (VPC). If the target is a Lambda function,    this parameter does not apply. Otherwise, this parameter is required.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: Option<String>,


    /// 
    /// The amount of time, in seconds, during which no response from a target means a failed    health check. The range is 2–120 seconds. For target groups with a protocol of HTTP, the    default is 6 seconds. For target groups with a protocol of TCP, TLS or HTTPS, the default    is 10 seconds. For target groups with a protocol of GENEVE, the default is 5 seconds. If    the target type is lambda, the default is 30 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 120
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: Option<i64>,


    /// 
    /// Indicates whether health checks are enabled. If the target type is lambda,    health checks are disabled by default but can be enabled. If the target type is     instance, ip, or alb, health checks are always    enabled and cannot be disabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckEnabled")]
    pub health_check_enabled: Option<bool>,


    /// 
    /// [HTTP/HTTPS protocol] The protocol version. The possible values are GRPC,     HTTP1, and HTTP2.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,


    /// 
    /// The attributes.
    /// 
    /// Required: No
    ///
    /// Type: List of TargetGroupAttribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetGroupAttributes")]
    pub target_group_attributes: Option<Vec<TargetGroupAttribute>>,


    /// 
    /// The tags.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The number of consecutive health check failures required before considering a target unhealthy. The range is    2-10. If the target group protocol is TCP, TCP_UDP, UDP, TLS, HTTP or HTTPS, the default is 2. For target groups    with a protocol of GENEVE, the default is 2. If the target type    is lambda, the default is 5.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 2
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: Option<i64>,


    /// 
    /// The type of target that you must specify when registering targets with this target group.    You can't specify targets for a target group using more than one target type.
    /// 
    /// instance - Register targets by instance ID. This is the default      value.                        ip - Register targets by IP address. You can specify IP addresses from      the subnets of the virtual private cloud (VPC) for the target group, the RFC 1918 range      (10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16), and the RFC 6598 range (100.64.0.0/10).      You can't specify publicly routable IP addresses.                        lambda - Register a single Lambda function as a target.                        alb - Register a single Application Load Balancer as a target.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: alb | instance | ip | lambda
    ///
    /// Update requires: Replacement
    #[serde(rename = "TargetType")]
    pub target_type: Option<String>,


    /// 
    /// The protocol to use for routing traffic to the targets. For Application Load Balancers,    the supported protocols are HTTP and HTTPS. For Network Load Balancers, the supported    protocols are TCP, TLS, UDP, or TCP_UDP. For Gateway Load Balancers, the supported protocol is    GENEVE. A TCP_UDP listener must be associated with a TCP_UDP target group. If the target is a    Lambda function, this parameter does not apply.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Allowed values: GENEVE | HTTP | HTTPS | TCP | TCP_UDP | TLS | UDP
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// The type of IP address used for this target group. The possible values are     ipv4 and ipv6. This is an optional parameter. If not specified,    the IP address type defaults to ipv4.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ipv4 | ipv6
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,


    /// 
    /// The port the load balancer uses when performing health checks on targets. If the protocol    is HTTP, HTTPS, TCP, TLS, UDP, or TCP_UDP, the default is traffic-port, which is    the port on which each target receives traffic from the load balancer. If the protocol is    GENEVE, the default is port 80.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckPort")]
    pub health_check_port: Option<String>,

}


/// Specifies the HTTP codes that healthy targets must use when responding to an HTTP health     check.
#[derive(Default, serde::Serialize)]
pub struct Matcher {


    /// 
    /// You can specify values between 0 and 99. You can specify multiple values (for example,    "0,1") or a range of values (for example, "0-5"). The default value is 12.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "GrpcCode")]
    pub grpc_code: Option<String>,


    /// 
    /// For Application Load Balancers, you can specify values between 200 and 499, with the    default value being 200. You can specify multiple values (for example, "200,202") or a range of values (for example, "200-299").
    /// 
    /// For Network Load Balancers, you can specify values between 200 and 599, with the    default value being 200-399. You can specify multiple values (for example, "200,202") or a range of values (for example, "200-299").
    /// 
    /// For Gateway Load Balancers, this must be "200–399".
    /// 
    /// Note that when using shorthand syntax, some values such as commas need to be    escaped.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpCode")]
    pub http_code: Option<String>,

}


/// Specifies a target group attribute.
#[derive(Default, serde::Serialize)]
pub struct TargetGroupAttribute {


    /// 
    /// The value of the attribute.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: Option<String>,


    /// 
    /// The name of the attribute.
    /// 
    /// The following attributes are supported by all load balancers:
    /// 
    /// deregistration_delay.timeout_seconds - The amount of time, in seconds,      for Elastic Load Balancing to wait before changing the state of a deregistering target      from draining to unused. The range is 0-3600 seconds. The      default value is 300 seconds. If the target is a Lambda function, this attribute is not      supported.                        stickiness.enabled - Indicates whether target stickiness is enabled. The      value is true or false. The default is      false.                        stickiness.type - Indicates the type of stickiness. The possible values are:                                                                      lb_cookie and app_cookie for Application Load Balancers.                                 source_ip for Network Load Balancers.                                 source_ip_dest_ip and source_ip_dest_ip_proto for Gateway Load Balancers.
    /// 
    /// The following attributes are supported by Application Load Balancers and    Network Load Balancers:
    /// 
    /// load_balancing.cross_zone.enabled - Indicates whether cross zone load      balancing is enabled. The value is true, false or      use_load_balancer_configuration. The default is      use_load_balancer_configuration.                        target_group_health.dns_failover.minimum_healthy_targets.count -      The minimum number of targets that must be healthy.      If the number of healthy targets is below this value, mark the zone as unhealthy      in DNS, so that traffic is routed only to healthy zones. The possible values are      off or an integer from 1 to the maximum number of targets.      The default is off.                        target_group_health.dns_failover.minimum_healthy_targets.percentage -      The minimum percentage of targets that must be healthy.      If the percentage of healthy targets is below this value, mark the zone as unhealthy      in DNS, so that traffic is routed only to healthy zones. The possible values are      off or an integer from 1 to 100. The default is off.                        target_group_health.unhealthy_state_routing.minimum_healthy_targets.count -      The minimum number of targets that must be healthy.       If the number of healthy targets is below this value, send traffic to all targets, including unhealthy targets.      The possible values are 1 to the maximum number of targets. The default is 1.                        target_group_health.unhealthy_state_routing.minimum_healthy_targets.percentage -      The minimum percentage of targets that must be healthy.      If the percentage of healthy targets is below this value, send traffic to all targets, including unhealthy targets.      The possible values are off or an integer from 1 to 100.      The default is off.
    /// 
    /// The following attributes are supported only if the load balancer is an Application Load    Balancer and the target is an instance or an IP address:
    /// 
    /// load_balancing.algorithm.type - The load balancing algorithm determines      how the load balancer selects targets when routing requests. The value is       round_robin or least_outstanding_requests. The default is       round_robin.                        slow_start.duration_seconds - The time period, in seconds, during which a      newly registered target receives an increasing share of the traffic to the target group.      After this time period ends, the target receives its full share of traffic. The range is      30-900 seconds (15 minutes). The default is 0 seconds (disabled).                        stickiness.app_cookie.cookie_name - Indicates the name of the      application-based cookie. Names that start with the following prefixes are not allowed:       AWSALB, AWSALBAPP, and AWSALBTG; they're reserved      for use by the load balancer.                        stickiness.app_cookie.duration_seconds - The time period, in seconds,      during which requests from a client should be routed to the same target. After this time      period expires, the application-based cookie is considered stale. The range is 1 second to      1 week (604800 seconds). The default value is 1 day (86400 seconds).                        stickiness.lb_cookie.duration_seconds - The time period, in seconds,      during which requests from a client should be routed to the same target. After this time      period expires, the load balancer-generated cookie is considered stale. The range is 1      second to 1 week (604800 seconds). The default value is 1 day (86400 seconds).
    /// 
    /// The following attribute is supported only if the load balancer is an Application Load    Balancer and the target is a Lambda function:
    /// 
    /// lambda.multi_value_headers.enabled - Indicates whether the request and      response headers that are exchanged between the load balancer and the Lambda function      include arrays of values or strings. The value is true or false.      The default is false. If the value is false and the request      contains a duplicate header field name or query parameter key, the load balancer uses the      last value sent by the client.
    /// 
    /// The following attributes are supported only by Network Load Balancers:
    /// 
    /// deregistration_delay.connection_termination.enabled - Indicates whether      the load balancer terminates connections at the end of the deregistration timeout. The      value is true or false. The default is      false.                        preserve_client_ip.enabled - Indicates whether client IP preservation is      enabled. The value is true or false. The default is disabled if      the target group type is IP address and the target group protocol is TCP or TLS.      Otherwise, the default is enabled. Client IP preservation cannot be disabled for UDP and      TCP_UDP target groups.                        proxy_protocol_v2.enabled - Indicates whether Proxy Protocol version 2 is      enabled. The value is true or false. The default is       false.
    /// 
    /// The following attributes are supported only by Gateway Load Balancers:
    /// 
    /// target_failover.on_deregistration - Indicates how the Gateway Load      Balancer handles existing flows when a target is deregistered. The possible values are       rebalance and no_rebalance. The default is       no_rebalance. The two attributes       (target_failover.on_deregistration and       target_failover.on_unhealthy) can't be set independently. The value you set      for both attributes must be the same.                         target_failover.on_unhealthy - Indicates how the Gateway Load Balancer      handles existing flows when a target is unhealthy. The possible values are       rebalance and no_rebalance. The default is       no_rebalance. The two attributes       (target_failover.on_deregistration and       target_failover.on_unhealthy) cannot be set independently. The value you      set for both attributes must be the same.
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
    pub key: Option<String>,

}


/// Specifies a target to add to a target group.
#[derive(Default, serde::Serialize)]
pub struct TargetDescription {


    /// 
    /// The port on which the target is listening. If the target group protocol is GENEVE, the    supported port is 6081. If the target type is alb, the targeted Application Load    Balancer must have at least one listener whose port matches the target group port. This    parameter is not used if the target is a Lambda function.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// An Availability Zone or all. This determines whether the target receives    traffic from the load balancer nodes in the specified Availability Zone or from all enabled    Availability Zones for the load balancer.
    /// 
    /// For Application Load Balancer target groups, the specified Availability Zone value is only applicable    when cross-zone load balancing is off. Otherwise the parameter is ignored and treated    as all.
    /// 
    /// This parameter is not supported if the target type of the target group is    instance or alb.
    /// 
    /// If the target type is ip and the IP address is in a subnet of the VPC for the target group,    the Availability Zone is automatically detected and this parameter is optional. If the IP address is outside    the VPC, this parameter is required.
    /// 
    /// For Application Load Balancer target groups with cross-zone load balancing off, if the target type    is ip and the IP address is outside of the VPC for the target group, this should be an    Availability Zone inside the VPC for the target group.
    /// 
    /// If the target type is lambda, this parameter is optional and the only    supported value is all.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The ID of the target. If the target type of the target group is instance,    specify an instance ID. If the target type is ip, specify an IP address. If the    target type is lambda, specify the ARN of the Lambda function. If the target type    is alb, specify the ARN of the Application Load Balancer target.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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