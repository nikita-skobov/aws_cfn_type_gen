/// Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound:
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpoint {
    ///
    /// Indicates whether the Resolver endpoint allows inbound or outbound DNS queries:
    ///
    /// INBOUND: allows DNS queries to your VPC from your network                        OUTBOUND: allows DNS queries from your VPC to your network
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: INBOUND | OUTBOUND
    ///
    /// Update requires: Replacement
    #[serde(rename = "Direction")]
    pub direction: ResolverEndpointDirectionEnum,

    ///
    /// The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward 			DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC.
    ///
    /// Required: Yes
    ///
    /// Type: List of IpAddressRequest
    ///
    /// Maximum: 20
    ///
    /// Update requires: No interruption
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<IpAddressRequest>,

    ///
    /// A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "OutpostArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PreferredInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_instance_type: Option<cfn_resources::StrVal>,

    ///
    /// The Resolver endpoint IP address type.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DUALSTACK | IPV4 | IPV6
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResolverEndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_type: Option<ResolverEndpointResolverEndpointTypeEnum>,

    ///
    /// The ID of one or more security groups that control access to this VPC. The security group must include one or more inbound rules 			(for inbound endpoints) or outbound rules (for outbound endpoints). Inbound and outbound rules must allow TCP and UDP access. 			For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,

    ///
    /// Route 53 Resolver doesn't support updating tags through CloudFormation.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnResolverEndpointarn,

    #[serde(skip_serializing)]
    pub att_direction: CfnResolverEndpointdirection,

    #[serde(skip_serializing)]
    pub att_host_vpcid: CfnResolverEndpointhostvpcid,

    #[serde(skip_serializing)]
    pub att_ip_address_count: CfnResolverEndpointipaddresscount,

    #[serde(skip_serializing)]
    pub att_name: CfnResolverEndpointname,

    #[serde(skip_serializing)]
    pub att_outpost_arn: CfnResolverEndpointoutpostarn,

    #[serde(skip_serializing)]
    pub att_preferred_instance_type: CfnResolverEndpointpreferredinstancetype,

    #[serde(skip_serializing)]
    pub att_resolver_endpoint_id: CfnResolverEndpointresolverendpointid,

    #[serde(skip_serializing)]
    pub att_resolver_endpoint_type: CfnResolverEndpointresolverendpointtype,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResolverEndpointDirectionEnum {
    /// INBOUND
    #[serde(rename = "INBOUND")]
    Inbound,

    /// OUTBOUND
    #[serde(rename = "OUTBOUND")]
    Outbound,
}

impl Default for ResolverEndpointDirectionEnum {
    fn default() -> Self {
        ResolverEndpointDirectionEnum::Inbound
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResolverEndpointResolverEndpointTypeEnum {
    /// DUALSTACK
    #[serde(rename = "DUALSTACK")]
    Dualstack,

    /// IPV4
    #[serde(rename = "IPV4")]
    Ipv4,

    /// IPV6
    #[serde(rename = "IPV6")]
    Ipv6,
}

impl Default for ResolverEndpointResolverEndpointTypeEnum {
    fn default() -> Self {
        ResolverEndpointResolverEndpointTypeEnum::Dualstack
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointarn;
impl CfnResolverEndpointarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointdirection;
impl CfnResolverEndpointdirection {
    pub fn att_name(&self) -> &'static str {
        r#"Direction"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointhostvpcid;
impl CfnResolverEndpointhostvpcid {
    pub fn att_name(&self) -> &'static str {
        r#"HostVPCId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointipaddresscount;
impl CfnResolverEndpointipaddresscount {
    pub fn att_name(&self) -> &'static str {
        r#"IpAddressCount"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointname;
impl CfnResolverEndpointname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointoutpostarn;
impl CfnResolverEndpointoutpostarn {
    pub fn att_name(&self) -> &'static str {
        r#"OutpostArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointpreferredinstancetype;
impl CfnResolverEndpointpreferredinstancetype {
    pub fn att_name(&self) -> &'static str {
        r#"PreferredInstanceType"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointresolverendpointid;
impl CfnResolverEndpointresolverendpointid {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverEndpointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverEndpointresolverendpointtype;
impl CfnResolverEndpointresolverendpointtype {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverEndpointType"#
    }
}

impl cfn_resources::CfnResource for CfnResolverEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.ip_addresses;

        if the_val.len() > 20 as _ {
            return Err(format!(
                "Max validation failed on field 'ip_addresses'. {} is greater than 20",
                the_val.len()
            ));
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 64",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// In a 			CreateResolverEndpoint 			request, the IP address that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). 			IpAddressRequest also includes the ID of the subnet that contains the IP address.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IpAddressRequest {
    ///
    /// The IPv4 address that you want to use for DNS queries.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 7
    ///
    /// Maximum: 36
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<cfn_resources::StrVal>,

    ///
    /// The IPv6 address that you want to use for DNS queries.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 7
    ///
    /// Maximum: 39
    ///
    /// Update requires: No interruption
    #[serde(rename = "Ipv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the subnet that contains the IP address.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 32
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IpAddressRequest {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.ip {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 36 as _ {
                    return Err(format!(
                        "Max validation failed on field 'ip'. {} is greater than 36",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.ip {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 7 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ip'. {} is less than 7",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.ipv6 {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 39 as _ {
                    return Err(format!(
                        "Max validation failed on field 'ipv6'. {} is greater than 39",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.ipv6 {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 7 as _ {
                    return Err(format!(
                        "Min validation failed on field 'ipv6'. {} is less than 7",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.subnet_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 32 as _ {
                return Err(format!(
                    "Max validation failed on field 'subnet_id'. {} is greater than 32",
                    s.len()
                ));
            }
        }

        let the_val = &self.subnet_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'subnet_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

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
