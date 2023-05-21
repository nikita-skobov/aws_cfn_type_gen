

/// Specifies a set of DHCP options for your VPC.
///
/// You must specify at least one of the following properties:     DomainNameServers, NetbiosNameServers,     NtpServers. If you specify NetbiosNameServers, you must specify     NetbiosNodeType.
#[derive(Default, serde::Serialize)]
pub struct CfnDHCPOptions {


    /// 
    /// The IPv4 addresses of up to four NetBIOS name servers.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetbiosNameServers")]
    pub netbios_name_servers: Option<Vec<String>>,


    /// 
    /// The IPv4 addresses of up to four Network Time Protocol (NTP) servers.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NtpServers")]
    pub ntp_servers: Option<Vec<String>>,


    /// 
    /// This value is used to complete unqualified DNS hostnames. If you're using     AmazonProvidedDNS in us-east-1, specify ec2.internal. If you're     using AmazonProvidedDNS in another Region, specify     region.compute.internal (for example,     ap-northeast-1.compute.internal). Otherwise, specify a domain name (for     example, MyCompany.com).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,


    /// 
    /// Any tags assigned to the DHCP options set.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The NetBIOS node type (1, 2, 4, or 8). We recommend that you specify 2 (broadcast and     multicast are not currently supported).
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetbiosNodeType")]
    pub netbios_node_type: Option<i64>,


    /// 
    /// The IPv4 addresses of up to four domain name servers, or AmazonProvidedDNS.      The default is AmazonProvidedDNS. To have your instance receive a custom      DNS hostname as specified in DomainName, you must set this property to a      custom DNS server.
    /// 
    /// Required: Conditional
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DomainNameServers")]
    pub domain_name_servers: Option<Vec<String>>,

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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
