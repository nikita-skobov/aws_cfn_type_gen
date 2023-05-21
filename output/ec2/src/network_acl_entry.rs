

/// Specifies an entry, known as a rule, in a network ACL with a rule number you specify.     Each network ACL has a set of numbered ingress rules and a separate set of numbered egress     rules.
///
/// For information about the protocol value, see Protocol       Numbers on the Internet Assigned Numbers Authority (IANA) website.
#[derive(Default, serde::Serialize)]
pub struct CfnNetworkAclEntry {


    /// 
    /// Whether to allow or deny traffic that matches the rule; valid values are "allow" or     "deny".
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: allow | deny
    ///
    /// Update requires: No interruption
    #[serde(rename = "RuleAction")]
    pub rule_action: String,


    /// 
    /// The IPv4 CIDR range to allow or deny, in CIDR notation (for example, 172.16.0.0/24).     Requirement is conditional: You must specify the CidrBlock or       Ipv6CidrBlock property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,


    /// 
    /// Whether this rule applies to egress traffic from the subnet (true) or     ingress traffic to the subnet (false). By default, AWS CloudFormation     specifies false.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Egress")]
    pub egress: Option<bool>,


    /// 
    /// The IPv6 network range to allow or deny, in CIDR notation. Requirement is conditional:     You must specify the CidrBlock or Ipv6CidrBlock property.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6CidrBlock")]
    pub ipv6_cidr_block: Option<String>,


    /// 
    /// The range of port numbers for the UDP/TCP protocol. Conditional required if specifying 6     (TCP) or 17 (UDP) for the protocol parameter.
    /// 
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    pub port_range: Option<PortRange>,


    /// 
    /// Rule number to assign to the entry, such as 100. ACL entries are processed in ascending     order by rule number. Entries can't use the same rule number unless one is an egress rule     and the other is an ingress rule.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleNumber")]
    pub rule_number: i64,


    /// 
    /// The IP protocol that the rule applies to. You must specify -1 or a protocol number. You     can specify -1 for all protocols.
    /// 
    /// NoteIf you specify -1, all ports are opened and the PortRange property is       ignored.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Protocol")]
    pub protocol: i64,


    /// 
    /// The ID of the ACL for the entry.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: String,


    /// 
    /// The Internet Control Message Protocol (ICMP) code and type. Requirement is conditional:     Required if specifying 1 (ICMP) for the protocol parameter.
    /// 
    /// Required: No
    ///
    /// Type: Icmp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icmp")]
    pub icmp: Option<Icmp>,

}


/// Describes a range of ports.
#[derive(Default, serde::Serialize)]
pub struct PortRange {


    /// 
    /// The last port in the range. Required if you specify 6 (TCP) or 17 (UDP) for the protocol     parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "To")]
    pub to: Option<i64>,


    /// 
    /// The first port in the range. Required if you specify 6 (TCP) or 17 (UDP) for the     protocol parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    pub from: Option<i64>,

}


/// Describes the ICMP type and code.
#[derive(Default, serde::Serialize)]
pub struct Icmp {


    /// 
    /// The Internet Control Message Protocol (ICMP) code. You can use -1 to specify all ICMP     codes for the given ICMP type. Requirement is conditional: Required if you specify 1 (ICMP)     for the protocol parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Code")]
    pub code: Option<i64>,


    /// 
    /// The Internet Control Message Protocol (ICMP) type. You can use -1 to specify all ICMP     types. Conditional requirement: Required if you specify 1 (ICMP) for the       CreateNetworkAclEntry protocol parameter.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: Option<i64>,

}
