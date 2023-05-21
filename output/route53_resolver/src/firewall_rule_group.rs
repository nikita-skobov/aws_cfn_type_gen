/// High-level information for a firewall rule group. A firewall             rule group is a collection of rules that DNS Firewall uses to filter DNS network traffic for a VPC. To retrieve the rules for the rule group, call        ListFirewallRules.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFirewallRuleGroup {
    ///
    /// A list of the rules that you have defined.
    ///
    /// Required: No
    ///
    /// Type: List of FirewallRule
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallRules")]
    pub firewall_rules: Option<Vec<FirewallRule>>,

    ///
    /// The name of the rule group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 64
    ///
    /// Pattern: (?!^[0-9]+$)([a-zA-Z0-9\-_' ']+)
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

    ///
    /// A list of the tag keys and values that you want to associate with the rule group.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnFirewallRuleGroup {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::FirewallRuleGroup"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if the_val.len() > 64 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 64",
                    the_val.len()
                ));
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

/// A single firewall rule in a rule group.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FirewallRule {
    ///
    /// The action that DNS Firewall should take on a DNS query when it matches one of the domains in the rule's domain list:
    ///
    /// ALLOW - Permit the request to go through.                         ALERT - Permit the request to go through but send an alert to the logs.                         BLOCK - Disallow the request. If this is specified,then BlockResponse must also be specified.         if BlockResponse is OVERRIDE, then all of the           following OVERRIDE attributes must be specified:                                                BlockOverrideDnsType          BlockOverrideDomainBlockOverrideTtl
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALERT | ALLOW | BLOCK
    ///
    /// Update requires: No interruption
    #[serde(rename = "Action")]
    pub action: FirewallRuleActionEnum,

    ///
    /// The DNS record's type. This determines the format of the record value that you provided in BlockOverrideDomain. Used for the rule action BLOCK with a BlockResponse setting of OVERRIDE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CNAME
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockOverrideDnsType")]
    pub block_override_dns_type: Option<FirewallRuleBlockOverrideDnsTypeEnum>,

    ///
    /// The custom DNS record to send back in response to the query. Used for the rule action BLOCK with a BlockResponse setting of OVERRIDE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockOverrideDomain")]
    pub block_override_domain: Option<String>,

    ///
    /// The recommended amount of time, in seconds, for the DNS resolver or web browser to cache the provided override record. Used for the rule action BLOCK with a BlockResponse setting of OVERRIDE.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockOverrideTtl")]
    pub block_override_ttl: Option<i64>,

    ///
    /// The way that you want DNS Firewall to block the request. Used for the rule action setting BLOCK.
    ///
    /// NODATA - Respond indicating that the query was successful, but no response is available for it.                        NXDOMAIN - Respond indicating that the domain name that's in the query doesn't exist.                        OVERRIDE - Provide a custom override in the response. This option requires custom handling details in the rule's BlockOverride* settings.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: NODATA | NXDOMAIN | OVERRIDE
    ///
    /// Update requires: No interruption
    #[serde(rename = "BlockResponse")]
    pub block_response: Option<FirewallRuleBlockResponseEnum>,

    ///
    /// The ID of the domain list that's used in the rule.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirewallDomainListId")]
    pub firewall_domain_list_id: String,

    ///
    /// The priority of the rule in the rule group. This value must be unique within the rule group. DNS Firewall processes the rules in a rule group by order of priority, starting from the lowest setting.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FirewallRuleActionEnum {
    /// ALERT
    #[serde(rename = "ALERT")]
    Alert,

    /// ALLOW
    #[serde(rename = "ALLOW")]
    Allow,

    /// BLOCK
    #[serde(rename = "BLOCK")]
    Block,
}

impl Default for FirewallRuleActionEnum {
    fn default() -> Self {
        FirewallRuleActionEnum::Alert
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FirewallRuleBlockOverrideDnsTypeEnum {
    /// CNAME
    #[serde(rename = "CNAME")]
    Cname,
}

impl Default for FirewallRuleBlockOverrideDnsTypeEnum {
    fn default() -> Self {
        FirewallRuleBlockOverrideDnsTypeEnum::Cname
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum FirewallRuleBlockResponseEnum {
    /// NODATA
    #[serde(rename = "NODATA")]
    Nodata,

    /// NXDOMAIN
    #[serde(rename = "NXDOMAIN")]
    Nxdomain,

    /// OVERRIDE
    #[serde(rename = "OVERRIDE")]
    Override,
}

impl Default for FirewallRuleBlockResponseEnum {
    fn default() -> Self {
        FirewallRuleBlockResponseEnum::Nodata
    }
}

impl cfn_resources::CfnResource for FirewallRule {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.block_override_domain {
            if the_val.len() > 255 as _ {
                return Err(format!("Max validation failed on field 'block_override_domain'. {} is greater than 255", the_val.len()));
            }
        }

        if let Some(the_val) = &self.block_override_domain {
            if the_val.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'block_override_domain'. {} is less than 1",
                    the_val.len()
                ));
            }
        }

        let the_val = &self.firewall_domain_list_id;

        if the_val.len() > 64 as _ {
            return Err(format!(
                "Max validation failed on field 'firewall_domain_list_id'. {} is greater than 64",
                the_val.len()
            ));
        }

        let the_val = &self.firewall_domain_list_id;

        if the_val.len() < 1 as _ {
            return Err(format!(
                "Min validation failed on field 'firewall_domain_list_id'. {} is less than 1",
                the_val.len()
            ));
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
