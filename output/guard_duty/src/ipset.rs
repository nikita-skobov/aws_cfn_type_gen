

/// The AWS::GuardDuty::IPSet resource specifies a new          IPSet. An IPSet is a list of trusted IP addresses from          which secure communication is allowed with AWS infrastructure and          applications.
#[derive(Default, serde::Serialize)]
pub struct CfnIPSet {


    /// 
    /// The unique ID of the detector of the GuardDuty account that you want to create an IPSet    for.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: Replacement
    #[serde(rename = "DetectorId")]
    pub detector_id: String,


    /// 
    /// The URI of the file that contains the IPSet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: String,


    /// 
    /// The tags to be added to a new IP set resource. Each tag consists of a key and an          optional value, both of which you define.
    /// 
    /// For more information, see Tag.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The format of the file that contains the IPSet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ALIEN_VAULT | FIRE_EYE | OTX_CSV | PROOF_POINT | STIX | TXT
    ///
    /// Update requires: Replacement
    #[serde(rename = "Format")]
    pub format: String,


    /// 
    /// Indicates whether or not uses the             IPSet.
    /// 
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "Activate")]
    pub activate: bool,


    /// 
    /// The user-friendly name to identify the IPSet.
    /// 
    /// Allowed characters are alphanumeric, whitespace, dash (-), and underscores (_).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Default, serde::Serialize)]
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