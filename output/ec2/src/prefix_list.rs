/// Specifies a managed prefix list. You can add one or more entries to the prefix list.     Each entry consists of a CIDR block and an optional description.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPrefixList {
    ///
    /// The IP address type.
    ///
    /// Valid Values: IPv4 | IPv6
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AddressFamily")]
    pub address_family: PrefixListAddressFamilyEnum,

    ///
    /// One or more entries for the prefix list.
    ///
    /// Required: No
    ///
    /// Type: List of Entry
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Entry>>,

    ///
    /// The maximum number of entries for the prefix list.
    ///
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxEntries")]
    pub max_entries: i64,

    ///
    /// A name for the prefix list.
    ///
    /// Constraints: Up to 255 characters in length. The name cannot start with com.amazonaws.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixListName")]
    pub prefix_list_name: cfn_resources::StrVal,

    ///
    /// The tags for the prefix list.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PrefixListAddressFamilyEnum {
    /// IPv4
    #[serde(rename = "IPv4")]
    Ipv4,

    /// IPv6
    #[serde(rename = "IPv6")]
    Ipv6,
}

impl Default for PrefixListAddressFamilyEnum {
    fn default() -> Self {
        PrefixListAddressFamilyEnum::Ipv4
    }
}

impl cfn_resources::CfnResource for CfnPrefixList {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::PrefixList"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.entries {
            if the_val.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'entries'. {} is greater than 100",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// An entry for a prefix list.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Entry {
    ///
    /// The CIDR block.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    pub cidr: cfn_resources::StrVal,

    ///
    /// A description for the entry.
    ///
    /// Constraints: Up to 255 characters in length.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for Entry {
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
