

/// Creates a service network. A service network is a logical boundary for a collection of  services. You can associate services and VPCs with a service network.
///
/// For more information, see Service networks in the   Amazon VPC Lattice User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnServiceNetwork {


    /// 
    /// The type of IAM policy.
    /// 
    /// NONE: The resource does not use an IAM policy. This is the default.     AWS_IAM: The resource uses an IAM policy. When this type is used, auth is enabled and an auth policy is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,


    /// 
    /// The tags for the service network.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The name of the service network. The name must be unique to the account. The valid    characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last    character, or immediately after another hyphen.
    /// 
    /// If you don't specify a name, CloudFormation generates one. However, if    you specify a name, and later want to replace the resource, you must specify a new    name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
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
