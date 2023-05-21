

/// IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across AWS Regions and accounts throughout your AWS Organization. For more information, see What is IPAM? in the Amazon VPC IPAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnIPAM {


    /// 
    /// The operating Regions for an IPAM. Operating Regions are AWS Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the AWS Regions you select as operating Regions.
    /// 
    /// For more information about operating Regions, see Create an IPAM in the Amazon VPC IPAM User Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of IpamOperatingRegion
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperatingRegions")]
    pub operating_regions: Option<Vec<IpamOperatingRegion>>,


    /// 
    /// The IPAM's default resource discovery association ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceDiscoveryAssociationId")]
    pub default_resource_discovery_association_id: Option<String>,


    /// 
    /// The IPAM's default resource discovery ID.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DefaultResourceDiscoveryId")]
    pub default_resource_discovery_id: Option<String>,


    /// 
    /// The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value.   For example, to find all resources that have a tag with the key Owner and the value TeamA, specify tag:Owner for the filter name and TeamA for the filter value.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The description for the IPAM.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

impl cfn_resources::CfnResource for CfnIPAM {
    fn type_string() -> &'static str {
        "AWS::EC2::IPAM"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The operating Regions for an IPAM. Operating Regions are AWS Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the AWS Regions you select as operating Regions.
///
/// For more information about operating Regions, see Create an IPAM in the Amazon VPC IPAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IpamOperatingRegion {


    /// 
    /// The name of the operating Region.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RegionName")]
    pub region_name: String,

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
