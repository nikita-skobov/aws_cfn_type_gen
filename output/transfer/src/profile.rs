

/// Creates the local or partner profile to use for AS2 transfers.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProfile {


    /// 
    /// The As2Id is the AS2-name, as defined in the   RFC 4130. For inbound transfers, this is the AS2-From header for the AS2 messages    sent from the partner. For outbound connectors, this is the AS2-To header for the    AS2 messages sent to the partner using the StartFileTransfer API operation. This ID cannot include spaces.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 128
    ///
    /// Pattern: ^[\p{Print}\s]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "As2Id")]
    pub as2_id: String,


    /// 
    /// An array of identifiers for the imported certificates. You use this identifier for working with profiles and partner profiles.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CertificateIds")]
    pub certificate_ids: Option<Vec<String>>,


    /// 
    /// Indicates whether to list only LOCAL type profiles or only PARTNER type profiles.   If not supplied in the request, the command lists all types of profiles.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LOCAL | PARTNER
    ///
    /// Update requires: Replacement
    #[serde(rename = "ProfileType")]
    pub profile_type: ProfileProfileTypeEnum,


    /// 
    /// Key-value pairs that can be used to group and search for profiles.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ProfileProfileTypeEnum {

    /// LOCAL
    #[serde(rename = "LOCAL")]
    Local,

    /// PARTNER
    #[serde(rename = "PARTNER")]
    Partner,

}

impl Default for ProfileProfileTypeEnum {
    fn default() -> Self {
        ProfileProfileTypeEnum::Local
    }
}


impl cfn_resources::CfnResource for CfnProfile {
    fn type_string() -> &'static str {
        "AWS::Transfer::Profile"
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


