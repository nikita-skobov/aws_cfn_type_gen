/// High-level information about a list of firewall domains for use in a AWS::Route53Resolver::FirewallRule. This is returned by GetFirewallDomainList.
///
/// To retrieve the domains that are defined for this domain list, call        ListFirewallDomains.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_file_url: Option<cfn_resources::StrVal>,

    ///
    /// A list of the domain lists that you have defined.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnFirewallDomainListarn,

    #[serde(skip_serializing)]
    pub att_creation_time: CfnFirewallDomainListcreationtime,

    #[serde(skip_serializing)]
    pub att_creator_request_id: CfnFirewallDomainListcreatorrequestid,

    #[serde(skip_serializing)]
    pub att_id: CfnFirewallDomainListid,

    #[serde(skip_serializing)]
    pub att_managed_owner_name: CfnFirewallDomainListmanagedownername,

    #[serde(skip_serializing)]
    pub att_modification_time: CfnFirewallDomainListmodificationtime,

    #[serde(skip_serializing)]
    pub att_status: CfnFirewallDomainListstatus,

    #[serde(skip_serializing)]
    pub att_status_message: CfnFirewallDomainListstatusmessage,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListarn;
impl CfnFirewallDomainListarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListcreationtime;
impl CfnFirewallDomainListcreationtime {
    pub fn att_name(&self) -> &'static str {
        r#"CreationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListcreatorrequestid;
impl CfnFirewallDomainListcreatorrequestid {
    pub fn att_name(&self) -> &'static str {
        r#"CreatorRequestId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListid;
impl CfnFirewallDomainListid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListmanagedownername;
impl CfnFirewallDomainListmanagedownername {
    pub fn att_name(&self) -> &'static str {
        r#"ManagedOwnerName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListmodificationtime;
impl CfnFirewallDomainListmodificationtime {
    pub fn att_name(&self) -> &'static str {
        r#"ModificationTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListstatus;
impl CfnFirewallDomainListstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnFirewallDomainListstatusmessage;
impl CfnFirewallDomainListstatusmessage {
    pub fn att_name(&self) -> &'static str {
        r#"StatusMessage"#
    }
}

impl cfn_resources::CfnResource for CfnFirewallDomainList {
    fn type_string(&self) -> &'static str {
        "AWS::Route53Resolver::FirewallDomainList"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.domain_file_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'domain_file_url'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.domain_file_url {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'domain_file_url'. {} is less than 1",
                        s.len()
                    ));
                }
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
