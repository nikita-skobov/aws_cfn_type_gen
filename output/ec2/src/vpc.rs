/// Specifies a virtual private cloud (VPC).
///
/// You can optionally request an IPv6 CIDR block for the VPC. You can request an Amazon-provided      IPv6 CIDR block from Amazon's pool of IPv6 addresses, or an IPv6 CIDR block from an IPv6 address      pool that you provisioned through bring your own IP addresses (BYOIP).
///
/// For more information, see Virtual private clouds (VPC) in the Amazon VPC User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnVPC {
    ///
    /// The IPv4 network range for the VPC, in CIDR notation. For example,     10.0.0.0/16. We modify the specified CIDR block to its canonical form; for example, if you specify 100.68.0.18/18, we modify it to 100.68.0.0/18.
    ///
    /// You must specify eitherCidrBlock or Ipv4IpamPoolId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CidrBlock")]
    pub cidr_block: Option<String>,

    ///
    /// Indicates whether the instances launched in the VPC get DNS hostnames. If enabled,     instances in the VPC get DNS hostnames; otherwise, they do not. Disabled by default for     nondefault VPCs. For more information, see DNS attributes in your       VPC.
    ///
    /// You can only enable DNS hostnames if you've enabled DNS support.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDnsHostnames")]
    pub enable_dns_hostnames: Option<bool>,

    ///
    /// Indicates whether the DNS resolution is supported for the VPC. If enabled, queries to     the Amazon provided DNS server at the 169.254.169.253 IP address, or the reserved IP     address at the base of the VPC network range "plus two" succeed. If disabled, the Amazon     provided DNS service in the VPC that resolves public DNS hostnames to IP addresses is not     enabled. Enabled by default. For more information, see DNS attributes in your       VPC.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDnsSupport")]
    pub enable_dns_support: Option<bool>,

    ///
    /// The allowed tenancy of instances launched into the VPC.
    ///
    /// default: An instance launched into the VPC runs on shared hardware        by default, unless you explicitly specify a different tenancy during instance        launch.            dedicated: An instance launched into the VPC runs on dedicated        hardware by default, unless you explicitly specify a tenancy of host         during instance launch. You cannot specify a tenancy of default during         instance launch.
    ///
    /// Updating InstanceTenancy requires no replacement only if you are updating     its value from dedicated to default. Updating     InstanceTenancy from default to dedicated     requires replacement.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: dedicated | default | host
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "InstanceTenancy")]
    pub instance_tenancy: Option<VPCInstanceTenancyEnum>,

    ///
    /// The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. For more information, see      What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// You must specify eitherCidrBlock or Ipv4IpamPoolId.
    ///
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv4IpamPoolId")]
    pub ipv4_ipam_pool_id: Option<String>,

    ///
    /// The netmask length of the IPv4 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see What is IPAM? in the Amazon VPC IPAM User Guide.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Ipv4NetmaskLength")]
    pub ipv4_netmask_length: Option<i64>,

    ///
    /// The tags for the VPC.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum VPCInstanceTenancyEnum {
    /// dedicated
    #[serde(rename = "dedicated")]
    Dedicated,

    /// default
    #[serde(rename = "default")]
    Default,

    /// host
    #[serde(rename = "host")]
    Host,
}

impl Default for VPCInstanceTenancyEnum {
    fn default() -> Self {
        VPCInstanceTenancyEnum::Dedicated
    }
}

impl cfn_resources::CfnResource for CfnVPC {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::VPC"
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
