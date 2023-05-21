

/// An association between a firewall rule group and a VPC, which enables DNS filtering for 			the VPC.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFirewallRuleGroupAssociation {


    /// 
    /// The unique identifier of the firewall rule group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "FirewallRuleGroupId")]
    pub firewall_rule_group_id: String,


    /// 
    /// If enabled, this setting disallows modification or removal of the association, to help prevent against accidentally altering DNS firewall protections.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DISABLED | ENABLED
    ///
    /// Update requires: No interruption
    #[serde(rename = "MutationProtection")]
    pub mutation_protection: Option<FirewallRuleGroupAssociationMutationProtectionEnum>,


    /// 
    /// The name of the association.
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
    pub name: Option<String>,


    /// 
    /// The setting that determines the processing order of the rule group among the rule groups that are associated with a single VPC. DNS Firewall       filters VPC traffic starting from rule group with the lowest numeric priority setting.
    /// 
    /// You must specify a unique priority for each rule group that you associate with a single VPC.       To make it easier to insert rule groups later, leave space between the numbers, for example, use 101, 200, and so on.       You can change the priority setting for a rule group association after you create it.
    /// 
    /// The allowed values for Priority are between 100 and 9900 (excluding 100 and 9900).
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    pub priority: i64,


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


    /// 
    /// The unique identifier of the VPC that is associated with the rule group.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 64
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcId")]
    pub vpc_id: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FirewallRuleGroupAssociationMutationProtectionEnum {

    /// DISABLED
    #[serde(rename = "DISABLED")]
    Disabled,

    /// ENABLED
    #[serde(rename = "ENABLED")]
    Enabled,

}

impl Default for FirewallRuleGroupAssociationMutationProtectionEnum {
    fn default() -> Self {
        FirewallRuleGroupAssociationMutationProtectionEnum::Disabled
    }
}


impl cfn_resources::CfnResource for CfnFirewallRuleGroupAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::FirewallRuleGroupAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        let the_val = &self.firewall_rule_group_id;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'firewall_rule_group_id'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.firewall_rule_group_id;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'firewall_rule_group_id'. {} is less than 1", the_val.len()));
        }

        
        if let Some(the_val) = &self.name {

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 64", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.tags {

        if the_val.len() > 200 as _ {
            return Err(format!("Max validation failed on field 'tags'. {} is greater than 200", the_val.len()));
        }

        }
        
        let the_val = &self.vpc_id;

        if the_val.len() > 64 as _ {
            return Err(format!("Max validation failed on field 'vpc_id'. {} is greater than 64", the_val.len()));
        }

        
        let the_val = &self.vpc_id;

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'vpc_id'. {} is less than 1", the_val.len()));
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