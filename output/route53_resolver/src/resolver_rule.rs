/// For DNS queries that originate in your VPCs, specifies which Resolver endpoint the queries pass through, 			one domain name that you want to forward to your network, and the IP addresses of the DNS resolvers in your network.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRule {
    ///
    /// DNS queries for this domain name are forwarded to the IP addresses that are specified in TargetIps. If a query matches 			multiple Resolver rules (example.com and www.example.com), the query is routed using the Resolver rule that contains the most specific domain name 			(www.example.com).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 256
    ///
    /// Update requires: Some interruptions
    #[serde(rename = "DomainName")]
    pub domain_name: cfn_resources::StrVal,

    ///
    /// The name for the Resolver rule, which you specified when you created the Resolver rule.
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

    ///
    /// The ID of the endpoint that the rule is associated with.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResolverEndpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_endpoint_id: Option<cfn_resources::StrVal>,

    ///
    /// When you want to forward DNS queries for specified domain name to resolvers on your network, specify FORWARD.
    ///
    /// When you have a forwarding rule to forward DNS queries for a domain to your network and you want Resolver to process queries for 			a subdomain of that domain, specify SYSTEM.
    ///
    /// For example, to forward DNS queries for example.com to resolvers on your network, you create a rule and specify FORWARD 			for RuleType. To then have Resolver process queries for apex.example.com, you create a rule and specify 			SYSTEM for RuleType.
    ///
    /// Currently, only Resolver can create rules that have a value of RECURSIVE for RuleType.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: FORWARD | RECURSIVE | SYSTEM
    ///
    /// Update requires: Replacement
    #[serde(rename = "RuleType")]
    pub rule_type: ResolverRuleRuleTypeEnum,

    ///
    /// Tags help organize and categorize your Resolver rules. Each tag consists of a key and an optional value, both of which you define.
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

    ///
    /// An array that contains the IP addresses and ports that an outbound endpoint forwards DNS queries to. Typically, 			these are the IP addresses of DNS resolvers on your network.
    ///
    /// Required: No
    ///
    /// Type: List of TargetAddress
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ips: Option<Vec<TargetAddress>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnResolverRulearn,

    #[serde(skip_serializing)]
    pub att_domain_name: CfnResolverRuledomainname,

    #[serde(skip_serializing)]
    pub att_name: CfnResolverRulename,

    #[serde(skip_serializing)]
    pub att_resolver_endpoint_id: CfnResolverRuleresolverendpointid,

    #[serde(skip_serializing)]
    pub att_resolver_rule_id: CfnResolverRuleresolverruleid,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResolverRuleRuleTypeEnum {
    /// FORWARD
    #[serde(rename = "FORWARD")]
    Forward,

    /// RECURSIVE
    #[serde(rename = "RECURSIVE")]
    Recursive,

    /// SYSTEM
    #[serde(rename = "SYSTEM")]
    System,
}

impl Default for ResolverRuleRuleTypeEnum {
    fn default() -> Self {
        ResolverRuleRuleTypeEnum::Forward
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRulearn;
impl CfnResolverRulearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuledomainname;
impl CfnResolverRuledomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRulename;
impl CfnResolverRulename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleresolverendpointid;
impl CfnResolverRuleresolverendpointid {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverEndpointId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResolverRuleresolverruleid;
impl CfnResolverRuleresolverruleid {
    pub fn att_name(&self) -> &'static str {
        r#"ResolverRuleId"#
    }
}

impl cfn_resources::CfnResource for CfnResolverRule {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::ResolverRule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 256 as _ {
                return Err(format!(
                    "Max validation failed on field 'domain_name'. {} is greater than 256",
                    s.len()
                ));
            }
        }

        let the_val = &self.domain_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'domain_name'. {} is less than 1",
                    s.len()
                ));
            }
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

        if let Some(the_val) = &self.resolver_endpoint_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 64 as _ {
                    return Err(format!("Max validation failed on field 'resolver_endpoint_id'. {} is greater than 64", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.resolver_endpoint_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'resolver_endpoint_id'. {} is less than 1",
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

/// In a 			CreateResolverRule 			request, an array of the IPs that you want to forward DNS queries to.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TargetAddress {
    ///
    /// One IPv4 address that you want to forward DNS queries to.
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
    /// One IPv6 address that you want to forward DNS queries to.
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
    /// The port at Ip that you want to forward DNS queries to.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: No interruption
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TargetAddress {
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

        if let Some(the_val) = &self.port {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 65535 as _ {
                    return Err(format!(
                        "Max validation failed on field 'port'. {} is greater than 65535",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.port {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'port'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}
