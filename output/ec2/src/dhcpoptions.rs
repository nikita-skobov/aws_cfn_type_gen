/// Specifies a set of DHCP options for your VPC.
///
/// You must specify at least one of the following properties:     DomainNameServers, NetbiosNameServers,     NtpServers. If you specify NetbiosNameServers, you must specify     NetbiosNodeType.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnDHCPOptions {
    ///
    /// This value is used to complete unqualified DNS hostnames. If you're using     AmazonProvidedDNS in us-east-1, specify ec2.internal. If you're     using AmazonProvidedDNS in another Region, specify     region.compute.internal (for example,     ap-northeast-1.compute.internal). Otherwise, specify a domain name (for     example, MyCompany.com).
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// The IPv4 addresses of up to four domain name servers, or AmazonProvidedDNS.      The default is AmazonProvidedDNS. To have your instance receive a custom      DNS hostname as specified in DomainName, you must set this property to a      custom DNS server.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_servers: Option<Vec<String>>,

    ///
    /// The IPv4 addresses of up to four NetBIOS name servers.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetbiosNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netbios_name_servers: Option<Vec<String>>,

    ///
    /// The NetBIOS node type (1, 2, 4, or 8). We recommend that you specify 2 (broadcast and     multicast are not currently supported).
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetbiosNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netbios_node_type: Option<i64>,

    ///
    /// The IPv4 addresses of up to four Network Time Protocol (NTP) servers.
    ///
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NtpServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<Vec<String>>,

    ///
    /// Any tags assigned to the DHCP options set.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_dhcp_options_id: CfnDHCPOptionsdhcpoptionsid,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnDHCPOptionsdhcpoptionsid;
impl CfnDHCPOptionsdhcpoptionsid {
    pub fn att_name(&self) -> &'static str {
        r#"DhcpOptionsId"#
    }
}

impl cfn_resources::CfnResource for CfnDHCPOptions {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::DHCPOptions"
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
