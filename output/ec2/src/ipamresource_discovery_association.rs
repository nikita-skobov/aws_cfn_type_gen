/// An IPAM resource discovery association. An associated resource discovery is a resource discovery that has been associated with an IPAM. IPAM aggregates the resource CIDRs discovered by the associated resource discovery.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnIPAMResourceDiscoveryAssociation {
    ///
    /// The IPAM ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamId")]
    pub ipam_id: cfn_resources::StrVal,

    ///
    /// The resource discovery ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "IpamResourceDiscoveryId")]
    pub ipam_resource_discovery_id: cfn_resources::StrVal,

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
    pub att_ipam_arn: CfnIPAMResourceDiscoveryAssociationipamarn,

    #[serde(skip_serializing)]
    pub att_ipam_region: CfnIPAMResourceDiscoveryAssociationipamregion,

    #[serde(skip_serializing)]
    pub att_ipam_resource_discovery_association_arn:
        CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationarn,

    #[serde(skip_serializing)]
    pub att_ipam_resource_discovery_association_id:
        CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationid,

    #[serde(skip_serializing)]
    pub att_owner_id: CfnIPAMResourceDiscoveryAssociationownerid,

    #[serde(skip_serializing)]
    pub att_resource_discovery_status: CfnIPAMResourceDiscoveryAssociationresourcediscoverystatus,

    #[serde(skip_serializing)]
    pub att_state: CfnIPAMResourceDiscoveryAssociationstate,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationipamarn;
impl CfnIPAMResourceDiscoveryAssociationipamarn {
    pub fn att_name(&self) -> &'static str {
        r#"IpamArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationipamregion;
impl CfnIPAMResourceDiscoveryAssociationipamregion {
    pub fn att_name(&self) -> &'static str {
        r#"IpamRegion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationarn;
impl CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationarn {
    pub fn att_name(&self) -> &'static str {
        r#"IpamResourceDiscoveryAssociationArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationid;
impl CfnIPAMResourceDiscoveryAssociationipamresourcediscoveryassociationid {
    pub fn att_name(&self) -> &'static str {
        r#"IpamResourceDiscoveryAssociationId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationownerid;
impl CfnIPAMResourceDiscoveryAssociationownerid {
    pub fn att_name(&self) -> &'static str {
        r#"OwnerId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationresourcediscoverystatus;
impl CfnIPAMResourceDiscoveryAssociationresourcediscoverystatus {
    pub fn att_name(&self) -> &'static str {
        r#"ResourceDiscoveryStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnIPAMResourceDiscoveryAssociationstate;
impl CfnIPAMResourceDiscoveryAssociationstate {
    pub fn att_name(&self) -> &'static str {
        r#"State"#
    }
}

impl cfn_resources::CfnResource for CfnIPAMResourceDiscoveryAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::IPAMResourceDiscoveryAssociation"
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
#[serde(default)]
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
