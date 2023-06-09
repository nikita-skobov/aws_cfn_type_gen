/// Adds the specified egress rules to a security group.
///
/// An outbound rule permits instances to send traffic to the specified destination IPv4 or     IPv6 CIDR address ranges, or to the specified destination security groups for the same     VPC.
///
/// You specify a protocol for each rule (for example, TCP). For the TCP and UDP protocols,     you must also specify the destination port or port range. For the ICMP protocol, you must     also specify the ICMP type and code. You can use -1 for the type or code to mean all types     or all codes.
///
/// You must specify only one of the following properties: CidrIp,     CidrIpv6, DestinationPrefixListId, or     DestinationSecurityGroupId.
///
/// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6). If you do not specify one of these parameters, the stack will     launch successfully but the rule will not be added to the security group.
///
/// Rule changes are propagated to affected instances as quickly as possible. However, a     small delay might occur.
///
/// For more information about VPC security group limits, see Amazon VPC Limits.
///
/// Use AWS::EC2::SecurityGroupIngress and       AWS::EC2::SecurityGroupEgress only when necessary, typically to allow     security groups to reference each other in ingress and egress rules. Otherwise, use the     embedded ingress and egress rules of the security group. For more information, see Amazon       EC2 Security Groups.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnSecurityGroupEgress {
    ///
    /// The IPv4 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to     security groups for specific access scenarios, see Security group rules       for different use cases in the Amazon EC2 User        Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<cfn_resources::StrVal>,

    ///
    /// The IPv6 address range, in CIDR format.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// For examples of rules that you can add to     security groups for specific access scenarios, see Security group rules       for different use cases in the Amazon EC2 User        Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<cfn_resources::StrVal>,

    ///
    /// The description of an egress (outbound) security group rule.
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
    /// The prefix list IDs for an AWS service. This is the       AWS service that you want to access through a VPC endpoint from     instances associated with the security group.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPrefixListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix_list_id: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the security group.
    ///
    /// You must specify a destination security group (DestinationPrefixListId or       DestinationSecurityGroupId) or a CIDR range (CidrIp or       CidrIpv6).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
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
    /// Update requires: Replacement
    #[serde(rename = "FromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,

    ///
    /// The ID of the security group. You must specify either the security group ID or the 			security group name in the request. For security groups in a nondefault VPC, you must 			specify the security group ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupId")]
    pub group_id: cfn_resources::StrVal,

    ///
    /// The IP protocol name (tcp, udp, icmp, icmpv6)     or number (see Protocol Numbers).
    ///
    /// Use -1 to specify all protocols. When authorizing     security group rules, specifying -1 or a protocol number other than     tcp, udp, icmp, or icmpv6 allows     traffic on all ports, regardless of any port range you specify. For tcp,     udp, and icmp, you must specify a port range. For icmpv6,     the port range is optional; if you omit the port range, traffic for all types and codes is allowed.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: cfn_resources::StrVal,

    ///
    /// If the protocol is TCP or UDP, this is the end of the port range.      If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes.      If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ToPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

impl cfn_resources::CfnResource for CfnSecurityGroupEgress {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::SecurityGroupEgress"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
