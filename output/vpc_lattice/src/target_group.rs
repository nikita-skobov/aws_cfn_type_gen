

/// Creates a target group. A target group is a collection of targets, or compute resources,  that run your application or service. A target group can only be used by a single service.
///
/// For more information, see Target groups in the   Amazon VPC Lattice User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnTargetGroup {


    /// 
    /// The target group configuration. If type is set to LAMBDA, this  parameter doesn't apply.
    /// 
    /// Required: No
    ///
    /// Type: TargetGroupConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Config")]
    pub config: Option<TargetGroupConfig>,


    /// 
    /// The name of the target group. The name must be unique within the account. The valid    characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last    character, or immediately after another hyphen.
    /// 
    /// If you don't specify a name, CloudFormation generates one. However, if    you specify a name, and later want to replace the resource, you must specify a new    name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The tags for the target group.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Describes a target.
    /// 
    /// Required: No
    ///
    /// Type: List of Target
    ///
    /// Update requires: No interruption
    #[serde(rename = "Targets")]
    pub targets: Option<Vec<Target>>,


    /// 
    /// The type of target group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,

}



impl cfn_resources::CfnResource for CfnTargetGroup {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::TargetGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.config.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The health check configuration of a target group. Health check configurations aren't used  for LAMBDA and ALB target groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct HealthCheckConfig {


    /// 
    /// Indicates whether health checking is enabled.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,


    /// 
    /// The approximate amount of time, in seconds, between health checks of an individual target.  The range is 5–300 seconds. The default is 30 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: Option<i64>,


    /// 
    /// The amount of time, in seconds, to wait before reporting a target as unhealthy. The range is  1–120 seconds. The default is 5 seconds.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: Option<i64>,


    /// 
    /// The number of consecutive successful health checks required before considering an unhealthy  target healthy. The range is 2–10. The default is 5.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthyThresholdCount")]
    pub healthy_threshold_count: Option<i64>,


    /// 
    /// The codes to use when checking for a successful response from a target. These are called   Success codes in the console.
    /// 
    /// Required: No
    ///
    /// Type: Matcher
    ///
    /// Update requires: No interruption
    #[serde(rename = "Matcher")]
    pub matcher: Option<Matcher>,


    /// 
    /// The destination for health checks on the targets. If the protocol version is   HTTP/1.1 or HTTP/2, specify a valid URI (for example,   /path?query). The default path is /. Health checks are not supported  if the protocol version is gRPC, however, you can choose HTTP/1.1 or   HTTP/2 and specify a valid URI.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Path")]
    pub path: Option<String>,


    /// 
    /// The port used when performing health checks on targets. The default setting is the port that  a target receives traffic on.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,


    /// 
    /// The protocol used when performing health checks on targets. The possible protocols are   HTTP and HTTPS. The default is HTTP.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,


    /// 
    /// The protocol version used when performing health checks on targets. The possible protocol  versions are HTTP1 and HTTP2.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,


    /// 
    /// The number of consecutive failed health checks required before considering a target  unhealthy. The range is 2–10. The default is 2.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: Option<i64>,

}



impl cfn_resources::CfnResource for HealthCheckConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.matcher.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The codes to use when checking for a successful response from a target for health  checks.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Matcher {


    /// 
    /// The HTTP code to use when checking for a successful response from a target.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HttpCode")]
    pub http_code: String,

}



impl cfn_resources::CfnResource for Matcher {
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

/// Describes a target.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Target {


    /// 
    /// The ID of the target. If the target type of the target group is INSTANCE, this  is an instance ID. If the target type is IP , this is an IP address. If the target  type is LAMBDA, this is the ARN of the Lambda function. If the target type is   ALB, this is the ARN of the Application Load Balancer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The port on which the target is listening. For HTTP, the default is 80. For  HTTPS, the default is 443.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    pub port: Option<i64>,

}



impl cfn_resources::CfnResource for Target {
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

/// Describes the configuration of a target group. Lambda functions don't support target group  configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TargetGroupConfig {


    /// 
    /// The health check configuration.
    /// 
    /// Required: No
    ///
    /// Type: HealthCheckConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthCheckConfig>,


    /// 
    /// The type of IP address used for the target group. The possible values are ipv4  and ipv6. This is an optional parameter. If not specified, the IP address type  defaults to ipv4.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpAddressType")]
    pub ip_address_type: Option<String>,


    /// 
    /// The port on which the targets are listening. For HTTP, the default is 80. For  HTTPS, the default is 443
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Port")]
    pub port: i64,


    /// 
    /// The protocol to use for routing traffic to the targets. Default is the protocol of a target  group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: String,


    /// 
    /// The protocol version. Default value is HTTP1.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProtocolVersion")]
    pub protocol_version: Option<String>,


    /// 
    /// The ID of the VPC.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcIdentifier")]
    pub vpc_identifier: String,

}



impl cfn_resources::CfnResource for TargetGroupConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.health_check.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}