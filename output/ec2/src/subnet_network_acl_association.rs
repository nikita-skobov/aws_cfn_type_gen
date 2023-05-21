

/// Associates a subnet with a network ACL. For more information, see ReplaceNetworkAclAssociation in the Amazon EC2 API     Reference.
///
/// When AWS::EC2::SubnetNetworkAclAssociation resources are created during     create or update operations, AWS CloudFormation adopts existing resources that share     the same key properties (the properties that contribute to uniquely identify the resource).     However, if the operation fails and rolls back, AWS CloudFormation deletes the     previously out-of-band resources. You can protect against this behavior by using     Retain deletion policies. For more information, see DeletionPolicy Attribute.
#[derive(Default, serde::Serialize)]
pub struct CfnSubnetNetworkAclAssociation {


    /// 
    /// The ID of the network ACL.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "NetworkAclId")]
    pub network_acl_id: String,


    /// 
    /// The ID of the subnet.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,

}
