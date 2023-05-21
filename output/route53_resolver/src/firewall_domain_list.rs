

/// High-level information about a list of firewall domains for use in a AWS::Route53Resolver::FirewallRule. This is returned by GetFirewallDomainList.
///
/// To retrieve the domains that are defined for this domain list, call        ListFirewallDomains.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFirewallDomainList {


    /// 
    /// The fully qualified URL or URI of the file stored in Amazon Simple Storage Service 			(Amazon S3) that contains the list of domains to import.
    /// 
    /// The file must be in an S3 bucket that's in the same Region    as your DNS Firewall. The file must be a text file and must contain a single domain per line.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainFileUrl")]
    pub domain_file_url: Option<String>,


    /// 
    /// A list of the domain lists that you have defined.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domains")]
    pub domains: Option<Vec<String>>,


    /// 
    /// The name of the domain list.
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
    /// A list of the tag keys and values that you want to associate with the domain list.
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



impl cfn_resources::CfnResource for CfnFirewallDomainList {
    fn type_string() -> &'static str {
        "AWS::Route53Resolver::FirewallDomainList"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.domain_file_url {

        if the_val.len() > 1024 as _ {
            return Err(format!("Max validation failed on field 'domain_file_url'. {} is greater than 1024", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.domain_file_url {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'domain_file_url'. {} is less than 1", the_val.len()));
        }

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
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        Ok(())
    }
}