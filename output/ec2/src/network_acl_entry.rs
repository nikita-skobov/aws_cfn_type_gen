/// Specifies an entry, known as a rule, in a network ACL with a rule number you specify.     Each network ACL has a set of numbered ingress rules and a separate set of numbered egress     rules.
///
/// For information about the protocol value, see Protocol       Numbers on the Internet Assigned Numbers Authority (IANA) website.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnNetworkAclEntry {
    ///
    /// The IPv4 CIDR range to allow or deny, in CIDR notation (for example, 172.16.0.0/24).     Requirement is conditional: You must specify the CidrBlock or       Ipv6CidrBlock property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// Whether this rule applies to egress traffic from the subnet (true) or     ingress traffic to the subnet (false). By default, AWS CloudFormation     specifies false.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Egress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<bool>,

    ///
    /// The Internet Control Message Protocol (ICMP) code and type. Requirement is conditional:     Required if specifying 1 (ICMP) for the protocol parameter.
    ///
    /// Required: No
    ///
    /// Type: Icmp
    ///
    /// Update requires: No interruption
    #[serde(rename = "Icmp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp: Option<Icmp>,

    ///
    /// The IPv6 network range to allow or deny, in CIDR notation. Requirement is conditional:     You must specify the CidrBlock or Ipv6CidrBlock property.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the ACL for the entry.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: cfn_resources::StrVal,

    ///
    /// The range of port numbers for the UDP/TCP protocol. Conditional required if specifying 6     (TCP) or 17 (UDP) for the protocol parameter.
    ///
    /// Required: No
    ///
    /// Type: PortRange
    ///
    /// Update requires: No interruption
    #[serde(rename = "PortRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRange>,

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
    pub rule_action: NetworkAclEntryRuleActionEnum,

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

    #[serde(skip_serializing)]
    pub att_id: CfnNetworkAclEntryid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NetworkAclEntryRuleActionEnum {
    /// allow
    #[serde(rename = "allow")]
    Allow,

    /// deny
    #[serde(rename = "deny")]
    Deny,
}

impl Default for NetworkAclEntryRuleActionEnum {
    fn default() -> Self {
        NetworkAclEntryRuleActionEnum::Allow
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnNetworkAclEntryid;
impl CfnNetworkAclEntryid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

impl cfn_resources::CfnResource for CfnNetworkAclEntry {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::NetworkAclEntry"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.icmp.as_ref().map_or(Ok(()), |val| val.validate())?;

        self.port_range
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Describes the ICMP type and code.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<i64>,
}

impl cfn_resources::CfnResource for Icmp {
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

/// Describes a range of ports.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PortRange {
    ///
    /// The first port in the range. Required if you specify 6 (TCP) or 17 (UDP) for the     protocol parameter.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    ///
    /// The last port in the range. Required if you specify 6 (TCP) or 17 (UDP) for the protocol     parameter.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl cfn_resources::CfnResource for PortRange {
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
