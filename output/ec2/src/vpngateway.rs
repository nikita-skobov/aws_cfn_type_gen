

/// Specifies a virtual private gateway. A virtual private gateway is the endpoint on the     VPC side of your VPN connection. You can create a virtual private gateway before creating     the VPC itself.
///
/// For more information, see AWS Site-to-Site VPN in the        AWS Site-to-Site VPN User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnVPNGateway {


    /// 
    /// The type of VPN connection the virtual private gateway supports.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: ipsec.1
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: String,


    /// 
    /// The private Autonomous System Number (ASN) for the Amazon side of a BGP       session.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "AmazonSideAsn")]
    pub amazon_side_asn: Option<i64>,


    /// 
    /// Any tags assigned to the virtual private gateway.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

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
