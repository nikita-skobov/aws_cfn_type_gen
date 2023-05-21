

/// Specifies a managed prefix list. You can add one or more entries to the prefix list.     Each entry consists of a CIDR block and an optional description.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnPrefixList {


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
    pub entries: Option<Vec<Entry>>,


    /// 
    /// The tags for the prefix list.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


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
    pub prefix_list_name: String,

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
    fn type_string() -> &'static str {
        "AWS::EC2::PrefixList"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
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




/// An entry for a prefix list.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Entry {


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
    pub description: Option<String>,


    /// 
    /// The CIDR block.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Cidr")]
    pub cidr: String,

}


