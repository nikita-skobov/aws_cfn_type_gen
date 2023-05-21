

/// Specifies a path to analyze for reachability.
///
/// VPC Reachability Analyzer enables you to analyze and debug network reachability between     two resources in your virtual private cloud (VPC). For more information, see the Reachability Analyzer User Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnNetworkInsightsPath {


    /// 
    /// Scopes the analysis to network paths that match specific filters at the source. If you specify      this parameter, you can't specify the parameters for the source IP address or the destination port.
    /// 
    /// Required: No
    ///
    /// Type: PathFilter
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterAtSource")]
    pub filter_at_source: Option<PathFilter>,


    /// 
    /// Scopes the analysis to network paths that match specific filters at the destination. If you specify      this parameter, you can't specify the parameter for the destination IP address.
    /// 
    /// Required: No
    ///
    /// Type: PathFilter
    ///
    /// Update requires: Replacement
    #[serde(rename = "FilterAtDestination")]
    pub filter_at_destination: Option<PathFilter>,


    /// 
    /// The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Destination")]
    pub destination: Option<String>,


    /// 
    /// The IP address of the destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationIp")]
    pub destination_ip: Option<String>,


    /// 
    /// The IP address of the source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceIp")]
    pub source_ip: Option<String>,


    /// 
    /// The tags to add to the path.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The destination port.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPort")]
    pub destination_port: Option<i64>,


    /// 
    /// The ID or ARN of the source. If the resource is in another account, you must specify an ARN.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Source")]
    pub source: String,


    /// 
    /// The protocol.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: tcp | udp
    ///
    /// Update requires: Replacement
    #[serde(rename = "Protocol")]
    pub protocol: String,

}


/// Describes a set of filters for a path analysis. Use path filters to scope the analysis when      there can be multiple resulting paths.
#[derive(Default, serde::Serialize)]
pub struct PathFilter {


    /// 
    /// The source IPv4 address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourceAddress")]
    pub source_address: Option<String>,


    /// 
    /// The destination IPv4 address.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 15
    ///
    /// Pattern: ^([0-9]{1,3}.){3}[0-9]{1,3}$
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationAddress")]
    pub destination_address: Option<String>,


    /// 
    /// The source port range.
    /// 
    /// Required: No
    ///
    /// Type: FilterPortRange
    ///
    /// Update requires: Replacement
    #[serde(rename = "SourcePortRange")]
    pub source_port_range: Option<FilterPortRange>,


    /// 
    /// The destination port range.
    /// 
    /// Required: No
    ///
    /// Type: FilterPortRange
    ///
    /// Update requires: Replacement
    #[serde(rename = "DestinationPortRange")]
    pub destination_port_range: Option<FilterPortRange>,

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


/// Describes a port range.
#[derive(Default, serde::Serialize)]
pub struct FilterPortRange {


    /// 
    /// The last port in the range.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Replacement
    #[serde(rename = "ToPort")]
    pub to_port: Option<i64>,


    /// 
    /// The first port in the range.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 0
    ///
    /// Maximum: 65535
    ///
    /// Update requires: Replacement
    #[serde(rename = "FromPort")]
    pub from_port: Option<i64>,

}