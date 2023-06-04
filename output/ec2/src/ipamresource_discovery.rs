/// A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscovery {
    ///
    /// The resource discovery description.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The operating Regions for the resource discovery. Operating Regions are AWS Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the AWS Regions you select as operating Regions.
    ///
    /// Required: No
    ///
    /// Type: List of IpamOperatingRegion
    ///
    /// Update requires: No interruption
    #[serde(rename = "OperatingRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_regions: Option<Vec<IpamOperatingRegion>>,

    ///
    /// A tag is a label that you assign to an AWS resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your AWS costs.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_ipam_resource_discovery_arn: CfnIPAMResourceDiscoveryipamresourcediscoveryarn,

    #[serde(skip_serializing)]
    pub att_ipam_resource_discovery_id: CfnIPAMResourceDiscoveryipamresourcediscoveryid,

    #[serde(skip_serializing)]
    pub att_ipam_resource_discovery_region: CfnIPAMResourceDiscoveryipamresourcediscoveryregion,

    #[serde(skip_serializing)]
    pub att_owner_id: CfnIPAMResourceDiscoveryownerid,

    #[serde(skip_serializing)]
    pub att_state: CfnIPAMResourceDiscoverystate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryipamresourcediscoveryarn;
impl CfnIPAMResourceDiscoveryipamresourcediscoveryarn {
    pub fn att_name(&self) -> &'static str {
        r#"IpamResourceDiscoveryArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryipamresourcediscoveryid;
impl CfnIPAMResourceDiscoveryipamresourcediscoveryid {
    pub fn att_name(&self) -> &'static str {
        r#"IpamResourceDiscoveryId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryipamresourcediscoveryregion;
impl CfnIPAMResourceDiscoveryipamresourcediscoveryregion {
    pub fn att_name(&self) -> &'static str {
        r#"IpamResourceDiscoveryRegion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryownerid;
impl CfnIPAMResourceDiscoveryownerid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoverystate;
impl CfnIPAMResourceDiscoverystate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnIPAMResourceDiscovery {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::IPAMResourceDiscovery"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The operating Regions for an IPAM. Operating Regions are AWS Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the AWS Regions you select as operating Regions.
///
/// For more information about operating Regions, see Create an IPAM in the Amazon VPC IPAM User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub region_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for IpamOperatingRegion {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
