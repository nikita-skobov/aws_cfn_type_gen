

/// Associates a VPC with a service network. When you associate a VPC with the service network,  it enables all the resources within that VPC to be clients and communicate with other services in  the service network. For more information, see Manage VPC associations in the Amazon VPC Lattice User Guide.
///
/// You can't use this operation if there is a disassociation in progress. If the association  fails, retry by deleting the association and recreating it.
///
/// As a result of this operation, the association gets created in the service network account  and the VPC owner account.
///
/// If you add a security group to the service network and VPC association, the association must  continue to always have at least one security group. You can add or edit security groups at any  time. However, to remove all security groups, you must first delete the association and recreate  it without security groups.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServiceNetworkVpcAssociation {


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


    /// 
    /// The ID of the VPC.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcIdentifier")]
    pub vpc_identifier: Option<String>,


    /// 
    /// The IDs of the security groups. Security groups aren't added by default. You can add a  security group to apply network level controls to control which resources in a VPC are allowed to  access the service network and its services. For more information, see Control traffic to   resources using security groups in the Amazon VPC User  Guide.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,


    /// 
    /// The ID or Amazon Resource Name (ARN) of the service network. You must use the ARN when the  resources specified in the operation are in different accounts.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceNetworkIdentifier")]
    pub service_network_identifier: Option<String>,

}



impl cfn_resources::CfnResource for CfnServiceNetworkVpcAssociation {
    fn type_string() -> &'static str {
        "AWS::VpcLattice::ServiceNetworkVpcAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
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


