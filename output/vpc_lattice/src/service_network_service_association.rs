/// Associates a service with a service network.
///
/// You can't use this operation if the service and service network are already associated or if  there is a disassociation or deletion in progress. If the association fails, you can retry the  operation by deleting the association and recreating it.
///
/// You cannot associate a service and service network that are shared with a caller. The caller  must own either the service or the service network.
///
/// As a result of this operation, the association is created in the service network account and  the association owner account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociation {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DnsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_entry: Option<DnsEntry>,

    ///
    /// The ID or Amazon Resource Name (ARN) of the service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The ID or Amazon Resource Name (ARN) of the service network. You must use the ARN if the  resources specified in the operation are in different accounts.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceNetworkIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_network_identifier: Option<cfn_resources::StrVal>,

    ///
    /// The tags for the association.
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
    pub att_arn: CfnServiceNetworkServiceAssociationarn,

    #[serde(skip_serializing)]
    pub att_created_at: CfnServiceNetworkServiceAssociationcreatedat,

    #[serde(skip_serializing)]
    pub att_dns_entry_domain_name: CfnServiceNetworkServiceAssociationdnsentrydomainname,

    #[serde(skip_serializing)]
    pub att_dns_entry_hosted_zone_id: CfnServiceNetworkServiceAssociationdnsentryhostedzoneid,

    #[serde(skip_serializing)]
    pub att_id: CfnServiceNetworkServiceAssociationid,

    #[serde(skip_serializing)]
    pub att_service_arn: CfnServiceNetworkServiceAssociationservicearn,

    #[serde(skip_serializing)]
    pub att_service_id: CfnServiceNetworkServiceAssociationserviceid,

    #[serde(skip_serializing)]
    pub att_service_name: CfnServiceNetworkServiceAssociationservicename,

    #[serde(skip_serializing)]
    pub att_service_network_arn: CfnServiceNetworkServiceAssociationservicenetworkarn,

    #[serde(skip_serializing)]
    pub att_service_network_id: CfnServiceNetworkServiceAssociationservicenetworkid,

    #[serde(skip_serializing)]
    pub att_service_network_name: CfnServiceNetworkServiceAssociationservicenetworkname,

    #[serde(skip_serializing)]
    pub att_status: CfnServiceNetworkServiceAssociationstatus,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationarn;
impl CfnServiceNetworkServiceAssociationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationcreatedat;
impl CfnServiceNetworkServiceAssociationcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationdnsentrydomainname;
impl CfnServiceNetworkServiceAssociationdnsentrydomainname {
    pub fn att_name(&self) -> &'static str {
        r#"DnsEntry.DomainName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationdnsentryhostedzoneid;
impl CfnServiceNetworkServiceAssociationdnsentryhostedzoneid {
    pub fn att_name(&self) -> &'static str {
        r#"DnsEntry.HostedZoneId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationid;
impl CfnServiceNetworkServiceAssociationid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationservicearn;
impl CfnServiceNetworkServiceAssociationservicearn {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationserviceid;
impl CfnServiceNetworkServiceAssociationserviceid {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationservicename;
impl CfnServiceNetworkServiceAssociationservicename {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationservicenetworkarn;
impl CfnServiceNetworkServiceAssociationservicenetworkarn {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationservicenetworkid;
impl CfnServiceNetworkServiceAssociationservicenetworkid {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkId"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationservicenetworkname;
impl CfnServiceNetworkServiceAssociationservicenetworkname {
    pub fn att_name(&self) -> &'static str {
        r#"ServiceNetworkName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServiceNetworkServiceAssociationstatus;
impl CfnServiceNetworkServiceAssociationstatus {
    pub fn att_name(&self) -> &'static str {
        r#"Status"#
    }
}

impl cfn_resources::CfnResource for CfnServiceNetworkServiceAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::VpcLattice::ServiceNetworkServiceAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.dns_entry
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// DNS information about the service.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DnsEntry {
    ///
    /// The domain name of the service.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<cfn_resources::StrVal>,

    ///
    /// The ID of the hosted zone.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DnsEntry {
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
