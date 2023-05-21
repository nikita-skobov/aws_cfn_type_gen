

/// Creates an agreement. An agreement is a bilateral trading partner agreement, or partnership,    between an AWS Transfer Family server and an AS2 process. The agreement defines the file and message    transfer relationship between the server and the AS2 process. To define an agreement, Transfer Family    combines a server, local profile, partner profile, certificate, and other    attributes.
///
/// The partner is identified with the PartnerProfileId, and the AS2 process is identified with the LocalProfileId.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAgreement {


    /// 
    /// The current status of the agreement, either ACTIVE or    INACTIVE.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ACTIVE | INACTIVE
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    pub status: Option<String>,


    /// 
    /// A unique identifier for the AS2 local profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocalProfileId")]
    pub local_profile_id: String,


    /// 
    /// The landing directory (folder) for files that are transferred by using the AS2    protocol.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: ^$|/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BaseDirectory")]
    pub base_directory: String,


    /// 
    /// A unique identifier for the partner profile used in the agreement.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^p-([0-9a-f]{17})$
    ///
    /// Update requires: No interruption
    #[serde(rename = "PartnerProfileId")]
    pub partner_profile_id: String,


    /// 
    /// With AS2, you can send files by calling StartFileTransfer and specifying the    file paths in the request parameter, SendFilePaths. We use the file’s parent    directory (for example, for --send-file-paths /bucket/dir/file.txt, parent    directory is /bucket/dir/) to temporarily store a processed AS2 message file,    store the MDN when we receive them from the partner, and write a final JSON file containing    relevant metadata of the transmission. So, the AccessRole needs to provide read    and write access to the parent directory of the file location used in the     StartFileTransfer request. Additionally, you need to provide read and write    access to the parent directory of the files that you intend to send with     StartFileTransfer.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:.*role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccessRole")]
    pub access_role: String,


    /// 
    /// The name or short description that's used to identify the agreement.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 200
    ///
    /// Pattern: ^[\p{Graph}]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A system-assigned unique identifier for a server instance. This identifier indicates the    specific server that the agreement uses.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 19
    ///
    /// Maximum: 19
    ///
    /// Pattern: ^s-([0-9a-f]{17})$
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServerId")]
    pub server_id: String,


    /// 
    /// Key-value pairs that can be used to group and search for agreements.
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

impl cfn_resources::CfnResource for CfnAgreement {
    fn type_string() -> &'static str {
        "AWS::Transfer::Agreement"
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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}
