

/// Associates a service with a service network.
///
/// You can't use this operation if the service and service network are already associated or if  there is a disassociation or deletion in progress. If the association fails, you can retry the  operation by deleting the association and recreating it.
///
/// You cannot associate a service and service network that are shared with a caller. The caller  must own either the service or the service network.
///
/// As a result of this operation, the association is created in the service network account and  the association owner account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceNetworkServiceAssociation {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: DnsEntry
    ///
    /// Update requires: No interruption
    #[serde(rename = "DnsEntry")]
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
    pub service_identifier: Option<String>,


    /// 
    /// The ID or Amazon Resource Name (ARN) of the service network. You must use the ARN if the  resources specified in the operation are in different accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceNetworkIdentifier")]
    pub service_network_identifier: Option<String>,


    /// 
    /// The tags for the association.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}



impl cfn_resources::CfnResource for CfnServiceNetworkServiceAssociation {
    fn type_string() -> &'static str {
        "AWS::VpcLattice::ServiceNetworkServiceAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// DNS information about the service.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub domain_name: Option<String>,


    /// 
    /// The ID of the hosted zone.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedZoneId")]
    pub hosted_zone_id: Option<String>,

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


