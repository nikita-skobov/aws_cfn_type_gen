

/// Creates a public namespace based on DNS, which is visible on the internet. The namespace  defines your service naming scheme. For example, if you name your namespace   example.com and name your service backend, the resulting DNS name for  the service is backend.example.com. You can discover instances that were registered  with a public DNS namespace by using either a DiscoverInstances request or using  DNS. For the current quota on the number of namespaces that you can create using the same AWS account, see AWS Cloud Map quotas in the           AWS Cloud Map Developer Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnPublicDnsNamespace {


    /// 
    /// A description for the namespace.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The tags for the namespace. Each tag consists of a key and an optional value, both of which you define. Tag keys  can have a maximum character length of 128 characters, and tag values can have a maximum length of 256  characters.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: Updates are not supported.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// Properties for the  public DNS namespace.
    /// 
    /// Required: No
    ///
    /// Type: Properties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,


    /// 
    /// The name that you want to assign to this namespace.
    /// 
    /// NoteDo not include sensitive information in the name. The name is publicly available using DNS queries.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?$
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// DNS properties for  the public DNS namespace.
#[derive(Default, serde::Serialize)]
pub struct PublicDnsPropertiesMutable {


    /// 
    /// Start of Authority  (SOA) record for the hosted zone for the public DNS namespace.
    /// 
    /// Required: No
    ///
    /// Type: SOA
    ///
    /// Update requires: No interruption
    #[serde(rename = "SOA")]
    pub soa: Option<SOA>,

}


/// Properties for the  public DNS namespace.
#[derive(Default, serde::Serialize)]
pub struct Properties {


    /// 
    /// DNS properties for  the public DNS namespace.
    /// 
    /// Required: No
    ///
    /// Type: PublicDnsPropertiesMutable
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsProperties")]
    pub dns_properties: Option<PublicDnsPropertiesMutable>,

}


/// Start of Authority  (SOA) properties for a public or private DNS namespace.
#[derive(Default, serde::Serialize)]
pub struct SOA {


    /// 
    /// The time to live  (TTL) for purposes of negative caching.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "TTL")]
    pub ttl: Option<f64>,

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
