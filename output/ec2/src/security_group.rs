/// Specifies a security group. To create a security group, use the VpcId property to specify the VPC for which to create the security     group.
///
/// If you do not specify an egress rule, we add egress rules that allow IPv4      and IPv6 traffic on all ports and protocols to any destination. We do not add     these rules if you specify your own egress rules. If you later remove your      egress rules, we restore the default egress rules.
///
/// This type supports updates. For more information about updating stacks, see AWS CloudFormation Stacks Updates.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSecurityGroup {
    ///
    /// A description for the security group.
    ///
    /// Constraints: Up to 255 characters in length
    ///
    /// Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupDescription")]
    pub group_description: cfn_resources::StrVal,

    ///
    /// The name of the security group.
    ///
    /// Constraints: Up to 255 characters in length. Cannot start with sg-.
    ///
    /// Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&;{}!$*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<cfn_resources::StrVal>,

    ///
    /// The outbound rules associated with the security group. There is a short     interruption during which you cannot connect to the security group.
    ///
    /// Required: No
    ///
    /// Type: List of Egress
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SecurityGroupEgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_egress: Option<Vec<Egress>>,

    ///
    /// The inbound rules associated with the security group. There is a short interruption     during which you cannot connect to the security group.
    ///
    /// Required: No
    ///
    /// Type: List of Ingress
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "SecurityGroupIngress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ingress: Option<Vec<Ingress>>,

    ///
    /// Any tags assigned to the security group.
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
    /// The ID of the VPC for the security group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_group_id: CfnSecurityGroupgroupid,

    #[serde(skip_serializing)]
    pub att_vpc_id: CfnSecurityGroupvpcid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSecurityGroupgroupid;
impl CfnSecurityGroupgroupid {
    pub fn att_name(&self) -> &'static str {
        r#"GroupId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSecurityGroupvpcid;
impl CfnSecurityGroupvpcid {
    pub fn att_name(&self) -> &'static str {
        r#"VpcId"#
    }
}

impl cfn_resources::CfnResource for CfnSecurityGroup {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::SecurityGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Adds the specified egress rules to a security group for use with a VPC.
///
/// An outbound rule permits instances to send traffic to the specified destination IPv4 or     IPv6 CIDR address ranges, or to the specified destination security groups for the same     VPC.
///
/// You specify a protocol for each rule (for example, TCP). For the TCP and UDP protocols,     you must also specify the destination port or port range. For the ICMP protocol, you must     also specify the ICMP type and code. You can use -1 for the type or code to mean all types     or all codes.
///
/// You must specify only one of the following properties: CidrIp,       CidrIpv6, DestinationPrefixListId, or       DestinationSecurityGroupId.
///
/// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6). If you do not specify one of these parameters, the stack will     launch successfully but the rule will not be added to the security group.
///
/// Rule changes are propagated to affected instances as quickly as possible. However, a     small delay might occur.
///
/// For more information about VPC security group limits, see Amazon VPC Limits.
///
/// Use SecurityGroup.Ingress and SecurityGroup.Egress only when     necessary, typically to allow security groups to reference each other in ingress and egress     rules. Otherwise, use the embedded ingress and egress rules of the security group. For more     information, see Amazon EC2 Security       Groups.
///
/// The EC2 Security Group Rule is an embedded property of the       AWS::EC2::SecurityGroup type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Egress {
    ///
    /// The IPv4 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User       Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<cfn_resources::StrVal>,

    ///
    /// The IPv6 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User       Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<cfn_resources::StrVal>,

    ///
    /// A description for the security group rule.
    ///
    /// Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9,     spaces, and ._-:/()#,@[]+=;{}!$*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The prefix list IDs for the destination AWS service.     This is the AWS service that you want to access through a VPC endpoint     from instances associated with the security group.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or     DestinationSecurityGroupId) or a CIDR range (CidrIp or     CidrIpv6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationPrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix_list_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the destination VPC security group.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_security_group_id: Option<cfn_resources::StrVal>,

    ///
    /// If the protocol is TCP or UDP, this is the start of the port range.     If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types.     If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,

    ///
    /// The IP protocol name (tcp, udp, icmp, icmpv6)     or number (see Protocol Numbers).
    ///
    /// Use -1 to specify all protocols. When authorizing     security group rules, specifying -1 or a protocol number other than     tcp, udp, icmp, or icmpv6 allows     traffic on all ports, regardless of any port range you specify. For tcp,     udp, and icmp, you must specify a port range. For icmpv6,     the port range is optional; if you omit the port range, traffic for all types and codes is allowed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: cfn_resources::StrVal,

    ///
    /// If the protocol is TCP or UDP, this is the end of the port range.      If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes.      If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

impl cfn_resources::CfnResource for Egress {
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

/// Adds the specified egress rules to a security group for use with a VPC.
///
/// An outbound rule permits instances to send traffic to the specified destination IPv4 or     IPv6 CIDR address ranges, or to the specified destination security groups for the same     VPC.
///
/// You specify a protocol for each rule (for example, TCP). For the TCP and UDP protocols,     you must also specify the destination port or port range. For the ICMP protocol, you must     also specify the ICMP type and code. You can use -1 for the type or code to mean all types     or all codes.
///
/// You must specify only one of the following properties: CidrIp,       CidrIpv6, DestinationPrefixListId, or       DestinationSecurityGroupId.
///
/// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6). If you do not specify one of these parameters, the stack will     launch successfully but the rule will not be added to the security group.
///
/// Rule changes are propagated to affected instances as quickly as possible. However, a     small delay might occur.
///
/// For more information about VPC security group limits, see Amazon VPC Limits.
///
/// Use SecurityGroup.Ingress and SecurityGroup.Egress only when     necessary, typically to allow security groups to reference each other in ingress and egress     rules. Otherwise, use the embedded ingress and egress rules of the security group. For more     information, see Amazon EC2 Security       Groups.
///
/// The EC2 Security Group Rule is an embedded property of the       AWS::EC2::SecurityGroup type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Ingress {
    ///
    /// The IPv4 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User       Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<cfn_resources::StrVal>,

    ///
    /// The IPv6 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User       Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<cfn_resources::StrVal>,

    ///
    /// A description for the security group rule.
    ///
    /// Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9,     spaces, and ._-:/()#,@[]+=;{}!$*
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// If the protocol is TCP or UDP, this is the start of the port range.     If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types.     If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,

    ///
    /// The IP protocol name (tcp, udp, icmp, icmpv6)     or number (see Protocol Numbers).
    ///
    /// Use -1 to specify all protocols. When authorizing     security group rules, specifying -1 or a protocol number other than     tcp, udp, icmp, or icmpv6 allows     traffic on all ports, regardless of any port range you specify. For tcp,     udp, and icmp, you must specify a port range. For icmpv6,     the port range is optional; if you omit the port range, traffic for all types and codes is allowed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: cfn_resources::StrVal,

    ///
    /// The ID of a prefix list.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourcePrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prefix_list_id: Option<cfn_resources::StrVal>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html#cfn-ec2-security-group-rule-sourcesecuritygroupid
    #[serde(rename = "SourceSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group_id: Option<cfn_resources::StrVal>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html#cfn-ec2-security-group-rule-sourcesecuritygroupname
    #[serde(rename = "SourceSecurityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group_name: Option<cfn_resources::StrVal>,

    /// Failed to resolve https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-security-group-rule.html#cfn-ec2-security-group-rule-sourcesecuritygroupownerid
    #[serde(rename = "SourceSecurityGroupOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group_owner_id: Option<cfn_resources::StrVal>,

    ///
    /// If the protocol is TCP or UDP, this is the end of the port range.      If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes.      If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

impl cfn_resources::CfnResource for Ingress {
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
