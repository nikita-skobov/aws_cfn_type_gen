

/// Creates a multicast domain using the specified transit gateway.
///
/// The transit gateway must be in the available state before you create a domain.
#[derive(Default, serde::Serialize)]
pub struct CfnTransitGatewayMulticastDomain {


    /// 
    /// The tags for the transit gateway multicast domain.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The options for the transit gateway multicast domain.
    /// 
    /// AutoAcceptSharedAssociations (enable | disable)            Igmpv2Support (enable | disable)            StaticSourcesSupport (enable | disable)
    /// 
    /// Required: No
    ///
    /// Type: Options
    ///
    /// Update requires: No interruption
    #[serde(rename = "Options")]
    pub options: Option<Options>,


    /// 
    /// The ID of the transit gateway.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TransitGatewayId")]
    pub transit_gateway_id: String,

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


/// The options for the transit gateway multicast domain.
#[derive(Default, serde::Serialize)]
pub struct Options {


    /// 
    /// Indicates whether to automatically accept cross-account subnet associations that are associated with the transit gateway multicast domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutoAcceptSharedAssociations")]
    pub auto_accept_shared_associations: Option<String>,


    /// 
    /// Specify whether to enable support for statically configuring multicast group sources for a domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "StaticSourcesSupport")]
    pub static_sources_support: Option<String>,


    /// 
    /// Specify whether to enable Internet Group Management Protocol (IGMP) version 2 for the transit gateway multicast domain.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: disable | enable
    ///
    /// Update requires: No interruption
    #[serde(rename = "Igmpv2Support")]
    pub igmpv2_support: Option<String>,

}