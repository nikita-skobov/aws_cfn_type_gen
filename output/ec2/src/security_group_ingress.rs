

/// Adds an inbound rule to a security group.
///
/// An inbound rule permits instances to receive traffic from the specified IPv4 or IPv6     CIDR address range, or from the instances associated with the specified security     group.
///
/// You must specify only one of the following properties: CidrIp,     CidrIpv6, SourcePrefixListId,     SourceSecurityGroupId, or SourceSecurityGroupName.
///
/// You specify a protocol for each rule (for example, TCP). For TCP and UDP, you must also     specify a port or port range. For ICMP/ICMPv6, you must also specify the ICMP/ICMPv6 type     and code. You can use -1 to mean all types or all codes.
///
/// You must specify a source security group (SourcePrefixListId,     SourceSecurityGroupId, or SourceSecurityGroupName) or a CIDR     range (CidrIp or CidrIpv6). If you do not specify one of these     parameters, the stack will launch successfully but the rule will not be added to the     security group.
///
/// Rule changes are propagated to instances within the security group as quickly as     possible. However, a small delay might occur.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSecurityGroupIngress {


    /// 
    /// The ID of the security group.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,


    /// 
    /// [Default VPC] The name of the source security group. You must specify either the security group ID      or the security group name. You can't specify the group name in combination with an IP address range.      Creates rules that grant full ICMP, UDP, and TCP access.
    /// 
    /// For security groups in a nondefault VPC, you must specify the group ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceSecurityGroupName")]
    pub source_security_group_name: Option<String>,


    /// 
    /// The ID of a prefix list.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePrefixListId")]
    pub source_prefix_list_id: Option<String>,


    /// 
    /// Updates the description of an ingress (inbound) security group rule. You can replace an     existing description, or add a description to a rule that did not have one     previously.
    /// 
    /// Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9,     spaces, and ._-:/()#,@[]+=;{}!$*
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The IPv4 address range, in CIDR format.
    /// 
    /// You must specify a source security group (SourcePrefixListId or     SourceSecurityGroupId) or a CIDR range (CidrIp or     CidrIpv6).
    /// 
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User        Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrIp")]
    pub cidr_ip: Option<String>,


    /// 
    /// The end of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 code. A value of     -1 indicates all ICMP/ICMPv6 codes for the specified ICMP type. If you     specify all ICMP/ICMPv6 types, you must specify all codes.
    /// 
    /// Use this for ICMP and any protocol that uses ports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "ToPort")]
    pub to_port: Option<i64>,


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
    pub group_name: Option<String>,


    /// 
    /// [nondefault VPC] The AWS account ID for the source security group, if     the source security group is in a different account. You can't specify this property with     an IP address range. Creates rules that grant full ICMP, UDP, and TCP access.
    /// 
    /// If you specify SourceSecurityGroupName or       SourceSecurityGroupId and that security group is owned by a different     account than the account creating the stack, you must specify       SourceSecurityGroupOwnerId; otherwise, this property is optional.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceSecurityGroupOwnerId")]
    pub source_security_group_owner_id: Option<String>,


    /// 
    /// The ID of the security group. You must specify either the security group ID or the     security group name. For security groups in a nondefault VPC, you must specify the security     group ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceSecurityGroupId")]
    pub source_security_group_id: Option<String>,


    /// 
    /// The start of port range for the TCP and UDP protocols, or an ICMP/ICMPv6 type number. A     value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6     types, you must specify all codes.
    /// 
    /// Use this for ICMP and any protocol that uses ports.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "FromPort")]
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
    /// Update requires: Replacement
    #[serde(rename = "IpProtocol")]
    pub ip_protocol: String,


    /// 
    /// The IPv6 address range, in CIDR format.
    /// 
    /// You must specify a source security group (SourcePrefixListId or     SourceSecurityGroupId) or a CIDR range (CidrIp or     CidrIpv6).
    /// 
    /// For examples of rules that you can add to security groups for specific access scenarios,     see Security group rules       for different use cases in the Amazon EC2 User        Guide.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrIpv6")]
    pub cidr_ipv6: Option<String>,

}



impl cfn_resources::CfnResource for CfnSecurityGroupIngress {
    fn type_string() -> &'static str {
        "AWS::EC2::SecurityGroupIngress"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}
