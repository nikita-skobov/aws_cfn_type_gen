/// Associates a subnet with a network ACL. For more information, see ReplaceNetworkAclAssociation in the Amazon EC2 API     Reference.
///
/// When AWS::EC2::SubnetNetworkAclAssociation resources are created during     create or update operations, AWS CloudFormation adopts existing resources that share     the same key properties (the properties that contribute to uniquely identify the resource).     However, if the operation fails and rolls back, AWS CloudFormation deletes the     previously out-of-band resources. You can protect against this behavior by using     Retain deletion policies. For more information, see DeletionPolicy Attribute.
#[derive(Clone, Debug, Default, serde::Serialize)]
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
    pub network_acl_id: cfn_resources::StrVal,

    ///
    /// The ID of the subnet.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetId")]
    pub subnet_id: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_association_id: CfnSubnetNetworkAclAssociationassociationid,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSubnetNetworkAclAssociationassociationid;
impl CfnSubnetNetworkAclAssociationassociationid {
    pub fn att_name(&self) -> &'static str {
        r#"AssociationId"#
    }
}

impl cfn_resources::CfnResource for CfnSubnetNetworkAclAssociation {
    fn type_string(&self) -> &'static str {
        "AWS::EC2::SubnetNetworkAclAssociation"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
